<!--
## Generic Data Types
-->

## Les types de données génériques

<!--
We use generics to create definitions for items like function signatures or
structs, which we can then use with many different concrete data types. Let's
first look at how to define functions, structs, enums, and methods using
generics. Then, we'll discuss how generics affect code performance.
-->

Nous utilisons les génériques pour créer des définitions d'éléments comme des
signatures de fonctions ou des structs, que nous pouvons ensuite utiliser avec
de nombreux types de données concrets. Voyons d'abord comment définir des
fonctions, des structs, des énumérations et des méthodes en utilisant les
génériques. Ensuite, nous verrons comment les génériques affectent les
performances du code.

<!--
### In Function Definitions
-->

### Dans les définitions de fonctions

<!--
When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code duplication.
-->

Lorsque nous définissons une fonction qui utilise des génériques, nous plaçons
les génériques dans la signature de la fonction à l'endroit où nous
spécifierions normalement les types de données des paramètres et de la valeur
de retour. Ce faisant, notre code devient plus flexible et offre plus de
fonctionnalités aux appelants de notre fonction tout en évitant la duplication
de code.

<!--
Continuing with our `largest` function, Listing 10-4 shows two functions that
both find the largest value in a slice. We'll then combine these into a single
function that uses generics.
-->

En poursuivant avec notre fonction `largest`, l'encart 10-4 montre deux
fonctions qui trouvent toutes deux la plus grande valeur dans une slice. Nous
les combinerons ensuite en une seule fonction qui utilise des génériques.

<Listing number="10-4" file-name="src/main.rs" caption="Deux fonctions qui ne diffèrent que par leurs noms et les types dans leurs signatures">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

<!--
The `largest_i32` function is the one we extracted in Listing 10-3 that finds
the largest `i32` in a slice. The `largest_char` function finds the largest
`char` in a slice. The function bodies have the same code, so let's eliminate
the duplication by introducing a generic type parameter in a single function.
-->

La fonction `largest_i32` est celle que nous avons extraite dans l'encart 10-3
et qui trouve le plus grand `i32` dans une slice. La fonction `largest_char`
trouve le plus grand `char` dans une slice. Les corps des fonctions contiennent
le même code, alors éliminons la duplication en introduisant un paramètre de
type générique dans une seule fonction.

<!--
To parameterize the types in a new single function, we need to name the type
parameter, just as we do for the value parameters to a function. You can use
any identifier as a type parameter name. But we'll use `T` because, by
convention, type parameter names in Rust are short, often just one letter, and
Rust's type-naming convention is UpperCamelCase. Short for _type_, `T` is the
default choice of most Rust programmers.
-->

Pour paramétrer les types dans une nouvelle fonction unique, nous devons nommer
le paramètre de type, tout comme nous le faisons pour les paramètres de valeur
d'une fonction. Vous pouvez utiliser n'importe quel identifiant comme nom de
paramètre de type. Mais nous utiliserons `T` car, par convention, les noms de
paramètres de type en Rust sont courts, souvent une seule lettre, et la
convention de nommage des types en Rust est l'UpperCamelCase. Abréviation de
_type_, `T` est le choix par défaut de la plupart des programmeurs Rust.

<!--
When we use a parameter in the body of the function, we have to declare the
parameter name in the signature so that the compiler knows what that name
means. Similarly, when we use a type parameter name in a function signature, we
have to declare the type parameter name before we use it. To define the generic
`largest` function, we place type name declarations inside angle brackets,
`<>`, between the name of the function and the parameter list, like this:
-->

Lorsque nous utilisons un paramètre dans le corps de la fonction, nous devons
déclarer le nom du paramètre dans la signature pour que le compilateur sache
ce que ce nom signifie. De même, lorsque nous utilisons un nom de paramètre de
type dans une signature de fonction, nous devons déclarer le nom du paramètre
de type avant de l'utiliser. Pour définir la fonction générique `largest`, nous
plaçons les déclarations de noms de types entre chevrons, `<>`, entre le nom
de la fonction et la liste des paramètres, comme ceci :

<!--
```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```
-->

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

<!--
We read this definition as "The function `largest` is generic over some type
`T`." This function has one parameter named `list`, which is a slice of values
of type `T`. The `largest` function will return a reference to a value of the
same type `T`.
-->

Nous lisons cette définition comme : « La fonction `largest` est générique
sur un certain type `T`. » Cette fonction a un paramètre nommé `list`, qui
est une slice de valeurs de type `T`. La fonction `largest` retournera une
référence vers une valeur du même type `T`.

<!--
Listing 10-5 shows the combined `largest` function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of `i32` values or `char` values. Note that this code won't
compile yet.
-->

L'encart 10-5 montre la définition combinée de la fonction `largest` utilisant
le type de données générique dans sa signature. L'encart montre aussi comment
nous pouvons appeler la fonction avec une slice de valeurs `i32` ou de valeurs
`char`. Notez que ce code ne compilera pas encore.

<Listing number="10-5" file-name="src/main.rs" caption="La fonction `largest` utilisant des paramètres de type générique ; ce code ne compile pas encore">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

<!--
If we compile this code right now, we'll get this error:
-->

Si nous compilons ce code maintenant, nous obtiendrons cette erreur :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

<!--
The help text mentions `std::cmp::PartialOrd`, which is a trait, and we're
going to talk about traits in the next section. For now, know that this error
states that the body of `largest` won't work for all possible types that `T`
could be. Because we want to compare values of type `T` in the body, we can
only use types whose values can be ordered. To enable comparisons, the standard
library has the `std::cmp::PartialOrd` trait that you can implement on types
(see Appendix C for more on this trait). To fix Listing 10-5, we can follow the
help text's suggestion and restrict the types valid for `T` to only those that
implement `PartialOrd`. The listing will then compile, because the standard
library implements `PartialOrd` on both `i32` and `char`.
-->

Le texte d'aide mentionne `std::cmp::PartialOrd`, qui est un trait, et nous
allons parler des traits dans la prochaine section. Pour l'instant, sachez
que cette erreur indique que le corps de `largest` ne fonctionnera pas pour
tous les types possibles que `T` pourrait être. Comme nous voulons comparer
des valeurs de type `T` dans le corps, nous ne pouvons utiliser que des types
dont les valeurs peuvent être ordonnées. Pour permettre les comparaisons, la
bibliothèque standard dispose du trait `std::cmp::PartialOrd` que vous pouvez
implémenter sur les types (voir l'annexe C pour plus d'informations sur ce
trait). Pour corriger l'encart 10-5, nous pouvons suivre la suggestion du
texte d'aide et restreindre les types valides pour `T` à ceux qui
implémentent `PartialOrd`. L'encart compilera alors, car la bibliothèque
standard implémente `PartialOrd` pour `i32` et `char`.

<!--
### In Struct Definitions
-->

### Dans les définitions de structs

<!--
We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. Listing 10-6 defines a `Point<T>` struct to hold
`x` and `y` coordinate values of any type.
-->

Nous pouvons aussi définir des structs qui utilisent un paramètre de type
générique dans un ou plusieurs champs en utilisant la syntaxe `<>`. L'encart
10-6 définit une struct `Point<T>` pour contenir des valeurs de coordonnées
`x` et `y` de n'importe quel type.

<Listing number="10-6" file-name="src/main.rs" caption="Une struct `Point<T>` qui contient des valeurs `x` et `y` de type `T`">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

<!--
The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then, we use the generic type
in the struct definition where we would otherwise specify concrete data types.
-->

La syntaxe pour utiliser les génériques dans les définitions de structs est
similaire à celle utilisée dans les définitions de fonctions. D'abord, nous
déclarons le nom du paramètre de type entre chevrons juste après le nom de la
struct. Ensuite, nous utilisons le type générique dans la définition de la
struct là où nous spécifierions autrement des types de données concrets.

<!--
Note that because we've used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are _both_ that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as in
Listing 10-7, our code won't compile.
-->

Notez que, comme nous n'avons utilisé qu'un seul type générique pour définir
`Point<T>`, cette définition indique que la struct `Point<T>` est générique
sur un certain type `T`, et que les champs `x` et `y` sont _tous les deux_ de
ce même type, quel qu'il soit. Si nous créons une instance de `Point<T>` avec
des valeurs de types différents, comme dans l'encart 10-7, notre code ne
compilera pas.

<Listing number="10-7" file-name="src/main.rs" caption="Les champs `x` et `y` doivent être du même type car ils ont tous les deux le même type de données générique `T`.">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

<!--
In this example, when we assign the integer value `5` to `x`, we let the
compiler know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then, when we specify `4.0` for `y`, which we've defined to have
the same type as `x`, we'll get a type mismatch error like this:
-->

Dans cet exemple, lorsque nous assignons la valeur entière `5` à `x`, nous
informons le compilateur que le type générique `T` sera un entier pour cette
instance de `Point<T>`. Ensuite, lorsque nous spécifions `4.0` pour `y`, que
nous avons défini comme ayant le même type que `x`, nous obtiendrons une
erreur de non-correspondance de type comme celle-ci :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

<!--
To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
Listing 10-8, we change the definition of `Point` to be generic over types `T`
and `U` where `x` is of type `T` and `y` is of type `U`.
-->

Pour définir une struct `Point` où `x` et `y` sont tous les deux génériques
mais pourraient avoir des types différents, nous pouvons utiliser plusieurs
paramètres de type générique. Par exemple, dans l'encart 10-8, nous changeons
la définition de `Point` pour qu'elle soit générique sur les types `T` et `U`,
où `x` est de type `T` et `y` est de type `U`.

<Listing number="10-8" file-name="src/main.rs" caption="Un `Point<T, U>` générique sur deux types pour que `x` et `y` puissent être des valeurs de types différents">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

<!--
Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. If you're finding you need lots of generic types in
your code, it could indicate that your code needs restructuring into smaller
pieces.
-->

Maintenant, toutes les instances de `Point` présentées sont autorisées ! Vous
pouvez utiliser autant de paramètres de type générique que vous le souhaitez
dans une définition, mais en utiliser trop rend votre code difficile à lire.
Si vous constatez que vous avez besoin de beaucoup de types génériques dans
votre code, cela pourrait indiquer que votre code a besoin d'être restructuré
en morceaux plus petits.

<!--
### In Enum Definitions
-->

### Dans les définitions d'énumérations

<!--
As we did with structs, we can define enums to hold generic data types in their
variants. Let's take another look at the `Option<T>` enum that the standard
library provides, which we used in Chapter 6:
-->

Comme nous l'avons fait avec les structs, nous pouvons définir des
énumérations qui contiennent des types de données génériques dans leurs
variantes. Regardons à nouveau l'énumération `Option<T>` que la bibliothèque
standard fournit et que nous avons utilisée au chapitre 6 :

<!--
```rust
enum Option<T> {
    Some(T),
    None,
}
```
-->

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<!--
This definition should now make more sense to you. As you can see, the
`Option<T>` enum is generic over type `T` and has two variants: `Some`, which
holds one value of type `T`, and a `None` variant that doesn't hold any value.
By using the `Option<T>` enum, we can express the abstract concept of an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.
-->

Cette définition devrait maintenant avoir plus de sens pour vous. Comme vous
pouvez le voir, l'énumération `Option<T>` est générique sur le type `T` et
possède deux variantes : `Some`, qui contient une valeur de type `T`, et une
variante `None` qui ne contient aucune valeur. En utilisant l'énumération
`Option<T>`, nous pouvons exprimer le concept abstrait d'une valeur
optionnelle, et comme `Option<T>` est générique, nous pouvons utiliser cette
abstraction quel que soit le type de la valeur optionnelle.

<!--
Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:
-->

Les énumérations peuvent aussi utiliser plusieurs types génériques. La
définition de l'énumération `Result` que nous avons utilisée au chapitre 9
en est un exemple :

<!--
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
-->

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`). In fact, this is what we used to open a
file in Listing 9-3, where `T` was filled in with the type `std::fs::File` when
the file was opened successfully and `E` was filled in with the type
`std::io::Error` when there were problems opening the file.
-->

L'énumération `Result` est générique sur deux types, `T` et `E`, et possède
deux variantes : `Ok`, qui contient une valeur de type `T`, et `Err`, qui
contient une valeur de type `E`. Cette définition rend pratique l'utilisation
de l'énumération `Result` partout où nous avons une opération qui peut réussir
(retourner une valeur d'un certain type `T`) ou échouer (retourner une erreur
d'un certain type `E`). En fait, c'est ce que nous avons utilisé pour ouvrir
un fichier dans l'encart 9-3, où `T` a été rempli avec le type
`std::fs::File` lorsque le fichier a été ouvert avec succès et `E` a été
rempli avec le type `std::io::Error` lorsqu'il y a eu des problèmes à
l'ouverture du fichier.

<!--
When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
avoid duplication by using generic types instead.
-->

Lorsque vous reconnaissez dans votre code des situations avec plusieurs
définitions de structs ou d'énumérations qui ne diffèrent que par les types
des valeurs qu'elles contiennent, vous pouvez éviter la duplication en
utilisant des types génériques à la place.

<!--
### In Method Definitions
-->

### Dans les définitions de méthodes

<!--
We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions too. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6 with a method named `x` implemented on it.
-->

Nous pouvons implémenter des méthodes sur les structs et les énumérations
(comme nous l'avons fait au chapitre 5) et utiliser des types génériques dans
leurs définitions aussi. L'encart 10-9 montre la struct `Point<T>` que nous
avons définie dans l'encart 10-6 avec une méthode nommée `x` implémentée
dessus.

<Listing number="10-9" file-name="src/main.rs" caption="Implémentation d'une méthode nommée `x` sur la struct `Point<T>` qui retournera une référence vers le champ `x` de type `T`">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

<!--
Here, we've defined a method named `x` on `Point<T>` that returns a reference
to the data in the field `x`.
-->

Ici, nous avons défini une méthode nommée `x` sur `Point<T>` qui retourne une
référence vers les données du champ `x`.

<!--
Note that we have to declare `T` just after `impl` so that we can use `T` to
specify that we're implementing methods on the type `Point<T>`. By declaring
`T` as a generic type after `impl`, Rust can identify that the type in the
angle brackets in `Point` is a generic type rather than a concrete type. We
could have chosen a different name for this generic parameter than the generic
parameter declared in the struct definition, but using the same name is
conventional. If you write a method within an `impl` that declares a generic
type, that method will be defined on any instance of the type, no matter what
concrete type ends up substituting for the generic type.
-->

Notez que nous devons déclarer `T` juste après `impl` pour pouvoir utiliser
`T` afin de spécifier que nous implémentons des méthodes sur le type
`Point<T>`. En déclarant `T` comme type générique après `impl`, Rust peut
identifier que le type entre chevrons dans `Point` est un type générique
plutôt qu'un type concret. Nous aurions pu choisir un nom différent pour ce
paramètre générique par rapport au paramètre générique déclaré dans la
définition de la struct, mais utiliser le même nom est conventionnel. Si vous
écrivez une méthode dans un `impl` qui déclare un type générique, cette
méthode sera définie sur n'importe quelle instance du type, quel que soit le
type concret qui finit par se substituer au type générique.

<!--
We can also specify constraints on generic types when defining methods on the
type. We could, for example, implement methods only on `Point<f32>` instances
rather than on `Point<T>` instances with any generic type. In Listing 10-10, we
use the concrete type `f32`, meaning we don't declare any types after `impl`.
-->

Nous pouvons aussi spécifier des contraintes sur les types génériques lors de
la définition de méthodes sur le type. Nous pourrions, par exemple, implémenter
des méthodes uniquement sur les instances de `Point<f32>` plutôt que sur les
instances de `Point<T>` avec n'importe quel type générique. Dans l'encart
10-10, nous utilisons le type concret `f32`, ce qui signifie que nous ne
déclarons aucun type après `impl`.

<Listing number="10-10" file-name="src/main.rs" caption="Un bloc `impl` qui ne s'applique qu'à une struct avec un type concret particulier pour le paramètre de type générique `T`">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

<!--
This code means the type `Point<f32>` will have a `distance_from_origin`
method; other instances of `Point<T>` where `T` is not of type `f32` will not
have this method defined. The method measures how far our point is from the
point at coordinates (0.0, 0.0) and uses mathematical operations that are
available only for floating-point types.
-->

Ce code signifie que le type `Point<f32>` aura une méthode
`distance_from_origin` ; les autres instances de `Point<T>` où `T` n'est pas
de type `f32` n'auront pas cette méthode définie. La méthode mesure la
distance entre notre point et le point aux coordonnées (0.0, 0.0) et utilise
des opérations mathématiques qui ne sont disponibles que pour les types à
virgule flottante.

<!--
Generic type parameters in a struct definition aren't always the same as those
you use in that same struct's method signatures. Listing 10-11 uses the generic
types `X1` and `Y1` for the `Point` struct and `X2` and `Y2` for the `mixup`
method signature to make the example clearer. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `X1`) and the `y`
value from the passed-in `Point` (of type `Y2`).
-->

Les paramètres de type générique dans une définition de struct ne sont pas
toujours les mêmes que ceux que vous utilisez dans les signatures de méthodes
de cette même struct. L'encart 10-11 utilise les types génériques `X1` et `Y1`
pour la struct `Point` et `X2` et `Y2` pour la signature de la méthode `mixup`
afin de rendre l'exemple plus clair. La méthode crée une nouvelle instance de
`Point` avec la valeur `x` du `Point` `self` (de type `X1`) et la valeur `y`
du `Point` passé en argument (de type `Y2`).

<Listing number="10-11" file-name="src/main.rs" caption="Une méthode qui utilise des types génériques différents de ceux de la définition de sa struct">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

<!--
In `main`, we've defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x` because `x` came from `p1`. The `p3` variable
will have a `char` for `y` because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.
-->

Dans `main`, nous avons défini un `Point` qui a un `i32` pour `x` (avec la
valeur `5`) et un `f64` pour `y` (avec la valeur `10.4`). La variable `p2`
est une struct `Point` qui a une slice de chaîne de caractères pour `x` (avec
la valeur `"Hello"`) et un `char` pour `y` (avec la valeur `c`). Appeler
`mixup` sur `p1` avec l'argument `p2` nous donne `p3`, qui aura un `i32` pour
`x` car `x` vient de `p1`. La variable `p3` aura un `char` pour `y` car `y`
vient de `p2`. L'appel de la macro `println!` affichera `p3.x = 5, p3.y = c`.

<!--
The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `X1` and `Y1` are declared after
`impl` because they go with the struct definition. The generic parameters `X2`
and `Y2` are declared after `fn mixup` because they're only relevant to the
method.
-->

Le but de cet exemple est de démontrer une situation dans laquelle certains
paramètres génériques sont déclarés avec `impl` et d'autres sont déclarés avec
la définition de la méthode. Ici, les paramètres génériques `X1` et `Y1` sont
déclarés après `impl` car ils accompagnent la définition de la struct. Les
paramètres génériques `X2` et `Y2` sont déclarés après `fn mixup` car ils ne
sont pertinents que pour la méthode.

<!--
### Performance of Code Using Generics
-->

### Performances du code utilisant les génériques

<!--
You might be wondering whether there is a runtime cost when using generic type
parameters. The good news is that using generic types won't make your program
run any slower than it would with concrete types.
-->

Vous vous demandez peut-être s'il y a un coût à l'exécution lors de
l'utilisation de paramètres de type générique. La bonne nouvelle est que
l'utilisation de types génériques ne ralentira pas votre programme par rapport
à l'utilisation de types concrets.

<!--
Rust accomplishes this by performing monomorphization of the code using
generics at compile time. _Monomorphization_ is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled. In this process, the compiler does the opposite of the steps we used
to create the generic function in Listing 10-5: The compiler looks at all the
places where generic code is called and generates code for the concrete types
the generic code is called with.
-->

Rust accomplit cela en effectuant la monomorphisation du code utilisant les
génériques au moment de la compilation. La _monomorphisation_ est le processus
de transformation du code générique en code spécifique en remplissant les types
concrets utilisés lors de la compilation. Dans ce processus, le compilateur
fait l'inverse des étapes que nous avons utilisées pour créer la fonction
générique dans l'encart 10-5 : le compilateur examine tous les endroits où le
code générique est appelé et génère du code pour les types concrets avec
lesquels le code générique est appelé.

<!--
Let's look at how this works by using the standard library's generic
`Option<T>` enum:
-->

Voyons comment cela fonctionne en utilisant l'énumération générique
`Option<T>` de la bibliothèque standard :

<!--
```rust
let integer = Some(5);
let float = Some(5.0);
```
-->

```rust
let integer = Some(5);
let float = Some(5.0);
```

<!--
When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: One is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into two
definitions specialized to `i32` and `f64`, thereby replacing the generic
definition with the specific ones.
-->

Lorsque Rust compile ce code, il effectue la monomorphisation. Au cours de ce
processus, le compilateur lit les valeurs qui ont été utilisées dans les
instances d'`Option<T>` et identifie deux sortes d'`Option<T>` : l'une est
`i32` et l'autre est `f64`. Ainsi, il développe la définition générique
d'`Option<T>` en deux définitions spécialisées pour `i32` et `f64`, remplaçant
ainsi la définition générique par les définitions spécifiques.

<!--
The monomorphized version of the code looks similar to the following (the
compiler uses different names than what we're using here for illustration):
-->

La version monomorphisée du code ressemble à ce qui suit (le compilateur
utilise des noms différents de ceux que nous utilisons ici à titre
d'illustration) :

<Listing file-name="src/main.rs">

<!--
```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
-->

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

</Listing>

<!--
The generic `Option<T>` is replaced with the specific definitions created by
the compiler. Because Rust compiles generic code into code that specifies the
type in each instance, we pay no runtime cost for using generics. When the code
runs, it performs just as it would if we had duplicated each definition by
hand. The process of monomorphization makes Rust's generics extremely efficient
at runtime.
-->

Le `Option<T>` générique est remplacé par les définitions spécifiques créées
par le compilateur. Comme Rust compile le code générique en code qui spécifie
le type dans chaque instance, nous ne payons aucun coût à l'exécution pour
l'utilisation des génériques. Lorsque le code s'exécute, il se comporte
exactement comme si nous avions dupliqué chaque définition à la main. Le
processus de monomorphisation rend les génériques de Rust extrêmement
efficaces à l'exécution.
