<!--
## Hello, Cargo!
-->

## Hello, Cargo !

<!--
Cargo is Rust's build system and package manager. Most Rustaceans use this tool
to manage their Rust projects because Cargo handles a lot of tasks for you,
such as building your code, downloading the libraries your code depends on, and
building those libraries. (We call the libraries that your code needs
_dependencies_.)
-->

Cargo est le systeme de build et le gestionnaire de paquets de Rust. La plupart des Rustaceans utilisent cet outil pour gerer leurs projets Rust car Cargo gere de nombreuses taches pour vous, comme compiler votre code, telecharger les bibliotheques dont votre code depend, et compiler ces bibliotheques. (Nous appelons les bibliotheques dont votre code a besoin des _dependances_.)

<!--
The simplest Rust programs, like the one we've written so far, don't have any
dependencies. If we had built the "Hello, world!" project with Cargo, it would
only use the part of Cargo that handles building your code. As you write more
complex Rust programs, you'll add dependencies, and if you start a project
using Cargo, adding dependencies will be much easier to do.
-->

Les programmes Rust les plus simples, comme celui que nous avons ecrit jusqu'ici, n'ont aucune dependance. Si nous avions construit le projet "Hello, world!" avec Cargo, nous n'aurions utilise que la partie de Cargo qui gere la compilation de votre code. A mesure que vous ecrirez des programmes Rust plus complexes, vous ajouterez des dependances, et si vous demarrez un projet avec Cargo, l'ajout de dependances sera beaucoup plus facile.

<!--
Because the vast majority of Rust projects use Cargo, the rest of this book
assumes that you're using Cargo too. Cargo comes installed with Rust if you
used the official installers discussed in the
["Installation"][installation] ignore
--> section. If you installed Rust
through some other means, check whether Cargo is installed by entering the
following in your terminal:
-->

Comme la grande majorite des projets Rust utilisent Cargo, le reste de ce livre suppose que vous utilisez egalement Cargo. Cargo est installe avec Rust si vous avez utilise les installateurs officiels presentes dans la section ["Installation"][installation]<!--
ignore
-->. Si vous avez installe Rust par un autre moyen, verifiez si Cargo est installe en entrant la commande suivante dans votre terminal :

<!--
```console
$ cargo --version
```
-->

```console
$ cargo --version
```

<!--
If you see a version number, you have it! If you see an error, such as `command
not found`, look at the documentation for your method of installation to
determine how to install Cargo separately.
-->

Si vous voyez un numero de version, vous l'avez ! Si vous voyez une erreur, comme `command not found`, consultez la documentation de votre methode d'installation pour determiner comment installer Cargo separement.

<!--
### Creating a Project with Cargo
-->

### Creer un projet avec Cargo

<!--
Let's create a new project using Cargo and look at how it differs from our
original "Hello, world!" project. Navigate back to your _projects_ directory
(or wherever you decided to store your code). Then, on any operating system,
run the following:
-->

Creons un nouveau projet en utilisant Cargo et voyons en quoi il differe de notre projet "Hello, world!" original. Retournez dans votre repertoire _projects_ (ou la ou vous avez decide de stocker votre code). Puis, sur n'importe quel systeme d'exploitation, executez la commande suivante :

<!--
```console
$ cargo new hello_cargo
$ cd hello_cargo
```
-->

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

<!--
The first command creates a new directory and project called _hello_cargo_.
We've named our project _hello_cargo_, and Cargo creates its files in a
directory of the same name.
-->

La premiere commande cree un nouveau repertoire et un projet appele _hello_cargo_. Nous avons nomme notre projet _hello_cargo_, et Cargo cree ses fichiers dans un repertoire du meme nom.

<!--
Go into the _hello_cargo_ directory and list the files. You'll see that Cargo
has generated two files and one directory for us: a _Cargo.toml_ file and a
_src_ directory with a _main.rs_ file inside.
-->

Entrez dans le repertoire _hello_cargo_ et listez les fichiers. Vous verrez que Cargo a genere deux fichiers et un repertoire pour nous : un fichier _Cargo.toml_ et un repertoire _src_ contenant un fichier _main.rs_.

<!--
It has also initialized a new Git repository along with a _.gitignore_ file.
Git files won't be generated if you run `cargo new` within an existing Git
repository; you can override this behavior by using `cargo new --vcs=git`.
-->

Il a egalement initialise un nouveau depot Git avec un fichier _.gitignore_. Les fichiers Git ne seront pas generes si vous executez `cargo new` dans un depot Git existant ; vous pouvez forcer ce comportement en utilisant `cargo new --vcs=git`.

<!--
> Note: Git is a common version control system. You can change `cargo new` to
> use a different version control system or no version control system by using
> the `--vcs` flag. Run `cargo new --help` to see the available options.
-->

> Note : Git est un systeme de controle de version courant. Vous pouvez configurer `cargo new` pour utiliser un autre systeme de controle de version ou aucun systeme de controle de version en utilisant le drapeau `--vcs`. Executez `cargo new --help` pour voir les options disponibles.

<!--
Open _Cargo.toml_ in your text editor of choice. It should look similar to the
code in Listing 1-2.
-->

Ouvrez _Cargo.toml_ dans l'editeur de texte de votre choix. Il devrait ressembler au code de l'Encadre 1-2.

<Listing number="1-2" file-name="Cargo.toml" caption="Contenu de *Cargo.toml* genere par `cargo new`">

<!--
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```
-->

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

<!--
This file is in the [_TOML_][toml] ignore
--> (_Tom's Obvious, Minimal
Language_) format, which is Cargo's configuration format.
-->

Ce fichier est au format [_TOML_][toml]<!--
ignore
--> (_Tom's Obvious, Minimal Language_), qui est le format de configuration de Cargo.

<!--
The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we'll add other sections.
-->

La premiere ligne, `[package]`, est un en-tete de section qui indique que les instructions suivantes configurent un package. Au fur et a mesure que nous ajouterons des informations a ce fichier, nous ajouterons d'autres sections.

<!--
The next three lines set the configuration information Cargo needs to compile
your program: the name, the version, and the edition of Rust to use. We'll talk
about the `edition` key in [Appendix E][appendix-e] ignore
-->.
-->

Les trois lignes suivantes definissent les informations de configuration dont Cargo a besoin pour compiler votre programme : le nom, la version et l'edition de Rust a utiliser. Nous parlerons de la cle `edition` dans l'[Annexe E][appendix-e]<!--
ignore
-->.

<!--
The last line, `[dependencies]`, is the start of a section for you to list any
of your project's dependencies. In Rust, packages of code are referred to as
_crates_. We won't need any other crates for this project, but we will in the
first project in Chapter 2, so we'll use this dependencies section then.
-->

La derniere ligne, `[dependencies]`, est le debut d'une section ou vous pouvez lister les dependances de votre projet. En Rust, les packages de code sont appeles des _crates_. Nous n'aurons besoin d'aucune autre crate pour ce projet, mais nous en aurons besoin dans le premier projet du Chapitre 2, et nous utiliserons alors cette section de dependances.

<!--
Now open _src/main.rs_ and take a look:
-->

Maintenant, ouvrez _src/main.rs_ et jetez-y un coup d'oeil :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Nom de fichier : src/main.rs</span>

<!--
```rust
fn main() {
    println!("Hello, world!");
}
```
-->

```rust
fn main() {
    println!("Hello, world!");
}
```

<!--
Cargo has generated a "Hello, world!" program for you, just like the one we
wrote in Listing 1-1! So far, the differences between our project and the
project Cargo generated are that Cargo placed the code in the _src_ directory,
and we have a _Cargo.toml_ configuration file in the top directory.
-->

Cargo a genere un programme "Hello, world!" pour vous, exactement comme celui que nous avons ecrit dans l'Encadre 1-1 ! Jusqu'ici, les differences entre notre projet et le projet genere par Cargo sont que Cargo a place le code dans le repertoire _src_, et que nous avons un fichier de configuration _Cargo.toml_ dans le repertoire racine.

<!--
Cargo expects your source files to live inside the _src_ directory. The
top-level project directory is just for README files, license information,
configuration files, and anything else not related to your code. Using Cargo
helps you organize your projects. There's a place for everything, and
everything is in its place.
-->

Cargo s'attend a ce que vos fichiers source se trouvent dans le repertoire _src_. Le repertoire racine du projet est reserve aux fichiers README, aux informations de licence, aux fichiers de configuration et a tout ce qui n'est pas lie a votre code. Utiliser Cargo vous aide a organiser vos projets. Il y a une place pour chaque chose, et chaque chose est a sa place.

<!--
If you started a project that doesn't use Cargo, as we did with the "Hello,
world!" project, you can convert it to a project that does use Cargo. Move the
project code into the _src_ directory and create an appropriate _Cargo.toml_
file. One easy way to get that _Cargo.toml_ file is to run `cargo init`, which
will create it for you automatically.
-->

Si vous avez demarre un projet qui n'utilise pas Cargo, comme nous l'avons fait avec le projet "Hello, world!", vous pouvez le convertir en un projet qui utilise Cargo. Deplacez le code du projet dans le repertoire _src_ et creez un fichier _Cargo.toml_ appropriate. Un moyen facile d'obtenir ce fichier _Cargo.toml_ est d'executer `cargo init`, qui le creera automatiquement pour vous.

<!--
### Building and Running a Cargo Project
-->

### Compiler et executer un projet Cargo

<!--
Now let's look at what's different when we build and run the "Hello, world!"
program with Cargo! From your _hello_cargo_ directory, build your project by
entering the following command:
-->

Voyons maintenant ce qui change lorsque nous compilons et executons le programme "Hello, world!" avec Cargo ! Depuis votre repertoire _hello_cargo_, compilez votre projet en entrant la commande suivante :

<!--
```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
-->

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

<!--
This command creates an executable file in _target/debug/hello_cargo_ (or
_target\debug\hello_cargo.exe_ on Windows) rather than in your current
directory. Because the default build is a debug build, Cargo puts the binary in
a directory named _debug_. You can run the executable with this command:
-->

Cette commande cree un fichier executable dans _target/debug/hello_cargo_ (ou _target\debug\hello_cargo.exe_ sous Windows) plutot que dans votre repertoire courant. Comme la compilation par defaut est une compilation de debogage, Cargo place le binaire dans un repertoire nomme _debug_. Vous pouvez executer l'executable avec cette commande :

<!--
```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```
-->

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

<!--
If all goes well, `Hello, world!` should print to the terminal. Running `cargo
build` for the first time also causes Cargo to create a new file at the top
level: _Cargo.lock_. This file keeps track of the exact versions of
dependencies in your project. This project doesn't have dependencies, so the
file is a bit sparse. You won't ever need to change this file manually; Cargo
manages its contents for you.
-->

Si tout se passe bien, `Hello, world!` devrait s'afficher dans le terminal. Executer `cargo build` pour la premiere fois amene egalement Cargo a creer un nouveau fichier a la racine : _Cargo.lock_. Ce fichier garde la trace des versions exactes des dependances de votre projet. Ce projet n'a pas de dependances, donc le fichier est un peu vide. Vous n'aurez jamais besoin de modifier ce fichier manuellement ; Cargo gere son contenu pour vous.

<!--
We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile the
code and then run the resultant executable all in one command:
-->

Nous venons de compiler un projet avec `cargo build` et de l'executer avec `./target/debug/hello_cargo`, mais nous pouvons aussi utiliser `cargo run` pour compiler le code puis executer l'executable resultant, le tout en une seule commande :

<!--
```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
-->

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

<!--
Using `cargo run` is more convenient than having to remember to run `cargo
build` and then use the whole path to the binary, so most developers use `cargo
run`.
-->

Utiliser `cargo run` est plus pratique que de devoir se souvenir d'executer `cargo build` puis d'utiliser le chemin complet vers le binaire, c'est pourquoi la plupart des developpeurs utilisent `cargo run`.

<!--
Notice that this time we didn't see output indicating that Cargo was compiling
`hello_cargo`. Cargo figured out that the files hadn't changed, so it didn't
rebuild but just ran the binary. If you had modified your source code, Cargo
would have rebuilt the project before running it, and you would have seen this
output:
-->

Remarquez que cette fois, nous n'avons pas vu de sortie indiquant que Cargo compilait `hello_cargo`. Cargo a determine que les fichiers n'avaient pas change, donc il n'a pas recompile mais a simplement execute le binaire. Si vous aviez modifie votre code source, Cargo aurait recompile le projet avant de l'executer, et vous auriez vu cette sortie :

<!--
```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
-->

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

<!--
Cargo also provides a command called `cargo check`. This command quickly checks
your code to make sure it compiles but doesn't produce an executable:
-->

Cargo fournit egalement une commande appelee `cargo check`. Cette commande verifie rapidement votre code pour s'assurer qu'il compile, mais ne produit pas d'executable :

<!--
```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
-->

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

<!--
Why would you not want an executable? Often, `cargo check` is much faster than
`cargo build` because it skips the step of producing an executable. If you're
continually checking your work while writing the code, using `cargo check` will
speed up the process of letting you know if your project is still compiling! As
such, many Rustaceans run `cargo check` periodically as they write their
program to make sure it compiles. Then, they run `cargo build` when they're
ready to use the executable.
-->

Pourquoi ne voudriez-vous pas d'executable ? Souvent, `cargo check` est beaucoup plus rapide que `cargo build` car il saute l'etape de production d'un executable. Si vous verifiez continuellement votre travail pendant que vous ecrivez le code, utiliser `cargo check` accelerera le processus pour savoir si votre projet compile toujours ! C'est pourquoi de nombreux Rustaceans executent `cargo check` periodiquement pendant qu'ils ecrivent leur programme pour s'assurer qu'il compile. Ensuite, ils executent `cargo build` lorsqu'ils sont prets a utiliser l'executable.

<!--
Let's recap what we've learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using
  `cargo check`.
- Instead of saving the result of the build in the same directory as our code,
  Cargo stores it in the _target/debug_ directory.
-->

Recapitulons ce que nous avons appris jusqu'ici sur Cargo :

- Nous pouvons creer un projet en utilisant `cargo new`.
- Nous pouvons compiler un projet en utilisant `cargo build`.
- Nous pouvons compiler et executer un projet en une seule etape en utilisant `cargo run`.
- Nous pouvons compiler un projet sans produire de binaire pour verifier les erreurs en utilisant `cargo check`.
- Au lieu de sauvegarder le resultat de la compilation dans le meme repertoire que notre code, Cargo le stocke dans le repertoire _target/debug_.

<!--
An additional advantage of using Cargo is that the commands are the same no
matter which operating system you're working on. So, at this point, we'll no
longer provide specific instructions for Linux and macOS versus Windows.
-->

Un avantage supplementaire de l'utilisation de Cargo est que les commandes sont les memes quel que soit le systeme d'exploitation sur lequel vous travaillez. Donc, a partir de maintenant, nous ne fournirons plus d'instructions specifiques pour Linux et macOS par rapport a Windows.

<!--
### Building for Release
-->

### Compiler pour la publication

<!--
When your project is finally ready for release, you can use `cargo build
--release` to compile it with optimizations. This command will create an
executable in _target/release_ instead of _target/debug_. The optimizations
make your Rust code run faster, but turning them on lengthens the time it takes
for your program to compile. This is why there are two different profiles: one
for development, when you want to rebuild quickly and often, and another for
building the final program you'll give to a user that won't be rebuilt
repeatedly and that will run as fast as possible. If you're benchmarking your
code's running time, be sure to run `cargo build --release` and benchmark with
the executable in _target/release_.
-->

Lorsque votre projet est enfin pret pour la publication, vous pouvez utiliser `cargo build --release` pour le compiler avec des optimisations. Cette commande creera un executable dans _target/release_ au lieu de _target/debug_. Les optimisations rendent votre code Rust plus rapide, mais les activer allonge le temps de compilation de votre programme. C'est pourquoi il y a deux profils differents : un pour le developpement, quand vous voulez recompiler rapidement et souvent, et un autre pour construire le programme final que vous donnerez a un utilisateur, qui ne sera pas recompile frequemment et qui s'executera aussi vite que possible. Si vous mesurez les performances de votre code, assurez-vous d'executer `cargo build --release` et de tester avec l'executable dans _target/release_.

<!--
Old headings. Do not remove or links may break.
-->
<a id="cargo-as-convention"></a>

<!--
### Leveraging Cargo's Conventions
-->

### Tirer parti des conventions de Cargo

<!--
With simple projects, Cargo doesn't provide a lot of value over just using
`rustc`, but it will prove its worth as your programs become more intricate.
Once programs grow to multiple files or need a dependency, it's much easier to
let Cargo coordinate the build.
-->

Pour les projets simples, Cargo n'apporte pas beaucoup de valeur ajoutee par rapport a la simple utilisation de `rustc`, mais il prouvera sa valeur a mesure que vos programmes deviendront plus complexes. Une fois que les programmes s'etendent a plusieurs fichiers ou necessitent une dependance, il est beaucoup plus facile de laisser Cargo coordonner la compilation.

<!--
Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you'll use in the rest of your Rust career. In fact, to work on any
existing projects, you can use the following commands to check out the code
using Git, change to that project's directory, and build:
-->

Meme si le projet `hello_cargo` est simple, il utilise deja une grande partie des outils reels que vous utiliserez tout au long de votre parcours avec Rust. En fait, pour travailler sur n'importe quel projet existant, vous pouvez utiliser les commandes suivantes pour recuperer le code avec Git, acceder au repertoire du projet et compiler :

<!--
```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```
-->

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

<!--
For more information about Cargo, check out [its documentation][cargo].
-->

Pour plus d'informations sur Cargo, consultez [sa documentation][cargo].

<!--
## Summary
-->

## Resume

<!--
You're already off to a great start on your Rust journey! In this chapter, you
learned how to:

- Install the latest stable version of Rust using `rustup`.
- Update to a newer Rust version.
- Open locally installed documentation.
- Write and run a "Hello, world!" program using `rustc` directly.
- Create and run a new project using the conventions of Cargo.
-->

Vous avez deja pris un excellent depart dans votre parcours Rust ! Dans ce chapitre, vous avez appris a :

- Installer la derniere version stable de Rust en utilisant `rustup`.
- Mettre a jour vers une version plus recente de Rust.
- Ouvrir la documentation installee localement.
- Ecrire et executer un programme "Hello, world!" en utilisant directement `rustc`.
- Creer et executer un nouveau projet en utilisant les conventions de Cargo.

<!--
This is a great time to build a more substantial program to get used to reading
and writing Rust code. So, in Chapter 2, we'll build a guessing game program.
If you would rather start by learning how common programming concepts work in
Rust, see Chapter 3 and then return to Chapter 2.
-->

C'est le moment ideal pour construire un programme plus consequent afin de vous habituer a lire et ecrire du code Rust. Ainsi, au Chapitre 2, nous construirons un programme de jeu de devinettes. Si vous preferez commencer par apprendre comment les concepts de programmation courants fonctionnent en Rust, consultez le Chapitre 3 puis revenez au Chapitre 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
