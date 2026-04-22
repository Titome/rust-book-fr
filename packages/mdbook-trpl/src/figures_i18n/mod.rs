use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use mdbook_preprocessor::{
    book::{Book, BookItem},
    Preprocessor,
};
use serde::Deserialize;

use crate::CompositeError;

/// A preprocessor that substitutes English figure captions and image alt text
/// with translated equivalents.
///
/// Complements `mdbook-i18n-helpers`, which does not extract `<figcaption>` or
/// `<img alt="...">` content as translatable msgids (they sit inside HTML
/// blocks that its xgettext renderer treats as opaque).
///
/// Reads translations from a per-language TOML file pointed at by the
/// `file` key in `[preprocessor.trpl-figures-i18n]`:
///
/// ```toml
/// [preprocessor.trpl-figures-i18n]
/// file = "po/figures-fr.toml"
/// language = "fr"
/// ```
///
/// The TOML file has entries of the form:
///
/// ```toml
/// [[sub]]
/// en = "Figure 17-1: A concurrent workflow, switching between Task A and Task B"
/// fr = "Figure 17-1 : Un flux de travail concurrent, …"
/// ```
///
/// The preprocessor only runs when the book's language (`ctx.config.book.language`)
/// matches the `language` key in its config, so the English build is untouched.
pub struct TrplFiguresI18n;

impl TrplFiguresI18n {
    pub fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    file: String,
    language: String,
}

#[derive(Debug, Deserialize)]
struct SubsFile {
    #[serde(default)]
    sub: Vec<Substitution>,
}

#[derive(Debug, Deserialize)]
struct Substitution {
    en: String,
    fr: String,
}

impl Preprocessor for TrplFiguresI18n {
    fn name(&self) -> &str {
        "trpl-figures-i18n"
    }

    fn run(
        &self,
        ctx: &mdbook_preprocessor::PreprocessorContext,
        mut book: Book,
    ) -> Result<Book> {
        let config: Config = ctx
            .config
            .get(&format!("preprocessor.{}", self.name()))
            .context("reading preprocessor config")?
            .ok_or_else(|| anyhow!("missing [preprocessor.{}] config", self.name()))?;

        // Only substitute when the book's selected language matches the
        // preprocessor's configured target language.
        let current_lang = ctx.config.book.language.clone().unwrap_or_default();
        if current_lang != config.language {
            return Ok(book);
        }

        let file_path = ctx.root.join(&config.file);
        let subs: HashMap<String, String> = if file_path.exists() {
            let text = std::fs::read_to_string(&file_path)
                .with_context(|| format!("reading {}", file_path.display()))?;
            let parsed: SubsFile = toml::from_str(&text)
                .with_context(|| format!("parsing {}", file_path.display()))?;
            parsed
                .sub
                .into_iter()
                .map(|s| (s.en, s.fr))
                .collect()
        } else {
            return Ok(book);
        };

        let mut errors = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite(&chapter.content, &subs) {
                    Ok(rewritten) => chapter.content = rewritten,
                    Err(reason) => errors.push(reason),
                }
            }
        });

        if errors.is_empty() {
            Ok(book)
        } else {
            Err(CompositeError(errors).into())
        }
    }
}

/// Apply the substitutions to the content.
///
/// Matches with flexible whitespace: each run of whitespace in the `en`
/// pattern matches any run of whitespace in the content (`\s+`). This is
/// necessary because `<img alt="…">` attribute values in the pristine source
/// are hard-wrapped mid-word at column boundaries, so the `en` form written
/// in the TOML (which uses normal sentence-spacing) needs to match content
/// that has embedded newlines inside words.
fn rewrite(content: &str, subs: &HashMap<String, String>) -> Result<String, anyhow::Error> {
    let mut out = content.to_string();
    for (en, fr) in subs {
        // Build a regex from the `en` pattern: split on whitespace, escape
        // each token, join with `\s+`. This tolerates any amount of
        // whitespace (including newlines) between words.
        let tokens: Vec<String> = en
            .split_whitespace()
            .map(regex::escape)
            .collect();
        let pattern = tokens.join(r"\s+");
        let re = regex::Regex::new(&pattern)
            .with_context(|| format!("building regex from en substitution: {en:?}"))?;
        // `fr` may contain regex-meta characters ($ etc.); use no_expand to
        // treat it as a literal replacement.
        out = re.replace_all(&out, regex::NoExpand(fr)).into_owned();
    }
    Ok(out)
}
