<!--
## Pattern Syntax
-->

## La syntaxe des motifs

<!--
In this section, we gather all the syntax that is valid in patterns and discuss
why and when you might want to use each one.
-->

Dans cette section, nous rassemblons toute la syntaxe valide dans les motifs et discutons pourquoi et quand vous pourriez vouloir utiliser chacune d'elles.

<!--
### Matching Literals
-->

### Correspondance avec des littéraux

<!--
As you saw in Chapter 6, you can match patterns against literals directly. The
following code gives some examples:
-->

Comme vous l'avez vu au chapitre 6, vous pouvez faire correspondre des motifs directement avec des littéraux. Le code suivant donne quelques exemples :


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

<!--
This code prints `one` because the value in `x` is `1`. This syntax is useful
when you want your code to take an action if it gets a particular concrete
value.
-->

Ce code affiche `one` car la valeur de `x` est `1`. Cette syntaxe est utile lorsque vous voulez que votre code effectue une action s'il reçoit une valeur concrète particulière.

<!--
### Matching Named Variables
-->

### Correspondance avec des variables nommées

<!--
Named variables are irrefutable patterns that match any value, and we've used
them many times in this book. However, there is a complication when you use
named variables in `match`, `if let`, or `while let` expressions. Because each
of these kinds of expressions starts a new scope, variables declared as part of
a pattern inside these expressions will shadow those with the same name outside
the constructs, as is the case with all variables. In Listing 19-11, we declare
a variable named `x` with the value `Some(5)` and a variable `y` with the value
`10`. We then create a `match` expression on the value `x`. Look at the
patterns in the match arms and `println!` at the end, and try to figure out
what the code will print before running this code or reading further.
-->

Les variables nommées sont des motifs irréfutables qui correspondent à n'importe quelle valeur, et nous les avons utilisées de nombreuses fois dans ce livre. Cependant, il y a une complication lorsque vous utilisez des variables nommées dans des expressions `match`, `if let` ou `while let`. Comme chacun de ces types d'expressions ouvre une nouvelle portée, les variables déclarées en tant que partie d'un motif à l'intérieur de ces expressions masqueront celles ayant le même nom en dehors de la construction, comme c'est le cas pour toutes les variables. Dans l'encart 19-11, nous déclarons une variable nommée `x` avec la valeur `Some(5)` et une variable `y` avec la valeur `10`. Nous créons ensuite une expression `match` sur la valeur `x`. Regardez les motifs dans les branches du match et le `println!` à la fin, et essayez de deviner ce que le code affichera avant de l'exécuter ou de lire plus loin.

<Listing number="19-11" file-name="src/main.rs" caption="Une expression `match` avec une branche qui introduit une nouvelle variable masquant une variable `y` existante">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

<!--
Let's walk through what happens when the `match` expression runs. The pattern
in the first match arm doesn't match the defined value of `x`, so the code
continues.
-->

Voyons ce qui se passe lorsque l'expression `match` s'exécute. Le motif de la première branche ne correspond pas à la valeur définie de `x`, donc le code continue.

<!--
The pattern in the second match arm introduces a new variable named `y` that
will match any value inside a `Some` value. Because we're in a new scope inside
the `match` expression, this is a new `y` variable, not the `y` we declared at
the beginning with the value `10`. This new `y` binding will match any value
inside a `Some`, which is what we have in `x`. Therefore, this new `y` binds to
the inner value of the `Some` in `x`. That value is `5`, so the expression for
that arm executes and prints `Matched, y = 5`.
-->

Le motif de la deuxième branche introduit une nouvelle variable nommée `y` qui correspondra à n'importe quelle valeur à l'intérieur d'une valeur `Some`. Comme nous sommes dans une nouvelle portée à l'intérieur de l'expression `match`, c'est une nouvelle variable `y`, pas le `y` que nous avons déclaré au début avec la valeur `10`. Cette nouvelle liaison `y` correspondra à n'importe quelle valeur à l'intérieur d'un `Some`, ce qui est ce que nous avons dans `x`. Par conséquent, ce nouveau `y` se lie à la valeur interne du `Some` dans `x`. Cette valeur est `5`, donc l'expression de cette branche s'exécute et affiche `Matched, y = 5`.

<!--
If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
two arms wouldn't have matched, so the value would have matched to the
underscore. We didn't introduce the `x` variable in the pattern of the
underscore arm, so the `x` in the expression is still the outer `x` that hasn't
been shadowed. In this hypothetical case, the `match` would print `Default case,
x = None`.
-->

Si `x` avait été une valeur `None` au lieu de `Some(5)`, les motifs des deux premières branches n'auraient pas correspondu, donc la valeur aurait correspondu au tiret bas. Nous n'avons pas introduit la variable `x` dans le motif de la branche avec le tiret bas, donc le `x` dans l'expression est toujours le `x` extérieur qui n'a pas été masqué. Dans ce cas hypothétique, le `match` aurait affiché `Default case, x = None`.

<!--
When the `match` expression is done, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.
-->

Lorsque l'expression `match` est terminée, sa portée se termine, et celle du `y` interne aussi. Le dernier `println!` produit `at the end: x = Some(5), y = 10`.

<!--
To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a new variable that shadows the existing `y`
variable, we would need to use a match guard conditional instead. We'll talk
about match guards later in the ["Adding Conditionals with Match
Guards"](#adding-conditionals-with-match-guards) ignore
--> section.
-->

Pour créer une expression `match` qui compare les valeurs du `x` et du `y` extérieurs, plutôt que d'introduire une nouvelle variable qui masque la variable `y` existante, nous devrions utiliser une condition de garde de correspondance à la place. Nous parlerons des gardes de correspondance plus loin dans la section ["Ajouter des conditions avec les gardes de correspondance"](#adding-conditionals-with-match-guards)<!--
ignore
-->.

<!--
Old headings. Do not remove or links may break.
-->
<a id="multiple-patterns"></a>

<!--
### Matching Multiple Patterns
-->

### Correspondance avec plusieurs motifs

<!--
In `match` expressions, you can match multiple patterns using the `|` syntax,
which is the pattern _or_ operator. For example, in the following code, we match
the value of `x` against the match arms, the first of which has an _or_ option,
meaning if the value of `x` matches either of the values in that arm, that
arm's code will run:
-->

Dans les expressions `match`, vous pouvez faire correspondre plusieurs motifs en utilisant la syntaxe `|`, qui est l'opérateur _ou_ pour les motifs. Par exemple, dans le code suivant, nous faisons correspondre la valeur de `x` aux branches du match, la première ayant une option _ou_, ce qui signifie que si la valeur de `x` correspond à l'une ou l'autre des valeurs de cette branche, le code de cette branche s'exécutera :


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

<!--
This code prints `one or two`.
-->

Ce code affiche `one or two`.

<!--
### Matching Ranges of Values with `..=`
-->

### Correspondance avec des intervalles de valeurs avec `..=`

<!--
The `..=` syntax allows us to match to an inclusive range of values. In the
following code, when a pattern matches any of the values within the given
range, that arm will execute:
-->

La syntaxe `..=` nous permet de faire correspondre un intervalle inclusif de valeurs. Dans le code suivant, lorsqu'un motif correspond à l'une des valeurs de l'intervalle donné, cette branche s'exécutera :


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

<!--
If `x` is `1`, `2`, `3`, `4`, or `5`, the first arm will match. This syntax is
more convenient for multiple match values than using the `|` operator to
express the same idea; if we were to use `|`, we would have to specify `1 | 2 |
3 | 4 | 5`. Specifying a range is much shorter, especially if we want to match,
say, any number between 1 and 1,000!
-->

Si `x` vaut `1`, `2`, `3`, `4` ou `5`, la première branche correspondra. Cette syntaxe est plus pratique pour des correspondances avec plusieurs valeurs que d'utiliser l'opérateur `|` pour exprimer la même idée ; si nous utilisions `|`, nous devrions spécifier `1 | 2 | 3 | 4 | 5`. Spécifier un intervalle est beaucoup plus court, surtout si nous voulons correspondre, disons, à n'importe quel nombre entre 1 et 1 000 !

<!--
The compiler checks that the range isn't empty at compile time, and because the
only types for which Rust can tell if a range is empty or not are `char` and
numeric values, ranges are only allowed with numeric or `char` values.
-->

Le compilateur vérifie que l'intervalle n'est pas vide au moment de la compilation, et comme les seuls types pour lesquels Rust peut déterminer si un intervalle est vide ou non sont `char` et les valeurs numériques, les intervalles ne sont autorisés qu'avec des valeurs numériques ou `char`.

<!--
Here is an example using ranges of `char` values:
-->

Voici un exemple utilisant des intervalles de valeurs `char` :


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

<!--
Rust can tell that `'c'` is within the first pattern's range and prints `early
ASCII letter`.
-->

Rust peut déterminer que `'c'` se trouve dans l'intervalle du premier motif et affiche `early ASCII letter`.

<!--
### Destructuring to Break Apart Values
-->

### La déstructuration pour décomposer des valeurs

<!--
We can also use patterns to destructure structs, enums, and tuples to use
different parts of these values. Let's walk through each value.
-->

Nous pouvons également utiliser des motifs pour déstructurer des structures, des enums et des tuples afin d'utiliser différentes parties de ces valeurs. Parcourons chaque type de valeur.

<!--
Old headings. Do not remove or links may break.
-->

<a id="destructuring-structs"></a>

<!--
#### Structs
-->

#### Les structures

<!--
Listing 19-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement.
-->

L'encart 19-12 montre une structure `Point` avec deux champs, `x` et `y`, que nous pouvons décomposer en utilisant un motif avec une instruction `let`.

<Listing number="19-12" file-name="src/main.rs" caption="Déstructurer les champs d'une structure en variables séparées">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

<!--
This code creates the variables `a` and `b` that match the values of the `x`
and `y` fields of the `p` struct. This example shows that the names of the
variables in the pattern don't have to match the field names of the struct.
However, it's common to match the variable names to the field names to make it
easier to remember which variables came from which fields. Because of this
common usage, and because writing `let Point { x: x, y: y } = p;` contains a
lot of duplication, Rust has a shorthand for patterns that match struct fields:
You only need to list the name of the struct field, and the variables created
from the pattern will have the same names. Listing 19-13 behaves in the same
way as the code in Listing 19-12, but the variables created in the `let`
pattern are `x` and `y` instead of `a` and `b`.
-->

Ce code crée les variables `a` et `b` qui correspondent aux valeurs des champs `x` et `y` de la structure `p`. Cet exemple montre que les noms des variables dans le motif n'ont pas besoin de correspondre aux noms des champs de la structure. Cependant, il est courant de faire correspondre les noms de variables aux noms de champs pour faciliter la mémorisation de quelles variables proviennent de quels champs. En raison de cet usage courant, et parce qu'écrire `let Point { x: x, y: y } = p;` contient beaucoup de duplication, Rust propose un raccourci pour les motifs qui correspondent aux champs de structures : vous n'avez qu'à lister le nom du champ de la structure, et les variables créées à partir du motif auront les mêmes noms. L'encart 19-13 se comporte de la même manière que le code de l'encart 19-12, mais les variables créées dans le motif `let` sont `x` et `y` au lieu de `a` et `b`.

<Listing number="19-13" file-name="src/main.rs" caption="Déstructurer les champs d'une structure en utilisant le raccourci des champs de structure">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

<!--
This code creates the variables `x` and `y` that match the `x` and `y` fields
of the `p` variable. The outcome is that the variables `x` and `y` contain the
values from the `p` struct.
-->

Ce code crée les variables `x` et `y` qui correspondent aux champs `x` et `y` de la variable `p`. Le résultat est que les variables `x` et `y` contiennent les valeurs de la structure `p`.

<!--
We can also destructure with literal values as part of the struct pattern
rather than creating variables for all the fields. Doing so allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.
-->

Nous pouvons aussi déstructurer avec des valeurs littérales dans le motif de la structure plutôt que de créer des variables pour tous les champs. Cela nous permet de tester certains champs pour des valeurs particulières tout en créant des variables pour déstructurer les autres champs.

<!--
In Listing 19-14, we have a `match` expression that separates `Point` values
into three cases: points that lie directly on the `x` axis (which is true when
`y = 0`), on the `y` axis (`x = 0`), or on neither axis.
-->

Dans l'encart 19-14, nous avons une expression `match` qui sépare les valeurs `Point` en trois cas : les points qui se trouvent directement sur l'axe `x` (ce qui est vrai lorsque `y = 0`), sur l'axe `y` (`x = 0`), ou sur aucun des deux axes.

<Listing number="19-14" file-name="src/main.rs" caption="Déstructurer et faire correspondre des valeurs littérales dans un seul motif">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

<!--
The first arm will match any point that lies on the `x` axis by specifying that
the `y` field matches if its value matches the literal `0`. The pattern still
creates an `x` variable that we can use in the code for this arm.
-->

La première branche correspondra à tout point situé sur l'axe `x` en spécifiant que le champ `y` correspond si sa valeur correspond au littéral `0`. Le motif crée toujours une variable `x` que nous pouvons utiliser dans le code de cette branche.

<!--
Similarly, the second arm matches any point on the `y` axis by specifying that
the `x` field matches if its value is `0` and creates a variable `y` for the
value of the `y` field. The third arm doesn't specify any literals, so it
matches any other `Point` and creates variables for both the `x` and `y` fields.
-->

De même, la deuxième branche correspond à tout point sur l'axe `y` en spécifiant que le champ `x` correspond si sa valeur est `0` et crée une variable `y` pour la valeur du champ `y`. La troisième branche ne spécifie aucun littéral, elle correspond donc à tout autre `Point` et crée des variables pour les champs `x` et `y`.

<!--
In this example, the value `p` matches the second arm by virtue of `x`
containing a `0`, so this code will print `On the y axis at 7`.
-->

Dans cet exemple, la valeur `p` correspond à la deuxième branche car `x` contient `0`, donc ce code affichera `On the y axis at 7`.

<!--
Remember that a `match` expression stops checking arms once it has found the
first matching pattern, so even though `Point { x: 0, y: 0 }` is on the `x` axis
and the `y` axis, this code would only print `On the x axis at 0`.
-->

Rappelez-vous qu'une expression `match` arrête de vérifier les branches dès qu'elle a trouvé le premier motif correspondant, donc même si `Point { x: 0, y: 0 }` est sur l'axe `x` et l'axe `y`, ce code n'afficherait que `On the x axis at 0`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="destructuring-enums"></a>

<!--
#### Enums
-->

#### Les enums

<!--
We've destructured enums in this book (for example, Listing 6-5 in Chapter 6),
but we haven't yet explicitly discussed that the pattern to destructure an enum
corresponds to the way the data stored within the enum is defined. As an
example, in Listing 19-15, we use the `Message` enum from Listing 6-2 and write
a `match` with patterns that will destructure each inner value.
-->

Nous avons déstructuré des enums dans ce livre (par exemple, l'encart 6-5 au chapitre 6), mais nous n'avons pas encore discuté explicitement du fait que le motif pour déstructurer un enum correspond à la manière dont les données stockées dans l'enum sont définies. À titre d'exemple, dans l'encart 19-15, nous utilisons l'enum `Message` de l'encart 6-2 et écrivons un `match` avec des motifs qui déstructureront chaque valeur interne.

<Listing number="19-15" file-name="src/main.rs" caption="Déstructurer des variantes d'enum contenant différents types de valeurs">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

<!--
This code will print `Change color to red 0, green 160, and blue 255`. Try
changing the value of `msg` to see the code from the other arms run.
-->

Ce code affichera `Change color to red 0, green 160, and blue 255`. Essayez de changer la valeur de `msg` pour voir le code des autres branches s'exécuter.

<!--
For enum variants without any data, like `Message::Quit`, we can't destructure
the value any further. We can only match on the literal `Message::Quit` value,
and no variables are in that pattern.
-->

Pour les variantes d'enum sans données, comme `Message::Quit`, nous ne pouvons pas déstructurer davantage la valeur. Nous ne pouvons que correspondre à la valeur littérale `Message::Quit`, et il n'y a pas de variables dans ce motif.

<!--
For struct-like enum variants, such as `Message::Move`, we can use a pattern
similar to the pattern we specify to match structs. After the variant name, we
place curly brackets and then list the fields with variables so that we break
apart the pieces to use in the code for this arm. Here we use the shorthand
form as we did in Listing 19-13.
-->

Pour les variantes d'enum de type structure, comme `Message::Move`, nous pouvons utiliser un motif similaire à celui que nous spécifions pour correspondre aux structures. Après le nom de la variante, nous plaçons des accolades, puis listons les champs avec des variables afin de décomposer les éléments à utiliser dans le code de cette branche. Ici, nous utilisons la forme raccourcie comme dans l'encart 19-13.

<!--
For tuple-like enum variants, like `Message::Write` that holds a tuple with one
element and `Message::ChangeColor` that holds a tuple with three elements, the
pattern is similar to the pattern we specify to match tuples. The number of
variables in the pattern must match the number of elements in the variant we're
matching.
-->

Pour les variantes d'enum de type tuple, comme `Message::Write` qui contient un tuple avec un élément et `Message::ChangeColor` qui contient un tuple avec trois éléments, le motif est similaire à celui que nous spécifions pour correspondre aux tuples. Le nombre de variables dans le motif doit correspondre au nombre d'éléments dans la variante que nous faisons correspondre.

<!--
Old headings. Do not remove or links may break.
-->

<a id="destructuring-nested-structs-and-enums"></a>

<!--
#### Nested Structs and Enums
-->

#### Les structures et enums imbriqués

<!--
So far, our examples have all been matching structs or enums one level deep,
but matching can work on nested items too! For example, we can refactor the
code in Listing 19-15 to support RGB and HSV colors in the `ChangeColor`
message, as shown in Listing 19-16.
-->

Jusqu'à présent, nos exemples ont tous fait correspondre des structures ou des enums à un seul niveau de profondeur, mais la correspondance peut aussi fonctionner sur des éléments imbriqués ! Par exemple, nous pouvons remanier le code de l'encart 19-15 pour prendre en charge les couleurs RGB et HSV dans le message `ChangeColor`, comme montré dans l'encart 19-16.

<Listing number="19-16" caption="Correspondance avec des enums imbriqués">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

<!--
The pattern of the first arm in the `match` expression matches a
`Message::ChangeColor` enum variant that contains a `Color::Rgb` variant; then,
the pattern binds to the three inner `i32` values. The pattern of the second
arm also matches a `Message::ChangeColor` enum variant, but the inner enum
matches `Color::Hsv` instead. We can specify these complex conditions in one
`match` expression, even though two enums are involved.
-->

Le motif de la première branche dans l'expression `match` correspond à une variante d'enum `Message::ChangeColor` qui contient une variante `Color::Rgb` ; ensuite, le motif se lie aux trois valeurs `i32` internes. Le motif de la deuxième branche correspond aussi à une variante d'enum `Message::ChangeColor`, mais l'enum interne correspond à `Color::Hsv` à la place. Nous pouvons spécifier ces conditions complexes dans une seule expression `match`, même si deux enums sont impliqués.

<!--
Old headings. Do not remove or links may break.
-->

<a id="destructuring-structs-and-tuples"></a>

<!--
#### Structs and Tuples
-->

#### Les structures et les tuples

<!--
We can mix, match, and nest destructuring patterns in even more complex ways.
The following example shows a complicated destructure where we nest structs and
tuples inside a tuple and destructure all the primitive values out:
-->

Nous pouvons combiner, faire correspondre et imbriquer des motifs de déstructuration de manières encore plus complexes. L'exemple suivant montre une déstructuration compliquée où nous imbriquons des structures et des tuples dans un tuple et déstructurons toutes les valeurs primitives :


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

<!--
This code lets us break complex types into their component parts so that we can
use the values we're interested in separately.
-->

Ce code nous permet de décomposer des types complexes en leurs éléments constitutifs afin que nous puissions utiliser séparément les valeurs qui nous intéressent.

<!--
Destructuring with patterns is a convenient way to use pieces of values, such
as the value from each field in a struct, separately from each other.
-->

La déstructuration avec des motifs est un moyen pratique d'utiliser des morceaux de valeurs, comme la valeur de chaque champ d'une structure, séparément les uns des autres.

<!--
### Ignoring Values in a Pattern
-->

### Ignorer des valeurs dans un motif

<!--
You've seen that it's sometimes useful to ignore values in a pattern, such as
in the last arm of a `match`, to get a catch-all that doesn't actually do
anything but does account for all remaining possible values. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which you've seen), using the `_` pattern within another pattern,
using a name that starts with an underscore, or using `..` to ignore remaining
parts of a value. Let's explore how and why to use each of these patterns.
-->

Vous avez vu qu'il est parfois utile d'ignorer des valeurs dans un motif, comme dans la dernière branche d'un `match`, pour obtenir un attrape-tout qui ne fait rien en pratique mais couvre toutes les valeurs possibles restantes. Il existe plusieurs manières d'ignorer des valeurs entières ou des parties de valeurs dans un motif : en utilisant le motif `_` (que vous avez déjà vu), en utilisant le motif `_` à l'intérieur d'un autre motif, en utilisant un nom commençant par un tiret bas, ou en utilisant `..` pour ignorer les parties restantes d'une valeur. Explorons comment et pourquoi utiliser chacun de ces motifs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ignoring-an-entire-value-with-_"></a>

<!--
#### An Entire Value with `_`
-->

#### Une valeur entière avec `_`

<!--
We've used the underscore as a wildcard pattern that will match any value but
not bind to the value. This is especially useful as the last arm in a `match`
expression, but we can also use it in any pattern, including function
parameters, as shown in Listing 19-17.
-->

Nous avons utilisé le tiret bas comme motif joker qui correspond à n'importe quelle valeur sans se lier à la valeur. C'est particulièrement utile comme dernière branche d'une expression `match`, mais nous pouvons aussi l'utiliser dans n'importe quel motif, y compris les paramètres de fonction, comme montré dans l'encart 19-17.

<Listing number="19-17" file-name="src/main.rs" caption="Utiliser `_` dans une signature de fonction">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

<!--
This code will completely ignore the value `3` passed as the first argument,
and will print `This code only uses the y parameter: 4`.
-->

Ce code ignorera complètement la valeur `3` passée comme premier argument, et affichera `This code only uses the y parameter: 4`.

<!--
In most cases when you no longer need a particular function parameter, you
would change the signature so that it doesn't include the unused parameter.
Ignoring a function parameter can be especially useful in cases when, for
example, you're implementing a trait when you need a certain type signature but
the function body in your implementation doesn't need one of the parameters.
You then avoid getting a compiler warning about unused function parameters, as
you would if you used a name instead.
-->

Dans la plupart des cas, lorsque vous n'avez plus besoin d'un paramètre de fonction particulier, vous changeriez la signature pour qu'elle n'inclue pas le paramètre inutilisé. Ignorer un paramètre de fonction peut être particulièrement utile dans les cas où, par exemple, vous implémentez un trait nécessitant une certaine signature de type mais le corps de la fonction dans votre implémentation n'a pas besoin de l'un des paramètres. Vous évitez alors un avertissement du compilateur concernant les paramètres de fonction inutilisés, comme cela se produirait si vous utilisiez un nom à la place.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>

<!--
#### Parts of a Value with a Nested `_`
-->

#### Des parties d'une valeur avec un `_` imbriqué

<!--
We can also use `_` inside another pattern to ignore just part of a value, for
example, when we want to test for only part of a value but have no use for the
other parts in the corresponding code we want to run. Listing 19-18 shows code
responsible for managing a setting's value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting but can unset the setting and give it a value if it is currently unset.
-->

Nous pouvons aussi utiliser `_` à l'intérieur d'un autre motif pour ignorer seulement une partie d'une valeur, par exemple, lorsque nous voulons tester seulement une partie d'une valeur mais n'avons pas besoin des autres parties dans le code correspondant que nous voulons exécuter. L'encart 19-18 montre du code chargé de gérer la valeur d'un paramètre. Les exigences métier sont que l'utilisateur ne doit pas être autorisé à écraser une personnalisation existante d'un paramètre, mais peut annuler le paramètre et lui donner une valeur s'il n'est pas actuellement défini.

<Listing number="19-18" caption="Utiliser un tiret bas dans des motifs qui correspondent à des variantes `Some` lorsque nous n'avons pas besoin d'utiliser la valeur à l'intérieur du `Some`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

<!--
This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`. In the first match arm, we don't need to match on or use
the values inside either `Some` variant, but we do need to test for the case
when `setting_value` and `new_setting_value` are the `Some` variant. In that
case, we print the reason for not changing `setting_value`, and it doesn't get
changed.
-->

Ce code affichera `Can't overwrite an existing customized value` puis `setting is Some(5)`. Dans la première branche du match, nous n'avons pas besoin de faire correspondre ou d'utiliser les valeurs à l'intérieur des variantes `Some`, mais nous devons tester le cas où `setting_value` et `new_setting_value` sont des variantes `Some`. Dans ce cas, nous affichons la raison pour laquelle nous ne changeons pas `setting_value`, et elle n'est pas modifiée.

<!--
In all other cases (if either `setting_value` or `new_setting_value` is `None`)
expressed by the `_` pattern in the second arm, we want to allow
`new_setting_value` to become `setting_value`.
-->

Dans tous les autres cas (si soit `setting_value` soit `new_setting_value` est `None`) exprimés par le motif `_` dans la deuxième branche, nous voulons permettre à `new_setting_value` de devenir `setting_value`.

<!--
We can also use underscores in multiple places within one pattern to ignore
particular values. Listing 19-19 shows an example of ignoring the second and
fourth values in a tuple of five items.
-->

Nous pouvons aussi utiliser des tirets bas à plusieurs endroits dans un seul motif pour ignorer des valeurs particulières. L'encart 19-19 montre un exemple d'ignorance de la deuxième et de la quatrième valeurs dans un tuple de cinq éléments.

<Listing number="19-19" caption="Ignorer plusieurs parties d'un tuple">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

<!--
This code will print `Some numbers: 2, 8, 32`, and the values `4` and `16` will
be ignored.
-->

Ce code affichera `Some numbers: 2, 8, 32`, et les valeurs `4` et `16` seront ignorées.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ignoring-an-unused-variable-by-starting-its-name-with-_"></a>

<!--
#### An Unused Variable by Starting Its Name with `_`
-->

#### Une variable inutilisée en commençant son nom par `_`

<!--
If you create a variable but don't use it anywhere, Rust will usually issue a
warning because an unused variable could be a bug. However, sometimes it's
useful to be able to create a variable you won't use yet, such as when you're
prototyping or just starting a project. In this situation, you can tell Rust
not to warn you about the unused variable by starting the name of the variable
with an underscore. In Listing 19-20, we create two unused variables, but when
we compile this code, we should only get a warning about one of them.
-->

Si vous créez une variable mais ne l'utilisez nulle part, Rust émettra généralement un avertissement car une variable inutilisée pourrait être un bogue. Cependant, il est parfois utile de pouvoir créer une variable que vous n'utiliserez pas encore, par exemple lorsque vous faites du prototypage ou que vous démarrez un projet. Dans cette situation, vous pouvez dire à Rust de ne pas vous avertir de la variable inutilisée en commençant le nom de la variable par un tiret bas. Dans l'encart 19-20, nous créons deux variables inutilisées, mais lorsque nous compilons ce code, nous ne devrions recevoir un avertissement que pour l'une d'entre elles.

<Listing number="19-20" file-name="src/main.rs" caption="Commencer un nom de variable par un tiret bas pour éviter les avertissements de variable inutilisée">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

<!--
Here, we get a warning about not using the variable `y`, but we don't get a
warning about not using `_x`.
-->

Ici, nous recevons un avertissement pour la non-utilisation de la variable `y`, mais nous n'en recevons pas pour la non-utilisation de `_x`.

<!--
Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. The syntax `_x` still binds the value to the
variable, whereas `_` doesn't bind at all. To show a case where this
distinction matters, Listing 19-21 will provide us with an error.
-->

Notez qu'il y a une différence subtile entre utiliser uniquement `_` et utiliser un nom qui commence par un tiret bas. La syntaxe `_x` lie toujours la valeur à la variable, tandis que `_` ne lie pas du tout. Pour montrer un cas où cette distinction est importante, l'encart 19-21 nous donnera une erreur.

<Listing number="19-21" caption="Une variable inutilisée commençant par un tiret bas lie toujours la valeur, ce qui pourrait prendre possession de la valeur.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

<!--
We'll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. However, using the underscore by itself
doesn't ever bind to the value. Listing 19-22 will compile without any errors
because `s` doesn't get moved into `_`.
-->

Nous recevrons une erreur car la valeur `s` sera toujours déplacée dans `_s`, ce qui nous empêche d'utiliser `s` à nouveau. Cependant, utiliser le tiret bas seul ne lie jamais la valeur. L'encart 19-22 compilera sans erreur car `s` n'est pas déplacé dans `_`.

<Listing number="19-22" caption="Utiliser un tiret bas ne lie pas la valeur.">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

<!--
This code works just fine because we never bind `s` to anything; it isn't moved.
-->

Ce code fonctionne parfaitement car nous ne lions jamais `s` à quoi que ce soit ; il n'est pas déplacé.

<a id="ignoring-remaining-parts-of-a-value-with-"></a>

<!--
#### Remaining Parts of a Value with `..`
-->

#### Les parties restantes d'une valeur avec `..`

<!--
With values that have many parts, we can use the `..` syntax to use specific
parts and ignore the rest, avoiding the need to list underscores for each
ignored value. The `..` pattern ignores any parts of a value that we haven't
explicitly matched in the rest of the pattern. In Listing 19-23, we have a
`Point` struct that holds a coordinate in three-dimensional space. In the
`match` expression, we want to operate only on the `x` coordinate and ignore
the values in the `y` and `z` fields.
-->

Avec des valeurs qui ont de nombreuses parties, nous pouvons utiliser la syntaxe `..` pour utiliser des parties spécifiques et ignorer le reste, évitant ainsi de devoir lister des tirets bas pour chaque valeur ignorée. Le motif `..` ignore toutes les parties d'une valeur que nous n'avons pas explicitement fait correspondre dans le reste du motif. Dans l'encart 19-23, nous avons une structure `Point` qui contient une coordonnée dans l'espace tridimensionnel. Dans l'expression `match`, nous voulons opérer uniquement sur la coordonnée `x` et ignorer les valeurs des champs `y` et `z`.

<Listing number="19-23" caption="Ignorer tous les champs d'un `Point` sauf `x` en utilisant `..`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

<!--
We list the `x` value and then just include the `..` pattern. This is quicker
than having to list `y: _` and `z: _`, particularly when we're working with
structs that have lots of fields in situations where only one or two fields are
relevant.
-->

Nous listons la valeur `x` puis incluons simplement le motif `..`. C'est plus rapide que de devoir lister `y: _` et `z: _`, en particulier lorsque nous travaillons avec des structures ayant de nombreux champs dans des situations où seuls un ou deux champs sont pertinents.

<!--
The syntax `..` will expand to as many values as it needs to be. Listing 19-24
shows how to use `..` with a tuple.
-->

La syntaxe `..` s'étendra à autant de valeurs que nécessaire. L'encart 19-24 montre comment utiliser `..` avec un tuple.

<Listing number="19-24" file-name="src/main.rs" caption="Ne faire correspondre que la première et la dernière valeur d'un tuple et ignorer toutes les autres valeurs">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

<!--
In this code, the first and last values are matched with `first` and `last`.
The `..` will match and ignore everything in the middle.
-->

Dans ce code, la première et la dernière valeur sont mises en correspondance avec `first` et `last`. Le `..` correspondra et ignorera tout ce qui se trouve au milieu.

<!--
However, using `..` must be unambiguous. If it is unclear which values are
intended for matching and which should be ignored, Rust will give us an error.
Listing 19-25 shows an example of using `..` ambiguously, so it will not
compile.
-->

Cependant, l'utilisation de `..` doit être non ambiguë. S'il n'est pas clair quelles valeurs sont destinées à la correspondance et lesquelles doivent être ignorées, Rust nous donnera une erreur. L'encart 19-25 montre un exemple d'utilisation ambiguë de `..`, qui ne compilera donc pas.

<Listing number="19-25" file-name="src/main.rs" caption="Une tentative d'utilisation de `..` de manière ambiguë">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

<!--
When we compile this example, we get this error:
-->

Lorsque nous compilons cet exemple, nous obtenons cette erreur :


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

<!--
It's impossible for Rust to determine how many values in the tuple to ignore
before matching a value with `second` and then how many further values to
ignore thereafter. This code could mean that we want to ignore `2`, bind
`second` to `4`, and then ignore `8`, `16`, and `32`; or that we want to ignore
`2` and `4`, bind `second` to `8`, and then ignore `16` and `32`; and so forth.
The variable name `second` doesn't mean anything special to Rust, so we get a
compiler error because using `..` in two places like this is ambiguous.
-->

Il est impossible pour Rust de déterminer combien de valeurs du tuple ignorer avant de faire correspondre une valeur avec `second` et combien de valeurs supplémentaires ignorer ensuite. Ce code pourrait signifier que nous voulons ignorer `2`, lier `second` à `4`, puis ignorer `8`, `16` et `32` ; ou que nous voulons ignorer `2` et `4`, lier `second` à `8`, puis ignorer `16` et `32` ; et ainsi de suite. Le nom de variable `second` n'a aucune signification particulière pour Rust, donc nous obtenons une erreur du compilateur car utiliser `..` à deux endroits comme cela est ambigu.

<!--
Old headings. Do not remove or links may break.
-->

<a id="extra-conditionals-with-match-guards"></a>

<!--
### Adding Conditionals with Match Guards
-->

### Ajouter des conditions avec les gardes de correspondance

<!--
A _match guard_ is an additional `if` condition, specified after the pattern in
a `match` arm, that must also match for that arm to be chosen. Match guards are
useful for expressing more complex ideas than a pattern alone allows. Note,
however, that they are only available in `match` expressions, not `if let` or
`while let` expressions.
-->

Une _garde de correspondance_ (match guard) est une condition `if` supplémentaire, spécifiée après le motif dans une branche de `match`, qui doit également être satisfaite pour que cette branche soit choisie. Les gardes de correspondance sont utiles pour exprimer des idées plus complexes que ce qu'un motif seul permet. Notez, cependant, qu'elles ne sont disponibles que dans les expressions `match`, pas dans les expressions `if let` ou `while let`.

<!--
The condition can use variables created in the pattern. Listing 19-26 shows a
`match` where the first arm has the pattern `Some(x)` and also has a match
guard of `if x % 2 == 0` (which will be `true` if the number is even).
-->

La condition peut utiliser des variables créées dans le motif. L'encart 19-26 montre un `match` où la première branche a le motif `Some(x)` et aussi une garde de correspondance `if x % 2 == 0` (qui sera `true` si le nombre est pair).

<Listing number="19-26" caption="Ajouter une garde de correspondance à un motif">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

<!--
This example will print `The number 4 is even`. When `num` is compared to the
pattern in the first arm, it matches because `Some(4)` matches `Some(x)`. Then,
the match guard checks whether the remainder of dividing `x` by 2 is equal to
0, and because it is, the first arm is selected.
-->

Cet exemple affichera `The number 4 is even`. Lorsque `num` est comparé au motif de la première branche, il correspond car `Some(4)` correspond à `Some(x)`. Ensuite, la garde de correspondance vérifie si le reste de la division de `x` par 2 est égal à 0, et comme c'est le cas, la première branche est sélectionnée.

<!--
If `num` had been `Some(5)` instead, the match guard in the first arm would
have been `false` because the remainder of 5 divided by 2 is 1, which is not
equal to 0. Rust would then go to the second arm, which would match because the
second arm doesn't have a match guard and therefore matches any `Some` variant.
-->

Si `num` avait été `Some(5)`, la garde de correspondance de la première branche aurait été `false` car le reste de 5 divisé par 2 est 1, ce qui n'est pas égal à 0. Rust passerait alors à la deuxième branche, qui correspondrait car la deuxième branche n'a pas de garde de correspondance et correspond donc à n'importe quelle variante `Some`.

<!--
There is no way to express the `if x % 2 == 0` condition within a pattern, so
the match guard gives us the ability to express this logic. The downside of
this additional expressiveness is that the compiler doesn't try to check for
exhaustiveness when match guard expressions are involved.
-->

Il n'y a aucun moyen d'exprimer la condition `if x % 2 == 0` dans un motif, donc la garde de correspondance nous donne la possibilité d'exprimer cette logique. L'inconvénient de cette expressivité supplémentaire est que le compilateur n'essaie pas de vérifier l'exhaustivité lorsque des expressions de garde de correspondance sont impliquées.

<!--
When discussing Listing 19-11, we mentioned that we could use match guards to
solve our pattern-shadowing problem. Recall that we created a new variable
inside the pattern in the `match` expression instead of using the variable
outside the `match`. That new variable meant we couldn't test against the value
of the outer variable. Listing 19-27 shows how we can use a match guard to fix
this problem.
-->

Lorsque nous avons discuté de l'encart 19-11, nous avons mentionné que nous pouvions utiliser des gardes de correspondance pour résoudre notre problème de masquage de motif. Rappelez-vous que nous avions créé une nouvelle variable à l'intérieur du motif dans l'expression `match` au lieu d'utiliser la variable en dehors du `match`. Cette nouvelle variable signifiait que nous ne pouvions pas tester contre la valeur de la variable extérieure. L'encart 19-27 montre comment nous pouvons utiliser une garde de correspondance pour corriger ce problème.

<Listing number="19-27" file-name="src/main.rs" caption="Utiliser une garde de correspondance pour tester l'égalité avec une variable extérieure">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

<!--
This code will now print `Default case, x = Some(5)`. The pattern in the second
match arm doesn't introduce a new variable `y` that would shadow the outer `y`,
meaning we can use the outer `y` in the match guard. Instead of specifying the
pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`. This creates a new variable `n` that doesn't shadow anything because
there is no `n` variable outside the `match`.
-->

Ce code affichera maintenant `Default case, x = Some(5)`. Le motif de la deuxième branche n'introduit pas de nouvelle variable `y` qui masquerait le `y` extérieur, ce qui signifie que nous pouvons utiliser le `y` extérieur dans la garde de correspondance. Au lieu de spécifier le motif comme `Some(y)`, ce qui aurait masqué le `y` extérieur, nous spécifions `Some(n)`. Cela crée une nouvelle variable `n` qui ne masque rien car il n'y a pas de variable `n` en dehors du `match`.

<!--
The match guard `if n == y` is not a pattern and therefore doesn't introduce new
variables. This `y` _is_ the outer `y` rather than a new `y` shadowing it, and
we can look for a value that has the same value as the outer `y` by comparing
`n` to `y`.
-->

La garde de correspondance `if n == y` n'est pas un motif et n'introduit donc pas de nouvelles variables. Ce `y` _est_ le `y` extérieur plutôt qu'un nouveau `y` le masquant, et nous pouvons chercher une valeur ayant la même valeur que le `y` extérieur en comparant `n` à `y`.

<!--
You can also use the _or_ operator `|` in a match guard to specify multiple
patterns; the match guard condition will apply to all the patterns. Listing
19-28 shows the precedence when combining a pattern that uses `|` with a match
guard. The important part of this example is that the `if y` match guard
applies to `4`, `5`, _and_ `6`, even though it might look like `if y` only
applies to `6`.
-->

Vous pouvez aussi utiliser l'opérateur _ou_ `|` dans une garde de correspondance pour spécifier plusieurs motifs ; la condition de la garde de correspondance s'appliquera à tous les motifs. L'encart 19-28 montre la précédence lors de la combinaison d'un motif utilisant `|` avec une garde de correspondance. La partie importante de cet exemple est que la garde de correspondance `if y` s'applique à `4`, `5` _et_ `6`, même s'il pourrait sembler que `if y` ne s'applique qu'à `6`.

<Listing number="19-28" caption="Combiner plusieurs motifs avec une garde de correspondance">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

<!--
The match condition states that the arm only matches if the value of `x` is
equal to `4`, `5`, or `6` _and_ if `y` is `true`. When this code runs, the
pattern of the first arm matches because `x` is `4`, but the match guard `if y`
is `false`, so the first arm is not chosen. The code moves on to the second
arm, which does match, and this program prints `no`. The reason is that the
`if` condition applies to the whole pattern `4 | 5 | 6`, not just to the last
value `6`. In other words, the precedence of a match guard in relation to a
pattern behaves like this:
-->

La condition de correspondance stipule que la branche ne correspond que si la valeur de `x` est égale à `4`, `5` ou `6` _et_ si `y` est `true`. Lorsque ce code s'exécute, le motif de la première branche correspond car `x` vaut `4`, mais la garde de correspondance `if y` est `false`, donc la première branche n'est pas choisie. Le code passe à la deuxième branche, qui correspond, et ce programme affiche `no`. La raison est que la condition `if` s'applique à l'ensemble du motif `4 | 5 | 6`, pas seulement à la dernière valeur `6`. En d'autres termes, la précédence d'une garde de correspondance par rapport à un motif se comporte comme ceci :

<!--
```text
(4 | 5 | 6) if y => ...
```
-->

```text
(4 | 5 | 6) if y => ...
```

<!--
rather than this:
-->

plutôt que comme ceci :

<!--
```text
4 | 5 | (6 if y) => ...
```
-->

```text
4 | 5 | (6 if y) => ...
```

<!--
After running the code, the precedence behavior is evident: If the match guard
were applied only to the final value in the list of values specified using the
`|` operator, the arm would have matched, and the program would have printed
`yes`.
-->

Après l'exécution du code, le comportement de précédence est évident : si la garde de correspondance n'était appliquée qu'à la dernière valeur de la liste de valeurs spécifiées avec l'opérateur `|`, la branche aurait correspondu, et le programme aurait affiché `yes`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="-bindings"></a>

<!--
### Using `@` Bindings
-->

### Utiliser les liaisons `@`

<!--
The _at_ operator `@` lets us create a variable that holds a value at the same
time we're testing that value for a pattern match. In Listing 19-29, we want to
test that a `Message::Hello` `id` field is within the range `3..=7`. We also
want to bind the value to the variable `id` so that we can use it in the code
associated with the arm.
-->

L'opérateur _at_ `@` nous permet de créer une variable qui contient une valeur en même temps que nous testons cette valeur pour une correspondance de motif. Dans l'encart 19-29, nous voulons tester que le champ `id` d'un `Message::Hello` se trouve dans l'intervalle `3..=7`. Nous voulons aussi lier la valeur à la variable `id` afin de pouvoir l'utiliser dans le code associé à la branche.

<Listing number="19-29" caption="Utiliser `@` pour lier une valeur dans un motif tout en la testant">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

<!--
This example will print `Found an id in range: 5`. By specifying `id @` before
the range `3..=7`, we're capturing whatever value matched the range in a
variable named `id` while also testing that the value matched the range pattern.
-->

Cet exemple affichera `Found an id in range: 5`. En spécifiant `id @` avant l'intervalle `3..=7`, nous capturons la valeur qui correspond à l'intervalle dans une variable nommée `id` tout en testant que la valeur correspond au motif d'intervalle.

<!--
In the second arm, where we only have a range specified in the pattern, the code
associated with the arm doesn't have a variable that contains the actual value
of the `id` field. The `id` field's value could have been 10, 11, or 12, but
the code that goes with that pattern doesn't know which it is. The pattern code
isn't able to use the value from the `id` field because we haven't saved the
`id` value in a variable.
-->

Dans la deuxième branche, où nous n'avons qu'un intervalle spécifié dans le motif, le code associé à la branche n'a pas de variable contenant la valeur réelle du champ `id`. La valeur du champ `id` aurait pu être 10, 11 ou 12, mais le code qui accompagne ce motif ne sait pas laquelle. Le code du motif ne peut pas utiliser la valeur du champ `id` car nous n'avons pas sauvegardé la valeur de `id` dans une variable.

<!--
In the last arm, where we've specified a variable without a range, we do have
the value available to use in the arm's code in a variable named `id`. The
reason is that we've used the struct field shorthand syntax. But we haven't
applied any test to the value in the `id` field in this arm, as we did with the
first two arms: Any value would match this pattern.
-->

Dans la dernière branche, où nous avons spécifié une variable sans intervalle, nous avons bien la valeur disponible pour utilisation dans le code de la branche dans une variable nommée `id`. La raison est que nous avons utilisé la syntaxe raccourcie des champs de structure. Mais nous n'avons appliqué aucun test à la valeur du champ `id` dans cette branche, comme nous l'avons fait avec les deux premières branches : n'importe quelle valeur correspondrait à ce motif.

<!--
Using `@` lets us test a value and save it in a variable within one pattern.
-->

Utiliser `@` nous permet de tester une valeur et de la sauvegarder dans une variable au sein d'un seul motif.

<!--
## Summary
-->

## Résumé

<!--
Rust's patterns are very useful in distinguishing between different kinds of
data. When used in `match` expressions, Rust ensures that your patterns cover
every possible value, or your program won't compile. Patterns in `let`
statements and function parameters make those constructs more useful, enabling
the destructuring of values into smaller parts and assigning those parts to
variables. We can create simple or complex patterns to suit our needs.
-->

Les motifs de Rust sont très utiles pour distinguer les différents types de données. Lorsqu'ils sont utilisés dans des expressions `match`, Rust s'assure que vos motifs couvrent toutes les valeurs possibles, sinon votre programme ne compilera pas. Les motifs dans les instructions `let` et les paramètres de fonction rendent ces constructions plus utiles, permettant la déstructuration des valeurs en parties plus petites et l'assignation de ces parties à des variables. Nous pouvons créer des motifs simples ou complexes pour répondre à nos besoins.

<!--
Next, for the penultimate chapter of the book, we'll look at some advanced
aspects of a variety of Rust's features.
-->

Ensuite, pour l'avant-dernier chapitre du livre, nous examinerons quelques aspects avancés de diverses fonctionnalités de Rust.
