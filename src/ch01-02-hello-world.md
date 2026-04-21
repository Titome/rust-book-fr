<!--
## Hello, World!
-->

## Hello, World !

<!--
Now that you've installed Rust, it's time to write your first Rust program.
It's traditional when learning a new language to write a little program that
prints the text `Hello, world!` to the screen, so we'll do the same here!
-->

Maintenant que vous avez installe Rust, il est temps d'ecrire votre premier programme Rust. C'est une tradition lorsqu'on apprend un nouveau langage d'ecrire un petit programme qui affiche le texte `Hello, world!` a l'ecran, alors faisons de meme !

<!--
> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an IDE instead of the command line, feel free to use your
> favorite IDE. Many IDEs now have some degree of Rust support; check the IDE's
> documentation for details. The Rust team has been focusing on enabling great
> IDE support via `rust-analyzer`. See [Appendix D][devtools] ignore
-->
> for more details.
-->

> Note : ce livre suppose une familiarite basique avec la ligne de commande. Rust n'impose aucune exigence particuliere concernant votre editeur, vos outils ou l'emplacement de votre code. Si vous preferez utiliser un IDE plutot que la ligne de commande, n'hesitez pas a utiliser votre IDE prefere. De nombreux IDE offrent desormais un certain niveau de prise en charge de Rust ; consultez la documentation de l'IDE pour plus de details. L'equipe Rust s'est concentree sur l'activation d'une excellente prise en charge des IDE via `rust-analyzer`. Consultez l'[Annexe D][devtools]<!--
ignore
--> pour plus de details.

<!--
Old headings. Do not remove or links may break.
-->
<a id="creating-a-project-directory"></a>

<!--
### Project Directory Setup
-->

### Mise en place du repertoire du projet

<!--
You'll start by making a directory to store your Rust code. It doesn't matter
to Rust where your code lives, but for the exercises and projects in this book,
we suggest making a _projects_ directory in your home directory and keeping all
your projects there.
-->

Vous commencerez par creer un repertoire pour stocker votre code Rust. Peu importe pour Rust ou se trouve votre code, mais pour les exercices et projets de ce livre, nous suggerons de creer un repertoire _projects_ dans votre repertoire personnel et d'y conserver tous vos projets.

<!--
Open a terminal and enter the following commands to make a _projects_ directory
and a directory for the "Hello, world!" project within the _projects_ directory.
-->

Ouvrez un terminal et entrez les commandes suivantes pour creer un repertoire _projects_ et un repertoire pour le projet "Hello, world!" a l'interieur du repertoire _projects_.

<!--
For Linux, macOS, and PowerShell on Windows, enter this:
-->

Pour Linux, macOS et PowerShell sur Windows, entrez ceci :

<!--
```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```
-->

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

<!--
For Windows CMD, enter this:
-->

Pour le CMD Windows, entrez ceci :

<!--
```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```
-->

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!--
Old headings. Do not remove or links may break.
-->
<a id="writing-and-running-a-rust-program"></a>

<!--
### Rust Program Basics
-->

### Les bases d'un programme Rust

<!--
Next, make a new source file and call it _main.rs_. Rust files always end with
the _.rs_ extension. If you're using more than one word in your filename, the
convention is to use an underscore to separate them. For example, use
_hello_world.rs_ rather than _helloworld.rs_.
-->

Ensuite, creez un nouveau fichier source et appelez-le _main.rs_. Les fichiers Rust se terminent toujours par l'extension _.rs_. Si vous utilisez plusieurs mots dans votre nom de fichier, la convention est d'utiliser un underscore pour les separer. Par exemple, utilisez _hello_world.rs_ plutot que _helloworld.rs_.

<!--
Now open the _main.rs_ file you just created and enter the code in Listing 1-1.
-->

Maintenant, ouvrez le fichier _main.rs_ que vous venez de creer et entrez le code de l'Encadre 1-1.

<Listing number="1-1" file-name="main.rs" caption="Un programme qui affiche `Hello, world!`">

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

</Listing>

<!--
Save the file and go back to your terminal window in the
_~/projects/hello_world_ directory. On Linux or macOS, enter the following
commands to compile and run the file:
-->

Sauvegardez le fichier et retournez a votre fenetre de terminal dans le repertoire _~/projects/hello_world_. Sous Linux ou macOS, entrez les commandes suivantes pour compiler et executer le fichier :

<!--
```console
$ rustc main.rs
$ ./main
Hello, world!
```
-->

```console
$ rustc main.rs
$ ./main
Hello, world!
```

<!--
On Windows, enter the command `.\main` instead of `./main`:
-->

Sous Windows, entrez la commande `.\main` au lieu de `./main` :

<!--
```powershell
> rustc main.rs
> .\main
Hello, world!
```
-->

```powershell
> rustc main.rs
> .\main
Hello, world!
```

<!--
Regardless of your operating system, the string `Hello, world!` should print to
the terminal. If you don't see this output, refer back to the
["Troubleshooting"][troubleshooting] ignore
--> part of the Installation
section for ways to get help.
-->

Quel que soit votre systeme d'exploitation, la chaine `Hello, world!` devrait s'afficher dans le terminal. Si vous ne voyez pas cette sortie, reportez-vous a la partie ["Resolution de problemes"][troubleshooting]<!--
ignore
--> de la section Installation pour obtenir de l'aide.

<!--
If `Hello, world!` did print, congratulations! You've officially written a Rust
program. That makes you a Rust programmer—welcome!
-->

Si `Hello, world!` s'est bien affiche, felicitations ! Vous avez officiellement ecrit un programme Rust. Cela fait de vous un programmeur Rust -- bienvenue !

<!--
Old headings. Do not remove or links may break.
-->

<a id="anatomy-of-a-rust-program"></a>

<!--
### The Anatomy of a Rust Program
-->

### L'anatomie d'un programme Rust

<!--
Let's review this "Hello, world!" program in detail. Here's the first piece of
the puzzle:
-->

Examinons ce programme "Hello, world!" en detail. Voici le premier element du puzzle :

<!--
```rust
fn main() {

}
```
-->

```rust
fn main() {

}
```

<!--
These lines define a function named `main`. The `main` function is special: It
is always the first code that runs in every executable Rust program. Here, the
first line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses (`()`).
-->

Ces lignes definissent une fonction nommee `main`. La fonction `main` est speciale : c'est toujours le premier code qui s'execute dans tout programme Rust executable. Ici, la premiere ligne declare une fonction nommee `main` qui n'a pas de parametres et ne retourne rien. S'il y avait des parametres, ils iraient a l'interieur des parentheses (`()`).

<!--
The function body is wrapped in `{}`. Rust requires curly brackets around all
function bodies. It's good style to place the opening curly bracket on the same
line as the function declaration, adding one space in between.
-->

Le corps de la fonction est entoure par `{}`. Rust exige des accolades autour de tous les corps de fonctions. Il est de bon style de placer l'accolade ouvrante sur la meme ligne que la declaration de la fonction, en ajoutant un espace entre les deux.

<!--
> Note: If you want to stick to a standard style across Rust projects, you can
> use an automatic formatter tool called `rustfmt` to format your code in a
> particular style (more on `rustfmt` in
> [Appendix D][devtools] ignore
-->). The Rust team has included this tool
> with the standard Rust distribution, as `rustc` is, so it should already be
> installed on your computer!
-->

> Note : si vous souhaitez respecter un style standard dans vos projets Rust, vous pouvez utiliser un outil de formatage automatique appele `rustfmt` pour formater votre code dans un style particulier (plus d'informations sur `rustfmt` dans l'[Annexe D][devtools]<!--
ignore
-->). L'equipe Rust a inclus cet outil dans la distribution standard de Rust, tout comme `rustc`, il devrait donc deja etre installe sur votre ordinateur !

<!--
The body of the `main` function holds the following code:
-->

Le corps de la fonction `main` contient le code suivant :

<!--
```rust
println!("Hello, world!");
```
-->

```rust
println!("Hello, world!");
```

<!--
This line does all the work in this little program: It prints text to the
screen. There are three important details to notice here.
-->

Cette ligne fait tout le travail dans ce petit programme : elle affiche du texte a l'ecran. Il y a trois details importants a remarquer ici.

<!--
First, `println!` calls a Rust macro. If it had called a function instead, it
would be entered as `println` (without the `!`). Rust macros are a way to write
code that generates code to extend Rust syntax, and we'll discuss them in more
detail in [Chapter 20][ch20-macros] ignore
-->. For now, you just need to
know that using a `!` means that you're calling a macro instead of a normal
function and that macros don't always follow the same rules as functions.
-->

Premierement, `println!` appelle une macro Rust. Si c'etait un appel de fonction, cela s'ecrirait `println` (sans le `!`). Les macros Rust sont un moyen d'ecrire du code qui genere du code pour etendre la syntaxe de Rust, et nous en discuterons plus en detail au [Chapitre 20][ch20-macros]<!--
ignore
-->. Pour l'instant, vous devez simplement savoir que l'utilisation d'un `!` signifie que vous appelez une macro plutot qu'une fonction normale, et que les macros ne suivent pas toujours les memes regles que les fonctions.

<!--
Second, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.
-->

Deuxiemement, vous voyez la chaine `"Hello, world!"`. Nous passons cette chaine en argument a `println!`, et la chaine est affichee a l'ecran.

<!--
Third, we end the line with a semicolon (`;`), which indicates that this
expression is over, and the next one is ready to begin. Most lines of Rust code
end with a semicolon.
-->

Troisiemement, nous terminons la ligne par un point-virgule (`;`), qui indique que cette expression est terminee et que la suivante est prete a commencer. La plupart des lignes de code Rust se terminent par un point-virgule.

<!--
Old headings. Do not remove or links may break.
-->
<a id="compiling-and-running-are-separate-steps"></a>

<!--
### Compilation and Execution
-->

### Compilation et execution

<!--
You've just run a newly created program, so let's examine each step in the
process.
-->

Vous venez d'executer un programme nouvellement cree, alors examinons chaque etape du processus.

<!--
Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:
-->

Avant d'executer un programme Rust, vous devez le compiler en utilisant le compilateur Rust en entrant la commande `rustc` et en lui passant le nom de votre fichier source, comme ceci :

<!--
```console
$ rustc main.rs
```
-->

```console
$ rustc main.rs
```

<!--
If you have a C or C++ background, you'll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.
-->

Si vous avez une experience en C ou C++, vous remarquerez que c'est similaire a `gcc` ou `clang`. Apres une compilation reussie, Rust produit un executable binaire.

<!--
On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell:
-->

Sous Linux, macOS et PowerShell sur Windows, vous pouvez voir l'executable en entrant la commande `ls` dans votre terminal :

<!--
```console
$ ls
main  main.rs
```
-->

```console
$ ls
main  main.rs
```

<!--
On Linux and macOS, you'll see two files. With PowerShell on Windows, you'll
see the same three files that you would see using CMD. With CMD on Windows, you
would enter the following:
-->

Sous Linux et macOS, vous verrez deux fichiers. Avec PowerShell sur Windows, vous verrez les memes trois fichiers que vous verriez en utilisant CMD. Avec CMD sur Windows, vous entreriez la commande suivante :

<!--
```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```
-->

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

<!--
This shows the source code file with the _.rs_ extension, the executable file
(_main.exe_ on Windows, but _main_ on all other platforms), and, when using
Windows, a file containing debugging information with the _.pdb_ extension.
From here, you run the _main_ or _main.exe_ file, like this:
-->

Cela montre le fichier de code source avec l'extension _.rs_, le fichier executable (_main.exe_ sous Windows, mais _main_ sur toutes les autres plateformes), et, sous Windows, un fichier contenant des informations de debogage avec l'extension _.pdb_. A partir de la, vous executez le fichier _main_ ou _main.exe_, comme ceci :

<!--
```console
$ ./main # or .\main on Windows
```
-->

```console
$ ./main # or .\main on Windows
```

<!--
If your _main.rs_ is your "Hello, world!" program, this line prints `Hello,
world!` to your terminal.
-->

Si votre _main.rs_ est votre programme "Hello, world!", cette ligne affiche `Hello, world!` dans votre terminal.

<!--
If you're more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an _ahead-of-time compiled_ language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a _.rb_, _.py_, or
_.js_ file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.
-->

Si vous etes plus familier avec un langage dynamique, comme Ruby, Python ou JavaScript, vous n'etes peut-etre pas habitue a compiler et executer un programme en deux etapes separees. Rust est un langage _compile a l'avance_ (ahead-of-time compiled), ce qui signifie que vous pouvez compiler un programme et donner l'executable a quelqu'un d'autre, qui pourra l'executer meme sans avoir Rust installe. Si vous donnez a quelqu'un un fichier _.rb_, _.py_ ou _.js_, cette personne doit avoir une implementation de Ruby, Python ou JavaScript installee (respectivement). Mais dans ces langages, vous n'avez besoin que d'une seule commande pour compiler et executer votre programme. Tout est un compromis dans la conception des langages.

<!--
Just compiling with `rustc` is fine for simple programs, but as your project
grows, you'll want to manage all the options and make it easy to share your
code. Next, we'll introduce you to the Cargo tool, which will help you write
real-world Rust programs.
-->

Compiler simplement avec `rustc` convient pour les programmes simples, mais a mesure que votre projet grandit, vous voudrez gerer toutes les options et faciliter le partage de votre code. Ensuite, nous vous presenterons l'outil Cargo, qui vous aidera a ecrire des programmes Rust concrets.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
