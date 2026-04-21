<!--
## Refutability: Whether a Pattern Might Fail to Match
-->

## La réfutabilité : quand un motif pourrait ne pas correspondre

<!--
Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are _irrefutable_. An example would be `x` in the
statement `let x = 5;` because `x` matches anything and therefore cannot fail
to match. Patterns that can fail to match for some possible value are
_refutable_. An example would be `Some(x)` in the expression `if let Some(x) =
a_value` because if the value in the `a_value` variable is `None` rather than
`Some`, the `Some(x)` pattern will not match.
-->

Les motifs se présentent sous deux formes : réfutables et irréfutables. Les motifs qui correspondent à toute valeur possible sont _irréfutables_. Un exemple serait `x` dans l'instruction `let x = 5;` car `x` correspond à n'importe quoi et ne peut donc pas échouer. Les motifs qui peuvent échouer pour certaines valeurs possibles sont _réfutables_. Un exemple serait `Some(x)` dans l'expression `if let Some(x) = a_value` car si la valeur dans la variable `a_value` est `None` plutôt que `Some`, le motif `Some(x)` ne correspondra pas.

<!--
Function parameters, `let` statements, and `for` loops can only accept
irrefutable patterns because the program cannot do anything meaningful when
values don't match. The `if let` and `while let` expressions and the
`let...else` statement accept refutable and irrefutable patterns, but the
compiler warns against irrefutable patterns because, by definition, they're
intended to handle possible failure: The functionality of a conditional is in
its ability to perform differently depending on success or failure.
-->

Les paramètres de fonction, les instructions `let` et les boucles `for` ne peuvent accepter que des motifs irréfutables car le programme ne peut rien faire de significatif lorsque les valeurs ne correspondent pas. Les expressions `if let` et `while let` et l'instruction `let...else` acceptent des motifs réfutables et irréfutables, mais le compilateur avertit contre les motifs irréfutables car, par définition, ils sont destinés à gérer un éventuel échec : la fonctionnalité d'une condition réside dans sa capacité à se comporter différemment selon le succès ou l'échec.

<!--
In general, you shouldn't have to worry about the distinction between refutable
and irrefutable patterns; however, you do need to be familiar with the concept
of refutability so that you can respond when you see it in an error message. In
those cases, you'll need to change either the pattern or the construct you're
using the pattern with, depending on the intended behavior of the code.
-->

En général, vous ne devriez pas avoir à vous soucier de la distinction entre motifs réfutables et irréfutables ; cependant, vous devez être familier avec le concept de réfutabilité afin de pouvoir réagir lorsque vous le rencontrez dans un message d'erreur. Dans ces cas, vous devrez modifier soit le motif, soit la construction avec laquelle vous utilisez le motif, selon le comportement souhaité du code.

<!--
Let's look at an example of what happens when we try to use a refutable pattern
where Rust requires an irrefutable pattern and vice versa. Listing 19-8 shows a
`let` statement, but for the pattern, we've specified `Some(x)`, a refutable
pattern. As you might expect, this code will not compile.
-->

Regardons un exemple de ce qui se passe lorsque nous essayons d'utiliser un motif réfutable là où Rust exige un motif irréfutable et vice versa. L'encart 19-8 montre une instruction `let`, mais pour le motif, nous avons spécifié `Some(x)`, un motif réfutable. Comme vous pouvez vous y attendre, ce code ne compilera pas.

<Listing number="19-8" caption="Tentative d'utilisation d'un motif réfutable avec `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

<!--
If `some_option_value` were a `None` value, it would fail to match the pattern
`Some(x)`, meaning the pattern is refutable. However, the `let` statement can
only accept an irrefutable pattern because there is nothing valid the code can
do with a `None` value. At compile time, Rust will complain that we've tried to
use a refutable pattern where an irrefutable pattern is required:
-->

Si `some_option_value` était une valeur `None`, elle ne correspondrait pas au motif `Some(x)`, ce qui signifie que le motif est réfutable. Cependant, l'instruction `let` ne peut accepter qu'un motif irréfutable car le code ne peut rien faire de valide avec une valeur `None`. Au moment de la compilation, Rust se plaindra que nous avons essayé d'utiliser un motif réfutable là où un motif irréfutable est requis :


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

<!--
Because we didn't cover (and couldn't cover!) every valid value with the
pattern `Some(x)`, Rust rightfully produces a compiler error.
-->

Comme nous n'avons pas couvert (et ne pouvions pas couvrir !) toutes les valeurs valides avec le motif `Some(x)`, Rust produit à juste titre une erreur de compilation.

<!--
If we have a refutable pattern where an irrefutable pattern is needed, we can
fix it by changing the code that uses the pattern: Instead of using `let`, we
can use `let...else`. Then, if the pattern doesn't match, the code in the curly
brackets will handle the value. Listing 19-9 shows how to fix the code in
Listing 19-8.
-->

Si nous avons un motif réfutable là où un motif irréfutable est nécessaire, nous pouvons corriger cela en modifiant le code qui utilise le motif : au lieu d'utiliser `let`, nous pouvons utiliser `let...else`. Alors, si le motif ne correspond pas, le code entre les accolades gérera la valeur. L'encart 19-9 montre comment corriger le code de l'encart 19-8.

<Listing number="19-9" caption="Utiliser `let...else` et un bloc avec des motifs réfutables au lieu de `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

<!--
We've given the code an out! This code is perfectly valid, although it means we
cannot use an irrefutable pattern without receiving a warning. If we give
`let...else` a pattern that will always match, such as `x`, as shown in Listing
19-10, the compiler will give a warning.
-->

Nous avons donné au code une porte de sortie ! Ce code est parfaitement valide, bien que cela signifie que nous ne pouvons pas utiliser un motif irréfutable sans recevoir un avertissement. Si nous donnons à `let...else` un motif qui correspondra toujours, comme `x`, comme montré dans l'encart 19-10, le compilateur émettra un avertissement.

<Listing number="19-10" caption="Tentative d'utilisation d'un motif irréfutable avec `let...else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

<!--
Rust complains that it doesn't make sense to use `let...else` with an
irrefutable pattern:
-->

Rust se plaint qu'il n'a pas de sens d'utiliser `let...else` avec un motif irréfutable :


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

<!--
For this reason, match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern. Rust
allows us to use an irrefutable pattern in a `match` with only one arm, but
this syntax isn't particularly useful and could be replaced with a simpler
`let` statement.
-->

Pour cette raison, les branches de `match` doivent utiliser des motifs réfutables, sauf pour la dernière branche, qui doit correspondre à toutes les valeurs restantes avec un motif irréfutable. Rust nous permet d'utiliser un motif irréfutable dans un `match` avec une seule branche, mais cette syntaxe n'est pas particulièrement utile et pourrait être remplacée par une instruction `let` plus simple.

<!--
Now that you know where to use patterns and the difference between refutable
and irrefutable patterns, let's cover all the syntax we can use to create
patterns.
-->

Maintenant que vous savez où utiliser les motifs et la différence entre les motifs réfutables et irréfutables, couvrons toute la syntaxe que nous pouvons utiliser pour créer des motifs.
