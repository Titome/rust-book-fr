<!--
## Advanced Types
-->

## Les types avancés

<!--
The Rust type system has some features that we've so far mentioned but haven't
yet discussed. We'll start by discussing newtypes in general as we examine why
they are useful as types. Then, we'll move on to type aliases, a feature
similar to newtypes but with slightly different semantics. We'll also discuss
the `!` type and dynamically sized types.
-->

Le système de types de Rust possède des fonctionnalités que nous avons mentionnées jusqu'à présent mais que nous n'avons pas encore discutées. Nous commencerons par discuter des newtypes en général en examinant pourquoi ils sont utiles en tant que types. Ensuite, nous passerons aux alias de types, une fonctionnalité similaire aux newtypes mais avec une sémantique légèrement différente. Nous discuterons aussi du type `!` et des types à taille dynamique.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-the-newtype-pattern-for-type-safety-and-abstraction"></a>

<!--
### Type Safety and Abstraction with the Newtype Pattern
-->

### Sécurité de type et abstraction avec le patron newtype

<!--
This section assumes you've read the earlier section ["Implementing External
Traits with the Newtype Pattern"][newtype] ignore
-->. The newtype pattern
is also useful for tasks beyond those we've discussed so far, including
statically enforcing that values are never confused and indicating the units of
a value. You saw an example of using newtypes to indicate units in Listing
20-16: Recall that the `Millimeters` and `Meters` structs wrapped `u32` values
in a newtype. If we wrote a function with a parameter of type `Millimeters`, we
wouldn't be able to compile a program that accidentally tried to call that
function with a value of type `Meters` or a plain `u32`.
-->

Cette section suppose que vous avez lu la section précédente ["Implémenter des traits externes avec le patron newtype"][newtype]<!--
ignore
-->. Le patron newtype est aussi utile pour des tâches au-delà de celles que nous avons discutées jusqu'à présent, notamment pour imposer statiquement que les valeurs ne soient jamais confondues et indiquer les unités d'une valeur. Vous avez vu un exemple d'utilisation des newtypes pour indiquer les unités dans l'encart 20-16 : rappelez-vous que les structures `Millimeters` et `Meters` enveloppaient des valeurs `u32` dans un newtype. Si nous écrivions une fonction avec un paramètre de type `Millimeters`, nous ne pourrions pas compiler un programme qui essaierait accidentellement d'appeler cette fonction avec une valeur de type `Meters` ou un simple `u32`.

<!--
We can also use the newtype pattern to abstract away some implementation
details of a type: The new type can expose a public API that is different from
the API of the private inner type.
-->

Nous pouvons aussi utiliser le patron newtype pour abstraire certains détails d'implémentation d'un type : le nouveau type peut exposer une API publique différente de l'API du type interne privé.

<!--
Newtypes can also hide internal implementation. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person's ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection; that code wouldn't need to know that we assign an `i32` ID to names
internally. The newtype pattern is a lightweight way to achieve encapsulation
to hide implementation details, which we discussed in the ["Encapsulation that
Hides Implementation
Details"][encapsulation-that-hides-implementation-details] ignore
-->
section in Chapter 18.
-->

Les newtypes peuvent aussi masquer l'implémentation interne. Par exemple, nous pourrions fournir un type `People` pour envelopper un `HashMap<i32, String>` qui stocke l'identifiant d'une personne associé à son nom. Le code utilisant `People` n'interagirait qu'avec l'API publique que nous fournissons, comme une méthode pour ajouter un nom en chaîne de caractères à la collection `People` ; ce code n'aurait pas besoin de savoir que nous assignons un identifiant `i32` aux noms en interne. Le patron newtype est un moyen léger d'atteindre l'encapsulation pour masquer les détails d'implémentation, ce dont nous avons discuté dans la section ["L'encapsulation qui masque les détails d'implémentation"][encapsulation-that-hides-implementation-details]<!--
ignore
--> du chapitre 18.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-type-synonyms-with-type-aliases"></a>

<!--
### Type Synonyms and Type Aliases
-->

### Les synonymes de types et les alias de types

<!--
Rust provides the ability to declare a _type alias_ to give an existing type
another name. For this we use the `type` keyword. For example, we can create
the alias `Kilometers` to `i32` like so:
-->

Rust offre la possibilité de déclarer un _alias de type_ pour donner un autre nom à un type existant. Pour cela, nous utilisons le mot-clé `type`. Par exemple, nous pouvons créer l'alias `Kilometers` pour `i32` comme suit :


```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

<!--
Now the alias `Kilometers` is a _synonym_ for `i32`; unlike the `Millimeters`
and `Meters` types we created in Listing 20-16, `Kilometers` is not a separate,
new type. Values that have the type `Kilometers` will be treated the same as
values of type `i32`:
-->

Maintenant l'alias `Kilometers` est un _synonyme_ de `i32` ; contrairement aux types `Millimeters` et `Meters` que nous avons créés dans l'encart 20-16, `Kilometers` n'est pas un type nouveau et distinct. Les valeurs de type `Kilometers` seront traitées de la même manière que les valeurs de type `i32` :


```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

<!--
Because `Kilometers` and `i32` are the same type, we can add values of both
types and can pass `Kilometers` values to functions that take `i32`
parameters. However, using this method, we don't get the type-checking benefits
that we get from the newtype pattern discussed earlier. In other words, if we
mix up `Kilometers` and `i32` values somewhere, the compiler will not give us
an error.
-->

Comme `Kilometers` et `i32` sont le même type, nous pouvons additionner des valeurs des deux types et passer des valeurs `Kilometers` à des fonctions qui prennent des paramètres `i32`. Cependant, avec cette méthode, nous n'obtenons pas les avantages de vérification de types que nous obtenons avec le patron newtype discuté précédemment. En d'autres termes, si nous mélangeons des valeurs `Kilometers` et `i32` quelque part, le compilateur ne nous donnera pas d'erreur.

<!--
The main use case for type synonyms is to reduce repetition. For example, we
might have a lengthy type like this:
-->

Le principal cas d'utilisation des synonymes de types est de réduire la répétition. Par exemple, nous pourrions avoir un type long comme celui-ci :

<!--
```rust,ignore
Box<dyn Fn() + Send + 'static>
```
-->

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

<!--
Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error-prone. Imagine having a project full of
code like that in Listing 20-25.
-->

Écrire ce type long dans les signatures de fonctions et comme annotations de type partout dans le code peut être fastidieux et source d'erreurs. Imaginez avoir un projet plein de code comme dans l'encart 20-25.

<Listing number="20-25" caption="Utiliser un type long à de nombreux endroits">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

<!--
A type alias makes this code more manageable by reducing the repetition. In
Listing 20-26, we've introduced an alias named `Thunk` for the verbose type and
can replace all uses of the type with the shorter alias `Thunk`.
-->

Un alias de type rend ce code plus gérable en réduisant la répétition. Dans l'encart 20-26, nous avons introduit un alias nommé `Thunk` pour le type verbeux et pouvons remplacer toutes les utilisations du type par l'alias plus court `Thunk`.

<Listing number="20-26" caption="Introduire un alias de type, `Thunk`, pour réduire la répétition">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

<!--
This code is much easier to read and write! Choosing a meaningful name for a
type alias can help communicate your intent as well (_thunk_ is a word for code
to be evaluated at a later time, so it's an appropriate name for a closure that
gets stored).
-->

Ce code est beaucoup plus facile à lire et à écrire ! Choisir un nom significatif pour un alias de type peut aussi aider à communiquer votre intention (_thunk_ est un mot désignant du code à évaluer ultérieurement, c'est donc un nom approprié pour une fermeture qui est stockée).

<!--
Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition. Consider the `std::io` module in the standard library. I/O
operations often return a `Result<T, E>` to handle situations when operations
fail to work. This library has a `std::io::Error` struct that represents all
possible I/O errors. Many of the functions in `std::io` will be returning
`Result<T, E>` where the `E` is `std::io::Error`, such as these functions in
the `Write` trait:
-->

Les alias de types sont aussi couramment utilisés avec le type `Result<T, E>` pour réduire la répétition. Considérez le module `std::io` de la bibliothèque standard. Les opérations d'E/S retournent souvent un `Result<T, E>` pour gérer les situations où les opérations échouent. Cette bibliothèque possède une structure `std::io::Error` qui représente toutes les erreurs d'E/S possibles. Beaucoup de fonctions dans `std::io` retourneront un `Result<T, E>` où `E` est `std::io::Error`, comme ces fonctions dans le trait `Write` :


```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

<!--
The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type
alias declaration:
-->

Le `Result<..., Error>` est beaucoup répété. C'est pourquoi `std::io` possède cette déclaration d'alias de type :


```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

<!--
Because this declaration is in the `std::io` module, we can use the fully
qualified alias `std::io::Result<T>`; that is, a `Result<T, E>` with the `E`
filled in as `std::io::Error`. The `Write` trait function signatures end up
looking like this:
-->

Comme cette déclaration se trouve dans le module `std::io`, nous pouvons utiliser l'alias pleinement qualifié `std::io::Result<T>` ; c'est-à-dire un `Result<T, E>` avec `E` rempli par `std::io::Error`. Les signatures des fonctions du trait `Write` finissent par ressembler à ceci :


```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

<!--
The type alias helps in two ways: It makes code easier to write _and_ it gives
us a consistent interface across all of `std::io`. Because it's an alias, it's
just another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, as well as special syntax like the `?` operator.
-->

L'alias de type aide de deux manières : il rend le code plus facile à écrire _et_ nous donne une interface cohérente dans tout `std::io`. Parce que c'est un alias, c'est juste un autre `Result<T, E>`, ce qui signifie que nous pouvons utiliser toutes les méthodes qui fonctionnent sur `Result<T, E>` avec lui, ainsi que la syntaxe spéciale comme l'opérateur `?`.

<!--
### The Never Type That Never Returns
-->

### Le type never qui ne retourne jamais

<!--
Rust has a special type named `!` that's known in type theory lingo as the
_empty type_ because it has no values. We prefer to call it the _never type_
because it stands in the place of the return type when a function will never
return. Here is an example:
-->

Rust possède un type spécial nommé `!` qui est connu dans le jargon de la théorie des types comme le _type vide_ car il n'a pas de valeurs. Nous préférons l'appeler le _type never_ car il tient la place du type de retour lorsqu'une fonction ne retournera jamais. Voici un exemple :


```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

<!--
This code is read as "the function `bar` returns never." Functions that return
never are called _diverging functions_. We can't create values of the type `!`,
so `bar` can never possibly return.
-->

Ce code se lit comme "la fonction `bar` ne retourne jamais." Les fonctions qui ne retournent jamais sont appelées _fonctions divergentes_. Nous ne pouvons pas créer de valeurs du type `!`, donc `bar` ne peut jamais retourner.

<!--
But what use is a type you can never create values for? Recall the code from
Listing 2-5, part of the number-guessing game; we've reproduced a bit of it
here in Listing 20-27.
-->

Mais à quoi sert un type dont vous ne pouvez jamais créer de valeurs ? Rappelez-vous le code de l'encart 2-5, partie du jeu de devinette de nombres ; nous en avons reproduit un extrait ici dans l'encart 20-27.

<Listing number="20-27" caption="Un `match` avec une branche qui se termine par `continue`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

<!--
At the time, we skipped over some details in this code. In ["The `match`
Control Flow Construct"][the-match-control-flow-construct] ignore
-->
section in Chapter 6, we discussed that `match` arms must all return the same
type. So, for example, the following code doesn't work:
-->

À l'époque, nous avions passé sous silence certains détails de ce code. Dans la section ["La construction de flux de contrôle `match`"][the-match-control-flow-construct]<!--
ignore
--> du chapitre 6, nous avons discuté du fait que toutes les branches de `match` doivent retourner le même type. Ainsi, par exemple, le code suivant ne fonctionne pas :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

<!--
The type of `guess` in this code would have to be an integer _and_ a string,
and Rust requires that `guess` have only one type. So, what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 20-27?
-->

Le type de `guess` dans ce code devrait être à la fois un entier _et_ une chaîne de caractères, et Rust exige que `guess` n'ait qu'un seul type. Alors, que retourne `continue` ? Comment avons-nous été autorisés à retourner un `u32` d'une branche et à avoir une autre branche qui se termine par `continue` dans l'encart 20-27 ?

<!--
As you might have guessed, `continue` has a `!` value. That is, when Rust
computes the type of `guess`, it looks at both match arms, the former with a
value of `u32` and the latter with a `!` value. Because `!` can never have a
value, Rust decides that the type of `guess` is `u32`.
-->

Comme vous l'avez peut-être deviné, `continue` a une valeur de type `!`. C'est-à-dire, lorsque Rust calcule le type de `guess`, il regarde les deux branches du match, la première avec une valeur `u32` et la seconde avec une valeur `!`. Parce que `!` ne peut jamais avoir de valeur, Rust décide que le type de `guess` est `u32`.

<!--
The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We're allowed to end this `match` arm with
`continue` because `continue` doesn't return a value; instead, it moves control
back to the top of the loop, so in the `Err` case, we never assign a value to
`guess`.
-->

La manière formelle de décrire ce comportement est que les expressions de type `!` peuvent être converties implicitement en n'importe quel autre type. Nous sommes autorisés à terminer cette branche du `match` avec `continue` car `continue` ne retourne pas de valeur ; à la place, il transfère le contrôle au début de la boucle, donc dans le cas `Err`, nous n'assignons jamais de valeur à `guess`.

<!--
The never type is useful with the `panic!` macro as well. Recall the `unwrap`
function that we call on `Option<T>` values to produce a value or panic with
this definition:
-->

Le type never est aussi utile avec la macro `panic!`. Rappelez-vous la fonction `unwrap` que nous appelons sur les valeurs `Option<T>` pour produire une valeur ou paniquer avec cette définition :


```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

<!--
In this code, the same thing happens as in the `match` in Listing 20-27: Rust
sees that `val` has the type `T` and `panic!` has the type `!`, so the result
of the overall `match` expression is `T`. This code works because `panic!`
doesn't produce a value; it ends the program. In the `None` case, we won't be
returning a value from `unwrap`, so this code is valid.
-->

Dans ce code, la même chose se passe que dans le `match` de l'encart 20-27 : Rust voit que `val` a le type `T` et `panic!` a le type `!`, donc le résultat de l'expression `match` globale est `T`. Ce code fonctionne car `panic!` ne produit pas de valeur ; il termine le programme. Dans le cas `None`, nous ne retournerons pas de valeur depuis `unwrap`, donc ce code est valide.

<!--
One final expression that has the type `!` is a loop:
-->

Une dernière expression qui a le type `!` est une boucle :


```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

<!--
Here, the loop never ends, so `!` is the value of the expression. However, this
wouldn't be true if we included a `break`, because the loop would terminate
when it got to the `break`.
-->

Ici, la boucle ne se termine jamais, donc `!` est la valeur de l'expression. Cependant, ce ne serait pas vrai si nous incluions un `break`, car la boucle se terminerait lorsqu'elle atteindrait le `break`.

<!--
### Dynamically Sized Types and the `Sized` Trait
-->

### Les types à taille dynamique et le trait `Sized`

<!--
Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type. This leaves one corner of its type
system a little confusing at first: the concept of _dynamically sized types_.
Sometimes referred to as _DSTs_ or _unsized types_, these types let us write
code using values whose size we can know only at runtime.
-->

Rust a besoin de connaître certains détails sur ses types, comme combien d'espace allouer pour une valeur d'un type particulier. Cela laisse un coin de son système de types un peu déroutant au début : le concept de _types à taille dynamique_. Parfois appelés _DST_ ou _types non dimensionnés_, ces types nous permettent d'écrire du code utilisant des valeurs dont nous ne pouvons connaître la taille qu'à l'exécution.

<!--
Let's dig into the details of a dynamically sized type called `str`, which
we've been using throughout the book. That's right, not `&str`, but `str` on
its own, is a DST. In many cases, such as when storing text entered by a user,
we can't know how long the string is until runtime. That means we can't create
a variable of type `str`, nor can we take an argument of type `str`. Consider
the following code, which does not work:
-->

Creusons les détails d'un type à taille dynamique appelé `str`, que nous avons utilisé tout au long du livre. C'est exact, pas `&str`, mais `str` seul, est un DST. Dans de nombreux cas, comme lors du stockage de texte saisi par un utilisateur, nous ne pouvons pas savoir quelle est la longueur de la chaîne avant l'exécution. Cela signifie que nous ne pouvons pas créer une variable de type `str`, ni prendre un argument de type `str`. Considérez le code suivant, qui ne fonctionne pas :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

<!--
Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If Rust
allowed us to write this code, these two `str` values would need to take up the
same amount of space. But they have different lengths: `s1` needs 12 bytes of
storage and `s2` needs 15. This is why it's not possible to create a variable
holding a dynamically sized type.
-->

Rust a besoin de savoir combien de mémoire allouer pour toute valeur d'un type particulier, et toutes les valeurs d'un type doivent utiliser la même quantité de mémoire. Si Rust nous permettait d'écrire ce code, ces deux valeurs `str` devraient occuper la même quantité d'espace. Mais elles ont des longueurs différentes : `s1` a besoin de 12 octets de stockage et `s2` a besoin de 15. C'est pourquoi il n'est pas possible de créer une variable contenant un type à taille dynamique.

<!--
So, what do we do? In this case, you already know the answer: We make the type
of `s1` and `s2` string slice (`&str`) rather than `str`. Recall from the
["String Slices"][string-slices] ignore
--> section in Chapter 4 that the
slice data structure only stores the starting position and the length of the
slice. So, although `&T` is a single value that stores the memory address of
where the `T` is located, a string slice is _two_ values: the address of the
`str` and its length. As such, we can know the size of a string slice value at
compile time: It's twice the length of a `usize`. That is, we always know the
size of a string slice, no matter how long the string it refers to is. In
general, this is the way in which dynamically sized types are used in Rust:
They have an extra bit of metadata that stores the size of the dynamic
information. The golden rule of dynamically sized types is that we must always
put values of dynamically sized types behind a pointer of some kind.
-->

Alors, que faisons-nous ? Dans ce cas, vous connaissez déjà la réponse : nous faisons du type de `s1` et `s2` une slice de chaîne (`&str`) plutôt que `str`. Rappelez-vous de la section ["Les slices de chaînes"][string-slices]<!--
ignore
--> du chapitre 4 que la structure de données slice ne stocke que la position de début et la longueur de la slice. Ainsi, bien que `&T` soit une seule valeur qui stocke l'adresse mémoire où se trouve le `T`, une slice de chaîne est _deux_ valeurs : l'adresse du `str` et sa longueur. En tant que tel, nous pouvons connaître la taille d'une valeur de slice de chaîne au moment de la compilation : c'est deux fois la longueur d'un `usize`. C'est-à-dire que nous connaissons toujours la taille d'une slice de chaîne, quelle que soit la longueur de la chaîne à laquelle elle se réfère. En général, c'est de cette manière que les types à taille dynamique sont utilisés en Rust : ils ont un bit supplémentaire de métadonnées qui stocke la taille de l'information dynamique. La règle d'or des types à taille dynamique est que nous devons toujours placer les valeurs de types à taille dynamique derrière un pointeur d'un type ou d'un autre.

<!--
We can combine `str` with all kinds of pointers: for example, `Box<str>` or
`Rc<str>`. In fact, you've seen this before but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In the ["Using Trait Objects to Abstract over
Shared Behavior"][using-trait-objects-to-abstract-over-shared-behavior]
ignore
--> section in Chapter 18, we mentioned that to use traits as trait
objects, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn
Trait>` (`Rc<dyn Trait>` would work too).
-->

Nous pouvons combiner `str` avec toutes sortes de pointeurs : par exemple, `Box<str>` ou `Rc<str>`. En fait, vous avez déjà vu cela mais avec un type à taille dynamique différent : les traits. Chaque trait est un type à taille dynamique auquel nous pouvons nous référer en utilisant le nom du trait. Dans la section ["Utiliser les objets trait pour abstraire un comportement partagé"][using-trait-objects-to-abstract-over-shared-behavior]<!--
ignore
--> du chapitre 18, nous avons mentionné que pour utiliser des traits comme objets trait, nous devons les placer derrière un pointeur, comme `&dyn Trait` ou `Box<dyn Trait>` (`Rc<dyn Trait>` fonctionnerait aussi).

<!--
To work with DSTs, Rust provides the `Sized` trait to determine whether or not
a type's size is known at compile time. This trait is automatically implemented
for everything whose size is known at compile time. In addition, Rust
implicitly adds a bound on `Sized` to every generic function. That is, a
generic function definition like this:
-->

Pour travailler avec les DST, Rust fournit le trait `Sized` pour déterminer si la taille d'un type est connue ou non au moment de la compilation. Ce trait est automatiquement implémenté pour tout ce dont la taille est connue au moment de la compilation. De plus, Rust ajoute implicitement une contrainte sur `Sized` à chaque fonction générique. C'est-à-dire qu'une définition de fonction générique comme celle-ci :


```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

<!--
is actually treated as though we had written this:
-->

est en fait traitée comme si nous avions écrit ceci :


```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

<!--
By default, generic functions will work only on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:
-->

Par défaut, les fonctions génériques ne fonctionneront que sur des types dont la taille est connue au moment de la compilation. Cependant, vous pouvez utiliser la syntaxe spéciale suivante pour assouplir cette restriction :


```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

<!--
A trait bound on `?Sized` means "`T` may or may not be `Sized`," and this
notation overrides the default that generic types must have a known size at
compile time. The `?Trait` syntax with this meaning is only available for
`Sized`, not any other traits.
-->

Une contrainte de trait sur `?Sized` signifie "`T` peut ou non être `Sized`", et cette notation remplace la valeur par défaut selon laquelle les types génériques doivent avoir une taille connue au moment de la compilation. La syntaxe `?Trait` avec cette signification n'est disponible que pour `Sized`, pas pour d'autres traits.

<!--
Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be `Sized`, we need to use it behind some kind of
pointer. In this case, we've chosen a reference.
-->

Notez aussi que nous avons changé le type du paramètre `t` de `T` à `&T`. Parce que le type pourrait ne pas être `Sized`, nous devons l'utiliser derrière un pointeur d'un type ou d'un autre. Dans ce cas, nous avons choisi une référence.

<!--
Next, we'll talk about functions and closures!
-->

Ensuite, nous allons parler des fonctions et des fermetures !

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-construct]: ch06-02-match.html#the-match-control-flow-construct
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[newtype]: ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern
