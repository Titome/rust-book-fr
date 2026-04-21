<!--
## Variables and Mutability
-->

## Les variables et la mutabilité

<!--
As mentioned in the ["Storing Values with
Variables"][storing-values-with-variables] ignore
--> section, by default,
variables are immutable. This is one of many nudges Rust gives you to write
your code in a way that takes advantage of the safety and easy concurrency that
Rust offers. However, you still have the option to make your variables mutable.
Let's explore how and why Rust encourages you to favor immutability and why
sometimes you might want to opt out.
-->

Comme mentionné dans la section [« Stocker des valeurs dans des
variables »][storing-values-with-variables]<!--
ignore
-->, par défaut, les
variables sont immuables. C'est l'un des nombreux coups de pouce que Rust vous
donne pour écrire votre code d'une manière qui tire parti de la sécurité et de
la concurrence facile qu'offre Rust. Cependant, vous avez toujours la
possibilité de rendre vos variables mutables. Voyons comment et pourquoi Rust
vous encourage à privilégier l'immuabilité, et pourquoi vous pourriez parfois
vouloir y renoncer.

<!--
When a variable is immutable, once a value is bound to a name, you can't change
that value. To illustrate this, generate a new project called _variables_ in
your _projects_ directory by using `cargo new variables`.
-->

Lorsqu'une variable est immuable, une fois qu'une valeur est liée à un nom,
vous ne pouvez pas modifier cette valeur. Pour illustrer cela, générez un
nouveau projet appelé _variables_ dans votre répertoire _projects_ en utilisant
`cargo new variables`.

<!--
Then, in your new _variables_ directory, open _src/main.rs_ and replace its
code with the following code, which won't compile just yet:
-->

Ensuite, dans votre nouveau répertoire _variables_, ouvrez _src/main.rs_ et
remplacez son code par le code suivant, qui ne compilera pas encore :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

<!--
Save and run the program using `cargo run`. You should receive an error message
regarding an immutability error, as shown in this output:
-->

Enregistrez et exécutez le programme avec `cargo run`. Vous devriez recevoir un
message d'erreur concernant une erreur d'immuabilité, comme le montre cette
sortie :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

<!--
This example shows how the compiler helps you find errors in your programs.
Compiler errors can be frustrating, but really they only mean your program
isn't safely doing what you want it to do yet; they do _not_ mean that you're
not a good programmer! Experienced Rustaceans still get compiler errors.
-->

Cet exemple montre comment le compilateur vous aide à trouver les erreurs dans
vos programmes. Les erreurs de compilation peuvent être frustrantes, mais en
réalité elles signifient seulement que votre programme ne fait pas encore ce que
vous voulez qu'il fasse de manière sûre ; elles ne signifient _pas_ que vous
n'êtes pas un bon programmeur ! Les Rustacés expérimentés obtiennent eux aussi
des erreurs de compilation.

<!--
You received the error message `` cannot assign twice to immutable variable `x` `` because you tried to assign a second value to the immutable `x` variable.
-->

Vous avez reçu le message d'erreur `` cannot assign twice to immutable variable `x` `` parce que vous avez essayé d'assigner une seconde valeur à la variable immuable `x`.

<!--
It's important that we get compile-time errors when we attempt to change a
value that's designated as immutable, because this very situation can lead to
bugs. If one part of our code operates on the assumption that a value will
never change and another part of our code changes that value, it's possible
that the first part of the code won't do what it was designed to do. The cause
of this kind of bug can be difficult to track down after the fact, especially
when the second piece of code changes the value only _sometimes_. The Rust
compiler guarantees that when you state that a value won't change, it really
won't change, so you don't have to keep track of it yourself. Your code is thus
easier to reason through.
-->

Il est important que nous obtenions des erreurs à la compilation lorsque nous
essayons de modifier une valeur désignée comme immuable, car cette situation
peut mener à des bogues. Si une partie de notre code fonctionne en supposant
qu'une valeur ne changera jamais et qu'une autre partie de notre code modifie
cette valeur, il est possible que la première partie du code ne fasse pas ce
pour quoi elle a été conçue. La cause de ce type de bogue peut être difficile à
retrouver après coup, surtout lorsque la seconde portion de code ne modifie la
valeur que _parfois_. Le compilateur Rust garantit que lorsque vous déclarez
qu'une valeur ne changera pas, elle ne changera vraiment pas, de sorte que vous
n'avez pas à en garder la trace vous-même. Votre code est ainsi plus facile à
comprendre.

<!--
But mutability can be very useful and can make code more convenient to write.
Although variables are immutable by default, you can make them mutable by
adding `mut` in front of the variable name as you did in [Chapter
2][storing-values-with-variables] ignore
-->. Adding `mut` also conveys
intent to future readers of the code by indicating that other parts of the code
will be changing this variable's value.
-->

Mais la mutabilité peut être très utile et peut rendre le code plus pratique à
écrire. Bien que les variables soient immuables par défaut, vous pouvez les
rendre mutables en ajoutant `mut` devant le nom de la variable, comme vous
l'avez fait au [chapitre 2][storing-values-with-variables]<!--
ignore
-->.
L'ajout de `mut` transmet également une intention aux futurs lecteurs du code en
indiquant que d'autres parties du code modifieront la valeur de cette variable.

<!--
For example, let's change _src/main.rs_ to the following:
-->

Par exemple, modifions _src/main.rs_ comme suit :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

<!--
When we run the program now, we get this:
-->

Lorsque nous exécutons le programme maintenant, nous obtenons ceci :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

<!--
We're allowed to change the value bound to `x` from `5` to `6` when `mut` is
used. Ultimately, deciding whether to use mutability or not is up to you and
depends on what you think is clearest in that particular situation.
-->

Nous sommes autorisés à changer la valeur liée à `x` de `5` à `6` lorsque
`mut` est utilisé. En fin de compte, le choix d'utiliser ou non la mutabilité
vous appartient et dépend de ce que vous estimez être le plus clair dans cette
situation particulière.

<!--
Old headings. Do not remove or links may break.
-->
<a id="constants"></a>

<!--
### Declaring Constants
-->

### Déclarer des constantes

<!--
Like immutable variables, _constants_ are values that are bound to a name and
are not allowed to change, but there are a few differences between constants
and variables.
-->

Comme les variables immuables, les _constantes_ sont des valeurs qui sont liées
à un nom et ne peuvent pas être modifiées, mais il existe quelques différences
entre les constantes et les variables.

<!--
First, you aren't allowed to use `mut` with constants. Constants aren't just
immutable by default—they're always immutable. You declare constants using the
`const` keyword instead of the `let` keyword, and the type of the value _must_
be annotated. We'll cover types and type annotations in the next section,
["Data Types"][data-types] ignore
-->, so don't worry about the details
right now. Just know that you must always annotate the type.
-->

Premièrement, vous ne pouvez pas utiliser `mut` avec les constantes. Les
constantes ne sont pas simplement immuables par défaut — elles sont toujours
immuables. Vous déclarez les constantes en utilisant le mot-clé `const` au lieu
du mot-clé `let`, et le type de la valeur _doit_ être annoté. Nous aborderons
les types et les annotations de type dans la section suivante, [« Les types de
données »][data-types]<!--
ignore
-->, donc ne vous souciez pas des détails pour
le moment. Sachez simplement que vous devez toujours annoter le type.

<!--
Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.
-->

Les constantes peuvent être déclarées dans n'importe quelle portée, y compris
la portée globale, ce qui les rend utiles pour les valeurs que de nombreuses
parties du code doivent connaître.

<!--
The last difference is that constants may be set only to a constant expression,
not the result of a value that could only be computed at runtime.
-->

La dernière différence est que les constantes ne peuvent être définies qu'avec
une expression constante, et non avec le résultat d'une valeur qui ne pourrait
être calculée qu'à l'exécution.

<!--
Here's an example of a constant declaration:
-->

Voici un exemple de déclaration de constante :

<!--
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
-->

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

<!--
The constant's name is `THREE_HOURS_IN_SECONDS`, and its value is set to the
result of multiplying 60 (the number of seconds in a minute) by 60 (the number
of minutes in an hour) by 3 (the number of hours we want to count in this
program). Rust's naming convention for constants is to use all uppercase with
underscores between words. The compiler is able to evaluate a limited set of
operations at compile time, which lets us choose to write out this value in a
way that's easier to understand and verify, rather than setting this constant
to the value 10,800. See the [Rust Reference's section on constant
evaluation][const-eval] for more information on what operations can be used
when declaring constants.
-->

Le nom de la constante est `THREE_HOURS_IN_SECONDS`, et sa valeur est définie
comme le résultat de la multiplication de 60 (le nombre de secondes dans une
minute) par 60 (le nombre de minutes dans une heure) par 3 (le nombre d'heures
que nous voulons compter dans ce programme). La convention de nommage de Rust
pour les constantes est d'utiliser des majuscules avec des tirets bas entre les
mots. Le compilateur est capable d'évaluer un ensemble limité d'opérations à la
compilation, ce qui nous permet de choisir d'écrire cette valeur d'une manière
plus facile à comprendre et à vérifier, plutôt que de définir cette constante à
la valeur 10 800. Consultez la [section de la référence Rust sur l'évaluation
des constantes][const-eval] pour plus d'informations sur les opérations
utilisables lors de la déclaration de constantes.

<!--
Constants are valid for the entire time a program runs, within the scope in
which they were declared. This property makes constants useful for values in
your application domain that multiple parts of the program might need to know
about, such as the maximum number of points any player of a game is allowed to
earn, or the speed of light.
-->

Les constantes sont valides pendant toute la durée d'exécution d'un programme,
dans la portée dans laquelle elles ont été déclarées. Cette propriété rend les
constantes utiles pour les valeurs de votre domaine applicatif que plusieurs
parties du programme pourraient avoir besoin de connaître, comme le nombre
maximum de points qu'un joueur peut gagner dans un jeu, ou la vitesse de la
lumière.

<!--
Naming hardcoded values used throughout your program as constants is useful in
conveying the meaning of that value to future maintainers of the code. It also
helps to have only one place in your code that you would need to change if the
hardcoded value needed to be updated in the future.
-->

Nommer les valeurs codées en dur utilisées dans votre programme sous forme de
constantes est utile pour transmettre la signification de cette valeur aux
futurs mainteneurs du code. Cela permet également de n'avoir qu'un seul endroit
dans votre code à modifier si la valeur codée en dur devait être mise à jour à
l'avenir.

<!--
### Shadowing
-->

### Le masquage

<!--
As you saw in the guessing game tutorial in [Chapter
2][comparing-the-guess-to-the-secret-number] ignore
-->, you can declare a
new variable with the same name as a previous variable. Rustaceans say that the
first variable is _shadowed_ by the second, which means that the second
variable is what the compiler will see when you use the name of the variable.
In effect, the second variable overshadows the first, taking any uses of the
variable name to itself until either it itself is shadowed or the scope ends.
We can shadow a variable by using the same variable's name and repeating the
use of the `let` keyword as follows:
-->

Comme vous l'avez vu dans le tutoriel du jeu de devinettes au [chapitre
2][comparing-the-guess-to-the-secret-number]<!--
ignore
-->, vous pouvez
déclarer une nouvelle variable portant le même nom qu'une variable précédente.
Les Rustacés disent que la première variable est _masquée_ par la seconde, ce
qui signifie que la seconde variable est ce que le compilateur verra lorsque
vous utiliserez le nom de la variable. En effet, la seconde variable éclipse la
première, s'appropriant toutes les utilisations du nom de la variable jusqu'à ce
qu'elle soit elle-même masquée ou que la portée se termine. On peut masquer une
variable en utilisant le même nom de variable et en répétant l'utilisation du
mot-clé `let` comme suit :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

<!--
This program first binds `x` to a value of `5`. Then, it creates a new variable
`x` by repeating `let x =`, taking the original value and adding `1` so that
the value of `x` is `6`. Then, within an inner scope created with the curly
brackets, the third `let` statement also shadows `x` and creates a new
variable, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:
-->

Ce programme lie d'abord `x` à la valeur `5`. Puis, il crée une nouvelle
variable `x` en répétant `let x =`, prenant la valeur originale et ajoutant `1`
de sorte que la valeur de `x` est `6`. Ensuite, dans une portée intérieure créée
avec les accolades, la troisième instruction `let` masque également `x` et crée
une nouvelle variable, multipliant la valeur précédente par `2` pour donner à
`x` une valeur de `12`. Quand cette portée se termine, le masquage intérieur
prend fin et `x` redevient `6`. Lorsque nous exécutons ce programme, il
affichera ce qui suit :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

<!--
Shadowing is different from marking a variable as `mut` because we'll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations
on a value but have the variable be immutable after those transformations have
completed.
-->

Le masquage est différent du fait de marquer une variable comme `mut` car nous
obtiendrons une erreur à la compilation si nous essayons accidentellement de
réassigner cette variable sans utiliser le mot-clé `let`. En utilisant `let`,
nous pouvons effectuer quelques transformations sur une valeur tout en rendant
la variable immuable une fois ces transformations terminées.

<!--
The other difference between `mut` and shadowing is that because we're
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, and then we want to store that input as a number:
-->

L'autre différence entre `mut` et le masquage est que, puisque nous créons
effectivement une nouvelle variable lorsque nous utilisons à nouveau le mot-clé
`let`, nous pouvons changer le type de la valeur tout en réutilisant le même
nom. Par exemple, supposons que notre programme demande à un utilisateur de
montrer combien d'espaces il souhaite entre du texte en saisissant des
caractères espace, et que nous voulons ensuite stocker cette entrée sous forme
de nombre :


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

<!--
The first `spaces` variable is a string type, and the second `spaces` variable
is a number type. Shadowing thus spares us from having to come up with
different names, such as `spaces_str` and `spaces_num`; instead, we can reuse
the simpler `spaces` name. However, if we try to use `mut` for this, as shown
here, we'll get a compile-time error:
-->

La première variable `spaces` est de type chaîne de caractères, et la seconde
variable `spaces` est de type numérique. Le masquage nous évite ainsi de devoir
trouver des noms différents, comme `spaces_str` et `spaces_num` ; nous pouvons
plutôt réutiliser le nom plus simple `spaces`. Cependant, si nous essayons
d'utiliser `mut` pour cela, comme montré ici, nous obtiendrons une erreur à la
compilation :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

<!--
The error says we're not allowed to mutate a variable's type:
-->

L'erreur indique que nous ne sommes pas autorisés à changer le type d'une
variable :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

<!--
Now that we've explored how variables work, let's look at more data types they
can have.
-->

Maintenant que nous avons exploré le fonctionnement des variables, examinons les
différents types de données qu'elles peuvent avoir.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
