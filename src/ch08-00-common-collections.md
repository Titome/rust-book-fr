<!--
# Common Collections
-->

# Les collections standard

<!--
Rust's standard library includes a number of very useful data structures called
_collections_. Most other data types represent one specific value, but
collections can contain multiple values. Unlike the built-in array and tuple
types, the data that these collections point to is stored on the heap, which
means the amount of data does not need to be known at compile time and can grow
or shrink as the program runs. Each kind of collection has different
capabilities and costs, and choosing an appropriate one for your current
situation is a skill you'll develop over time. In this chapter, we'll discuss
three collections that are used very often in Rust programs:
-->

La bibliothèque standard de Rust comprend un certain nombre de structures de
données très utiles appelées _collections_. La plupart des autres types de
données représentent une valeur spécifique, mais les collections peuvent
contenir plusieurs valeurs. Contrairement aux types tableau et tuple intégrés,
les données vers lesquelles ces collections pointent sont stockées sur le tas,
ce qui signifie que la quantité de données n'a pas besoin d'être connue à la
compilation et peut augmenter ou diminuer pendant l'exécution du programme.
Chaque type de collection a des capacités et des coûts différents, et choisir
celui qui convient à votre situation actuelle est une compétence que vous
développerez avec le temps. Dans ce chapitre, nous allons aborder trois
collections qui sont très fréquemment utilisées dans les programmes Rust :

<!--
- A _vector_ allows you to store a variable number of values next to each other.
- A _string_ is a collection of characters. We've mentioned the `String` type
  previously, but in this chapter, we'll talk about it in depth.
- A _hash map_ allows you to associate a value with a specific key. It's a
  particular implementation of the more general data structure called a _map_.
-->

- Un _vector_ vous permet de stocker un nombre variable de valeurs les unes à
  côté des autres.
- Une _string_ (chaîne de caractères) est une collection de caractères. Nous
  avons mentionné le type `String` précédemment, mais dans ce chapitre, nous
  en parlerons en profondeur.
- Une _hash map_ (table de hachage) vous permet d'associer une valeur à une
  clé spécifique. C'est une implémentation particulière de la structure de
  données plus générale appelée _map_ (tableau associatif).

<!--
To learn about the other kinds of collections provided by the standard library,
see [the documentation][collections].
-->

Pour en savoir plus sur les autres types de collections fournies par la
bibliothèque standard, consultez [la documentation][collections].

<!--
We'll discuss how to create and update vectors, strings, and hash maps, as well
as what makes each special.
-->

Nous verrons comment créer et mettre à jour des vectors, des strings et des
hash maps, ainsi que ce qui rend chacune d'entre elles spéciale.

[collections]: ../std/collections/index.html
