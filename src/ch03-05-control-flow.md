<!--
## Control Flow
-->

## Les structures de contrôle

<!--
The ability to run some code depending on whether a condition is `true` and the
ability to run some code repeatedly while a condition is `true` are basic
building blocks in most programming languages. The most common constructs that
let you control the flow of execution of Rust code are `if` expressions and
loops.
-->

La capacité d'exécuter du code selon qu'une condition est `true` et la capacité
d'exécuter du code de manière répétée tant qu'une condition est `true` sont des
éléments fondamentaux dans la plupart des langages de programmation. Les
constructions les plus courantes qui vous permettent de contrôler le flux
d'exécution du code Rust sont les expressions `if` et les boucles.

<!--
### `if` Expressions
-->

### Les expressions `if`

<!--
An `if` expression allows you to branch your code depending on conditions. You
provide a condition and then state, "If this condition is met, run this block
of code. If the condition is not met, do not run this block of code."
-->

Une expression `if` vous permet de créer des embranchements dans votre code en
fonction de conditions. Vous fournissez une condition puis déclarez : "Si cette
condition est remplie, exécute ce bloc de code. Si la condition n'est pas
remplie, n'exécute pas ce bloc de code."

<!--
Create a new project called _branches_ in your _projects_ directory to explore
the `if` expression. In the _src/main.rs_ file, input the following:
-->

Créez un nouveau projet appelé _branches_ dans votre répertoire _projects_ pour
explorer l'expression `if`. Dans le fichier _src/main.rs_, saisissez ce qui
suit :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

<!--
All `if` expressions start with the keyword `if`, followed by a condition. In
this case, the condition checks whether or not the variable `number` has a
value less than 5. We place the block of code to execute if the condition is
`true` immediately after the condition inside curly brackets. Blocks of code
associated with the conditions in `if` expressions are sometimes called _arms_,
just like the arms in `match` expressions that we discussed in the ["Comparing
the Guess to the Secret Number"][comparing-the-guess-to-the-secret-number]
ignore
--> section of Chapter 2.
-->

Toutes les expressions `if` commencent par le mot-clé `if`, suivi d'une
condition. Dans ce cas, la condition vérifie si la variable `number` a une
valeur inférieure à 5. Nous plaçons le bloc de code à exécuter si la condition
est `true` immédiatement après la condition, entre accolades. Les blocs de code
associés aux conditions dans les expressions `if` sont parfois appelés des
_branches_ (arms), tout comme les branches dans les expressions `match` que nous
avons abordées dans la section ["Comparing the Guess to the Secret
Number"][comparing-the-guess-to-the-secret-number]<!--
ignore
--> du chapitre 2.

<!--
Optionally, we can also include an `else` expression, which we chose to do
here, to give the program an alternative block of code to execute should the
condition evaluate to `false`. If you don't provide an `else` expression and
the condition is `false`, the program will just skip the `if` block and move on
to the next bit of code.
-->

Optionnellement, nous pouvons également inclure une expression `else`, ce que
nous avons choisi de faire ici, pour donner au programme un bloc de code
alternatif à exécuter si la condition s'évalue à `false`. Si vous ne fournissez
pas d'expression `else` et que la condition est `false`, le programme passera
simplement le bloc `if` et continuera avec le code suivant.

<!--
Try running this code; you should see the following output:
-->

Essayez d'exécuter ce code ; vous devriez voir la sortie suivante :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

<!--
Let's try changing the value of `number` to a value that makes the condition
`false` to see what happens:
-->

Essayons de changer la valeur de `number` pour une valeur qui rend la condition
`false` afin de voir ce qui se passe :


```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

<!--
Run the program again, and look at the output:
-->

Exécutez le programme à nouveau et observez la sortie :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

<!--
It's also worth noting that the condition in this code _must_ be a `bool`. If
the condition isn't a `bool`, we'll get an error. For example, try running the
following code:
-->

Il est également important de noter que la condition dans ce code _doit_ être un
`bool`. Si la condition n'est pas un `bool`, nous obtiendrons une erreur. Par
exemple, essayez d'exécuter le code suivant :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

<!--
The `if` condition evaluates to a value of `3` this time, and Rust throws an
error:
-->

La condition `if` s'évalue à une valeur de `3` cette fois, et Rust génère une
erreur :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

<!--
The error indicates that Rust expected a `bool` but got an integer. Unlike
languages such as Ruby and JavaScript, Rust will not automatically try to
convert non-Boolean types to a Boolean. You must be explicit and always provide
`if` with a Boolean as its condition. If we want the `if` code block to run
only when a number is not equal to `0`, for example, we can change the `if`
expression to the following:
-->

L'erreur indique que Rust attendait un `bool` mais a reçu un entier.
Contrairement à des langages comme Ruby et JavaScript, Rust ne tentera pas
automatiquement de convertir des types non-booléens en booléen. Vous devez être
explicite et toujours fournir un booléen comme condition au `if`. Si nous
voulons que le bloc de code `if` ne s'exécute que lorsqu'un nombre n'est pas
égal à `0`, par exemple, nous pouvons modifier l'expression `if` comme suit :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

<!--
Running this code will print `number was something other than zero`.
-->

L'exécution de ce code affichera `number was something other than zero`.

<!--
#### Handling Multiple Conditions with `else if`
-->

#### Gérer plusieurs conditions avec `else if`

<!--
You can use multiple conditions by combining `if` and `else` in an `else if`
expression. For example:
-->

Vous pouvez utiliser plusieurs conditions en combinant `if` et `else` dans une
expression `else if`. Par exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

<!--
This program has four possible paths it can take. After running it, you should
see the following output:
-->

Ce programme a quatre chemins possibles qu'il peut emprunter. Après l'avoir
exécuté, vous devriez voir la sortie suivante :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

<!--
When this program executes, it checks each `if` expression in turn and executes
the first body for which the condition evaluates to `true`. Note that even
though 6 is divisible by 2, we don't see the output `number is divisible by 2`,
nor do we see the `number is not divisible by 4, 3, or 2` text from the `else`
block. That's because Rust only executes the block for the first `true`
condition, and once it finds one, it doesn't even check the rest.
-->

Lorsque ce programme s'exécute, il vérifie chaque expression `if` tour à tour et
exécute le premier corps pour lequel la condition s'évalue à `true`. Notez que
même si 6 est divisible par 2, nous ne voyons pas la sortie `number is divisible
by 2`, ni le texte `number is not divisible by 4, 3, or 2` du bloc `else`.
C'est parce que Rust n'exécute que le bloc de la première condition `true`, et
une fois qu'il en a trouvé une, il ne vérifie même pas les suivantes.

<!--
Using too many `else if` expressions can clutter your code, so if you have more
than one, you might want to refactor your code. Chapter 6 describes a powerful
Rust branching construct called `match` for these cases.
-->

Utiliser trop d'expressions `else if` peut encombrer votre code, donc si vous en
avez plus d'une, vous voudrez peut-être restructurer votre code. Le chapitre 6
décrit une construction de branchement puissante de Rust appelée `match` pour
ces cas-là.

<!--
#### Using `if` in a `let` Statement
-->

#### Utiliser `if` dans une instruction `let`

<!--
Because `if` is an expression, we can use it on the right side of a `let`
statement to assign the outcome to a variable, as in Listing 3-2.
-->

Comme `if` est une expression, nous pouvons l'utiliser du côté droit d'une
instruction `let` pour assigner le résultat à une variable, comme dans le
listing 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="Assigner le résultat d'une expression `if` à une variable">


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

<!--
The `number` variable will be bound to a value based on the outcome of the `if`
expression. Run this code to see what happens:
-->

La variable `number` sera liée à une valeur en fonction du résultat de
l'expression `if`. Exécutez ce code pour voir ce qui se passe :


```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

<!--
Remember that blocks of code evaluate to the last expression in them, and
numbers by themselves are also expressions. In this case, the value of the
whole `if` expression depends on which block of code executes. This means the
values that have the potential to be results from each arm of the `if` must be
the same type; in Listing 3-2, the results of both the `if` arm and the `else`
arm were `i32` integers. If the types are mismatched, as in the following
example, we'll get an error:
-->

Rappelez-vous que les blocs de code s'évaluent à la dernière expression qu'ils
contiennent, et que les nombres seuls sont aussi des expressions. Dans ce cas,
la valeur de l'expression `if` entière dépend du bloc de code qui s'exécute.
Cela signifie que les valeurs qui peuvent potentiellement être les résultats de
chaque branche du `if` doivent être du même type ; dans le listing 3-2, les
résultats de la branche `if` et de la branche `else` étaient tous deux des
entiers `i32`. Si les types ne correspondent pas, comme dans l'exemple suivant,
nous obtiendrons une erreur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

<!--
When we try to compile this code, we'll get an error. The `if` and `else` arms
have value types that are incompatible, and Rust indicates exactly where to
find the problem in the program:
-->

Lorsque nous essayons de compiler ce code, nous obtenons une erreur. Les
branches `if` et `else` ont des types de valeurs incompatibles, et Rust indique
exactement où trouver le problème dans le programme :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

<!--
The expression in the `if` block evaluates to an integer, and the expression in
the `else` block evaluates to a string. This won't work, because variables must
have a single type, and Rust needs to know definitively at compile time what
type the `number` variable is. Knowing the type of `number` lets the compiler
verify the type is valid everywhere we use `number`. Rust wouldn't be able to
do that if the type of `number` was only determined at runtime; the compiler
would be more complex and would make fewer guarantees about the code if it had
to keep track of multiple hypothetical types for any variable.
-->

L'expression dans le bloc `if` s'évalue en un entier, et l'expression dans le
bloc `else` s'évalue en une chaîne de caractères. Cela ne fonctionnera pas, car
les variables doivent avoir un seul type, et Rust doit savoir de manière
définitive au moment de la compilation quel est le type de la variable `number`.
Connaître le type de `number` permet au compilateur de vérifier que le type est
valide partout où nous utilisons `number`. Rust ne pourrait pas faire cela si le
type de `number` n'était déterminé qu'à l'exécution ; le compilateur serait plus
complexe et offrirait moins de garanties sur le code s'il devait suivre
plusieurs types hypothétiques pour chaque variable.

<!--
### Repetition with Loops
-->

### La répétition avec les boucles

<!--
It's often useful to execute a block of code more than once. For this task,
Rust provides several _loops_, which will run through the code inside the loop
body to the end and then start immediately back at the beginning. To experiment
with loops, let's make a new project called _loops_.
-->

Il est souvent utile d'exécuter un bloc de code plus d'une fois. Pour cette
tâche, Rust fournit plusieurs _boucles_, qui exécuteront le code à l'intérieur
du corps de la boucle jusqu'à la fin, puis recommenceront immédiatement au
début. Pour expérimenter avec les boucles, créons un nouveau projet appelé
_loops_.

<!--
Rust has three kinds of loops: `loop`, `while`, and `for`. Let's try each one.
-->

Rust dispose de trois types de boucles : `loop`, `while` et `for`. Essayons
chacune d'entre elles.

<!--
#### Repeating Code with `loop`
-->

#### Répéter du code avec `loop`

<!--
The `loop` keyword tells Rust to execute a block of code over and over again
either forever or until you explicitly tell it to stop.
-->

Le mot-clé `loop` indique à Rust d'exécuter un bloc de code encore et encore,
soit indéfiniment, soit jusqu'à ce que vous lui disiez explicitement de
s'arrêter.

<!--
As an example, change the _src/main.rs_ file in your _loops_ directory to look
like this:
-->

À titre d'exemple, modifiez le fichier _src/main.rs_ dans votre répertoire
_loops_ pour qu'il ressemble à ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

<!--
When we run this program, we'll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support the keyboard shortcut
<kbd>ctrl</kbd>-<kbd>C</kbd> to interrupt a program that is stuck in a continual
loop. Give it a try:
-->

Lorsque nous exécutons ce programme, nous verrons `again!` s'afficher en continu
jusqu'à ce que nous arrêtions le programme manuellement. La plupart des
terminaux prennent en charge le raccourci clavier
<kbd>ctrl</kbd>-<kbd>C</kbd> pour interrompre un programme bloqué dans une
boucle continue. Essayez :

<!--
manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

<!--
The symbol `^C` represents where you pressed <kbd>ctrl</kbd>-<kbd>C</kbd>.
-->

Le symbole `^C` représente l'endroit où vous avez appuyé sur
<kbd>ctrl</kbd>-<kbd>C</kbd>.

<!--
You may or may not see the word `again!` printed after the `^C`, depending on
where the code was in the loop when it received the interrupt signal.
-->

Vous verrez peut-être ou non le mot `again!` affiché après le `^C`, selon
l'endroit où le code se trouvait dans la boucle lorsqu'il a reçu le signal
d'interruption.

<!--
Fortunately, Rust also provides a way to break out of a loop using code. You
can place the `break` keyword within the loop to tell the program when to stop
executing the loop. Recall that we did this in the guessing game in the
["Quitting After a Correct Guess"][quitting-after-a-correct-guess] ignore
--> section of Chapter 2 to exit the program when the user won the game by
guessing the correct number.
-->

Heureusement, Rust fournit également un moyen de sortir d'une boucle par le
code. Vous pouvez placer le mot-clé `break` à l'intérieur de la boucle pour
indiquer au programme quand arrêter l'exécution de la boucle. Rappelez-vous que
nous avons fait cela dans le jeu de devinettes dans la section ["Quitting After
a Correct Guess"][quitting-after-a-correct-guess]<!--
ignore
--> du chapitre 2
pour quitter le programme lorsque l'utilisateur gagnait le jeu en devinant le
bon nombre.

<!--
We also used `continue` in the guessing game, which in a loop tells the program
to skip over any remaining code in this iteration of the loop and go to the
next iteration.
-->

Nous avons également utilisé `continue` dans le jeu de devinettes, qui dans une
boucle indique au programme de passer tout le code restant dans cette itération
de la boucle et de passer à l'itération suivante.

<!--
#### Returning Values from Loops
-->

#### Retourner des valeurs depuis les boucles

<!--
One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. To do
this, you can add the value you want returned after the `break` expression you
use to stop the loop; that value will be returned out of the loop so that you
can use it, as shown here:
-->

L'une des utilisations d'une boucle `loop` est de réessayer une opération dont
vous savez qu'elle pourrait échouer, comme vérifier si un thread a terminé son
travail. Vous pourriez également avoir besoin de transmettre le résultat de
cette opération hors de la boucle au reste de votre code. Pour ce faire, vous
pouvez ajouter la valeur que vous souhaitez retourner après l'expression `break`
que vous utilisez pour arrêter la boucle ; cette valeur sera retournée hors de
la boucle afin que vous puissiez l'utiliser, comme montré ici :


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

<!--
Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then, we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the `counter` is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is `20`.
-->

Avant la boucle, nous déclarons une variable nommée `counter` et l'initialisons
à `0`. Puis, nous déclarons une variable nommée `result` pour contenir la valeur
retournée par la boucle. À chaque itération de la boucle, nous ajoutons `1` à
la variable `counter`, puis vérifions si `counter` est égal à `10`. Quand c'est
le cas, nous utilisons le mot-clé `break` avec la valeur `counter * 2`. Après la
boucle, nous utilisons un point-virgule pour terminer l'instruction qui assigne
la valeur à `result`. Enfin, nous affichons la valeur dans `result`, qui dans ce
cas est `20`.

<!--
You can also `return` from inside a loop. While `break` only exits the current
loop, `return` always exits the current function.
-->

Vous pouvez également utiliser `return` depuis l'intérieur d'une boucle. Alors
que `break` ne quitte que la boucle en cours, `return` quitte toujours la
fonction en cours.

<!--
Old headings. Do not remove or links may break.
-->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

<!--
#### Disambiguating with Loop Labels
-->

#### Lever l'ambiguïté avec les étiquettes de boucle

<!--
If you have loops within loops, `break` and `continue` apply to the innermost
loop at that point. You can optionally specify a _loop label_ on a loop that
you can then use with `break` or `continue` to specify that those keywords
apply to the labeled loop instead of the innermost loop. Loop labels must begin
with a single quote. Here's an example with two nested loops:
-->

Si vous avez des boucles à l'intérieur de boucles, `break` et `continue`
s'appliquent à la boucle la plus interne à ce moment-là. Vous pouvez
optionnellement spécifier une _étiquette de boucle_ (loop label) sur une boucle
que vous pouvez ensuite utiliser avec `break` ou `continue` pour spécifier que
ces mots-clés s'appliquent à la boucle étiquetée plutôt qu'à la boucle la plus
interne. Les étiquettes de boucle doivent commencer par une apostrophe. Voici un
exemple avec deux boucles imbriquées :


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

<!--
The outer loop has the label `'counting_up`, and it will count up from 0 to 2.
The inner loop without a label counts down from 10 to 9. The first `break` that
doesn't specify a label will exit the inner loop only. The `break
'counting_up;` statement will exit the outer loop. This code prints:
-->

La boucle extérieure a l'étiquette `'counting_up`, et elle comptera de 0 à 2.
La boucle intérieure sans étiquette compte à rebours de 10 à 9. Le premier
`break` qui ne spécifie pas d'étiquette ne quittera que la boucle intérieure.
L'instruction `break 'counting_up;` quittera la boucle extérieure. Ce code
affiche :


```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!--
Old headings. Do not remove or links may break.
-->
<a id="conditional-loops-with-while"></a>

<!--
#### Streamlining Conditional Loops with while
-->

#### Simplifier les boucles conditionnelles avec while

<!--
A program will often need to evaluate a condition within a loop. While the
condition is `true`, the loop runs. When the condition ceases to be `true`, the
program calls `break`, stopping the loop. It's possible to implement behavior
like this using a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you'd like. However, this pattern is so common
that Rust has a built-in language construct for it, called a `while` loop. In
Listing 3-3, we use `while` to loop the program three times, counting down each
time, and then, after the loop, to print a message and exit.
-->

Un programme aura souvent besoin d'évaluer une condition à l'intérieur d'une
boucle. Tant que la condition est `true`, la boucle s'exécute. Lorsque la
condition cesse d'être `true`, le programme appelle `break`, arrêtant la boucle.
Il est possible d'implémenter ce comportement en utilisant une combinaison de
`loop`, `if`, `else` et `break` ; vous pourriez essayer cela maintenant dans un
programme, si vous le souhaitez. Cependant, ce patron est si courant que Rust
dispose d'une construction de langage intégrée pour cela, appelée boucle
`while`. Dans le listing 3-3, nous utilisons `while` pour boucler le programme
trois fois, en comptant à rebours à chaque fois, puis, après la boucle, pour
afficher un message et quitter.

<Listing number="3-3" file-name="src/main.rs" caption="Utiliser une boucle `while` pour exécuter du code tant qu'une condition s'évalue à `true`">


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

<!--
This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it's clearer. While a condition
evaluates to `true`, the code runs; otherwise, it exits the loop.
-->

Cette construction élimine beaucoup d'imbrication qui serait nécessaire si vous
utilisiez `loop`, `if`, `else` et `break`, et elle est plus claire. Tant qu'une
condition s'évalue à `true`, le code s'exécute ; sinon, il quitte la boucle.

<!--
#### Looping Through a Collection with `for`
-->

#### Parcourir une collection avec `for`

<!--
You can choose to use the `while` construct to loop over the elements of a
collection, such as an array. For example, the loop in Listing 3-4 prints each
element in the array `a`.
-->

Vous pouvez choisir d'utiliser la construction `while` pour parcourir les
éléments d'une collection, comme un tableau. Par exemple, la boucle dans le
listing 3-4 affiche chaque élément du tableau `a`.

<Listing number="3-4" file-name="src/main.rs" caption="Parcourir chaque élément d'une collection en utilisant une boucle `while`">


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

<!--
Here, the code counts up through the elements in the array. It starts at index
`0` and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer `true`). Running this code will print every
element in the array:
-->

Ici, le code parcourt les éléments du tableau en incrémentant. Il commence à
l'indice `0` puis boucle jusqu'à atteindre le dernier indice du tableau (c'est-
à-dire quand `index < 5` n'est plus `true`). L'exécution de ce code affichera
chaque élément du tableau :


```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

<!--
All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.
-->

Les cinq valeurs du tableau apparaissent dans le terminal, comme prévu. Même si
`index` atteindra la valeur `5` à un moment donné, la boucle s'arrête de
s'exécuter avant d'essayer de récupérer une sixième valeur du tableau.

<!--
However, this approach is error-prone; we could cause the program to panic if
the index value or test condition is incorrect. For example, if you changed the
definition of the `a` array to have four elements but forgot to update the
condition to `while index < 4`, the code would panic. It's also slow, because
the compiler adds runtime code to perform the conditional check of whether the
index is within the bounds of the array on every iteration through the loop.
-->

Cependant, cette approche est sujette aux erreurs ; nous pourrions provoquer un
panic du programme si la valeur de l'indice ou la condition de test est
incorrecte. Par exemple, si vous changiez la définition du tableau `a` pour
avoir quatre éléments mais oubliiez de mettre à jour la condition en
`while index < 4`, le code provoquerait un panic. C'est également lent, car le
compilateur ajoute du code à l'exécution pour vérifier si l'indice est dans les
limites du tableau à chaque itération de la boucle.

<!--
As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.
-->

Comme alternative plus concise, vous pouvez utiliser une boucle `for` et
exécuter du code pour chaque élément d'une collection. Une boucle `for`
ressemble au code du listing 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="Parcourir chaque élément d'une collection en utilisant une boucle `for`">


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

<!--
When we run this code, we'll see the same output as in Listing 3-4. More
importantly, we've now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items. Machine code generated from `for`
loops can be more efficient as well because the index doesn't need to be
compared to the length of the array at every iteration.
-->

Lorsque nous exécutons ce code, nous verrons la même sortie que dans le listing
3-4. Plus important encore, nous avons maintenant augmenté la sécurité du code
et éliminé le risque de bugs qui pourraient résulter d'un dépassement de la fin
du tableau ou d'un parcours insuffisant manquant certains éléments. Le code
machine généré par les boucles `for` peut également être plus efficace car
l'indice n'a pas besoin d'être comparé à la longueur du tableau à chaque
itération.

<!--
Using the `for` loop, you wouldn't need to remember to change any other code if
you changed the number of values in the array, as you would with the method
used in Listing 3-4.
-->

En utilisant la boucle `for`, vous n'auriez pas besoin de vous souvenir de
modifier d'autre code si vous changiez le nombre de valeurs dans le tableau,
comme ce serait le cas avec la méthode utilisée dans le listing 3-4.

<!--
The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, provided by the standard library, which generates
all numbers in sequence starting from one number and ending before another
number.
-->

La sécurité et la concision des boucles `for` en font la construction de boucle
la plus couramment utilisée en Rust. Même dans les situations où vous voulez
exécuter du code un certain nombre de fois, comme dans l'exemple du compte à
rebours qui utilisait une boucle `while` dans le listing 3-3, la plupart des
Rustaceans utiliseraient une boucle `for`. La façon de le faire serait
d'utiliser un `Range`, fourni par la bibliothèque standard, qui génère tous les
nombres en séquence en commençant par un nombre et en s'arrêtant avant un autre
nombre.

<!--
Here's what the countdown would look like using a `for` loop and another method
we've not yet talked about, `rev`, to reverse the range:
-->

Voici à quoi ressemblerait le compte à rebours en utilisant une boucle `for` et
une autre méthode dont nous n'avons pas encore parlé, `rev`, pour inverser
l'intervalle :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

<!--
This code is a bit nicer, isn't it?
-->

Ce code est un peu plus élégant, n'est-ce pas ?

<!--
## Summary
-->

## Résumé

<!--
You made it! This was a sizable chapter: You learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! To
practice with the concepts discussed in this chapter, try building programs to
do the following:
-->

Vous y êtes arrivé ! C'était un chapitre conséquent : vous avez appris les
variables, les types de données scalaires et composés, les fonctions, les
commentaires, les expressions `if` et les boucles ! Pour vous entraîner avec les
concepts abordés dans ce chapitre, essayez de construire des programmes pour
faire ce qui suit :

<!--
- Convert temperatures between Fahrenheit and Celsius.
- Generate the *n*th Fibonacci number.
- Print the lyrics to the Christmas carol "The Twelve Days of Christmas,"
  taking advantage of the repetition in the song.
-->

- Convertir des températures entre Fahrenheit et Celsius.
- Générer le *n*-ième nombre de Fibonacci.
- Afficher les paroles du chant de Noël "The Twelve Days of Christmas", en
  tirant parti de la répétition dans la chanson.

<!--
When you're ready to move on, we'll talk about a concept in Rust that _doesn't_
commonly exist in other programming languages: ownership.
-->

Quand vous serez prêt à continuer, nous parlerons d'un concept en Rust qui
n'existe _pas_ couramment dans les autres langages de programmation :
l'ownership (la possession).

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
