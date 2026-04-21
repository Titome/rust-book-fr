<!--
## Using `Box<T>` to Point to Data on the Heap
-->

## Utiliser `Box<T>` pour pointer vers des données sur le tas

<!--
The most straightforward smart pointer is a box, whose type is written
`Box<T>`. _Boxes_ allow you to store data on the heap rather than the stack.
What remains on the stack is the pointer to the heap data. Refer to Chapter 4
to review the difference between the stack and the heap.
-->

Le pointeur intelligent le plus simple est une boîte (box), dont le type s'écrit `Box<T>`. Les _boîtes_ vous permettent de stocker des données sur le tas plutôt que sur la pile. Ce qui reste sur la pile est le pointeur vers les données du tas. Reportez-vous au chapitre 4 pour revoir la différence entre la pile et le tas.

<!--
Boxes don't have performance overhead, other than storing their data on the
heap instead of on the stack. But they don't have many extra capabilities
either. You'll use them most often in these situations:

- When you have a type whose size can't be known at compile time, and you want
  to use a value of that type in a context that requires an exact size
- When you have a large amount of data, and you want to transfer ownership but
  ensure that the data won't be copied when you do so
- When you want to own a value, and you care only that it's a type that
  implements a particular trait rather than being of a specific type
-->

Les boîtes n'ont pas de surcoût en performance, hormis le fait de stocker leurs données sur le tas au lieu de la pile. Mais elles n'ont pas non plus beaucoup de capacités supplémentaires. Vous les utiliserez le plus souvent dans ces situations :

- Quand vous avez un type dont la taille ne peut pas être connue à la compilation, et que vous voulez utiliser une valeur de ce type dans un contexte qui nécessite une taille exacte
- Quand vous avez une grande quantité de données, et que vous voulez transférer la possession tout en vous assurant que les données ne seront pas copiées
- Quand vous voulez posséder une valeur, et que vous vous souciez uniquement du fait qu'elle implémente un trait particulier plutôt que d'être d'un type spécifique

<!--
We'll demonstrate the first situation in ["Enabling Recursive Types with
Boxes"](#enabling-recursive-types-with-boxes) ignore
-->. In the second
case, transferring ownership of a large amount of data can take a long time
because the data is copied around on the stack. To improve performance in this
situation, we can store the large amount of data on the heap in a box. Then,
only the small amount of pointer data is copied around on the stack, while the
data it references stays in one place on the heap. The third case is known as a
_trait object_, and ["Using Trait Objects to Abstract over Shared
Behavior"][trait-objects]<!--
ignore
--> in Chapter 18 is devoted to that
topic. So, what you learn here you'll apply again in that section!
-->

Nous démontrerons la première situation dans ["Permettre les types récursifs avec les boîtes"](#enabling-recursive-types-with-boxes)<!--
ignore
-->. Dans le deuxième cas, transférer la possession d'une grande quantité de données peut prendre beaucoup de temps car les données sont copiées sur la pile. Pour améliorer les performances dans cette situation, nous pouvons stocker la grande quantité de données sur le tas dans une boîte. Ainsi, seule la petite quantité de données du pointeur est copiée sur la pile, tandis que les données référencées restent au même endroit sur le tas. Le troisième cas est connu sous le nom d'_objet trait_, et ["Utiliser les objets trait pour abstraire des comportements communs"][trait-objects]<!--
ignore
--> au chapitre 18 est consacré à ce sujet. Donc, ce que vous apprenez ici sera réutilisé dans cette section !

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-boxt-to-store-data-on-the-heap"></a>

<!--
### Storing Data on the Heap
-->

### Stocker des données sur le tas

<!--
Before we discuss the heap storage use case for `Box<T>`, we'll cover the
syntax and how to interact with values stored within a `Box<T>`.
-->

Avant de discuter du cas d'utilisation de `Box<T>` pour le stockage sur le tas, nous couvrirons la syntaxe et comment interagir avec les valeurs stockées dans un `Box<T>`.

<!--
Listing 15-1 shows how to use a box to store an `i32` value on the heap.
-->

L'encart 15-1 montre comment utiliser une boîte pour stocker une valeur `i32` sur le tas.

<Listing number="15-1" file-name="src/main.rs" caption="Stocker une valeur `i32` sur le tas en utilisant une boîte">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

<!--
We define the variable `b` to have the value of a `Box` that points to the
value `5`, which is allocated on the heap. This program will print `b = 5`; in
this case, we can access the data in the box similarly to how we would if this
data were on the stack. Just like any owned value, when a box goes out of
scope, as `b` does at the end of `main`, it will be deallocated. The
deallocation happens both for the box (stored on the stack) and the data it
points to (stored on the heap).
-->

Nous définissons la variable `b` avec la valeur d'un `Box` qui pointe vers la valeur `5`, qui est allouée sur le tas. Ce programme affichera `b = 5` ; dans ce cas, nous pouvons accéder aux données dans la boîte de manière similaire à ce que nous ferions si ces données étaient sur la pile. Comme toute valeur possédée, quand une boîte sort de la portée, comme `b` le fait à la fin de `main`, elle sera désallouée. La désallocation se produit à la fois pour la boîte (stockée sur la pile) et les données vers lesquelles elle pointe (stockées sur le tas).

<!--
Putting a single value on the heap isn't very useful, so you won't use boxes by
themselves in this way very often. Having values like a single `i32` on the
stack, where they're stored by default, is more appropriate in the majority of
situations. Let's look at a case where boxes allow us to define types that we
wouldn't be allowed to define if we didn't have boxes.
-->

Placer une seule valeur sur le tas n'est pas très utile, donc vous n'utiliserez pas les boîtes seules de cette manière très souvent. Avoir des valeurs comme un seul `i32` sur la pile, où elles sont stockées par défaut, est plus approprié dans la majorité des situations. Examinons un cas où les boîtes nous permettent de définir des types que nous ne pourrions pas définir sans elles.

<!--
### Enabling Recursive Types with Boxes
-->

### Permettre les types récursifs avec les boîtes

<!--
A value of a _recursive type_ can have another value of the same type as part of
itself. Recursive types pose an issue because Rust needs to know at compile time
how much space a type takes up. However, the nesting of values of recursive
types could theoretically continue infinitely, so Rust can't know how much space
the value needs. Because boxes have a known size, we can enable recursive types
by inserting a box in the recursive type definition.
-->

Une valeur d'un _type récursif_ peut contenir une autre valeur du même type comme partie d'elle-même. Les types récursifs posent un problème car Rust a besoin de savoir à la compilation combien d'espace un type occupe. Cependant, l'imbrication de valeurs de types récursifs pourrait théoriquement continuer à l'infini, donc Rust ne peut pas savoir combien d'espace la valeur nécessite. Comme les boîtes ont une taille connue, nous pouvons permettre les types récursifs en insérant une boîte dans la définition du type récursif.

<!--
As an example of a recursive type, let's explore the cons list. This is a data
type commonly found in functional programming languages. The cons list type
we'll define is straightforward except for the recursion; therefore, the
concepts in the example we'll work with will be useful anytime you get into
more complex situations involving recursive types.
-->

Comme exemple de type récursif, explorons la liste cons. C'est un type de données couramment trouvé dans les langages de programmation fonctionnelle. Le type de liste cons que nous définirons est simple à l'exception de la récursion ; par conséquent, les concepts de l'exemple avec lequel nous travaillerons seront utiles chaque fois que vous rencontrerez des situations plus complexes impliquant des types récursifs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="more-information-about-the-cons-list"></a>

<!--
#### Understanding the Cons List
-->

#### Comprendre la liste cons

<!--
A _cons list_ is a data structure that comes from the Lisp programming language
and its dialects, is made up of nested pairs, and is the Lisp version of a
linked list. Its name comes from the `cons` function (short for _construct
function_) in Lisp that constructs a new pair from its two arguments. By
calling `cons` on a pair consisting of a value and another pair, we can
construct cons lists made up of recursive pairs.
-->

Une _liste cons_ est une structure de données qui provient du langage de programmation Lisp et de ses dialectes, elle est composée de paires imbriquées, et c'est la version Lisp d'une liste chaînée. Son nom vient de la fonction `cons` (abréviation de _construct function_, fonction de construction) en Lisp qui construit une nouvelle paire à partir de ses deux arguments. En appelant `cons` sur une paire composée d'une valeur et d'une autre paire, nous pouvons construire des listes cons composées de paires récursives.

<!--
For example, here's a pseudocode representation of a cons list containing the
list `1, 2, 3` with each pair in parentheses:
-->

Par exemple, voici une représentation en pseudo-code d'une liste cons contenant la liste `1, 2, 3` avec chaque paire entre parenthèses :

<!--
```text
(1, (2, (3, Nil)))
```
-->

```text
(1, (2, (3, Nil)))
```

<!--
Each item in a cons list contains two elements: the value of the current item
and of the next item. The last item in the list contains only a value called
`Nil` without a next item. A cons list is produced by recursively calling the
`cons` function. The canonical name to denote the base case of the recursion is
`Nil`. Note that this is not the same as the "null" or "nil" concept discussed
in Chapter 6, which is an invalid or absent value.
-->

Chaque élément d'une liste cons contient deux éléments : la valeur de l'élément courant et celle de l'élément suivant. Le dernier élément de la liste ne contient qu'une valeur appelée `Nil` sans élément suivant. Une liste cons est produite en appelant récursivement la fonction `cons`. Le nom canonique pour désigner le cas de base de la récursion est `Nil`. Notez que ce n'est pas la même chose que le concept de "null" ou "nil" abordé au chapitre 6, qui est une valeur invalide ou absente.

<!--
The cons list isn't a commonly used data structure in Rust. Most of the time
when you have a list of items in Rust, `Vec<T>` is a better choice to use.
Other, more complex recursive data types _are_ useful in various situations,
but by starting with the cons list in this chapter, we can explore how boxes
let us define a recursive data type without much distraction.
-->

La liste cons n'est pas une structure de données couramment utilisée en Rust. La plupart du temps, quand vous avez une liste d'éléments en Rust, `Vec<T>` est un meilleur choix. D'autres types de données récursifs plus complexes _sont_ utiles dans diverses situations, mais en commençant par la liste cons dans ce chapitre, nous pouvons explorer comment les boîtes nous permettent de définir un type de données récursif sans trop de distraction.

<!--
Listing 15-2 contains an enum definition for a cons list. Note that this code
won't compile yet, because the `List` type doesn't have a known size, which
we'll demonstrate.
-->

L'encart 15-2 contient une définition d'enum pour une liste cons. Notez que ce code ne compilera pas encore, car le type `List` n'a pas de taille connue, ce que nous allons démontrer.

<Listing number="15-2" file-name="src/main.rs" caption="La première tentative de définition d'une enum pour représenter une structure de données de liste cons de valeurs `i32`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

<!--
> Note: We're implementing a cons list that holds only `i32` values for the
> purposes of this example. We could have implemented it using generics, as we
> discussed in Chapter 10, to define a cons list type that could store values of
> any type.
-->

> Remarque : nous implémentons une liste cons qui ne contient que des valeurs `i32` pour les besoins de cet exemple. Nous aurions pu l'implémenter avec des génériques, comme nous l'avons vu au chapitre 10, pour définir un type de liste cons pouvant stocker des valeurs de n'importe quel type.

<!--
Using the `List` type to store the list `1, 2, 3` would look like the code in
Listing 15-3.
-->

Utiliser le type `List` pour stocker la liste `1, 2, 3` ressemblerait au code de l'encart 15-3.

<Listing number="15-3" file-name="src/main.rs" caption="Utiliser l'enum `List` pour stocker la liste `1, 2, 3`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

<!--
The first `Cons` value holds `1` and another `List` value. This `List` value is
another `Cons` value that holds `2` and another `List` value. This `List` value
is one more `Cons` value that holds `3` and a `List` value, which is finally
`Nil`, the non-recursive variant that signals the end of the list.
-->

La première valeur `Cons` contient `1` et une autre valeur `List`. Cette valeur `List` est une autre valeur `Cons` qui contient `2` et une autre valeur `List`. Cette valeur `List` est encore une autre valeur `Cons` qui contient `3` et une valeur `List`, qui est finalement `Nil`, la variante non récursive qui signale la fin de la liste.

<!--
If we try to compile the code in Listing 15-3, we get the error shown in
Listing 15-4.
-->

Si nous essayons de compiler le code de l'encart 15-3, nous obtenons l'erreur montrée dans l'encart 15-4.

<Listing number="15-4" caption="L'erreur que nous obtenons en tentant de définir une enum récursive">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

<!--
The error shows this type "has infinite size." The reason is that we've defined
`List` with a variant that is recursive: It holds another value of itself
directly. As a result, Rust can't figure out how much space it needs to store a
`List` value. Let's break down why we get this error. First, we'll look at how
Rust decides how much space it needs to store a value of a non-recursive type.
-->

L'erreur indique que ce type "a une taille infinie". La raison est que nous avons défini `List` avec une variante qui est récursive : elle contient directement une autre valeur d'elle-même. En conséquence, Rust ne peut pas déterminer combien d'espace il a besoin pour stocker une valeur `List`. Analysons pourquoi nous obtenons cette erreur. D'abord, nous verrons comment Rust décide de l'espace nécessaire pour stocker une valeur d'un type non récursif.

<!--
#### Computing the Size of a Non-Recursive Type
-->

#### Calculer la taille d'un type non récursif

<!--
Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:
-->

Rappelez-vous l'enum `Message` que nous avons définie dans l'encart 6-2 lorsque nous avons abordé les définitions d'enum au chapitre 6 :

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<!--
To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see which variant needs the most space. Rust
sees that `Message::Quit` doesn't need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Because only one variant will be
used, the most space a `Message` value will need is the space it would take to
store the largest of its variants.
-->

Pour déterminer combien d'espace allouer pour une valeur `Message`, Rust parcourt chacune des variantes pour voir laquelle nécessite le plus d'espace. Rust constate que `Message::Quit` n'a besoin d'aucun espace, `Message::Move` a besoin de suffisamment d'espace pour stocker deux valeurs `i32`, et ainsi de suite. Comme une seule variante sera utilisée, l'espace maximum dont une valeur `Message` aura besoin est l'espace nécessaire pour stocker la plus grande de ses variantes.

<!--
Contrast this with what happens when Rust tries to determine how much space a
recursive type like the `List` enum in Listing 15-2 needs. The compiler starts
by looking at the `Cons` variant, which holds a value of type `i32` and a value
of type `List`. Therefore, `Cons` needs an amount of space equal to the size of
an `i32` plus the size of a `List`. To figure out how much memory the `List`
type needs, the compiler looks at the variants, starting with the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, and this process continues infinitely, as shown in Figure 15-1.
-->

Comparez cela avec ce qui se passe lorsque Rust essaie de déterminer combien d'espace un type récursif comme l'enum `List` de l'encart 15-2 nécessite. Le compilateur commence par examiner la variante `Cons`, qui contient une valeur de type `i32` et une valeur de type `List`. Par conséquent, `Cons` a besoin d'un espace égal à la taille d'un `i32` plus la taille d'un `List`. Pour déterminer combien de mémoire le type `List` nécessite, le compilateur examine les variantes, en commençant par la variante `Cons`. La variante `Cons` contient une valeur de type `i32` et une valeur de type `List`, et ce processus continue à l'infini, comme illustré dans la figure 15-1.

<img alt="An infinite Cons list: a rectangle labeled 'Cons' split into two smaller rectangles. The first smaller rectangle holds the label 'i32', and the second smaller rectangle holds the label 'Cons' and a smaller version of the outer 'Cons' rectangle. The 'Cons' rectangles continue to hold smaller and smaller versions of themselves until the smallest comfortably sized rectangle holds an infinity symbol, indicating that this repetition goes on forever." src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 15-1: An infinite `List` consisting of infinite
`Cons` variants</span>
-->

<span class="caption">Figure 15-1 : Une `List` infinie composée de variantes `Cons` infinies</span>

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-boxt-to-get-a-recursive-type-with-a-known-size"></a>

<!--
#### Getting a Recursive Type with a Known Size
-->

#### Obtenir un type récursif avec une taille connue

<!--
Because Rust can't figure out how much space to allocate for recursively
defined types, the compiler gives an error with this helpful suggestion:
-->

Comme Rust ne peut pas déterminer combien d'espace allouer pour les types définis récursivement, le compilateur donne une erreur avec cette suggestion utile :

<!--
manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

<!--
In this suggestion, _indirection_ means that instead of storing a value
directly, we should change the data structure to store the value indirectly by
storing a pointer to the value instead.
-->

Dans cette suggestion, _indirection_ signifie qu'au lieu de stocker une valeur directement, nous devrions modifier la structure de données pour stocker la valeur indirectement en stockant un pointeur vers la valeur à la place.

<!--
Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>`
needs: A pointer's size doesn't change based on the amount of data it's
pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead
of another `List` value directly. The `Box<T>` will point to the next `List`
value that will be on the heap rather than inside the `Cons` variant.
Conceptually, we still have a list, created with lists holding other lists, but
this implementation is now more like placing the items next to one another
rather than inside one another.
-->

Comme un `Box<T>` est un pointeur, Rust sait toujours combien d'espace un `Box<T>` nécessite : la taille d'un pointeur ne change pas en fonction de la quantité de données vers laquelle il pointe. Cela signifie que nous pouvons mettre un `Box<T>` dans la variante `Cons` au lieu d'une autre valeur `List` directement. Le `Box<T>` pointera vers la prochaine valeur `List` qui sera sur le tas plutôt qu'à l'intérieur de la variante `Cons`. Conceptuellement, nous avons toujours une liste, créée avec des listes contenant d'autres listes, mais cette implémentation ressemble désormais davantage à placer les éléments les uns à côté des autres plutôt que les uns dans les autres.

<!--
We can change the definition of the `List` enum in Listing 15-2 and the usage
of the `List` in Listing 15-3 to the code in Listing 15-5, which will compile.
-->

Nous pouvons modifier la définition de l'enum `List` de l'encart 15-2 et l'utilisation de `List` de l'encart 15-3 pour obtenir le code de l'encart 15-5, qui compilera.

<Listing number="15-5" file-name="src/main.rs" caption="La définition de `List` qui utilise `Box<T>` pour avoir une taille connue">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

<!--
The `Cons` variant needs the size of an `i32` plus the space to store the box's
pointer data. The `Nil` variant stores no values, so it needs less space on the
stack than the `Cons` variant. We now know that any `List` value will take up
the size of an `i32` plus the size of a box's pointer data. By using a box,
we've broken the infinite, recursive chain, so the compiler can figure out the
size it needs to store a `List` value. Figure 15-2 shows what the `Cons`
variant looks like now.
-->

La variante `Cons` a besoin de la taille d'un `i32` plus l'espace pour stocker les données du pointeur de la boîte. La variante `Nil` ne stocke aucune valeur, donc elle nécessite moins d'espace sur la pile que la variante `Cons`. Nous savons maintenant que toute valeur `List` occupera la taille d'un `i32` plus la taille des données du pointeur d'une boîte. En utilisant une boîte, nous avons brisé la chaîne récursive infinie, de sorte que le compilateur peut déterminer la taille nécessaire pour stocker une valeur `List`. La figure 15-2 montre à quoi ressemble désormais la variante `Cons`.

<img alt="A rectangle labeled 'Cons' split into two smaller rectangles. The first smaller rectangle holds the label 'i32', and the second smaller rectangle holds the label 'Box' with one inner rectangle that contains the label 'usize', representing the finite size of the box's pointer." src="img/trpl15-02.svg" class="center" />

<!--
<span class="caption">Figure 15-2: A `List` that is not infinitely sized,
because `Cons` holds a `Box`</span>
-->

<span class="caption">Figure 15-2 : Une `List` qui n'est pas de taille infinie, car `Cons` contient un `Box`</span>

<!--
Boxes provide only the indirection and heap allocation; they don't have any
other special capabilities, like those we'll see with the other smart pointer
types. They also don't have the performance overhead that these special
capabilities incur, so they can be useful in cases like the cons list where the
indirection is the only feature we need. We'll look at more use cases for boxes
in Chapter 18.
-->

Les boîtes ne fournissent que l'indirection et l'allocation sur le tas ; elles n'ont pas d'autres capacités spéciales, comme celles que nous verrons avec les autres types de pointeurs intelligents. Elles n'ont pas non plus le surcoût en performance que ces capacités spéciales entraînent, ce qui les rend utiles dans des cas comme la liste cons où l'indirection est la seule fonctionnalité dont nous avons besoin. Nous examinerons d'autres cas d'utilisation des boîtes au chapitre 18.

<!--
The `Box<T>` type is a smart pointer because it implements the `Deref` trait,
which allows `Box<T>` values to be treated like references. When a `Box<T>`
value goes out of scope, the heap data that the box is pointing to is cleaned
up as well because of the `Drop` trait implementation. These two traits will be
even more important to the functionality provided by the other smart pointer
types we'll discuss in the rest of this chapter. Let's explore these two traits
in more detail.
-->

Le type `Box<T>` est un pointeur intelligent car il implémente le trait `Deref`, qui permet aux valeurs `Box<T>` d'être traitées comme des références. Quand une valeur `Box<T>` sort de la portée, les données du tas vers lesquelles la boîte pointe sont également nettoyées grâce à l'implémentation du trait `Drop`. Ces deux traits seront encore plus importants pour les fonctionnalités fournies par les autres types de pointeurs intelligents que nous aborderons dans le reste de ce chapitre. Explorons ces deux traits plus en détail.

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
