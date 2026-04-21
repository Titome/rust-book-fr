<!--
# Functional Language Features: Iterators and Closures
-->

# Fonctionnalites du langage fonctionnel : les iterateurs et les fermetures

<!--
Rust's design has taken inspiration from many existing languages and
techniques, and one significant influence is _functional programming_.
Programming in a functional style often includes using functions as values by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth.
-->

La conception de Rust s'est inspiree de nombreux langages et techniques existants, et une influence notable est la _programmation fonctionnelle_. Programmer dans un style fonctionnel consiste souvent a utiliser des fonctions comme des valeurs en les passant en arguments, en les retournant depuis d'autres fonctions, en les assignant a des variables pour une execution ulterieure, et ainsi de suite.

<!--
In this chapter, we won't debate the issue of what functional programming is or
isn't but will instead discuss some features of Rust that are similar to
features in many languages often referred to as functional.
-->

Dans ce chapitre, nous ne debattrons pas de ce qu'est ou n'est pas la programmation fonctionnelle, mais nous aborderons plutot certaines fonctionnalites de Rust qui sont similaires a celles de nombreux langages souvent qualifies de fonctionnels.

<!--
More specifically, we'll cover:

- _Closures_, a function-like construct you can store in a variable
- _Iterators_, a way of processing a series of elements
- How to use closures and iterators to improve the I/O project in Chapter 12
- The performance of closures and iterators (spoiler alert: They're faster than
  you might think!)
-->

Plus precisement, nous aborderons :

- Les _fermetures_ (closures), une construction semblable a une fonction que vous pouvez stocker dans une variable
- Les _iterateurs_, un moyen de traiter une serie d'elements
- Comment utiliser les fermetures et les iterateurs pour ameliorer le projet d'E/S du chapitre 12
- Les performances des fermetures et des iterateurs (attention spoiler : ils sont plus rapides que vous ne le pensez !)

<!--
We've already covered some other Rust features, such as pattern matching and
enums, that are also influenced by the functional style. Because mastering
closures and iterators is an important part of writing fast, idiomatic, Rust
code, we'll devote this entire chapter to them.
-->

Nous avons deja couvert d'autres fonctionnalites de Rust, comme le filtrage par motif et les enums, qui sont egalement influencees par le style fonctionnel. Parce que maitriser les fermetures et les iterateurs est une part importante de l'ecriture de code Rust rapide et idiomatique, nous consacrerons ce chapitre entier a ces sujets.
