<!--
# Patterns and Matching
-->

# Les motifs et le filtrage par motif

<!--
Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over a program's
control flow. A pattern consists of some combination of the following:
-->

Les motifs sont une syntaxe spéciale en Rust permettant de faire correspondre la structure des types, qu'ils soient complexes ou simples. L'utilisation de motifs en conjonction avec les expressions `match` et d'autres constructions vous donne davantage de contrôle sur le flux d'exécution d'un programme. Un motif est constitué d'une combinaison des éléments suivants :

<!--
- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders
-->

- Des littéraux
- Des tableaux, enums, structures ou tuples déstructurés
- Des variables
- Des jokers (wildcards)
- Des espaces réservés (placeholders)

<!--
Some example patterns include `x`, `(a, 3)`, and `Some(Color::Red)`. In the
contexts in which patterns are valid, these components describe the shape of
data. Our program then matches values against the patterns to determine whether
it has the correct shape of data to continue running a particular piece of code.
-->

Quelques exemples de motifs incluent `x`, `(a, 3)` et `Some(Color::Red)`. Dans les contextes où les motifs sont valides, ces composants décrivent la forme des données. Notre programme compare ensuite les valeurs aux motifs pour déterminer si les données ont la forme correcte pour continuer l'exécution d'un morceau de code particulier.

<!--
To use a pattern, we compare it to some value. If the pattern matches the
value, we use the value parts in our code. Recall the `match` expressions in
Chapter 6 that used patterns, such as the coin-sorting machine example. If the
value fits the shape of the pattern, we can use the named pieces. If it
doesn't, the code associated with the pattern won't run.
-->

Pour utiliser un motif, nous le comparons à une valeur. Si le motif correspond à la valeur, nous utilisons les parties de la valeur dans notre code. Rappelez-vous les expressions `match` du chapitre 6 qui utilisaient des motifs, comme l'exemple de la machine à trier les pièces de monnaie. Si la valeur correspond à la forme du motif, nous pouvons utiliser les éléments nommés. Sinon, le code associé au motif ne s'exécutera pas.

<!--
This chapter is a reference on all things related to patterns. We'll cover the
valid places to use patterns, the difference between refutable and irrefutable
patterns, and the different kinds of pattern syntax that you might see. By the
end of the chapter, you'll know how to use patterns to express many concepts in
a clear way.
-->

Ce chapitre est une référence sur tout ce qui concerne les motifs. Nous couvrirons les endroits valides pour utiliser des motifs, la différence entre les motifs réfutables et irréfutables, et les différents types de syntaxe de motifs que vous pourriez rencontrer. À la fin du chapitre, vous saurez comment utiliser les motifs pour exprimer de nombreux concepts de manière claire.
