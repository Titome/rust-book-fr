# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository purpose

**This repo is a personal in-progress French translation of _The Rust Programming Language_**, built on top of a fork of upstream `rust-lang/book` (the `origin` remote still points at upstream). The user is the translator; the default language for working with them is French.

The translation lives in `src/*.md`. The convention is **dual-prose**: every original English block is preserved verbatim inside an HTML comment, followed by the French translation as live prose. Example:

```md
<!--
# Getting Started
-->

# Mise en route

<!--
Let's start your Rust journey!
-->

Commençons votre aventure avec Rust !
```

Keep that shape when touching prose:

- Do not delete the `<!-- ... -->` English blocks. They are how upstream merges stay tractable — when `rust-lang/book` edits a paragraph, the English inside the comment is what `git` diffs against, which is how we detect that the surrounding French translation needs updating.
- When a new upstream paragraph arrives without a French counterpart, translate it and add the French block below the `<!-- ... -->`. Do not leave English prose rendered into the book.
- When upstream edits an existing English block, update the French translation to match.
- Because both languages are present, the 80-column wrap applies to the French as well.

The widespread `M src/*.md` in `git status` is expected — it is the translation-in-progress against the upstream baseline, not dirty state to clean up.

Runnable example code lives as full Cargo projects under `listings/` and is spliced into the prose via `mdbook` include directives. Code itself stays in English (identifiers, stdlib calls); only comments and user-facing strings get translated when the prose relies on them.

The `nostarch/` directory contains snapshots sent to the print publisher of the English book. **Never edit files under `nostarch/`** — they are only touched by upstream's scripts in `tools/` when pushing a new round of edits, and are irrelevant to the translation. The `first-edition/`, `second-edition/`, `2018-edition/`, and `redirects/` directories are frozen archives.

## Common commands

Toolchain pin is Rust `1.90` (`rust-toolchain`). `mdbook` version is `0.5.1` (pinned in CI — install with `cargo install mdbook --locked --version 0.5.1`).

```sh
# Build the book -> ./book/index.html
mdbook build

# Run all code-in-prose tests. The trpl crate must be pre-built so that
# mdbook test can find it on the library search path.
cd packages/trpl && cargo build && cd ../..
mdbook test --library-path packages/trpl/target/debug/deps

# Workspace tests (the `tools` bin crates)
cargo test

# mdbook-trpl preprocessor tests (separate workspace)
cargo test --manifest-path packages/mdbook-trpl/Cargo.toml

# Lints run in CI
bash ci/spellcheck.sh list          # aspell against ci/dictionary.txt
cargo run --bin lfp src             # local-file-path lint
bash ci/validate.sh                 # link2print reference validation
find . -name '*.sh' -print0 | xargs -0 shellcheck

# Formatting
rustfmt <file.rs>                   # Rust code (including inside listings/)
dprint fmt <file.md>                # Markdown / JSON / CSS — see dprint.jsonc excludes
```

To run a single listing directly, `cd` into `listings/chXX-.../listing-XX-YY/` and use normal `cargo run` / `cargo test`. Each listing is an independent Cargo project deliberately excluded from the workspace (see `Cargo.toml` `exclude`).

## Architecture

### Two independent Cargo workspaces

The repo intentionally contains more than one build graph. Do not try to unify them.

1. **Root workspace** (`Cargo.toml`): a single member, `packages/tools`, which produces helper binaries (`link2print`, `lfp`, `release_listings`, `concat_chapters`, `remove_hidden_lines`, `remove_links`, `remove_markup`, `cleanup_blockquotes`, `convert_quotes`) used by `ci/` and `tools/` scripts. The `listings/`, `tmp/`, and `linkchecker/` trees are explicitly `exclude`d so each listing compiles on its own.
2. **`packages/mdbook-trpl`**: its own workspace producing the mdbook preprocessors. `book.toml` wires them in via `cargo run --manifest-path packages/mdbook-trpl/Cargo.toml --bin mdbook-trpl-{note,listing}`, so `mdbook build` triggers builds in this crate transparently. Bins: `note`, `listing`, `figure`, `heading`.
3. **`packages/trpl`**: the public facade crate readers depend on throughout the book. It mostly re-exports from Tokio et al. so readers add one dependency and the book is insulated from upstream breaking changes. Its tests run inside its own directory.

### How prose and code connect

Prose in `src/chXX-YY-*.md` embeds code from `listings/` using mdbook directives:

- `{{#rustdoc_include path/to/file.rs:anchor}}` for Rust — hides the non-anchored portion from the reader but still hands the full file to `rustdoc` during `mdbook test`.
- `{{#include ...}}` for non-Rust files (e.g. `output.txt`, `Cargo.toml` snippets).
- Anchors are `// ANCHOR: tag` / `// ANCHOR_END: tag` pairs in the source file.

Listing directory naming is load-bearing and consumed by the tooling:

- `listing-XX-YY/` — numbered listings (renumbering requires updating the number in prose too).
- `no-listing-NN-short-description/` — unnumbered code examples.
- `output-only-NN-short-description/` — only the `output.txt` is shown in the book.
- A `rustfmt-ignore` file in a listing dir opts it out of bulk `rustfmt` runs (some examples must not parse).

### Regenerating compiler output

`output.txt` files beside listings are _regenerated_, not hand-edited. The `./tools/update-rustc.sh` script rebuilds every listing with `cargo clean && cargo <cmd>`, normalizes paths (user home, rustup toolchain triple, compile time, test binary hash) so diffs stay small, and writes the result back. Run this after bumping the toolchain or editing code. For output that cannot be scripted (user input, network calls), the markdown contains a `manual-regeneration` comment with instructions — grep for it.

### mdbook preprocessors (what they do to your markdown)

- `mdbook-trpl-note` transforms block-quotes starting with `> Note:` / `> Tip:` into semantic `<section class="note">` aside elements (styled by `theme/semantic-notes.css`).
- `mdbook-trpl-listing` converts a custom `<Listing>...</Listing>` element wrapping a code block into the numbered-caption HTML figure rendered in the book (styled by `theme/listing.css`).

When you write new prose, use these elements — they are part of the authoring vocabulary, not incidental HTML.

### Chapter-number redirects

`book.toml` contains a large `[output.html.redirect]` table mapping old URLs (ch17/18/19/20) to their current numbers. Chapters were renumbered; preserve these redirects when editing `book.toml`.

## Conventions when editing

`style-guide.md` describes the upstream English conventions. The English inside the `<!-- ... -->` blocks should not be restyled (it is upstream's prose), but a few of its rules transfer naturally to the French:

- Hard-wrap prose at 80 columns (French too).
- When referring to a method in prose, omit parens: `read_line`, not `read_line()`.
- Put a filename label above a code block when it represents an actual project file.
- Intra-book and stdlib API links should be relative so the book works both offline and on docs.rust-lang.org.

For the French translation specifically:

- Respect French orthography fully: accents (é, è, ê, à, ù, ç…), typographic conventions, no ASCII substitutions.
- French typographic spacing (non-breaking space before `?`, `!`, `:`, `;`, inside `«  »`) is the standard for published French prose — follow what the surrounding already-translated text does rather than mixing styles.
- Rust keywords, API names, crate names, and code identifiers stay in English.

The upstream `ci/spellcheck.sh` is built for English against `ci/dictionary.txt` — it will flag essentially every French word. Do not attempt to "fix" the spellcheck output by massaging the French; it is effectively disabled for translation work, and running it is only useful on the English blocks. The upstream CI lints (`lfp`, `validate.sh`, link-checker) still apply and are worth running before committing.
