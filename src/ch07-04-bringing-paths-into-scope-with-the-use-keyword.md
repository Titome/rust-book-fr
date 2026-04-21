<!--
## Bringing Paths into Scope with the `use` Keyword
-->

## Amener des chemins dans la portée avec le mot-clé `use`

<!--
Having to write out the paths to call functions can feel inconvenient and
repetitive. In Listing 7-7, whether we chose the absolute or relative path to
the `add_to_waitlist` function, every time we wanted to call `add_to_waitlist`
we had to specify `front_of_house` and `hosting` too. Fortunately, there's a
way to simplify this process: We can create a shortcut to a path with the `use`
keyword once and then use the shorter name everywhere else in the scope.
-->

Devoir écrire les chemins pour appeler des fonctions peut sembler peu pratique
et répétitif. Dans le listing 7-7, que nous ayons choisi le chemin absolu ou
relatif vers la fonction `add_to_waitlist`, chaque fois que nous voulions
appeler `add_to_waitlist`, nous devions aussi spécifier `front_of_house` et
`hosting`. Heureusement, il existe un moyen de simplifier ce processus : nous
pouvons créer un raccourci vers un chemin avec le mot-clé `use` une seule fois,
puis utiliser le nom plus court partout ailleurs dans la portée.

<!--
In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the
scope of the `eat_at_restaurant` function so that we only have to specify
`hosting::add_to_waitlist` to call the `add_to_waitlist` function in
`eat_at_restaurant`.
-->

Dans le listing 7-11, nous amenons le module `crate::front_of_house::hosting`
dans la portée de la fonction `eat_at_restaurant` afin de n'avoir qu'à spécifier
`hosting::add_to_waitlist` pour appeler la fonction `add_to_waitlist` dans
`eat_at_restaurant`.


<Listing number="7-11" file-name="src/lib.rs" caption="Amener un module dans la portée avec `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

<!--
Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::front_of_house::hosting` in the crate
root, `hosting` is now a valid name in that scope, just as though the `hosting`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.
-->

Ajouter `use` et un chemin dans une portée est similaire à la création d'un lien
symbolique dans le système de fichiers. En ajoutant
`use crate::front_of_house::hosting` à la racine de la crate, `hosting` est
désormais un nom valide dans cette portée, comme si le module `hosting` avait
été défini à la racine de la crate. Les chemins amenés dans la portée avec `use`
vérifient également la confidentialité, comme tout autre chemin.

<!--
Note that `use` only creates the shortcut for the particular scope in which the
`use` occurs. Listing 7-12 moves the `eat_at_restaurant` function into a new
child module named `customer`, which is then a different scope than the `use`
statement, so the function body won't compile.
-->

Notez que `use` ne crée le raccourci que pour la portée particulière dans
laquelle le `use` se trouve. Le listing 7-12 déplace la fonction
`eat_at_restaurant` dans un nouveau module enfant nommé `customer`, qui est
alors une portée différente de celle de l'instruction `use`, donc le corps de la
fonction ne compilera pas.


<Listing number="7-12" file-name="src/lib.rs" caption="Une instruction `use` ne s'applique que dans la portée dans laquelle elle se trouve.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

<!--
The compiler error shows that the shortcut no longer applies within the
`customer` module:
-->

L'erreur du compilateur montre que le raccourci ne s'applique plus au sein du
module `customer` :


```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

<!--
Notice there's also a warning that the `use` is no longer used in its scope! To
fix this problem, move the `use` within the `customer` module too, or reference
the shortcut in the parent module with `super::hosting` within the child
`customer` module.
-->

Remarquez qu'il y a aussi un avertissement indiquant que le `use` n'est plus
utilisé dans sa portée ! Pour corriger ce problème, déplacez le `use` dans le
module `customer` également, ou faites référence au raccourci du module parent
avec `super::hosting` dans le module enfant `customer`.

<!--
### Creating Idiomatic `use` Paths
-->

### Créer des chemins `use` idiomatiques

<!--
In Listing 7-11, you might have wondered why we specified `use
crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in
`eat_at_restaurant`, rather than specifying the `use` path all the way out to
the `add_to_waitlist` function to achieve the same result, as in Listing 7-13.
-->

Dans le listing 7-11, vous vous êtes peut-être demandé pourquoi nous avons
spécifié `use crate::front_of_house::hosting` puis appelé
`hosting::add_to_waitlist` dans `eat_at_restaurant`, plutôt que de spécifier le
chemin `use` jusqu'à la fonction `add_to_waitlist` pour obtenir le même
résultat, comme dans le listing 7-13.


<Listing number="7-13" file-name="src/lib.rs" caption="Amener la fonction `add_to_waitlist` dans la portée avec `use`, ce qui n'est pas idiomatique">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

<!--
Although both Listing 7-11 and Listing 7-13 accomplish the same task, Listing
7-11 is the idiomatic way to bring a function into scope with `use`. Bringing
the function's parent module into scope with `use` means we have to specify the
parent module when calling the function. Specifying the parent module when
calling the function makes it clear that the function isn't locally defined
while still minimizing repetition of the full path. The code in Listing 7-13 is
unclear as to where `add_to_waitlist` is defined.
-->

Bien que le listing 7-11 et le listing 7-13 accomplissent la même tâche, le
listing 7-11 est la manière idiomatique d'amener une fonction dans la portée
avec `use`. Amener le module parent de la fonction dans la portée avec `use`
signifie que nous devons spécifier le module parent lors de l'appel de la
fonction. Spécifier le module parent lors de l'appel de la fonction rend clair
que la fonction n'est pas définie localement tout en minimisant la répétition du
chemin complet. Le code du listing 7-13 ne permet pas de savoir clairement où
`add_to_waitlist` est défini.

<!--
On the other hand, when bringing in structs, enums, and other items with `use`,
it's idiomatic to specify the full path. Listing 7-14 shows the idiomatic way
to bring the standard library's `HashMap` struct into the scope of a binary
crate.
-->

D'un autre côté, lorsqu'on amène des structs, des enums et d'autres éléments
avec `use`, il est idiomatique de spécifier le chemin complet. Le listing 7-14
montre la manière idiomatique d'amener la struct `HashMap` de la bibliothèque
standard dans la portée d'une crate binaire.


<Listing number="7-14" file-name="src/main.rs" caption="Amener `HashMap` dans la portée de manière idiomatique">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

<!--
There's no strong reason behind this idiom: It's just the convention that has
emerged, and folks have gotten used to reading and writing Rust code this way.
-->

Il n'y a pas de raison forte derrière cet idiome : c'est simplement la
convention qui a émergé, et les gens se sont habitués à lire et écrire du code
Rust de cette manière.

<!--
The exception to this idiom is if we're bringing two items with the same name
into scope with `use` statements, because Rust doesn't allow that. Listing 7-15
shows how to bring two `Result` types into scope that have the same name but
different parent modules, and how to refer to them.
-->

L'exception à cet idiome est lorsque nous amenons deux éléments portant le même
nom dans la portée avec des instructions `use`, car Rust ne le permet pas. Le
listing 7-15 montre comment amener deux types `Result` dans la portée qui ont le
même nom mais des modules parents différents, et comment s'y référer.


<Listing number="7-15" file-name="src/lib.rs" caption="Amener deux types portant le même nom dans la même portée nécessite d'utiliser leurs modules parents.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

<!--
As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we'd
have two `Result` types in the same scope, and Rust wouldn't know which one we
meant when we used `Result`.
-->

Comme vous pouvez le voir, utiliser les modules parents permet de distinguer les
deux types `Result`. Si au contraire nous avions spécifié `use std::fmt::Result`
et `use std::io::Result`, nous aurions deux types `Result` dans la même portée,
et Rust ne saurait pas lequel nous voulions dire quand nous utiliserions
`Result`.

<!--
### Providing New Names with the `as` Keyword
-->

### Fournir de nouveaux noms avec le mot-clé `as`

<!--
There's another solution to the problem of bringing two types of the same name
into the same scope with `use`: After the path, we can specify `as` and a new
local name, or _alias_, for the type. Listing 7-16 shows another way to write
the code in Listing 7-15 by renaming one of the two `Result` types using `as`.
-->

Il existe une autre solution au problème d'amener deux types portant le même nom
dans la même portée avec `use` : après le chemin, nous pouvons spécifier `as` et
un nouveau nom local, ou *alias*, pour le type. Le listing 7-16 montre une autre
façon d'écrire le code du listing 7-15 en renommant l'un des deux types `Result`
en utilisant `as`.


<Listing number="7-16" file-name="src/lib.rs" caption="Renommer un type lorsqu'il est amené dans la portée avec le mot-clé `as`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

<!--
In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won't conflict with the `Result` from `std::fmt`
that we've also brought into scope. Listing 7-15 and Listing 7-16 are
considered idiomatic, so the choice is up to you!
-->

Dans la deuxième instruction `use`, nous avons choisi le nouveau nom `IoResult`
pour le type `std::io::Result`, qui n'entrera pas en conflit avec le `Result` de
`std::fmt` que nous avons également amené dans la portée. Le listing 7-15 et le
listing 7-16 sont considérés comme idiomatiques, donc le choix vous appartient !

<!--
### Re-exporting Names with `pub use`
-->

### Réexporter des noms avec `pub use`

<!--
When we bring a name into scope with the `use` keyword, the name is private to
the scope into which we imported it. To enable code outside that scope to refer
to that name as if it had been defined in that scope, we can combine `pub` and
`use`. This technique is called _re-exporting_ because we're bringing an item
into scope but also making that item available for others to bring into their
scope.
-->

Lorsque nous amenons un nom dans la portée avec le mot-clé `use`, le nom est
privé à la portée dans laquelle nous l'avons importé. Pour permettre au code en
dehors de cette portée de se référer à ce nom comme s'il avait été défini dans
cette portée, nous pouvons combiner `pub` et `use`. Cette technique s'appelle
la *réexportation* (*re-exporting*) parce que nous amenons un élément dans la
portée mais le rendons également disponible pour que d'autres puissent l'amener
dans leur portée.

<!--
Listing 7-17 shows the code in Listing 7-11 with `use` in the root module
changed to `pub use`.
-->

Le listing 7-17 montre le code du listing 7-11 avec `use` dans le module racine
changé en `pub use`.


<Listing number="7-17" file-name="src/lib.rs" caption="Rendre un nom disponible pour que tout code puisse l'utiliser depuis une nouvelle portée avec `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

<!--
Before this change, external code would have to call the `add_to_waitlist`
function by using the path
`restaurant::front_of_house::hosting::add_to_waitlist()`, which also would have
required the `front_of_house` module to be marked as `pub`. Now that this `pub
use` has re-exported the `hosting` module from the root module, external code
can use the path `restaurant::hosting::add_to_waitlist()` instead.
-->

Avant ce changement, le code externe aurait dû appeler la fonction
`add_to_waitlist` en utilisant le chemin
`restaurant::front_of_house::hosting::add_to_waitlist()`, ce qui aurait
également nécessité que le module `front_of_house` soit marqué comme `pub`.
Maintenant que ce `pub use` a réexporté le module `hosting` depuis le module
racine, le code externe peut utiliser le chemin
`restaurant::hosting::add_to_waitlist()` à la place.

<!--
Re-exporting is useful when the internal structure of your code is different
from how programmers calling your code would think about the domain. For
example, in this restaurant metaphor, the people running the restaurant think
about "front of house" and "back of house." But customers visiting a restaurant
probably won't think about the parts of the restaurant in those terms. With `pub
use`, we can write our code with one structure but expose a different structure.
Doing so makes our library well organized for programmers working on the library
and programmers calling the library. We'll look at another example of `pub use`
and how it affects your crate's documentation in ["Exporting a Convenient Public
API"][ch14-pub-use] ignore
--> in Chapter 14.
-->

La réexportation est utile lorsque la structure interne de votre code est
différente de la façon dont les développeurs appelant votre code penseraient au
domaine. Par exemple, dans cette métaphore du restaurant, les personnes qui
gèrent le restaurant pensent en termes de « salle » et « cuisines ». Mais les
clients visitant un restaurant ne penseront probablement pas aux parties du
restaurant en ces termes. Avec `pub use`, nous pouvons écrire notre code avec
une structure mais en exposer une différente. Ce faisant, notre bibliothèque est
bien organisée à la fois pour les développeurs travaillant sur la bibliothèque
et ceux qui l'utilisent. Nous verrons un autre exemple de `pub use` et comment
cela affecte la documentation de votre crate dans [« Exporter une API publique
pratique »][ch14-pub-use]<!--
ignore
--> au chapitre 14.

<!--
### Using External Packages
-->

### Utiliser des packages externes

<!--
In Chapter 2, we programmed a guessing game project that used an external
package called `rand` to get random numbers. To use `rand` in our project, we
added this line to _Cargo.toml_:
-->

Au chapitre 2, nous avons programmé un projet de jeu de devinettes qui utilisait
un package externe appelé `rand` pour obtenir des nombres aléatoires. Pour
utiliser `rand` dans notre projet, nous avons ajouté cette ligne à *Cargo.toml* :

<!--
When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->


<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

<!--
Adding `rand` as a dependency in _Cargo.toml_ tells Cargo to download the
`rand` package and any dependencies from [crates.io](https://crates.io/) and
make `rand` available to our project.
-->

Ajouter `rand` comme dépendance dans *Cargo.toml* indique à Cargo de
télécharger le package `rand` et toutes ses dépendances depuis
[crates.io](https://crates.io/) et de rendre `rand` disponible pour notre
projet.

<!--
Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the crate, `rand`, and listed the items we
wanted to bring into scope. Recall that in ["Generating a Random
Number"][rand] ignore
--> in Chapter 2, we brought the `Rng` trait into
scope and called the `rand::thread_rng` function:
-->

Ensuite, pour amener les définitions de `rand` dans la portée de notre package,
nous avons ajouté une ligne `use` commençant par le nom de la crate, `rand`, et
listé les éléments que nous voulions amener dans la portée. Rappelons que dans
[« Générer un nombre aléatoire »][rand]<!--
ignore
--> au chapitre 2, nous avons
amené le trait `Rng` dans la portée et appelé la fonction `rand::thread_rng` :


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

<!--
Members of the Rust community have made many packages available at
[crates.io](https://crates.io/), and pulling any of them into your package
involves these same steps: listing them in your package's _Cargo.toml_ file and
using `use` to bring items from their crates into scope.
-->

Les membres de la communauté Rust ont rendu de nombreux packages disponibles sur
[crates.io](https://crates.io/), et intégrer n'importe lequel d'entre eux dans
votre package implique les mêmes étapes : les lister dans le fichier
*Cargo.toml* de votre package et utiliser `use` pour amener les éléments de
leurs crates dans la portée.

<!--
Note that the standard `std` library is also a crate that's external to our
package. Because the standard library is shipped with the Rust language, we
don't need to change _Cargo.toml_ to include `std`. But we do need to refer to
it with `use` to bring items from there into our package's scope. For example,
with `HashMap` we would use this line:
-->

Notez que la bibliothèque standard `std` est également une crate externe à notre
package. Comme la bibliothèque standard est livrée avec le langage Rust, nous
n'avons pas besoin de modifier *Cargo.toml* pour inclure `std`. Mais nous devons
nous y référer avec `use` pour amener les éléments dans la portée de notre
package. Par exemple, pour `HashMap`, nous utiliserions cette ligne :

<!--
```rust
use std::collections::HashMap;
```
-->

```rust
use std::collections::HashMap;
```

<!--
This is an absolute path starting with `std`, the name of the standard library
crate.
-->

C'est un chemin absolu commençant par `std`, le nom de la crate de la
bibliothèque standard.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

<!--
### Using Nested Paths to Clean Up `use` Lists
-->

### Utiliser des chemins imbriqués pour nettoyer les listes `use`

<!--
If we're using multiple items defined in the same crate or same module, listing
each item on its own line can take up a lot of vertical space in our files. For
example, these two `use` statements we had in the guessing game in Listing 2-4
bring items from `std` into scope:
-->

Si nous utilisons plusieurs éléments définis dans la même crate ou le même
module, lister chaque élément sur sa propre ligne peut prendre beaucoup d'espace
vertical dans nos fichiers. Par exemple, ces deux instructions `use` que nous
avions dans le jeu de devinettes du listing 2-4 amènent des éléments de `std`
dans la portée :


<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

<!--
Instead, we can use nested paths to bring the same items into scope in one
line. We do this by specifying the common part of the path, followed by two
colons, and then curly brackets around a list of the parts of the paths that
differ, as shown in Listing 7-18.
-->

À la place, nous pouvons utiliser des chemins imbriqués pour amener les mêmes
éléments dans la portée en une seule ligne. Nous faisons cela en spécifiant la
partie commune du chemin, suivie de deux deux-points, puis des accolades autour
d'une liste des parties des chemins qui diffèrent, comme montré dans le
listing 7-18.


<Listing number="7-18" file-name="src/main.rs" caption="Spécifier un chemin imbriqué pour amener plusieurs éléments avec le même préfixe dans la portée">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

<!--
In bigger programs, bringing many items into scope from the same crate or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!
-->

Dans des programmes plus grands, amener de nombreux éléments dans la portée
depuis la même crate ou le même module en utilisant des chemins imbriqués peut
réduire considérablement le nombre d'instructions `use` séparées nécessaires !

<!--
We can use a nested path at any level in a path, which is useful when combining
two `use` statements that share a subpath. For example, Listing 7-19 shows two
`use` statements: one that brings `std::io` into scope and one that brings
`std::io::Write` into scope.
-->

Nous pouvons utiliser un chemin imbriqué à n'importe quel niveau d'un chemin, ce
qui est utile lorsqu'on combine deux instructions `use` qui partagent un
sous-chemin. Par exemple, le listing 7-19 montre deux instructions `use` : une
qui amène `std::io` dans la portée et une qui amène `std::io::Write` dans la
portée.


<Listing number="7-19" file-name="src/lib.rs" caption="Deux instructions `use` où l'une est un sous-chemin de l'autre">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

<!--
The common part of these two paths is `std::io`, and that's the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path, as shown in Listing 7-20.
-->

La partie commune de ces deux chemins est `std::io`, et c'est le premier chemin
complet. Pour fusionner ces deux chemins en une seule instruction `use`, nous
pouvons utiliser `self` dans le chemin imbriqué, comme montré dans le
listing 7-20.


<Listing number="7-20" file-name="src/lib.rs" caption="Combiner les chemins du listing 7-19 en une seule instruction `use`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

<!--
This line brings `std::io` and `std::io::Write` into scope.
-->

Cette ligne amène `std::io` et `std::io::Write` dans la portée.

<!--
Old headings. Do not remove or links may break.
-->

<a id="the-glob-operator"></a>

<!--
### Importing Items with the Glob Operator
-->

### Importer des éléments avec l'opérateur glob

<!--
If we want to bring _all_ public items defined in a path into scope, we can
specify that path followed by the `*` glob operator:
-->

Si nous voulons amener *tous* les éléments publics définis dans un chemin dans
la portée, nous pouvons spécifier ce chemin suivi de l'opérateur glob `*` :

<!--
```rust
use std::collections::*;
```
-->

```rust
use std::collections::*;
```

<!--
This `use` statement brings all public items defined in `std::collections` into
the current scope. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program
was defined. Additionally, if the dependency changes its definitions, what
you've imported changes as well, which may lead to compiler errors when you
upgrade the dependency if the dependency adds a definition with the same name
as a definition of yours in the same scope, for example.
-->

Cette instruction `use` amène tous les éléments publics définis dans
`std::collections` dans la portée courante. Soyez prudent lorsque vous utilisez
l'opérateur glob ! Glob peut rendre plus difficile de savoir quels noms sont
dans la portée et où un nom utilisé dans votre programme a été défini. De plus,
si la dépendance modifie ses définitions, ce que vous avez importé change
également, ce qui peut entraîner des erreurs de compilation lorsque vous mettez
à jour la dépendance si celle-ci ajoute par exemple une définition portant le
même nom qu'une de vos définitions dans la même portée.

<!--
The glob operator is often used when testing to bring everything under test into
the `tests` module; we'll talk about that in ["How to Write
Tests"][writing-tests] ignore
--> in Chapter 11. The glob operator is also
sometimes used as part of the prelude pattern: See [the standard library
documentation](../std/prelude/index.html#other-preludes)<!--
ignore
--> for more
information on that pattern.
-->

L'opérateur glob est souvent utilisé lors des tests pour amener tout ce qui est
testé dans le module `tests` ; nous en parlerons dans [« Comment écrire des
tests »][writing-tests]<!--
ignore
--> au chapitre 11. L'opérateur glob est
aussi parfois utilisé dans le cadre du patron *prelude* : consultez [la
documentation de la bibliothèque
standard](../std/prelude/index.html#other-preludes)<!--
ignore
--> pour plus
d'informations sur ce patron.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
