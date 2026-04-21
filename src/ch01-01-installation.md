<!--
## Installation
-->

## Installation

<!--
The first step is to install Rust. We'll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You'll need
an internet connection for the download.
-->

La premiere etape est d'installer Rust. Nous telechargerons Rust via `rustup`, un outil en ligne de commande permettant de gerer les versions de Rust et les outils associes. Vous aurez besoin d'une connexion internet pour le telechargement.

<!--
> Note: If you prefer not to use `rustup` for some reason, please see the
> [Other Rust Installation Methods page][otherinstall] for more options.
-->

> Note : si vous preferez ne pas utiliser `rustup` pour une raison quelconque, veuillez consulter la [page des autres methodes d'installation de Rust][otherinstall] pour plus d'options.

<!--
The following steps install the latest stable version of the Rust compiler.
Rust's stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions because Rust often improves error messages and
warnings. In other words, any newer, stable version of Rust you install using
these steps should work as expected with the content of this book.
-->

Les etapes suivantes installent la derniere version stable du compilateur Rust. Les garanties de stabilite de Rust assurent que tous les exemples du livre qui compilent continueront a compiler avec les versions plus recentes de Rust. La sortie peut varier legerement d'une version a l'autre, car Rust ameliore souvent les messages d'erreur et les avertissements. Autrement dit, toute version stable plus recente de Rust que vous installez en suivant ces etapes devrait fonctionner comme prevu avec le contenu de ce livre.

<!--
> ### Command Line Notation
>
> In this chapter and throughout the book, we'll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don't need to type the `$` character; it's the command line prompt shown to
> indicate the start of each command. Lines that don't start with `$` typically
> show the output of the previous command. Additionally, PowerShell-specific
> examples will use `>` rather than `$`.
-->

> ### Notation de la ligne de commande
>
> Dans ce chapitre et tout au long du livre, nous montrerons certaines commandes utilisees dans le terminal. Les lignes que vous devez saisir dans un terminal commencent toutes par `$`. Vous n'avez pas besoin de taper le caractere `$` ; c'est l'invite de la ligne de commande affichee pour indiquer le debut de chaque commande. Les lignes qui ne commencent pas par `$` affichent generalement la sortie de la commande precedente. De plus, les exemples specifiques a PowerShell utiliseront `>` au lieu de `$`.

<!--
### Installing `rustup` on Linux or macOS
-->

### Installer `rustup` sur Linux ou macOS

<!--
If you're using Linux or macOS, open a terminal and enter the following command:
-->

Si vous utilisez Linux ou macOS, ouvrez un terminal et entrez la commande suivante :

<!--
```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
-->

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

<!--
The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:
-->

La commande telecharge un script et lance l'installation de l'outil `rustup`, qui installe la derniere version stable de Rust. Il se peut que votre mot de passe vous soit demande. Si l'installation reussit, la ligne suivante apparaitra :

<!--
```text
Rust is installed now. Great!
```
-->

```text
Rust is installed now. Great!
```

<!--
You will also need a _linker_, which is a program that Rust uses to join its
compiled outputs into one file. It is likely you already have one. If you get
linker errors, you should install a C compiler, which will typically include a
linker. A C compiler is also useful because some common Rust packages depend on
C code and will need a C compiler.
-->

Vous aurez egalement besoin d'un _linker_ (editeur de liens), qui est un programme que Rust utilise pour regrouper ses sorties compilees en un seul fichier. Il est probable que vous en ayez deja un. Si vous obtenez des erreurs de linker, vous devriez installer un compilateur C, qui inclura generalement un editeur de liens. Un compilateur C est egalement utile car certains packages Rust courants dependent de code C et necessitent un compilateur C.

<!--
On macOS, you can get a C compiler by running:
-->

Sur macOS, vous pouvez obtenir un compilateur C en executant :

<!--
```console
$ xcode-select --install
```
-->

```console
$ xcode-select --install
```

<!--
Linux users should generally install GCC or Clang, according to their
distribution's documentation. For example, if you use Ubuntu, you can install
the `build-essential` package.
-->

Les utilisateurs Linux devraient generalement installer GCC ou Clang, conformement a la documentation de leur distribution. Par exemple, si vous utilisez Ubuntu, vous pouvez installer le paquet `build-essential`.

<!--
### Installing `rustup` on Windows
-->

### Installer `rustup` sur Windows

<!--
On Windows, go to [https://www.rust-lang.org/tools/install][install] ignore
--> and follow the instructions for installing Rust. At some point in the
installation, you'll be prompted to install Visual Studio. This provides a
linker and the native libraries needed to compile programs. If you need more
help with this step, see
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore
-->.
-->

Sur Windows, rendez-vous sur [https://www.rust-lang.org/tools/install][install]<!--
ignore
--> et suivez les instructions pour installer Rust. A un moment de l'installation, il vous sera demande d'installer Visual Studio. Celui-ci fournit un editeur de liens et les bibliotheques natives necessaires pour compiler les programmes. Si vous avez besoin d'aide supplementaire pour cette etape, consultez [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore
-->.

<!--
The rest of this book uses commands that work in both _cmd.exe_ and PowerShell.
If there are specific differences, we'll explain which to use.
-->

Le reste de ce livre utilise des commandes qui fonctionnent aussi bien dans _cmd.exe_ que dans PowerShell. S'il y a des differences specifiques, nous expliquerons laquelle utiliser.

<!--
### Troubleshooting
-->

### Resolution de problemes

<!--
To check whether you have Rust installed correctly, open a shell and enter this
line:
-->

Pour verifier que Rust est correctement installe, ouvrez un terminal et entrez cette ligne :

<!--
```console
$ rustc --version
```
-->

```console
$ rustc --version
```

<!--
You should see the version number, commit hash, and commit date for the latest
stable version that has been released, in the following format:
-->

Vous devriez voir le numero de version, le hash du commit et la date du commit pour la derniere version stable publiee, dans le format suivant :

<!--
```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```
-->

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

<!--
If you see this information, you have installed Rust successfully! If you don't
see this information, check that Rust is in your `%PATH%` system variable as
follows.
-->

Si vous voyez cette information, vous avez installe Rust avec succes ! Si vous ne voyez pas cette information, verifiez que Rust se trouve dans votre variable systeme `%PATH%` comme suit.

<!--
In Windows CMD, use:
-->

Dans le CMD Windows, utilisez :

<!--
```console
> echo %PATH%
```
-->

```console
> echo %PATH%
```

<!--
In PowerShell, use:
-->

Dans PowerShell, utilisez :

<!--
```powershell
> echo $env:Path
```
-->

```powershell
> echo $env:Path
```

<!--
In Linux and macOS, use:
-->

Sous Linux et macOS, utilisez :

<!--
```console
$ echo $PATH
```
-->

```console
$ echo $PATH
```

<!--
If that's all correct and Rust still isn't working, there are a number of
places you can get help. Find out how to get in touch with other Rustaceans (a
silly nickname we call ourselves) on [the community page][community].
-->

Si tout est correct et que Rust ne fonctionne toujours pas, il existe plusieurs endroits ou vous pouvez obtenir de l'aide. Decouvrez comment entrer en contact avec d'autres Rustaceans (un surnom amusant que nous nous donnons) sur [la page de la communaute][community].

<!--
### Updating and Uninstalling
-->

### Mise a jour et desinstallation

<!--
Once Rust is installed via `rustup`, updating to a newly released version is
easy. From your shell, run the following update script:
-->

Une fois Rust installe via `rustup`, la mise a jour vers une nouvelle version est simple. Depuis votre terminal, executez le script de mise a jour suivant :

<!--
```console
$ rustup update
```
-->

```console
$ rustup update
```

<!--
To uninstall Rust and `rustup`, run the following uninstall script from your
shell:
-->

Pour desinstaller Rust et `rustup`, executez le script de desinstallation suivant depuis votre terminal :

<!--
```console
$ rustup self uninstall
```
-->

```console
$ rustup self uninstall
```

<!--
Old headings. Do not remove or links may break.
-->
<a id="local-documentation"></a>

<!--
### Reading the Local Documentation
-->

### Lire la documentation locale

<!--
The installation of Rust also includes a local copy of the documentation so
that you can read it offline. Run `rustup doc` to open the local documentation
in your browser.
-->

L'installation de Rust inclut egalement une copie locale de la documentation afin que vous puissiez la lire hors ligne. Executez `rustup doc` pour ouvrir la documentation locale dans votre navigateur.

<!--
Any time a type or function is provided by the standard library and you're not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
-->

Chaque fois qu'un type ou une fonction est fourni par la bibliotheque standard et que vous n'etes pas sur de ce qu'il fait ou comment l'utiliser, consultez la documentation de l'interface de programmation applicative (API) pour le decouvrir !

<!--
Old headings. Do not remove or links may break.
-->
<a id="text-editors-and-integrated-development-environments"></a>

<!--
### Using Text Editors and IDEs
-->

### Utiliser des editeurs de texte et des IDE

<!--
This book makes no assumptions about what tools you use to author Rust code.
Just about any text editor will get the job done! However, many text editors and
integrated development environments (IDEs) have built-in support for Rust. You
can always find a fairly current list of many editors and IDEs on [the tools
page][tools] on the Rust website.
-->

Ce livre ne fait aucune hypothese sur les outils que vous utilisez pour ecrire du code Rust. Presque n'importe quel editeur de texte fera l'affaire ! Cependant, de nombreux editeurs de texte et environnements de developpement integres (IDE) offrent une prise en charge integree de Rust. Vous pouvez toujours trouver une liste assez a jour de nombreux editeurs et IDE sur [la page des outils][tools] du site web de Rust.

<!--
### Working Offline with This Book
-->

### Travailler hors ligne avec ce livre

<!--
In several examples, we will use Rust packages beyond the standard library. To
work through those examples, you will either need to have an internet connection
or to have downloaded those dependencies ahead of time. To download the
dependencies ahead of time, you can run the following commands. (We'll explain
what `cargo` is and what each of these commands does in detail later.)
-->

Dans plusieurs exemples, nous utiliserons des packages Rust au-dela de la bibliotheque standard. Pour travailler sur ces exemples, vous aurez besoin soit d'une connexion internet, soit d'avoir telecharge ces dependances a l'avance. Pour telecharger les dependances a l'avance, vous pouvez executer les commandes suivantes. (Nous expliquerons ce qu'est `cargo` et ce que fait chacune de ces commandes en detail plus tard.)

<!--
```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```
-->

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

<!--
This will cache the downloads for these packages so you will not need to
download them later. Once you have run this command, you do not need to keep the
`get-dependencies` folder. If you have run this command, you can use the
`--offline` flag with all `cargo` commands in the rest of the book to use these
cached versions instead of attempting to use the network.
-->

Cela mettra en cache les telechargements de ces packages afin que vous n'ayez pas a les telecharger plus tard. Une fois cette commande executee, vous n'avez pas besoin de conserver le dossier `get-dependencies`. Si vous avez execute cette commande, vous pouvez utiliser le drapeau `--offline` avec toutes les commandes `cargo` dans le reste du livre pour utiliser ces versions en cache au lieu de tenter d'utiliser le reseau.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
