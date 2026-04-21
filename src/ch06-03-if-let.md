<!--
## Concise Control Flow with `if let` and `let...else`
-->

## Flux de contrôle concis avec `if let` et `let...else`

<!--
The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern while ignoring the rest. Consider the
program in Listing 6-6 that matches on an `Option<u8>` value in the
`config_max` variable but only wants to execute code if the value is the `Some`
variant.
-->

La syntaxe `if let` vous permet de combiner `if` et `let` de manière moins
verbeuse pour gérer les valeurs qui correspondent à un motif tout en ignorant
les autres. Considérez le programme de l'encart 6-6 qui fait un filtrage sur
une valeur `Option<u8>` dans la variable `config_max` mais ne veut exécuter
du code que si la valeur est la variante `Some`.

<Listing number="6-6" caption="Un `match` qui ne se soucie que d'exécuter du code lorsque la valeur est `Some`">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

<!--
If the value is `Some`, we print out the value in the `Some` variant by binding
the value to the variable `max` in the pattern. We don't want to do anything
with the `None` value. To satisfy the `match` expression, we have to add `_ =>
()` after processing just one variant, which is annoying boilerplate code to
add.
-->

Si la valeur est `Some`, nous affichons la valeur de la variante `Some` en
liant la valeur à la variable `max` dans le motif. Nous ne voulons rien faire
avec la valeur `None`. Pour satisfaire l'expression `match`, nous devons
ajouter `_ => ()` après avoir traité une seule variante, ce qui est du code
passe-partout ennuyeux à ajouter.

<!--
Instead, we could write this in a shorter way using `if let`. The following
code behaves the same as the `match` in Listing 6-6:
-->

À la place, nous pourrions écrire cela de manière plus courte en utilisant
`if let`. Le code suivant se comporte de la même manière que le `match` de
l'encart 6-6 :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

<!--
The syntax `if let` takes a pattern and an expression separated by an equal
sign. It works the same way as a `match`, where the expression is given to the
`match` and the pattern is its first arm. In this case, the pattern is
`Some(max)`, and the `max` binds to the value inside the `Some`. We can then
use `max` in the body of the `if let` block in the same way we used `max` in
the corresponding `match` arm. The code in the `if let` block only runs if the
value matches the pattern.
-->

La syntaxe `if let` prend un motif et une expression séparés par un signe
égal. Elle fonctionne de la même manière qu'un `match`, où l'expression est
donnée au `match` et le motif est son premier bras. Dans ce cas, le motif est
`Some(max)`, et `max` se lie à la valeur à l'intérieur du `Some`. Nous pouvons
ensuite utiliser `max` dans le corps du bloc `if let` de la même manière que
nous avons utilisé `max` dans le bras `match` correspondant. Le code dans le
bloc `if let` ne s'exécute que si la valeur correspond au motif.

<!--
Using `if let` means less typing, less indentation, and less boilerplate code.
However, you lose the exhaustive checking `match` enforces that ensures that
you aren't forgetting to handle any cases. Choosing between `match` and `if
let` depends on what you're doing in your particular situation and whether
gaining conciseness is an appropriate trade-off for losing exhaustive checking.
-->

Utiliser `if let` signifie moins de saisie, moins d'indentation et moins de
code passe-partout. Cependant, vous perdez la vérification exhaustive que
`match` impose et qui garantit que vous n'oubliez pas de gérer certains cas.
Le choix entre `match` et `if let` dépend de ce que vous faites dans votre
situation particulière et de savoir si gagner en concision est un compromis
approprié par rapport à la perte de la vérification exhaustive.

<!--
In other words, you can think of `if let` as syntax sugar for a `match` that
runs code when the value matches one pattern and then ignores all other values.
-->

En d'autres termes, vous pouvez considérer `if let` comme du sucre syntaxique
pour un `match` qui exécute du code quand la valeur correspond à un motif puis
ignore toutes les autres valeurs.

<!--
We can include an `else` with an `if let`. The block of code that goes with the
`else` is the same as the block of code that would go with the `_` case in the
`match` expression that is equivalent to the `if let` and `else`. Recall the
`Coin` enum definition in Listing 6-4, where the `Quarter` variant also held a
`UsState` value. If we wanted to count all non-quarter coins we see while also
announcing the state of the quarters, we could do that with a `match`
expression, like this:
-->

Nous pouvons inclure un `else` avec un `if let`. Le bloc de code qui
accompagne le `else` est le même que le bloc de code qui accompagnerait le cas
`_` dans l'expression `match` équivalente au `if let` et `else`. Rappelons la
définition de l'enum `Coin` dans l'encart 6-4, où la variante `Quarter`
contenait aussi une valeur `UsState`. Si nous voulions compter toutes les
pièces qui ne sont pas des quarters tout en annonçant l'État des quarters,
nous pourrions le faire avec une expression `match`, comme ceci :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

<!--
Or we could use an `if let` and `else` expression, like this:
-->

Ou nous pourrions utiliser une expression `if let` et `else`, comme ceci :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

<!--
## Staying on the "Happy Path" with `let...else`
-->

## Rester sur le « chemin heureux » avec `let...else`

<!--
The common pattern is to perform some computation when a value is present and
return a default value otherwise. Continuing with our example of coins with a
`UsState` value, if we wanted to say something funny depending on how old the
state on the quarter was, we might introduce a method on `UsState` to check the
age of a state, like so:
-->

Le motif courant consiste à effectuer un calcul lorsqu'une valeur est présente
et à renvoyer une valeur par défaut dans le cas contraire. En poursuivant avec
notre exemple de pièces avec une valeur `UsState`, si nous voulions dire
quelque chose d'amusant en fonction de l'ancienneté de l'État sur le quarter,
nous pourrions introduire une méthode sur `UsState` pour vérifier l'âge d'un
État, comme ceci :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

<!--
Then, we might use `if let` to match on the type of coin, introducing a `state`
variable within the body of the condition, as in Listing 6-7.
-->

Ensuite, nous pourrions utiliser `if let` pour filtrer sur le type de pièce, en
introduisant une variable `state` dans le corps de la condition, comme dans
l'encart 6-7.

<Listing number="6-7" caption="Vérifier si un État existait en 1900 en utilisant des conditions imbriquées dans un `if let`">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

<!--
That gets the job done, but it has pushed the work into the body of the `if
let` statement, and if the work to be done is more complicated, it might be
hard to follow exactly how the top-level branches relate. We could also take
advantage of the fact that expressions produce a value either to produce the
`state` from the `if let` or to return early, as in Listing 6-8. (You could do
something similar with a `match`, too.)
-->

Cela fait le travail, mais cela a poussé le travail dans le corps de
l'instruction `if let`, et si le travail à faire est plus complexe, il pourrait
être difficile de suivre exactement comment les branches de premier niveau sont
liées. Nous pourrions aussi tirer parti du fait que les expressions produisent
une valeur soit pour produire `state` à partir du `if let`, soit pour retourner
prématurément, comme dans l'encart 6-8. (Vous pourriez aussi faire quelque
chose de similaire avec un `match`.)

<Listing number="6-8" caption="Utiliser `if let` pour produire une valeur ou retourner prématurément">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

<!--
This is a bit annoying to follow in its own way, though! One branch of the `if
let` produces a value, and the other one returns from the function entirely.
-->

C'est cependant un peu pénible à suivre à sa manière ! Une branche du `if let`
produit une valeur, et l'autre retourne entièrement de la fonction.

<!--
To make this common pattern nicer to express, Rust has `let...else`. The
`let...else` syntax takes a pattern on the left side and an expression on the
right, very similar to `if let`, but it does not have an `if` branch, only an
`else` branch. If the pattern matches, it will bind the value from the pattern
in the outer scope. If the pattern does _not_ match, the program will flow into
the `else` arm, which must return from the function.
-->

Pour rendre ce motif courant plus agréable à exprimer, Rust propose
`let...else`. La syntaxe `let...else` prend un motif à gauche et une expression
à droite, de manière très similaire à `if let`, mais elle n'a pas de branche
`if`, seulement une branche `else`. Si le motif correspond, la valeur du motif
sera liée dans la portée extérieure. Si le motif ne correspond _pas_, le
programme entrera dans le bras `else`, qui doit retourner de la fonction.

<!--
In Listing 6-9, you can see how Listing 6-8 looks when using `let...else` in
place of `if let`.
-->

Dans l'encart 6-9, vous pouvez voir à quoi ressemble l'encart 6-8 lorsqu'on
utilise `let...else` à la place de `if let`.

<Listing number="6-9" caption="Utiliser `let...else` pour clarifier le flux à travers la fonction">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

<!--
Notice that it stays on the "happy path" in the main body of the function this
way, without having significantly different control flow for two branches the
way the `if let` did.
-->

Remarquez que de cette manière, le code reste sur le « chemin heureux » dans le
corps principal de la fonction, sans avoir un flux de contrôle significativement
différent pour les deux branches comme le faisait le `if let`.

<!--
If you have a situation in which your program has logic that is too verbose to
express using a `match`, remember that `if let` and `let...else` are in your
Rust toolbox as well.
-->

Si vous avez une situation dans laquelle votre programme a une logique trop
verbeuse à exprimer avec un `match`, rappelez-vous que `if let` et `let...else`
font aussi partie de votre boîte à outils Rust.

<!--
## Summary
-->

## Résumé

<!--
We've now covered how to use enums to create custom types that can be one of a
set of enumerated values. We've shown how the standard library's `Option<T>`
type helps you use the type system to prevent errors. When enum values have
data inside them, you can use `match` or `if let` to extract and use those
values, depending on how many cases you need to handle.
-->

Nous avons maintenant vu comment utiliser les enums pour créer des types
personnalisés qui peuvent être l'une des valeurs d'un ensemble énuméré. Nous
avons montré comment le type `Option<T>` de la bibliothèque standard vous aide
à utiliser le système de types pour prévenir les erreurs. Quand les valeurs
d'enum contiennent des données, vous pouvez utiliser `match` ou `if let` pour
extraire et utiliser ces valeurs, selon le nombre de cas que vous devez gérer.

<!--
Your Rust programs can now express concepts in your domain using structs and
enums. Creating custom types to use in your API ensures type safety: The
compiler will make certain your functions only get values of the type each
function expects.
-->

Vos programmes Rust peuvent maintenant exprimer des concepts de votre domaine
en utilisant des structures et des enums. Créer des types personnalisés à
utiliser dans votre API garantit la sécurité des types : le compilateur
s'assurera que vos fonctions ne reçoivent que des valeurs du type attendu par
chaque fonction.

<!--
In order to provide a well-organized API to your users that is straightforward
to use and only exposes exactly what your users will need, let's now turn to
Rust's modules.
-->

Afin de fournir à vos utilisateurs une API bien organisée, simple à utiliser
et qui n'expose que ce dont vos utilisateurs auront besoin, tournons-nous
maintenant vers les modules de Rust.
