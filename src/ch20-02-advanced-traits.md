<!--
## Advanced Traits
-->

## Les traits avancés

<!--
We first covered traits in the ["Defining Shared Behavior with
Traits"][traits] ignore
--> section in Chapter 10, but we didn't discuss
the more advanced details. Now that you know more about Rust, we can get into
the nitty-gritty.
-->

Nous avons d'abord couvert les traits dans la section ["Définir un comportement partagé avec les traits"][traits]<!--
ignore
--> du chapitre 10, mais nous n'avons pas discuté des détails plus avancés. Maintenant que vous en savez plus sur Rust, nous pouvons entrer dans le vif du sujet.

<!--
Old headings. Do not remove or links may break.
-->

<a id="specifying-placeholder-types-in-trait-definitions-with-associated-types"></a>
<a id="associated-types"></a>

<!--
### Defining Traits with Associated Types
-->

### Définir des traits avec des types associés

<!--
_Associated types_ connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures. The
implementor of a trait will specify the concrete type to be used instead of the
placeholder type for the particular implementation. That way, we can define a
trait that uses some types without needing to know exactly what those types are
until the trait is implemented.
-->

Les _types associés_ connectent un type générique de substitution avec un trait de sorte que les définitions des méthodes du trait puissent utiliser ces types de substitution dans leurs signatures. L'implémenteur d'un trait spécifiera le type concret à utiliser à la place du type de substitution pour l'implémentation particulière. De cette manière, nous pouvons définir un trait qui utilise certains types sans avoir besoin de savoir exactement quels sont ces types jusqu'à ce que le trait soit implémenté.

<!--
We've described most of the advanced features in this chapter as being rarely
needed. Associated types are somewhere in the middle: They're used more rarely
than features explained in the rest of the book but more commonly than many of
the other features discussed in this chapter.
-->

Nous avons décrit la plupart des fonctionnalités avancées de ce chapitre comme étant rarement nécessaires. Les types associés se situent quelque part au milieu : ils sont utilisés plus rarement que les fonctionnalités expliquées dans le reste du livre mais plus couramment que beaucoup d'autres fonctionnalités discutées dans ce chapitre.

<!--
One example of a trait with an associated type is the `Iterator` trait that the
standard library provides. The associated type is named `Item` and stands in
for the type of the values the type implementing the `Iterator` trait is
iterating over. The definition of the `Iterator` trait is as shown in Listing
20-13.
-->

Un exemple de trait avec un type associé est le trait `Iterator` fourni par la bibliothèque standard. Le type associé est nommé `Item` et représente le type des valeurs sur lesquelles le type implémentant le trait `Iterator` itère. La définition du trait `Iterator` est celle montrée dans l'encart 20-13.

<Listing number="20-13" caption="La définition du trait `Iterator` qui possède un type associé `Item`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

<!--
The type `Item` is a placeholder, and the `next` method's definition shows that
it will return values of type `Option<Self::Item>`. Implementors of the
`Iterator` trait will specify the concrete type for `Item`, and the `next`
method will return an `Option` containing a value of that concrete type.
-->

Le type `Item` est un type de substitution, et la définition de la méthode `next` montre qu'elle retournera des valeurs de type `Option<Self::Item>`. Les implémenteurs du trait `Iterator` spécifieront le type concret pour `Item`, et la méthode `next` retournera une `Option` contenant une valeur de ce type concret.

<!--
Associated types might seem like a similar concept to generics, in that the
latter allow us to define a function without specifying what types it can
handle. To examine the difference between the two concepts, we'll look at an
implementation of the `Iterator` trait on a type named `Counter` that specifies
the `Item` type is `u32`:
-->

Les types associés pourraient sembler être un concept similaire aux génériques, dans la mesure où ces derniers nous permettent de définir une fonction sans spécifier quels types elle peut traiter. Pour examiner la différence entre les deux concepts, nous allons regarder une implémentation du trait `Iterator` sur un type nommé `Counter` qui spécifie que le type `Item` est `u32` :

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

<!--
This syntax seems comparable to that of generics. So, why not just define the
`Iterator` trait with generics, as shown in Listing 20-14?
-->

Cette syntaxe semble comparable à celle des génériques. Alors, pourquoi ne pas simplement définir le trait `Iterator` avec des génériques, comme montré dans l'encart 20-14 ?

<Listing number="20-14" caption="Une définition hypothétique du trait `Iterator` utilisant des génériques">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

<!--
The difference is that when using generics, as in Listing 20-14, we must
annotate the types in each implementation; because we can also implement
`Iterator<String> for Counter` or any other type, we could have multiple
implementations of `Iterator` for `Counter`. In other words, when a trait has a
generic parameter, it can be implemented for a type multiple times, changing
the concrete types of the generic type parameters each time. When we use the
`next` method on `Counter`, we would have to provide type annotations to
indicate which implementation of `Iterator` we want to use.
-->

La différence est que lorsque nous utilisons des génériques, comme dans l'encart 20-14, nous devons annoter les types dans chaque implémentation ; parce que nous pourrions aussi implémenter `Iterator<String> for Counter` ou n'importe quel autre type, nous pourrions avoir plusieurs implémentations d'`Iterator` pour `Counter`. En d'autres termes, lorsqu'un trait a un paramètre générique, il peut être implémenté pour un type plusieurs fois, en changeant les types concrets des paramètres de type générique à chaque fois. Lorsque nous utilisons la méthode `next` sur `Counter`, nous devrions fournir des annotations de type pour indiquer quelle implémentation d'`Iterator` nous voulons utiliser.

<!--
With associated types, we don't need to annotate types, because we can't
implement a trait on a type multiple times. In Listing 20-13 with the
definition that uses associated types, we can choose what the type of `Item`
will be only once because there can be only one `impl Iterator for Counter`. We
don't have to specify that we want an iterator of `u32` values everywhere we
call `next` on `Counter`.
-->

Avec les types associés, nous n'avons pas besoin d'annoter les types, car nous ne pouvons pas implémenter un trait sur un type plusieurs fois. Dans l'encart 20-13 avec la définition qui utilise des types associés, nous ne pouvons choisir le type d'`Item` qu'une seule fois car il ne peut y avoir qu'un seul `impl Iterator for Counter`. Nous n'avons pas à spécifier que nous voulons un itérateur de valeurs `u32` partout où nous appelons `next` sur `Counter`.

<!--
Associated types also become part of the trait's contract: Implementors of the
trait must provide a type to stand in for the associated type placeholder.
Associated types often have a name that describes how the type will be used,
and documenting the associated type in the API documentation is a good practice.
-->

Les types associés font aussi partie du contrat du trait : les implémenteurs du trait doivent fournir un type pour remplacer le type de substitution associé. Les types associés ont souvent un nom qui décrit comment le type sera utilisé, et documenter le type associé dans la documentation de l'API est une bonne pratique.

<!--
Old headings. Do not remove or links may break.
-->

<a id="default-generic-type-parameters-and-operator-overloading"></a>

<!--
### Using Default Generic Parameters and Operator Overloading
-->

### Utiliser les paramètres de type générique par défaut et la surcharge d'opérateurs

<!--
When we use generic type parameters, we can specify a default concrete type for
the generic type. This eliminates the need for implementors of the trait to
specify a concrete type if the default type works. You specify a default type
when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax.
-->

Lorsque nous utilisons des paramètres de type générique, nous pouvons spécifier un type concret par défaut pour le type générique. Cela élimine la nécessité pour les implémenteurs du trait de spécifier un type concret si le type par défaut convient. Vous spécifiez un type par défaut en déclarant un type générique avec la syntaxe `<TypeDeSubstitution=TypeConcret>`.

<!--
A great example of a situation where this technique is useful is with _operator
overloading_, in which you customize the behavior of an operator (such as `+`)
in particular situations.
-->

Un excellent exemple de situation où cette technique est utile est la _surcharge d'opérateurs_, dans laquelle vous personnalisez le comportement d'un opérateur (comme `+`) dans des situations particulières.

<!--
Rust doesn't allow you to create your own operators or overload arbitrary
operators. But you can overload the operations and corresponding traits listed
in `std::ops` by implementing the traits associated with the operator. For
example, in Listing 20-15, we overload the `+` operator to add two `Point`
instances together. We do this by implementing the `Add` trait on a `Point`
struct.
-->

Rust ne vous permet pas de créer vos propres opérateurs ni de surcharger des opérateurs arbitraires. Mais vous pouvez surcharger les opérations et les traits correspondants listés dans `std::ops` en implémentant les traits associés à l'opérateur. Par exemple, dans l'encart 20-15, nous surchargeons l'opérateur `+` pour additionner deux instances de `Point`. Nous faisons cela en implémentant le trait `Add` sur une structure `Point`.

<Listing number="20-15" file-name="src/main.rs" caption="Implémenter le trait `Add` pour surcharger l'opérateur `+` pour les instances de `Point`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

<!--
The `add` method adds the `x` values of two `Point` instances and the `y`
values of two `Point` instances to create a new `Point`. The `Add` trait has an
associated type named `Output` that determines the type returned from the `add`
method.
-->

La méthode `add` additionne les valeurs `x` de deux instances de `Point` et les valeurs `y` de deux instances de `Point` pour créer un nouveau `Point`. Le trait `Add` possède un type associé nommé `Output` qui détermine le type retourné par la méthode `add`.

<!--
The default generic type in this code is within the `Add` trait. Here is its
definition:
-->

Le type générique par défaut dans ce code se trouve dans le trait `Add`. Voici sa définition :

<!--
```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```
-->

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

<!--
This code should look generally familiar: a trait with one method and an
associated type. The new part is `Rhs=Self`: This syntax is called _default
type parameters_. The `Rhs` generic type parameter (short for "right-hand
side") defines the type of the `rhs` parameter in the `add` method. If we don't
specify a concrete type for `Rhs` when we implement the `Add` trait, the type
of `Rhs` will default to `Self`, which will be the type we're implementing
`Add` on.
-->

Ce code devrait vous sembler généralement familier : un trait avec une méthode et un type associé. La partie nouvelle est `Rhs=Self` : cette syntaxe s'appelle les _paramètres de type par défaut_. Le paramètre de type générique `Rhs` (abréviation de "right-hand side", côté droit) définit le type du paramètre `rhs` dans la méthode `add`. Si nous ne spécifions pas de type concret pour `Rhs` lorsque nous implémentons le trait `Add`, le type de `Rhs` sera par défaut `Self`, qui sera le type sur lequel nous implémentons `Add`.

<!--
When we implemented `Add` for `Point`, we used the default for `Rhs` because we
wanted to add two `Point` instances. Let's look at an example of implementing
the `Add` trait where we want to customize the `Rhs` type rather than using the
default.
-->

Lorsque nous avons implémenté `Add` pour `Point`, nous avons utilisé la valeur par défaut pour `Rhs` car nous voulions additionner deux instances de `Point`. Regardons un exemple d'implémentation du trait `Add` où nous voulons personnaliser le type `Rhs` plutôt que d'utiliser la valeur par défaut.

<!--
We have two structs, `Millimeters` and `Meters`, holding values in different
units. This thin wrapping of an existing type in another struct is known as the
_newtype pattern_, which we describe in more detail in the ["Implementing
External Traits with the Newtype Pattern"][newtype] ignore
--> section. We
want to add values in millimeters to values in meters and have the
implementation of `Add` do the conversion correctly. We can implement `Add` for
`Millimeters` with `Meters` as the `Rhs`, as shown in Listing 20-16.
-->

Nous avons deux structures, `Millimeters` et `Meters`, contenant des valeurs dans des unités différentes. Cet enveloppement fin d'un type existant dans une autre structure est connu sous le nom de _patron newtype_, que nous décrivons plus en détail dans la section ["Implémenter des traits externes avec le patron newtype"][newtype]<!--
ignore
-->. Nous voulons additionner des valeurs en millimètres à des valeurs en mètres et que l'implémentation d'`Add` fasse la conversion correctement. Nous pouvons implémenter `Add` pour `Millimeters` avec `Meters` comme `Rhs`, comme montré dans l'encart 20-16.

<Listing number="20-16" file-name="src/lib.rs" caption="Implémenter le trait `Add` sur `Millimeters` pour additionner `Millimeters` et `Meters`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

<!--
To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the
value of the `Rhs` type parameter instead of using the default of `Self`.
-->

Pour additionner `Millimeters` et `Meters`, nous spécifions `impl Add<Meters>` pour définir la valeur du paramètre de type `Rhs` au lieu d'utiliser la valeur par défaut `Self`.

<!--
You'll use default type parameters in two main ways:
-->

Vous utiliserez les paramètres de type par défaut de deux manières principales :

<!--
1. To extend a type without breaking existing code
2. To allow customization in specific cases most users won't need
-->

1. Pour étendre un type sans casser le code existant
2. Pour permettre la personnalisation dans des cas spécifiques dont la plupart des utilisateurs n'auront pas besoin

<!--
The standard library's `Add` trait is an example of the second purpose:
Usually, you'll add two like types, but the `Add` trait provides the ability to
customize beyond that. Using a default type parameter in the `Add` trait
definition means you don't have to specify the extra parameter most of the
time. In other words, a bit of implementation boilerplate isn't needed, making
it easier to use the trait.
-->

Le trait `Add` de la bibliothèque standard est un exemple du second objectif : généralement, vous additionnerez deux types identiques, mais le trait `Add` offre la possibilité de personnaliser au-delà. Utiliser un paramètre de type par défaut dans la définition du trait `Add` signifie que vous n'avez pas à spécifier le paramètre supplémentaire la plupart du temps. En d'autres termes, un peu de code standard d'implémentation n'est pas nécessaire, rendant le trait plus facile à utiliser.

<!--
The first purpose is similar to the second but in reverse: If you want to add a
type parameter to an existing trait, you can give it a default to allow
extension of the functionality of the trait without breaking the existing
implementation code.
-->

Le premier objectif est similaire au second mais en sens inverse : si vous voulez ajouter un paramètre de type à un trait existant, vous pouvez lui donner une valeur par défaut pour permettre l'extension de la fonctionnalité du trait sans casser le code d'implémentation existant.

<!--
Old headings. Do not remove or links may break.
-->

<a id="fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name"></a>
<a id="disambiguating-between-methods-with-the-same-name"></a>

<!--
### Disambiguating Between Identically Named Methods
-->

### Lever l'ambiguïté entre des méthodes portant le même nom

<!--
Nothing in Rust prevents a trait from having a method with the same name as
another trait's method, nor does Rust prevent you from implementing both traits
on one type. It's also possible to implement a method directly on the type with
the same name as methods from traits.
-->

Rien en Rust n'empêche un trait d'avoir une méthode portant le même nom que la méthode d'un autre trait, et Rust ne vous empêche pas non plus d'implémenter les deux traits sur un même type. Il est aussi possible d'implémenter une méthode directement sur le type avec le même nom que les méthodes des traits.

<!--
When calling methods with the same name, you'll need to tell Rust which one you
want to use. Consider the code in Listing 20-17 where we've defined two traits,
`Pilot` and `Wizard`, that both have a method called `fly`. We then implement
both traits on a type `Human` that already has a method named `fly` implemented
on it. Each `fly` method does something different.
-->

Lors de l'appel de méthodes portant le même nom, vous devrez indiquer à Rust laquelle vous souhaitez utiliser. Considérez le code de l'encart 20-17 où nous avons défini deux traits, `Pilot` et `Wizard`, qui ont tous deux une méthode appelée `fly`. Nous implémentons ensuite les deux traits sur un type `Human` qui possède déjà une méthode nommée `fly`. Chaque méthode `fly` fait quelque chose de différent.

<Listing number="20-17" file-name="src/main.rs" caption="Deux traits sont définis avec une méthode `fly` et sont implémentés sur le type `Human`, et une méthode `fly` est implémentée directement sur `Human`.">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

<!--
When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 20-18.
-->

Lorsque nous appelons `fly` sur une instance de `Human`, le compilateur appelle par défaut la méthode directement implémentée sur le type, comme montré dans l'encart 20-18.

<Listing number="20-18" file-name="src/main.rs" caption="Appeler `fly` sur une instance de `Human`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

<!--
Running this code will print `*waving arms furiously*`, showing that Rust
called the `fly` method implemented on `Human` directly.
-->

L'exécution de ce code affichera `*waving arms furiously*`, montrant que Rust a appelé la méthode `fly` implémentée directement sur `Human`.

<!--
To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait,
we need to use more explicit syntax to specify which `fly` method we mean.
Listing 20-19 demonstrates this syntax.
-->

Pour appeler les méthodes `fly` du trait `Pilot` ou du trait `Wizard`, nous devons utiliser une syntaxe plus explicite pour spécifier quelle méthode `fly` nous voulons dire. L'encart 20-19 illustre cette syntaxe.

<Listing number="20-19" file-name="src/main.rs" caption="Spécifier quelle méthode `fly` de quel trait nous voulons appeler">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

<!--
Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also write
`Human::fly(&person)`, which is equivalent to the `person.fly()` that we used
in Listing 20-19, but this is a bit longer to write if we don't need to
disambiguate.
-->

Spécifier le nom du trait avant le nom de la méthode clarifie pour Rust quelle implémentation de `fly` nous voulons appeler. Nous pourrions aussi écrire `Human::fly(&person)`, qui est équivalent au `person.fly()` que nous avons utilisé dans l'encart 20-19, mais c'est un peu plus long à écrire si nous n'avons pas besoin de lever l'ambiguïté.

<!--
Running this code prints the following:
-->

L'exécution de ce code affiche ce qui suit :


```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

<!--
Because the `fly` method takes a `self` parameter, if we had two _types_ that
both implement one _trait_, Rust could figure out which implementation of a
trait to use based on the type of `self`.
-->

Parce que la méthode `fly` prend un paramètre `self`, si nous avions deux _types_ qui implémentent tous les deux un même _trait_, Rust pourrait déterminer quelle implémentation du trait utiliser en se basant sur le type de `self`.

<!--
However, associated functions that are not methods don't have a `self`
parameter. When there are multiple types or traits that define non-method
functions with the same function name, Rust doesn't always know which type you
mean unless you use fully qualified syntax. For example, in Listing 20-20, we
create a trait for an animal shelter that wants to name all baby dogs Spot. We
make an `Animal` trait with an associated non-method function `baby_name`. The
`Animal` trait is implemented for the struct `Dog`, on which we also provide an
associated non-method function `baby_name` directly.
-->

Cependant, les fonctions associées qui ne sont pas des méthodes n'ont pas de paramètre `self`. Lorsqu'il y a plusieurs types ou traits qui définissent des fonctions non-méthodes avec le même nom de fonction, Rust ne sait pas toujours quel type vous voulez dire à moins que vous n'utilisiez la syntaxe pleinement qualifiée. Par exemple, dans l'encart 20-20, nous créons un trait pour un refuge animalier qui veut nommer tous les chiots Spot. Nous créons un trait `Animal` avec une fonction associée non-méthode `baby_name`. Le trait `Animal` est implémenté pour la structure `Dog`, sur laquelle nous fournissons aussi directement une fonction associée non-méthode `baby_name`.

<Listing number="20-20" file-name="src/main.rs" caption="Un trait avec une fonction associée et un type avec une fonction associée du même nom qui implémente aussi le trait">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

<!--
We implement the code for naming all puppies Spot in the `baby_name` associated
function that is defined on `Dog`. The `Dog` type also implements the trait
`Animal`, which describes characteristics that all animals have. Baby dogs are
called puppies, and that is expressed in the implementation of the `Animal`
trait on `Dog` in the `baby_name` function associated with the `Animal` trait.
-->

Nous implémentons le code pour nommer tous les chiots Spot dans la fonction associée `baby_name` définie sur `Dog`. Le type `Dog` implémente aussi le trait `Animal`, qui décrit les caractéristiques communes à tous les animaux. Les bébés chiens s'appellent des chiots, et cela est exprimé dans l'implémentation du trait `Animal` sur `Dog` dans la fonction `baby_name` associée au trait `Animal`.

<!--
In `main`, we call the `Dog::baby_name` function, which calls the associated
function defined on `Dog` directly. This code prints the following:
-->

Dans `main`, nous appelons la fonction `Dog::baby_name`, qui appelle la fonction associée définie directement sur `Dog`. Ce code affiche ce qui suit :


```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

<!--
This output isn't what we wanted. We want to call the `baby_name` function that
is part of the `Animal` trait that we implemented on `Dog` so that the code
prints `A baby dog is called a puppy`. The technique of specifying the trait
name that we used in Listing 20-19 doesn't help here; if we change `main` to
the code in Listing 20-21, we'll get a compilation error.
-->

Cette sortie n'est pas ce que nous voulions. Nous voulons appeler la fonction `baby_name` qui fait partie du trait `Animal` que nous avons implémenté sur `Dog` pour que le code affiche `A baby dog is called a puppy`. La technique de spécification du nom du trait que nous avons utilisée dans l'encart 20-19 ne nous aide pas ici ; si nous changeons `main` avec le code de l'encart 20-21, nous obtiendrons une erreur de compilation.

<Listing number="20-21" file-name="src/main.rs" caption="Tentative d'appel de la fonction `baby_name` du trait `Animal`, mais Rust ne sait pas quelle implémentation utiliser">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

<!--
Because `Animal::baby_name` doesn't have a `self` parameter, and there could be
other types that implement the `Animal` trait, Rust can't figure out which
implementation of `Animal::baby_name` we want. We'll get this compiler error:
-->

Parce que `Animal::baby_name` n'a pas de paramètre `self`, et qu'il pourrait y avoir d'autres types qui implémentent le trait `Animal`, Rust ne peut pas déterminer quelle implémentation d'`Animal::baby_name` nous voulons. Nous obtiendrons cette erreur du compilateur :


```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

<!--
To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog` as opposed to the implementation of `Animal` for some other
type, we need to use fully qualified syntax. Listing 20-22 demonstrates how to
use fully qualified syntax.
-->

Pour lever l'ambiguïté et dire à Rust que nous voulons utiliser l'implémentation d'`Animal` pour `Dog` par opposition à l'implémentation d'`Animal` pour un autre type, nous devons utiliser la syntaxe pleinement qualifiée. L'encart 20-22 montre comment utiliser la syntaxe pleinement qualifiée.

<Listing number="20-22" file-name="src/main.rs" caption="Utiliser la syntaxe pleinement qualifiée pour spécifier que nous voulons appeler la fonction `baby_name` du trait `Animal` telle qu'implémentée sur `Dog`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

<!--
We're providing Rust with a type annotation within the angle brackets, which
indicates we want to call the `baby_name` method from the `Animal` trait as
implemented on `Dog` by saying that we want to treat the `Dog` type as an
`Animal` for this function call. This code will now print what we want:
-->

Nous fournissons à Rust une annotation de type entre les chevrons, qui indique que nous voulons appeler la méthode `baby_name` du trait `Animal` telle qu'implémentée sur `Dog` en disant que nous voulons traiter le type `Dog` comme un `Animal` pour cet appel de fonction. Ce code affichera maintenant ce que nous voulons :


```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

<!--
In general, fully qualified syntax is defined as follows:
-->

En général, la syntaxe pleinement qualifiée est définie comme suit :

<!--
```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```
-->

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

<!--
For associated functions that aren't methods, there would not be a `receiver`:
There would only be the list of other arguments. You could use fully qualified
syntax everywhere that you call functions or methods. However, you're allowed
to omit any part of this syntax that Rust can figure out from other information
in the program. You only need to use this more verbose syntax in cases where
there are multiple implementations that use the same name and Rust needs help
to identify which implementation you want to call.
-->

Pour les fonctions associées qui ne sont pas des méthodes, il n'y aurait pas de `receiver` : il n'y aurait que la liste des autres arguments. Vous pourriez utiliser la syntaxe pleinement qualifiée partout où vous appelez des fonctions ou des méthodes. Cependant, vous êtes autorisé à omettre toute partie de cette syntaxe que Rust peut déduire d'autres informations dans le programme. Vous n'avez besoin d'utiliser cette syntaxe plus verbeuse que dans les cas où il y a plusieurs implémentations utilisant le même nom et où Rust a besoin d'aide pour identifier quelle implémentation vous voulez appeler.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-supertraits-to-require-one-traits-functionality-within-another-trait"></a>

<!--
### Using Supertraits
-->

### Utiliser les supertraits

<!--
Sometimes you might write a trait definition that depends on another trait: For
a type to implement the first trait, you want to require that type to also
implement the second trait. You would do this so that your trait definition can
make use of the associated items of the second trait. The trait your trait
definition is relying on is called a _supertrait_ of your trait.
-->

Parfois, vous pourriez écrire une définition de trait qui dépend d'un autre trait : pour qu'un type implémente le premier trait, vous voulez exiger que ce type implémente aussi le second trait. Vous feriez cela pour que la définition de votre trait puisse utiliser les éléments associés du second trait. Le trait sur lequel votre définition de trait s'appuie est appelé un _supertrait_ de votre trait.

<!--
For example, let's say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a given value formatted so that it's
framed in asterisks. That is, given a `Point` struct that implements the
standard library trait `Display` to result in `(x, y)`, when we call
`outline_print` on a `Point` instance that has `1` for `x` and `3` for `y`, it
should print the following:
-->

Par exemple, disons que nous voulons créer un trait `OutlinePrint` avec une méthode `outline_print` qui affichera une valeur donnée formatée de sorte qu'elle soit encadrée d'astérisques. C'est-à-dire, étant donné une structure `Point` qui implémente le trait `Display` de la bibliothèque standard pour produire `(x, y)`, lorsque nous appelons `outline_print` sur une instance de `Point` qui a `1` pour `x` et `3` pour `y`, elle devrait afficher ce qui suit :

<!--
```text
**********
*        *
* (1, 3) *
*        *
**********
```
-->

```text
**********
*        *
* (1, 3) *
*        *
**********
```

<!--
In the implementation of the `outline_print` method, we want to use the
`Display` trait's functionality. Therefore, we need to specify that the
`OutlinePrint` trait will work only for types that also implement `Display` and
provide the functionality that `OutlinePrint` needs. We can do that in the
trait definition by specifying `OutlinePrint: Display`. This technique is
similar to adding a trait bound to the trait. Listing 20-23 shows an
implementation of the `OutlinePrint` trait.
-->

Dans l'implémentation de la méthode `outline_print`, nous voulons utiliser la fonctionnalité du trait `Display`. Par conséquent, nous devons spécifier que le trait `OutlinePrint` ne fonctionnera que pour les types qui implémentent aussi `Display` et fournissent la fonctionnalité dont `OutlinePrint` a besoin. Nous pouvons faire cela dans la définition du trait en spécifiant `OutlinePrint: Display`. Cette technique est similaire à l'ajout d'une contrainte de trait au trait. L'encart 20-23 montre une implémentation du trait `OutlinePrint`.

<Listing number="20-23" file-name="src/main.rs" caption="Implémenter le trait `OutlinePrint` qui nécessite la fonctionnalité de `Display`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

<!--
Because we've specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding a
colon and specifying the `Display` trait after the trait name, we'd get an
error saying that no method named `to_string` was found for the type `&Self` in
the current scope.
-->

Parce que nous avons spécifié qu'`OutlinePrint` nécessite le trait `Display`, nous pouvons utiliser la fonction `to_string` qui est automatiquement implémentée pour tout type qui implémente `Display`. Si nous essayions d'utiliser `to_string` sans ajouter un deux-points et spécifier le trait `Display` après le nom du trait, nous obtiendrions une erreur disant qu'aucune méthode nommée `to_string` n'a été trouvée pour le type `&Self` dans la portée actuelle.

<!--
Let's see what happens when we try to implement `OutlinePrint` on a type that
doesn't implement `Display`, such as the `Point` struct:
-->

Voyons ce qui se passe lorsque nous essayons d'implémenter `OutlinePrint` sur un type qui n'implémente pas `Display`, comme la structure `Point` :

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

<!--
We get an error saying that `Display` is required but not implemented:
-->

Nous obtenons une erreur disant que `Display` est requis mais n'est pas implémenté :


```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

<!--
To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:
-->

Pour corriger cela, nous implémentons `Display` sur `Point` et satisfaisons la contrainte qu'`OutlinePrint` exige, comme suit :

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

<!--
Then, implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.
-->

Ensuite, l'implémentation du trait `OutlinePrint` sur `Point` compilera avec succès, et nous pourrons appeler `outline_print` sur une instance de `Point` pour l'afficher dans un cadre d'astérisques.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-the-newtype-pattern-to-implement-external-traits-on-external-types"></a>
<a id="using-the-newtype-pattern-to-implement-external-traits"></a>

<!--
### Implementing External Traits with the Newtype Pattern
-->

### Implémenter des traits externes avec le patron newtype

<!--
In the ["Implementing a Trait on a Type"][implementing-a-trait-on-a-type]
ignore
--> section in Chapter 10, we mentioned the orphan rule that states
we're only allowed to implement a trait on a type if either the trait or the
type, or both, are local to our crate. It's possible to get around this
restriction using the newtype pattern, which involves creating a new type in a
tuple struct. (We covered tuple structs in the ["Creating Different Types with
Tuple Structs"][tuple-structs]<!--
ignore
--> section in Chapter 5.) The tuple
struct will have one field and be a thin wrapper around the type for which we
want to implement a trait. Then, the wrapper type is local to our crate, and we
can implement the trait on the wrapper. _Newtype_ is a term that originates
from the Haskell programming language. There is no runtime performance penalty
for using this pattern, and the wrapper type is elided at compile time.
-->

Dans la section ["Implémenter un trait sur un type"][implementing-a-trait-on-a-type]<!--
ignore
--> du chapitre 10, nous avons mentionné la règle de l'orphelin qui stipule que nous ne sommes autorisés à implémenter un trait sur un type que si soit le trait, soit le type, soit les deux, sont locaux à notre crate. Il est possible de contourner cette restriction en utilisant le patron newtype, qui consiste à créer un nouveau type dans une structure tuple. (Nous avons couvert les structures tuple dans la section ["Créer différents types avec les structures tuple"][tuple-structs]<!--
ignore
--> du chapitre 5.) La structure tuple aura un seul champ et sera un enveloppement fin autour du type pour lequel nous voulons implémenter un trait. Ensuite, le type enveloppeur est local à notre crate, et nous pouvons implémenter le trait sur l'enveloppeur. _Newtype_ est un terme qui provient du langage de programmation Haskell. Il n'y a pas de pénalité de performance à l'exécution pour l'utilisation de ce patron, et le type enveloppeur est éliminé au moment de la compilation.

<!--
As an example, let's say we want to implement `Display` on `Vec<T>`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct
that holds an instance of `Vec<T>`; then, we can implement `Display` on
`Wrapper` and use the `Vec<T>` value, as shown in Listing 20-24.
-->

À titre d'exemple, disons que nous voulons implémenter `Display` sur `Vec<T>`, ce que la règle de l'orphelin nous empêche de faire directement car le trait `Display` et le type `Vec<T>` sont définis en dehors de notre crate. Nous pouvons créer une structure `Wrapper` qui contient une instance de `Vec<T>` ; ensuite, nous pouvons implémenter `Display` sur `Wrapper` et utiliser la valeur `Vec<T>`, comme montré dans l'encart 20-24.

<Listing number="20-24" file-name="src/main.rs" caption="Créer un type `Wrapper` autour de `Vec<String>` pour implémenter `Display`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

<!--
The implementation of `Display` uses `self.0` to access the inner `Vec<T>`
because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the
tuple. Then, we can use the functionality of the `Display` trait on `Wrapper`.
-->

L'implémentation de `Display` utilise `self.0` pour accéder au `Vec<T>` interne car `Wrapper` est une structure tuple et `Vec<T>` est l'élément à l'indice 0 dans le tuple. Ensuite, nous pouvons utiliser la fonctionnalité du trait `Display` sur `Wrapper`.

<!--
The downside of using this technique is that `Wrapper` is a new type, so it
doesn't have the methods of the value it's holding. We would have to implement
all the methods of `Vec<T>` directly on `Wrapper` such that the methods
delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a
`Vec<T>`. If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait on the `Wrapper` to return the inner type would
be a solution (we discussed implementing the `Deref` trait in the ["Treating
Smart Pointers Like Regular References"][smart-pointer-deref] ignore
-->
section in Chapter 15). If we didn't want the `Wrapper` type to have all the
methods of the inner type—for example, to restrict the `Wrapper` type's
behavior—we would have to implement just the methods we do want manually.
-->

L'inconvénient d'utiliser cette technique est que `Wrapper` est un nouveau type, il n'a donc pas les méthodes de la valeur qu'il contient. Nous devrions implémenter toutes les méthodes de `Vec<T>` directement sur `Wrapper` de sorte que les méthodes délèguent à `self.0`, ce qui nous permettrait de traiter `Wrapper` exactement comme un `Vec<T>`. Si nous voulions que le nouveau type ait toutes les méthodes du type interne, implémenter le trait `Deref` sur `Wrapper` pour retourner le type interne serait une solution (nous avons discuté de l'implémentation du trait `Deref` dans la section ["Traiter les pointeurs intelligents comme des références normales"][smart-pointer-deref]<!--
ignore
--> du chapitre 15). Si nous ne voulions pas que le type `Wrapper` ait toutes les méthodes du type interne -- par exemple, pour restreindre le comportement du type `Wrapper` -- nous devrions implémenter manuellement seulement les méthodes que nous souhaitons.

<!--
This newtype pattern is also useful even when traits are not involved. Let's
switch focus and look at some advanced ways to interact with Rust's type system.
-->

Ce patron newtype est aussi utile même quand les traits ne sont pas impliqués. Changeons de sujet et examinons quelques manières avancées d'interagir avec le système de types de Rust.

[newtype]: ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern
[implementing-a-trait-on-a-type]: ch10-02-traits.html#implementing-a-trait-on-a-type
[traits]: ch10-02-traits.html
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references
[tuple-structs]: ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
