<!--
# Generic Types, Traits, and Lifetimes
-->

# Les types génériques, les traits et les durées de vie

<!--
Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is _generics_: abstract stand-ins for
concrete types or other properties. We can express the behavior of generics or
how they relate to other generics without knowing what will be in their place
when compiling and running the code.
-->

Chaque langage de programmation dispose d'outils pour gérer efficacement la
duplication de concepts. En Rust, l'un de ces outils est la _généricité_ :
des substituts abstraits pour des types concrets ou d'autres propriétés. Nous
pouvons exprimer le comportement des génériques ou la manière dont ils
interagissent entre eux sans savoir ce qui les remplacera lors de la
compilation et de l'exécution du code.

<!--
Functions can take parameters of some generic type, instead of a concrete type
like `i32` or `String`, in the same way they take parameters with unknown
values to run the same code on multiple concrete values. In fact, we already
used generics in Chapter 6 with `Option<T>`, in Chapter 8 with `Vec<T>` and
`HashMap<K, V>`, and in Chapter 9 with `Result<T, E>`. In this chapter, you'll
explore how to define your own types, functions, and methods with generics!
-->

Les fonctions peuvent prendre des paramètres d'un type générique, plutôt qu'un
type concret comme `i32` ou `String`, de la même manière qu'elles prennent des
paramètres avec des valeurs inconnues pour exécuter le même code sur plusieurs
valeurs concrètes. En fait, nous avons déjà utilisé les génériques au
chapitre 6 avec `Option<T>`, au chapitre 8 avec `Vec<T>` et `HashMap<K, V>`,
et au chapitre 9 avec `Result<T, E>`. Dans ce chapitre, vous découvrirez
comment définir vos propres types, fonctions et méthodes avec des génériques !

<!--
First, we'll review how to extract a function to reduce code duplication. We'll
then use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We'll also explain how to use
generic types in struct and enum definitions.
-->

D'abord, nous reverrons comment extraire une fonction pour réduire la
duplication de code. Nous utiliserons ensuite la même technique pour créer une
fonction générique à partir de deux fonctions qui ne diffèrent que par les
types de leurs paramètres. Nous expliquerons aussi comment utiliser les types
génériques dans les définitions de structs et d'énumérations.

<!--
Then, you'll learn how to use traits to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to accept
only those types that have a particular behavior, as opposed to just any type.
-->

Ensuite, vous apprendrez à utiliser les traits pour définir un comportement
de manière générique. Vous pouvez combiner les traits avec les types génériques
pour contraindre un type générique à n'accepter que les types qui ont un
comportement particulier, plutôt que n'importe quel type.

<!--
Finally, we'll discuss _lifetimes_: a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to give the compiler enough information about borrowed values so that it can
ensure that references will be valid in more situations than it could without
our help.
-->

Enfin, nous aborderons les _durées de vie_ (lifetimes) : une variété de
génériques qui donnent au compilateur des informations sur la manière dont les
références sont liées entre elles. Les durées de vie nous permettent de donner
au compilateur suffisamment d'informations sur les valeurs empruntées pour
qu'il puisse garantir que les références seront valides dans plus de situations
qu'il ne le pourrait sans notre aide.

<!--
## Removing Duplication by Extracting a Function
-->

## Éliminer la duplication en extrayant une fonction

<!--
Generics allow us to replace specific types with a placeholder that represents
multiple types to remove code duplication. Before diving into generics syntax,
let's first look at how to remove duplication in a way that doesn't involve
generic types by extracting a function that replaces specific values with a
placeholder that represents multiple values. Then, we'll apply the same
technique to extract a generic function! By looking at how to recognize
duplicated code you can extract into a function, you'll start to recognize
duplicated code that can use generics.
-->

Les génériques nous permettent de remplacer des types spécifiques par un
espace réservé qui représente plusieurs types afin d'éliminer la duplication
de code. Avant de plonger dans la syntaxe des génériques, voyons d'abord
comment éliminer la duplication d'une manière qui n'implique pas de types
génériques, en extrayant une fonction qui remplace des valeurs spécifiques par
un espace réservé représentant plusieurs valeurs. Ensuite, nous appliquerons
la même technique pour extraire une fonction générique ! En apprenant à
reconnaître le code dupliqué que vous pouvez extraire dans une fonction, vous
commencerez à reconnaître le code dupliqué qui peut utiliser des génériques.

<!--
We'll begin with the short program in Listing 10-1 that finds the largest
number in a list.
-->

Nous commencerons par le court programme de l'encart 10-1 qui trouve le plus
grand nombre dans une liste.

<Listing number="10-1" file-name="src/main.rs" caption="Trouver le plus grand nombre dans une liste de nombres">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

<!--
We store a list of integers in the variable `number_list` and place a reference
to the first number in the list in a variable named `largest`. We then iterate
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, we replace the reference in that variable.
However, if the current number is less than or equal to the largest number seen
so far, the variable doesn't change, and the code moves on to the next number
in the list. After considering all the numbers in the list, `largest` should
refer to the largest number, which in this case is 100.
-->

Nous stockons une liste d'entiers dans la variable `number_list` et plaçons
une référence vers le premier nombre de la liste dans une variable nommée
`largest`. Nous itérons ensuite sur tous les nombres de la liste, et si le
nombre actuel est supérieur au nombre stocké dans `largest`, nous remplaçons
la référence dans cette variable. En revanche, si le nombre actuel est
inférieur ou égal au plus grand nombre rencontré jusqu'à présent, la variable
ne change pas, et le code passe au nombre suivant dans la liste. Après avoir
examiné tous les nombres de la liste, `largest` devrait référencer le plus
grand nombre, qui dans ce cas est 100.

<!--
We've now been tasked with finding the largest number in two different lists of
numbers. To do so, we can choose to duplicate the code in Listing 10-1 and use
the same logic at two different places in the program, as shown in Listing 10-2.
-->

On nous demande maintenant de trouver le plus grand nombre dans deux listes
de nombres différentes. Pour ce faire, nous pouvons choisir de dupliquer le
code de l'encart 10-1 et utiliser la même logique à deux endroits différents
du programme, comme le montre l'encart 10-2.

<Listing number="10-2" file-name="src/main.rs" caption="Code pour trouver le plus grand nombre dans *deux* listes de nombres">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

<!--
Although this code works, duplicating code is tedious and error-prone. We also
have to remember to update the code in multiple places when we want to change
it.
-->

Bien que ce code fonctionne, dupliquer du code est fastidieux et source
d'erreurs. Nous devons aussi nous souvenir de mettre à jour le code à
plusieurs endroits lorsque nous voulons le modifier.

<!--
To eliminate this duplication, we'll create an abstraction by defining a
function that operates on any list of integers passed in as a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.
-->

Pour éliminer cette duplication, nous allons créer une abstraction en
définissant une fonction qui opère sur n'importe quelle liste d'entiers
passée en paramètre. Cette solution rend notre code plus clair et nous permet
d'exprimer de manière abstraite le concept de recherche du plus grand nombre
dans une liste.

<!--
In Listing 10-3, we extract the code that finds the largest number into a
function named `largest`. Then, we call the function to find the largest number
in the two lists from Listing 10-2. We could also use the function on any other
list of `i32` values we might have in the future.
-->

Dans l'encart 10-3, nous extrayons le code qui trouve le plus grand nombre
dans une fonction nommée `largest`. Ensuite, nous appelons la fonction pour
trouver le plus grand nombre dans les deux listes de l'encart 10-2. Nous
pourrions aussi utiliser la fonction sur n'importe quelle autre liste de
valeurs `i32` que nous pourrions avoir à l'avenir.

<Listing number="10-3" file-name="src/main.rs" caption="Code abstrait pour trouver le plus grand nombre dans deux listes">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

<!--
The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values we might pass into the function. As a result,
when we call the function, the code runs on the specific values that we pass
in.
-->

La fonction `largest` a un paramètre appelé `list`, qui représente n'importe
quelle slice concrète de valeurs `i32` que nous pourrions passer à la
fonction. En conséquence, lorsque nous appelons la fonction, le code s'exécute
sur les valeurs spécifiques que nous lui passons.

<!--
In summary, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:
-->

En résumé, voici les étapes que nous avons suivies pour transformer le code
de l'encart 10-2 en l'encart 10-3 :

<!--
1. Identify duplicate code.
1. Extract the duplicate code into the body of the function, and specify the
   inputs and return values of that code in the function signature.
1. Update the two instances of duplicated code to call the function instead.
-->

1. Identifier le code dupliqué.
1. Extraire le code dupliqué dans le corps de la fonction, et spécifier les
   entrées et valeurs de retour de ce code dans la signature de la fonction.
1. Mettre à jour les deux instances de code dupliqué pour appeler la fonction
   à la place.

<!--
Next, we'll use these same steps with generics to reduce code duplication. In
the same way that the function body can operate on an abstract `list` instead
of specific values, generics allow code to operate on abstract types.
-->

Ensuite, nous utiliserons ces mêmes étapes avec les génériques pour réduire
la duplication de code. De la même manière que le corps de la fonction peut
opérer sur une `list` abstraite plutôt que sur des valeurs spécifiques, les
génériques permettent au code d'opérer sur des types abstraits.

<!--
For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let's find out!
-->

Par exemple, supposons que nous ayons deux fonctions : l'une qui trouve le plus
grand élément dans une slice de valeurs `i32` et l'autre qui trouve le plus
grand élément dans une slice de valeurs `char`. Comment éliminerions-nous cette
duplication ? Découvrons-le !
