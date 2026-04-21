<!--
# Using Structs to Structure Related Data
-->

# Utiliser les structs pour structurer des données apparentées

<!--
A _struct_, or _structure_, is a custom data type that lets you package
together and name multiple related values that make up a meaningful group. If
you're familiar with an object-oriented language, a struct is like an object's
data attributes. In this chapter, we'll compare and contrast tuples with
structs to build on what you already know and demonstrate when structs are a
better way to group data.
-->

Une _struct_, ou _structure_, est un type de données personnalisé qui vous permet
de regrouper et de nommer plusieurs valeurs apparentées qui forment un ensemble
cohérent. Si vous êtes familier avec un langage orienté objet, une struct
ressemble aux attributs de données d'un objet. Dans ce chapitre, nous comparerons
les tuples et les structs en nous appuyant sur ce que vous savez déjà et nous
montrerons quand les structs sont un meilleur moyen de regrouper des données.

<!--
We'll demonstrate how to define and instantiate structs. We'll discuss how to
define associated functions, especially the kind of associated functions called
_methods_, to specify behavior associated with a struct type. Structs and enums
(discussed in Chapter 6) are the building blocks for creating new types in your
program's domain to take full advantage of Rust's compile-time type checking.
-->

Nous montrerons comment définir et instancier des structs. Nous verrons comment
définir des fonctions associées, en particulier le type de fonctions associées
appelées _méthodes_, pour spécifier le comportement associé à un type struct.
Les structs et les enums (abordés au chapitre 6) sont les briques de base pour
créer de nouveaux types dans le domaine de votre programme afin de tirer
pleinement parti de la vérification de types à la compilation de Rust.
