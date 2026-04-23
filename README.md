# The Rust Programming Language — Traduction française

[![Deploy FR to GitHub Pages](https://github.com/Titome/rust-book-fr/actions/workflows/deploy.yml/badge.svg)](https://github.com/Titome/rust-book-fr/actions/workflows/deploy.yml)
[![CI](https://github.com/Titome/rust-book-fr/workflows/CI/badge.svg)](https://github.com/Titome/rust-book-fr/actions/workflows/main.yml)

Traduction française de _The Rust Programming Language_ (le livre officiel, alias « the book »), maintenue comme fork de [`rust-lang/book`][upstream].

**→ [Lire la version FR en ligne](https://titome.github.io/rust-book-fr/)**

La traduction anglaise originale reste disponible chez `rust-lang` :
[stable][stable-en] · [beta][beta-en] · [nightly][nightly-en] · [livre papier][nostarch].

[upstream]: https://github.com/rust-lang/book
[stable-en]: https://doc.rust-lang.org/stable/book/
[beta-en]: https://doc.rust-lang.org/beta/book/
[nightly-en]: https://doc.rust-lang.org/nightly/book/
[nostarch]: https://nostarch.com/rust-programming-language-3rd-edition

## Comment ce fork fonctionne

L'arborescence `src/` reste **pristine upstream** — jamais modifiée. La traduction française vit dans `po/fr.po` (format gettext) et est substituée au moment du build par le préprocesseur [`mdbook-i18n-helpers`][mdbook-i18n-helpers]. Les légendes de figures et textes alternatifs d'images, qui ne sont pas extraits comme msgids par `mdbook-i18n-helpers`, vivent dans `po/figures-fr.toml` et sont substitués par un préprocesseur maison, `mdbook-trpl-figures-i18n`.

[mdbook-i18n-helpers]: https://github.com/google/mdbook-i18n-helpers

Pourquoi ce workflow plutôt que des fichiers `src/*.md` bilingues ? Parce que les pulls upstream sont transparents : si `rust-lang/book` modifie un paragraphe, `git merge upstream/main` applique le changement sur `src/` (qui est pristine), puis `msgmerge` marque l'entrée correspondante dans `po/fr.po` comme `fuzzy`, signalant explicitement que la traduction FR a besoin d'être révisée. Rien ne se désynchronise silencieusement.

## Pré-requis

- [Rust 1.90][rust-toolchain] (pinné via `rust-toolchain`)
- [mdBook 0.5.1][mdbook] — la même version que CI
- [`mdbook-i18n-helpers`][mdbook-i18n-helpers] — requis seulement pour le build FR

[rust-toolchain]: ./rust-toolchain
[mdbook]: https://github.com/rust-lang/mdBook

```sh
cargo install mdbook --locked --version 0.5.1
cargo install mdbook-i18n-helpers --locked    # uniquement pour le build FR
```

## Build

### Version française

```sh
MDBOOK_BOOK__LANGUAGE=fr mdbook build
```

Le rendu HTML est dans `book/`. Ouvre `book/index.html` dans ton navigateur.

### Version anglaise (fallback pristine upstream)

```sh
mdbook build
```

`[preprocessor.gettext]` est marqué `optional = true` dans `book.toml`, donc le build EN fonctionne même si `mdbook-i18n-helpers` n'est pas installé.

### Serve avec rechargement auto

```sh
MDBOOK_BOOK__LANGUAGE=fr mdbook serve --open
```

## Tests

Les exemples de code du livre sont testés via `mdbook test`. La crate `trpl` (façade vers `tokio` & co. utilisée par les lecteurs) doit être pré-compilée pour que `mdbook test` trouve ses dépendances :

```sh
cd packages/trpl && cargo build && cd ../..
mdbook test --library-path packages/trpl/target/debug/deps                           # EN
MDBOOK_BOOK__LANGUAGE=fr mdbook test --library-path packages/trpl/target/debug/deps  # FR
```

Les préprocesseurs maison (`mdbook-trpl-note`, `mdbook-trpl-listing`, `mdbook-trpl-figures-i18n`, …) sont également testés :

```sh
cargo test --manifest-path packages/mdbook-trpl/Cargo.toml
```

## Workflow de maintenance : sync avec upstream

Quand `rust-lang/book` évolue :

```sh
# 1. Pull les changements upstream
git fetch upstream
git merge upstream/main                          # résoudre les conflits éventuels sur src/

# 2. Régénérer le POT depuis le nouveau src/
MDBOOK_OUTPUT__XGETTEXT__POT_FILE=messages.pot mdbook build -d /tmp/po-extract
mv /tmp/po-extract/xgettext/messages.pot po/messages.pot
rm -rf /tmp/po-extract

# 3. Merger les nouveaux msgids dans po/fr.po
msgmerge --update po/fr.po po/messages.pot      # marque les entrées modifiées 'fuzzy'

# 4. Réviser po/fr.po : traduire les nouvelles entrées, relire les 'fuzzy'
#    (chercher `, fuzzy` dans le fichier pour les trouver)
$EDITOR po/fr.po

# 5. Build/test
cd packages/trpl && cargo build && cd ../..
MDBOOK_BOOK__LANGUAGE=fr mdbook test --library-path packages/trpl/target/debug/deps

# 6. Commit + push
git add . && git commit -m "Sync upstream + update FR translations"
git push
```

Les légendes de figures (`po/figures-fr.toml`) sont à mettre à jour manuellement si upstream ajoute/modifie des `<figcaption>` ou `<img alt="...">`.

## Organisation du repo

- `src/` — pristine upstream, **ne pas éditer**.
- `po/fr.po` — traductions FR (5 089 msgids) au format gettext standard.
- `po/messages.pot` — template extrait depuis `src/` par `mdbook-i18n-helpers`.
- `po/figures-fr.toml` — traductions des `<figcaption>` et attributs `alt=` non extraits comme msgids.
- `packages/mdbook-trpl/` — préprocesseurs maison (`note`, `listing`, `figures-i18n`, `figure`, `heading`).
- `packages/trpl/` — crate façade dépendue par les exemples du livre.
- `listings/` — exemples Cargo complets inclus dans la prose via directives mdbook.
- `book.toml` — config mdbook, preprocessors wiring, redirects.
- `.github/workflows/deploy.yml` — build FR + `mdbook test` + déploiement Pages.
- `.github/workflows/main.yml` — CI héritée upstream (tests + lints EN, inchangée).

## Contributions

Si tu repères une coquille, une traduction maladroite, ou une section désynchronisée d'upstream, une issue ou PR est bienvenue. Merci de préciser la section concernée (chapitre + paragraphe) pour faciliter la revue.

La contribution upstream (vers `rust-lang/book` en anglais) suit un [processus séparé][upstream-contrib].

[upstream-contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

## Licence

Comme upstream, double-licence MIT ou Apache 2.0 (voir `LICENSE-MIT` et `LICENSE-APACHE`).
