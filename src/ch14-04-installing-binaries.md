<!--
Old headings. Do not remove or links may break.
-->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

<!--
## Installing Binaries with `cargo install`
-->

## Installer des binaires avec `cargo install`

<!--
The `cargo install` command allows you to install and use binary crates
locally. This isn't intended to replace system packages; it's meant to be a
convenient way for Rust developers to install tools that others have shared on
[crates.io](https://crates.io/) ignore
-->. Note that you can only install
packages that have binary targets. A _binary target_ is the runnable program
that is created if the crate has a _src/main.rs_ file or another file specified
as a binary, as opposed to a library target that isn't runnable on its own but
is suitable for including within other programs. Usually, crates have
information in the README file about whether a crate is a library, has a
binary target, or both.
-->

La commande `cargo install` vous permet d'installer et d'utiliser des crates binaires localement. Cela n'est pas destine a remplacer les paquets systeme ; c'est un moyen pratique pour les developpeurs Rust d'installer des outils que d'autres ont partages sur [crates.io](https://crates.io/)<!--
ignore
-->. Notez que vous ne pouvez installer que des paquets qui ont des cibles binaires. Une _cible binaire_ est le programme executable qui est cree si le crate possede un fichier _src/main.rs_ ou un autre fichier specifie comme binaire, par opposition a une cible de bibliotheque qui n'est pas executable en soi mais qui est adaptee pour etre incluse dans d'autres programmes. En general, les crates contiennent des informations dans le fichier README indiquant si un crate est une bibliotheque, possede une cible binaire, ou les deux.

<!--
All binaries installed with `cargo install` are stored in the installation
root's _bin_ folder. If you installed Rust using _rustup.rs_ and don't have any
custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that
this directory is in your `$PATH` to be able to run programs you've installed
with `cargo install`.
-->

Tous les binaires installes avec `cargo install` sont stockes dans le dossier _bin_ du repertoire racine d'installation. Si vous avez installe Rust avec _rustup.rs_ et que vous n'avez aucune configuration personnalisee, ce repertoire sera *$HOME/.cargo/bin*. Assurez-vous que ce repertoire est dans votre `$PATH` pour pouvoir executer les programmes que vous avez installes avec `cargo install`.

<!--
For example, in Chapter 12 we mentioned that there's a Rust implementation of
the `grep` tool called `ripgrep` for searching files. To install `ripgrep`, we
can run the following:
-->

Par exemple, dans le chapitre 12, nous avons mentionne qu'il existe une implementation en Rust de l'outil `grep` appelee `ripgrep` pour rechercher dans les fichiers. Pour installer `ripgrep`, nous pouvons executer la commande suivante :

<!--
manual-regeneration
cargo install something you don't have, copy relevant output below
-->

<!--
```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

<!--
The second-to-last line of the output shows the location and the name of the
installed binary, which in the case of `ripgrep` is `rg`. As long as the
installation directory is in your `$PATH`, as mentioned previously, you can
then run `rg --help` and start using a faster, Rustier tool for searching files!
-->

L'avant-derniere ligne de la sortie indique l'emplacement et le nom du binaire installe, qui dans le cas de `ripgrep` est `rg`. Tant que le repertoire d'installation est dans votre `$PATH`, comme mentionne precedemment, vous pouvez alors executer `rg --help` et commencer a utiliser un outil plus rapide, ecrit en Rust, pour rechercher dans les fichiers !
