<!--
## Storing Lists of Values with Vectors
-->

## Stocker des listes de valeurs avec les vectors

<!--
The first collection type we'll look at is `Vec<T>`, also known as a vector.
Vectors allow you to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.
-->

Le premier type de collection que nous allons examiner est `Vec<T>`, également
appelé vector. Les vectors vous permettent de stocker plus d'une valeur dans
une seule structure de données qui place toutes les valeurs les unes à côté des
autres en mémoire. Les vectors ne peuvent stocker que des valeurs du même type.
Ils sont utiles lorsque vous avez une liste d'éléments, comme les lignes de
texte dans un fichier ou les prix des articles dans un panier.

<!--
### Creating a New Vector
-->

### Créer un nouveau vector

<!--
To create a new, empty vector, we call the `Vec::new` function, as shown in
Listing 8-1.
-->

Pour créer un nouveau vector vide, nous appelons la fonction `Vec::new`, comme
montré dans l'encart 8-1.

<Listing number="8-1" caption="Création d'un nouveau vector vide pour contenir des valeurs de type `i32`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

<!--
Note that we added a type annotation here. Because we aren't inserting any
values into this vector, Rust doesn't know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we'll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type.
When we create a vector to hold a specific type, we can specify the type within
angle brackets. In Listing 8-1, we've told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.
-->

Notez que nous avons ajouté une annotation de type ici. Comme nous n'insérons
aucune valeur dans ce vector, Rust ne sait pas quel type d'éléments nous avons
l'intention de stocker. C'est un point important. Les vectors sont implémentés
à l'aide de la généricité ; nous verrons comment utiliser la généricité avec
vos propres types au chapitre 10. Pour l'instant, sachez que le type `Vec<T>`
fourni par la bibliothèque standard peut contenir n'importe quel type. Quand
nous créons un vector pour contenir un type spécifique, nous pouvons préciser
le type entre chevrons. Dans l'encart 8-1, nous avons indiqué à Rust que le
`Vec<T>` dans `v` contiendra des éléments de type `i32`.

<!--
More often, you'll create a `Vec<T>` with initial values, and Rust will infer
the type of value you want to store, so you rarely need to do this type
annotation. Rust conveniently provides the `vec!` macro, which will create a
new vector that holds the values you give it. Listing 8-2 creates a new
`Vec<i32>` that holds the values `1`, `2`, and `3`. The integer type is `i32`
because that's the default integer type, as we discussed in the ["Data
Types"][data-types] ignore
--> section of Chapter 3.
-->

Le plus souvent, vous créerez un `Vec<T>` avec des valeurs initiales, et Rust
déduira le type de valeur que vous souhaitez stocker, de sorte que vous avez
rarement besoin de faire cette annotation de type. Rust fournit la macro `vec!`
pour plus de commodité, qui crée un nouveau vector contenant les valeurs que
vous lui donnez. L'encart 8-2 crée un nouveau `Vec<i32>` qui contient les
valeurs `1`, `2` et `3`. Le type entier est `i32` car c'est le type entier par
défaut, comme nous l'avons vu dans la section
["Types de données"][data-types]<!--
ignore
--> du chapitre 3.

<Listing number="8-2" caption="Création d'un nouveau vector contenant des valeurs">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

<!--
Because we've given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn't necessary. Next, we'll look at how
to modify a vector.
-->

Comme nous avons donné des valeurs `i32` initiales, Rust peut déduire que le
type de `v` est `Vec<i32>`, et l'annotation de type n'est pas nécessaire.
Voyons maintenant comment modifier un vector.

<!--
### Updating a Vector
-->

### Mettre à jour un vector

<!--
To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.
-->

Pour créer un vector puis y ajouter des éléments, nous pouvons utiliser la
méthode `push`, comme montré dans l'encart 8-3.

<Listing number="8-3" caption="Utilisation de la méthode `push` pour ajouter des valeurs à un vector">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

<!--
As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don't need the `Vec<i32>` annotation.
-->

Comme pour toute variable, si nous voulons pouvoir modifier sa valeur, nous
devons la rendre mutable en utilisant le mot-clé `mut`, comme nous l'avons vu
au chapitre 3. Les nombres que nous plaçons à l'intérieur sont tous de type
`i32`, et Rust le déduit à partir des données, donc nous n'avons pas besoin de
l'annotation `Vec<i32>`.

<!--
### Reading Elements of Vectors
-->

### Lire les éléments d'un vector

<!--
There are two ways to reference a value stored in a vector: via indexing or by
using the `get` method. In the following examples, we've annotated the types of
the values that are returned from these functions for extra clarity.
-->

Il existe deux façons de référencer une valeur stockée dans un vector : par
l'indexation ou en utilisant la méthode `get`. Dans les exemples suivants, nous
avons annoté les types des valeurs renvoyées par ces fonctions pour plus de
clarté.

<!--
Listing 8-4 shows both methods of accessing a value in a vector, with indexing
syntax and the `get` method.
-->

L'encart 8-4 montre les deux méthodes d'accès à une valeur dans un vector,
avec la syntaxe d'indexation et la méthode `get`.

<Listing number="8-4" caption="Utilisation de la syntaxe d'indexation et de la méthode `get` pour accéder à un élément d'un vector">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

<!--
Note a few details here. We use the index value of `2` to get the third element
because vectors are indexed by number, starting at zero. Using `&` and `[]`
gives us a reference to the element at the index value. When we use the `get`
method with the index passed as an argument, we get an `Option<&T>` that we can
use with `match`.
-->

Notez quelques détails ici. Nous utilisons l'indice `2` pour obtenir le
troisième élément car les vectors sont indexés par numéro, en commençant à
zéro. L'utilisation de `&` et `[]` nous donne une référence à l'élément situé
à cet indice. Lorsque nous utilisons la méthode `get` avec l'indice passé en
argument, nous obtenons un `Option<&T>` que nous pouvons utiliser avec `match`.

<!--
Rust provides these two ways to reference an element so that you can choose how
the program behaves when you try to use an index value outside the range of
existing elements. As an example, let's see what happens when we have a vector
of five elements and then we try to access an element at index 100 with each
technique, as shown in Listing 8-5.
-->

Rust fournit ces deux façons de référencer un élément afin que vous puissiez
choisir comment le programme se comporte lorsque vous essayez d'utiliser un
indice en dehors de la plage des éléments existants. Par exemple, voyons ce qui
se passe lorsque nous avons un vector de cinq éléments et que nous essayons
d'accéder à un élément à l'indice 100 avec chaque technique, comme montré dans
l'encart 8-5.

<Listing number="8-5" caption="Tentative d'accès à l'élément à l'indice 100 dans un vector contenant cinq éléments">


```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

<!--
When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there's an attempt to access an element past the
end of the vector.
-->

Lorsque nous exécutons ce code, la première méthode avec `[]` provoquera un
panic du programme car elle référence un élément inexistant. Cette méthode est
à privilégier lorsque vous souhaitez que votre programme plante s'il y a une
tentative d'accès à un élément au-delà de la fin du vector.

<!--
When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector may happen occasionally under normal
circumstances. Your code will then have logic to handle having either
`Some(&element)` or `None`, as discussed in Chapter 6. For example, the index
could be coming from a person entering a number. If they accidentally enter a
number that's too large and the program gets a `None` value, you could tell the
user how many items are in the current vector and give them another chance to
enter a valid value. That would be more user-friendly than crashing the program
due to a typo!
-->

Lorsque la méthode `get` reçoit un indice en dehors du vector, elle renvoie
`None` sans paniquer. Vous utiliseriez cette méthode si l'accès à un élément
au-delà de la plage du vector peut arriver occasionnellement dans des
circonstances normales. Votre code aura alors la logique pour gérer soit
`Some(&element)` soit `None`, comme nous l'avons vu au chapitre 6. Par
exemple, l'indice pourrait provenir d'une personne qui saisit un nombre. Si
elle entre accidentellement un nombre trop grand et que le programme obtient
une valeur `None`, vous pourriez indiquer à l'utilisateur combien d'éléments
se trouvent dans le vector actuel et lui donner une autre chance de saisir une
valeur valide. Ce serait plus convivial que de faire planter le programme à
cause d'une faute de frappe !

<!--
When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure that this
reference and any other references to the contents of the vector remain valid.
Recall the rule that states you can't have mutable and immutable references in
the same scope. That rule applies in Listing 8-6, where we hold an immutable
reference to the first element in a vector and try to add an element to the
end. This program won't work if we also try to refer to that element later in
the function.
-->

Lorsque le programme détient une référence valide, le vérificateur d'emprunt
applique les règles de possession et d'emprunt (couvertes au chapitre 4) pour
garantir que cette référence et toute autre référence au contenu du vector
restent valides. Rappelez-vous la règle qui stipule que vous ne pouvez pas
avoir des références mutables et immuables dans la même portée. Cette règle
s'applique dans l'encart 8-6, où nous détenons une référence immuable au
premier élément d'un vector et essayons d'ajouter un élément à la fin. Ce
programme ne fonctionnera pas si nous essayons aussi de faire référence à cet
élément plus tard dans la fonction.

<Listing number="8-6" caption="Tentative d'ajout d'un élément à un vector tout en détenant une référence à un élément">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

<!--
Compiling this code will result in this error:
-->

La compilation de ce code produira cette erreur :


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

<!--
The code in Listing 8-6 might look like it should work: Why should a reference
to the first element care about changes at the end of the vector? This error is
due to the way vectors work: Because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require
allocating new memory and copying the old elements to the new space, if there
isn't enough room to put all the elements next to each other where the vector
is currently stored. In that case, the reference to the first element would be
pointing to deallocated memory. The borrowing rules prevent programs from
ending up in that situation.
-->

Le code de l'encart 8-6 pourrait sembler devoir fonctionner : pourquoi une
référence au premier élément se soucierait-elle de changements à la fin du
vector ? Cette erreur est due au fonctionnement des vectors : comme les vectors
placent les valeurs les unes à côté des autres en mémoire, l'ajout d'un nouvel
élément à la fin du vector pourrait nécessiter l'allocation de nouvelle mémoire
et la copie des anciens éléments vers le nouvel espace, s'il n'y a pas assez
de place pour mettre tous les éléments les uns à côté des autres là où le
vector est actuellement stocké. Dans ce cas, la référence au premier élément
pointerait vers de la mémoire désallouée. Les règles d'emprunt empêchent les
programmes de se retrouver dans cette situation.

<!--
> Note: For more on the implementation details of the `Vec<T>` type, see ["The
> Rustonomicon"][nomicon].
-->

> Remarque : Pour plus de détails sur l'implémentation du type `Vec<T>`,
> consultez ["The Rustonomicon"][nomicon].

<!--
### Iterating Over the Values in a Vector
-->

### Itérer sur les valeurs d'un vector

<!--
To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time. Listing 8-7 shows how
to use a `for` loop to get immutable references to each element in a vector of
`i32` values and print them.
-->

Pour accéder à chaque élément d'un vector à tour de rôle, nous itérerions sur
tous les éléments plutôt que d'utiliser des indices pour y accéder un par un.
L'encart 8-7 montre comment utiliser une boucle `for` pour obtenir des
références immuables à chaque élément d'un vector de valeurs `i32` et les
afficher.

<Listing number="8-7" caption="Affichage de chaque élément d'un vector en itérant sur les éléments à l'aide d'une boucle `for`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

<!--
We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-8
will add `50` to each element.
-->

Nous pouvons également itérer sur des références mutables à chaque élément d'un
vector mutable afin d'apporter des modifications à tous les éléments. La boucle
`for` de l'encart 8-8 ajoutera `50` à chaque élément.

<Listing number="8-8" caption="Itération sur des références mutables aux éléments d'un vector">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

<!--
To change the value that the mutable reference refers to, we have to use the
`*` dereference operator to get to the value in `i` before we can use the `+=`
operator. We'll talk more about the dereference operator in the ["Following the
Reference to the Value"][deref] ignore
--> section of Chapter 15.
-->

Pour modifier la valeur à laquelle la référence mutable fait référence, nous
devons utiliser l'opérateur de déréférencement `*` pour accéder à la valeur
dans `i` avant de pouvoir utiliser l'opérateur `+=`. Nous parlerons davantage
de l'opérateur de déréférencement dans la section ["Suivre la référence
jusqu'à la valeur"][deref]<!--
ignore
--> du chapitre 15.

<!--
Iterating over a vector, whether immutably or mutably, is safe because of the
borrow checker's rules. If we attempted to insert or remove items in the `for`
loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error
similar to the one we got with the code in Listing 8-6. The reference to the
vector that the `for` loop holds prevents simultaneous modification of the
whole vector.
-->

Itérer sur un vector, que ce soit de manière immuable ou mutable, est sûr grâce
aux règles du vérificateur d'emprunt. Si nous tentions d'insérer ou de
supprimer des éléments dans le corps des boucles `for` des encarts 8-7 et 8-8,
nous obtiendrions une erreur de compilation similaire à celle que nous avons
obtenue avec le code de l'encart 8-6. La référence au vector que la boucle
`for` détient empêche la modification simultanée de l'ensemble du vector.

<!--
### Using an Enum to Store Multiple Types
-->

### Utiliser une enum pour stocker plusieurs types

<!--
Vectors can only store values that are of the same type. This can be
inconvenient; there are definitely use cases for needing to store a list of
items of different types. Fortunately, the variants of an enum are defined
under the same enum type, so when we need one type to represent elements of
different types, we can define and use an enum!
-->

Les vectors ne peuvent stocker que des valeurs du même type. Cela peut être
gênant ; il existe assurément des cas d'utilisation où l'on a besoin de stocker
une liste d'éléments de différents types. Heureusement, les variantes d'une
enum sont définies sous le même type d'enum, donc lorsque nous avons besoin
d'un seul type pour représenter des éléments de différents types, nous pouvons
définir et utiliser une enum !

<!--
For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and all the enum variants will be considered the same type: that
of the enum. Then, we can create a vector to hold that enum and so, ultimately,
hold different types. We've demonstrated this in Listing 8-9.
-->

Par exemple, supposons que nous voulions obtenir des valeurs d'une ligne dans
un tableur dans lequel certaines colonnes de la ligne contiennent des entiers,
d'autres des nombres à virgule flottante, et d'autres des chaînes de
caractères. Nous pouvons définir une enum dont les variantes contiendront les
différents types de valeurs, et toutes les variantes de l'enum seront
considérées comme le même type : celui de l'enum. Ensuite, nous pouvons créer
un vector pour contenir cette enum et ainsi, au final, contenir des types
différents. Nous l'avons démontré dans l'encart 8-9.

<Listing number="8-9" caption="Définition d'une enum pour stocker des valeurs de différents types dans un seul vector">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

<!--
Rust needs to know what types will be in the vector at compile time so that it
knows exactly how much memory on the heap will be needed to store each element.
We must also be explicit about what types are allowed in this vector. If Rust
allowed a vector to hold any type, there would be a chance that one or more of
the types would cause errors with the operations performed on the elements of
the vector. Using an enum plus a `match` expression means that Rust will ensure
at compile time that every possible case is handled, as discussed in Chapter 6.
-->

Rust a besoin de savoir quels types seront dans le vector à la compilation afin
de savoir exactement combien de mémoire sur le tas sera nécessaire pour stocker
chaque élément. Nous devons également être explicites sur les types autorisés
dans ce vector. Si Rust permettait à un vector de contenir n'importe quel type,
il y aurait un risque qu'un ou plusieurs de ces types causent des erreurs avec
les opérations effectuées sur les éléments du vector. L'utilisation d'une enum
combinée à une expression `match` signifie que Rust s'assurera à la compilation
que chaque cas possible est traité, comme nous l'avons vu au chapitre 6.

<!--
If you don't know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won't work. Instead, you can use a trait
object, which we'll cover in Chapter 18.
-->

Si vous ne connaissez pas l'ensemble exhaustif des types qu'un programme
recevra à l'exécution pour les stocker dans un vector, la technique de l'enum
ne fonctionnera pas. À la place, vous pouvez utiliser un objet trait, que nous
couvrirons au chapitre 18.

<!--
Now that we've discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api] ignore
--> for all of the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element.
-->

Maintenant que nous avons discuté de certaines des façons les plus courantes
d'utiliser les vectors, n'oubliez pas de consulter [la documentation de
l'API][vec-api]<!--
ignore
--> pour toutes les nombreuses méthodes utiles
définies sur `Vec<T>` par la bibliothèque standard. Par exemple, en plus de
`push`, une méthode `pop` supprime et renvoie le dernier élément.

<!--
### Dropping a Vector Drops Its Elements
-->

### Libérer un vector libère ses éléments

<!--
Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-10.
-->

Comme toute autre `struct`, un vector est libéré lorsqu'il sort de la portée,
comme annoté dans l'encart 8-10.

<Listing number="8-10" caption="Montrer où le vector et ses éléments sont libérés">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

<!--
When the vector gets dropped, all of its contents are also dropped, meaning the
integers it holds will be cleaned up. The borrow checker ensures that any
references to contents of a vector are only used while the vector itself is
valid.
-->

Lorsque le vector est libéré, tout son contenu est également libéré, ce qui
signifie que les entiers qu'il contient seront nettoyés. Le vérificateur
d'emprunt garantit que toute référence au contenu d'un vector n'est utilisée
que tant que le vector lui-même est valide.

<!--
Let's move on to the next collection type: `String`!
-->

Passons au type de collection suivant : `String` !

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
