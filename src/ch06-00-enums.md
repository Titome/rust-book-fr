<!--
# Enums and Pattern Matching
-->

# Les énumérations et le filtrage par motif

<!--
In this chapter, we'll look at enumerations, also referred to as _enums_.
Enums allow you to define a type by enumerating its possible variants. First
we'll define and use an enum to show how an enum can encode meaning along with
data. Next, we'll explore a particularly useful enum, called `Option`, which
expresses that a value can be either something or nothing. Then, we'll look at
how pattern matching in the `match` expression makes it easy to run different
code for different values of an enum. Finally, we'll cover how the `if let`
construct is another convenient and concise idiom available to handle enums in
your code.
-->

Dans ce chapitre, nous allons étudier les énumérations, également appelées
_enums_. Les enums vous permettent de définir un type en énumérant ses
variantes possibles. D'abord, nous allons définir et utiliser une enum pour
montrer comment une enum peut encoder du sens en même temps que des données.
Ensuite, nous explorerons une enum particulièrement utile, appelée `Option`,
qui exprime qu'une valeur peut être soit quelque chose, soit rien. Puis, nous
verrons comment le filtrage par motif avec l'expression `match` facilite
l'exécution de code différent selon les valeurs d'une enum. Enfin, nous
aborderons la construction `if let`, un autre idiome pratique et concis
disponible pour gérer les enums dans votre code.
