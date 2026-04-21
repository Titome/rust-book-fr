<!--
## Unsafe Rust
-->

## Le Rust unsafe

<!--
All the code we've discussed so far has had Rust's memory safety guarantees
enforced at compile time. However, Rust has a second language hidden inside it
that doesn't enforce these memory safety guarantees: It's called _unsafe Rust_
and works just like regular Rust but gives us extra superpowers.
-->

Tout le code que nous avons vu jusqu'à présent bénéficiait des garanties de sécurité mémoire de Rust appliquées au moment de la compilation. Cependant, Rust contient un second langage caché qui n'applique pas ces garanties de sécurité mémoire : il s'appelle _Rust unsafe_ et fonctionne comme le Rust normal mais nous donne des super-pouvoirs supplémentaires.

<!--
Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler tries to determine whether or not code upholds the guarantees,
it's better for it to reject some valid programs than to accept some invalid
programs. Although the code _might_ be okay, if the Rust compiler doesn't have
enough information to be confident, it will reject the code. In these cases,
you can use unsafe code to tell the compiler, "Trust me, I know what I'm
doing." Be warned, however, that you use unsafe Rust at your own risk: If you
use unsafe code incorrectly, problems can occur due to memory unsafety, such as
null pointer dereferencing.
-->

Le Rust unsafe existe car, par nature, l'analyse statique est conservatrice. Lorsque le compilateur essaie de déterminer si le code respecte les garanties, il vaut mieux pour lui rejeter certains programmes valides que d'accepter des programmes invalides. Bien que le code _puisse_ être correct, si le compilateur Rust n'a pas assez d'informations pour en être certain, il rejettera le code. Dans ces cas, vous pouvez utiliser du code unsafe pour dire au compilateur : "Fais-moi confiance, je sais ce que je fais." Soyez cependant averti que vous utilisez le Rust unsafe à vos propres risques : si vous utilisez du code unsafe incorrectement, des problèmes liés à l'insécurité mémoire peuvent survenir, comme le déréférencement de pointeur nul.

<!--
Another reason Rust has an unsafe alter ego is that the underlying computer
hardware is inherently unsafe. If Rust didn't let you do unsafe operations, you
couldn't do certain tasks. Rust needs to allow you to do low-level systems
programming, such as directly interacting with the operating system or even
writing your own operating system. Working with low-level systems programming
is one of the goals of the language. Let's explore what we can do with unsafe
Rust and how to do it.
-->

Une autre raison pour laquelle Rust a un alter ego unsafe est que le matériel informatique sous-jacent est intrinsèquement non sécurisé. Si Rust ne vous permettait pas de faire des opérations unsafe, vous ne pourriez pas accomplir certaines tâches. Rust doit vous permettre de faire de la programmation système bas niveau, comme interagir directement avec le système d'exploitation ou même écrire votre propre système d'exploitation. Travailler avec la programmation système bas niveau est l'un des objectifs du langage. Explorons ce que nous pouvons faire avec le Rust unsafe et comment le faire.

<!--
Old headings. Do not remove or links may break.
-->

<a id="unsafe-superpowers"></a>

<!--
### Performing Unsafe Superpowers
-->

### Utiliser les super-pouvoirs unsafe

<!--
To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust that you
can't in safe Rust, which we call _unsafe superpowers_. Those superpowers
include the ability to:
-->

Pour passer en Rust unsafe, utilisez le mot-clé `unsafe` puis ouvrez un nouveau bloc contenant le code unsafe. Vous pouvez effectuer cinq actions en Rust unsafe que vous ne pouvez pas faire en Rust safe, que nous appelons les _super-pouvoirs unsafe_. Ces super-pouvoirs incluent la capacité de :

<!--
1. Dereference a raw pointer.
1. Call an unsafe function or method.
1. Access or modify a mutable static variable.
1. Implement an unsafe trait.
1. Access fields of `union`s.
-->

1. Déréférencer un pointeur brut.
1. Appeler une fonction ou méthode unsafe.
1. Accéder ou modifier une variable statique mutable.
1. Implémenter un trait unsafe.
1. Accéder aux champs de `union`s.

<!--
It's important to understand that `unsafe` doesn't turn off the borrow checker
or disable any of Rust's other safety checks: If you use a reference in unsafe
code, it will still be checked. The `unsafe` keyword only gives you access to
these five features that are then not checked by the compiler for memory
safety. You'll still get some degree of safety inside an unsafe block.
-->

Il est important de comprendre que `unsafe` ne désactive pas le vérificateur d'emprunt ni aucune des autres vérifications de sécurité de Rust : si vous utilisez une référence dans du code unsafe, elle sera toujours vérifiée. Le mot-clé `unsafe` ne vous donne accès qu'à ces cinq fonctionnalités qui ne sont alors pas vérifiées par le compilateur pour la sécurité mémoire. Vous bénéficierez toujours d'un certain degré de sécurité à l'intérieur d'un bloc unsafe.

<!--
In addition, `unsafe` does not mean the code inside the block is necessarily
dangerous or that it will definitely have memory safety problems: The intent is
that as the programmer, you'll ensure that the code inside an `unsafe` block
will access memory in a valid way.
-->

De plus, `unsafe` ne signifie pas que le code à l'intérieur du bloc est nécessairement dangereux ou qu'il aura forcément des problèmes de sécurité mémoire : l'intention est que vous, en tant que programmeur, vous assurerez que le code à l'intérieur d'un bloc `unsafe` accédera à la mémoire de manière valide.

<!--
People are fallible and mistakes will happen, but by requiring these five
unsafe operations to be inside blocks annotated with `unsafe`, you'll know that
any errors related to memory safety must be within an `unsafe` block. Keep
`unsafe` blocks small; you'll be thankful later when you investigate memory
bugs.
-->

Les gens sont faillibles et les erreurs arrivent, mais en exigeant que ces cinq opérations unsafe soient dans des blocs annotés avec `unsafe`, vous saurez que toute erreur liée à la sécurité mémoire doit se trouver dans un bloc `unsafe`. Gardez les blocs `unsafe` petits ; vous vous en féliciterez plus tard lorsque vous chercherez des bogues mémoire.

<!--
To isolate unsafe code as much as possible, it's best to enclose such code
within a safe abstraction and provide a safe API, which we'll discuss later in
the chapter when we examine unsafe functions and methods. Parts of the standard
library are implemented as safe abstractions over unsafe code that has been
audited. Wrapping unsafe code in a safe abstraction prevents uses of `unsafe`
from leaking out into all the places that you or your users might want to use
the functionality implemented with `unsafe` code, because using a safe
abstraction is safe.
-->

Pour isoler le code unsafe autant que possible, il est préférable d'encapsuler ce code dans une abstraction sûre et de fournir une API sûre, ce dont nous discuterons plus loin dans le chapitre lorsque nous examinerons les fonctions et méthodes unsafe. Des parties de la bibliothèque standard sont implémentées comme des abstractions sûres par-dessus du code unsafe qui a été audité. Encapsuler du code unsafe dans une abstraction sûre empêche les utilisations d'`unsafe` de fuiter dans tous les endroits où vous ou vos utilisateurs pourriez vouloir utiliser la fonctionnalité implémentée avec du code `unsafe`, car utiliser une abstraction sûre est sûr.

<!--
Let's look at each of the five unsafe superpowers in turn. We'll also look at
some abstractions that provide a safe interface to unsafe code.
-->

Examinons chacun des cinq super-pouvoirs unsafe à tour de rôle. Nous verrons aussi quelques abstractions qui fournissent une interface sûre au code unsafe.

<!--
### Dereferencing a Raw Pointer
-->

### Déréférencer un pointeur brut

<!--
In Chapter 4, in the ["Dangling References"][dangling-references] ignore
--> section, we mentioned that the compiler ensures that references are always
valid. Unsafe Rust has two new types called _raw pointers_ that are similar to
references. As with references, raw pointers can be immutable or mutable and
are written as `*const T` and `*mut T`, respectively. The asterisk isn't the
dereference operator; it's part of the type name. In the context of raw
pointers, _immutable_ means that the pointer can't be directly assigned to
after being dereferenced.
-->

Au chapitre 4, dans la section ["Les références pendantes"][dangling-references]<!--
ignore
-->, nous avons mentionné que le compilateur s'assure que les références sont toujours valides. Le Rust unsafe possède deux nouveaux types appelés _pointeurs bruts_ qui sont similaires aux références. Comme les références, les pointeurs bruts peuvent être immuables ou mutables et s'écrivent respectivement `*const T` et `*mut T`. L'astérisque n'est pas l'opérateur de déréférencement ; il fait partie du nom du type. Dans le contexte des pointeurs bruts, _immuable_ signifie que le pointeur ne peut pas être directement assigné après avoir été déréférencé.

<!--
Different from references and smart pointers, raw pointers:
-->

Contrairement aux références et aux pointeurs intelligents, les pointeurs bruts :

<!--
- Are allowed to ignore the borrowing rules by having both immutable and
  mutable pointers or multiple mutable pointers to the same location
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup
-->

- Peuvent ignorer les règles d'emprunt en ayant à la fois des pointeurs immuables et mutables ou plusieurs pointeurs mutables vers le même emplacement
- Ne sont pas garantis de pointer vers de la mémoire valide
- Sont autorisés à être nuls
- N'implémentent aucun nettoyage automatique

<!--
By opting out of having Rust enforce these guarantees, you can give up
guaranteed safety in exchange for greater performance or the ability to
interface with another language or hardware where Rust's guarantees don't apply.
-->

En renonçant à ce que Rust applique ces garanties, vous pouvez abandonner la sécurité garantie en échange de meilleures performances ou de la capacité d'interfacer avec un autre langage ou du matériel où les garanties de Rust ne s'appliquent pas.

<!--
Listing 20-1 shows how to create an immutable and a mutable raw pointer.
-->

L'encart 20-1 montre comment créer un pointeur brut immuable et un pointeur brut mutable.

<Listing number="20-1" caption="Créer des pointeurs bruts avec les opérateurs d'emprunt brut">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

<!--
Notice that we don't include the `unsafe` keyword in this code. We can create
raw pointers in safe code; we just can't dereference raw pointers outside an
unsafe block, as you'll see in a bit.
-->

Remarquez que nous n'incluons pas le mot-clé `unsafe` dans ce code. Nous pouvons créer des pointeurs bruts dans du code safe ; nous ne pouvons simplement pas déréférencer des pointeurs bruts en dehors d'un bloc unsafe, comme vous le verrez bientôt.

<!--
We've created raw pointers by using the raw borrow operators: `&raw const num`
creates a `*const i32` immutable raw pointer, and `&raw mut num` creates a `*mut
i32` mutable raw pointer. Because we created them directly from a local
variable, we know these particular raw pointers are valid, but we can't make
that assumption about just any raw pointer.
-->

Nous avons créé des pointeurs bruts en utilisant les opérateurs d'emprunt brut : `&raw const num` crée un pointeur brut immuable `*const i32`, et `&raw mut num` crée un pointeur brut mutable `*mut i32`. Comme nous les avons créés directement à partir d'une variable locale, nous savons que ces pointeurs bruts particuliers sont valides, mais nous ne pouvons pas faire cette hypothèse pour n'importe quel pointeur brut.

<!--
To demonstrate this, next we'll create a raw pointer whose validity we can't be
so certain of, using the keyword `as` to cast a value instead of using the raw
borrow operator. Listing 20-2 shows how to create a raw pointer to an arbitrary
location in memory. Trying to use arbitrary memory is undefined: There might be
data at that address or there might not, the compiler might optimize the code
so that there is no memory access, or the program might terminate with a
segmentation fault. Usually, there is no good reason to write code like this,
especially in cases where you can use a raw borrow operator instead, but it is
possible.
-->

Pour illustrer cela, nous allons ensuite créer un pointeur brut dont nous ne pouvons pas être aussi certains de la validité, en utilisant le mot-clé `as` pour convertir une valeur au lieu d'utiliser l'opérateur d'emprunt brut. L'encart 20-2 montre comment créer un pointeur brut vers un emplacement arbitraire en mémoire. Essayer d'utiliser de la mémoire arbitraire est un comportement indéfini : il peut y avoir des données à cette adresse ou non, le compilateur peut optimiser le code de sorte qu'il n'y ait pas d'accès mémoire, ou le programme peut se terminer par une erreur de segmentation. En général, il n'y a pas de bonne raison d'écrire du code comme celui-ci, surtout dans les cas où vous pouvez utiliser un opérateur d'emprunt brut à la place, mais c'est possible.

<Listing number="20-2" caption="Créer un pointeur brut vers une adresse mémoire arbitraire">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

<!--
Recall that we can create raw pointers in safe code, but we can't dereference
raw pointers and read the data being pointed to. In Listing 20-3, we use the
dereference operator `*` on a raw pointer that requires an `unsafe` block.
-->

Rappelez-vous que nous pouvons créer des pointeurs bruts dans du code safe, mais nous ne pouvons pas déréférencer des pointeurs bruts et lire les données pointées. Dans l'encart 20-3, nous utilisons l'opérateur de déréférencement `*` sur un pointeur brut, ce qui nécessite un bloc `unsafe`.

<Listing number="20-3" caption="Déréférencer des pointeurs bruts dans un bloc `unsafe`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

<!--
Creating a pointer does no harm; it's only when we try to access the value that
it points at that we might end up dealing with an invalid value.
-->

Créer un pointeur ne fait aucun mal ; c'est seulement lorsque nous essayons d'accéder à la valeur vers laquelle il pointe que nous risquons de traiter une valeur invalide.

<!--
Note also that in Listings 20-1 and 20-3, we created `*const i32` and `*mut
i32` raw pointers that both pointed to the same memory location, where `num` is
stored. If we instead tried to create an immutable and a mutable reference to
`num`, the code would not have compiled because Rust's ownership rules don't
allow a mutable reference at the same time as any immutable references. With
raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race. Be careful!
-->

Notez aussi que dans les encarts 20-1 et 20-3, nous avons créé des pointeurs bruts `*const i32` et `*mut i32` qui pointaient tous les deux vers le même emplacement mémoire, où `num` est stocké. Si nous avions essayé de créer une référence immuable et une référence mutable vers `num`, le code n'aurait pas compilé car les règles de possession de Rust n'autorisent pas une référence mutable en même temps que des références immuables. Avec les pointeurs bruts, nous pouvons créer un pointeur mutable et un pointeur immuable vers le même emplacement et modifier les données via le pointeur mutable, créant potentiellement une situation de compétition de données. Soyez prudent !

<!--
With all of these dangers, why would you ever use raw pointers? One major use
case is when interfacing with C code, as you'll see in the next section.
Another case is when building up safe abstractions that the borrow checker
doesn't understand. We'll introduce unsafe functions and then look at an
example of a safe abstraction that uses unsafe code.
-->

Avec tous ces dangers, pourquoi utiliseriez-vous des pointeurs bruts ? Un cas d'utilisation majeur est l'interfaçage avec du code C, comme vous le verrez dans la section suivante. Un autre cas est la construction d'abstractions sûres que le vérificateur d'emprunt ne comprend pas. Nous allons présenter les fonctions unsafe puis examiner un exemple d'abstraction sûre utilisant du code unsafe.

<!--
### Calling an Unsafe Function or Method
-->

### Appeler une fonction ou méthode unsafe

<!--
The second type of operation you can perform in an unsafe block is calling
unsafe functions. Unsafe functions and methods look exactly like regular
functions and methods, but they have an extra `unsafe` before the rest of the
definition. The `unsafe` keyword in this context indicates the function has
requirements we need to uphold when we call this function, because Rust can't
guarantee we've met these requirements. By calling an unsafe function within an
`unsafe` block, we're saying that we've read this function's documentation and
we take responsibility for upholding the function's contracts.
-->

Le deuxième type d'opération que vous pouvez effectuer dans un bloc unsafe est l'appel de fonctions unsafe. Les fonctions et méthodes unsafe ressemblent exactement aux fonctions et méthodes normales, mais elles ont un `unsafe` supplémentaire avant le reste de la définition. Le mot-clé `unsafe` dans ce contexte indique que la fonction a des exigences que nous devons respecter lorsque nous l'appelons, car Rust ne peut pas garantir que nous les avons satisfaites. En appelant une fonction unsafe dans un bloc `unsafe`, nous disons que nous avons lu la documentation de cette fonction et que nous prenons la responsabilité de respecter les contrats de la fonction.

<!--
Here is an unsafe function named `dangerous` that doesn't do anything in its
body:
-->

Voici une fonction unsafe nommée `dangerous` qui ne fait rien dans son corps :


```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

<!--
We must call the `dangerous` function within a separate `unsafe` block. If we
try to call `dangerous` without the `unsafe` block, we'll get an error:
-->

Nous devons appeler la fonction `dangerous` dans un bloc `unsafe` séparé. Si nous essayons d'appeler `dangerous` sans le bloc `unsafe`, nous obtiendrons une erreur :


```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

<!--
With the `unsafe` block, we're asserting to Rust that we've read the function's
documentation, we understand how to use it properly, and we've verified that
we're fulfilling the contract of the function.
-->

Avec le bloc `unsafe`, nous affirmons à Rust que nous avons lu la documentation de la fonction, que nous comprenons comment l'utiliser correctement, et que nous avons vérifié que nous respectons le contrat de la fonction.

<!--
To perform unsafe operations in the body of an `unsafe` function, you still
need to use an `unsafe` block, just as within a regular function, and the
compiler will warn you if you forget. This helps us keep `unsafe` blocks as
small as possible, as unsafe operations may not be needed across the whole
function body.
-->

Pour effectuer des opérations unsafe dans le corps d'une fonction `unsafe`, vous devez toujours utiliser un bloc `unsafe`, comme dans une fonction normale, et le compilateur vous avertira si vous oubliez. Cela nous aide à garder les blocs `unsafe` aussi petits que possible, car les opérations unsafe peuvent ne pas être nécessaires dans tout le corps de la fonction.

<!--
#### Creating a Safe Abstraction over Unsafe Code
-->

#### Créer une abstraction sûre par-dessus du code unsafe

<!--
Just because a function contains unsafe code doesn't mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is
a common abstraction. As an example, let's study the `split_at_mut` function
from the standard library, which requires some unsafe code. We'll explore how
we might implement it. This safe method is defined on mutable slices: It takes
one slice and makes it two by splitting the slice at the index given as an
argument. Listing 20-4 shows how to use `split_at_mut`.
-->

Ce n'est pas parce qu'une fonction contient du code unsafe que nous devons marquer toute la fonction comme unsafe. En fait, encapsuler du code unsafe dans une fonction sûre est une abstraction courante. À titre d'exemple, étudions la fonction `split_at_mut` de la bibliothèque standard, qui nécessite du code unsafe. Nous allons explorer comment nous pourrions l'implémenter. Cette méthode sûre est définie sur les slices mutables : elle prend une slice et la divise en deux en la scindant à l'indice donné en argument. L'encart 20-4 montre comment utiliser `split_at_mut`.

<Listing number="20-4" caption="Utiliser la fonction sûre `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

<!--
We can't implement this function using only safe Rust. An attempt might look
something like Listing 20-5, which won't compile. For simplicity, we'll
implement `split_at_mut` as a function rather than a method and only for slices
of `i32` values rather than for a generic type `T`.
-->

Nous ne pouvons pas implémenter cette fonction en utilisant uniquement du Rust safe. Une tentative pourrait ressembler à l'encart 20-5, qui ne compilera pas. Par souci de simplicité, nous allons implémenter `split_at_mut` comme une fonction plutôt qu'une méthode et uniquement pour des slices de valeurs `i32` plutôt que pour un type générique `T`.

<Listing number="20-5" caption="Une tentative d'implémentation de `split_at_mut` en utilisant uniquement du Rust safe">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

<!--
This function first gets the total length of the slice. Then, it asserts that
the index given as a parameter is within the slice by checking whether it's
less than or equal to the length. The assertion means that if we pass an index
that is greater than the length to split the slice at, the function will panic
before it attempts to use that index.
-->

Cette fonction obtient d'abord la longueur totale de la slice. Ensuite, elle vérifie par une assertion que l'indice donné en paramètre se trouve dans la slice en vérifiant s'il est inférieur ou égal à la longueur. L'assertion signifie que si nous passons un indice supérieur à la longueur pour scinder la slice, la fonction paniquera avant de tenter d'utiliser cet indice.

<!--
Then, we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index and another from `mid` to the end of the
slice.
-->

Ensuite, nous retournons deux slices mutables dans un tuple : une du début de la slice originale jusqu'à l'indice `mid` et une autre de `mid` jusqu'à la fin de la slice.

<!--
When we try to compile the code in Listing 20-5, we'll get an error:
-->

Lorsque nous essayons de compiler le code de l'encart 20-5, nous obtenons une erreur :


```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

<!--
Rust's borrow checker can't understand that we're borrowing different parts of
the slice; it only knows that we're borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two
slices aren't overlapping, but Rust isn't smart enough to know this. When we
know code is okay, but Rust doesn't, it's time to reach for unsafe code.
-->

Le vérificateur d'emprunt de Rust ne peut pas comprendre que nous empruntons des parties différentes de la slice ; il sait seulement que nous empruntons la même slice deux fois. Emprunter des parties différentes d'une slice est fondamentalement correct car les deux slices ne se chevauchent pas, mais Rust n'est pas assez intelligent pour le savoir. Quand nous savons que le code est correct, mais que Rust ne le sait pas, il est temps de recourir au code unsafe.

<!--
Listing 20-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work.
-->

L'encart 20-6 montre comment utiliser un bloc `unsafe`, un pointeur brut et quelques appels à des fonctions unsafe pour faire fonctionner l'implémentation de `split_at_mut`.

<Listing number="20-6" caption="Utiliser du code unsafe dans l'implémentation de la fonction `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

<!--
Recall from ["The Slice Type"][the-slice-type] ignore
--> section in
Chapter 4 that a slice is a pointer to some data and the length of the slice.
We use the `len` method to get the length of a slice and the `as_mut_ptr`
method to access the raw pointer of a slice. In this case, because we have a
mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type
`*mut i32`, which we've stored in the variable `ptr`.
-->

Rappelez-vous de la section ["Le type slice"][the-slice-type]<!--
ignore
--> du chapitre 4 qu'une slice est un pointeur vers des données et la longueur de la slice. Nous utilisons la méthode `len` pour obtenir la longueur d'une slice et la méthode `as_mut_ptr` pour accéder au pointeur brut d'une slice. Dans ce cas, parce que nous avons une slice mutable de valeurs `i32`, `as_mut_ptr` retourne un pointeur brut avec le type `*mut i32`, que nous avons stocké dans la variable `ptr`.

<!--
We keep the assertion that the `mid` index is within the slice. Then, we get to
the unsafe code: The `slice::from_raw_parts_mut` function takes a raw pointer
and a length, and it creates a slice. We use this function to create a slice
that starts from `ptr` and is `mid` items long. Then, we call the `add` method
on `ptr` with `mid` as an argument to get a raw pointer that starts at `mid`,
and we create a slice using that pointer and the remaining number of items
after `mid` as the length.
-->

Nous conservons l'assertion que l'indice `mid` se trouve dans la slice. Ensuite, nous passons au code unsafe : la fonction `slice::from_raw_parts_mut` prend un pointeur brut et une longueur, et crée une slice. Nous utilisons cette fonction pour créer une slice qui commence à `ptr` et fait `mid` éléments de long. Ensuite, nous appelons la méthode `add` sur `ptr` avec `mid` en argument pour obtenir un pointeur brut commençant à `mid`, et nous créons une slice utilisant ce pointeur et le nombre restant d'éléments après `mid` comme longueur.

<!--
The function `slice::from_raw_parts_mut` is unsafe because it takes a raw
pointer and must trust that this pointer is valid. The `add` method on raw
pointers is also unsafe because it must trust that the offset location is also
a valid pointer. Therefore, we had to put an `unsafe` block around our calls to
`slice::from_raw_parts_mut` and `add` so that we could call them. By looking at
the code and by adding the assertion that `mid` must be less than or equal to
`len`, we can tell that all the raw pointers used within the `unsafe` block
will be valid pointers to data within the slice. This is an acceptable and
appropriate use of `unsafe`.
-->

La fonction `slice::from_raw_parts_mut` est unsafe car elle prend un pointeur brut et doit faire confiance au fait que ce pointeur est valide. La méthode `add` sur les pointeurs bruts est aussi unsafe car elle doit faire confiance au fait que l'emplacement décalé est aussi un pointeur valide. C'est pourquoi nous avons dû placer un bloc `unsafe` autour de nos appels à `slice::from_raw_parts_mut` et `add` pour pouvoir les appeler. En examinant le code et en ajoutant l'assertion que `mid` doit être inférieur ou égal à `len`, nous pouvons affirmer que tous les pointeurs bruts utilisés dans le bloc `unsafe` seront des pointeurs valides vers des données dans la slice. C'est une utilisation acceptable et appropriée d'`unsafe`.

<!--
Note that we don't need to mark the resultant `split_at_mut` function as
`unsafe`, and we can call this function from safe Rust. We've created a safe
abstraction to the unsafe code with an implementation of the function that uses
`unsafe` code in a safe way, because it creates only valid pointers from the
data this function has access to.
-->

Notez que nous n'avons pas besoin de marquer la fonction résultante `split_at_mut` comme `unsafe`, et nous pouvons appeler cette fonction depuis du Rust safe. Nous avons créé une abstraction sûre pour le code unsafe avec une implémentation de la fonction qui utilise du code `unsafe` de manière sûre, car elle ne crée que des pointeurs valides à partir des données auxquelles cette fonction a accès.

<!--
In contrast, the use of `slice::from_raw_parts_mut` in Listing 20-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice 10,000 items long.
-->

En revanche, l'utilisation de `slice::from_raw_parts_mut` dans l'encart 20-7 provoquerait probablement un plantage lorsque la slice est utilisée. Ce code prend un emplacement mémoire arbitraire et crée une slice de 10 000 éléments.

<Listing number="20-7" caption="Créer une slice à partir d'un emplacement mémoire arbitraire">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

<!--
We don't own the memory at this arbitrary location, and there is no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`values` as though it's a valid slice results in undefined behavior.
-->

Nous ne possédons pas la mémoire à cet emplacement arbitraire, et il n'y a aucune garantie que la slice créée par ce code contienne des valeurs `i32` valides. Tenter d'utiliser `values` comme s'il s'agissait d'une slice valide entraîne un comportement indéfini.

<!--
#### Using `extern` Functions to Call External Code
-->

#### Utiliser les fonctions `extern` pour appeler du code externe

<!--
Sometimes your Rust code might need to interact with code written in another
language. For this, Rust has the keyword `extern` that facilitates the creation
and use of a _Foreign Function Interface (FFI)_, which is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.
-->

Parfois, votre code Rust peut avoir besoin d'interagir avec du code écrit dans un autre langage. Pour cela, Rust possède le mot-clé `extern` qui facilite la création et l'utilisation d'une _interface de fonctions étrangères (FFI)_, qui est un moyen pour un langage de programmation de définir des fonctions et de permettre à un autre langage de programmation (étranger) d'appeler ces fonctions.

<!--
Listing 20-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
generally unsafe to call from Rust code, so `extern` blocks must also be marked
`unsafe`. The reason is that other languages don't enforce Rust's rules and
guarantees, and Rust can't check them, so responsibility falls on the
programmer to ensure safety.
-->

L'encart 20-8 montre comment mettre en place une intégration avec la fonction `abs` de la bibliothèque standard C. Les fonctions déclarées dans les blocs `extern` sont généralement unsafe à appeler depuis du code Rust, donc les blocs `extern` doivent aussi être marqués `unsafe`. La raison est que les autres langages n'appliquent pas les règles et garanties de Rust, et Rust ne peut pas les vérifier, donc la responsabilité incombe au programmeur d'assurer la sécurité.

<Listing number="20-8" file-name="src/main.rs" caption="Déclarer et appeler une fonction `extern` définie dans un autre langage">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

<!--
Within the `unsafe extern "C"` block, we list the names and signatures of
external functions from another language we want to call. The `"C"` part
defines which _application binary interface (ABI)_ the external function uses:
The ABI defines how to call the function at the assembly level. The `"C"` ABI
is the most common and follows the C programming language's ABI. Information
about all the ABIs Rust supports is available in [the Rust Reference][ABI].
-->

Dans le bloc `unsafe extern "C"`, nous listons les noms et signatures des fonctions externes d'un autre langage que nous voulons appeler. La partie `"C"` définit quelle _interface binaire d'application (ABI)_ la fonction externe utilise : l'ABI définit comment appeler la fonction au niveau de l'assembleur. L'ABI `"C"` est la plus courante et suit l'ABI du langage de programmation C. Des informations sur toutes les ABI prises en charge par Rust sont disponibles dans [la Référence Rust][ABI].

<!--
Every item declared within an `unsafe extern` block is implicitly unsafe.
However, some FFI functions *are* safe to call. For example, the `abs` function
from C's standard library does not have any memory safety considerations, and we
know it can be called with any `i32`. In cases like this, we can use the `safe`
keyword to say that this specific function is safe to call even though it is in
an `unsafe extern` block. Once we make that change, calling it no longer
requires an `unsafe` block, as shown in Listing 20-9.
-->

Chaque élément déclaré dans un bloc `unsafe extern` est implicitement unsafe. Cependant, certaines fonctions FFI *sont* sûres à appeler. Par exemple, la fonction `abs` de la bibliothèque standard C n'a aucune considération de sécurité mémoire, et nous savons qu'elle peut être appelée avec n'importe quel `i32`. Dans de tels cas, nous pouvons utiliser le mot-clé `safe` pour indiquer que cette fonction spécifique est sûre à appeler même si elle se trouve dans un bloc `unsafe extern`. Une fois ce changement effectué, l'appeler ne nécessite plus de bloc `unsafe`, comme montré dans l'encart 20-9.

<Listing number="20-9" file-name="src/main.rs" caption="Marquer explicitement une fonction comme `safe` dans un bloc `unsafe extern` et l'appeler de manière sûre">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

<!--
Marking a function as `safe` does not inherently make it safe! Instead, it is
like a promise you are making to Rust that it is safe. It is still your
responsibility to make sure that promise is kept!
-->

Marquer une fonction comme `safe` ne la rend pas intrinsèquement sûre ! C'est plutôt une promesse que vous faites à Rust qu'elle est sûre. Il reste de votre responsabilité de vous assurer que cette promesse est tenue !

<!--
#### Calling Rust Functions from Other Languages
-->

#### Appeler des fonctions Rust depuis d'autres langages

<!--
We can also use `extern` to create an interface that allows other languages to
call Rust functions. Instead of creating a whole `extern` block, we add the
`extern` keyword and specify the ABI to use just before the `fn` keyword for
the relevant function. We also need to add an `#[unsafe(no_mangle)]` annotation
to tell the Rust compiler not to mangle the name of this function. _Mangling_
is when a compiler changes the name we've given a function to a different name
that contains more information for other parts of the compilation process to
consume but is less human readable. Every programming language compiler mangles
names slightly differently, so for a Rust function to be nameable by other
languages, we must disable the Rust compiler's name mangling. This is unsafe
because there might be name collisions across libraries without the built-in
mangling, so it is our responsibility to make sure the name we choose is safe
to export without mangling.
-->

Nous pouvons aussi utiliser `extern` pour créer une interface permettant à d'autres langages d'appeler des fonctions Rust. Au lieu de créer un bloc `extern` complet, nous ajoutons le mot-clé `extern` et spécifions l'ABI à utiliser juste avant le mot-clé `fn` de la fonction concernée. Nous devons aussi ajouter une annotation `#[unsafe(no_mangle)]` pour dire au compilateur Rust de ne pas modifier le nom de cette fonction. Le _name mangling_ est le processus par lequel un compilateur change le nom que nous avons donné à une fonction en un nom différent contenant plus d'informations pour d'autres parties du processus de compilation, mais moins lisible par l'humain. Chaque compilateur de langage de programmation modifie les noms légèrement différemment, donc pour qu'une fonction Rust soit nommable par d'autres langages, nous devons désactiver le name mangling du compilateur Rust. C'est unsafe car il pourrait y avoir des collisions de noms entre bibliothèques sans le mangling intégré, donc c'est notre responsabilité de s'assurer que le nom choisi peut être exporté sans mangling en toute sécurité.

<!--
In the following example, we make the `call_from_c` function accessible from C
code, after it's compiled to a shared library and linked from C:
-->

Dans l'exemple suivant, nous rendons la fonction `call_from_c` accessible depuis du code C, après compilation en bibliothèque partagée et liaison depuis C :

<!--
```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```
-->

```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

<!--
This usage of `extern` requires `unsafe` only in the attribute, not on the
`extern` block.
-->

Cette utilisation d'`extern` ne nécessite `unsafe` que dans l'attribut, pas sur le bloc `extern`.

<!--
### Accessing or Modifying a Mutable Static Variable
-->

### Accéder ou modifier une variable statique mutable

<!--
In this book, we've not yet talked about global variables, which Rust does
support but which can be problematic with Rust's ownership rules. If two
threads are accessing the same mutable global variable, it can cause a data
race.
-->

Dans ce livre, nous n'avons pas encore parlé des variables globales, que Rust prend en charge mais qui peuvent être problématiques avec les règles de possession de Rust. Si deux threads accèdent à la même variable globale mutable, cela peut provoquer une situation de compétition de données.

<!--
In Rust, global variables are called _static_ variables. Listing 20-10 shows an
example declaration and use of a static variable with a string slice as a
value.
-->

En Rust, les variables globales sont appelées variables _statiques_. L'encart 20-10 montre un exemple de déclaration et d'utilisation d'une variable statique avec une slice de chaîne de caractères comme valeur.

<Listing number="20-10" file-name="src/main.rs" caption="Définir et utiliser une variable statique immuable">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

<!--
Static variables are similar to constants, which we discussed in the
["Declaring Constants"][constants] ignore
--> section in Chapter 3. The
names of static variables are in `SCREAMING_SNAKE_CASE` by convention. Static
variables can only store references with the `'static` lifetime, which means
the Rust compiler can figure out the lifetime and we aren't required to
annotate it explicitly. Accessing an immutable static variable is safe.
-->

Les variables statiques sont similaires aux constantes, que nous avons abordées dans la section ["Déclarer des constantes"][constants]<!--
ignore
--> du chapitre 3. Les noms des variables statiques sont en `SCREAMING_SNAKE_CASE` par convention. Les variables statiques ne peuvent stocker que des références avec la durée de vie `'static`, ce qui signifie que le compilateur Rust peut déterminer la durée de vie et que nous n'avons pas besoin de l'annoter explicitement. Accéder à une variable statique immuable est sûr.

<!--
A subtle difference between constants and immutable static variables is that
values in a static variable have a fixed address in memory. Using the value
will always access the same data. Constants, on the other hand, are allowed to
duplicate their data whenever they're used. Another difference is that static
variables can be mutable. Accessing and modifying mutable static variables is
_unsafe_. Listing 20-11 shows how to declare, access, and modify a mutable
static variable named `COUNTER`.
-->

Une différence subtile entre les constantes et les variables statiques immuables est que les valeurs dans une variable statique ont une adresse fixe en mémoire. Utiliser la valeur accédera toujours aux mêmes données. Les constantes, en revanche, sont autorisées à dupliquer leurs données chaque fois qu'elles sont utilisées. Une autre différence est que les variables statiques peuvent être mutables. Accéder et modifier des variables statiques mutables est _unsafe_. L'encart 20-11 montre comment déclarer, accéder et modifier une variable statique mutable nommée `COUNTER`.

<Listing number="20-11" file-name="src/main.rs" caption="Lire ou écrire dans une variable statique mutable est unsafe.">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

<!--
As with regular variables, we specify mutability using the `mut` keyword. Any
code that reads or writes from `COUNTER` must be within an `unsafe` block. The
code in Listing 20-11 compiles and prints `COUNTER: 3` as we would expect
because it's single threaded. Having multiple threads access `COUNTER` would
likely result in data races, so it is undefined behavior. Therefore, we need to
mark the entire function as `unsafe` and document the safety limitation so that
anyone calling the function knows what they are and are not allowed to do
safely.
-->

Comme pour les variables normales, nous spécifions la mutabilité avec le mot-clé `mut`. Tout code qui lit ou écrit depuis `COUNTER` doit se trouver dans un bloc `unsafe`. Le code de l'encart 20-11 compile et affiche `COUNTER: 3` comme nous l'attendrions car il est mono-thread. Avoir plusieurs threads accédant à `COUNTER` entraînerait probablement des situations de compétition de données, ce qui est un comportement indéfini. Par conséquent, nous devons marquer la fonction entière comme `unsafe` et documenter la limitation de sécurité afin que quiconque appelle la fonction sache ce qu'il est autorisé ou non à faire de manière sûre.

<!--
Whenever we write an unsafe function, it is idiomatic to write a comment
starting with `SAFETY` and explaining what the caller needs to do to call the
function safely. Likewise, whenever we perform an unsafe operation, it is
idiomatic to write a comment starting with `SAFETY` to explain how the safety
rules are upheld.
-->

Chaque fois que nous écrivons une fonction unsafe, il est idiomatique d'écrire un commentaire commençant par `SAFETY` et expliquant ce que l'appelant doit faire pour appeler la fonction de manière sûre. De même, chaque fois que nous effectuons une opération unsafe, il est idiomatique d'écrire un commentaire commençant par `SAFETY` pour expliquer comment les règles de sécurité sont respectées.

<!--
Additionally, the compiler will deny by default any attempt to create
references to a mutable static variable through a compiler lint. You must
either explicitly opt out of that lint's protections by adding an
`#[allow(static_mut_refs)]` annotation or access the mutable static variable
via a raw pointer created with one of the raw borrow operators. That includes
cases where the reference is created invisibly, as when it is used in the
`println!` in this code listing. Requiring references to static mutable
variables to be created via raw pointers helps make the safety requirements for
using them more obvious.
-->

De plus, le compilateur refusera par défaut toute tentative de créer des références vers une variable statique mutable via un lint du compilateur. Vous devez soit explicitement désactiver les protections de ce lint en ajoutant une annotation `#[allow(static_mut_refs)]`, soit accéder à la variable statique mutable via un pointeur brut créé avec l'un des opérateurs d'emprunt brut. Cela inclut les cas où la référence est créée de manière invisible, comme lorsqu'elle est utilisée dans le `println!` de cet encart de code. Exiger que les références aux variables statiques mutables soient créées via des pointeurs bruts aide à rendre les exigences de sécurité pour leur utilisation plus évidentes.

<!--
With mutable data that is globally accessible, it's difficult to ensure that
there are no data races, which is why Rust considers mutable static variables
to be unsafe. Where possible, it's preferable to use the concurrency techniques
and thread-safe smart pointers we discussed in Chapter 16 so that the compiler
checks that data access from different threads is done safely.
-->

Avec des données mutables accessibles globalement, il est difficile de s'assurer qu'il n'y a pas de situations de compétition de données, c'est pourquoi Rust considère les variables statiques mutables comme unsafe. Dans la mesure du possible, il est préférable d'utiliser les techniques de concurrence et les pointeurs intelligents thread-safe que nous avons vus au chapitre 16 afin que le compilateur vérifie que l'accès aux données depuis différents threads se fait de manière sûre.

<!--
### Implementing an Unsafe Trait
-->

### Implémenter un trait unsafe

<!--
We can use `unsafe` to implement an unsafe trait. A trait is unsafe when at
least one of its methods has some invariant that the compiler can't verify. We
declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait`
and marking the implementation of the trait as `unsafe` too, as shown in
Listing 20-12.
-->

Nous pouvons utiliser `unsafe` pour implémenter un trait unsafe. Un trait est unsafe lorsqu'au moins une de ses méthodes possède un invariant que le compilateur ne peut pas vérifier. Nous déclarons qu'un trait est `unsafe` en ajoutant le mot-clé `unsafe` avant `trait` et en marquant l'implémentation du trait comme `unsafe` également, comme montré dans l'encart 20-12.

<Listing number="20-12" caption="Définir et implémenter un trait unsafe">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

<!--
By using `unsafe impl`, we're promising that we'll uphold the invariants that
the compiler can't verify.
-->

En utilisant `unsafe impl`, nous promettons de respecter les invariants que le compilateur ne peut pas vérifier.

<!--
As an example, recall the `Send` and `Sync` marker traits we discussed in the
["Extensible Concurrency with `Send` and `Sync`"][send-and-sync] ignore
-->
section in Chapter 16: The compiler implements these traits automatically if
our types are composed entirely of other types that implement `Send` and
`Sync`. If we implement a type that contains a type that does not implement
`Send` or `Sync`, such as raw pointers, and we want to mark that type as `Send`
or `Sync`, we must use `unsafe`. Rust can't verify that our type upholds the
guarantees that it can be safely sent across threads or accessed from multiple
threads; therefore, we need to do those checks manually and indicate as such
with `unsafe`.
-->

À titre d'exemple, rappelez-vous les traits marqueurs `Send` et `Sync` que nous avons vus dans la section ["La concurrence extensible avec `Send` et `Sync`"][send-and-sync]<!--
ignore
--> du chapitre 16 : le compilateur implémente ces traits automatiquement si nos types sont entièrement composés d'autres types qui implémentent `Send` et `Sync`. Si nous implémentons un type contenant un type qui n'implémente pas `Send` ou `Sync`, comme les pointeurs bruts, et que nous voulons marquer ce type comme `Send` ou `Sync`, nous devons utiliser `unsafe`. Rust ne peut pas vérifier que notre type respecte les garanties qu'il peut être envoyé entre les threads de manière sûre ou accédé depuis plusieurs threads ; par conséquent, nous devons effectuer ces vérifications manuellement et l'indiquer avec `unsafe`.

<!--
### Accessing Fields of a Union
-->

### Accéder aux champs d'une union

<!--
The final action that works only with `unsafe` is accessing fields of a union.
A *union* is similar to a `struct`, but only one declared field is used in a
particular instance at one time. Unions are primarily used to interface with
unions in C code. Accessing union fields is unsafe because Rust can't guarantee
the type of the data currently being stored in the union instance. You can
learn more about unions in [the Rust Reference][unions].
-->

La dernière action qui ne fonctionne qu'avec `unsafe` est l'accès aux champs d'une union. Une *union* est similaire à une `struct`, mais un seul champ déclaré est utilisé dans une instance particulière à un moment donné. Les unions sont principalement utilisées pour interfacer avec les unions du code C. L'accès aux champs d'une union est unsafe car Rust ne peut pas garantir le type des données actuellement stockées dans l'instance de l'union. Vous pouvez en apprendre plus sur les unions dans [la Référence Rust][unions].

<!--
### Using Miri to Check Unsafe Code
-->

### Utiliser Miri pour vérifier le code unsafe

<!--
When writing unsafe code, you might want to check that what you have written
actually is safe and correct. One of the best ways to do that is to use Miri,
an official Rust tool for detecting undefined behavior. Whereas the borrow
checker is a _static_ tool that works at compile time, Miri is a _dynamic_
tool that works at runtime. It checks your code by running your program, or
its test suite, and detecting when you violate the rules it understands about
how Rust should work.
-->

Lorsque vous écrivez du code unsafe, vous voudrez peut-être vérifier que ce que vous avez écrit est réellement sûr et correct. L'un des meilleurs moyens de le faire est d'utiliser Miri, un outil officiel de Rust pour détecter les comportements indéfinis. Alors que le vérificateur d'emprunt est un outil _statique_ qui fonctionne au moment de la compilation, Miri est un outil _dynamique_ qui fonctionne à l'exécution. Il vérifie votre code en exécutant votre programme, ou sa suite de tests, et en détectant quand vous violez les règles qu'il comprend sur le fonctionnement de Rust.

<!--
Using Miri requires a nightly build of Rust (which we talk about more in
[Appendix G: How Rust is Made and "Nightly Rust"][nightly] ignore
-->). You
can install both a nightly version of Rust and the Miri tool by typing `rustup
+nightly component add miri`. This does not change what version of Rust your
project uses; it only adds the tool to your system so you can use it when you
want to. You can run Miri on a project by typing `cargo +nightly miri run` or
`cargo +nightly miri test`.
-->

L'utilisation de Miri nécessite une version nightly de Rust (dont nous parlons davantage dans [l'Annexe G : Comment Rust est fait et "Nightly Rust"][nightly]<!--
ignore
-->). Vous pouvez installer à la fois une version nightly de Rust et l'outil Miri en tapant `rustup +nightly component add miri`. Cela ne change pas la version de Rust utilisée par votre projet ; cela ajoute seulement l'outil à votre système pour que vous puissiez l'utiliser quand vous le souhaitez. Vous pouvez exécuter Miri sur un projet en tapant `cargo +nightly miri run` ou `cargo +nightly miri test`.

<!--
For an example of how helpful this can be, consider what happens when we run it
against Listing 20-7.
-->

Pour un exemple de l'utilité de cet outil, voyons ce qui se passe lorsque nous l'exécutons sur l'encart 20-7.


```console
{{#include ../listings/ch20-advanced-features/listing-20-07/output.txt}}
```

<!--
Miri correctly warns us that we're casting an integer to a pointer, which might
be a problem, but Miri can't determine whether a problem exists because it
doesn't know how the pointer originated. Then, Miri returns an error where
Listing 20-7 has undefined behavior because we have a dangling pointer. Thanks
to Miri, we now know there is a risk of undefined behavior, and we can think
about how to make the code safe. In some cases, Miri can even make
recommendations about how to fix errors.
-->

Miri nous avertit correctement que nous convertissons un entier en pointeur, ce qui pourrait être un problème, mais Miri ne peut pas déterminer si un problème existe car il ne sait pas d'où provient le pointeur. Ensuite, Miri renvoie une erreur là où l'encart 20-7 a un comportement indéfini car nous avons un pointeur pendant. Grâce à Miri, nous savons maintenant qu'il y a un risque de comportement indéfini, et nous pouvons réfléchir à comment rendre le code sûr. Dans certains cas, Miri peut même faire des recommandations sur la façon de corriger les erreurs.

<!--
Miri doesn't catch everything you might get wrong when writing unsafe code.
Miri is a dynamic analysis tool, so it only catches problems with code that
actually gets run. That means you will need to use it in conjunction with good
testing techniques to increase your confidence about the unsafe code you have
written. Miri also does not cover every possible way your code can be unsound.
-->

Miri n'attrape pas tout ce que vous pourriez mal faire en écrivant du code unsafe. Miri est un outil d'analyse dynamique, donc il ne détecte que les problèmes avec du code qui est réellement exécuté. Cela signifie que vous devrez l'utiliser en conjonction avec de bonnes techniques de test pour augmenter votre confiance dans le code unsafe que vous avez écrit. Miri ne couvre pas non plus toutes les manières possibles dont votre code peut être incorrect.

<!--
Put another way: If Miri _does_ catch a problem, you know there's a bug, but
just because Miri _doesn't_ catch a bug doesn't mean there isn't a problem. It
can catch a lot, though. Try running it on the other examples of unsafe code in
this chapter and see what it says!
-->

Autrement dit : si Miri _détecte_ un problème, vous savez qu'il y a un bogue, mais ce n'est pas parce que Miri _ne détecte pas_ un bogue qu'il n'y a pas de problème. Il peut cependant en attraper beaucoup. Essayez de l'exécuter sur les autres exemples de code unsafe de ce chapitre et voyez ce qu'il dit !

<!--
You can learn more about Miri at [its GitHub repository][miri].
-->

Vous pouvez en apprendre plus sur Miri dans [son dépôt GitHub][miri].

<!--
Old headings. Do not remove or links may break.
-->

<a id="when-to-use-unsafe-code"></a>

<!--
### Using Unsafe Code Correctly
-->

### Utiliser le code unsafe correctement

<!--
Using `unsafe` to use one of the five superpowers just discussed isn't wrong or
even frowned upon, but it is trickier to get `unsafe` code correct because the
compiler can't help uphold memory safety. When you have a reason to use
`unsafe` code, you can do so, and having the explicit `unsafe` annotation makes
it easier to track down the source of problems when they occur. Whenever you
write unsafe code, you can use Miri to help you be more confident that the code
you have written upholds Rust's rules.
-->

Utiliser `unsafe` pour utiliser l'un des cinq super-pouvoirs que nous venons de voir n'est ni incorrect ni mal vu, mais il est plus délicat d'obtenir un code `unsafe` correct car le compilateur ne peut pas aider à maintenir la sécurité mémoire. Lorsque vous avez une raison d'utiliser du code `unsafe`, vous pouvez le faire, et avoir l'annotation explicite `unsafe` facilite la recherche de la source des problèmes lorsqu'ils surviennent. Chaque fois que vous écrivez du code unsafe, vous pouvez utiliser Miri pour vous aider à être plus confiant que le code que vous avez écrit respecte les règles de Rust.

<!--
For a much deeper exploration of how to work effectively with unsafe Rust, read
Rust's official guide for `unsafe`, [The Rustonomicon][nomicon].
-->

Pour une exploration beaucoup plus approfondie de la manière de travailler efficacement avec le Rust unsafe, lisez le guide officiel de Rust pour `unsafe`, [le Rustonomicon][nomicon].

[dangling-references]: ch04-02-references-and-borrowing.html#dangling-references
[ABI]: ../reference/items/external-blocks.html#abi
[constants]: ch03-01-variables-and-mutability.html#declaring-constants
[send-and-sync]: ch16-04-extensible-concurrency-sync-and-send.html
[the-slice-type]: ch04-03-slices.html#the-slice-type
[unions]: ../reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: appendix-05-editions.html
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
