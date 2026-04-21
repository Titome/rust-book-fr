<!--
## Defining and Instantiating Structs
-->

## Définir et instancier des structs

<!--
Structs are similar to tuples, discussed in ["The Tuple Type"][tuples]
ignore
--> section, in that both hold multiple related values. Like tuples, the
pieces of a struct can be different types. Unlike with tuples, in a struct
you'll name each piece of data so it's clear what the values mean. Adding these
names means that structs are more flexible than tuples: You don't have to rely
on the order of the data to specify or access the values of an instance.
-->

Les structs sont similaires aux tuples, abordés dans la section [« Le type tuple »][tuples]<!--
ignore
-->, en ce sens que les deux contiennent plusieurs valeurs apparentées. Comme les tuples, les éléments d'une struct peuvent être de types différents. Contrairement aux tuples, dans une struct, vous nommerez chaque élément de données pour que la signification des valeurs soit claire. L'ajout de ces noms signifie que les structs sont plus flexibles que les tuples : vous n'avez pas besoin de vous fier à l'ordre des données pour spécifier ou accéder aux valeurs d'une instance.

<!--
To define a struct, we enter the keyword `struct` and name the entire struct. A
struct's name should describe the significance of the pieces of data being
grouped together. Then, inside curly brackets, we define the names and types of
the pieces of data, which we call _fields_. For example, Listing 5-1 shows a
struct that stores information about a user account.
-->

Pour définir une struct, nous utilisons le mot-clé `struct` et nommons l'ensemble de la struct. Le nom d'une struct devrait décrire la signification des éléments de données regroupés. Ensuite, à l'intérieur d'accolades, nous définissons les noms et les types des éléments de données, que nous appelons des _champs_. Par exemple, le listing 5-1 montre une struct qui stocke des informations sur un compte utilisateur.

<Listing number="5-1" file-name="src/main.rs" caption="Définition d'une struct `User`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

<!--
To use a struct after we've defined it, we create an _instance_ of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing _`key:
value`_ pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields. We don't have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user as shown in Listing 5-2.
-->

Pour utiliser une struct après l'avoir définie, nous créons une _instance_ de cette struct en spécifiant des valeurs concrètes pour chacun des champs. Nous créons une instance en indiquant le nom de la struct puis en ajoutant des accolades contenant des paires _`clé: valeur`_, où les clés sont les noms des champs et les valeurs sont les données que nous voulons stocker dans ces champs. Nous n'avons pas besoin de spécifier les champs dans le même ordre que celui dans lequel nous les avons déclarés dans la struct. Autrement dit, la définition de la struct est comme un modèle général pour le type, et les instances remplissent ce modèle avec des données particulières pour créer des valeurs du type. Par exemple, nous pouvons déclarer un utilisateur particulier comme le montre le listing 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="Création d'une instance de la struct `User`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

<!--
To get a specific value from a struct, we use dot notation. For example, to
access this user's email address, we use `user1.email`. If the instance is
mutable, we can change a value by using the dot notation and assigning into a
particular field. Listing 5-3 shows how to change the value in the `email`
field of a mutable `User` instance.
-->

Pour obtenir une valeur spécifique d'une struct, nous utilisons la notation par point. Par exemple, pour accéder à l'adresse e-mail de cet utilisateur, nous utilisons `user1.email`. Si l'instance est mutable, nous pouvons modifier une valeur en utilisant la notation par point et en assignant une valeur à un champ particulier. Le listing 5-3 montre comment modifier la valeur du champ `email` d'une instance mutable de `User`.

<Listing number="5-3" file-name="src/main.rs" caption="Modification de la valeur du champ `email` d'une instance de `User`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

<!--
Note that the entire instance must be mutable; Rust doesn't allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.
-->

Notez que l'instance entière doit être mutable ; Rust ne nous permet pas de marquer seulement certains champs comme mutables. Comme pour toute expression, nous pouvons construire une nouvelle instance de la struct comme dernière expression dans le corps de la fonction pour retourner implicitement cette nouvelle instance.

<!--
Listing 5-4 shows a `build_user` function that returns a `User` instance with
the given email and username. The `active` field gets the value `true`, and the
`sign_in_count` gets a value of `1`.
-->

Le listing 5-4 montre une fonction `build_user` qui retourne une instance de `User` avec l'e-mail et le nom d'utilisateur donnés. Le champ `active` reçoit la valeur `true`, et `sign_in_count` reçoit la valeur `1`.

<Listing number="5-4" file-name="src/main.rs" caption="Une fonction `build_user` qui prend un e-mail et un nom d'utilisateur et retourne une instance de `User`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

<!--
It makes sense to name the function parameters with the same name as the struct
fields, but having to repeat the `email` and `username` field names and
variables is a bit tedious. If the struct had more fields, repeating each name
would get even more annoying. Luckily, there's a convenient shorthand!
-->

Il est logique de nommer les paramètres de la fonction avec le même nom que les champs de la struct, mais devoir répéter les noms des champs `email` et `username` et les variables est un peu fastidieux. Si la struct avait plus de champs, répéter chaque nom deviendrait encore plus pénible. Heureusement, il existe un raccourci pratique !

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

<!--
### Using the Field Init Shorthand
-->

### Utiliser le raccourci d'initialisation de champ

<!--
Because the parameter names and the struct field names are exactly the same in
Listing 5-4, we can use the _field init shorthand_ syntax to rewrite
`build_user` so that it behaves exactly the same but doesn't have the
repetition of `username` and `email`, as shown in Listing 5-5.
-->

Comme les noms des paramètres et les noms des champs de la struct sont exactement les mêmes dans le listing 5-4, nous pouvons utiliser la syntaxe du _raccourci d'initialisation de champ_ pour réécrire `build_user` de manière à ce qu'elle se comporte exactement de la même façon mais sans la répétition de `username` et `email`, comme le montre le listing 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="Une fonction `build_user` qui utilise le raccourci d'initialisation de champ car les paramètres `username` et `email` ont le même nom que les champs de la struct">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

<!--
Here, we're creating a new instance of the `User` struct, which has a field
named `email`. We want to set the `email` field's value to the value in the
`email` parameter of the `build_user` function. Because the `email` field and
the `email` parameter have the same name, we only need to write `email` rather
than `email: email`.
-->

Ici, nous créons une nouvelle instance de la struct `User`, qui a un champ nommé `email`. Nous voulons définir la valeur du champ `email` à la valeur du paramètre `email` de la fonction `build_user`. Comme le champ `email` et le paramètre `email` portent le même nom, nous n'avons besoin d'écrire que `email` plutôt que `email: email`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

<!--
### Creating Instances with Struct Update Syntax
-->

### Créer des instances avec la syntaxe de mise à jour de struct

<!--
It's often useful to create a new instance of a struct that includes most of
the values from another instance of the same type, but changes some of them.
You can do this using struct update syntax.
-->

Il est souvent utile de créer une nouvelle instance d'une struct qui inclut la plupart des valeurs d'une autre instance du même type, mais en modifie certaines. Vous pouvez le faire en utilisant la syntaxe de mise à jour de struct.

<!--
First, in Listing 5-6 we show how to create a new `User` instance in `user2` in
the regular way, without the update syntax. We set a new value for `email` but
otherwise use the same values from `user1` that we created in Listing 5-2.
-->

D'abord, dans le listing 5-6, nous montrons comment créer une nouvelle instance de `User` dans `user2` de manière classique, sans la syntaxe de mise à jour. Nous définissons une nouvelle valeur pour `email` mais utilisons par ailleurs les mêmes valeurs de `user1` que nous avons créé dans le listing 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="Création d'une nouvelle instance de `User` en utilisant toutes les valeurs de `user1` sauf une">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

<!--
Using struct update syntax, we can achieve the same effect with less code, as
shown in Listing 5-7. The syntax `..` specifies that the remaining fields not
explicitly set should have the same value as the fields in the given instance.
-->

En utilisant la syntaxe de mise à jour de struct, nous pouvons obtenir le même effet avec moins de code, comme le montre le listing 5-7. La syntaxe `..` spécifie que les champs restants non explicitement définis doivent avoir la même valeur que les champs de l'instance donnée.

<Listing number="5-7" file-name="src/main.rs" caption="Utilisation de la syntaxe de mise à jour de struct pour définir une nouvelle valeur d'`email` pour une instance de `User` tout en utilisant le reste des valeurs de `user1`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

<!--
The code in Listing 5-7 also creates an instance in `user2` that has a
different value for `email` but has the same values for the `username`,
`active`, and `sign_in_count` fields from `user1`. The `..user1` must come last
to specify that any remaining fields should get their values from the
corresponding fields in `user1`, but we can choose to specify values for as
many fields as we want in any order, regardless of the order of the fields in
the struct's definition.
-->

Le code du listing 5-7 crée également une instance dans `user2` qui a une valeur différente pour `email` mais qui conserve les mêmes valeurs pour les champs `username`, `active` et `sign_in_count` de `user1`. Le `..user1` doit apparaître en dernier pour spécifier que tous les champs restants doivent obtenir leurs valeurs des champs correspondants dans `user1`, mais nous pouvons choisir de spécifier des valeurs pour autant de champs que nous le souhaitons, dans n'importe quel ordre, indépendamment de l'ordre des champs dans la définition de la struct.

<!--
Note that the struct update syntax uses `=` like an assignment; this is because
it moves the data, just as we saw in the ["Variables and Data Interacting with
Move"][move] ignore
--> section. In this example, we can no longer use
`user1` after creating `user2` because the `String` in the `username` field of
`user1` was moved into `user2`. If we had given `user2` new `String` values for
both `email` and `username`, and thus only used the `active` and `sign_in_count`
values from `user1`, then `user1` would still be valid after creating `user2`.
Both `active` and `sign_in_count` are types that implement the `Copy` trait, so
the behavior we discussed in the ["Stack-Only Data: Copy"][copy]<!--
ignore
-->
section would apply. We can also still use `user1.email` in this example,
because its value was not moved out of `user1`.
-->

Notez que la syntaxe de mise à jour de struct utilise `=` comme une assignation ; c'est parce qu'elle déplace les données, tout comme nous l'avons vu dans la section [« Les variables et les données interagissant avec le déplacement »][move]<!--
ignore
-->. Dans cet exemple, nous ne pouvons plus utiliser `user1` après avoir créé `user2` parce que la `String` du champ `username` de `user1` a été déplacée dans `user2`. Si nous avions donné à `user2` de nouvelles valeurs `String` pour `email` et `username`, et donc n'avions utilisé que les valeurs `active` et `sign_in_count` de `user1`, alors `user1` serait toujours valide après la création de `user2`. `active` et `sign_in_count` sont tous deux des types qui implémentent le trait `Copy`, donc le comportement que nous avons abordé dans la section [« Données uniquement sur la pile : Copy »][copy]<!--
ignore
--> s'appliquerait. Nous pouvons également toujours utiliser `user1.email` dans cet exemple, car sa valeur n'a pas été déplacée hors de `user1`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

<!--
### Creating Different Types with Tuple Structs
-->

### Créer des types différents avec les structs tuples

<!--
Rust also supports structs that look similar to tuples, called _tuple structs_.
Tuple structs have the added meaning the struct name provides but don't have
names associated with their fields; rather, they just have the types of the
fields. Tuple structs are useful when you want to give the whole tuple a name
and make the tuple a different type from other tuples, and when naming each
field as in a regular struct would be verbose or redundant.
-->

Rust prend également en charge des structs qui ressemblent à des tuples, appelées _structs tuples_. Les structs tuples ont la signification supplémentaire que le nom de la struct apporte, mais n'ont pas de noms associés à leurs champs ; elles n'ont que les types des champs. Les structs tuples sont utiles lorsque vous voulez donner un nom à l'ensemble du tuple et faire du tuple un type différent des autres tuples, et quand nommer chaque champ comme dans une struct classique serait verbeux ou redondant.

<!--
To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple. For example, here we define and use two
tuple structs named `Color` and `Point`:
-->

Pour définir une struct tuple, commencez par le mot-clé `struct` et le nom de la struct, suivis des types dans le tuple. Par exemple, ici nous définissons et utilisons deux structs tuples nommées `Color` et `Point` :


<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

<!--
Note that the `black` and `origin` values are different types because they're
instances of different tuple structs. Each struct you define is its own type,
even though the fields within the struct might have the same types. For
example, a function that takes a parameter of type `Color` cannot take a
`Point` as an argument, even though both types are made up of three `i32`
values. Otherwise, tuple struct instances are similar to tuples in that you can
destructure them into their individual pieces, and you can use a `.` followed
by the index to access an individual value. Unlike tuples, tuple structs
require you to name the type of the struct when you destructure them. For
example, we would write `let Point(x, y, z) = origin;` to destructure the
values in the `origin` point into variables named `x`, `y`, and `z`.
-->

Notez que les valeurs `black` et `origin` sont de types différents car ce sont des instances de structs tuples différentes. Chaque struct que vous définissez est son propre type, même si les champs de la struct peuvent avoir les mêmes types. Par exemple, une fonction qui prend un paramètre de type `Color` ne peut pas prendre un `Point` comme argument, même si les deux types sont composés de trois valeurs `i32`. Par ailleurs, les instances de structs tuples sont similaires aux tuples en ce que vous pouvez les déstructurer en leurs éléments individuels, et vous pouvez utiliser un `.` suivi de l'index pour accéder à une valeur individuelle. Contrairement aux tuples, les structs tuples exigent que vous nommiez le type de la struct lorsque vous les déstructurez. Par exemple, nous écririons `let Point(x, y, z) = origin;` pour déstructurer les valeurs du point `origin` dans des variables nommées `x`, `y` et `z`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="unit-like-structs-without-any-fields"></a>

<!--
### Defining Unit-Like Structs
-->

### Définir des structs unitaires

<!--
You can also define structs that don't have any fields! These are called
_unit-like structs_ because they behave similarly to `()`, the unit type that
we mentioned in ["The Tuple Type"][tuples] ignore
--> section. Unit-like
structs can be useful when you need to implement a trait on some type but don't
have any data that you want to store in the type itself. We'll discuss traits
in Chapter 10. Here's an example of declaring and instantiating a unit struct
named `AlwaysEqual`:
-->

Vous pouvez également définir des structs qui n'ont aucun champ ! Celles-ci sont appelées _structs unitaires_ car elles se comportent de manière similaire à `()`, le type unitaire que nous avons mentionné dans la section [« Le type tuple »][tuples]<!--
ignore
-->. Les structs unitaires peuvent être utiles lorsque vous devez implémenter un trait sur un type mais que vous n'avez aucune donnée à stocker dans le type lui-même. Nous aborderons les traits au chapitre 10. Voici un exemple de déclaration et d'instanciation d'une struct unitaire nommée `AlwaysEqual` :


<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

<!--
To define `AlwaysEqual`, we use the `struct` keyword, the name we want, and
then a semicolon. No need for curly brackets or parentheses! Then, we can get
an instance of `AlwaysEqual` in the `subject` variable in a similar way: using
the name we defined, without any curly brackets or parentheses. Imagine that
later we'll implement behavior for this type such that every instance of
`AlwaysEqual` is always equal to every instance of any other type, perhaps to
have a known result for testing purposes. We wouldn't need any data to
implement that behavior! You'll see in Chapter 10 how to define traits and
implement them on any type, including unit-like structs.
-->

Pour définir `AlwaysEqual`, nous utilisons le mot-clé `struct`, le nom que nous souhaitons, puis un point-virgule. Pas besoin d'accolades ni de parenthèses ! Ensuite, nous pouvons obtenir une instance d'`AlwaysEqual` dans la variable `subject` de manière similaire : en utilisant le nom que nous avons défini, sans accolades ni parenthèses. Imaginez que plus tard nous implémenterons un comportement pour ce type de sorte que chaque instance d'`AlwaysEqual` soit toujours égale à chaque instance de tout autre type, peut-être pour avoir un résultat connu à des fins de test. Nous n'aurions besoin d'aucune donnée pour implémenter ce comportement ! Vous verrez au chapitre 10 comment définir des traits et les implémenter sur n'importe quel type, y compris les structs unitaires.

<!--
> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want each instance of this struct to own all of its data and for
> that data to be valid for as long as the entire struct is valid.
>
> It's also possible for structs to store references to data owned by something
> else, but to do so requires the use of _lifetimes_, a Rust feature that we'll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let's say you try to store a reference
> in a struct without specifying lifetimes, like the following in
> *src/main.rs*; this won't work:
>
> 
>
>  CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127
-->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> In Chapter 10, we'll discuss how to fix these errors so that you can store
> references in structs, but for now, we'll fix errors like these using owned
> types like `String` instead of references like `&str`.
-->

> ### Possession des données d'une struct
>
> Dans la définition de la struct `User` du listing 5-1, nous avons utilisé le
> type possédé `String` plutôt que le type de slice de chaîne `&str`. C'est un
> choix délibéré car nous voulons que chaque instance de cette struct possède
> toutes ses données et que ces données soient valides aussi longtemps que la
> struct entière est valide.
>
> Il est également possible pour les structs de stocker des références vers des
> données possédées par autre chose, mais cela nécessite l'utilisation de
> _durées de vie_ (lifetimes), une fonctionnalité de Rust que nous aborderons au
> chapitre 10. Les durées de vie garantissent que les données référencées par une
> struct sont valides aussi longtemps que la struct l'est. Supposons que vous
> essayiez de stocker une référence dans une struct sans spécifier de durées de
> vie, comme dans le code suivant dans *src/main.rs* ; cela ne fonctionnera pas :
>
> <Listing file-name="src/main.rs">
>
> <!--
CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127
-->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> Le compilateur se plaindra qu'il a besoin de spécificateurs de durée de vie :
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> Au chapitre 10, nous verrons comment corriger ces erreurs pour pouvoir stocker
> des références dans des structs, mais pour l'instant, nous corrigerons ce type
> d'erreurs en utilisant des types possédés comme `String` au lieu de références
> comme `&str`.

<!--
manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line
-->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
