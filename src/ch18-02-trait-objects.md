<!--
Old headings. Do not remove or links may break.
-->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>

<!--
## Using Trait Objects to Abstract over Shared Behavior
-->

## Utiliser les objets trait pour abstraire un comportement partagé

<!--
In Chapter 8, we mentioned that one limitation of vectors is that they can
store elements of only one type. We created a workaround in Listing 8-9 where
we defined a `SpreadsheetCell` enum that had variants to hold integers, floats,
and text. This meant we could store different types of data in each cell and
still have a vector that represented a row of cells. This is a perfectly good
solution when our interchangeable items are a fixed set of types that we know
when our code is compiled.
-->

Au chapitre 8, nous avons mentionné qu'une limitation des vecteurs est qu'ils ne peuvent stocker des éléments que d'un seul type. Nous avons créé une solution de contournement dans l'encart 8-9 où nous avons défini un enum `SpreadsheetCell` qui avait des variantes pour contenir des entiers, des flottants et du texte. Cela signifiait que nous pouvions stocker différents types de données dans chaque cellule et avoir quand même un vecteur représentant une ligne de cellules. C'est une solution parfaitement valable quand nos éléments interchangeables sont un ensemble fixe de types que nous connaissons au moment de la compilation de notre code.

<!--
However, sometimes we want our library user to be able to extend the set of
types that are valid in a particular situation. To show how we might achieve
this, we'll create an example graphical user interface (GUI) tool that iterates
through a list of items, calling a `draw` method on each one to draw it to the
screen—a common technique for GUI tools. We'll create a library crate called
`gui` that contains the structure of a GUI library. This crate might include
some types for people to use, such as `Button` or `TextField`. In addition,
`gui` users will want to create their own types that can be drawn: For
instance, one programmer might add an `Image`, and another might add a
`SelectBox`.
-->

Cependant, parfois nous voulons que l'utilisateur de notre bibliothèque puisse étendre l'ensemble des types valides dans une situation particulière. Pour montrer comment nous pourrions y parvenir, nous allons créer un exemple d'outil d'interface graphique utilisateur (GUI) qui parcourt une liste d'éléments, en appelant une méthode `draw` sur chacun pour le dessiner à l'écran — une technique courante pour les outils GUI. Nous créerons un crate de bibliothèque appelé `gui` qui contient la structure d'une bibliothèque GUI. Ce crate pourrait inclure certains types à utiliser, comme `Button` ou `TextField`. De plus, les utilisateurs de `gui` voudront créer leurs propres types pouvant être dessinés : par exemple, un programmeur pourrait ajouter un `Image`, et un autre pourrait ajouter un `SelectBox`.

<!--
At the time of writing the library, we can't know and define all the types
other programmers might want to create. But we do know that `gui` needs to keep
track of many values of different types, and it needs to call a `draw` method
on each of these differently typed values. It doesn't need to know exactly what
will happen when we call the `draw` method, just that the value will have that
method available for us to call.
-->

Au moment d'écrire la bibliothèque, nous ne pouvons pas connaître et définir tous les types que d'autres programmeurs pourraient vouloir créer. Mais nous savons que `gui` doit garder une trace de nombreuses valeurs de différents types, et qu'il doit appeler une méthode `draw` sur chacune de ces valeurs de types différents. Il n'a pas besoin de savoir exactement ce qui va se passer quand nous appelons la méthode `draw`, juste que la valeur aura cette méthode disponible pour que nous puissions l'appeler.

<!--
To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes, such as
`Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But because Rust
doesn't have inheritance, we need another way to structure the `gui` library to
allow users to create new types compatible with the library.
-->

Pour faire cela dans un langage avec héritage, nous pourrions définir une classe nommée `Component` qui a une méthode nommée `draw`. Les autres classes, comme `Button`, `Image` et `SelectBox`, hériteraient de `Component` et donc hériteraient de la méthode `draw`. Elles pourraient chacune surcharger la méthode `draw` pour définir leur comportement personnalisé, mais le framework pourrait traiter tous les types comme s'ils étaient des instances de `Component` et appeler `draw` sur eux. Mais comme Rust n'a pas d'héritage, nous avons besoin d'une autre façon de structurer la bibliothèque `gui` pour permettre aux utilisateurs de créer de nouveaux types compatibles avec la bibliothèque.

<!--
### Defining a Trait for Common Behavior
-->

### Définir un trait pour un comportement commun

<!--
To implement the behavior that we want `gui` to have, we'll define a trait
named `Draw` that will have one method named `draw`. Then, we can define a
vector that takes a trait object. A _trait object_ points to both an instance
of a type implementing our specified trait and a table used to look up trait
methods on that type at runtime. We create a trait object by specifying some
sort of pointer, such as a reference or a `Box<T>` smart pointer, then the
`dyn` keyword, and then specifying the relevant trait. (We'll talk about the
reason trait objects must use a pointer in ["Dynamically Sized Types and the
`Sized` Trait"][dynamically-sized] ignore
--> in Chapter 20.) We can use
trait objects in place of a generic or concrete type. Wherever we use a trait
object, Rust's type system will ensure at compile time that any value used in
that context will implement the trait object's trait. Consequently, we don't
need to know all the possible types at compile time.
-->

Pour implémenter le comportement que nous voulons que `gui` ait, nous définirons un trait nommé `Draw` qui aura une méthode nommée `draw`. Ensuite, nous pourrons définir un vecteur qui prend un objet trait. Un _objet trait_ pointe à la fois vers une instance d'un type implémentant notre trait spécifié et vers une table utilisée pour rechercher les méthodes du trait sur ce type à l'exécution. Nous créons un objet trait en spécifiant une sorte de pointeur, comme une référence ou un pointeur intelligent `Box<T>`, puis le mot-clé `dyn`, puis en spécifiant le trait pertinent. (Nous parlerons de la raison pour laquelle les objets trait doivent utiliser un pointeur dans [« Types de taille dynamique et le trait `Sized` »][dynamically-sized]<!--
ignore
--> au chapitre 20.) Nous pouvons utiliser des objets trait à la place d'un type générique ou concret. Partout où nous utilisons un objet trait, le système de types de Rust s'assurera au moment de la compilation que toute valeur utilisée dans ce contexte implémentera le trait de l'objet trait. Par conséquent, nous n'avons pas besoin de connaître tous les types possibles au moment de la compilation.

<!--
We've mentioned that, in Rust, we refrain from calling structs and enums
"objects" to distinguish them from other languages' objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. Trait objects differ from objects in other
languages in that we can't add data to a trait object. Trait objects aren't as
generally useful as objects in other languages: Their specific purpose is to
allow abstraction across common behavior.
-->

Nous avons mentionné qu'en Rust, nous nous abstenons d'appeler les structs et les enums des « objets » pour les distinguer des objets des autres langages. Dans une struct ou un enum, les données dans les champs de la struct et le comportement dans les blocs `impl` sont séparés, tandis que dans d'autres langages, les données et le comportement combinés en un seul concept sont souvent qualifiés d'objet. Les objets trait diffèrent des objets dans d'autres langages en ce que nous ne pouvons pas ajouter de données à un objet trait. Les objets trait ne sont pas aussi généralement utiles que les objets dans d'autres langages : leur but spécifique est de permettre l'abstraction à travers un comportement commun.

<!--
Listing 18-3 shows how to define a trait named `Draw` with one method named
`draw`.
-->

L'encart 18-3 montre comment définir un trait nommé `Draw` avec une méthode nommée `draw`.

<Listing number="18-3" file-name="src/lib.rs" caption="Définition du trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

<!--
This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 18-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it's a stand-in for any type inside a
`Box` that implements the `Draw` trait.
-->

Cette syntaxe devrait vous être familière de nos discussions sur la façon de définir des traits au chapitre 10. Vient ensuite une nouvelle syntaxe : l'encart 18-4 définit une struct nommée `Screen` qui contient un vecteur nommé `components`. Ce vecteur est de type `Box<dyn Draw>`, qui est un objet trait ; c'est un substitut pour tout type à l'intérieur d'un `Box` qui implémente le trait `Draw`.

<Listing number="18-4" file-name="src/lib.rs" caption="Définition de la struct `Screen` avec un champ `components` contenant un vecteur d'objets trait qui implémentent le trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

<!--
On the `Screen` struct, we'll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 18-5.
-->

Sur la struct `Screen`, nous définirons une méthode nommée `run` qui appellera la méthode `draw` sur chacun de ses `components`, comme montré dans l'encart 18-5.

<Listing number="18-5" file-name="src/lib.rs" caption="Une méthode `run` sur `Screen` qui appelle la méthode `draw` sur chaque composant">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

<!--
This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can be substituted with
only one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound,
as in Listing 18-6.
-->

Cela fonctionne différemment de la définition d'une struct qui utilise un paramètre de type générique avec des contraintes de trait. Un paramètre de type générique ne peut être substitué qu'avec un seul type concret à la fois, tandis que les objets trait permettent à plusieurs types concrets de remplir le rôle de l'objet trait à l'exécution. Par exemple, nous aurions pu définir la struct `Screen` en utilisant un type générique et une contrainte de trait, comme dans l'encart 18-6.

<Listing number="18-6" file-name="src/lib.rs" caption="Une implémentation alternative de la struct `Screen` et de sa méthode `run` utilisant des génériques et des contraintes de trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

<!--
This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you'll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.
-->

Cela nous restreint à une instance de `Screen` qui a une liste de composants tous de type `Button` ou tous de type `TextField`. Si vous n'aurez jamais que des collections homogènes, utiliser des génériques et des contraintes de trait est préférable car les définitions seront monomorphisées au moment de la compilation pour utiliser les types concrets.

<!--
On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let's look at how this works, and then we'll talk about the
runtime performance implications.
-->

D'un autre côté, avec la méthode utilisant des objets trait, une instance de `Screen` peut contenir un `Vec<T>` qui contient un `Box<Button>` ainsi qu'un `Box<TextField>`. Voyons comment cela fonctionne, puis nous parlerons des implications en termes de performance à l'exécution.

<!--
### Implementing the Trait
-->

### Implémenter le trait

<!--
Now we'll add some types that implement the `Draw` trait. We'll provide the
`Button` type. Again, actually implementing a GUI library is beyond the scope
of this book, so the `draw` method won't have any useful implementation in its
body. To imagine what the implementation might look like, a `Button` struct
might have fields for `width`, `height`, and `label`, as shown in Listing 18-7.
-->

Nous allons maintenant ajouter quelques types qui implémentent le trait `Draw`. Nous fournirons le type `Button`. Encore une fois, implémenter réellement une bibliothèque GUI dépasse le cadre de ce livre, donc la méthode `draw` n'aura pas d'implémentation utile dans son corps. Pour imaginer à quoi l'implémentation pourrait ressembler, une struct `Button` pourrait avoir des champs pour `width`, `height` et `label`, comme montré dans l'encart 18-7.

<Listing number="18-7" file-name="src/lib.rs" caption="Une struct `Button` qui implémente le trait `Draw`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

<!--
The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components; for example, a `TextField` type might have those
same fields plus a `placeholder` field. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, as mentioned). The `Button` type, for instance,
might have an additional `impl` block containing methods related to what
happens when a user clicks the button. These kinds of methods won't apply to
types like `TextField`.
-->

Les champs `width`, `height` et `label` de `Button` différeront des champs des autres composants ; par exemple, un type `TextField` pourrait avoir ces mêmes champs plus un champ `placeholder`. Chacun des types que nous voulons dessiner à l'écran implémentera le trait `Draw` mais utilisera un code différent dans la méthode `draw` pour définir comment dessiner ce type particulier, comme `Button` le fait ici (sans le code GUI réel, comme mentionné). Le type `Button`, par exemple, pourrait avoir un bloc `impl` supplémentaire contenant des méthodes liées à ce qui se passe quand un utilisateur clique sur le bouton. Ces types de méthodes ne s'appliqueront pas à des types comme `TextField`.

<!--
If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they would implement the `Draw` trait
on the `SelectBox` type as well, as shown in Listing 18-8.
-->

Si quelqu'un utilisant notre bibliothèque décide d'implémenter une struct `SelectBox` qui a des champs `width`, `height` et `options`, il implémenterait aussi le trait `Draw` sur le type `SelectBox`, comme montré dans l'encart 18-8.

<Listing number="18-8" file-name="src/main.rs" caption="Un autre crate utilisant `gui` et implémentant le trait `Draw` sur une struct `SelectBox`">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

<!--
Our library's user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 18-9 shows this implementation.
-->

L'utilisateur de notre bibliothèque peut maintenant écrire sa fonction `main` pour créer une instance de `Screen`. À l'instance de `Screen`, il peut ajouter un `SelectBox` et un `Button` en mettant chacun dans un `Box<T>` pour devenir un objet trait. Il peut ensuite appeler la méthode `run` sur l'instance de `Screen`, qui appellera `draw` sur chacun des composants. L'encart 18-9 montre cette implémentation.

<Listing number="18-9" file-name="src/main.rs" caption="Utiliser des objets trait pour stocker des valeurs de différents types qui implémentent le même trait">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

<!--
When we wrote the library, we didn't know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.
-->

Quand nous avons écrit la bibliothèque, nous ne savions pas que quelqu'un pourrait ajouter le type `SelectBox`, mais notre implémentation de `Screen` a pu opérer sur le nouveau type et le dessiner parce que `SelectBox` implémente le trait `Draw`, ce qui signifie qu'il implémente la méthode `draw`.

<!--
This concept—of being concerned only with the messages a value responds to
rather than the value's concrete type—is similar to the concept of _duck
typing_ in dynamically typed languages: If it walks like a duck and quacks like
a duck, then it must be a duck! In the implementation of `run` on `Screen` in
Listing 18-5, `run` doesn't need to know what the concrete type of each
component is. It doesn't check whether a component is an instance of a `Button`
or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<dyn Draw>` as the type of the values in the `components`
vector, we've defined `Screen` to need values that we can call the `draw`
method on.
-->

Ce concept — ne se préoccuper que des messages auxquels une valeur répond plutôt que du type concret de la valeur — est similaire au concept de _duck typing_ dans les langages à typage dynamique : si ça marche comme un canard et si ça cancane comme un canard, alors ça doit être un canard ! Dans l'implémentation de `run` sur `Screen` dans l'encart 18-5, `run` n'a pas besoin de connaître le type concret de chaque composant. Il ne vérifie pas si un composant est une instance de `Button` ou de `SelectBox`, il appelle simplement la méthode `draw` sur le composant. En spécifiant `Box<dyn Draw>` comme type des valeurs dans le vecteur `components`, nous avons défini `Screen` comme ayant besoin de valeurs sur lesquelles nous pouvons appeler la méthode `draw`.

<!--
The advantage of using trait objects and Rust's type system to write code
similar to code using duck typing is that we never have to check whether a
value implements a particular method at runtime or worry about getting errors
if a value doesn't implement a method but we call it anyway. Rust won't compile
our code if the values don't implement the traits that the trait objects need.
-->

L'avantage d'utiliser des objets trait et le système de types de Rust pour écrire du code similaire au code utilisant le duck typing est que nous n'avons jamais à vérifier si une valeur implémente une méthode particulière à l'exécution ni à nous inquiéter d'obtenir des erreurs si une valeur n'implémente pas une méthode mais que nous l'appelons quand même. Rust ne compilera pas notre code si les valeurs n'implémentent pas les traits dont les objets trait ont besoin.

<!--
For example, Listing 18-10 shows what happens if we try to create a `Screen`
with a `String` as a component.
-->

Par exemple, l'encart 18-10 montre ce qui se passe si nous essayons de créer un `Screen` avec une `String` comme composant.

<Listing number="18-10" file-name="src/main.rs" caption="Tentative d'utilisation d'un type qui n'implémente pas le trait de l'objet trait">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

<!--
We'll get this error because `String` doesn't implement the `Draw` trait:
-->

Nous obtiendrons cette erreur parce que `String` n'implémente pas le trait `Draw` :

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

<!--
This error lets us know that either we're passing something to `Screen` that we
didn't mean to pass and so should pass a different type, or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.
-->

Cette erreur nous indique que soit nous passons quelque chose à `Screen` que nous n'avions pas l'intention de passer et donc nous devrions passer un type différent, soit nous devrions implémenter `Draw` sur `String` pour que `Screen` puisse appeler `draw` dessus.

<!--
Old headings. Do not remove or links may break.
-->

<a id="trait-objects-perform-dynamic-dispatch"></a>

<!--
### Performing Dynamic Dispatch
-->

### Effectuer la répartition dynamique

<!--
Recall in ["Performance of Code Using
Generics"][performance-of-code-using-generics] ignore
--> in Chapter 10 our
discussion on the monomorphization process performed on generics by the
compiler: The compiler generates nongeneric implementations of functions and
methods for each concrete type that we use in place of a generic type
parameter. The code that results from monomorphization is doing _static
dispatch_, which is when the compiler knows what method you're calling at
compile time. This is opposed to _dynamic dispatch_, which is when the compiler
can't tell at compile time which method you're calling. In dynamic dispatch
cases, the compiler emits code that at runtime will know which method to call.
-->

Rappelez-vous notre discussion dans [« Performance du code utilisant les génériques »][performance-of-code-using-generics]<!--
ignore
--> au chapitre 10 sur le processus de monomorphisation effectué sur les génériques par le compilateur : le compilateur génère des implémentations non-génériques de fonctions et de méthodes pour chaque type concret que nous utilisons à la place d'un paramètre de type générique. Le code résultant de la monomorphisation effectue une _répartition statique_ (_static dispatch_), c'est-à-dire que le compilateur sait quelle méthode vous appelez au moment de la compilation. Cela s'oppose à la _répartition dynamique_ (_dynamic dispatch_), où le compilateur ne peut pas déterminer au moment de la compilation quelle méthode vous appelez. Dans les cas de répartition dynamique, le compilateur émet du code qui saura à l'exécution quelle méthode appeler.

<!--
When we use trait objects, Rust must use dynamic dispatch. The compiler doesn't
know all the types that might be used with the code that's using trait objects,
so it doesn't know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to
call. This lookup incurs a runtime cost that doesn't occur with static dispatch.
Dynamic dispatch also prevents the compiler from choosing to inline a method's
code, which in turn prevents some optimizations, and Rust has some rules about
where you can and cannot use dynamic dispatch, called _dyn compatibility_. Those
rules are beyond the scope of this discussion, but you can read more about them
[in the reference][dyn-compatibility] ignore
-->. However, we did get extra
flexibility in the code that we wrote in Listing 18-5 and were able to support
in Listing 18-9, so it's a trade-off to consider.
-->

Quand nous utilisons des objets trait, Rust doit utiliser la répartition dynamique. Le compilateur ne connaît pas tous les types qui pourraient être utilisés avec le code utilisant des objets trait, donc il ne sait pas quelle méthode implémentée sur quel type appeler. À la place, à l'exécution, Rust utilise les pointeurs à l'intérieur de l'objet trait pour savoir quelle méthode appeler. Cette recherche engendre un coût à l'exécution qui ne se produit pas avec la répartition statique. La répartition dynamique empêche aussi le compilateur de choisir d'inliner le code d'une méthode, ce qui à son tour empêche certaines optimisations, et Rust a certaines règles sur les endroits où vous pouvez et ne pouvez pas utiliser la répartition dynamique, appelées _compatibilité dyn_. Ces règles dépassent le cadre de cette discussion, mais vous pouvez en lire davantage [dans la référence][dyn-compatibility]<!--
ignore
-->. Cependant, nous avons obtenu une flexibilité supplémentaire dans le code que nous avons écrit dans l'encart 18-5 et que nous avons pu prendre en charge dans l'encart 18-9, donc c'est un compromis à considérer.

[performance-of-code-using-generics]: ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
