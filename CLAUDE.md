# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository purpose

**This repo is a personal French translation of _The Rust Programming Language_**, built on top of a fork of upstream `rust-lang/book` (the `origin` remote still points at upstream). The user is the translator; the default language for working with them is French.

The translation is stored in **`po/fr.po`** (gettext PO format) — `src/*.md` stays pristine-upstream, and the French rendering is produced at build time by the `mdbook-i18n-helpers` gettext preprocessor. To build the French book:

```sh
MDBOOK_BOOK__LANGUAGE=fr mdbook build
```

Omitting the env var builds the upstream English book (no gettext substitution).

### Workflow when upstream changes

1. `git fetch origin && git merge origin/main` to pull upstream edits into `src/*.md`.
2. Regenerate the POT:
   ```sh
   MDBOOK_OUTPUT__XGETTEXT__POT_FILE=messages.pot mdbook build -d /tmp/po-extract
   mv /tmp/po-extract/xgettext/messages.pot po/messages.pot
   rm -rf /tmp/po-extract
   ```
3. `msgmerge --update po/fr.po po/messages.pot` — this marks edited entries `fuzzy` and leaves them in place next to the new msgid. New msgids appear with an empty msgstr.
4. Open `po/fr.po` in your editor, translate the new entries, and remove the `, fuzzy` flag from edited ones after updating the msgstr.
5. `MDBOOK_BOOK__LANGUAGE=fr mdbook build` to verify; `MDBOOK_BOOK__LANGUAGE=fr mdbook test --library-path packages/trpl/target/debug/deps` to make sure code examples still compile in the French rendering.

### What NOT to touch

- **`src/*.md`** is pristine upstream — do not edit these files. `git status` in a clean tree should show no modifications under `src/`.
- **`nostarch/`**, **`first-edition/`**, **`second-edition/`**, **`2018-edition/`**, and **`redirects/`** are frozen archives or touched only by upstream's `tools/` scripts.

Runnable example code lives as full Cargo projects under `listings/` and is spliced into the prose via `mdbook` include directives. Code itself stays in English (identifiers, stdlib calls); only comments and user-facing strings get translated when the prose relies on them.

### Historical note

Prior to the 2026-04-22 po4a migration, the translation lived inline in `src/*.md` using a bilingual convention (English in `<!-- … -->` HTML comments + French prose below). That format is preserved on the `main` branch up to commit `5ec20efa` for reference, but the active branch is `migrate-to-po4a` and it uses the PO workflow described above. If you see `<!-- … -->` English blocks in the working tree, you are on the wrong branch.

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
- `mdbook-trpl-listing` converts a custom `<Listing>...</Listing>` element wrapping a code block into the numbered-caption HTML figure rendered in the book (styled by `theme/listing.css`). **Patched** to match `<Listing>` tags case-insensitively and re-insert blank-line separators, because the `gettext` preprocessor used for translation lowercases HTML tag names and can collapse blank lines around custom block elements.
- `gettext` (from `mdbook-i18n-helpers`) is wired in by `book.toml` with `after = ["links"]`, and both `trpl-note` / `trpl-listing` run with `after = ["gettext"]`. This order means the French translation is substituted into the markdown before the custom-element preprocessors transform it. The POT file (`po/messages.pot`) is generated by the same preprocessor running under the `xgettext` renderer.

When you write new prose, use these elements — they are part of the authoring vocabulary, not incidental HTML.

### Chapter-number redirects

`book.toml` contains a large `[output.html.redirect]` table mapping old URLs (ch17/18/19/20) to their current numbers. Chapters were renumbered; preserve these redirects when editing `book.toml`.

## Conventions when editing

`style-guide.md` describes the upstream English conventions. For `po/fr.po` msgstrs:

- **Do not hard-wrap msgstr content** at 80 columns — gettext wraps long strings automatically using its own line-continuation format (`msgstr ""` then `"..."` continuations). Hard newlines in msgid/msgstr are for actual paragraph breaks (rare).
- Match the msgid's inline markdown structure exactly: if msgid has `[foo](url)` (inline link), the msgstr should also use the inline form with the same URL. Do not use reference-style `[foo][ref]` links — the link reference definitions at the end of the pristine source are not available at substitution time.
- Preserve backtick-code spans (`read_line`), code blocks (```` ``` ````), and custom elements (`<Listing>`, `<section class="note">`) as-is.

For the French translation specifically:

- Respect French orthography fully: accents (é, è, ê, à, ù, ç…), typographic conventions, no ASCII substitutions.
- **Typographic apostrophes** `'` (not ASCII `'`) and **guillemets** `« … »` with non-breaking spaces (U+00A0) for quoted terms. NBSP before `? ! : ;`.
- Rust keywords, API names, crate names, and code identifiers stay in English.
- Accept that the `ci/spellcheck.sh` lint is for English only and will flag every French word; it is effectively disabled for translation work. The upstream CI lints (`lfp`, `validate.sh`, link-checker) still apply and are worth running before committing.
