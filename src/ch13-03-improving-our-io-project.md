<!--
## Improving Our I/O Project
-->

## Ameliorer notre projet d'E/S

<!--
With this new knowledge about iterators, we can improve the I/O project in
Chapter 12 by using iterators to make places in the code clearer and more
concise. Let's look at how iterators can improve our implementation of the
`Config::build` function and the `search` function.
-->

Avec ces nouvelles connaissances sur les iterateurs, nous pouvons ameliorer le projet d'E/S du chapitre 12 en utilisant des iterateurs pour rendre certaines parties du code plus claires et plus concises. Voyons comment les iterateurs peuvent ameliorer notre implementation de la fonction `Config::build` et de la fonction `search`.

<!--
### Removing a `clone` Using an Iterator
-->

### Supprimer un `clone` en utilisant un iterateur

<!--
In Listing 12-6, we added code that took a slice of `String` values and created
an instance of the `Config` struct by indexing into the slice and cloning the
values, allowing the `Config` struct to own those values. In Listing 13-17,
we've reproduced the implementation of the `Config::build` function as it was
in Listing 12-23.
-->

Dans l'encart 12-6, nous avons ajoute du code qui prenait une slice de valeurs `String` et creait une instance de la struct `Config` en indexant la slice et en clonant les valeurs, permettant a la struct `Config` de posseder ces valeurs. Dans l'encart 13-17, nous avons reproduit l'implementation de la fonction `Config::build` telle qu'elle etait dans l'encart 12-23.

<Listing number="13-17" file-name="src/main.rs" caption="Reproduction de la fonction `Config::build` de l'encart 12-23">


```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

<!--
At the time, we said not to worry about the inefficient `clone` calls because
we would remove them in the future. Well, that time is now!
-->

A l'epoque, nous avions dit de ne pas s'inquieter des appels inefficaces a `clone` car nous les supprimerions a l'avenir. Eh bien, ce moment est arrive !

<!--
We needed `clone` here because we have a slice with `String` elements in the
parameter `args`, but the `build` function doesn't own `args`. To return
ownership of a `Config` instance, we had to clone the values from the `query`
and `file_path` fields of `Config` so that the `Config` instance can own its
values.
-->

Nous avions besoin de `clone` ici car nous avons une slice avec des elements `String` dans le parametre `args`, mais la fonction `build` ne possede pas `args`. Pour retourner la possession d'une instance de `Config`, nous devions cloner les valeurs des champs `query` et `file_path` de `Config` afin que l'instance de `Config` puisse posseder ses valeurs.

<!--
With our new knowledge about iterators, we can change the `build` function to
take ownership of an iterator as its argument instead of borrowing a slice.
We'll use the iterator functionality instead of the code that checks the length
of the slice and indexes into specific locations. This will clarify what the
`Config::build` function is doing because the iterator will access the values.
-->

Avec nos nouvelles connaissances sur les iterateurs, nous pouvons modifier la fonction `build` pour qu'elle prenne la possession d'un iterateur comme argument au lieu d'emprunter une slice. Nous utiliserons la fonctionnalite d'iterateur au lieu du code qui verifie la longueur de la slice et indexe des emplacements specifiques. Cela clarifiera ce que fait la fonction `Config::build` car l'iterateur accede aux valeurs.

<!--
Once `Config::build` takes ownership of the iterator and stops using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.
-->

Une fois que `Config::build` prend la possession de l'iterateur et cesse d'utiliser des operations d'indexation qui empruntent, nous pouvons deplacer les valeurs `String` de l'iterateur dans `Config` plutot que d'appeler `clone` et de faire une nouvelle allocation.

<!--
#### Using the Returned Iterator Directly
-->

#### Utiliser l'iterateur retourne directement

<!--
Open your I/O project's _src/main.rs_ file, which should look like this:
-->

Ouvrez le fichier _src/main.rs_ de votre projet d'E/S, qui devrait ressembler a ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

<!--
We'll first change the start of the `main` function that we had in Listing
12-24 to the code in Listing 13-18, which this time uses an iterator. This
won't compile until we update `Config::build` as well.
-->

Nous allons d'abord modifier le debut de la fonction `main` que nous avions dans l'encart 12-24 par le code de l'encart 13-18, qui cette fois utilise un iterateur. Cela ne compilera pas tant que nous n'aurons pas egalement mis a jour `Config::build`.

<Listing number="13-18" file-name="src/main.rs" caption="Passer la valeur de retour de `env::args` a `Config::build`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

<!--
The `env::args` function returns an iterator! Rather than collecting the
iterator values into a vector and then passing a slice to `Config::build`, now
we're passing ownership of the iterator returned from `env::args` to
`Config::build` directly.
-->

La fonction `env::args` retourne un iterateur ! Plutot que de collecter les valeurs de l'iterateur dans un vecteur puis de passer une slice a `Config::build`, nous passons maintenant directement la possession de l'iterateur retourne par `env::args` a `Config::build`.

<!--
Next, we need to update the definition of `Config::build`. Let's change the
signature of `Config::build` to look like Listing 13-19. This still won't
compile, because we need to update the function body.
-->

Ensuite, nous devons mettre a jour la definition de `Config::build`. Modifions la signature de `Config::build` pour qu'elle ressemble a l'encart 13-19. Cela ne compilera toujours pas, car nous devons mettre a jour le corps de la fonction.

<Listing number="13-19" file-name="src/main.rs" caption="Mise a jour de la signature de `Config::build` pour attendre un iterateur">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

<!--
The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`, and that type implements
the `Iterator` trait and returns `String` values.
-->

La documentation de la bibliotheque standard pour la fonction `env::args` montre que le type de l'iterateur qu'elle retourne est `std::env::Args`, et ce type implemente le trait `Iterator` et retourne des valeurs `String`.

<!--
We've updated the signature of the `Config::build` function so that the
parameter `args` has a generic type with the trait bounds `impl Iterator<Item =
String>` instead of `&[String]`. This usage of the `impl Trait` syntax we
discussed in the ["Using Traits as Parameters"][impl-trait] ignore
-->
section of Chapter 10 means that `args` can be any type that implements the
`Iterator` trait and returns `String` items.
-->

Nous avons mis a jour la signature de la fonction `Config::build` pour que le parametre `args` ait un type generique avec les contraintes de trait `impl Iterator<Item = String>` au lieu de `&[String]`. Cette utilisation de la syntaxe `impl Trait` que nous avons abordee dans la section ["Utiliser les traits comme parametres"][impl-trait]<!--
ignore
--> du chapitre 10 signifie que `args` peut etre n'importe quel type qui implemente le trait `Iterator` et retourne des elements `String`.

<!--
Because we're taking ownership of `args` and we'll be mutating `args` by
iterating over it, we can add the `mut` keyword into the specification of the
`args` parameter to make it mutable.
-->

Parce que nous prenons la possession de `args` et que nous allons muter `args` en iterant dessus, nous pouvons ajouter le mot-cle `mut` dans la specification du parametre `args` pour le rendre mutable.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-iterator-trait-methods-instead-of-indexing"></a>

<!--
#### Using `Iterator` Trait Methods
-->

#### Utiliser les methodes du trait `Iterator`

<!--
Next, we'll fix the body of `Config::build`. Because `args` implements the
`Iterator` trait, we know we can call the `next` method on it! Listing 13-20
updates the code from Listing 12-23 to use the `next` method.
-->

Ensuite, nous allons corriger le corps de `Config::build`. Parce que `args` implemente le trait `Iterator`, nous savons que nous pouvons appeler la methode `next` dessus ! L'encart 13-20 met a jour le code de l'encart 12-23 pour utiliser la methode `next`.

<Listing number="13-20" file-name="src/main.rs" caption="Modification du corps de `Config::build` pour utiliser les methodes d'iterateur">


```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

<!--
Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Then, we call `next` to get the
value we want to put in the `query` field of `Config`. If `next` returns
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given, and we return early with an `Err` value. We do
the same thing for the `file_path` value.
-->

Rappelez-vous que la premiere valeur dans la valeur de retour de `env::args` est le nom du programme. Nous voulons l'ignorer et passer a la valeur suivante, donc nous appelons d'abord `next` et ne faisons rien avec la valeur de retour. Ensuite, nous appelons `next` pour obtenir la valeur que nous voulons mettre dans le champ `query` de `Config`. Si `next` retourne `Some`, nous utilisons un `match` pour extraire la valeur. Si elle retourne `None`, cela signifie que pas assez d'arguments ont ete fournis, et nous retournons prematurement avec une valeur `Err`. Nous faisons la meme chose pour la valeur `file_path`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="making-code-clearer-with-iterator-adapters"></a>

<!--
### Clarifying Code with Iterator Adapters
-->

### Clarifier le code avec les adaptateurs d'iterateurs

<!--
We can also take advantage of iterators in the `search` function in our I/O
project, which is reproduced here in Listing 13-21 as it was in Listing 12-19.
-->

Nous pouvons egalement tirer parti des iterateurs dans la fonction `search` de notre projet d'E/S, qui est reproduite ici dans l'encart 13-21 telle qu'elle etait dans l'encart 12-19.

<Listing number="13-21" file-name="src/lib.rs" caption="L'implementation de la fonction `search` de l'encart 12-19">


```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

<!--
We can write this code in a more concise way using iterator adapter methods.
Doing so also lets us avoid having a mutable intermediate `results` vector. The
functional programming style prefers to minimize the amount of mutable state to
make code clearer. Removing the mutable state might enable a future enhancement
to make searching happen in parallel because we wouldn't have to manage
concurrent access to the `results` vector. Listing 13-22 shows this change.
-->

Nous pouvons ecrire ce code de maniere plus concise en utilisant des methodes d'adaptateur d'iterateur. Ce faisant, nous evitons egalement d'avoir un vecteur `results` intermediaire mutable. Le style de programmation fonctionnelle prefere minimiser la quantite d'etat mutable pour rendre le code plus clair. Supprimer l'etat mutable pourrait permettre une amelioration future pour effectuer la recherche en parallele, car nous n'aurions pas a gerer l'acces concurrent au vecteur `results`. L'encart 13-22 montre ce changement.

<Listing number="13-22" file-name="src/lib.rs" caption="Utilisation des methodes d'adaptateur d'iterateur dans l'implementation de la fonction `search`">


```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

<!--
Recall that the purpose of the `search` function is to return all lines in
`contents` that contain the `query`. Similar to the `filter` example in Listing
13-16, this code uses the `filter` adapter to keep only the lines for which
`line.contains(query)` returns `true`. We then collect the matching lines into
another vector with `collect`. Much simpler! Feel free to make the same change
to use iterator methods in the `search_case_insensitive` function as well.
-->

Rappelez-vous que le but de la fonction `search` est de retourner toutes les lignes de `contents` qui contiennent la `query`. De maniere similaire a l'exemple `filter` de l'encart 13-16, ce code utilise l'adaptateur `filter` pour ne garder que les lignes pour lesquelles `line.contains(query)` retourne `true`. Nous collectons ensuite les lignes correspondantes dans un autre vecteur avec `collect`. Bien plus simple ! N'hesitez pas a faire le meme changement pour utiliser les methodes d'iterateur dans la fonction `search_case_insensitive` egalement.

<!--
For a further improvement, return an iterator from the `search` function by
removing the call to `collect` and changing the return type to `impl
Iterator<Item = &'a str>` so that the function becomes an iterator adapter.
Note that you'll also need to update the tests! Search through a large file
using your `minigrep` tool before and after making this change to observe the
difference in behavior. Before this change, the program won't print any results
until it has collected all of the results, but after the change, the results
will be printed as each matching line is found because the `for` loop in the
`run` function is able to take advantage of the laziness of the iterator.
-->

Pour une amelioration supplementaire, retournez un iterateur depuis la fonction `search` en supprimant l'appel a `collect` et en changeant le type de retour en `impl Iterator<Item = &'a str>` afin que la fonction devienne un adaptateur d'iterateur. Notez que vous devrez egalement mettre a jour les tests ! Effectuez une recherche dans un gros fichier en utilisant votre outil `minigrep` avant et apres ce changement pour observer la difference de comportement. Avant ce changement, le programme n'affichera aucun resultat tant qu'il n'aura pas collecte tous les resultats, mais apres le changement, les resultats seront affiches au fur et a mesure que chaque ligne correspondante est trouvee car la boucle `for` dans la fonction `run` peut tirer parti de la paresse de l'iterateur.

<!--
Old headings. Do not remove or links may break.
-->

<a id="choosing-between-loops-or-iterators"></a>

<!--
### Choosing Between Loops and Iterators
-->

### Choisir entre les boucles et les iterateurs

<!--
The next logical question is which style you should choose in your own code and
why: the original implementation in Listing 13-21 or the version using
iterators in Listing 13-22 (assuming we're collecting all the results before
returning them rather than returning the iterator). Most Rust programmers
prefer to use the iterator style. It's a bit tougher to get the hang of at
first, but once you get a feel for the various iterator adapters and what they
do, iterators can be easier to understand. Instead of fiddling with the various
bits of looping and building new vectors, the code focuses on the high-level
objective of the loop. This abstracts away some of the commonplace code so that
it's easier to see the concepts that are unique to this code, such as the
filtering condition each element in the iterator must pass.
-->

La question logique suivante est quel style vous devriez choisir dans votre propre code et pourquoi : l'implementation originale de l'encart 13-21 ou la version utilisant les iterateurs de l'encart 13-22 (en supposant que nous collectons tous les resultats avant de les retourner plutot que de retourner l'iterateur). La plupart des programmeurs Rust preferent utiliser le style iterateur. C'est un peu plus difficile a maitriser au debut, mais une fois que vous avez une bonne comprehension des differents adaptateurs d'iterateurs et de ce qu'ils font, les iterateurs peuvent etre plus faciles a comprendre. Au lieu de bricoler les differentes parties des boucles et de construire de nouveaux vecteurs, le code se concentre sur l'objectif de haut niveau de la boucle. Cela abstrait une partie du code courant de sorte qu'il est plus facile de voir les concepts propres a ce code, comme la condition de filtrage que chaque element de l'iterateur doit satisfaire.

<!--
But are the two implementations truly equivalent? The intuitive assumption
might be that the lower-level loop will be faster. Let's talk about performance.
-->

Mais les deux implementations sont-elles vraiment equivalentes ? L'hypothese intuitive pourrait etre que la boucle de bas niveau sera plus rapide. Parlons des performances.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
