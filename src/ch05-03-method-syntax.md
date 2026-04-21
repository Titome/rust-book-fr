<!--
## Methods
-->

## Les méthodes

<!--
Methods are similar to functions: We declare them with the `fn` keyword and a
name, they can have parameters and a return value, and they contain some code
that's run when the method is called from somewhere else. Unlike functions,
methods are defined within the context of a struct (or an enum or a trait
object, which we cover in [Chapter 6][enums] ignore
--> and [Chapter
18][trait-objects]<!--
ignore
-->, respectively), and their first parameter is
always `self`, which represents the instance of the struct the method is being
called on.
-->

Les méthodes sont similaires aux fonctions : nous les déclarons avec le mot-clé `fn` et un nom, elles peuvent avoir des paramètres et une valeur de retour, et elles contiennent du code qui est exécuté lorsque la méthode est appelée depuis un autre endroit. Contrairement aux fonctions, les méthodes sont définies dans le contexte d'une struct (ou d'un enum ou d'un objet trait, que nous abordons respectivement au [chapitre 6][enums]<!--
ignore
--> et au [chapitre 18][trait-objects]<!--
ignore
-->), et leur premier paramètre est toujours `self`, qui représente l'instance de la struct sur laquelle la méthode est appelée.

<!--
Old headings. Do not remove or links may break.
-->

<a id="defining-methods"></a>

<!--
### Method Syntax
-->

### Syntaxe des méthodes

<!--
Let's change the `area` function that has a `Rectangle` instance as a parameter
and instead make an `area` method defined on the `Rectangle` struct, as shown
in Listing 5-13.
-->

Modifions la fonction `area` qui prend une instance de `Rectangle` comme paramètre et faisons-en plutôt une méthode `area` définie sur la struct `Rectangle`, comme le montre le listing 5-13.

<Listing number="5-13" file-name="src/main.rs" caption="Définition d'une méthode `area` sur la struct `Rectangle`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

<!--
To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block for `Rectangle`. Everything within this `impl` block
will be associated with the `Rectangle` type. Then, we move the `area` function
within the `impl` curly brackets and change the first (and in this case, only)
parameter to be `self` in the signature and everywhere within the body. In
`main`, where we called the `area` function and passed `rect1` as an argument,
we can instead use _method syntax_ to call the `area` method on our `Rectangle`
instance. The method syntax goes after an instance: We add a dot followed by
the method name, parentheses, and any arguments.
-->

Pour définir la fonction dans le contexte de `Rectangle`, nous commençons un bloc `impl` (implémentation) pour `Rectangle`. Tout ce qui se trouve dans ce bloc `impl` sera associé au type `Rectangle`. Ensuite, nous déplaçons la fonction `area` à l'intérieur des accolades du bloc `impl` et changeons le premier (et dans ce cas, le seul) paramètre pour qu'il soit `self` dans la signature et partout dans le corps. Dans `main`, là où nous appelions la fonction `area` en passant `rect1` comme argument, nous pouvons à la place utiliser la _syntaxe de méthode_ pour appeler la méthode `area` sur notre instance de `Rectangle`. La syntaxe de méthode s'écrit après une instance : nous ajoutons un point suivi du nom de la méthode, des parenthèses et des éventuels arguments.

<!--
In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`.
The `&self` is actually short for `self: &Self`. Within an `impl` block, the
type `Self` is an alias for the type that the `impl` block is for. Methods must
have a parameter named `self` of type `Self` for their first parameter, so Rust
lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthand to
indicate that this method borrows the `Self` instance, just as we did in
`rectangle: &Rectangle`. Methods can take ownership of `self`, borrow `self`
immutably, as we've done here, or borrow `self` mutably, just as they can any
other parameter.
-->

Dans la signature de `area`, nous utilisons `&self` au lieu de `rectangle: &Rectangle`. Le `&self` est en fait l'abréviation de `self: &Self`. Dans un bloc `impl`, le type `Self` est un alias pour le type auquel le bloc `impl` est destiné. Les méthodes doivent avoir un paramètre nommé `self` de type `Self` comme premier paramètre, donc Rust vous permet d'abréger cela avec uniquement le nom `self` en première position de paramètre. Notez que nous devons toujours utiliser le `&` devant le raccourci `self` pour indiquer que cette méthode emprunte l'instance de `Self`, tout comme nous l'avons fait avec `rectangle: &Rectangle`. Les méthodes peuvent prendre possession de `self`, emprunter `self` de manière immutable, comme nous l'avons fait ici, ou emprunter `self` de manière mutable, tout comme elles le peuvent pour n'importe quel autre paramètre.

<!--
We chose `&self` here for the same reason we used `&Rectangle` in the function
version: We don't want to take ownership, and we just want to read the data in
the struct, not write to it. If we wanted to change the instance that we've
called the method on as part of what the method does, we'd use `&mut self` as
the first parameter. Having a method that takes ownership of the instance by
using just `self` as the first parameter is rare; this technique is usually
used when the method transforms `self` into something else and you want to
prevent the caller from using the original instance after the transformation.
-->

Nous avons choisi `&self` ici pour la même raison que nous avons utilisé `&Rectangle` dans la version avec fonction : nous ne voulons pas prendre possession, et nous voulons seulement lire les données de la struct, pas les modifier. Si nous voulions modifier l'instance sur laquelle nous avons appelé la méthode dans le cadre de ce que fait la méthode, nous utiliserions `&mut self` comme premier paramètre. Avoir une méthode qui prend possession de l'instance en utilisant simplement `self` comme premier paramètre est rare ; cette technique est généralement utilisée quand la méthode transforme `self` en quelque chose d'autre et que vous voulez empêcher l'appelant d'utiliser l'instance originale après la transformation.

<!--
The main reason for using methods instead of functions, in addition to
providing method syntax and not having to repeat the type of `self` in every
method's signature, is for organization. We've put all the things we can do
with an instance of a type in one `impl` block rather than making future users
of our code search for capabilities of `Rectangle` in various places in the
library we provide.
-->

La raison principale d'utiliser des méthodes plutôt que des fonctions, en plus de fournir la syntaxe de méthode et de ne pas avoir à répéter le type de `self` dans la signature de chaque méthode, est l'organisation. Nous avons regroupé tout ce que nous pouvons faire avec une instance d'un type dans un seul bloc `impl` plutôt que d'obliger les futurs utilisateurs de notre code à chercher les fonctionnalités de `Rectangle` à différents endroits dans la bibliothèque que nous fournissons.

<!--
Note that we can choose to give a method the same name as one of the struct's
fields. For example, we can define a method on `Rectangle` that is also named
`width`:
-->

Notez que nous pouvons choisir de donner à une méthode le même nom qu'un des champs de la struct. Par exemple, nous pouvons définir une méthode sur `Rectangle` qui s'appelle également `width` :


<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

<!--
Here, we're choosing to make the `width` method return `true` if the value in
the instance's `width` field is greater than `0` and `false` if the value is
`0`: We can use a field within a method of the same name for any purpose. In
`main`, when we follow `rect1.width` with parentheses, Rust knows we mean the
method `width`. When we don't use parentheses, Rust knows we mean the field
`width`.
-->

Ici, nous choisissons de faire en sorte que la méthode `width` retourne `true` si la valeur du champ `width` de l'instance est supérieure à `0` et `false` si la valeur est `0` : nous pouvons utiliser un champ dans une méthode du même nom pour n'importe quel usage. Dans `main`, quand nous faisons suivre `rect1.width` de parenthèses, Rust sait que nous parlons de la méthode `width`. Quand nous n'utilisons pas de parenthèses, Rust sait que nous parlons du champ `width`.

<!--
Often, but not always, when we give a method the same name as a field we want
it to only return the value in the field and do nothing else. Methods like this
are called _getters_, and Rust does not implement them automatically for struct
fields as some other languages do. Getters are useful because you can make the
field private but the method public and thus enable read-only access to that
field as part of the type's public API. We will discuss what public and private
are and how to designate a field or method as public or private in [Chapter
7][public] ignore
-->.
-->

Souvent, mais pas toujours, quand nous donnons à une méthode le même nom qu'un champ, nous voulons qu'elle retourne uniquement la valeur du champ sans rien faire d'autre. Les méthodes comme celles-ci sont appelées des _accesseurs_ (getters), et Rust ne les implémente pas automatiquement pour les champs des structs comme le font certains autres langages. Les accesseurs sont utiles car vous pouvez rendre le champ privé mais la méthode publique et ainsi permettre un accès en lecture seule à ce champ dans le cadre de l'API publique du type. Nous verrons ce que sont le public et le privé et comment désigner un champ ou une méthode comme public ou privé au [chapitre 7][public]<!--
ignore
-->.

<!--
> ### Where's the `->` Operator?
>
> In C and C++, two different operators are used for calling methods: You use
> `.` if you're calling a method on the object directly and `->` if you're
> calling the method on a pointer to the object and need to dereference the
> pointer first. In other words, if `object` is a pointer,
> `object->something()` is similar to `(*object).something()`.
>
> Rust doesn't have an equivalent to the `->` operator; instead, Rust has a
> feature called _automatic referencing and dereferencing_. Calling methods is
> one of the few places in Rust with this behavior.
>
> Here's how it works: When you call a method with `object.something()`, Rust
> automatically adds in `&`, `&mut`, or `*` so that `object` matches the
> signature of the method. In other words, the following are the same:
>
>  CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127
-->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> The first one looks much cleaner. This automatic referencing behavior works
> because methods have a clear receiver—the type of `self`. Given the receiver
> and name of a method, Rust can figure out definitively whether the method is
> reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact
> that Rust makes borrowing implicit for method receivers is a big part of
> making ownership ergonomic in practice.
-->

> ### Où est l'opérateur `->` ?
>
> En C et C++, deux opérateurs différents sont utilisés pour appeler des
> méthodes : vous utilisez `.` si vous appelez une méthode directement sur
> l'objet et `->` si vous appelez la méthode sur un pointeur vers l'objet et
> que vous devez d'abord déréférencer le pointeur. Autrement dit, si `object`
> est un pointeur, `object->something()` est similaire à
> `(*object).something()`.
>
> Rust n'a pas d'équivalent à l'opérateur `->` ; à la place, Rust a une
> fonctionnalité appelée _référencement et déréférencement automatiques_.
> L'appel de méthodes est l'un des rares endroits en Rust où ce comportement
> s'applique.
>
> Voici comment cela fonctionne : quand vous appelez une méthode avec
> `object.something()`, Rust ajoute automatiquement `&`, `&mut` ou `*` pour
> que `object` corresponde à la signature de la méthode. Autrement dit, les
> deux lignes suivantes sont équivalentes :
>
> <!--
CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127
-->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> La première forme est bien plus propre. Ce comportement de référencement
> automatique fonctionne parce que les méthodes ont un récepteur clair -- le
> type de `self`. Étant donné le récepteur et le nom d'une méthode, Rust peut
> déterminer de manière définitive si la méthode lit (`&self`), modifie
> (`&mut self`) ou consomme (`self`). Le fait que Rust rende l'emprunt implicite
> pour les récepteurs de méthode est un élément important pour rendre la
> possession ergonomique en pratique.

<!--
### Methods with More Parameters
-->

### Méthodes avec plus de paramètres

<!--
Let's practice using methods by implementing a second method on the `Rectangle`
struct. This time we want an instance of `Rectangle` to take another instance
of `Rectangle` and return `true` if the second `Rectangle` can fit completely
within `self` (the first `Rectangle`); otherwise, it should return `false`.
That is, once we've defined the `can_hold` method, we want to be able to write
the program shown in Listing 5-14.
-->

Pratiquons l'utilisation des méthodes en implémentant une deuxième méthode sur la struct `Rectangle`. Cette fois, nous voulons qu'une instance de `Rectangle` prenne une autre instance de `Rectangle` et retourne `true` si le second `Rectangle` peut tenir entièrement dans `self` (le premier `Rectangle`) ; sinon, elle devrait retourner `false`. C'est-à-dire qu'une fois que nous aurons défini la méthode `can_hold`, nous voulons pouvoir écrire le programme montré dans le listing 5-14.

<Listing number="5-14" file-name="src/main.rs" caption="Utilisation de la méthode `can_hold` pas encore écrite">


```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

<!--
The expected output would look like the following because both dimensions of
`rect2` are smaller than the dimensions of `rect1`, but `rect3` is wider than
`rect1`:
-->

La sortie attendue ressemblerait à ce qui suit car les deux dimensions de `rect2` sont plus petites que les dimensions de `rect1`, mais `rect3` est plus large que `rect1` :

<!--
```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```
-->

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

<!--
We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at the code that calls the method:
`rect1.can_hold(&rect2)` passes in `&rect2`, which is an immutable borrow to
`rect2`, an instance of `Rectangle`. This makes sense because we only need to
read `rect2` (rather than write, which would mean we'd need a mutable borrow),
and we want `main` to retain ownership of `rect2` so that we can use it again
after calling the `can_hold` method. The return value of `can_hold` will be a
Boolean, and the implementation will check whether the width and height of
`self` are greater than the width and height of the other `Rectangle`,
respectively. Let's add the new `can_hold` method to the `impl` block from
Listing 5-13, shown in Listing 5-15.
-->

Nous savons que nous voulons définir une méthode, donc elle sera dans le bloc `impl Rectangle`. Le nom de la méthode sera `can_hold`, et elle prendra un emprunt immutable d'un autre `Rectangle` comme paramètre. Nous pouvons deviner le type du paramètre en regardant le code qui appelle la méthode : `rect1.can_hold(&rect2)` passe `&rect2`, qui est un emprunt immutable de `rect2`, une instance de `Rectangle`. Cela a du sens car nous n'avons besoin que de lire `rect2` (plutôt que d'écrire, ce qui signifierait que nous aurions besoin d'un emprunt mutable), et nous voulons que `main` conserve la possession de `rect2` pour pouvoir l'utiliser à nouveau après l'appel à la méthode `can_hold`. La valeur de retour de `can_hold` sera un booléen, et l'implémentation vérifiera si la largeur et la hauteur de `self` sont respectivement supérieures à la largeur et à la hauteur de l'autre `Rectangle`. Ajoutons la nouvelle méthode `can_hold` au bloc `impl` du listing 5-13, comme le montre le listing 5-15.

<Listing number="5-15" file-name="src/main.rs" caption="Implémentation de la méthode `can_hold` sur `Rectangle` qui prend une autre instance de `Rectangle` comme paramètre">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

<!--
When we run this code with the `main` function in Listing 5-14, we'll get our
desired output. Methods can take multiple parameters that we add to the
signature after the `self` parameter, and those parameters work just like
parameters in functions.
-->

Quand nous exécutons ce code avec la fonction `main` du listing 5-14, nous obtiendrons la sortie souhaitée. Les méthodes peuvent prendre plusieurs paramètres que nous ajoutons à la signature après le paramètre `self`, et ces paramètres fonctionnent exactement comme les paramètres dans les fonctions.

<!--
### Associated Functions
-->

### Fonctions associées

<!--
All functions defined within an `impl` block are called _associated functions_
because they're associated with the type named after the `impl`. We can define
associated functions that don't have `self` as their first parameter (and thus
are not methods) because they don't need an instance of the type to work with.
We've already used one function like this: the `String::from` function that's
defined on the `String` type.
-->

Toutes les fonctions définies dans un bloc `impl` sont appelées _fonctions associées_ car elles sont associées au type nommé après le `impl`. Nous pouvons définir des fonctions associées qui n'ont pas `self` comme premier paramètre (et ne sont donc pas des méthodes) car elles n'ont pas besoin d'une instance du type pour fonctionner. Nous avons déjà utilisé une telle fonction : la fonction `String::from` qui est définie sur le type `String`.

<!--
Associated functions that aren't methods are often used for constructors that
will return a new instance of the struct. These are often called `new`, but
`new` isn't a special name and isn't built into the language. For example, we
could choose to provide an associated function named `square` that would have
one dimension parameter and use that as both width and height, thus making it
easier to create a square `Rectangle` rather than having to specify the same
value twice:
-->

Les fonctions associées qui ne sont pas des méthodes sont souvent utilisées comme constructeurs qui retourneront une nouvelle instance de la struct. Elles sont souvent appelées `new`, mais `new` n'est pas un nom spécial et n'est pas intégré au langage. Par exemple, nous pourrions choisir de fournir une fonction associée nommée `square` qui aurait un seul paramètre de dimension et l'utiliserait à la fois comme largeur et comme hauteur, rendant ainsi plus facile la création d'un `Rectangle` carré plutôt que de devoir spécifier la même valeur deux fois :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

<!--
The `Self` keywords in the return type and in the body of the function are
aliases for the type that appears after the `impl` keyword, which in this case
is `Rectangle`.
-->

Les mots-clés `Self` dans le type de retour et dans le corps de la fonction sont des alias pour le type qui apparaît après le mot-clé `impl`, qui dans ce cas est `Rectangle`.

<!--
To call this associated function, we use the `::` syntax with the struct name;
`let sq = Rectangle::square(3);` is an example. This function is namespaced by
the struct: The `::` syntax is used for both associated functions and
namespaces created by modules. We'll discuss modules in [Chapter
7][modules] ignore
-->.
-->

Pour appeler cette fonction associée, nous utilisons la syntaxe `::` avec le nom de la struct ; `let sq = Rectangle::square(3);` en est un exemple. Cette fonction est dans l'espace de noms de la struct : la syntaxe `::` est utilisée à la fois pour les fonctions associées et les espaces de noms créés par les modules. Nous aborderons les modules au [chapitre 7][modules]<!--
ignore
-->.

<!--
### Multiple `impl` Blocks
-->

### Blocs `impl` multiples

<!--
Each struct is allowed to have multiple `impl` blocks. For example, Listing
5-15 is equivalent to the code shown in Listing 5-16, which has each method in
its own `impl` block.
-->

Chaque struct est autorisée à avoir plusieurs blocs `impl`. Par exemple, le listing 5-15 est équivalent au code montré dans le listing 5-16, qui a chaque méthode dans son propre bloc `impl`.

<Listing number="5-16" caption="Réécriture du listing 5-15 en utilisant des blocs `impl` multiples">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

<!--
There's no reason to separate these methods into multiple `impl` blocks here,
but this is valid syntax. We'll see a case in which multiple `impl` blocks are
useful in Chapter 10, where we discuss generic types and traits.
-->

Il n'y a aucune raison de séparer ces méthodes en plusieurs blocs `impl` ici, mais c'est une syntaxe valide. Nous verrons un cas où les blocs `impl` multiples sont utiles au chapitre 10, où nous aborderons les types génériques et les traits.

<!--
## Summary
-->

## Résumé

<!--
Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.
-->

Les structs vous permettent de créer des types personnalisés qui ont du sens pour votre domaine. En utilisant les structs, vous pouvez garder des éléments de données associés connectés les uns aux autres et nommer chaque élément pour rendre votre code clair. Dans les blocs `impl`, vous pouvez définir des fonctions qui sont associées à votre type, et les méthodes sont un type de fonction associée qui vous permet de spécifier le comportement que les instances de vos structs possèdent.

<!--
But structs aren't the only way you can create custom types: Let's turn to
Rust's enum feature to add another tool to your toolbox.
-->

Mais les structs ne sont pas le seul moyen de créer des types personnalisés : tournons-nous vers la fonctionnalité enum de Rust pour ajouter un autre outil à votre boîte à outils.

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
