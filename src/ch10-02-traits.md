<!--
Old headings. Do not remove or links may break.
-->

<a id="traits-defining-shared-behavior"></a>

<!--
## Defining Shared Behavior with Traits
-->

## Définir un comportement partagé avec les traits

<!--
A _trait_ defines the functionality a particular type has and can share with
other types. We can use traits to define shared behavior in an abstract way. We
can use _trait bounds_ to specify that a generic type can be any type that has
certain behavior.
-->

Un _trait_ définit la fonctionnalité qu'un type particulier possède et peut
partager avec d'autres types. Nous pouvons utiliser les traits pour définir un
comportement partagé de manière abstraite. Nous pouvons utiliser les
_trait bounds_ (limites de trait) pour spécifier qu'un type générique peut être
n'importe quel type possédant un certain comportement.

<!--
> Note: Traits are similar to a feature often called _interfaces_ in other
> languages, although with some differences.
-->

> Remarque : les traits sont similaires à une fonctionnalité souvent appelée
> _interfaces_ dans d'autres langages, bien qu'avec quelques différences.

<!--
### Defining a Trait
-->

### Définir un trait

<!--
A type's behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.
-->

Le comportement d'un type consiste en les méthodes que nous pouvons appeler
sur ce type. Différents types partagent le même comportement si nous pouvons
appeler les mêmes méthodes sur chacun de ces types. Les définitions de traits
sont un moyen de regrouper des signatures de méthodes pour définir un ensemble
de comportements nécessaires pour accomplir un certain objectif.

<!--
For example, let's say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `SocialPost` that can have, at most, 280 characters
along with metadata that indicates whether it was a new post, a repost, or a
reply to another post.
-->

Par exemple, supposons que nous ayons plusieurs structs qui contiennent
différents types et quantités de texte : une struct `NewsArticle` qui contient
un article de presse classé dans un lieu particulier et un `SocialPost` qui
peut avoir au maximum 280 caractères avec des métadonnées indiquant s'il
s'agissait d'une nouvelle publication, d'un repartage ou d'une réponse à une
autre publication.

<!--
We want to make a media aggregator library crate named `aggregator` that can
display summaries of data that might be stored in a `NewsArticle` or
`SocialPost` instance. To do this, we need a summary from each type, and we'll
request that summary by calling a `summarize` method on an instance. Listing
10-12 shows the definition of a public `Summary` trait that expresses this
behavior.
-->

Nous voulons créer une crate de bibliothèque d'agrégation de médias nommée
`aggregator` qui peut afficher des résumés de données qui pourraient être
stockées dans une instance de `NewsArticle` ou de `SocialPost`. Pour cela,
nous avons besoin d'un résumé de chaque type, et nous demanderons ce résumé
en appelant une méthode `summarize` sur une instance. L'encart 10-12 montre
la définition d'un trait public `Summary` qui exprime ce comportement.

<Listing number="10-12" file-name="src/lib.rs" caption="Un trait `Summary` qui consiste en le comportement fourni par une méthode `summarize`">


```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

<!--
Here, we declare a trait using the `trait` keyword and then the trait's name,
which is `Summary` in this case. We also declare the trait as `pub` so that
crates depending on this crate can make use of this trait too, as we'll see in
a few examples. Inside the curly brackets, we declare the method signatures
that describe the behaviors of the types that implement this trait, which in
this case is `fn summarize(&self) -> String`.
-->

Ici, nous déclarons un trait en utilisant le mot-clé `trait` puis le nom du
trait, qui est `Summary` dans ce cas. Nous déclarons aussi le trait comme `pub`
pour que les crates qui dépendent de cette crate puissent aussi utiliser ce
trait, comme nous le verrons dans quelques exemples. À l'intérieur des
accolades, nous déclarons les signatures de méthodes qui décrivent les
comportements des types qui implémentent ce trait, ce qui dans ce cas est
`fn summarize(&self) -> String`.

<!--
After the method signature, instead of providing an implementation within curly
brackets, we use a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method. The compiler will enforce
that any type that has the `Summary` trait will have the method `summarize`
defined with this signature exactly.
-->

Après la signature de la méthode, au lieu de fournir une implémentation entre
accolades, nous utilisons un point-virgule. Chaque type implémentant ce trait
doit fournir son propre comportement personnalisé pour le corps de la méthode.
Le compilateur s'assurera que tout type qui possède le trait `Summary` aura la
méthode `summarize` définie avec exactement cette signature.

<!--
A trait can have multiple methods in its body: The method signatures are listed
one per line, and each line ends in a semicolon.
-->

Un trait peut avoir plusieurs méthodes dans son corps : les signatures de
méthodes sont listées une par ligne, et chaque ligne se termine par un
point-virgule.

<!--
### Implementing a Trait on a Type
-->

### Implémenter un trait sur un type

<!--
Now that we've defined the desired signatures of the `Summary` trait's methods,
we can implement it on the types in our media aggregator. Listing 10-13 shows
an implementation of the `Summary` trait on the `NewsArticle` struct that uses
the headline, the author, and the location to create the return value of
`summarize`. For the `SocialPost` struct, we define `summarize` as the username
followed by the entire text of the post, assuming that the post content is
already limited to 280 characters.
-->

Maintenant que nous avons défini les signatures souhaitées des méthodes du
trait `Summary`, nous pouvons l'implémenter sur les types de notre agrégateur
de médias. L'encart 10-13 montre une implémentation du trait `Summary` sur la
struct `NewsArticle` qui utilise le titre, l'auteur et le lieu pour créer la
valeur de retour de `summarize`. Pour la struct `SocialPost`, nous définissons
`summarize` comme le nom d'utilisateur suivi du texte entier de la
publication, en supposant que le contenu de la publication est déjà limité à
280 caractères.

<Listing number="10-13" file-name="src/lib.rs" caption="Implémentation du trait `Summary` sur les types `NewsArticle` et `SocialPost`">


```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

<!--
Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name we want to implement,
then use the `for` keyword, and then specify the name of the type we want to
implement the trait for. Within the `impl` block, we put the method signatures
that the trait definition has defined. Instead of adding a semicolon after each
signature, we use curly brackets and fill in the method body with the specific
behavior that we want the methods of the trait to have for the particular type.
-->

Implémenter un trait sur un type est similaire à implémenter des méthodes
classiques. La différence est qu'après `impl`, nous mettons le nom du trait
que nous voulons implémenter, puis nous utilisons le mot-clé `for`, puis nous
spécifions le nom du type sur lequel nous voulons implémenter le trait. À
l'intérieur du bloc `impl`, nous mettons les signatures de méthodes que la
définition du trait a définies. Au lieu d'ajouter un point-virgule après
chaque signature, nous utilisons des accolades et remplissons le corps de la
méthode avec le comportement spécifique que nous voulons que les méthodes du
trait aient pour le type particulier.

<!--
Now that the library has implemented the `Summary` trait on `NewsArticle` and
`SocialPost`, users of the crate can call the trait methods on instances of
`NewsArticle` and `SocialPost` in the same way we call regular methods. The only
difference is that the user must bring the trait into scope as well as the
types. Here's an example of how a binary crate could use our `aggregator`
library crate:
-->

Maintenant que la bibliothèque a implémenté le trait `Summary` sur
`NewsArticle` et `SocialPost`, les utilisateurs de la crate peuvent appeler les
méthodes du trait sur des instances de `NewsArticle` et `SocialPost` de la
même manière que nous appelons des méthodes classiques. La seule différence est
que l'utilisateur doit importer le trait dans la portée ainsi que les types.
Voici un exemple de la façon dont une crate binaire pourrait utiliser notre
crate de bibliothèque `aggregator` :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

<!--
This code prints `1 new post: horse_ebooks: of course, as you probably already
know, people`.
-->

Ce code affiche `1 new post: horse_ebooks: of course, as you probably already
know, people`.

<!--
Other crates that depend on the `aggregator` crate can also bring the `Summary`
trait into scope to implement `Summary` on their own types. One restriction to
note is that we can implement a trait on a type only if either the trait or the
type, or both, are local to our crate. For example, we can implement standard
library traits like `Display` on a custom type like `SocialPost` as part of our
`aggregator` crate functionality because the type `SocialPost` is local to our
`aggregator` crate. We can also implement `Summary` on `Vec<T>` in our
`aggregator` crate because the trait `Summary` is local to our `aggregator`
crate.
-->

D'autres crates qui dépendent de la crate `aggregator` peuvent aussi importer
le trait `Summary` dans la portée pour implémenter `Summary` sur leurs propres
types. Une restriction à noter est que nous ne pouvons implémenter un trait sur
un type que si le trait ou le type, ou les deux, sont locaux à notre crate. Par
exemple, nous pouvons implémenter des traits de la bibliothèque standard comme
`Display` sur un type personnalisé comme `SocialPost` dans le cadre de la
fonctionnalité de notre crate `aggregator` car le type `SocialPost` est local
à notre crate `aggregator`. Nous pouvons aussi implémenter `Summary` sur
`Vec<T>` dans notre crate `aggregator` car le trait `Summary` est local à
notre crate `aggregator`.

<!--
But we can't implement external traits on external types. For example, we can't
implement the `Display` trait on `Vec<T>` within our `aggregator` crate,
because `Display` and `Vec<T>` are both defined in the standard library and
aren't local to our `aggregator` crate. This restriction is part of a property
called _coherence_, and more specifically the _orphan rule_, so named because
the parent type is not present. This rule ensures that other people's code
can't break your code and vice versa. Without the rule, two crates could
implement the same trait for the same type, and Rust wouldn't know which
implementation to use.
-->

Mais nous ne pouvons pas implémenter des traits externes sur des types
externes. Par exemple, nous ne pouvons pas implémenter le trait `Display` sur
`Vec<T>` dans notre crate `aggregator`, car `Display` et `Vec<T>` sont tous
deux définis dans la bibliothèque standard et ne sont pas locaux à notre crate
`aggregator`. Cette restriction fait partie d'une propriété appelée
_cohérence_, et plus spécifiquement la _règle de l'orphelin_, ainsi nommée
car le type parent n'est pas présent. Cette règle garantit que le code des
autres ne peut pas casser votre code et vice versa. Sans cette règle, deux
crates pourraient implémenter le même trait pour le même type, et Rust ne
saurait pas quelle implémentation utiliser.

<!--
Old headings. Do not remove or links may break.
-->

<a id="default-implementations"></a>

<!--
### Using Default Implementations
-->

### Utiliser les implémentations par défaut

<!--
Sometimes it's useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can keep or override
each method's default behavior.
-->

Parfois, il est utile d'avoir un comportement par défaut pour certaines ou
toutes les méthodes d'un trait au lieu d'exiger des implémentations pour
toutes les méthodes sur chaque type. Ensuite, lorsque nous implémentons le
trait sur un type particulier, nous pouvons conserver ou remplacer le
comportement par défaut de chaque méthode.

<!--
In Listing 10-14, we specify a default string for the `summarize` method of the
`Summary` trait instead of only defining the method signature, as we did in
Listing 10-12.
-->

Dans l'encart 10-14, nous spécifions une chaîne de caractères par défaut pour
la méthode `summarize` du trait `Summary` au lieu de définir uniquement la
signature de la méthode, comme nous l'avons fait dans l'encart 10-12.

<Listing number="10-14" file-name="src/lib.rs" caption="Définition d'un trait `Summary` avec une implémentation par défaut de la méthode `summarize`">


```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

<!--
To use a default implementation to summarize instances of `NewsArticle`, we
specify an empty `impl` block with `impl Summary for NewsArticle {}`.
-->

Pour utiliser une implémentation par défaut afin de résumer les instances de
`NewsArticle`, nous spécifions un bloc `impl` vide avec
`impl Summary for NewsArticle {}`.

<!--
Even though we're no longer defining the `summarize` method on `NewsArticle`
directly, we've provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:
-->

Même si nous ne définissons plus directement la méthode `summarize` sur
`NewsArticle`, nous avons fourni une implémentation par défaut et spécifié que
`NewsArticle` implémente le trait `Summary`. En conséquence, nous pouvons
toujours appeler la méthode `summarize` sur une instance de `NewsArticle`,
comme ceci :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

<!--
This code prints `New article available! (Read more...)`.
-->

Ce code affiche `New article available! (Read more...)`.

<!--
Creating a default implementation doesn't require us to change anything about
the implementation of `Summary` on `SocialPost` in Listing 10-13. The reason is
that the syntax for overriding a default implementation is the same as the
syntax for implementing a trait method that doesn't have a default
implementation.
-->

Créer une implémentation par défaut ne nous oblige pas à modifier quoi que ce
soit dans l'implémentation de `Summary` sur `SocialPost` dans l'encart 10-13.
La raison est que la syntaxe pour remplacer une implémentation par défaut est
la même que celle pour implémenter une méthode de trait qui n'a pas
d'implémentation par défaut.

<!--
Default implementations can call other methods in the same trait, even if those
other methods don't have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, and then define a
`summarize` method that has a default implementation that calls the
`summarize_author` method:
-->

Les implémentations par défaut peuvent appeler d'autres méthodes du même trait,
même si ces autres méthodes n'ont pas d'implémentation par défaut. De cette
façon, un trait peut fournir beaucoup de fonctionnalités utiles et n'exiger des
implémenteurs qu'ils ne spécifient qu'une petite partie. Par exemple, nous
pourrions définir le trait `Summary` avec une méthode `summarize_author` dont
l'implémentation est requise, puis définir une méthode `summarize` qui a une
implémentation par défaut qui appelle la méthode `summarize_author` :


```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

<!--
To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:
-->

Pour utiliser cette version de `Summary`, nous n'avons besoin de définir que
`summarize_author` lorsque nous implémentons le trait sur un type :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

<!--
After we define `summarize_author`, we can call `summarize` on instances of the
`SocialPost` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we've provided. Because we've implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code. Here's what
that looks like:
-->

Après avoir défini `summarize_author`, nous pouvons appeler `summarize` sur
des instances de la struct `SocialPost`, et l'implémentation par défaut de
`summarize` appellera la définition de `summarize_author` que nous avons
fournie. Comme nous avons implémenté `summarize_author`, le trait `Summary`
nous a donné le comportement de la méthode `summarize` sans que nous ayons
besoin d'écrire du code supplémentaire. Voici à quoi cela ressemble :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

<!--
This code prints `1 new post: (Read more from @horse_ebooks...)`.
-->

Ce code affiche `1 new post: (Read more from @horse_ebooks...)`.

<!--
Note that it isn't possible to call the default implementation from an
overriding implementation of that same method.
-->

Notez qu'il n'est pas possible d'appeler l'implémentation par défaut depuis
une implémentation qui remplace cette même méthode.

<!--
Old headings. Do not remove or links may break.
-->

<a id="traits-as-parameters"></a>

<!--
### Using Traits as Parameters
-->

### Utiliser les traits comme paramètres

<!--
Now that you know how to define and implement traits, we can explore how to use
traits to define functions that accept many different types. We'll use the
`Summary` trait we implemented on the `NewsArticle` and `SocialPost` types in
Listing 10-13 to define a `notify` function that calls the `summarize` method
on its `item` parameter, which is of some type that implements the `Summary`
trait. To do this, we use the `impl Trait` syntax, like this:
-->

Maintenant que vous savez comment définir et implémenter des traits, nous
pouvons explorer comment utiliser les traits pour définir des fonctions qui
acceptent de nombreux types différents. Nous utiliserons le trait `Summary` que
nous avons implémenté sur les types `NewsArticle` et `SocialPost` dans
l'encart 10-13 pour définir une fonction `notify` qui appelle la méthode
`summarize` sur son paramètre `item`, qui est d'un certain type implémentant
le trait `Summary`. Pour ce faire, nous utilisons la syntaxe `impl Trait`,
comme ceci :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

<!--
Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait name. This parameter accepts any type that implements the
specified trait. In the body of `notify`, we can call any methods on `item`
that come from the `Summary` trait, such as `summarize`. We can call `notify`
and pass in any instance of `NewsArticle` or `SocialPost`. Code that calls the
function with any other type, such as a `String` or an `i32`, won't compile,
because those types don't implement `Summary`.
-->

Au lieu d'un type concret pour le paramètre `item`, nous spécifions le mot-clé
`impl` et le nom du trait. Ce paramètre accepte n'importe quel type qui
implémente le trait spécifié. Dans le corps de `notify`, nous pouvons appeler
n'importe quelle méthode sur `item` provenant du trait `Summary`, comme
`summarize`. Nous pouvons appeler `notify` et passer n'importe quelle instance
de `NewsArticle` ou `SocialPost`. Le code qui appelle la fonction avec
n'importe quel autre type, comme un `String` ou un `i32`, ne compilera pas,
car ces types n'implémentent pas `Summary`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

<!--
#### Trait Bound Syntax
-->

#### La syntaxe des trait bounds

<!--
The `impl Trait` syntax works for straightforward cases but is actually syntax
sugar for a longer form known as a _trait bound_; it looks like this:
-->

La syntaxe `impl Trait` fonctionne pour les cas simples mais est en fait du
sucre syntaxique pour une forme plus longue connue sous le nom de _trait
bound_ ; elle ressemble à ceci :

<!--
```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
-->

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

<!--
This longer form is equivalent to the example in the previous section but is
more verbose. We place trait bounds with the declaration of the generic type
parameter after a colon and inside angle brackets.
-->

Cette forme plus longue est équivalente à l'exemple de la section précédente
mais est plus verbeuse. Nous plaçons les trait bounds avec la déclaration du
paramètre de type générique après un deux-points et entre chevrons.

<!--
The `impl Trait` syntax is convenient and makes for more concise code in simple
cases, while the fuller trait bound syntax can express more complexity in other
cases. For example, we can have two parameters that implement `Summary`. Doing
so with the `impl Trait` syntax looks like this:
-->

La syntaxe `impl Trait` est pratique et permet un code plus concis dans les cas
simples, tandis que la syntaxe plus complète des trait bounds peut exprimer
plus de complexité dans d'autres cas. Par exemple, nous pouvons avoir deux
paramètres qui implémentent `Summary`. Le faire avec la syntaxe `impl Trait`
ressemble à ceci :

<!--
```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
-->

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

<!--
Using `impl Trait` is appropriate if we want this function to allow `item1` and
`item2` to have different types (as long as both types implement `Summary`). If
we want to force both parameters to have the same type, however, we must use a
trait bound, like this:
-->

Utiliser `impl Trait` est approprié si nous voulons que cette fonction permette
à `item1` et `item2` d'avoir des types différents (tant que les deux types
implémentent `Summary`). Si nous voulons forcer les deux paramètres à avoir le
même type, cependant, nous devons utiliser un trait bound, comme ceci :

<!--
```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```
-->

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

<!--
The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.
-->

Le type générique `T` spécifié comme type des paramètres `item1` et `item2`
contraint la fonction de telle sorte que le type concret de la valeur passée
en argument pour `item1` et `item2` doit être le même.

<!--
Old headings. Do not remove or links may break.
-->

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>

<!--
#### Multiple Trait Bounds with the `+` Syntax
-->

#### Trait bounds multiples avec la syntaxe `+`

<!--
We can also specify more than one trait bound. Say we wanted `notify` to use
display formatting as well as `summarize` on `item`: We specify in the `notify`
definition that `item` must implement both `Display` and `Summary`. We can do
so using the `+` syntax:
-->

Nous pouvons aussi spécifier plus d'un trait bound. Supposons que nous voulions
que `notify` utilise le formatage d'affichage ainsi que `summarize` sur
`item` : nous spécifions dans la définition de `notify` que `item` doit
implémenter à la fois `Display` et `Summary`. Nous pouvons le faire en
utilisant la syntaxe `+` :

<!--
```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```
-->

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

<!--
The `+` syntax is also valid with trait bounds on generic types:
-->

La syntaxe `+` est aussi valide avec les trait bounds sur les types
génériques :

<!--
```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```
-->

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

<!--
With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.
-->

Avec les deux trait bounds spécifiés, le corps de `notify` peut appeler
`summarize` et utiliser `{}` pour formater `item`.

<!--
#### Clearer Trait Bounds with `where` Clauses
-->

#### Des trait bounds plus clairs avec les clauses `where`

<!--
Using too many trait bounds has its downsides. Each generic has its own trait
bounds, so functions with multiple generic type parameters can contain lots of
trait bound information between the function's name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So, instead of writing this:
-->

Utiliser trop de trait bounds a ses inconvénients. Chaque générique a ses
propres trait bounds, donc les fonctions avec plusieurs paramètres de type
générique peuvent contenir beaucoup d'informations de trait bounds entre le
nom de la fonction et sa liste de paramètres, rendant la signature de la
fonction difficile à lire. Pour cette raison, Rust dispose d'une syntaxe
alternative pour spécifier les trait bounds dans une clause `where` après la
signature de la fonction. Ainsi, au lieu d'écrire ceci :

<!--
```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
-->

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

<!--
we can use a `where` clause, like this:
-->

nous pouvons utiliser une clause `where`, comme ceci :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

<!--
This function's signature is less cluttered: The function name, parameter list,
and return type are close together, similar to a function without lots of trait
bounds.
-->

La signature de cette fonction est moins encombrée : le nom de la fonction, la
liste des paramètres et le type de retour sont proches les uns des autres,
similaire à une fonction sans beaucoup de trait bounds.

<!--
### Returning Types That Implement Traits
-->

### Retourner des types qui implémentent des traits

<!--
We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:
-->

Nous pouvons aussi utiliser la syntaxe `impl Trait` en position de retour pour
retourner une valeur d'un certain type qui implémente un trait, comme montré
ici :


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

<!--
By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `SocialPost`, but the code calling this function doesn't need to know
that.
-->

En utilisant `impl Summary` comme type de retour, nous spécifions que la
fonction `returns_summarizable` retourne un certain type qui implémente le
trait `Summary` sans nommer le type concret. Dans ce cas,
`returns_summarizable` retourne un `SocialPost`, mais le code qui appelle
cette fonction n'a pas besoin de le savoir.

<!--
The ability to specify a return type only by the trait it implements is
especially useful in the context of closures and iterators, which we cover in
Chapter 13. Closures and iterators create types that only the compiler knows or
types that are very long to specify. The `impl Trait` syntax lets you concisely
specify that a function returns some type that implements the `Iterator` trait
without needing to write out a very long type.
-->

La possibilité de spécifier un type de retour uniquement par le trait qu'il
implémente est particulièrement utile dans le contexte des fermetures
(closures) et des itérateurs, que nous couvrons au chapitre 13. Les fermetures
et les itérateurs créent des types que seul le compilateur connaît ou des
types très longs à spécifier. La syntaxe `impl Trait` vous permet de spécifier
de manière concise qu'une fonction retourne un certain type qui implémente le
trait `Iterator` sans avoir besoin d'écrire un type très long.

<!--
However, you can only use `impl Trait` if you're returning a single type. For
example, this code that returns either a `NewsArticle` or a `SocialPost` with
the return type specified as `impl Summary` wouldn't work:
-->

Cependant, vous ne pouvez utiliser `impl Trait` que si vous retournez un seul
type. Par exemple, ce code qui retourne soit un `NewsArticle` soit un
`SocialPost` avec le type de retour spécifié comme `impl Summary` ne
fonctionnerait pas :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

<!--
Returning either a `NewsArticle` or a `SocialPost` isn't allowed due to
restrictions around how the `impl Trait` syntax is implemented in the compiler.
We'll cover how to write a function with this behavior in the ["Using Trait
Objects to Abstract over Shared Behavior"][trait-objects] ignore
-->
section of Chapter 18.
-->

Retourner soit un `NewsArticle` soit un `SocialPost` n'est pas autorisé en
raison de restrictions liées à la façon dont la syntaxe `impl Trait` est
implémentée dans le compilateur. Nous verrons comment écrire une fonction avec
ce comportement dans la section [« Utiliser les objets trait pour abstraire un
comportement partagé »][trait-objects]<!--
ignore
--> du chapitre 18.

<!--
### Using Trait Bounds to Conditionally Implement Methods
-->

### Utiliser les trait bounds pour implémenter des méthodes conditionnellement

<!--
By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-15 always implements the
`new` function to return a new instance of `Pair<T>` (recall from the ["Method
Syntax"][methods] ignore
--> section of Chapter 5 that `Self` is a type
alias for the type of the `impl` block, which in this case is `Pair<T>`). But
in the next `impl` block, `Pair<T>` only implements the `cmp_display` method if
its inner type `T` implements the `PartialOrd` trait that enables comparison
_and_ the `Display` trait that enables printing.
-->

En utilisant un trait bound avec un bloc `impl` qui utilise des paramètres de
type générique, nous pouvons implémenter des méthodes conditionnellement pour
les types qui implémentent les traits spécifiés. Par exemple, le type
`Pair<T>` dans l'encart 10-15 implémente toujours la fonction `new` pour
retourner une nouvelle instance de `Pair<T>` (rappelez-vous de la section
[« La syntaxe des méthodes »][methods]<!--
ignore
--> du chapitre 5 que `Self`
est un alias de type pour le type du bloc `impl`, qui dans ce cas est
`Pair<T>`). Mais dans le bloc `impl` suivant, `Pair<T>` n'implémente la
méthode `cmp_display` que si son type interne `T` implémente le trait
`PartialOrd` qui permet la comparaison _et_ le trait `Display` qui permet
l'affichage.

<Listing number="10-15" file-name="src/lib.rs" caption="Implémentation conditionnelle de méthodes sur un type générique en fonction des trait bounds">


```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

<!--
We can also conditionally implement a trait for any type that implements
another trait. Implementations of a trait on any type that satisfies the trait
bounds are called _blanket implementations_ and are used extensively in the
Rust standard library. For example, the standard library implements the
`ToString` trait on any type that implements the `Display` trait. The `impl`
block in the standard library looks similar to this code:
-->

Nous pouvons aussi implémenter conditionnellement un trait pour n'importe quel
type qui implémente un autre trait. Les implémentations d'un trait sur
n'importe quel type qui satisfait les trait bounds sont appelées
_implémentations couvertures_ (blanket implementations) et sont largement
utilisées dans la bibliothèque standard de Rust. Par exemple, la bibliothèque
standard implémente le trait `ToString` sur n'importe quel type qui implémente
le trait `Display`. Le bloc `impl` dans la bibliothèque standard ressemble à
ce code :

<!--
```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```
-->

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

<!--
Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:
-->

Comme la bibliothèque standard possède cette implémentation couverture, nous
pouvons appeler la méthode `to_string` définie par le trait `ToString` sur
n'importe quel type qui implémente le trait `Display`. Par exemple, nous
pouvons convertir des entiers en leurs valeurs `String` correspondantes comme
ceci car les entiers implémentent `Display` :

<!--
```rust
let s = 3.to_string();
```
-->

```rust
let s = 3.to_string();
```

<!--
Blanket implementations appear in the documentation for the trait in the
"Implementors" section.
-->

Les implémentations couvertures apparaissent dans la documentation du trait
dans la section « Implementors ».

<!--
Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type that didn't define the method. But Rust
moves these errors to compile time so that we're forced to fix the problems
before our code is even able to run. Additionally, we don't have to write code
that checks for behavior at runtime, because we've already checked at compile
time. Doing so improves performance without having to give up the flexibility
of generics.
-->

Les traits et les trait bounds nous permettent d'écrire du code qui utilise des
paramètres de type générique pour réduire la duplication tout en spécifiant au
compilateur que nous voulons que le type générique ait un comportement
particulier. Le compilateur peut alors utiliser les informations des trait
bounds pour vérifier que tous les types concrets utilisés avec notre code
fournissent le comportement correct. Dans les langages à typage dynamique, nous
obtiendrions une erreur à l'exécution si nous appelions une méthode sur un
type qui ne définit pas cette méthode. Mais Rust déplace ces erreurs au moment
de la compilation pour que nous soyons obligés de corriger les problèmes avant
même que notre code ne puisse s'exécuter. De plus, nous n'avons pas besoin
d'écrire du code qui vérifie le comportement à l'exécution, car nous l'avons
déjà vérifié à la compilation. Cela améliore les performances sans avoir à
renoncer à la flexibilité des génériques.

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[methods]: ch05-03-method-syntax.html#method-syntax
