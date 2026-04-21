<!--
# Smart Pointers
-->

# Les pointeurs intelligents

<!--
A pointer is a general concept for a variable that contains an address in
memory. This address refers to, or "points at," some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don't have any special capabilities other than referring to
data, and they have no overhead.
-->

Un pointeur est un concept général pour une variable qui contient une adresse en mémoire. Cette adresse fait référence, ou "pointe vers", d'autres données. Le type de pointeur le plus courant en Rust est une référence, que vous avez découverte au chapitre 4. Les références sont indiquées par le symbole `&` et empruntent la valeur vers laquelle elles pointent. Elles n'ont pas de capacités particulières autres que de faire référence à des données, et elles n'ont aucun surcoût.

<!--
_Smart pointers_, on the other hand, are data structures that act like a
pointer but also have additional metadata and capabilities. The concept of
smart pointers isn't unique to Rust: Smart pointers originated in C++ and exist
in other languages as well. Rust has a variety of smart pointers defined in the
standard library that provide functionality beyond that provided by references.
To explore the general concept, we'll look at a couple of different examples of
smart pointers, including a _reference counting_ smart pointer type. This
pointer enables you to allow data to have multiple owners by keeping track of
the number of owners and, when no owners remain, cleaning up the data.
-->

Les _pointeurs intelligents_ (smart pointers), en revanche, sont des structures de données qui agissent comme un pointeur mais possèdent également des métadonnées et des capacités supplémentaires. Le concept de pointeurs intelligents n'est pas propre à Rust : les pointeurs intelligents sont apparus en C++ et existent également dans d'autres langages. Rust dispose d'une variété de pointeurs intelligents définis dans la bibliothèque standard qui fournissent des fonctionnalités au-delà de celles offertes par les références. Pour explorer le concept général, nous examinerons quelques exemples différents de pointeurs intelligents, notamment un type de pointeur intelligent à _comptage de références_. Ce pointeur vous permet de donner plusieurs propriétaires à des données en suivant le nombre de propriétaires et, lorsqu'il n'en reste plus aucun, en nettoyant les données.

<!--
In Rust, with its concept of ownership and borrowing, there is an additional
difference between references and smart pointers: While references only borrow
data, in many cases smart pointers _own_ the data they point to.
-->

En Rust, avec son concept de possession et d'emprunt, il existe une différence supplémentaire entre les références et les pointeurs intelligents : alors que les références ne font qu'emprunter des données, dans de nombreux cas les pointeurs intelligents _possèdent_ les données vers lesquelles ils pointent.

<!--
Smart pointers are usually implemented using structs. Unlike an ordinary
struct, smart pointers implement the `Deref` and `Drop` traits. The `Deref`
trait allows an instance of the smart pointer struct to behave like a reference
so that you can write your code to work with either references or smart
pointers. The `Drop` trait allows you to customize the code that's run when an
instance of the smart pointer goes out of scope. In this chapter, we'll discuss
both of these traits and demonstrate why they're important to smart pointers.
-->

Les pointeurs intelligents sont généralement implémentés à l'aide de structs. Contrairement à une struct ordinaire, les pointeurs intelligents implémentent les traits `Deref` et `Drop`. Le trait `Deref` permet à une instance de la struct du pointeur intelligent de se comporter comme une référence afin que vous puissiez écrire du code qui fonctionne avec des références ou des pointeurs intelligents. Le trait `Drop` vous permet de personnaliser le code qui s'exécute lorsqu'une instance du pointeur intelligent sort de la portée. Dans ce chapitre, nous aborderons ces deux traits et montrerons pourquoi ils sont importants pour les pointeurs intelligents.

<!--
Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won't cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We'll
cover the most common smart pointers in the standard library:
-->

Étant donné que le patron de pointeur intelligent est un patron de conception général fréquemment utilisé en Rust, ce chapitre ne couvrira pas tous les pointeurs intelligents existants. De nombreuses bibliothèques ont leurs propres pointeurs intelligents, et vous pouvez même écrire les vôtres. Nous couvrirons les pointeurs intelligents les plus courants de la bibliothèque standard :

<!--
- `Box<T>`, for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time
-->

- `Box<T>`, pour allouer des valeurs sur le tas
- `Rc<T>`, un type à comptage de références qui permet la possession multiple
- `Ref<T>` et `RefMut<T>`, accessibles via `RefCell<T>`, un type qui applique
  les règles d'emprunt à l'exécution plutôt qu'à la compilation

<!--
In addition, we'll cover the _interior mutability_ pattern where an immutable
type exposes an API for mutating an interior value. We'll also discuss
reference cycles: how they can leak memory and how to prevent them.
-->

De plus, nous couvrirons le patron de _mutabilité intérieure_ où un type immuable expose une API pour modifier une valeur intérieure. Nous aborderons également les cycles de références : comment ils peuvent provoquer des fuites de mémoire et comment les éviter.

<!--
Let's dive in!
-->

Plongeons-nous dans le sujet !
