<!--
## Characteristics of Object-Oriented Languages
-->

## Caractéristiques des langages orientés objet

<!--
There is no consensus in the programming community about what features a
language must have to be considered object oriented. Rust is influenced by many
programming paradigms, including OOP; for example, we explored the features
that came from functional programming in Chapter 13. Arguably, OOP languages
share certain common characteristics—namely, objects, encapsulation, and
inheritance. Let's look at what each of those characteristics means and whether
Rust supports it.
-->

Il n'y a pas de consensus dans la communauté de la programmation sur les fonctionnalités qu'un langage doit posséder pour être considéré comme orienté objet. Rust est influencé par de nombreux paradigmes de programmation, y compris la POO ; par exemple, nous avons exploré les fonctionnalités issues de la programmation fonctionnelle au chapitre 13. On peut argumenter que les langages POO partagent certaines caractéristiques communes — à savoir les objets, l'encapsulation et l'héritage. Voyons ce que chacune de ces caractéristiques signifie et si Rust la prend en charge.

<!--
### Objects Contain Data and Behavior
-->

### Les objets contiennent des données et du comportement

<!--
The book _Design Patterns: Elements of Reusable Object-Oriented Software_ by
Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wesley,
1994), colloquially referred to as _The Gang of Four_ book, is a catalog of
object-oriented design patterns. It defines OOP in this way:
-->

Le livre _Design Patterns: Elements of Reusable Object-Oriented Software_ d'Erich Gamma, Richard Helm, Ralph Johnson et John Vlissides (Addison-Wesley, 1994), communément appelé le livre du _Gang of Four_ (GoF), est un catalogue de patrons de conception orientés objet. Il définit la POO de cette manière :

<!--
> Object-oriented programs are made up of objects. An **object** packages both
> data and the procedures that operate on that data. The procedures are
> typically called **methods** or **operations**.
-->

> Les programmes orientés objet sont composés d'objets. Un **objet** regroupe à la fois les données et les procédures qui opèrent sur ces données. Les procédures sont généralement appelées **méthodes** ou **opérations**.

<!--
Using this definition, Rust is object oriented: Structs and enums have data,
and `impl` blocks provide methods on structs and enums. Even though structs and
enums with methods aren't _called_ objects, they provide the same
functionality, according to the Gang of Four's definition of objects.
-->

Selon cette définition, Rust est orienté objet : les structs et les enums contiennent des données, et les blocs `impl` fournissent des méthodes sur les structs et les enums. Même si les structs et les enums avec des méthodes ne sont pas _appelés_ objets, ils fournissent la même fonctionnalité, selon la définition des objets du Gang of Four.

<!--
### Encapsulation That Hides Implementation Details
-->

### L'encapsulation qui masque les détails d'implémentation

<!--
Another aspect commonly associated with OOP is the idea of _encapsulation_,
which means that the implementation details of an object aren't accessible to
code using that object. Therefore, the only way to interact with an object is
through its public API; code using the object shouldn't be able to reach into
the object's internals and change data or behavior directly. This enables the
programmer to change and refactor an object's internals without needing to
change the code that uses the object.
-->

Un autre aspect couramment associé à la POO est l'idée d'_encapsulation_, ce qui signifie que les détails d'implémentation d'un objet ne sont pas accessibles au code utilisant cet objet. Par conséquent, la seule façon d'interagir avec un objet est à travers son API publique ; le code utilisant l'objet ne devrait pas pouvoir accéder aux composants internes de l'objet et modifier directement les données ou le comportement. Cela permet au programmeur de modifier et refactoriser les composants internes d'un objet sans avoir besoin de modifier le code qui utilise l'objet.

<!--
We discussed how to control encapsulation in Chapter 7: We can use the `pub`
keyword to decide which modules, types, functions, and methods in our code
should be public, and by default everything else is private. For example, we
can define a struct `AveragedCollection` that has a field containing a vector
of `i32` values. The struct can also have a field that contains the average of
the values in the vector, meaning the average doesn't have to be computed on
demand whenever anyone needs it. In other words, `AveragedCollection` will
cache the calculated average for us. Listing 18-1 has the definition of the
`AveragedCollection` struct.
-->

Nous avons discuté de la façon de contrôler l'encapsulation au chapitre 7 : nous pouvons utiliser le mot-clé `pub` pour décider quels modules, types, fonctions et méthodes de notre code doivent être publics, et par défaut tout le reste est privé. Par exemple, nous pouvons définir une struct `AveragedCollection` qui a un champ contenant un vecteur de valeurs `i32`. La struct peut aussi avoir un champ qui contient la moyenne des valeurs du vecteur, ce qui signifie que la moyenne n'a pas besoin d'être calculée à la demande chaque fois que quelqu'un en a besoin. En d'autres termes, `AveragedCollection` mettra en cache la moyenne calculée pour nous. L'encart 18-1 contient la définition de la struct `AveragedCollection`.

<Listing number="18-1" file-name="src/lib.rs" caption="Une struct `AveragedCollection` qui maintient une liste d'entiers et la moyenne des éléments de la collection">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

<!--
The struct is marked `pub` so that other code can use it, but the fields within
the struct remain private. This is important in this case because we want to
ensure that whenever a value is added or removed from the list, the average is
also updated. We do this by implementing `add`, `remove`, and `average` methods
on the struct, as shown in Listing 18-2.
-->

La struct est marquée `pub` pour que d'autre code puisse l'utiliser, mais les champs au sein de la struct restent privés. C'est important dans ce cas car nous voulons nous assurer que chaque fois qu'une valeur est ajoutée ou retirée de la liste, la moyenne est également mise à jour. Nous faisons cela en implémentant les méthodes `add`, `remove` et `average` sur la struct, comme montré dans l'encart 18-2.

<Listing number="18-2" file-name="src/lib.rs" caption="Implémentations des méthodes publiques `add`, `remove` et `average` sur `AveragedCollection`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

<!--
The public methods `add`, `remove`, and `average` are the only ways to access
or modify data in an instance of `AveragedCollection`. When an item is added to
`list` using the `add` method or removed using the `remove` method, the
implementations of each call the private `update_average` method that handles
updating the `average` field as well.
-->

Les méthodes publiques `add`, `remove` et `average` sont les seules façons d'accéder ou de modifier les données dans une instance de `AveragedCollection`. Quand un élément est ajouté à `list` en utilisant la méthode `add` ou retiré en utilisant la méthode `remove`, les implémentations de chacune appellent la méthode privée `update_average` qui gère aussi la mise à jour du champ `average`.

<!--
We leave the `list` and `average` fields private so that there is no way for
external code to add or remove items to or from the `list` field directly;
otherwise, the `average` field might become out of sync when the `list`
changes. The `average` method returns the value in the `average` field,
allowing external code to read the `average` but not modify it.
-->

Nous laissons les champs `list` et `average` privés pour qu'il n'y ait aucun moyen pour du code externe d'ajouter ou de retirer des éléments du champ `list` directement ; sinon, le champ `average` pourrait se désynchroniser quand `list` change. La méthode `average` retourne la valeur du champ `average`, permettant au code externe de lire la moyenne mais pas de la modifier.

<!--
Because we've encapsulated the implementation details of the struct
`AveragedCollection`, we can easily change aspects, such as the data structure,
in the future. For instance, we could use a `HashSet<i32>` instead of a
`Vec<i32>` for the `list` field. As long as the signatures of the `add`,
`remove`, and `average` public methods stayed the same, code using
`AveragedCollection` wouldn't need to change. If we made `list` public instead,
this wouldn't necessarily be the case: `HashSet<i32>` and `Vec<i32>` have
different methods for adding and removing items, so the external code would
likely have to change if it were modifying `list` directly.
-->

Comme nous avons encapsulé les détails d'implémentation de la struct `AveragedCollection`, nous pouvons facilement changer des aspects, comme la structure de données, à l'avenir. Par exemple, nous pourrions utiliser un `HashSet<i32>` au lieu d'un `Vec<i32>` pour le champ `list`. Tant que les signatures des méthodes publiques `add`, `remove` et `average` restent les mêmes, le code utilisant `AveragedCollection` n'aurait pas besoin de changer. Si nous avions rendu `list` public à la place, ce ne serait pas nécessairement le cas : `HashSet<i32>` et `Vec<i32>` ont des méthodes différentes pour ajouter et retirer des éléments, donc le code externe devrait probablement changer s'il modifiait `list` directement.

<!--
If encapsulation is a required aspect for a language to be considered object
oriented, then Rust meets that requirement. The option to use `pub` or not for
different parts of code enables encapsulation of implementation details.
-->

Si l'encapsulation est un aspect requis pour qu'un langage soit considéré comme orienté objet, alors Rust satisfait cette exigence. L'option d'utiliser `pub` ou non pour différentes parties du code permet l'encapsulation des détails d'implémentation.

<!--
### Inheritance as a Type System and as Code Sharing
-->

### L'héritage comme système de types et comme partage de code

<!--
_Inheritance_ is a mechanism whereby an object can inherit elements from
another object's definition, thus gaining the parent object's data and behavior
without you having to define them again.
-->

L'_héritage_ est un mécanisme par lequel un objet peut hériter d'éléments de la définition d'un autre objet, obtenant ainsi les données et le comportement de l'objet parent sans avoir à les redéfinir.

<!--
If a language must have inheritance to be object oriented, then Rust is not
such a language. There is no way to define a struct that inherits the parent
struct's fields and method implementations without using a macro.
-->

Si un langage doit avoir l'héritage pour être orienté objet, alors Rust n'est pas un tel langage. Il n'y a aucun moyen de définir une struct qui hérite des champs et des implémentations de méthodes de la struct parente sans utiliser une macro.

<!--
However, if you're used to having inheritance in your programming toolbox, you
can use other solutions in Rust, depending on your reason for reaching for
inheritance in the first place.
-->

Cependant, si vous êtes habitué à avoir l'héritage dans votre boîte à outils de programmation, vous pouvez utiliser d'autres solutions en Rust, selon la raison pour laquelle vous avez recours à l'héritage en premier lieu.

<!--
You would choose inheritance for two main reasons. One is for reuse of code:
You can implement particular behavior for one type, and inheritance enables you
to reuse that implementation for a different type. You can do this in a limited
way in Rust code using default trait method implementations, which you saw in
Listing 10-14 when we added a default implementation of the `summarize` method
on the `Summary` trait. Any type implementing the `Summary` trait would have
the `summarize` method available on it without any further code. This is
similar to a parent class having an implementation of a method and an
inheriting child class also having the implementation of the method. We can
also override the default implementation of the `summarize` method when we
implement the `Summary` trait, which is similar to a child class overriding the
implementation of a method inherited from a parent class.
-->

Vous choisiriez l'héritage pour deux raisons principales. La première est la réutilisation du code : vous pouvez implémenter un comportement particulier pour un type, et l'héritage vous permet de réutiliser cette implémentation pour un type différent. Vous pouvez faire cela de manière limitée en Rust en utilisant les implémentations par défaut des méthodes de trait, ce que vous avez vu dans l'encart 10-14 quand nous avons ajouté une implémentation par défaut de la méthode `summarize` sur le trait `Summary`. Tout type implémentant le trait `Summary` aurait la méthode `summarize` disponible sans code supplémentaire. C'est similaire à une classe parente ayant une implémentation d'une méthode et une classe enfant héritante ayant aussi l'implémentation de la méthode. Nous pouvons aussi surcharger l'implémentation par défaut de la méthode `summarize` quand nous implémentons le trait `Summary`, ce qui est similaire à une classe enfant surchargeant l'implémentation d'une méthode héritée d'une classe parente.

<!--
The other reason to use inheritance relates to the type system: to enable a
child type to be used in the same places as the parent type. This is also
called _polymorphism_, which means that you can substitute multiple objects for
each other at runtime if they share certain characteristics.
-->

L'autre raison d'utiliser l'héritage est liée au système de types : permettre à un type enfant d'être utilisé aux mêmes endroits que le type parent. Cela s'appelle aussi le _polymorphisme_, ce qui signifie que vous pouvez substituer plusieurs objets les uns aux autres à l'exécution s'ils partagent certaines caractéristiques.

<!--
> ### Polymorphism
>
> To many people, polymorphism is synonymous with inheritance. But it's
> actually a more general concept that refers to code that can work with data of
> multiple types. For inheritance, those types are generally subclasses.
>
> Rust instead uses generics to abstract over different possible types and
> trait bounds to impose constraints on what those types must provide. This is
> sometimes called _bounded parametric polymorphism_.
-->

> ### Polymorphisme
>
> Pour beaucoup de gens, le polymorphisme est synonyme d'héritage. Mais c'est en réalité un concept plus général qui fait référence à du code pouvant travailler avec des données de plusieurs types. Pour l'héritage, ces types sont généralement des sous-classes.
>
> Rust utilise plutôt les génériques pour abstraire les différents types possibles et les contraintes de trait pour imposer des contraintes sur ce que ces types doivent fournir. Cela s'appelle parfois le _polymorphisme paramétrique borné_.

<!--
Rust has chosen a different set of trade-offs by not offering inheritance.
Inheritance is often at risk of sharing more code than necessary. Subclasses
shouldn't always share all characteristics of their parent class but will do so
with inheritance. This can make a program's design less flexible. It also
introduces the possibility of calling methods on subclasses that don't make
sense or that cause errors because the methods don't apply to the subclass. In
addition, some languages will only allow _single inheritance_ (meaning a
subclass can only inherit from one class), further restricting the flexibility
of a program's design.
-->

Rust a choisi un ensemble différent de compromis en ne proposant pas l'héritage. L'héritage risque souvent de partager plus de code que nécessaire. Les sous-classes ne devraient pas toujours partager toutes les caractéristiques de leur classe parente, mais elles le font avec l'héritage. Cela peut rendre la conception d'un programme moins flexible. Cela introduit aussi la possibilité d'appeler des méthodes sur des sous-classes qui n'ont pas de sens ou qui causent des erreurs parce que les méthodes ne s'appliquent pas à la sous-classe. De plus, certains langages ne permettent que l'_héritage simple_ (ce qui signifie qu'une sous-classe ne peut hériter que d'une seule classe), limitant encore davantage la flexibilité de la conception d'un programme.

<!--
For these reasons, Rust takes the different approach of using trait objects
instead of inheritance to achieve polymorphism at runtime. Let's look at how
trait objects work.
-->

Pour ces raisons, Rust adopte l'approche différente d'utiliser des objets trait au lieu de l'héritage pour obtenir le polymorphisme à l'exécution. Voyons comment les objets trait fonctionnent.
