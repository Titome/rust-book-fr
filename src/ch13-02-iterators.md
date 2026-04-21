<!--
## Processing a Series of Items with Iterators
-->

## Traiter une serie d'elements avec les iterateurs

<!--
The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don't
have to reimplement that logic yourself.
-->

Le patron de conception iterateur vous permet d'effectuer une tache sur une sequence d'elements tour a tour. Un iterateur est responsable de la logique d'iteration sur chaque element et de la determination de la fin de la sequence. Lorsque vous utilisez des iterateurs, vous n'avez pas a reimplementer cette logique vous-meme.

<!--
In Rust, iterators are _lazy_, meaning they have no effect until you call
methods that consume the iterator to use it up. For example, the code in
Listing 13-10 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn't do anything
useful.
-->

En Rust, les iterateurs sont _paresseux_ (lazy), ce qui signifie qu'ils n'ont aucun effet tant que vous n'appelez pas de methodes qui consomment l'iterateur pour l'utiliser. Par exemple, le code de l'encart 13-10 cree un iterateur sur les elements du vecteur `v1` en appelant la methode `iter` definie sur `Vec<T>`. Ce code en lui-meme ne fait rien d'utile.

<Listing number="13-10" file-name="src/main.rs" caption="Creation d'un iterateur">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

<!--
The iterator is stored in the `v1_iter` variable. Once we've created an
iterator, we can use it in a variety of ways. In Listing 3-5, we iterated over
an array using a `for` loop to execute some code on each of its items. Under
the hood, this implicitly created and then consumed an iterator, but we glossed
over how exactly that works until now.
-->

L'iterateur est stocke dans la variable `v1_iter`. Une fois que nous avons cree un iterateur, nous pouvons l'utiliser de diverses facons. Dans l'encart 3-5, nous avons itere sur un tableau en utilisant une boucle `for` pour executer du code sur chacun de ses elements. Sous le capot, cela a implicitement cree puis consomme un iterateur, mais nous avons passe sous silence le fonctionnement exact de ce mecanisme jusqu'a maintenant.

<!--
In the example in Listing 13-11, we separate the creation of the iterator from
the use of the iterator in the `for` loop. When the `for` loop is called using
the iterator in `v1_iter`, each element in the iterator is used in one
iteration of the loop, which prints out each value.
-->

Dans l'exemple de l'encart 13-11, nous separons la creation de l'iterateur de l'utilisation de l'iterateur dans la boucle `for`. Lorsque la boucle `for` est appelee en utilisant l'iterateur dans `v1_iter`, chaque element de l'iterateur est utilise dans une iteration de la boucle, ce qui affiche chaque valeur.

<Listing number="13-11" file-name="src/main.rs" caption="Utilisation d'un iterateur dans une boucle `for`">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

<!--
In languages that don't have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
items in the vector.
-->

Dans les langages dont les bibliotheques standard ne fournissent pas d'iterateurs, vous ecririez probablement cette meme fonctionnalite en initialisant une variable a l'index 0, en utilisant cette variable pour indexer le vecteur et obtenir une valeur, puis en incrementant la valeur de la variable dans une boucle jusqu'a atteindre le nombre total d'elements du vecteur.

<!--
Iterators handle all of that logic for you, cutting down on repetitive code you
could potentially mess up. Iterators give you more flexibility to use the same
logic with many different kinds of sequences, not just data structures you can
index into, like vectors. Let's examine how iterators do that.
-->

Les iterateurs gerent toute cette logique pour vous, reduisant le code repetitif que vous pourriez potentiellement mal ecrire. Les iterateurs vous donnent plus de flexibilite pour utiliser la meme logique avec de nombreux types differents de sequences, pas seulement des structures de donnees que vous pouvez indexer, comme les vecteurs. Examinons comment les iterateurs font cela.

<!--
### The `Iterator` Trait and the `next` Method
-->

### Le trait `Iterator` et la methode `next`

<!--
All iterators implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:
-->

Tous les iterateurs implementent un trait nomme `Iterator` qui est defini dans la bibliotheque standard. La definition du trait ressemble a ceci :

<!--
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
-->

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // les methodes avec des implementations par defaut sont omises
}
```

<!--
Notice that this definition uses some new syntax: `type Item` and `Self::Item`,
which are defining an associated type with this trait. We'll talk about
associated types in depth in Chapter 20. For now, all you need to know is that
this code says implementing the `Iterator` trait requires that you also define
an `Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type returned from the
iterator.
-->

Remarquez que cette definition utilise une syntaxe nouvelle : `type Item` et `Self::Item`, qui definissent un type associe a ce trait. Nous parlerons des types associes en detail dans le chapitre 20. Pour l'instant, tout ce que vous devez savoir est que ce code dit qu'implementer le trait `Iterator` necessite que vous definissiez egalement un type `Item`, et ce type `Item` est utilise dans le type de retour de la methode `next`. En d'autres termes, le type `Item` sera le type retourne par l'iterateur.

<!--
The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time, wrapped in
`Some`, and, when iteration is over, returns `None`.
-->

Le trait `Iterator` n'exige des implementeurs que la definition d'une seule methode : la methode `next`, qui retourne un element de l'iterateur a la fois, enveloppe dans `Some`, et, lorsque l'iteration est terminee, retourne `None`.

<!--
We can call the `next` method on iterators directly; Listing 13-12 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.
-->

Nous pouvons appeler la methode `next` directement sur les iterateurs ; l'encart 13-12 montre quelles valeurs sont retournees par des appels repetes a `next` sur l'iterateur cree a partir du vecteur.

<Listing number="13-12" file-name="src/lib.rs" caption="Appel de la methode `next` sur un iterateur">


```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

<!--
Note that we needed to make `v1_iter` mutable: Calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code _consumes_, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn't need
to make `v1_iter` mutable when we used a `for` loop, because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.
-->

Notez que nous avons du rendre `v1_iter` mutable : appeler la methode `next` sur un iterateur modifie l'etat interne que l'iterateur utilise pour garder une trace de sa position dans la sequence. En d'autres termes, ce code _consomme_, ou epuise, l'iterateur. Chaque appel a `next` consomme un element de l'iterateur. Nous n'avions pas besoin de rendre `v1_iter` mutable lorsque nous utilisions une boucle `for`, car la boucle a pris la possession de `v1_iter` et l'a rendue mutable en coulisses.

<!--
Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.
-->

Notez egalement que les valeurs que nous obtenons des appels a `next` sont des references immuables vers les valeurs du vecteur. La methode `iter` produit un iterateur sur des references immuables. Si nous voulons creer un iterateur qui prend la possession de `v1` et retourne des valeurs possedees, nous pouvons appeler `into_iter` au lieu de `iter`. De meme, si nous voulons iterer sur des references mutables, nous pouvons appeler `iter_mut` au lieu de `iter`.

<!--
### Methods That Consume the Iterator
-->

### Les methodes qui consomment l'iterateur

<!--
The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you're required to implement the `next` method when implementing the
`Iterator` trait.
-->

Le trait `Iterator` possede un certain nombre de methodes differentes avec des implementations par defaut fournies par la bibliotheque standard ; vous pouvez decouvrir ces methodes en consultant la documentation de l'API de la bibliotheque standard pour le trait `Iterator`. Certaines de ces methodes appellent la methode `next` dans leur definition, c'est pourquoi vous devez implementer la methode `next` lorsque vous implementez le trait `Iterator`.

<!--
Methods that call `next` are called _consuming adapters_ because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-13 has a
test illustrating a use of the `sum` method.
-->

Les methodes qui appellent `next` sont appelees _adaptateurs consommateurs_ car les appeler consomme l'iterateur. Un exemple est la methode `sum`, qui prend la possession de l'iterateur et parcourt les elements en appelant `next` de maniere repetee, consommant ainsi l'iterateur. Au fur et a mesure qu'elle itere, elle ajoute chaque element a un total cumulatif et retourne le total lorsque l'iteration est terminee. L'encart 13-13 contient un test illustrant l'utilisation de la methode `sum`.

<Listing number="13-13" file-name="src/lib.rs" caption="Appel de la methode `sum` pour obtenir le total de tous les elements de l'iterateur">


```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

<!--
We aren't allowed to use `v1_iter` after the call to `sum`, because `sum` takes
ownership of the iterator we call it on.
-->

Nous n'avons pas le droit d'utiliser `v1_iter` apres l'appel a `sum`, car `sum` prend la possession de l'iterateur sur lequel nous l'appelons.

<!--
### Methods That Produce Other Iterators
-->

### Les methodes qui produisent d'autres iterateurs

<!--
_Iterator adapters_ are methods defined on the `Iterator` trait that don't
consume the iterator. Instead, they produce different iterators by changing
some aspect of the original iterator.
-->

Les _adaptateurs d'iterateurs_ sont des methodes definies sur le trait `Iterator` qui ne consomment pas l'iterateur. Au lieu de cela, ils produisent des iterateurs differents en modifiant un aspect de l'iterateur original.

<!--
Listing 13-14 shows an example of calling the iterator adapter method `map`,
which takes a closure to call on each item as the items are iterated through.
The `map` method returns a new iterator that produces the modified items. The
closure here creates a new iterator in which each item from the vector will be
incremented by 1.
-->

L'encart 13-14 montre un exemple d'appel de la methode d'adaptateur d'iterateur `map`, qui prend une fermeture a appeler sur chaque element au fur et a mesure que les elements sont parcourus. La methode `map` retourne un nouvel iterateur qui produit les elements modifies. La fermeture ici cree un nouvel iterateur dans lequel chaque element du vecteur sera incremente de 1.

<Listing number="13-14" file-name="src/main.rs" caption="Appel de l'adaptateur d'iterateur `map` pour creer un nouvel iterateur">


```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

<!--
However, this code produces a warning:
-->

Cependant, ce code produit un avertissement :


```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

<!--
The code in Listing 13-14 doesn't do anything; the closure we've specified
never gets called. The warning reminds us why: Iterator adapters are lazy, and
we need to consume the iterator here.
-->

Le code de l'encart 13-14 ne fait rien ; la fermeture que nous avons specifiee n'est jamais appelee. L'avertissement nous rappelle pourquoi : les adaptateurs d'iterateurs sont paresseux, et nous devons consommer l'iterateur ici.

<!--
To fix this warning and consume the iterator, we'll use the `collect` method,
which we used with `env::args` in Listing 12-1. This method consumes the
iterator and collects the resultant values into a collection data type.
-->

Pour corriger cet avertissement et consommer l'iterateur, nous utiliserons la methode `collect`, que nous avons utilisee avec `env::args` dans l'encart 12-1. Cette methode consomme l'iterateur et collecte les valeurs resultantes dans un type de donnees de collection.

<!--
In Listing 13-15, we collect the results of iterating over the iterator that's
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector, incremented by 1.
-->

Dans l'encart 13-15, nous collectons les resultats de l'iteration sur l'iterateur retourne par l'appel a `map` dans un vecteur. Ce vecteur finira par contenir chaque element du vecteur original, incremente de 1.

<Listing number="13-15" file-name="src/main.rs" caption="Appel de la methode `map` pour creer un nouvel iterateur, puis appel de la methode `collect` pour consommer le nouvel iterateur et creer un vecteur">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

<!--
Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.
-->

Parce que `map` prend une fermeture, nous pouvons specifier n'importe quelle operation que nous voulons effectuer sur chaque element. C'est un excellent exemple de la facon dont les fermetures vous permettent de personnaliser un comportement tout en reutilisant le comportement d'iteration que le trait `Iterator` fournit.

<!--
You can chain multiple calls to iterator adapters to perform complex actions in
a readable way. But because all iterators are lazy, you have to call one of the
consuming adapter methods to get results from calls to iterator adapters.
-->

Vous pouvez enchainer plusieurs appels a des adaptateurs d'iterateurs pour effectuer des actions complexes de maniere lisible. Mais parce que tous les iterateurs sont paresseux, vous devez appeler l'une des methodes d'adaptateur consommateur pour obtenir des resultats a partir des appels aux adaptateurs d'iterateurs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-closures-that-capture-their-environment"></a>

<!--
### Closures That Capture Their Environment
-->

### Les fermetures qui capturent leur environnement

<!--
Many iterator adapters take closures as arguments, and commonly the closures
we'll specify as arguments to iterator adapters will be closures that capture
their environment.
-->

De nombreux adaptateurs d'iterateurs prennent des fermetures en arguments, et generalement les fermetures que nous specifierons comme arguments aux adaptateurs d'iterateurs seront des fermetures qui capturent leur environnement.

<!--
For this example, we'll use the `filter` method that takes a closure. The
closure gets an item from the iterator and returns a `bool`. If the closure
returns `true`, the value will be included in the iteration produced by
`filter`. If the closure returns `false`, the value won't be included.
-->

Pour cet exemple, nous utiliserons la methode `filter` qui prend une fermeture. La fermeture recoit un element de l'iterateur et retourne un `bool`. Si la fermeture retourne `true`, la valeur sera incluse dans l'iteration produite par `filter`. Si la fermeture retourne `false`, la valeur ne sera pas incluse.

<!--
In Listing 13-16, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.
-->

Dans l'encart 13-16, nous utilisons `filter` avec une fermeture qui capture la variable `shoe_size` de son environnement pour iterer sur une collection d'instances de la struct `Shoe`. Elle ne retournera que les chaussures de la taille specifiee.

<Listing number="13-16" file-name="src/lib.rs" caption="Utilisation de la methode `filter` avec une fermeture qui capture `shoe_size`">


```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

<!--
The `shoes_in_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.
-->

La fonction `shoes_in_size` prend la possession d'un vecteur de chaussures et une taille de chaussure en parametres. Elle retourne un vecteur contenant uniquement les chaussures de la taille specifiee.

<!--
In the body of `shoes_in_size`, we call `into_iter` to create an iterator that
takes ownership of the vector. Then, we call `filter` to adapt that iterator
into a new iterator that only contains elements for which the closure returns
`true`.
-->

Dans le corps de `shoes_in_size`, nous appelons `into_iter` pour creer un iterateur qui prend la possession du vecteur. Ensuite, nous appelons `filter` pour adapter cet iterateur en un nouvel iterateur qui ne contient que les elements pour lesquels la fermeture retourne `true`.

<!--
The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe's size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that's returned by the function.
-->

La fermeture capture le parametre `shoe_size` de l'environnement et compare la valeur avec la taille de chaque chaussure, ne gardant que les chaussures de la taille specifiee. Enfin, l'appel a `collect` rassemble les valeurs retournees par l'iterateur adapte dans un vecteur qui est retourne par la fonction.

<!--
The test shows that when we call `shoes_in_size`, we get back only shoes that
have the same size as the value we specified.
-->

Le test montre que lorsque nous appelons `shoes_in_size`, nous ne recuperons que les chaussures qui ont la meme taille que la valeur que nous avons specifiee.
