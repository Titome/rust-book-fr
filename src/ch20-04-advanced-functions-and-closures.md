<!--
## Advanced Functions and Closures
-->

## Les fonctions et fermetures avancées

<!--
This section explores some advanced features related to functions and closures,
including function pointers and returning closures.
-->

Cette section explore quelques fonctionnalités avancées liées aux fonctions et aux fermetures, notamment les pointeurs de fonction et le retour de fermetures.

<!--
### Function Pointers
-->

### Les pointeurs de fonction

<!--
We've talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you've already defined rather than defining a new closure. Functions
coerce to the type `fn` (with a lowercase _f_), not to be confused with the
`Fn` closure trait. The `fn` type is called a _function pointer_. Passing
functions with function pointers will allow you to use functions as arguments
to other functions.
-->

Nous avons parlé de comment passer des fermetures aux fonctions ; vous pouvez aussi passer des fonctions normales aux fonctions ! Cette technique est utile lorsque vous voulez passer une fonction que vous avez déjà définie plutôt que de définir une nouvelle fermeture. Les fonctions se convertissent implicitement vers le type `fn` (avec un _f_ minuscule), à ne pas confondre avec le trait de fermeture `Fn`. Le type `fn` est appelé un _pointeur de fonction_. Passer des fonctions avec des pointeurs de fonction vous permettra d'utiliser des fonctions comme arguments d'autres fonctions.

<!--
The syntax for specifying that a parameter is a function pointer is similar to
that of closures, as shown in Listing 20-28, where we've defined a function
`add_one` that adds 1 to its parameter. The function `do_twice` takes two
parameters: a function pointer to any function that takes an `i32` parameter
and returns an `i32`, and one `i32` value. The `do_twice` function calls the
function `f` twice, passing it the `arg` value, then adds the two function call
results together. The `main` function calls `do_twice` with the arguments
`add_one` and `5`.
-->

La syntaxe pour spécifier qu'un paramètre est un pointeur de fonction est similaire à celle des fermetures, comme montré dans l'encart 20-28, où nous avons défini une fonction `add_one` qui ajoute 1 à son paramètre. La fonction `do_twice` prend deux paramètres : un pointeur de fonction vers toute fonction qui prend un paramètre `i32` et retourne un `i32`, et une valeur `i32`. La fonction `do_twice` appelle la fonction `f` deux fois, en lui passant la valeur `arg`, puis additionne les deux résultats des appels de fonction. La fonction `main` appelle `do_twice` avec les arguments `add_one` et `5`.

<Listing number="20-28" file-name="src/main.rs" caption="Utiliser le type `fn` pour accepter un pointeur de fonction comme argument">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

<!--
This code prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.
-->

Ce code affiche `The answer is: 12`. Nous spécifions que le paramètre `f` dans `do_twice` est un `fn` qui prend un paramètre de type `i32` et retourne un `i32`. Nous pouvons ensuite appeler `f` dans le corps de `do_twice`. Dans `main`, nous pouvons passer le nom de fonction `add_one` comme premier argument à `do_twice`.

<!--
Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.
-->

Contrairement aux fermetures, `fn` est un type plutôt qu'un trait, donc nous spécifions `fn` comme type de paramètre directement plutôt que de déclarer un paramètre de type générique avec l'un des traits `Fn` comme contrainte de trait.

<!--
Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), meaning you can always pass a function pointer as an argument for a
function that expects a closure. It's best to write functions using a generic
type and one of the closure traits so that your functions can accept either
functions or closures.
-->

Les pointeurs de fonction implémentent les trois traits de fermeture (`Fn`, `FnMut` et `FnOnce`), ce qui signifie que vous pouvez toujours passer un pointeur de fonction comme argument d'une fonction qui attend une fermeture. Il est préférable d'écrire des fonctions utilisant un type générique et l'un des traits de fermeture afin que vos fonctions puissent accepter aussi bien des fonctions que des fermetures.

<!--
That said, one example of where you would want to only accept `fn` and not
closures is when interfacing with external code that doesn't have closures: C
functions can accept functions as arguments, but C doesn't have closures.
-->

Cela dit, un exemple où vous voudriez n'accepter que `fn` et pas les fermetures est lors de l'interfaçage avec du code externe qui n'a pas de fermetures : les fonctions C peuvent accepter des fonctions comme arguments, mais C n'a pas de fermetures.

<!--
As an example of where you could use either a closure defined inline or a named
function, let's look at a use of the `map` method provided by the `Iterator`
trait in the standard library. To use the `map` method to turn a vector of
numbers into a vector of strings, we could use a closure, as in Listing 20-29.
-->

Comme exemple d'un cas où vous pourriez utiliser soit une fermeture définie en ligne soit une fonction nommée, regardons une utilisation de la méthode `map` fournie par le trait `Iterator` de la bibliothèque standard. Pour utiliser la méthode `map` afin de transformer un vecteur de nombres en un vecteur de chaînes de caractères, nous pourrions utiliser une fermeture, comme dans l'encart 20-29.

<Listing number="20-29" caption="Utiliser une fermeture avec la méthode `map` pour convertir des nombres en chaînes de caractères">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

<!--
Or we could name a function as the argument to `map` instead of the closure.
Listing 20-30 shows what this would look like.
-->

Ou nous pourrions nommer une fonction comme argument de `map` au lieu de la fermeture. L'encart 20-30 montre à quoi cela ressemblerait.

<Listing number="20-30" caption="Utiliser la fonction `String::to_string` avec la méthode `map` pour convertir des nombres en chaînes de caractères">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

<!--
Note that we must use the fully qualified syntax that we talked about in the
["Advanced Traits"][advanced-traits] ignore
--> section because there are
multiple functions available named `to_string`.
-->

Notez que nous devons utiliser la syntaxe pleinement qualifiée dont nous avons parlé dans la section ["Les traits avancés"][advanced-traits]<!--
ignore
--> car il y a plusieurs fonctions disponibles nommées `to_string`.

<!--
Here, we're using the `to_string` function defined in the `ToString` trait,
which the standard library has implemented for any type that implements
`Display`.
-->

Ici, nous utilisons la fonction `to_string` définie dans le trait `ToString`, que la bibliothèque standard a implémenté pour tout type qui implémente `Display`.

<!--
Recall from the ["Enum Values"][enum-values] ignore
--> section in Chapter
6 that the name of each enum variant that we define also becomes an initializer
function. We can use these initializer functions as function pointers that
implement the closure traits, which means we can specify the initializer
functions as arguments for methods that take closures, as seen in Listing 20-31.
-->

Rappelez-vous de la section ["Les valeurs d'enum"][enum-values]<!--
ignore
--> du chapitre 6 que le nom de chaque variante d'enum que nous définissons devient aussi une fonction d'initialisation. Nous pouvons utiliser ces fonctions d'initialisation comme des pointeurs de fonction qui implémentent les traits de fermeture, ce qui signifie que nous pouvons spécifier les fonctions d'initialisation comme arguments des méthodes qui prennent des fermetures, comme vu dans l'encart 20-31.

<Listing number="20-31" caption="Utiliser un initialiseur d'enum avec la méthode `map` pour créer une instance de `Status` à partir de nombres">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

<!--
Here, we create `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.
-->

Ici, nous créons des instances de `Status::Value` en utilisant chaque valeur `u32` de l'intervalle sur lequel `map` est appelé en utilisant la fonction d'initialisation de `Status::Value`. Certaines personnes préfèrent ce style et d'autres préfèrent utiliser des fermetures. Ils compilent vers le même code, donc utilisez le style qui vous semble le plus clair.

<!--
### Returning Closures
-->

### Retourner des fermetures

<!--
Closures are represented by traits, which means you can't return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can't usually do that with closures because they don't
have a concrete type that is returnable; you're not allowed to use the function
pointer `fn` as a return type if the closure captures any values from its
scope, for example.
-->

Les fermetures sont représentées par des traits, ce qui signifie que vous ne pouvez pas retourner des fermetures directement. Dans la plupart des cas où vous voudriez retourner un trait, vous pouvez utiliser à la place le type concret qui implémente le trait comme valeur de retour de la fonction. Cependant, vous ne pouvez généralement pas faire cela avec les fermetures car elles n'ont pas de type concret qui peut être retourné ; vous n'êtes pas autorisé à utiliser le pointeur de fonction `fn` comme type de retour si la fermeture capture des valeurs de sa portée, par exemple.

<!--
Instead, you will normally use the `impl Trait` syntax we learned about in
Chapter 10. You can return any function type, using `Fn`, `FnOnce`, and `FnMut`.
For example, the code in Listing 20-32 will compile just fine.
-->

À la place, vous utiliserez normalement la syntaxe `impl Trait` que nous avons apprise au chapitre 10. Vous pouvez retourner n'importe quel type de fonction, en utilisant `Fn`, `FnOnce` et `FnMut`. Par exemple, le code de l'encart 20-32 compilera sans problème.

<Listing number="20-32" caption="Retourner une fermeture depuis une fonction en utilisant la syntaxe `impl Trait`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

<!--
However, as we noted in the ["Inferring and Annotating Closure
Types"][closure-types] ignore
--> section in Chapter 13, each closure is
also its own distinct type. If you need to work with multiple functions that
have the same signature but different implementations, you will need to use a
trait object for them. Consider what happens if you write code like that shown
in Listing 20-33.
-->

Cependant, comme nous l'avons noté dans la section ["Inférence et annotation des types de fermetures"][closure-types]<!--
ignore
--> du chapitre 13, chaque fermeture est aussi son propre type distinct. Si vous devez travailler avec plusieurs fonctions qui ont la même signature mais des implémentations différentes, vous devrez utiliser un objet trait pour elles. Considérez ce qui se passe si vous écrivez du code comme celui de l'encart 20-33.

<Listing file-name="src/main.rs" number="20-33" caption="Créer un `Vec<T>` de fermetures définies par des fonctions qui retournent des types `impl Fn`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

<!--
Here we have two functions, `returns_closure` and `returns_initialized_closure`,
which both return `impl Fn(i32) -> i32`. Notice that the closures that they
return are different, even though they implement the same type. If we try to
compile this, Rust lets us know that it won't work:
-->

Ici nous avons deux fonctions, `returns_closure` et `returns_initialized_closure`, qui retournent toutes les deux `impl Fn(i32) -> i32`. Remarquez que les fermetures qu'elles retournent sont différentes, même si elles implémentent le même type. Si nous essayons de compiler ceci, Rust nous fait savoir que cela ne fonctionnera pas :


```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

<!--
The error message tells us that whenever we return an `impl Trait`, Rust
creates a unique _opaque type_, a type where we cannot see into the details of
what Rust constructs for us, nor can we guess the type Rust will generate to
write ourselves. So, even though these functions return closures that implement
the same trait, `Fn(i32) -> i32`, the opaque types Rust generates for each are
distinct. (This is similar to how Rust produces different concrete types for
distinct async blocks even when they have the same output type, as we saw in
["The `Pin` Type and the `Unpin` Trait"][future-types] ignore
--> in
Chapter 17.) We have seen a solution to this problem a few times now: We can
use a trait object, as in Listing 20-34.
-->

Le message d'erreur nous dit que chaque fois que nous retournons un `impl Trait`, Rust crée un _type opaque_ unique, un type dont nous ne pouvons pas voir les détails de ce que Rust construit pour nous, et dont nous ne pouvons pas deviner le type que Rust générera pour l'écrire nous-mêmes. Ainsi, même si ces fonctions retournent des fermetures qui implémentent le même trait, `Fn(i32) -> i32`, les types opaques que Rust génère pour chacune sont distincts. (C'est similaire à la façon dont Rust produit des types concrets différents pour des blocs async distincts même quand ils ont le même type de sortie, comme nous l'avons vu dans ["Le type `Pin` et le trait `Unpin`"][future-types]<!--
ignore
--> au chapitre 17.) Nous avons vu une solution à ce problème plusieurs fois maintenant : nous pouvons utiliser un objet trait, comme dans l'encart 20-34.

<Listing number="20-34" caption="Créer un `Vec<T>` de fermetures définies par des fonctions qui retournent `Box<dyn Fn>` pour qu'elles aient le même type">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

<!--
This code will compile just fine. For more about trait objects, refer to the
section ["Using Trait Objects To Abstract over Shared
Behavior"][trait-objects] ignore
--> in Chapter 18.
-->

Ce code compilera sans problème. Pour en savoir plus sur les objets trait, référez-vous à la section ["Utiliser les objets trait pour abstraire un comportement partagé"][trait-objects]<!--
ignore
--> du chapitre 18.

<!--
Next, let's look at macros!
-->

Ensuite, regardons les macros !

[advanced-traits]: ch20-02-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[future-types]: ch17-03-more-futures.html
[trait-objects]: ch18-02-trait-objects.html
