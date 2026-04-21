<!--
## Storing Keys with Associated Values in Hash Maps
-->

## Stocker des clés avec des valeurs associées dans les hash maps

<!--
The last of our common collections is the hash map. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V` using a _hashing
function_, which determines how it places these keys and values into memory.
Many programming languages support this kind of data structure, but they often
use a different name, such as _hash_, _map_, _object_, _hash table_,
_dictionary_, or _associative array_, just to name a few.
-->

La dernière de nos collections courantes est la hash map. Le type
`HashMap<K, V>` stocke une association de clés de type `K` vers des valeurs de
type `V` en utilisant une _fonction de hachage_, qui détermine comment il place
ces clés et valeurs en mémoire. De nombreux langages de programmation prennent
en charge ce type de structure de données, mais ils utilisent souvent un nom
différent, comme _hash_, _map_, _object_, _hash table_, _dictionary_ ou
_associative array_ (tableau associatif), pour n'en citer que quelques-uns.

<!--
Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team's score in a hash map in which
each key is a team's name and the values are each team's score. Given a team
name, you can retrieve its score.
-->

Les hash maps sont utiles lorsque vous voulez rechercher des données non pas en
utilisant un indice, comme vous pouvez le faire avec les vectors, mais en
utilisant une clé qui peut être de n'importe quel type. Par exemple, dans un
jeu, vous pourriez suivre le score de chaque équipe dans une hash map dans
laquelle chaque clé est le nom d'une équipe et les valeurs sont les scores de
chaque équipe. À partir d'un nom d'équipe, vous pouvez récupérer son score.

<!--
We'll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.
-->

Nous allons parcourir l'API de base des hash maps dans cette section, mais de
nombreuses autres fonctionnalités se cachent dans les fonctions définies sur
`HashMap<K, V>` par la bibliothèque standard. Comme toujours, consultez la
documentation de la bibliothèque standard pour plus d'informations.

<!--
### Creating a New Hash Map
-->

### Créer une nouvelle hash map

<!--
One way to create an empty hash map is to use `new` and to add elements with
`insert`. In Listing 8-20, we're keeping track of the scores of two teams whose
names are _Blue_ and _Yellow_. The Blue team starts with 10 points, and the
Yellow team starts with 50.
-->

Une façon de créer une hash map vide est d'utiliser `new` et d'ajouter des
éléments avec `insert`. Dans l'encart 8-20, nous suivons les scores de deux
équipes dont les noms sont _Blue_ et _Yellow_. L'équipe Blue commence avec 10
points et l'équipe Yellow commence avec 50.

<Listing number="8-20" caption="Création d'une nouvelle hash map et insertion de quelques clés et valeurs">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

<!--
Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it's not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there's no built-in macro to construct them, for example.
-->

Notez que nous devons d'abord importer (`use`) le `HashMap` de la partie
collections de la bibliothèque standard. Parmi nos trois collections courantes,
celle-ci est la moins souvent utilisée, elle n'est donc pas incluse dans les
fonctionnalités automatiquement mises en portée par le prelude. Les hash maps
bénéficient également de moins de support de la part de la bibliothèque
standard ; il n'y a pas de macro intégrée pour les construire, par exemple.

<!--
Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: All of the keys must have the same type, and all of the values
must have the same type.
-->

Tout comme les vectors, les hash maps stockent leurs données sur le tas. Ce
`HashMap` a des clés de type `String` et des valeurs de type `i32`. Comme les
vectors, les hash maps sont homogènes : toutes les clés doivent avoir le même
type, et toutes les valeurs doivent avoir le même type.

<!--
### Accessing Values in a Hash Map
-->

### Accéder aux valeurs dans une hash map

<!--
We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-21.
-->

Nous pouvons obtenir une valeur de la hash map en fournissant sa clé à la
méthode `get`, comme montré dans l'encart 8-21.

<Listing number="8-21" caption="Accès au score de l'équipe Blue stocké dans la hash map">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

<!--
Here, `score` will have the value that's associated with the Blue team, and the
result will be `10`. The `get` method returns an `Option<&V>`; if there's no
value for that key in the hash map, `get` will return `None`. This program
handles the `Option` by calling `copied` to get an `Option<i32>` rather than an
`Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn't
have an entry for the key.
-->

Ici, `score` aura la valeur associée à l'équipe Blue, et le résultat sera `10`.
La méthode `get` renvoie un `Option<&V>` ; s'il n'y a pas de valeur pour cette
clé dans la hash map, `get` renverra `None`. Ce programme gère l'`Option` en
appelant `copied` pour obtenir un `Option<i32>` plutôt qu'un `Option<&i32>`,
puis `unwrap_or` pour mettre `score` à zéro si `scores` n'a pas d'entrée pour
la clé.

<!--
We can iterate over each key-value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:
-->

Nous pouvons itérer sur chaque paire clé-valeur dans une hash map de la même
manière que nous le faisons avec les vectors, en utilisant une boucle `for` :


```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

<!--
This code will print each pair in an arbitrary order:
-->

Ce code affichera chaque paire dans un ordre arbitraire :

<!--
```text
Yellow: 50
Blue: 10
```
-->

```text
Yellow: 50
Blue: 10
```

<!--
Old headings. Do not remove or links may break.
-->

<a id="hash-maps-and-ownership"></a>

<!--
### Managing Ownership in Hash Maps
-->

### Gérer la possession dans les hash maps

<!--
For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.
-->

Pour les types qui implémentent le trait `Copy`, comme `i32`, les valeurs sont
copiées dans la hash map. Pour les valeurs possédées comme `String`, les valeurs
seront déplacées et la hash map deviendra propriétaire de ces valeurs, comme
démontré dans l'encart 8-22.

<Listing number="8-22" caption="Montrer que les clés et les valeurs sont possédées par la hash map une fois qu'elles sont insérées">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

<!--
We aren't able to use the variables `field_name` and `field_value` after
they've been moved into the hash map with the call to `insert`.
-->

Nous ne pouvons pas utiliser les variables `field_name` et `field_value` après
qu'elles ont été déplacées dans la hash map lors de l'appel à `insert`.

<!--
If we insert references to values into the hash map, the values won't be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We'll talk more about these issues in
["Validating References with
Lifetimes"][validating-references-with-lifetimes] ignore
--> in Chapter 10.
-->

Si nous insérons des références vers des valeurs dans la hash map, les valeurs
ne seront pas déplacées dans la hash map. Les valeurs vers lesquelles les
références pointent doivent être valides au moins aussi longtemps que la hash
map est valide. Nous parlerons davantage de ces questions dans ["Valider les
références avec les durées de
vie"][validating-references-with-lifetimes]<!--
ignore
--> au chapitre 10.

<!--
### Updating a Hash Map
-->

### Mettre à jour une hash map

<!--
Although the number of key and value pairs is growable, each unique key can
only have one value associated with it at a time (but not vice versa: For
example, both the Blue team and the Yellow team could have the value `10`
stored in the `scores` hash map).
-->

Bien que le nombre de paires clé-valeur puisse augmenter, chaque clé unique ne
peut avoir qu'une seule valeur associée à la fois (mais pas l'inverse : par
exemple, l'équipe Blue et l'équipe Yellow pourraient toutes deux avoir la
valeur `10` stockée dans la hash map `scores`).

<!--
When you want to change the data in a hash map, you have to decide how to
handle the case when a key already has a value assigned. You could replace the
old value with the new value, completely disregarding the old value. You could
keep the old value and ignore the new value, only adding the new value if the
key _doesn't_ already have a value. Or you could combine the old value and the
new value. Let's look at how to do each of these!
-->

Lorsque vous voulez modifier les données dans une hash map, vous devez décider
comment gérer le cas où une clé a déjà une valeur assignée. Vous pourriez
remplacer l'ancienne valeur par la nouvelle, en ignorant complètement
l'ancienne valeur. Vous pourriez conserver l'ancienne valeur et ignorer la
nouvelle, en n'ajoutant la nouvelle valeur que si la clé n'a _pas_ déjà de
valeur. Ou vous pourriez combiner l'ancienne et la nouvelle valeur. Voyons
comment faire chacune de ces opérations !

<!--
#### Overwriting a Value
-->

#### Écraser une valeur

<!--
If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-23 calls `insert` twice, the hash map will
only contain one key-value pair because we're inserting the value for the Blue
team's key both times.
-->

Si nous insérons une clé et une valeur dans une hash map puis insérons cette
même clé avec une valeur différente, la valeur associée à cette clé sera
remplacée. Même si le code de l'encart 8-23 appelle `insert` deux fois, la
hash map ne contiendra qu'une seule paire clé-valeur car nous insérons la
valeur pour la clé de l'équipe Blue les deux fois.

<Listing number="8-23" caption="Remplacement d'une valeur stockée avec une clé particulière">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

<!--
This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.
-->

Ce code affichera `{"Blue": 25}`. La valeur originale de `10` a été écrasée.

<!--
Old headings. Do not remove or links may break.
-->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

<!--
#### Adding a Key and Value Only If a Key Isn't Present
-->

#### Ajouter une clé et une valeur uniquement si la clé n'est pas présente

<!--
It's common to check whether a particular key already exists in the hash map
with a value and then to take the following actions: If the key does exist in
the hash map, the existing value should remain the way it is; if the key
doesn't exist, insert it and a value for it.
-->

Il est courant de vérifier si une clé particulière existe déjà dans la hash map
avec une valeur, puis de prendre les actions suivantes : si la clé existe dans
la hash map, la valeur existante doit rester telle quelle ; si la clé n'existe
pas, l'insérer avec une valeur.

<!--
Hash maps have a special API for this called `entry` that takes the key you
want to check as a parameter. The return value of the `entry` method is an enum
called `Entry` that represents a value that might or might not exist. Let's say
we want to check whether the key for the Yellow team has a value associated
with it. If it doesn't, we want to insert the value `50`, and the same for the
Blue team. Using the `entry` API, the code looks like Listing 8-24.
-->

Les hash maps ont une API spéciale pour cela appelée `entry` qui prend en
paramètre la clé que vous souhaitez vérifier. La valeur de retour de la méthode
`entry` est une enum appelée `Entry` qui représente une valeur qui pourrait ou
non exister. Disons que nous voulons vérifier si la clé de l'équipe Yellow a
une valeur associée. Si ce n'est pas le cas, nous voulons insérer la valeur
`50`, et de même pour l'équipe Blue. En utilisant l'API `entry`, le code
ressemble à l'encart 8-24.

<Listing number="8-24" caption="Utilisation de la méthode `entry` pour n'insérer que si la clé n'a pas déjà une valeur">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

<!--
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not, it
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.
-->

La méthode `or_insert` sur `Entry` est définie pour renvoyer une référence
mutable à la valeur correspondant à la clé de l'`Entry` si cette clé existe,
et sinon, elle insère le paramètre comme nouvelle valeur pour cette clé et
renvoie une référence mutable à la nouvelle valeur. Cette technique est
beaucoup plus propre que d'écrire la logique nous-mêmes et, de plus,
fonctionne mieux avec le vérificateur d'emprunt.

<!--
Running the code in Listing 8-24 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
`50` because the Yellow team doesn't have a value already. The second call to
`entry` will not change the hash map, because the Blue team already has the
value `10`.
-->

L'exécution du code de l'encart 8-24 affichera `{"Yellow": 50, "Blue": 10}`.
Le premier appel à `entry` insérera la clé pour l'équipe Yellow avec la valeur
`50` car l'équipe Yellow n'a pas encore de valeur. Le second appel à `entry` ne
modifiera pas la hash map, car l'équipe Blue a déjà la valeur `10`.

<!--
#### Updating a Value Based on the Old Value
-->

#### Mettre à jour une valeur en fonction de l'ancienne valeur

<!--
Another common use case for hash maps is to look up a key's value and then
update it based on the old value. For instance, Listing 8-25 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we've
seen that word. If it's the first time we've seen a word, we'll first insert
the value `0`.
-->

Un autre cas d'utilisation courant des hash maps est de rechercher la valeur
d'une clé puis de la mettre à jour en fonction de l'ancienne valeur. Par
exemple, l'encart 8-25 montre du code qui compte combien de fois chaque mot
apparaît dans un texte. Nous utilisons une hash map avec les mots comme clés
et incrémentons la valeur pour suivre combien de fois nous avons vu ce mot.
Si c'est la première fois que nous voyons un mot, nous insérons d'abord la
valeur `0`.

<Listing number="8-25" caption="Compter les occurrences de mots en utilisant une hash map qui stocke les mots et les compteurs">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

<!--
This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. You might see
the same key-value pairs printed in a different order: Recall from ["Accessing
Values in a Hash Map"][access] ignore
--> that iterating over a hash map
happens in an arbitrary order.
-->

Ce code affichera `{"world": 2, "hello": 1, "wonderful": 1}`. Vous pourriez
voir les mêmes paires clé-valeur affichées dans un ordre différent :
rappelez-vous de la section ["Accéder aux valeurs dans une hash
map"][access]<!--
ignore
--> que l'itération sur une hash map se fait dans un
ordre arbitraire.

<!--
The `split_whitespace` method returns an iterator over subslices, separated by
whitespace, of the value in `text`. The `or_insert` method returns a mutable
reference (`&mut V`) to the value for the specified key. Here, we store that
mutable reference in the `count` variable, so in order to assign to that value,
we must first dereference `count` using the asterisk (`*`). The mutable
reference goes out of scope at the end of the `for` loop, so all of these
changes are safe and allowed by the borrowing rules.
-->

La méthode `split_whitespace` renvoie un itérateur sur des sous-slices,
séparées par des espaces, de la valeur dans `text`. La méthode `or_insert`
renvoie une référence mutable (`&mut V`) à la valeur pour la clé spécifiée.
Ici, nous stockons cette référence mutable dans la variable `count`, donc pour
assigner une valeur, nous devons d'abord déréférencer `count` en utilisant
l'astérisque (`*`). La référence mutable sort de la portée à la fin de la
boucle `for`, donc tous ces changements sont sûrs et autorisés par les règles
d'emprunt.

<!--
### Hashing Functions
-->

### Fonctions de hachage

<!--
By default, `HashMap` uses a hashing function called _SipHash_ that can provide
resistance to denial-of-service (DoS) attacks involving hash
tables[^siphash] ignore
-->. This is not the fastest hashing algorithm
available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default
hash function is too slow for your purposes, you can switch to another function
by specifying a different hasher. A _hasher_ is a type that implements the
`BuildHasher` trait. We'll talk about traits and how to implement them in
[Chapter 10][traits]<!--
ignore
-->. You don't necessarily have to implement
your own hasher from scratch; [crates.io](https://crates.io/)<!--
ignore
-->
has libraries shared by other Rust users that provide hashers implementing many
common hashing algorithms.
-->

Par défaut, `HashMap` utilise une fonction de hachage appelée _SipHash_ qui
peut fournir une résistance aux attaques par déni de service (DoS) impliquant
des tables de hachage[^siphash]<!--
ignore
-->. Ce n'est pas l'algorithme de
hachage le plus rapide disponible, mais le compromis pour une meilleure sécurité
qui accompagne la baisse de performance en vaut la peine. Si vous profilez votre
code et constatez que la fonction de hachage par défaut est trop lente pour vos
besoins, vous pouvez passer à une autre fonction en spécifiant un hasher
différent. Un _hasher_ est un type qui implémente le trait `BuildHasher`. Nous
parlerons des traits et de leur implémentation au
[chapitre 10][traits]<!--
ignore
-->. Vous n'avez pas nécessairement besoin
d'implémenter votre propre hasher de zéro ;
[crates.io](https://crates.io/)<!--
ignore
--> propose des bibliothèques
partagées par d'autres utilisateurs de Rust qui fournissent des hashers
implémentant de nombreux algorithmes de hachage courants.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

<!--
## Summary
-->

## Résumé

<!--
Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:
-->

Les vectors, les strings et les hash maps fournissent une grande quantité de
fonctionnalités nécessaires dans les programmes lorsque vous avez besoin de
stocker, d'accéder et de modifier des données. Voici quelques exercices que
vous devriez maintenant être en mesure de résoudre :

<!--
1. Given a list of integers, use a vector and return the median (when sorted,
   the value in the middle position) and mode (the value that occurs most
   often; a hash map will be helpful here) of the list.
1. Convert strings to Pig Latin. The first consonant of each word is moved to
   the end of the word and _ay_ is added, so _first_ becomes _irst-fay_. Words
   that start with a vowel have _hay_ added to the end instead (_apple_ becomes
   _apple-hay_). Keep in mind the details about UTF-8 encoding!
1. Using a hash map and vectors, create a text interface to allow a user to add
   employee names to a department in a company; for example, "Add Sally to
   Engineering" or "Add Amir to Sales." Then, let the user retrieve a list of
   all people in a department or all people in the company by department, sorted
   alphabetically.
-->

1. À partir d'une liste d'entiers, utilisez un vector et renvoyez la médiane
   (une fois triée, la valeur en position centrale) et le mode (la valeur qui
   apparaît le plus souvent ; une hash map sera utile ici) de la liste.
1. Convertissez des chaînes en Pig Latin. La première consonne de chaque mot
   est déplacée à la fin du mot et _ay_ est ajouté, donc _first_ devient
   _irst-fay_. Les mots qui commencent par une voyelle ont _hay_ ajouté à la
   fin à la place (_apple_ devient _apple-hay_). Gardez à l'esprit les détails
   concernant l'encodage UTF-8 !
1. En utilisant une hash map et des vectors, créez une interface textuelle pour
   permettre à un utilisateur d'ajouter des noms d'employés à un département
   dans une entreprise ; par exemple, "Add Sally to Engineering" ou "Add Amir
   to Sales." Ensuite, permettez à l'utilisateur de récupérer la liste de
   toutes les personnes d'un département ou de toutes les personnes de
   l'entreprise par département, triées par ordre alphabétique.

<!--
The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!
-->

La documentation de l'API de la bibliothèque standard décrit les méthodes que
possèdent les vectors, les strings et les hash maps et qui seront utiles pour
ces exercices !

<!--
We're getting into more complex programs in which operations can fail, so it's
a perfect time to discuss error handling. We'll do that next!
-->

Nous abordons des programmes plus complexes dans lesquels les opérations peuvent
échouer, c'est donc le moment idéal pour discuter de la gestion des erreurs.
C'est ce que nous ferons ensuite !

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
[traits]: ch10-02-traits.html
