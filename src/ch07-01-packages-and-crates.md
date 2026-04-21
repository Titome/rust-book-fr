<!--
## Packages and Crates
-->

## Les packages et les crates

<!--
The first parts of the module system we'll cover are packages and crates.
-->

Les premières parties du système de modules que nous aborderons sont les
packages et les crates.

<!--
A _crate_ is the smallest amount of code that the Rust compiler considers at a
time. Even if you run `rustc` rather than `cargo` and pass a single source code
file (as we did all the way back in ["Rust Program Basics"][basics] ignore
--> in Chapter 1), the compiler considers that file to be a crate. Crates can
contain modules, and the modules may be defined in other files that get
compiled with the crate, as we'll see in the coming sections.
-->

Une *crate* est la plus petite quantité de code que le compilateur Rust prend
en compte à la fois. Même si vous exécutez `rustc` plutôt que `cargo` et que
vous passez un seul fichier de code source (comme nous l'avons fait au tout
début dans [« Les bases d'un programme Rust »][basics]<!--
ignore
--> au
chapitre 1), le compilateur considère ce fichier comme une crate. Les crates
peuvent contenir des modules, et les modules peuvent être définis dans d'autres
fichiers qui sont compilés avec la crate, comme nous le verrons dans les
sections suivantes.

<!--
A crate can come in one of two forms: a binary crate or a library crate.
_Binary crates_ are programs you can compile to an executable that you can run,
such as a command line program or a server. Each must have a function called
`main` that defines what happens when the executable runs. All the crates we've
created so far have been binary crates.
-->

Une crate peut se présenter sous l'une des deux formes suivantes : une crate
binaire ou une crate de bibliothèque. Les *crates binaires* sont des programmes
que vous pouvez compiler en un exécutable que vous pouvez lancer, comme un
programme en ligne de commande ou un serveur. Chacune doit avoir une fonction
appelée `main` qui définit ce qui se passe lorsque l'exécutable s'exécute.
Toutes les crates que nous avons créées jusqu'à présent étaient des crates
binaires.

<!--
_Library crates_ don't have a `main` function, and they don't compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. For example, the `rand` crate we used in [Chapter
2][rand] ignore
--> provides functionality that generates random numbers.
Most of the time when Rustaceans say "crate," they mean library crate, and they
use "crate" interchangeably with the general programming concept of a "library."
-->

Les *crates de bibliothèque* n'ont pas de fonction `main` et ne se compilent
pas en exécutable. À la place, elles définissent des fonctionnalités destinées
à être partagées avec plusieurs projets. Par exemple, la crate `rand` que nous
avons utilisée au [chapitre 2][rand]<!--
ignore
--> fournit des fonctionnalités
qui génèrent des nombres aléatoires. La plupart du temps, quand les Rustacés
disent « crate », ils veulent dire crate de bibliothèque, et ils utilisent
« crate » de manière interchangeable avec le concept général de programmation
de « bibliothèque ».

<!--
The _crate root_ is a source file that the Rust compiler starts from and makes
up the root module of your crate (we'll explain modules in depth in ["Control
Scope and Privacy with Modules"][modules] ignore
-->).
-->

La *racine de la crate* (*crate root*) est un fichier source à partir duquel le
compilateur Rust commence et qui constitue le module racine de votre crate (nous
expliquerons les modules en détail dans [« Contrôler la portée et la
confidentialité avec les modules »][modules]<!--
ignore
-->).

<!--
A _package_ is a bundle of one or more crates that provides a set of
functionality. A package contains a _Cargo.toml_ file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command line tool you've been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command line tool uses.
-->

Un *package* est un ensemble d'une ou plusieurs crates qui fournit un ensemble
de fonctionnalités. Un package contient un fichier *Cargo.toml* qui décrit
comment compiler ces crates. Cargo est en fait un package qui contient la crate
binaire pour l'outil en ligne de commande que vous avez utilisé pour compiler
votre code. Le package Cargo contient également une crate de bibliothèque dont
la crate binaire dépend. D'autres projets peuvent dépendre de la crate de
bibliothèque de Cargo pour utiliser la même logique que l'outil en ligne de
commande Cargo.

<!--
A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that's a
library or binary crate.
-->

Un package peut contenir autant de crates binaires que vous le souhaitez, mais
au maximum une seule crate de bibliothèque. Un package doit contenir au moins
une crate, que ce soit une crate de bibliothèque ou une crate binaire.

<!--
Let's walk through what happens when we create a package. First, we enter the
command `cargo new my-project`:
-->

Voyons ce qui se passe lorsque nous créons un package. Tout d'abord, nous
entrons la commande `cargo new my-project` :

<!--
```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```
-->

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

<!--
After we run `cargo new my-project`, we use `ls` to see what Cargo creates. In
the _my-project_ directory, there's a _Cargo.toml_ file, giving us a package.
There's also a _src_ directory that contains _main.rs_. Open _Cargo.toml_ in
your text editor and note that there's no mention of _src/main.rs_. Cargo
follows a convention that _src/main.rs_ is the crate root of a binary crate
with the same name as the package. Likewise, Cargo knows that if the package
directory contains _src/lib.rs_, the package contains a library crate with the
same name as the package, and _src/lib.rs_ is its crate root. Cargo passes the
crate root files to `rustc` to build the library or binary.
-->

Après avoir exécuté `cargo new my-project`, nous utilisons `ls` pour voir ce
que Cargo crée. Dans le répertoire *my-project*, il y a un fichier *Cargo.toml*,
ce qui nous donne un package. Il y a aussi un répertoire *src* qui contient
*main.rs*. Ouvrez *Cargo.toml* dans votre éditeur de texte et notez qu'il n'y
a aucune mention de *src/main.rs*. Cargo suit une convention selon laquelle
*src/main.rs* est la racine de la crate binaire portant le même nom que le
package. De même, Cargo sait que si le répertoire du package contient
*src/lib.rs*, le package contient une crate de bibliothèque portant le même nom
que le package, et *src/lib.rs* est sa racine de crate. Cargo passe les fichiers
racines de crate à `rustc` pour compiler la bibliothèque ou le binaire.

<!--
Here, we have a package that only contains _src/main.rs_, meaning it only
contains a binary crate named `my-project`. If a package contains _src/main.rs_
and _src/lib.rs_, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the _src/bin_ directory: Each file will be a separate binary crate.
-->

Ici, nous avons un package qui ne contient que *src/main.rs*, ce qui signifie
qu'il ne contient qu'une crate binaire nommée `my-project`. Si un package
contient *src/main.rs* et *src/lib.rs*, il a deux crates : une binaire et une
de bibliothèque, toutes deux portant le même nom que le package. Un package
peut avoir plusieurs crates binaires en plaçant des fichiers dans le répertoire
*src/bin* : chaque fichier sera une crate binaire distincte.

[basics]: ch01-02-hello-world.html#rust-program-basics
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
