<!--
## Defining an Enum
-->

## Définir une enum

<!--
Where structs give you a way of grouping together related fields and data, like
a `Rectangle` with its `width` and `height`, enums give you a way of saying a
value is one of a possible set of values. For example, we may want to say that
`Rectangle` is one of a set of possible shapes that also includes `Circle` and
`Triangle`. To do this, Rust allows us to encode these possibilities as an enum.
-->

Là où les structures vous permettent de regrouper des champs et des données
liées, comme un `Rectangle` avec sa `width` (largeur) et sa `height` (hauteur),
les enums vous permettent d'exprimer qu'une valeur fait partie d'un ensemble
possible de valeurs. Par exemple, nous pourrions vouloir dire que `Rectangle`
est l'une des formes possibles d'un ensemble qui inclut aussi `Circle` et
`Triangle`. Pour ce faire, Rust nous permet d'encoder ces possibilités sous
forme d'enum.

<!--
Let's look at a situation we might want to express in code and see why enums
are useful and more appropriate than structs in this case. Say we need to work
with IP addresses. Currently, two major standards are used for IP addresses:
version four and version six. Because these are the only possibilities for an
IP address that our program will come across, we can _enumerate_ all possible
variants, which is where enumeration gets its name.
-->

Examinons une situation que nous pourrions vouloir exprimer dans du code et
voyons pourquoi les enums sont utiles et plus appropriées que les structures
dans ce cas. Imaginons que nous devions travailler avec des adresses IP.
Actuellement, deux standards principaux sont utilisés pour les adresses IP :
la version quatre et la version six. Comme ce sont les seules possibilités
d'adresse IP que notre programme rencontrera, nous pouvons _énumérer_ toutes
les variantes possibles, ce qui est l'origine du nom « énumération ».

<!--
Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate because an enum value can only be one of its variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.
-->

Toute adresse IP peut être soit une adresse en version quatre, soit une adresse
en version six, mais pas les deux en même temps. Cette propriété des adresses
IP rend la structure de données enum appropriée, car une valeur d'enum ne peut
être que l'une de ses variantes. Les adresses en version quatre et en version
six restent fondamentalement des adresses IP, elles doivent donc être traitées
comme le même type lorsque le code gère des situations qui s'appliquent à
n'importe quel type d'adresse IP.

<!--
We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`. These are the
variants of the enum:
-->

Nous pouvons exprimer ce concept dans le code en définissant une énumération
`IpAddrKind` et en listant les types possibles d'adresse IP, `V4` et `V6`.
Ce sont les variantes de l'enum :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

<!--
`IpAddrKind` is now a custom data type that we can use elsewhere in our code.
-->

`IpAddrKind` est maintenant un type de données personnalisé que nous pouvons
utiliser ailleurs dans notre code.

<!--
### Enum Values
-->

### Valeurs d'enum

<!--
We can create instances of each of the two variants of `IpAddrKind` like this:
-->

Nous pouvons créer des instances de chacune des deux variantes de `IpAddrKind`
comme ceci :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

<!--
Note that the variants of the enum are namespaced under its identifier, and we
use a double colon to separate the two. This is useful because now both values
`IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`. We
can then, for instance, define a function that takes any `IpAddrKind`:
-->

Remarquez que les variantes de l'enum sont dans l'espace de noms de son
identifiant, et nous utilisons un double deux-points pour les séparer. C'est
utile car maintenant les deux valeurs `IpAddrKind::V4` et `IpAddrKind::V6`
sont du même type : `IpAddrKind`. Nous pouvons alors, par exemple, définir une
fonction qui accepte n'importe quel `IpAddrKind` :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

<!--
And we can call this function with either variant:
-->

Et nous pouvons appeler cette fonction avec l'une ou l'autre variante :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

<!--
Using enums has even more advantages. Thinking more about our IP address type,
at the moment we don't have a way to store the actual IP address _data_; we
only know what _kind_ it is. Given that you just learned about structs in
Chapter 5, you might be tempted to tackle this problem with structs as shown in
Listing 6-1.
-->

Utiliser des enums présente encore plus d'avantages. En réfléchissant davantage
à notre type d'adresse IP, pour l'instant nous n'avons pas de moyen de stocker
les _données_ réelles de l'adresse IP ; nous savons seulement de quel _type_
elle est. Étant donné que vous venez d'apprendre les structures au chapitre 5,
vous pourriez être tenté de résoudre ce problème avec des structures comme le
montre l'encart 6-1.

<Listing number="6-1" caption="Stocker les données et la variante `IpAddrKind` d'une adresse IP en utilisant une `struct`">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

<!--
Here, we've defined a struct `IpAddr` that has two fields: a `kind` field that
is of type `IpAddrKind` (the enum we defined previously) and an `address` field
of type `String`. We have two instances of this struct. The first is `home`,
and it has the value `IpAddrKind::V4` as its `kind` with associated address
data of `127.0.0.1`. The second instance is `loopback`. It has the other
variant of `IpAddrKind` as its `kind` value, `V6`, and has address `::1`
associated with it. We've used a struct to bundle the `kind` and `address`
values together, so now the variant is associated with the value.
-->

Ici, nous avons défini une structure `IpAddr` qui possède deux champs : un
champ `kind` de type `IpAddrKind` (l'enum que nous avons définie précédemment)
et un champ `address` de type `String`. Nous avons deux instances de cette
structure. La première est `home`, et elle a la valeur `IpAddrKind::V4` comme
`kind` avec les données d'adresse associées `127.0.0.1`. La seconde instance
est `loopback`. Elle a l'autre variante de `IpAddrKind` comme valeur de `kind`,
`V6`, et a l'adresse `::1` associée. Nous avons utilisé une structure pour
regrouper les valeurs `kind` et `address`, donc maintenant la variante est
associée à la valeur.

<!--
However, representing the same concept using just an enum is more concise:
Rather than an enum inside a struct, we can put data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:
-->

Cependant, représenter le même concept en utilisant uniquement une enum est plus
concis : plutôt qu'une enum à l'intérieur d'une structure, nous pouvons placer
les données directement dans chaque variante de l'enum. Cette nouvelle
définition de l'enum `IpAddr` indique que les variantes `V4` et `V6` auront
toutes deux des valeurs `String` associées :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

<!--
We attach data to each variant of the enum directly, so there is no need for an
extra struct. Here, it's also easier to see another detail of how enums work:
The name of each enum variant that we define also becomes a function that
constructs an instance of the enum. That is, `IpAddr::V4()` is a function call
that takes a `String` argument and returns an instance of the `IpAddr` type. We
automatically get this constructor function defined as a result of defining the
enum.
-->

Nous attachons les données directement à chaque variante de l'enum, il n'y a
donc pas besoin d'une structure supplémentaire. Ici, il est aussi plus facile
de voir un autre détail du fonctionnement des enums : le nom de chaque variante
d'enum que nous définissons devient aussi une fonction qui construit une
instance de l'enum. C'est-à-dire que `IpAddr::V4()` est un appel de fonction
qui prend un argument `String` et renvoie une instance du type `IpAddr`. Nous
obtenons automatiquement cette fonction constructeur en définissant l'enum.

<!--
There's another advantage to using an enum rather than a struct: Each variant
can have different types and amounts of associated data. Version four IP
addresses will always have four numeric components that will have values
between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but
still express `V6` addresses as one `String` value, we wouldn't be able to with
a struct. Enums handle this case with ease:
-->

Il y a un autre avantage à utiliser une enum plutôt qu'une structure : chaque
variante peut avoir des types et des quantités différentes de données associées.
Les adresses IP en version quatre auront toujours quatre composants numériques
dont les valeurs seront comprises entre 0 et 255. Si nous voulions stocker les
adresses `V4` comme quatre valeurs `u8` tout en exprimant les adresses `V6`
comme une seule valeur `String`, nous ne pourrions pas le faire avec une
structure. Les enums gèrent ce cas avec facilité :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

<!--
We've shown several different ways to define data structures to store version
four and version six IP addresses. However, as it turns out, wanting to store
IP addresses and encode which kind they are is so common that [the standard
library has a definition we can use!][IpAddr] ignore
--> Let's look at how
the standard library defines `IpAddr`. It has the exact enum and variants that
we've defined and used, but it embeds the address data inside the variants in
the form of two different structs, which are defined differently for each
variant:
-->

Nous avons montré plusieurs façons différentes de définir des structures de
données pour stocker les adresses IP en version quatre et en version six.
Cependant, il s'avère que vouloir stocker des adresses IP et encoder leur type
est si courant que [la bibliothèque standard a une définition que nous pouvons
utiliser !][IpAddr]<!--
ignore
--> Voyons comment la bibliothèque standard
définit `IpAddr`. Elle possède exactement l'enum et les variantes que nous avons
définies et utilisées, mais elle intègre les données d'adresse dans les
variantes sous la forme de deux structures différentes, qui sont définies
différemment pour chaque variante :

<!--
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
-->

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

<!--
This code illustrates that you can put any kind of data inside an enum variant:
strings, numeric types, or structs, for example. You can even include another
enum! Also, standard library types are often not much more complicated than
what you might come up with.
-->

Ce code illustre que vous pouvez mettre n'importe quel type de données dans une
variante d'enum : des chaînes de caractères, des types numériques, ou des
structures, par exemple. Vous pouvez même inclure une autre enum ! De plus, les
types de la bibliothèque standard ne sont souvent pas beaucoup plus compliqués
que ce que vous pourriez concevoir vous-même.

<!--
Note that even though the standard library contains a definition for `IpAddr`,
we can still create and use our own definition without conflict because we
haven't brought the standard library's definition into our scope. We'll talk
more about bringing types into scope in Chapter 7.
-->

Notez que même si la bibliothèque standard contient une définition pour
`IpAddr`, nous pouvons toujours créer et utiliser notre propre définition sans
conflit car nous n'avons pas importé la définition de la bibliothèque standard
dans notre portée. Nous parlerons davantage de l'importation de types dans la
portée au chapitre 7.

<!--
Let's look at another example of an enum in Listing 6-2: This one has a wide
variety of types embedded in its variants.
-->

Examinons un autre exemple d'enum dans l'encart 6-2 : celle-ci a une grande
variété de types intégrés dans ses variantes.

<Listing number="6-2" caption="Une enum `Message` dont les variantes stockent chacune des quantités et des types de valeurs différents">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

<!--
This enum has four variants with different types:

- `Quit`: Has no data associated with it at all
- `Move`: Has named fields, like a struct does
- `Write`: Includes a single `String`
- `ChangeColor`: Includes three `i32` values
-->

Cette enum a quatre variantes avec des types différents :

- `Quit` : n'a aucune donnée associée
- `Move` : a des champs nommés, comme une structure
- `Write` : inclut une seule `String`
- `ChangeColor` : inclut trois valeurs `i32`

<!--
Defining an enum with variants such as the ones in Listing 6-2 is similar to
defining different kinds of struct definitions, except the enum doesn't use the
`struct` keyword and all the variants are grouped together under the `Message`
type. The following structs could hold the same data that the preceding enum
variants hold:
-->

Définir une enum avec des variantes telles que celles de l'encart 6-2 est
similaire à définir différents types de structures, sauf que l'enum n'utilise
pas le mot-clé `struct` et que toutes les variantes sont regroupées sous le
type `Message`. Les structures suivantes pourraient contenir les mêmes données
que les variantes de l'enum précédente :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

<!--
But if we used the different structs, each of which has its own type, we
couldn't as easily define a function to take any of these kinds of messages as
we could with the `Message` enum defined in Listing 6-2, which is a single type.
-->

Mais si nous utilisions les différentes structures, chacune ayant son propre
type, nous ne pourrions pas aussi facilement définir une fonction acceptant
n'importe lequel de ces types de messages que nous le pourrions avec l'enum
`Message` définie dans l'encart 6-2, qui est un type unique.

<!--
There is one more similarity between enums and structs: Just as we're able to
define methods on structs using `impl`, we're also able to define methods on
enums. Here's a method named `call` that we could define on our `Message` enum:
-->

Il y a encore une similarité entre les enums et les structures : tout comme
nous pouvons définir des méthodes sur les structures avec `impl`, nous pouvons
aussi définir des méthodes sur les enums. Voici une méthode nommée `call` que
nous pourrions définir sur notre enum `Message` :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

<!--
The body of the method would use `self` to get the value that we called the
method on. In this example, we've created a variable `m` that has the value
`Message::Write(String::from("hello"))`, and that is what `self` will be in the
body of the `call` method when `m.call()` runs.
-->

Le corps de la méthode utiliserait `self` pour obtenir la valeur sur laquelle
nous avons appelé la méthode. Dans cet exemple, nous avons créé une variable
`m` qui a la valeur `Message::Write(String::from("hello"))`, et c'est ce que
`self` sera dans le corps de la méthode `call` lorsque `m.call()` s'exécute.

<!--
Let's look at another enum in the standard library that is very common and
useful: `Option`.
-->

Examinons une autre enum de la bibliothèque standard qui est très courante et
utile : `Option`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="the-option-enum-and-its-advantages-over-null-values"></a>

<!--
### The `Option` Enum
-->

### L'enum `Option`

<!--
This section explores a case study of `Option`, which is another enum defined
by the standard library. The `Option` type encodes the very common scenario in
which a value could be something, or it could be nothing.
-->

Cette section explore une étude de cas sur `Option`, qui est une autre enum
définie par la bibliothèque standard. Le type `Option` encode le scénario très
courant dans lequel une valeur peut être quelque chose, ou peut être rien.

<!--
For example, if you request the first item in a non-empty list, you would get
a value. If you request the first item in an empty list, you would get nothing.
Expressing this concept in terms of the type system means the compiler can
check whether you've handled all the cases you should be handling; this
functionality can prevent bugs that are extremely common in other programming
languages.
-->

Par exemple, si vous demandez le premier élément d'une liste non vide, vous
obtiendrez une valeur. Si vous demandez le premier élément d'une liste vide,
vous n'obtiendrez rien. Exprimer ce concept en termes de système de types
signifie que le compilateur peut vérifier que vous avez géré tous les cas que
vous devriez gérer ; cette fonctionnalité peut prévenir des bogues qui sont
extrêmement courants dans d'autres langages de programmation.

<!--
Programming language design is often thought of in terms of which features you
include, but the features you exclude are important too. Rust doesn't have the
null feature that many other languages have. _Null_ is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.
-->

La conception de langages de programmation est souvent pensée en termes de
fonctionnalités que vous incluez, mais les fonctionnalités que vous excluez
sont importantes aussi. Rust n'a pas la fonctionnalité null que beaucoup
d'autres langages ont. _Null_ est une valeur qui signifie qu'il n'y a pas de
valeur. Dans les langages avec null, les variables peuvent toujours être dans
l'un de ces deux états : null ou non-null.

<!--
In his 2009 presentation "Null References: The Billion Dollar Mistake," Tony
Hoare, the inventor of null, had this to say:
-->

Dans sa présentation de 2009 intitulée « Null References: The Billion Dollar
Mistake » (Les références nulles : l'erreur à un milliard de dollars), Tony
Hoare, l'inventeur de null, a déclaré :

<!--
> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn't resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
-->

> Je l'appelle mon erreur à un milliard de dollars. À cette époque, je concevais
> le premier système de types complet pour les références dans un langage orienté
> objet. Mon objectif était de garantir que toute utilisation des références
> soit absolument sûre, avec une vérification effectuée automatiquement par le
> compilateur. Mais je n'ai pas pu résister à la tentation d'ajouter une
> référence nulle, simplement parce que c'était si facile à implémenter. Cela a
> conduit à d'innombrables erreurs, vulnérabilités et plantages système, qui ont
> probablement causé un milliard de dollars de douleur et de dommages au cours
> des quarante dernières années.

<!--
The problem with null values is that if you try to use a null value as a
not-null value, you'll get an error of some kind. Because this null or not-null
property is pervasive, it's extremely easy to make this kind of error.
-->

Le problème avec les valeurs null est que si vous essayez d'utiliser une valeur
null comme si c'était une valeur non-null, vous obtiendrez une erreur quelconque.
Parce que cette propriété null ou non-null est omniprésente, il est extrêmement
facile de faire ce type d'erreur.

<!--
However, the concept that null is trying to express is still a useful one: A
null is a value that is currently invalid or absent for some reason.
-->

Cependant, le concept que null essaie d'exprimer reste utile : un null est une
valeur qui est actuellement invalide ou absente pour une raison quelconque.

<!--
The problem isn't really with the concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option] ignore
-->
as follows:
-->

Le problème n'est pas vraiment le concept mais l'implémentation particulière.
En tant que tel, Rust n'a pas de null, mais il possède une enum qui peut encoder
le concept d'une valeur étant présente ou absente. Cette enum est `Option<T>`,
et elle est [définie par la bibliothèque standard][option]<!--
ignore
--> comme
suit :

<!--
```rust
enum Option<T> {
    None,
    Some(T),
}
```
-->

```rust
enum Option<T> {
    None,
    Some(T),
}
```

<!--
The `Option<T>` enum is so useful that it's even included in the prelude; you
don't need to bring it into scope explicitly. Its variants are also included in
the prelude: You can use `Some` and `None` directly without the `Option::`
prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and
`None` are still variants of type `Option<T>`.
-->

L'enum `Option<T>` est si utile qu'elle est même incluse dans le prelude ; vous
n'avez pas besoin de l'importer explicitement dans la portée. Ses variantes sont
également incluses dans le prelude : vous pouvez utiliser `Some` et `None`
directement sans le préfixe `Option::`. L'enum `Option<T>` reste simplement
une enum ordinaire, et `Some(T)` et `None` sont toujours des variantes du type
`Option<T>`.

<!--
The `<T>` syntax is a feature of Rust we haven't talked about yet. It's a
generic type parameter, and we'll cover generics in more detail in Chapter 10.
For now, all you need to know is that `<T>` means that the `Some` variant of
the `Option` enum can hold one piece of data of any type, and that each
concrete type that gets used in place of `T` makes the overall `Option<T>` type
a different type. Here are some examples of using `Option` values to hold
number types and char types:
-->

La syntaxe `<T>` est une fonctionnalité de Rust dont nous n'avons pas encore
parlé. C'est un paramètre de type générique, et nous couvrirons les génériques
plus en détail au chapitre 10. Pour l'instant, tout ce que vous devez savoir
est que `<T>` signifie que la variante `Some` de l'enum `Option` peut contenir
un élément de données de n'importe quel type, et que chaque type concret utilisé
à la place de `T` fait du type `Option<T>` global un type différent. Voici
quelques exemples d'utilisation de valeurs `Option` pour contenir des types
numériques et des types char :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

<!--
The type of `some_number` is `Option<i32>`. The type of `some_char` is
`Option<char>`, which is a different type. Rust can infer these types because
we've specified a value inside the `Some` variant. For `absent_number`, Rust
requires us to annotate the overall `Option` type: The compiler can't infer the
type that the corresponding `Some` variant will hold by looking only at a
`None` value. Here, we tell Rust that we mean for `absent_number` to be of type
`Option<i32>`.
-->

Le type de `some_number` est `Option<i32>`. Le type de `some_char` est
`Option<char>`, qui est un type différent. Rust peut inférer ces types parce
que nous avons spécifié une valeur à l'intérieur de la variante `Some`. Pour
`absent_number`, Rust nous demande d'annoter le type `Option` global : le
compilateur ne peut pas inférer le type que la variante `Some` correspondante
contiendra en regardant uniquement une valeur `None`. Ici, nous indiquons à
Rust que nous voulons que `absent_number` soit de type `Option<i32>`.

<!--
When we have a `Some` value, we know that a value is present, and the value is
held within the `Some`. When we have a `None` value, in some sense it means the
same thing as null: We don't have a valid value. So, why is having `Option<T>`
any better than having null?
-->

Quand nous avons une valeur `Some`, nous savons qu'une valeur est présente, et
la valeur est contenue dans le `Some`. Quand nous avons une valeur `None`, dans
un certain sens, cela signifie la même chose que null : nous n'avons pas de
valeur valide. Alors, pourquoi `Option<T>` est-il meilleur que null ?

<!--
In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won't let us use an `Option<T>` value as if it were
definitely a valid value. For example, this code won't compile, because it's
trying to add an `i8` to an `Option<i8>`:
-->

En bref, parce que `Option<T>` et `T` (où `T` peut être n'importe quel type)
sont des types différents, le compilateur ne nous laissera pas utiliser une
valeur `Option<T>` comme si c'était assurément une valeur valide. Par exemple,
ce code ne compilera pas, parce qu'il essaie d'additionner un `i8` et un
`Option<i8>` :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

<!--
If we run this code, we get an error message like this one:
-->

Si nous exécutons ce code, nous obtenons un message d'erreur comme celui-ci :


```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

<!--
Intense! In effect, this error message means that Rust doesn't understand how
to add an `i8` and an `Option<i8>`, because they're different types. When we
have a value of a type like `i8` in Rust, the compiler will ensure that we
always have a valid value. We can proceed confidently without having to check
for null before using that value. Only when we have an `Option<i8>` (or
whatever type of value we're working with) do we have to worry about possibly
not having a value, and the compiler will make sure we handle that case before
using the value.
-->

Intense ! En fait, ce message d'erreur signifie que Rust ne comprend pas
comment additionner un `i8` et un `Option<i8>`, parce que ce sont des types
différents. Quand nous avons une valeur d'un type comme `i8` en Rust, le
compilateur s'assure que nous avons toujours une valeur valide. Nous pouvons
procéder en toute confiance sans avoir à vérifier null avant d'utiliser cette
valeur. Ce n'est que lorsque nous avons un `Option<i8>` (ou quel que soit le
type de valeur avec lequel nous travaillons) que nous devons nous soucier de
la possibilité de ne pas avoir de valeur, et le compilateur s'assurera que nous
gérons ce cas avant d'utiliser la valeur.

<!--
In other words, you have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it. Generally, this helps catch one of the most
common issues with null: assuming that something isn't null when it actually is.
-->

En d'autres termes, vous devez convertir un `Option<T>` en `T` avant de pouvoir
effectuer des opérations de type `T` avec. Généralement, cela aide à détecter
l'un des problèmes les plus courants avec null : supposer que quelque chose
n'est pas null alors qu'en réalité il l'est.

<!--
Eliminating the risk of incorrectly assuming a not-null value helps you be more
confident in your code. In order to have a value that can possibly be null, you
must explicitly opt in by making the type of that value `Option<T>`. Then, when
you use that value, you are required to explicitly handle the case when the
value is null. Everywhere that a value has a type that isn't an `Option<T>`,
you _can_ safely assume that the value isn't null. This was a deliberate design
decision for Rust to limit null's pervasiveness and increase the safety of Rust
code.
-->

Éliminer le risque de supposer incorrectement une valeur non-null vous aide à
avoir plus confiance en votre code. Pour avoir une valeur qui peut possiblement
être nulle, vous devez explicitement opter pour cela en faisant du type de
cette valeur `Option<T>`. Ensuite, quand vous utilisez cette valeur, vous êtes
obligé de gérer explicitement le cas où la valeur est nulle. Partout où une
valeur a un type qui n'est pas un `Option<T>`, vous _pouvez_ supposer en toute
sécurité que la valeur n'est pas nulle. C'était une décision de conception
délibérée de Rust pour limiter l'omniprésence du null et augmenter la sécurité
du code Rust.

<!--
So how do you get the `T` value out of a `Some` variant when you have a value
of type `Option<T>` so that you can use that value? The `Option<T>` enum has a
large number of methods that are useful in a variety of situations; you can
check them out in [its documentation][docs] ignore
-->. Becoming familiar
with the methods on `Option<T>` will be extremely useful in your journey with
Rust.
-->

Alors, comment obtenir la valeur `T` d'une variante `Some` quand vous avez une
valeur de type `Option<T>` pour pouvoir utiliser cette valeur ? L'enum
`Option<T>` possède un grand nombre de méthodes utiles dans diverses
situations ; vous pouvez les consulter dans [sa documentation][docs]<!--
ignore
-->.
Se familiariser avec les méthodes de `Option<T>` sera extrêmement utile dans
votre parcours avec Rust.

<!--
In general, in order to use an `Option<T>` value, you want to have code that
will handle each variant. You want some code that will run only when you have a
`Some(T)` value, and this code is allowed to use the inner `T`. You want some
other code to run only if you have a `None` value, and that code doesn't have a
`T` value available. The `match` expression is a control flow construct that
does just this when used with enums: It will run different code depending on
which variant of the enum it has, and that code can use the data inside the
matching value.
-->

En général, pour utiliser une valeur `Option<T>`, vous voulez avoir du code
qui gère chaque variante. Vous voulez du code qui ne s'exécutera que lorsque
vous avez une valeur `Some(T)`, et ce code est autorisé à utiliser le `T`
interne. Vous voulez qu'un autre code ne s'exécute que si vous avez une valeur
`None`, et ce code n'a pas de valeur `T` disponible. L'expression `match` est
une construction de flux de contrôle qui fait exactement cela lorsqu'elle est
utilisée avec des enums : elle exécutera un code différent selon la variante
de l'enum, et ce code peut utiliser les données à l'intérieur de la valeur
correspondante.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
