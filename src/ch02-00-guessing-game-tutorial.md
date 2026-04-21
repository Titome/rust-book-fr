<!--
# Programming a Guessing Game
-->

# Programmer un jeu de devinettes

<!--
Let's jump into Rust by working through a hands-on project together! This
chapter introduces you to a few common Rust concepts by showing you how to use
them in a real program. You'll learn about `let`, `match`, methods, associated
functions, external crates, and more! In the following chapters, we'll explore
these ideas in more detail. In this chapter, you'll just practice the
fundamentals.
-->

Plongeons dans Rust en travaillant ensemble sur un projet concret ! Ce chapitre
vous présente quelques concepts courants de Rust en vous montrant comment les
utiliser dans un vrai programme. Vous découvrirez `let`, `match`, les méthodes,
les fonctions associées, les crates externes, et bien plus ! Dans les chapitres
suivants, nous explorerons ces notions plus en détail. Dans ce chapitre, vous
pratiquerez simplement les bases.

<!--
We'll implement a classic beginner programming problem: a guessing game. Here's
how it works: The program will generate a random integer between 1 and 100. It
will then prompt the player to enter a guess. After a guess is entered, the
program will indicate whether the guess is too low or too high. If the guess is
correct, the game will print a congratulatory message and exit.
-->

Nous allons implémenter un problème classique de programmation pour débutants :
un jeu de devinettes. Voici comment il fonctionne : le programme génère un
nombre entier aléatoire entre 1 et 100. Il demande ensuite au joueur de saisir
une proposition. Après chaque saisie, le programme indique si la proposition
est trop basse ou trop haute. Si la proposition est correcte, le jeu affiche un
message de félicitations et se termine.

<!--
## Setting Up a New Project
-->

## Mise en place d'un nouveau projet

<!--
To set up a new project, go to the _projects_ directory that you created in
Chapter 1 and make a new project using Cargo, like so:
-->

Pour mettre en place un nouveau projet, rendez-vous dans le répertoire
_projects_ que vous avez créé au chapitre 1 et créez un nouveau projet avec
Cargo, comme ceci :

<!--
```console
$ cargo new guessing_game
$ cd guessing_game
```
-->

```console
$ cargo new guessing_game
$ cd guessing_game
```

<!--
The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The second command changes to the new project's
directory.
-->

La première commande, `cargo new`, prend le nom du projet (`guessing_game`)
comme premier argument. La seconde commande se déplace dans le répertoire du
nouveau projet.

<!--
Look at the generated _Cargo.toml_ file:
-->

Regardons le fichier _Cargo.toml_ généré :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Fichier : Cargo.toml</span>


```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

<!--
As you saw in Chapter 1, `cargo new` generates a "Hello, world!" program for
you. Check out the _src/main.rs_ file:
-->

Comme vous l'avez vu au chapitre 1, `cargo new` génère un programme
"Hello, world!" pour vous. Regardons le fichier _src/main.rs_ :

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

<!--
Now let's compile this "Hello, world!" program and run it in the same step
using the `cargo run` command:
-->

Maintenant, compilons ce programme "Hello, world!" et exécutons-le en une seule
étape avec la commande `cargo run` :


```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

<!--
The `run` command comes in handy when you need to rapidly iterate on a project,
as we'll do in this game, quickly testing each iteration before moving on to
the next one.
-->

La commande `run` est pratique lorsque vous avez besoin d'itérer rapidement sur
un projet, comme nous le ferons dans ce jeu, en testant rapidement chaque
itération avant de passer à la suivante.

<!--
Reopen the _src/main.rs_ file. You'll be writing all the code in this file.
-->

Rouvrez le fichier _src/main.rs_. Vous écrirez tout le code dans ce fichier.

<!--
## Processing a Guess
-->

## Traitement d'une proposition

<!--
The first part of the guessing game program will ask for user input, process
that input, and check that the input is in the expected form. To start, we'll
allow the player to input a guess. Enter the code in Listing 2-1 into
_src/main.rs_.
-->

La première partie du programme du jeu de devinettes demandera une saisie à
l'utilisateur, traitera cette saisie et vérifiera qu'elle est dans la forme
attendue. Pour commencer, nous allons permettre au joueur de saisir une
proposition. Entrez le code de l'encart 2-1 dans _src/main.rs_.

<Listing number="2-1" file-name="src/main.rs" caption="Code qui récupère une proposition de l'utilisateur et l'affiche">


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

<!--
This code contains a lot of information, so let's go over it line by line. To
obtain user input and then print the result as output, we need to bring the
`io` input/output library into scope. The `io` library comes from the standard
library, known as `std`:
-->

Ce code contient beaucoup d'informations, alors parcourons-le ligne par ligne.
Pour obtenir la saisie de l'utilisateur puis afficher le résultat en sortie,
nous devons importer la bibliothèque d'entrée/sortie `io` dans la portée. La
bibliothèque `io` provient de la bibliothèque standard, connue sous le nom de
`std` :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

<!--
By default, Rust has a set of items defined in the standard library that it
brings into the scope of every program. This set is called the _prelude_, and
you can see everything in it [in the standard library documentation][prelude].
-->

Par défaut, Rust dispose d'un ensemble d'éléments définis dans la bibliothèque
standard qu'il importe dans la portée de chaque programme. Cet ensemble
s'appelle le _prelude_, et vous pouvez voir tout ce qu'il contient [dans la
documentation de la bibliothèque standard][prelude].

<!--
If a type you want to use isn't in the prelude, you have to bring that type
into scope explicitly with a `use` statement. Using the `std::io` library
provides you with a number of useful features, including the ability to accept
user input.
-->

Si un type que vous souhaitez utiliser n'est pas dans le prelude, vous devez
l'importer explicitement dans la portée avec une instruction `use`. Utiliser la
bibliothèque `std::io` vous fournit un certain nombre de fonctionnalités
utiles, notamment la possibilité d'accepter des saisies utilisateur.

<!--
As you saw in Chapter 1, the `main` function is the entry point into the
program:
-->

Comme vous l'avez vu au chapitre 1, la fonction `main` est le point d'entrée
du programme :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

<!--
The `fn` syntax declares a new function; the parentheses, `()`, indicate there
are no parameters; and the curly bracket, `{`, starts the body of the function.
-->

La syntaxe `fn` déclare une nouvelle fonction ; les parenthèses, `()`,
indiquent qu'il n'y a pas de paramètres ; et l'accolade ouvrante, `{`, commence
le corps de la fonction.

<!--
As you also learned in Chapter 1, `println!` is a macro that prints a string to
the screen:
-->

Comme vous l'avez également appris au chapitre 1, `println!` est une macro qui
affiche une chaîne de caractères à l'écran :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

<!--
This code is printing a prompt stating what the game is and requesting input
from the user.
-->

Ce code affiche un message indiquant de quoi il s'agit et demande une saisie à
l'utilisateur.

<!--
### Storing Values with Variables
-->

### Stocker des valeurs avec des variables

<!--
Next, we'll create a _variable_ to store the user input, like this:
-->

Ensuite, nous allons créer une _variable_ pour stocker la saisie de
l'utilisateur, comme ceci :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

<!--
Now the program is getting interesting! There's a lot going on in this little
line. We use the `let` statement to create the variable. Here's another example:
-->

Maintenant le programme devient intéressant ! Il se passe beaucoup de choses
dans cette petite ligne. Nous utilisons l'instruction `let` pour créer la
variable. Voici un autre exemple :

<!--
```rust,ignore
let apples = 5;
```
-->

```rust,ignore
let apples = 5;
```

<!--
This line creates a new variable named `apples` and binds it to the value `5`.
In Rust, variables are immutable by default, meaning once we give the variable
a value, the value won't change. We'll be discussing this concept in detail in
the ["Variables and Mutability"][variables-and-mutability] ignore
-->
section in Chapter 3. To make a variable mutable, we add `mut` before the
variable name:
-->

Cette ligne crée une nouvelle variable nommée `apples` et la lie à la valeur
`5`. En Rust, les variables sont immuables par défaut, ce qui signifie qu'une
fois que nous avons donné une valeur à la variable, la valeur ne changera pas.
Nous discuterons de ce concept en détail dans la section
["Variables et mutabilité"][variables-and-mutability]<!--
ignore
--> du
chapitre 3. Pour rendre une variable mutable, nous ajoutons `mut` devant le nom
de la variable :

<!--
```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
-->

```rust,ignore
let apples = 5; // immuable
let mut bananas = 5; // mutable
```

<!--
> Note: The `//` syntax starts a comment that continues until the end of the
> line. Rust ignores everything in comments. We'll discuss comments in more
> detail in [Chapter 3][comments] ignore
-->.
-->

> Remarque : la syntaxe `//` commence un commentaire qui se poursuit jusqu'à la
> fin de la ligne. Rust ignore tout ce qui se trouve dans les commentaires. Nous
> discuterons des commentaires plus en détail au
> [chapitre 3][comments]<!--
ignore
-->.

<!--
Returning to the guessing game program, you now know that `let mut guess` will
introduce a mutable variable named `guess`. The equal sign (`=`) tells Rust we
want to bind something to the variable now. On the right of the equal sign is
the value that `guess` is bound to, which is the result of calling
`String::new`, a function that returns a new instance of a `String`.
[`String`][string] ignore
--> is a string type provided by the standard
library that is a growable, UTF-8 encoded bit of text.
-->

En revenant au programme du jeu de devinettes, vous savez maintenant que
`let mut guess` introduit une variable mutable nommée `guess`. Le signe égal
(`=`) indique à Rust que nous voulons lier quelque chose à la variable
maintenant. À droite du signe égal se trouve la valeur à laquelle `guess` est
liée, qui est le résultat de l'appel à `String::new`, une fonction qui
renvoie une nouvelle instance de `String`. [`String`][string]<!--
ignore
-->
est un type de chaîne de caractères fourni par la bibliothèque standard, qui
est un texte extensible encodé en UTF-8.

<!--
The `::` syntax in the `::new` line indicates that `new` is an associated
function of the `String` type. An _associated function_ is a function that's
implemented on a type, in this case `String`. This `new` function creates a
new, empty string. You'll find a `new` function on many types because it's a
common name for a function that makes a new value of some kind.
-->

La syntaxe `::` dans la ligne `::new` indique que `new` est une fonction
associée du type `String`. Une _fonction associée_ est une fonction qui est
implémentée sur un type, dans ce cas `String`. Cette fonction `new` crée une
nouvelle chaîne de caractères vide. Vous trouverez une fonction `new` sur de
nombreux types car c'est un nom courant pour une fonction qui crée une nouvelle
valeur d'un certain type.

<!--
In full, the `let mut guess = String::new();` line has created a mutable
variable that is currently bound to a new, empty instance of a `String`. Whew!
-->

En résumé, la ligne `let mut guess = String::new();` a créé une variable
mutable qui est actuellement liée à une nouvelle instance vide de `String`.
Ouf !

<!--
### Receiving User Input
-->

### Recevoir la saisie de l'utilisateur

<!--
Recall that we included the input/output functionality from the standard
library with `use std::io;` on the first line of the program. Now we'll call
the `stdin` function from the `io` module, which will allow us to handle user
input:
-->

Rappelons que nous avons inclus la fonctionnalité d'entrée/sortie de la
bibliothèque standard avec `use std::io;` sur la première ligne du programme.
Maintenant, nous allons appeler la fonction `stdin` du module `io`, qui nous
permettra de gérer la saisie de l'utilisateur :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

<!--
If we hadn't imported the `io` module with `use std::io;` at the beginning of
the program, we could still use the function by writing this function call as
`std::io::stdin`. The `stdin` function returns an instance of
[`std::io::Stdin`][iostdin] ignore
-->, which is a type that represents a
handle to the standard input for your terminal.
-->

Si nous n'avions pas importé le module `io` avec `use std::io;` au début du
programme, nous pourrions quand même utiliser la fonction en écrivant l'appel
comme `std::io::stdin`. La fonction `stdin` renvoie une instance de
[`std::io::Stdin`][iostdin]<!--
ignore
-->, qui est un type représentant un
descripteur vers l'entrée standard de votre terminal.

<!--
Next, the line `.read_line(&mut guess)` calls the [`read_line`][read_line]
ignore
--> method on the standard input handle to get input from the user.
We're also passing `&mut guess` as the argument to `read_line` to tell it what
string to store the user input in. The full job of `read_line` is to take
whatever the user types into standard input and append that into a string
(without overwriting its contents), so we therefore pass that string as an
argument. The string argument needs to be mutable so that the method can change
the string's content.
-->

Ensuite, la ligne `.read_line(&mut guess)` appelle la méthode
[`read_line`][read_line]<!--
ignore
--> sur le descripteur d'entrée standard
pour obtenir la saisie de l'utilisateur. Nous passons également `&mut guess`
comme argument à `read_line` pour lui indiquer dans quelle chaîne stocker la
saisie de l'utilisateur. Le rôle complet de `read_line` est de prendre tout ce
que l'utilisateur tape dans l'entrée standard et de l'ajouter à la fin d'une
chaîne (sans écraser son contenu), c'est pourquoi nous passons cette chaîne en
argument. L'argument chaîne doit être mutable pour que la méthode puisse
modifier le contenu de la chaîne.

<!--
The `&` indicates that this argument is a _reference_, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times. References are a complex feature,
and one of Rust's major advantages is how safe and easy it is to use
references. You don't need to know a lot of those details to finish this
program. For now, all you need to know is that, like variables, references are
immutable by default. Hence, you need to write `&mut guess` rather than
`&guess` to make it mutable. (Chapter 4 will explain references more
thoroughly.)
-->

Le `&` indique que cet argument est une _référence_, qui vous permet de laisser
plusieurs parties de votre code accéder à une même donnée sans avoir besoin de
copier cette donnée en mémoire plusieurs fois. Les références sont une
fonctionnalité complexe, et l'un des avantages majeurs de Rust est la sécurité
et la facilité d'utilisation des références. Vous n'avez pas besoin de
connaître tous ces détails pour terminer ce programme. Pour l'instant, tout ce
que vous devez savoir, c'est que, comme les variables, les références sont
immuables par défaut. Par conséquent, vous devez écrire `&mut guess` plutôt que
`&guess` pour la rendre mutable. (Le chapitre 4 expliquera les références plus
en détail.)

<!--
Old headings. Do not remove or links may break.
-->

<a id="handling-potential-failure-with-the-result-type"></a>

<!--
### Handling Potential Failure with `Result`
-->

### Gérer les erreurs potentielles avec `Result`

<!--
We're still working on this line of code. We're now discussing a third line of
text, but note that it's still part of a single logical line of code. The next
part is this method:
-->

Nous travaillons toujours sur cette ligne de code. Nous discutons maintenant
d'une troisième ligne de texte, mais notez qu'elle fait toujours partie d'une
seule ligne logique de code. La partie suivante est cette méthode :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

<!--
We could have written this code as:
-->

Nous aurions pu écrire ce code ainsi :

<!--
```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```
-->

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

<!--
However, one long line is difficult to read, so it's best to divide it. It's
often wise to introduce a newline and other whitespace to help break up long
lines when you call a method with the `.method_name()` syntax. Now let's
discuss what this line does.
-->

Cependant, une seule longue ligne est difficile à lire, il est donc préférable
de la diviser. Il est souvent judicieux d'introduire un retour à la ligne et
d'autres espaces pour aérer les longues lignes lorsque vous appelez une méthode
avec la syntaxe `.nom_de_methode()`. Voyons maintenant ce que fait cette ligne.

<!--
As mentioned earlier, `read_line` puts whatever the user enters into the string
we pass to it, but it also returns a `Result` value. [`Result`][result]
ignore
--> is an [_enumeration_][enums]<!--
ignore
-->, often called an _enum_,
which is a type that can be in one of multiple possible states. We call each
possible state a _variant_.
-->

Comme mentionné précédemment, `read_line` place ce que l'utilisateur saisit
dans la chaîne que nous lui passons, mais elle renvoie également une valeur
`Result`. [`Result`][result]<!--
ignore
--> est une
[_énumération_][enums]<!--
ignore
-->, souvent appelée _enum_, qui est un type
pouvant se trouver dans l'un de plusieurs états possibles. Nous appelons chaque
état possible une _variante_.

<!--
[Chapter 6][enums] ignore
--> will cover enums in more detail. The purpose
of these `Result` types is to encode error-handling information.
-->

Le [chapitre 6][enums]<!--
ignore
--> couvrira les enums plus en détail.
L'objectif de ces types `Result` est d'encoder les informations de gestion
d'erreurs.

<!--
`Result`'s variants are `Ok` and `Err`. The `Ok` variant indicates the
operation was successful, and it contains the successfully generated value.
The `Err` variant means the operation failed, and it contains information
about how or why the operation failed.
-->

Les variantes de `Result` sont `Ok` et `Err`. La variante `Ok` indique que
l'opération a réussi et contient la valeur produite avec succès. La variante
`Err` signifie que l'opération a échoué et contient des informations sur la
manière ou la raison de l'échec.

<!--
Values of the `Result` type, like values of any type, have methods defined on
them. An instance of `Result` has an [`expect` method][expect] ignore
-->
that you can call. If this instance of `Result` is an `Err` value, `expect`
will cause the program to crash and display the message that you passed as an
argument to `expect`. If the `read_line` method returns an `Err`, it would
likely be the result of an error coming from the underlying operating system.
If this instance of `Result` is an `Ok` value, `expect` will take the return
value that `Ok` is holding and return just that value to you so that you can
use it. In this case, that value is the number of bytes in the user's input.
-->

Les valeurs du type `Result`, comme les valeurs de tout type, ont des méthodes
définies sur elles. Une instance de `Result` possède une
[méthode `expect`][expect]<!--
ignore
--> que vous pouvez appeler. Si cette
instance de `Result` est une valeur `Err`, `expect` provoquera le plantage du
programme et affichera le message que vous avez passé en argument à `expect`. Si
la méthode `read_line` renvoie un `Err`, il s'agirait probablement d'une erreur
provenant du système d'exploitation sous-jacent. Si cette instance de `Result`
est une valeur `Ok`, `expect` prendra la valeur de retour contenue dans `Ok` et
vous renverra uniquement cette valeur afin que vous puissiez l'utiliser. Dans ce
cas, cette valeur est le nombre d'octets dans la saisie de l'utilisateur.

<!--
If you don't call `expect`, the program will compile, but you'll get a warning:
-->

Si vous n'appelez pas `expect`, le programme compilera, mais vous obtiendrez un
avertissement :


```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

<!--
Rust warns that you haven't used the `Result` value returned from `read_line`,
indicating that the program hasn't handled a possible error.
-->

Rust vous avertit que vous n'avez pas utilisé la valeur `Result` renvoyée par
`read_line`, indiquant que le programme n'a pas géré une erreur possible.

<!--
The right way to suppress the warning is to actually write error-handling code,
but in our case we just want to crash this program when a problem occurs, so we
can use `expect`. You'll learn about recovering from errors in [Chapter
9][recover] ignore
-->.
-->

La bonne façon de supprimer l'avertissement est d'écrire réellement du code de
gestion d'erreurs, mais dans notre cas, nous voulons simplement que le programme
plante lorsqu'un problème survient, donc nous pouvons utiliser `expect`. Vous
apprendrez à récupérer des erreurs au [chapitre 9][recover]<!--
ignore
-->.

<!--
### Printing Values with `println!` Placeholders
-->

### Afficher des valeurs avec les espaces réservés de `println!`

<!--
Aside from the closing curly bracket, there's only one more line to discuss in
the code so far:
-->

Mis à part l'accolade fermante, il ne reste qu'une seule ligne à discuter dans
le code jusqu'ici :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

<!--
This line prints the string that now contains the user's input. The `{}` set of
curly brackets is a placeholder: Think of `{}` as little crab pincers that hold
a value in place. When printing the value of a variable, the variable name can
go inside the curly brackets. When printing the result of evaluating an
expression, place empty curly brackets in the format string, then follow the
format string with a comma-separated list of expressions to print in each empty
curly bracket placeholder in the same order. Printing a variable and the result
of an expression in one call to `println!` would look like this:
-->

Cette ligne affiche la chaîne qui contient maintenant la saisie de
l'utilisateur. Les accolades `{}` sont un espace réservé : pensez aux `{}`
comme de petites pinces de crabe qui maintiennent une valeur en place. Lors de
l'affichage de la valeur d'une variable, le nom de la variable peut être placé
à l'intérieur des accolades. Pour afficher le résultat de l'évaluation d'une
expression, placez des accolades vides dans la chaîne de format, puis faites
suivre la chaîne de format d'une liste d'expressions séparées par des virgules
à afficher dans chaque espace réservé dans le même ordre. Afficher une variable
et le résultat d'une expression en un seul appel à `println!` ressemblerait à
ceci :

<!--
```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```
-->

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

<!--
This code would print `x = 5 and y + 2 = 12`.
-->

Ce code afficherait `x = 5 and y + 2 = 12`.

<!--
### Testing the First Part
-->

### Tester la première partie

<!--
Let's test the first part of the guessing game. Run it using `cargo run`:
-->

Testons la première partie du jeu de devinettes. Exécutez-le avec `cargo run` :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

<!--
At this point, the first part of the game is done: We're getting input from the
keyboard and then printing it.
-->

À ce stade, la première partie du jeu est terminée : nous récupérons la saisie
au clavier puis nous l'affichons.

<!--
## Generating a Secret Number
-->

## Générer un nombre secret

<!--
Next, we need to generate a secret number that the user will try to guess. The
secret number should be different every time so that the game is fun to play
more than once. We'll use a random number between 1 and 100 so that the game
isn't too difficult. Rust doesn't yet include random number functionality in
its standard library. However, the Rust team does provide a [`rand`
crate][randcrate] with said functionality.
-->

Ensuite, nous devons générer un nombre secret que l'utilisateur essaiera de
deviner. Le nombre secret devrait être différent à chaque fois pour que le jeu
reste amusant à rejouer. Nous utiliserons un nombre aléatoire entre 1 et 100
pour que le jeu ne soit pas trop difficile. Rust n'inclut pas encore de
fonctionnalité de génération de nombres aléatoires dans sa bibliothèque
standard. Cependant, l'équipe Rust fournit une [crate `rand`][randcrate] avec
cette fonctionnalité.

<!--
Old headings. Do not remove or links may break.
-->
<a id="using-a-crate-to-get-more-functionality"></a>

<!--
### Increasing Functionality with a Crate
-->

### Enrichir les fonctionnalités avec une crate

<!--
Remember that a crate is a collection of Rust source code files. The project
we've been building is a binary crate, which is an executable. The `rand` crate
is a library crate, which contains code that is intended to be used in other
programs and can't be executed on its own.
-->

Rappelez-vous qu'une crate est une collection de fichiers de code source Rust.
Le projet que nous construisons est une crate binaire, c'est-à-dire un
exécutable. La crate `rand` est une crate de bibliothèque, qui contient du code
destiné à être utilisé dans d'autres programmes et ne peut pas être exécutée
seule.

<!--
Cargo's coordination of external crates is where Cargo really shines. Before we
can write code that uses `rand`, we need to modify the _Cargo.toml_ file to
include the `rand` crate as a dependency. Open that file now and add the
following line to the bottom, beneath the `[dependencies]` section header that
Cargo created for you. Be sure to specify `rand` exactly as we have here, with
this version number, or the code examples in this tutorial may not work:
-->

La coordination des crates externes par Cargo est là où Cargo brille vraiment.
Avant de pouvoir écrire du code qui utilise `rand`, nous devons modifier le
fichier _Cargo.toml_ pour inclure la crate `rand` comme dépendance. Ouvrez ce
fichier maintenant et ajoutez la ligne suivante en bas, sous l'en-tête de
section `[dependencies]` que Cargo a créé pour vous. Assurez-vous de spécifier
`rand` exactement comme nous l'avons fait ici, avec ce numéro de version, sinon
les exemples de code de ce tutoriel pourraient ne pas fonctionner :

<!--
When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Fichier : Cargo.toml</span>


```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

<!--
In the _Cargo.toml_ file, everything that follows a header is part of that
section that continues until another section starts. In `[dependencies]`, you
tell Cargo which external crates your project depends on and which versions of
those crates you require. In this case, we specify the `rand` crate with the
semantic version specifier `0.8.5`. Cargo understands [Semantic
Versioning][semver] ignore
--> (sometimes called _SemVer_), which is a
standard for writing version numbers. The specifier `0.8.5` is actually
shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but
below 0.9.0.
-->

Dans le fichier _Cargo.toml_, tout ce qui suit un en-tête fait partie de cette
section et continue jusqu'au début d'une autre section. Dans `[dependencies]`,
vous indiquez à Cargo de quelles crates externes votre projet dépend et quelles
versions de ces crates vous exigez. Dans ce cas, nous spécifions la crate
`rand` avec le spécificateur de version sémantique `0.8.5`. Cargo comprend le
[versionnement sémantique][semver]<!--
ignore
--> (parfois appelé _SemVer_),
qui est un standard pour écrire les numéros de version. Le spécificateur
`0.8.5` est en fait un raccourci pour `^0.8.5`, ce qui signifie toute version
supérieure ou égale à 0.8.5 mais inférieure à 0.9.0.

<!--
Cargo considers these versions to have public APIs compatible with version
0.8.5, and this specification ensures that you'll get the latest patch release
that will still compile with the code in this chapter. Any version 0.9.0 or
greater is not guaranteed to have the same API as what the following examples
use.
-->

Cargo considère que ces versions ont des API publiques compatibles avec la
version 0.8.5, et cette spécification garantit que vous obtiendrez la dernière
version corrective qui compilera encore avec le code de ce chapitre. Toute
version 0.9.0 ou supérieure n'est pas garantie d'avoir la même API que celle
utilisée dans les exemples suivants.

<!--
Now, without changing any of the code, let's build the project, as shown in
Listing 2-2.
-->

Maintenant, sans modifier le code, construisons le projet, comme montré dans
l'encart 2-2.

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build
-->

<Listing number="2-2" caption="La sortie de l'exécution de `cargo build` après avoir ajouté la crate `rand` comme dépendance">

<!--
```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```
-->

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

<!--
You may see different version numbers (but they will all be compatible with the
code, thanks to SemVer!) and different lines (depending on the operating
system), and the lines may be in a different order.
-->

Vous pourriez voir des numéros de version différents (mais ils seront tous
compatibles avec le code, grâce à SemVer !) et des lignes différentes (selon le
système d'exploitation), et les lignes pourraient être dans un ordre différent.

<!--
When we include an external dependency, Cargo fetches the latest versions of
everything that dependency needs from the _registry_, which is a copy of data
from [Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.
-->

Lorsque nous incluons une dépendance externe, Cargo récupère les dernières
versions de tout ce dont cette dépendance a besoin depuis le _registre_, qui est
une copie des données de [Crates.io][cratesio]. Crates.io est l'endroit où les
membres de l'écosystème Rust publient leurs projets Rust open source pour que
d'autres puissent les utiliser.

<!--
After updating the registry, Cargo checks the `[dependencies]` section and
downloads any crates listed that aren't already downloaded. In this case,
although we only listed `rand` as a dependency, Cargo also grabbed other crates
that `rand` depends on to work. After downloading the crates, Rust compiles
them and then compiles the project with the dependencies available.
-->

Après avoir mis à jour le registre, Cargo vérifie la section `[dependencies]`
et télécharge toutes les crates listées qui ne sont pas encore téléchargées.
Dans ce cas, bien que nous n'ayons listé que `rand` comme dépendance, Cargo a
également récupéré d'autres crates dont `rand` dépend pour fonctionner. Après
avoir téléchargé les crates, Rust les compile puis compile le projet avec les
dépendances disponibles.

<!--
If you immediately run `cargo build` again without making any changes, you
won't get any output aside from the `Finished` line. Cargo knows it has already
downloaded and compiled the dependencies, and you haven't changed anything
about them in your _Cargo.toml_ file. Cargo also knows that you haven't changed
anything about your code, so it doesn't recompile that either. With nothing to
do, it simply exits.
-->

Si vous exécutez immédiatement `cargo build` à nouveau sans apporter de
modifications, vous n'obtiendrez aucune sortie à part la ligne `Finished`.
Cargo sait qu'il a déjà téléchargé et compilé les dépendances, et que vous
n'avez rien changé dans votre fichier _Cargo.toml_. Cargo sait également que
vous n'avez rien changé dans votre code, donc il ne le recompile pas non plus.
N'ayant rien à faire, il se termine simplement.

<!--
If you open the _src/main.rs_ file, make a trivial change, and then save it and
build again, you'll only see two lines of output:
-->

Si vous ouvrez le fichier _src/main.rs_, faites une modification mineure, puis
sauvegardez-le et reconstruisez, vous ne verrez que deux lignes de sortie :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build
-->

<!--
```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```
-->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

<!--
These lines show that Cargo only updates the build with your tiny change to the
_src/main.rs_ file. Your dependencies haven't changed, so Cargo knows it can
reuse what it has already downloaded and compiled for those.
-->

Ces lignes montrent que Cargo ne met à jour la compilation qu'avec votre petite
modification du fichier _src/main.rs_. Vos dépendances n'ont pas changé, donc
Cargo sait qu'il peut réutiliser ce qu'il a déjà téléchargé et compilé pour
celles-ci.

<!--
Old headings. Do not remove or links may break.
-->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

<!--
#### Ensuring Reproducible Builds
-->

#### Garantir des compilations reproductibles

<!--
Cargo has a mechanism that ensures that you can rebuild the same artifact every
time you or anyone else builds your code: Cargo will use only the versions of
the dependencies you specified until you indicate otherwise. For example, say
that next week version 0.8.6 of the `rand` crate comes out, and that version
contains an important bug fix, but it also contains a regression that will
break your code. To handle this, Rust creates the _Cargo.lock_ file the first
time you run `cargo build`, so we now have this in the _guessing_game_
directory.
-->

Cargo dispose d'un mécanisme qui garantit que vous pouvez reconstruire le même
artefact à chaque fois que vous ou quelqu'un d'autre compile votre code : Cargo
n'utilisera que les versions des dépendances que vous avez spécifiées tant que
vous n'indiquez pas le contraire. Par exemple, supposons que la semaine
prochaine la version 0.8.6 de la crate `rand` sorte, et que cette version
contienne un correctif important, mais aussi une régression qui casserait votre
code. Pour gérer cela, Rust crée le fichier _Cargo.lock_ la première fois que
vous exécutez `cargo build`, donc nous avons maintenant ce fichier dans le
répertoire _guessing_game_.

<!--
When you build a project for the first time, Cargo figures out all the versions
of the dependencies that fit the criteria and then writes them to the
_Cargo.lock_ file. When you build your project in the future, Cargo will see
that the _Cargo.lock_ file exists and will use the versions specified there
rather than doing all the work of figuring out versions again. This lets you
have a reproducible build automatically. In other words, your project will
remain at 0.8.5 until you explicitly upgrade, thanks to the _Cargo.lock_ file.
Because the _Cargo.lock_ file is important for reproducible builds, it's often
checked into source control with the rest of the code in your project.
-->

Lorsque vous compilez un projet pour la première fois, Cargo détermine toutes
les versions des dépendances qui correspondent aux critères et les écrit dans
le fichier _Cargo.lock_. Lorsque vous compilerez votre projet à l'avenir, Cargo
verra que le fichier _Cargo.lock_ existe et utilisera les versions spécifiées
dedans plutôt que de refaire tout le travail de détermination des versions. Cela
vous permet d'avoir une compilation reproductible automatiquement. En d'autres
termes, votre projet restera en version 0.8.5 jusqu'à ce que vous fassiez une
mise à jour explicite, grâce au fichier _Cargo.lock_. Comme le fichier
_Cargo.lock_ est important pour les compilations reproductibles, il est souvent
versionné dans le contrôle de source avec le reste du code de votre projet.

<!--
#### Updating a Crate to Get a New Version
-->

#### Mettre à jour une crate pour obtenir une nouvelle version

<!--
When you _do_ want to update a crate, Cargo provides the command `update`,
which will ignore the _Cargo.lock_ file and figure out all the latest versions
that fit your specifications in _Cargo.toml_. Cargo will then write those
versions to the _Cargo.lock_ file. Otherwise, by default, Cargo will only look
for versions greater than 0.8.5 and less than 0.9.0. If the `rand` crate has
released the two new versions 0.8.6 and 0.999.0, you would see the following if
you ran `cargo update`:
-->

Lorsque vous souhaitez _effectivement_ mettre à jour une crate, Cargo fournit
la commande `update`, qui ignorera le fichier _Cargo.lock_ et déterminera
toutes les dernières versions correspondant à vos spécifications dans
_Cargo.toml_. Cargo écrira ensuite ces versions dans le fichier _Cargo.lock_.
Par défaut, Cargo ne cherchera que les versions supérieures à 0.8.5 et
inférieures à 0.9.0. Si la crate `rand` a publié deux nouvelles versions 0.8.6
et 0.999.0, vous verriez ce qui suit en exécutant `cargo update` :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here
-->

<!--
```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```
-->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

<!--
Cargo ignores the 0.999.0 release. At this point, you would also notice a
change in your _Cargo.lock_ file noting that the version of the `rand` crate
you are now using is 0.8.6. To use `rand` version 0.999.0 or any version in the
0.999._x_ series, you'd have to update the _Cargo.toml_ file to look like this
instead (don't actually make this change because the following examples assume
you're using `rand` 0.8):
-->

Cargo ignore la version 0.999.0. À ce stade, vous remarqueriez également un
changement dans votre fichier _Cargo.lock_ indiquant que la version de la crate
`rand` que vous utilisez maintenant est 0.8.6. Pour utiliser la version 0.999.0
de `rand` ou toute version de la série 0.999._x_, vous devriez mettre à jour le
fichier _Cargo.toml_ pour qu'il ressemble à ceci (ne faites pas réellement
cette modification car les exemples suivants supposent que vous utilisez `rand`
0.8) :

<!--
```toml
[dependencies]
rand = "0.999.0"
```
-->

```toml
[dependencies]
rand = "0.999.0"
```

<!--
The next time you run `cargo build`, Cargo will update the registry of crates
available and reevaluate your `rand` requirements according to the new version
you have specified.
-->

La prochaine fois que vous exécuterez `cargo build`, Cargo mettra à jour le
registre des crates disponibles et réévaluera vos exigences pour `rand` selon
la nouvelle version que vous avez spécifiée.

<!--
There's a lot more to say about [Cargo][doccargo] ignore
--> and [its
ecosystem][doccratesio]<!--
ignore
-->, which we'll discuss in Chapter 14, but
for now, that's all you need to know. Cargo makes it very easy to reuse
libraries, so Rustaceans are able to write smaller projects that are assembled
from a number of packages.
-->

Il y a beaucoup plus à dire sur [Cargo][doccargo]<!--
ignore
--> et [son
écosystème][doccratesio]<!--
ignore
-->, ce que nous aborderons au chapitre 14,
mais pour l'instant, c'est tout ce que vous devez savoir. Cargo facilite
grandement la réutilisation des bibliothèques, ce qui permet aux Rustacés
d'écrire des projets plus petits assemblés à partir de nombreux paquets.

<!--
### Generating a Random Number
-->

### Générer un nombre aléatoire

<!--
Let's start using `rand` to generate a number to guess. The next step is to
update _src/main.rs_, as shown in Listing 2-3.
-->

Commençons à utiliser `rand` pour générer un nombre à deviner. L'étape suivante
est de mettre à jour _src/main.rs_, comme montré dans l'encart 2-3.

<Listing number="2-3" file-name="src/main.rs" caption="Ajout du code pour générer un nombre aléatoire">


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

<!--
First, we add the line `use rand::Rng;`. The `Rng` trait defines methods that
random number generators implement, and this trait must be in scope for us to
use those methods. Chapter 10 will cover traits in detail.
-->

D'abord, nous ajoutons la ligne `use rand::Rng;`. Le trait `Rng` définit des
méthodes que les générateurs de nombres aléatoires implémentent, et ce trait
doit être dans la portée pour que nous puissions utiliser ces méthodes. Le
chapitre 10 couvrira les traits en détail.

<!--
Next, we're adding two lines in the middle. In the first line, we call the
`rand::thread_rng` function that gives us the particular random number
generator we're going to use: one that is local to the current thread of
execution and is seeded by the operating system. Then, we call the `gen_range`
method on the random number generator. This method is defined by the `Rng`
trait that we brought into scope with the `use rand::Rng;` statement. The
`gen_range` method takes a range expression as an argument and generates a
random number in the range. The kind of range expression we're using here takes
the form `start..=end` and is inclusive on the lower and upper bounds, so we
need to specify `1..=100` to request a number between 1 and 100.
-->

Ensuite, nous ajoutons deux lignes au milieu. Dans la première ligne, nous
appelons la fonction `rand::thread_rng` qui nous donne le générateur de nombres
aléatoires particulier que nous allons utiliser : un générateur local au thread
d'exécution actuel et initialisé par le système d'exploitation. Puis, nous
appelons la méthode `gen_range` sur le générateur de nombres aléatoires. Cette
méthode est définie par le trait `Rng` que nous avons importé dans la portée
avec l'instruction `use rand::Rng;`. La méthode `gen_range` prend une
expression d'intervalle comme argument et génère un nombre aléatoire dans cet
intervalle. Le type d'expression d'intervalle que nous utilisons ici prend la
forme `start..=end` et est inclusif sur les bornes inférieure et supérieure,
nous devons donc spécifier `1..=100` pour demander un nombre entre 1 et 100.

<!--
> Note: You won't just know which traits to use and which methods and functions
> to call from a crate, so each crate has documentation with instructions for
> using it. Another neat feature of Cargo is that running the `cargo doc
> --open` command will build documentation provided by all your dependencies
> locally and open it in your browser. If you're interested in other
> functionality in the `rand` crate, for example, run `cargo doc --open` and
> click `rand` in the sidebar on the left.
-->

> Remarque : vous ne saurez pas spontanément quels traits utiliser et quelles
> méthodes et fonctions appeler depuis une crate, c'est pourquoi chaque crate
> dispose d'une documentation avec des instructions d'utilisation. Une autre
> fonctionnalité pratique de Cargo est que l'exécution de la commande
> `cargo doc --open` construira la documentation fournie par toutes vos
> dépendances localement et l'ouvrira dans votre navigateur. Si vous êtes
> intéressé par d'autres fonctionnalités de la crate `rand`, par exemple,
> exécutez `cargo doc --open` et cliquez sur `rand` dans la barre latérale
> de gauche.

<!--
The second new line prints the secret number. This is useful while we're
developing the program to be able to test it, but we'll delete it from the
final version. It's not much of a game if the program prints the answer as soon
as it starts!
-->

La deuxième nouvelle ligne affiche le nombre secret. C'est utile pendant que
nous développons le programme pour pouvoir le tester, mais nous la supprimerons
de la version finale. Ce n'est pas vraiment un jeu si le programme affiche la
réponse dès qu'il démarre !

<!--
Try running the program a few times:
-->

Essayez d'exécuter le programme quelques fois :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

<!--
You should get different random numbers, and they should all be numbers between
1 and 100. Great job!
-->

Vous devriez obtenir des nombres aléatoires différents, et ils devraient tous
être des nombres entre 1 et 100. Bon travail !

<!--
## Comparing the Guess to the Secret Number
-->

## Comparer la proposition au nombre secret

<!--
Now that we have user input and a random number, we can compare them. That step
is shown in Listing 2-4. Note that this code won't compile just yet, as we will
explain.
-->

Maintenant que nous avons la saisie de l'utilisateur et un nombre aléatoire,
nous pouvons les comparer. Cette étape est montrée dans l'encart 2-4. Notez que
ce code ne compilera pas encore, comme nous allons l'expliquer.

<Listing number="2-4" file-name="src/main.rs" caption="Gestion des valeurs de retour possibles de la comparaison de deux nombres">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

<!--
First, we add another `use` statement, bringing a type called
`std::cmp::Ordering` into scope from the standard library. The `Ordering` type
is another enum and has the variants `Less`, `Greater`, and `Equal`. These are
the three outcomes that are possible when you compare two values.
-->

D'abord, nous ajoutons une autre instruction `use`, important un type appelé
`std::cmp::Ordering` depuis la bibliothèque standard. Le type `Ordering` est
un autre enum et possède les variantes `Less`, `Greater` et `Equal`. Ce sont les
trois résultats possibles lorsque vous comparez deux valeurs.

<!--
Then, we add five new lines at the bottom that use the `Ordering` type. The
`cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: Here, it's
comparing `guess` to `secret_number`. Then, it returns a variant of the
`Ordering` enum we brought into scope with the `use` statement. We use a
[`match`][match] ignore
--> expression to decide what to do next based on
which variant of `Ordering` was returned from the call to `cmp` with the values
in `guess` and `secret_number`.
-->

Ensuite, nous ajoutons cinq nouvelles lignes en bas qui utilisent le type
`Ordering`. La méthode `cmp` compare deux valeurs et peut être appelée sur tout
ce qui peut être comparé. Elle prend une référence vers ce avec quoi vous voulez
comparer : ici, elle compare `guess` à `secret_number`. Puis, elle renvoie une
variante de l'enum `Ordering` que nous avons importé dans la portée avec
l'instruction `use`. Nous utilisons une expression
[`match`][match]<!--
ignore
--> pour décider quoi faire ensuite en fonction de la
variante d'`Ordering` renvoyée par l'appel à `cmp` avec les valeurs de `guess`
et `secret_number`.

<!--
A `match` expression is made up of _arms_. An arm consists of a _pattern_ to
match against, and the code that should be run if the value given to `match`
fits that arm's pattern. Rust takes the value given to `match` and looks
through each arm's pattern in turn. Patterns and the `match` construct are
powerful Rust features: They let you express a variety of situations your code
might encounter, and they make sure you handle them all. These features will be
covered in detail in Chapter 6 and Chapter 19, respectively.
-->

Une expression `match` est composée de _branches_. Une branche consiste en un
_motif_ auquel correspondre et le code qui devrait être exécuté si la valeur
donnée au `match` correspond au motif de cette branche. Rust prend la valeur
donnée au `match` et examine le motif de chaque branche tour à tour. Les motifs
et la construction `match` sont des fonctionnalités puissantes de Rust : ils
vous permettent d'exprimer une variété de situations que votre code pourrait
rencontrer et s'assurent que vous les gérez toutes. Ces fonctionnalités seront
couvertes en détail aux chapitres 6 et 19, respectivement.

<!--
Let's walk through an example with the `match` expression we use here. Say that
the user has guessed 50 and the randomly generated secret number this time is
38.
-->

Parcourons un exemple avec l'expression `match` que nous utilisons ici. Disons
que l'utilisateur a proposé 50 et que le nombre secret généré aléatoirement
cette fois est 38.

<!--
When the code compares 50 to 38, the `cmp` method will return
`Ordering::Greater` because 50 is greater than 38. The `match` expression gets
the `Ordering::Greater` value and starts checking each arm's pattern. It looks
at the first arm's pattern, `Ordering::Less`, and sees that the value
`Ordering::Greater` does not match `Ordering::Less`, so it ignores the code in
that arm and moves to the next arm. The next arm's pattern is
`Ordering::Greater`, which _does_ match `Ordering::Greater`! The associated
code in that arm will execute and print `Too big!` to the screen. The `match`
expression ends after the first successful match, so it won't look at the last
arm in this scenario.
-->

Lorsque le code compare 50 à 38, la méthode `cmp` renverra
`Ordering::Greater` car 50 est supérieur à 38. L'expression `match` reçoit la
valeur `Ordering::Greater` et commence à vérifier le motif de chaque branche.
Elle regarde le motif de la première branche, `Ordering::Less`, et constate que
la valeur `Ordering::Greater` ne correspond pas à `Ordering::Less`, donc elle
ignore le code de cette branche et passe à la suivante. Le motif de la branche
suivante est `Ordering::Greater`, qui correspond _bien_ à
`Ordering::Greater` ! Le code associé à cette branche s'exécutera et affichera
`Too big!` à l'écran. L'expression `match` se termine après la première
correspondance réussie, donc elle ne regardera pas la dernière branche dans ce
scénario.

<!--
However, the code in Listing 2-4 won't compile yet. Let's try it:
-->

Cependant, le code de l'encart 2-4 ne compilera pas encore. Essayons :

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->


```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

<!--
The core of the error states that there are _mismatched types_. Rust has a
strong, static type system. However, it also has type inference. When we wrote
`let mut guess = String::new()`, Rust was able to infer that `guess` should be
a `String` and didn't make us write the type. The `secret_number`, on the other
hand, is a number type. A few of Rust's number types can have a value between 1
and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a
64-bit number; as well as others. Unless otherwise specified, Rust defaults to
an `i32`, which is the type of `secret_number` unless you add type information
elsewhere that would cause Rust to infer a different numerical type. The reason
for the error is that Rust cannot compare a string and a number type.
-->

Le coeur de l'erreur indique qu'il y a des _types incompatibles_. Rust possède
un système de types fort et statique. Cependant, il dispose également de
l'inférence de types. Lorsque nous avons écrit `let mut guess = String::new()`,
Rust a pu inférer que `guess` devait être un `String` et ne nous a pas demandé
d'écrire le type. Le `secret_number`, en revanche, est un type numérique.
Plusieurs types numériques de Rust peuvent avoir une valeur entre 1 et 100 :
`i32`, un nombre 32 bits ; `u32`, un nombre 32 bits non signé ; `i64`, un
nombre 64 bits ; ainsi que d'autres. Sauf indication contraire, Rust utilise
par défaut un `i32`, qui est le type de `secret_number` à moins que vous
n'ajoutiez des informations de type ailleurs qui amèneraient Rust à inférer un
type numérique différent. La raison de l'erreur est que Rust ne peut pas
comparer une chaîne de caractères et un type numérique.

<!--
Ultimately, we want to convert the `String` the program reads as input into a
number type so that we can compare it numerically to the secret number. We do
so by adding this line to the `main` function body:
-->

En fin de compte, nous voulons convertir le `String` que le programme lit en
entrée en un type numérique afin de pouvoir le comparer numériquement au nombre
secret. Nous le faisons en ajoutant cette ligne au corps de la fonction `main` :

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

<!--
The line is:
-->

La ligne est :

<!--
```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
-->

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

<!--
We create a variable named `guess`. But wait, doesn't the program already have
a variable named `guess`? It does, but helpfully Rust allows us to shadow the
previous value of `guess` with a new one. _Shadowing_ lets us reuse the `guess`
variable name rather than forcing us to create two unique variables, such as
`guess_str` and `guess`, for example. We'll cover this in more detail in
[Chapter 3][shadowing] ignore
-->, but for now, know that this feature is
often used when you want to convert a value from one type to another type.
-->

Nous créons une variable nommée `guess`. Mais attendez, le programme n'a-t-il
pas déjà une variable nommée `guess` ? C'est le cas, mais Rust nous permet
utilement de masquer la valeur précédente de `guess` avec une nouvelle. Le
_masquage_ (shadowing) nous permet de réutiliser le nom de variable `guess`
plutôt que de nous forcer à créer deux variables distinctes, comme `guess_str`
et `guess`, par exemple. Nous couvrirons cela plus en détail au
[chapitre 3][shadowing]<!--
ignore
-->, mais pour l'instant, sachez que cette
fonctionnalité est souvent utilisée lorsque vous voulez convertir une valeur
d'un type vers un autre type.

<!--
We bind this new variable to the expression `guess.trim().parse()`. The `guess`
in the expression refers to the original `guess` variable that contained the
input as a string. The `trim` method on a `String` instance will eliminate any
whitespace at the beginning and end, which we must do before we can convert the
string to a `u32`, which can only contain numerical data. The user must press
<kbd>enter</kbd> to satisfy `read_line` and input their guess, which adds a
newline character to the string. For example, if the user types <kbd>5</kbd> and
presses <kbd>enter</kbd>, `guess` looks like this: `5\n`. The `\n` represents
"newline." (On Windows, pressing <kbd>enter</kbd> results in a carriage return
and a newline, `\r\n`.) The `trim` method eliminates `\n` or `\r\n`, resulting
in just `5`.
-->

Nous lions cette nouvelle variable à l'expression `guess.trim().parse()`. Le
`guess` dans l'expression fait référence à la variable `guess` originale qui
contenait la saisie sous forme de chaîne. La méthode `trim` sur une instance de
`String` éliminera tous les espaces blancs au début et à la fin, ce que nous
devons faire avant de pouvoir convertir la chaîne en `u32`, qui ne peut contenir
que des données numériques. L'utilisateur doit appuyer sur
<kbd>Entrée</kbd> pour valider `read_line` et saisir sa proposition, ce qui
ajoute un caractère de nouvelle ligne à la chaîne. Par exemple, si l'utilisateur
tape <kbd>5</kbd> et appuie sur <kbd>Entrée</kbd>, `guess` ressemble à ceci :
`5\n`. Le `\n` représente un "retour à la ligne". (Sous Windows, appuyer sur
<kbd>Entrée</kbd> produit un retour chariot et un retour à la ligne, `\r\n`.)
La méthode `trim` élimine `\n` ou `\r\n`, ne laissant que `5`.

<!--
The [`parse` method on strings][parse] ignore
--> converts a string to
another type. Here, we use it to convert from a string to a number. We need to
tell Rust the exact number type we want by using `let guess: u32`. The colon
(`:`) after `guess` tells Rust we'll annotate the variable's type. Rust has a
few built-in number types; the `u32` seen here is an unsigned, 32-bit integer.
It's a good default choice for a small positive number. You'll learn about
other number types in [Chapter 3][integers]<!--
ignore
-->.
-->

La [méthode `parse` sur les chaînes][parse]<!--
ignore
--> convertit une chaîne
vers un autre type. Ici, nous l'utilisons pour convertir une chaîne en nombre.
Nous devons indiquer à Rust le type numérique exact que nous voulons en
utilisant `let guess: u32`. Les deux-points (`:`) après `guess` indiquent à
Rust que nous allons annoter le type de la variable. Rust dispose de quelques
types numériques intégrés ; le `u32` vu ici est un entier non signé de 32 bits.
C'est un bon choix par défaut pour un petit nombre positif. Vous en apprendrez
davantage sur les autres types numériques au
[chapitre 3][integers]<!--
ignore
-->.

<!--
Additionally, the `u32` annotation in this example program and the comparison
with `secret_number` means Rust will infer that `secret_number` should be a
`u32` as well. So, now the comparison will be between two values of the same
type!
-->

De plus, l'annotation `u32` dans ce programme d'exemple et la comparaison avec
`secret_number` signifient que Rust inférera que `secret_number` devrait
également être un `u32`. Ainsi, la comparaison se fera maintenant entre deux
valeurs du même type !

<!--
The `parse` method will only work on characters that can logically be converted
into numbers and so can easily cause errors. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in ["Handling Potential Failure with
`Result`"](#handling-potential-failure-with-result) ignore
-->). We'll treat
this `Result` the same way by using the `expect` method again. If `parse`
returns an `Err` `Result` variant because it couldn't create a number from the
string, the `expect` call will crash the game and print the message we give it.
If `parse` can successfully convert the string to a number, it will return the
`Ok` variant of `Result`, and `expect` will return the number that we want from
the `Ok` value.
-->

La méthode `parse` ne fonctionnera que sur des caractères qui peuvent
logiquement être convertis en nombres et peut donc facilement provoquer des
erreurs. Si, par exemple, la chaîne contenait `A👍%`, il n'y aurait aucun
moyen de la convertir en nombre. Comme elle peut échouer, la méthode `parse`
renvoie un type `Result`, tout comme la méthode `read_line` (discutée
précédemment dans ["Gérer les erreurs potentielles avec
`Result`"](#handling-potential-failure-with-result)<!--
ignore
-->). Nous
traiterons ce `Result` de la même manière en utilisant à nouveau la méthode
`expect`. Si `parse` renvoie une variante `Err` de `Result` parce qu'elle n'a
pas pu créer un nombre à partir de la chaîne, l'appel à `expect` fera planter
le jeu et affichera le message que nous lui donnons. Si `parse` peut convertir
avec succès la chaîne en nombre, elle renverra la variante `Ok` de `Result`, et
`expect` renverra le nombre que nous voulons à partir de la valeur `Ok`.

<!--
Let's run the program now:
-->

Exécutons le programme maintenant :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

<!--
Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: Guess the number correctly,
guess a number that is too high, and guess a number that is too low.
-->

Super ! Même si des espaces ont été ajoutés avant la proposition, le programme a
quand même déterminé que l'utilisateur a proposé 76. Exécutez le programme
plusieurs fois pour vérifier le comportement différent avec différents types de
saisie : devinez le nombre correctement, proposez un nombre trop élevé et
proposez un nombre trop bas.

<!--
We have most of the game working now, but the user can make only one guess.
Let's change that by adding a loop!
-->

La majeure partie du jeu fonctionne maintenant, mais l'utilisateur ne peut faire
qu'une seule proposition. Changeons cela en ajoutant une boucle !

<!--
## Allowing Multiple Guesses with Looping
-->

## Permettre plusieurs propositions avec une boucle

<!--
The `loop` keyword creates an infinite loop. We'll add a loop to give users
more chances at guessing the number:
-->

Le mot-clé `loop` crée une boucle infinie. Nous allons ajouter une boucle pour
donner aux utilisateurs plus de chances de deviner le nombre :

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

<!--
As you can see, we've moved everything from the guess input prompt onward into
a loop. Be sure to indent the lines inside the loop another four spaces each
and run the program again. The program will now ask for another guess forever,
which actually introduces a new problem. It doesn't seem like the user can quit!
-->

Comme vous pouvez le voir, nous avons déplacé tout ce qui suit l'invite de
saisie dans une boucle. Assurez-vous d'indenter les lignes à l'intérieur de la
boucle de quatre espaces supplémentaires chacune et relancez le programme. Le
programme demandera désormais une nouvelle proposition indéfiniment, ce qui
introduit en fait un nouveau problème. Il semble que l'utilisateur ne puisse pas
quitter !

<!--
The user could always interrupt the program by using the keyboard shortcut
<kbd>ctrl</kbd>-<kbd>C</kbd>. But there's another way to escape this insatiable
monster, as mentioned in the `parse` discussion in ["Comparing the Guess to the
Secret Number"](#comparing-the-guess-to-the-secret-number) ignore
-->: If
the user enters a non-number answer, the program will crash. We can take
advantage of that to allow the user to quit, as shown here:
-->

L'utilisateur pourrait toujours interrompre le programme en utilisant le
raccourci clavier <kbd>ctrl</kbd>-<kbd>C</kbd>. Mais il existe une autre façon
d'échapper à ce monstre insatiable, comme mentionné dans la discussion sur
`parse` dans ["Comparer la proposition au nombre
secret"](#comparing-the-guess-to-the-secret-number)<!--
ignore
--> : si
l'utilisateur saisit une réponse qui n'est pas un nombre, le programme
plantera. Nous pouvons en tirer parti pour permettre à l'utilisateur de quitter,
comme montré ici :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!--
Typing `quit` will quit the game, but as you'll notice, so will entering any
other non-number input. This is suboptimal, to say the least; we want the game
to also stop when the correct number is guessed.
-->

Taper `quit` quittera le jeu, mais comme vous le remarquerez, saisir toute
autre entrée non numérique le fera également. C'est sous-optimal, c'est le
moins que l'on puisse dire ; nous voulons que le jeu s'arrête également lorsque
le bon nombre est deviné.

<!--
### Quitting After a Correct Guess
-->

### Quitter après une bonne réponse

<!--
Let's program the game to quit when the user wins by adding a `break` statement:
-->

Programmons le jeu pour qu'il se termine lorsque l'utilisateur gagne en
ajoutant une instruction `break` :

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

<!--
Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.
-->

Ajouter la ligne `break` après `You win!` fait sortir le programme de la boucle
lorsque l'utilisateur devine correctement le nombre secret. Sortir de la boucle
signifie également quitter le programme, car la boucle est la dernière partie
de `main`.

<!--
### Handling Invalid Input
-->

### Gérer les saisies invalides

<!--
To further refine the game's behavior, rather than crashing the program when
the user inputs a non-number, let's make the game ignore a non-number so that
the user can continue guessing. We can do that by altering the line where
`guess` is converted from a `String` to a `u32`, as shown in Listing 2-5.
-->

Pour affiner davantage le comportement du jeu, plutôt que de faire planter le
programme lorsque l'utilisateur saisit autre chose qu'un nombre, faisons en
sorte que le jeu ignore les saisies non numériques pour que l'utilisateur puisse
continuer à deviner. Nous pouvons le faire en modifiant la ligne où `guess` est
converti d'un `String` en `u32`, comme montré dans l'encart 2-5.

<Listing number="2-5" file-name="src/main.rs" caption="Ignorer une proposition non numérique et demander une autre proposition au lieu de faire planter le programme">


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

<!--
We switch from an `expect` call to a `match` expression to move from crashing
on an error to handling the error. Remember that `parse` returns a `Result`
type and `Result` is an enum that has the variants `Ok` and `Err`. We're using
a `match` expression here, as we did with the `Ordering` result of the `cmp`
method.
-->

Nous passons d'un appel à `expect` à une expression `match` pour passer d'un
plantage en cas d'erreur à une gestion de l'erreur. Rappelez-vous que `parse`
renvoie un type `Result` et que `Result` est un enum avec les variantes `Ok` et
`Err`. Nous utilisons ici une expression `match`, comme nous l'avons fait avec
le résultat `Ordering` de la méthode `cmp`.

<!--
If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resultant number. That `Ok` value will
match the first arm's pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we're creating.
-->

Si `parse` parvient à transformer la chaîne en nombre, elle renverra une valeur
`Ok` contenant le nombre résultant. Cette valeur `Ok` correspondra au motif de
la première branche, et l'expression `match` renverra simplement la valeur
`num` que `parse` a produite et placée dans la valeur `Ok`. Ce nombre se
retrouvera exactement là où nous le voulons, dans la nouvelle variable `guess`
que nous créons.

<!--
If `parse` is _not_ able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catch-all value; in this example, we're saying we want to match all `Err`
values, no matter what information they have inside them. So, the program will
execute the second arm's code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So, effectively, the
program ignores all errors that `parse` might encounter!
-->

Si `parse` n'est _pas_ en mesure de transformer la chaîne en nombre, elle
renverra une valeur `Err` contenant plus d'informations sur l'erreur. La valeur
`Err` ne correspond pas au motif `Ok(num)` de la première branche du `match`,
mais elle correspond au motif `Err(_)` de la seconde branche. Le tiret bas,
`_`, est une valeur attrape-tout ; dans cet exemple, nous disons que nous
voulons correspondre à toutes les valeurs `Err`, peu importe les informations
qu'elles contiennent. Ainsi, le programme exécutera le code de la seconde
branche, `continue`, qui indique au programme de passer à l'itération suivante
de la boucle `loop` et de demander une autre proposition. De fait, le programme
ignore toutes les erreurs que `parse` pourrait rencontrer !

<!--
Now everything in the program should work as expected. Let's try it:
-->

Maintenant, tout dans le programme devrait fonctionner comme prévu. Essayons :

<!--
manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

<!--
```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

<!--
Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let's delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.
-->

Parfait ! Avec un dernier petit ajustement, nous terminerons le jeu de
devinettes. Rappelez-vous que le programme affiche encore le nombre secret. Cela
fonctionnait bien pour les tests, mais cela gâche le jeu. Supprimons le
`println!` qui affiche le nombre secret. L'encart 2-6 montre le code final.

<Listing number="2-6" file-name="src/main.rs" caption="Code complet du jeu de devinettes">


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

<!--
At this point, you've successfully built the guessing game. Congratulations!
-->

À ce stade, vous avez construit avec succès le jeu de devinettes.
Félicitations !

<!--
## Summary
-->

## Résumé

<!--
This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, functions, the use of external crates, and more. In the next
few chapters, you'll learn about these concepts in more detail. Chapter 3
covers concepts that most programming languages have, such as variables, data
types, and functions, and shows how to use them in Rust. Chapter 4 explores
ownership, a feature that makes Rust different from other languages. Chapter 5
discusses structs and method syntax, and Chapter 6 explains how enums work.
-->

Ce projet était une façon pratique de vous présenter de nombreux nouveaux
concepts de Rust : `let`, `match`, les fonctions, l'utilisation de crates
externes, et bien plus. Dans les prochains chapitres, vous en apprendrez
davantage sur ces concepts. Le chapitre 3 couvre les concepts que la plupart des
langages de programmation possèdent, comme les variables, les types de données
et les fonctions, et montre comment les utiliser en Rust. Le chapitre 4 explore
la possession (ownership), une fonctionnalité qui distingue Rust des autres
langages. Le chapitre 5 traite des structures (structs) et de la syntaxe des
méthodes, et le chapitre 6 explique le fonctionnement des enums.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
