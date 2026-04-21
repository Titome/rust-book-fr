<!--
Old headings. Do not remove or links may break.
-->

<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>

<!--
## Redirecting Errors to Standard Error
-->

## Rediriger les erreurs vers la sortie d'erreur standard

<!--
At the moment, we're writing all of our output to the terminal using the
`println!` macro. In most terminals, there are two kinds of output: _standard
output_ (`stdout`) for general information and _standard error_ (`stderr`) for
error messages. This distinction enables users to choose to direct the
successful output of a program to a file but still print error messages to the
screen.
-->

Pour le moment, nous écrivons toutes nos sorties sur le terminal en utilisant la macro `println!`. Dans la plupart des terminaux, il existe deux types de sortie : la _sortie standard_ (`stdout`) pour les informations générales et la _sortie d'erreur standard_ (`stderr`) pour les messages d'erreur. Cette distinction permet aux utilisateurs de choisir de rediriger la sortie réussie d'un programme vers un fichier tout en affichant les messages d'erreur à l'écran.

<!--
The `println!` macro is only capable of printing to standard output, so we have
to use something else to print to standard error.
-->

La macro `println!` n'est capable d'écrire que sur la sortie standard, nous devons donc utiliser autre chose pour écrire sur la sortie d'erreur standard.

<!--
### Checking Where Errors Are Written
-->

### Vérifier où les erreurs sont écrites

<!--
First, let's observe how the content printed by `minigrep` is currently being
written to standard output, including any error messages we want to write to
standard error instead. We'll do that by redirecting the standard output stream
to a file while intentionally causing an error. We won't redirect the standard
error stream, so any content sent to standard error will continue to display on
the screen.
-->

Tout d'abord, observons comment le contenu affiché par `minigrep` est actuellement écrit sur la sortie standard, y compris les messages d'erreur que nous voudrions écrire sur la sortie d'erreur standard à la place. Nous ferons cela en redirigeant le flux de sortie standard vers un fichier tout en provoquant intentionnellement une erreur. Nous ne redirigerons pas le flux d'erreur standard, donc tout contenu envoyé vers la sortie d'erreur standard continuera à s'afficher à l'écran.

<!--
Command line programs are expected to send error messages to the standard error
stream so that we can still see error messages on the screen even if we
redirect the standard output stream to a file. Our program is not currently
well behaved: We're about to see that it saves the error message output to a
file instead!
-->

Les programmes en ligne de commande sont censés envoyer les messages d'erreur sur le flux d'erreur standard afin que nous puissions toujours voir les messages d'erreur à l'écran même si nous redirigeons le flux de sortie standard vers un fichier. Notre programme ne se comporte pas correctement actuellement : nous allons voir qu'il enregistre le message d'erreur dans un fichier à la place !

<!--
To demonstrate this behavior, we'll run the program with `>` and the file path,
_output.txt_, that we want to redirect the standard output stream to. We won't
pass any arguments, which should cause an error:
-->

Pour démontrer ce comportement, nous exécuterons le programme avec `>` et le chemin de fichier, _output.txt_, vers lequel nous voulons rediriger le flux de sortie standard. Nous ne passerons aucun argument, ce qui devrait provoquer une erreur :

<!--
```console
$ cargo run > output.txt
```
-->

```console
$ cargo run > output.txt
```

<!--
The `>` syntax tells the shell to write the contents of standard output to
_output.txt_ instead of the screen. We didn't see the error message we were
expecting printed to the screen, so that means it must have ended up in the
file. This is what _output.txt_ contains:
-->

La syntaxe `>` indique au shell d'écrire le contenu de la sortie standard dans _output.txt_ au lieu de l'écran. Nous n'avons pas vu le message d'erreur que nous attendions à l'écran, ce qui signifie qu'il a dû finir dans le fichier. Voici ce que contient _output.txt_ :

<!--
```text
Problem parsing arguments: not enough arguments
```
-->

```text
Problem parsing arguments: not enough arguments
```

<!--
Yup, our error message is being printed to standard output. It's much more
useful for error messages like this to be printed to standard error so that
only data from a successful run ends up in the file. We'll change that.
-->

En effet, notre message d'erreur est affiché sur la sortie standard. Il est beaucoup plus utile que les messages d'erreur comme celui-ci soient affichés sur la sortie d'erreur standard afin que seules les données d'une exécution réussie se retrouvent dans le fichier. Nous allons changer cela.

<!--
### Printing Errors to Standard Error
-->

### Afficher les erreurs sur la sortie d'erreur standard

<!--
We'll use the code in Listing 12-24 to change how error messages are printed.
Because of the refactoring we did earlier in this chapter, all the code that
prints error messages is in one function, `main`. The standard library provides
the `eprintln!` macro that prints to the standard error stream, so let's change
the two places we were calling `println!` to print errors to use `eprintln!`
instead.
-->

Nous utiliserons le code de l'encart 12-24 pour changer la façon dont les messages d'erreur sont affichés. Grâce au refactoring que nous avons fait plus tôt dans ce chapitre, tout le code qui affiche des messages d'erreur se trouve dans une seule fonction, `main`. La bibliothèque standard fournit la macro `eprintln!` qui écrit sur le flux d'erreur standard, donc changeons les deux endroits où nous appelions `println!` pour afficher les erreurs afin d'utiliser `eprintln!` à la place.

<Listing number="12-24" file-name="src/main.rs" caption="Écriture des messages d'erreur sur la sortie d'erreur standard au lieu de la sortie standard en utilisant `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

<!--
Let's now run the program again in the same way, without any arguments and
redirecting standard output with `>`:
-->

Exécutons maintenant à nouveau le programme de la même manière, sans aucun argument et en redirigeant la sortie standard avec `>` :

<!--
```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```
-->

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

<!--
Now we see the error onscreen and _output.txt_ contains nothing, which is the
behavior we expect of command line programs.
-->

Maintenant, nous voyons l'erreur à l'écran et _output.txt_ ne contient rien, ce qui est le comportement attendu des programmes en ligne de commande.

<!--
Let's run the program again with arguments that don't cause an error but still
redirect standard output to a file, like so:
-->

Exécutons à nouveau le programme avec des arguments qui ne provoquent pas d'erreur mais en redirigeant toujours la sortie standard vers un fichier, comme ceci :

<!--
```console
$ cargo run -- to poem.txt > output.txt
```
-->

```console
$ cargo run -- to poem.txt > output.txt
```

<!--
We won't see any output to the terminal, and _output.txt_ will contain our
results:
-->

Nous ne verrons aucune sortie sur le terminal, et _output.txt_ contiendra nos résultats :

<!--
<span class="filename">Filename: output.txt</span>
-->

<span class="filename">Fichier : output.txt</span>

<!--
```text
Are you nobody, too?
How dreary to be somebody!
```
-->

```text
Are you nobody, too?
How dreary to be somebody!
```

<!--
This demonstrates that we're now using standard output for successful output
and standard error for error output as appropriate.
-->

Cela démontre que nous utilisons maintenant la sortie standard pour les sorties réussies et la sortie d'erreur standard pour les sorties d'erreur, comme il se doit.

<!--
## Summary
-->

## Résumé

<!--
This chapter recapped some of the major concepts you've learned so far and
covered how to perform common I/O operations in Rust. By using command line
arguments, files, environment variables, and the `eprintln!` macro for printing
errors, you're now prepared to write command line applications. Combined with
the concepts in previous chapters, your code will be well organized, store data
effectively in the appropriate data structures, handle errors nicely, and be
well tested.
-->

Ce chapitre a récapitulé certains des concepts majeurs que vous avez appris jusqu'ici et a couvert la façon d'effectuer des opérations d'E/S courantes en Rust. En utilisant les arguments de ligne de commande, les fichiers, les variables d'environnement et la macro `eprintln!` pour afficher les erreurs, vous êtes maintenant prêt à écrire des applications en ligne de commande. Combiné avec les concepts des chapitres précédents, votre code sera bien organisé, stockera les données efficacement dans les structures de données appropriées, gérera les erreurs correctement et sera bien testé.

<!--
Next, we'll explore some Rust features that were influenced by functional
languages: closures and iterators.
-->

Ensuite, nous explorerons certaines fonctionnalités de Rust qui ont été influencées par les langages fonctionnels : les fermetures (closures) et les itérateurs.
