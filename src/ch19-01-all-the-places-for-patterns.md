<!--
## All the Places Patterns Can Be Used
-->

## Tous les endroits où les motifs peuvent être utilisés

<!--
Patterns pop up in a number of places in Rust, and you've been using them a lot
without realizing it! This section discusses all the places where patterns are
valid.
-->

Les motifs apparaissent à de nombreux endroits en Rust, et vous les avez beaucoup utilisés sans vous en rendre compte ! Cette section présente tous les endroits où les motifs sont valides.

<!--
### `match` Arms
-->

### Les branches de `match`

<!--
As discussed in Chapter 6, we use patterns in the arms of `match` expressions.
Formally, `match` expressions are defined as the keyword `match`, a value to
match on, and one or more match arms that consist of a pattern and an
expression to run if the value matches that arm's pattern, like this:
-->

Comme nous l'avons vu au chapitre 6, nous utilisons des motifs dans les branches des expressions `match`. Formellement, les expressions `match` sont définies par le mot-clé `match`, une valeur sur laquelle effectuer la correspondance, et une ou plusieurs branches composées d'un motif et d'une expression à exécuter si la valeur correspond au motif de cette branche, comme ceci :

<!--
Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>VALEUR</em> {
    <em>MOTIF</em> => <em>EXPRESSION</em>,
    <em>MOTIF</em> => <em>EXPRESSION</em>,
    <em>MOTIF</em> => <em>EXPRESSION</em>,
}</code></pre>

<!--
For example, here's the `match` expression from Listing 6-5 that matches on an
`Option<i32>` value in the variable `x`:
-->

Par exemple, voici l'expression `match` de l'encart 6-5 qui effectue une correspondance sur une valeur `Option<i32>` dans la variable `x` :

<!--
```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```
-->

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

<!--
The patterns in this `match` expression are the `None` and `Some(i)` to the
left of each arrow.
-->

Les motifs dans cette expression `match` sont `None` et `Some(i)` à gauche de chaque flèche.

<!--
One requirement for `match` expressions is that they need to be exhaustive in
the sense that all possibilities for the value in the `match` expression must
be accounted for. One way to ensure that you've covered every possibility is to
have a catch-all pattern for the last arm: For example, a variable name
matching any value can never fail and thus covers every remaining case.
-->

Une exigence des expressions `match` est qu'elles doivent être exhaustives, c'est-à-dire que toutes les possibilités pour la valeur dans l'expression `match` doivent être couvertes. Un moyen de s'assurer que vous avez couvert toutes les possibilités est d'avoir un motif attrape-tout pour la dernière branche : par exemple, un nom de variable qui correspond à n'importe quelle valeur ne peut jamais échouer et couvre donc tous les cas restants.

<!--
The particular pattern `_` will match anything, but it never binds to a
variable, so it's often used in the last match arm. The `_` pattern can be
useful when you want to ignore any value not specified, for example. We'll
cover the `_` pattern in more detail in ["Ignoring Values in a
Pattern"][ignoring-values-in-a-pattern] ignore
--> later in this chapter.
-->

Le motif particulier `_` correspond à n'importe quoi, mais il ne se lie jamais à une variable, c'est pourquoi il est souvent utilisé dans la dernière branche de `match`. Le motif `_` peut être utile lorsque vous souhaitez ignorer toute valeur non spécifiée, par exemple. Nous couvrirons le motif `_` plus en détail dans la section ["Ignorer des valeurs dans un motif"][ignoring-values-in-a-pattern]<!--
ignore
--> plus loin dans ce chapitre.

<!--
### `let` Statements
-->

### Les instructions `let`

<!--
Prior to this chapter, we had only explicitly discussed using patterns with
`match` and `if let`, but in fact, we've used patterns in other places as well,
including in `let` statements. For example, consider this straightforward
variable assignment with `let`:
-->

Avant ce chapitre, nous n'avions discuté explicitement de l'utilisation des motifs qu'avec `match` et `if let`, mais en réalité, nous avons utilisé des motifs à d'autres endroits également, notamment dans les instructions `let`. Par exemple, considérez cette assignation simple de variable avec `let` :

<!--
```rust
let x = 5;
```
-->

```rust
let x = 5;
```

<!--
Every time you've used a `let` statement like this you've been using patterns,
although you might not have realized it! More formally, a `let` statement looks
like this:
-->

Chaque fois que vous avez utilisé une instruction `let` comme celle-ci, vous utilisiez des motifs, même si vous ne vous en êtes peut-être pas rendu compte ! Plus formellement, une instruction `let` ressemble à ceci :

<!--
Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>MOTIF</em> = <em>EXPRESSION</em>;</code>
</pre>

<!--
In statements like `let x = 5;` with a variable name in the PATTERN slot, the
variable name is just a particularly simple form of a pattern. Rust compares
the expression against the pattern and assigns any names it finds. So, in the
`let x = 5;` example, `x` is a pattern that means "bind what matches here to
the variable `x`." Because the name `x` is the whole pattern, this pattern
effectively means "bind everything to the variable `x`, whatever the value is."
-->

Dans les instructions comme `let x = 5;` avec un nom de variable à l'emplacement du MOTIF, le nom de variable n'est qu'une forme particulièrement simple de motif. Rust compare l'expression au motif et assigne tous les noms qu'il trouve. Ainsi, dans l'exemple `let x = 5;`, `x` est un motif qui signifie "lier ce qui correspond ici à la variable `x`." Comme le nom `x` constitue l'intégralité du motif, ce motif signifie effectivement "lier tout à la variable `x`, quelle que soit la valeur."

<!--
To see the pattern-matching aspect of `let` more clearly, consider Listing
19-1, which uses a pattern with `let` to destructure a tuple.
-->

Pour voir l'aspect de correspondance de motifs de `let` plus clairement, considérez l'encart 19-1, qui utilise un motif avec `let` pour déstructurer un tuple.

<Listing number="19-1" caption="Utiliser un motif pour déstructurer un tuple et créer trois variables en une seule fois">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

<!--
Here, we match a tuple against a pattern. Rust compares the value `(1, 2, 3)`
to the pattern `(x, y, z)` and sees that the value matches the pattern—that is,
it sees that the number of elements is the same in both—so Rust binds `1` to
`x`, `2` to `y`, and `3` to `z`. You can think of this tuple pattern as nesting
three individual variable patterns inside it.
-->

Ici, nous faisons correspondre un tuple à un motif. Rust compare la valeur `(1, 2, 3)` au motif `(x, y, z)` et constate que la valeur correspond au motif -- c'est-à-dire qu'il voit que le nombre d'éléments est le même dans les deux -- donc Rust lie `1` à `x`, `2` à `y` et `3` à `z`. Vous pouvez considérer ce motif de tuple comme l'imbrication de trois motifs de variables individuels.

<!--
If the number of elements in the pattern doesn't match the number of elements
in the tuple, the overall type won't match and we'll get a compiler error. For
example, Listing 19-2 shows an attempt to destructure a tuple with three
elements into two variables, which won't work.
-->

Si le nombre d'éléments dans le motif ne correspond pas au nombre d'éléments dans le tuple, le type global ne correspondra pas et nous obtiendrons une erreur du compilateur. Par exemple, l'encart 19-2 montre une tentative de déstructurer un tuple de trois éléments en deux variables, ce qui ne fonctionnera pas.

<Listing number="19-2" caption="Construction incorrecte d'un motif dont les variables ne correspondent pas au nombre d'éléments dans le tuple">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

<!--
Attempting to compile this code results in this type error:
-->

Tenter de compiler ce code produit cette erreur de type :


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

<!--
To fix the error, we could ignore one or more of the values in the tuple using
`_` or `..`, as you'll see in the ["Ignoring Values in a
Pattern"][ignoring-values-in-a-pattern] ignore
--> section. If the problem
is that we have too many variables in the pattern, the solution is to make the
types match by removing variables so that the number of variables equals the
number of elements in the tuple.
-->

Pour corriger l'erreur, nous pourrions ignorer une ou plusieurs valeurs du tuple en utilisant `_` ou `..`, comme vous le verrez dans la section ["Ignorer des valeurs dans un motif"][ignoring-values-in-a-pattern]<!--
ignore
-->. Si le problème est que nous avons trop de variables dans le motif, la solution est de faire correspondre les types en supprimant des variables afin que le nombre de variables soit égal au nombre d'éléments dans le tuple.

<!--
### Conditional `if let` Expressions
-->

### Les expressions conditionnelles `if let`

<!--
In Chapter 6, we discussed how to use `if let` expressions mainly as a shorter
way to write the equivalent of a `match` that only matches one case.
Optionally, `if let` can have a corresponding `else` containing code to run if
the pattern in the `if let` doesn't match.
-->

Au chapitre 6, nous avons vu comment utiliser les expressions `if let` principalement comme une manière plus concise d'écrire l'équivalent d'un `match` qui ne correspond qu'à un seul cas. Optionnellement, `if let` peut avoir un `else` correspondant contenant du code à exécuter si le motif du `if let` ne correspond pas.

<!--
Listing 19-3 shows that it's also possible to mix and match `if let`, `else
if`, and `else if let` expressions. Doing so gives us more flexibility than a
`match` expression in which we can express only one value to compare with the
patterns. Also, Rust doesn't require that the conditions in a series of `if
let`, `else if`, and `else if let` arms relate to each other.
-->

L'encart 19-3 montre qu'il est également possible de combiner des expressions `if let`, `else if` et `else if let`. Cela nous donne plus de flexibilité qu'une expression `match` dans laquelle nous ne pouvons exprimer qu'une seule valeur à comparer avec les motifs. De plus, Rust n'exige pas que les conditions d'une série de branches `if let`, `else if` et `else if let` soient liées entre elles.

<!--
The code in Listing 19-3 determines what color to make your background based on
a series of checks for several conditions. For this example, we've created
variables with hardcoded values that a real program might receive from user
input.
-->

Le code de l'encart 19-3 détermine quelle couleur donner à votre arrière-plan en fonction d'une série de vérifications de plusieurs conditions. Pour cet exemple, nous avons créé des variables avec des valeurs codées en dur qu'un vrai programme pourrait recevoir en entrée de l'utilisateur.

<Listing number="19-3" file-name="src/main.rs" caption="Combiner `if let`, `else if`, `else if let` et `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

<!--
If the user specifies a favorite color, that color is used as the background.
If no favorite color is specified and today is Tuesday, the background color is
green. Otherwise, if the user specifies their age as a string and we can parse
it as a number successfully, the color is either purple or orange depending on
the value of the number. If none of these conditions apply, the background
color is blue.
-->

Si l'utilisateur spécifie une couleur préférée, cette couleur est utilisée comme arrière-plan. Si aucune couleur préférée n'est spécifiée et que nous sommes mardi, la couleur d'arrière-plan est le vert. Sinon, si l'utilisateur spécifie son âge sous forme de chaîne de caractères et que nous pouvons l'analyser avec succès comme un nombre, la couleur est soit le violet soit l'orange selon la valeur du nombre. Si aucune de ces conditions ne s'applique, la couleur d'arrière-plan est le bleu.

<!--
This conditional structure lets us support complex requirements. With the
hardcoded values we have here, this example will print `Using purple as the
background color`.
-->

Cette structure conditionnelle nous permet de prendre en charge des exigences complexes. Avec les valeurs codées en dur que nous avons ici, cet exemple affichera `Using purple as the background color`.

<!--
You can see that `if let` can also introduce new variables that shadow existing
variables in the same way that `match` arms can: The line `if let Ok(age) = age`
introduces a new `age` variable that contains the value inside the `Ok` variant,
shadowing the existing `age` variable. This means we need to place the `if age >
30` condition within that block: We can't combine these two conditions into `if
let Ok(age) = age && age > 30`. The new `age` we want to compare to 30 isn't
valid until the new scope starts with the curly bracket.
-->

Vous pouvez voir que `if let` peut aussi introduire de nouvelles variables qui masquent les variables existantes de la même manière que les branches de `match` : la ligne `if let Ok(age) = age` introduit une nouvelle variable `age` qui contient la valeur à l'intérieur de la variante `Ok`, masquant la variable `age` existante. Cela signifie que nous devons placer la condition `if age > 30` à l'intérieur de ce bloc : nous ne pouvons pas combiner ces deux conditions en `if let Ok(age) = age && age > 30`. Le nouveau `age` que nous voulons comparer à 30 n'est pas valide tant que la nouvelle portée ne commence pas avec l'accolade ouvrante.

<!--
The downside of using `if let` expressions is that the compiler doesn't check
for exhaustiveness, whereas with `match` expressions it does. If we omitted the
last `else` block and therefore missed handling some cases, the compiler would
not alert us to the possible logic bug.
-->

L'inconvénient d'utiliser des expressions `if let` est que le compilateur ne vérifie pas l'exhaustivité, alors qu'il le fait avec les expressions `match`. Si nous omettions le dernier bloc `else` et manquions ainsi la gestion de certains cas, le compilateur ne nous alerterait pas du possible bogue logique.

<!--
### `while let` Conditional Loops
-->

### Les boucles conditionnelles `while let`

<!--
Similar in construction to `if let`, the `while let` conditional loop allows a
`while` loop to run for as long as a pattern continues to match. In Listing
19-4, we show a `while let` loop that waits on messages sent between threads,
but in this case checking a `Result` instead of an `Option`.
-->

De construction similaire à `if let`, la boucle conditionnelle `while let` permet à une boucle `while` de s'exécuter tant qu'un motif continue de correspondre. Dans l'encart 19-4, nous montrons une boucle `while let` qui attend des messages envoyés entre des tâches, mais dans ce cas en vérifiant un `Result` au lieu d'un `Option`.

<Listing number="19-4" caption="Utiliser une boucle `while let` pour afficher des valeurs tant que `rx.recv()` retourne `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

<!--
This example prints `1`, `2`, and then `3`. The `recv` method takes the first
message out of the receiver side of the channel and returns an `Ok(value)`. When
we first saw `recv` back in Chapter 16, we unwrapped the error directly, or
we interacted with it as an iterator using a `for` loop. As Listing 19-4 shows,
though, we can also use `while let`, because the `recv` method returns an `Ok`
each time a message arrives, as long as the sender exists, and then produces an
`Err` once the sender side disconnects.
-->

Cet exemple affiche `1`, `2`, puis `3`. La méthode `recv` prend le premier message du côté récepteur du canal et retourne un `Ok(value)`. Lorsque nous avons vu `recv` pour la première fois au chapitre 16, nous avions déballé l'erreur directement, ou nous avions interagi avec lui comme un itérateur en utilisant une boucle `for`. Comme le montre l'encart 19-4, cependant, nous pouvons aussi utiliser `while let`, car la méthode `recv` retourne un `Ok` chaque fois qu'un message arrive, tant que l'émetteur existe, puis produit un `Err` une fois que le côté émetteur se déconnecte.

<!--
### `for` Loops
-->

### Les boucles `for`

<!--
In a `for` loop, the value that directly follows the keyword `for` is a
pattern. For example, in `for x in y`, the `x` is the pattern. Listing 19-5
demonstrates how to use a pattern in a `for` loop to destructure, or break
apart, a tuple as part of the `for` loop.
-->

Dans une boucle `for`, la valeur qui suit directement le mot-clé `for` est un motif. Par exemple, dans `for x in y`, le `x` est le motif. L'encart 19-5 montre comment utiliser un motif dans une boucle `for` pour déstructurer, ou décomposer, un tuple dans le cadre de la boucle `for`.

<Listing number="19-5" caption="Utiliser un motif dans une boucle `for` pour déstructurer un tuple">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

<!--
The code in Listing 19-5 will print the following:
-->

Le code de l'encart 19-5 affichera ce qui suit :


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

<!--
We adapt an iterator using the `enumerate` method so that it produces a value
and the index for that value, placed into a tuple. The first value produced is
the tuple `(0, 'a')`. When this value is matched to the pattern `(index,
value)`, index will be `0` and value will be `'a'`, printing the first line of
the output.
-->

Nous adaptons un itérateur en utilisant la méthode `enumerate` afin qu'il produise une valeur et l'indice de cette valeur, placés dans un tuple. La première valeur produite est le tuple `(0, 'a')`. Lorsque cette valeur est mise en correspondance avec le motif `(index, value)`, index vaudra `0` et value vaudra `'a'`, affichant la première ligne de la sortie.

<!--
### Function Parameters
-->

### Les paramètres de fonction

<!--
Function parameters can also be patterns. The code in Listing 19-6, which
declares a function named `foo` that takes one parameter named `x` of type
`i32`, should by now look familiar.
-->

Les paramètres de fonction peuvent également être des motifs. Le code de l'encart 19-6, qui déclare une fonction nommée `foo` prenant un paramètre nommé `x` de type `i32`, devrait désormais vous sembler familier.

<Listing number="19-6" caption="Une signature de fonction utilisant des motifs dans les paramètres">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

<!--
The `x` part is a pattern! As we did with `let`, we could match a tuple in a
function's arguments to the pattern. Listing 19-7 splits the values in a tuple
as we pass it to a function.
-->

La partie `x` est un motif ! Comme nous l'avons fait avec `let`, nous pourrions faire correspondre un tuple dans les arguments d'une fonction au motif. L'encart 19-7 décompose les valeurs d'un tuple lorsque nous le passons à une fonction.

<Listing number="19-7" file-name="src/main.rs" caption="Une fonction avec des paramètres qui déstructurent un tuple">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

<!--
This code prints `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` is the value `3` and `y` is the value `5`.
-->

Ce code affiche `Current location: (3, 5)`. Les valeurs `&(3, 5)` correspondent au motif `&(x, y)`, donc `x` vaut `3` et `y` vaut `5`.

<!--
We can also use patterns in closure parameter lists in the same way as in
function parameter lists because closures are similar to functions, as
discussed in Chapter 13.
-->

Nous pouvons également utiliser des motifs dans les listes de paramètres des fermetures de la même manière que dans les listes de paramètres des fonctions, car les fermetures sont similaires aux fonctions, comme nous l'avons vu au chapitre 13.

<!--
At this point, you've seen several ways to use patterns, but patterns don't
work the same in every place we can use them. In some places, the patterns must
be irrefutable; in other circumstances, they can be refutable. We'll discuss
these two concepts next.
-->

À ce stade, vous avez vu plusieurs manières d'utiliser les motifs, mais les motifs ne fonctionnent pas de la même façon à chaque endroit où nous pouvons les utiliser. À certains endroits, les motifs doivent être irréfutables ; dans d'autres circonstances, ils peuvent être réfutables. Nous allons discuter de ces deux concepts maintenant.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern
