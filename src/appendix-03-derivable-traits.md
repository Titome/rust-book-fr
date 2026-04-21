<!--
## Appendix C: Derivable Traits
-->
## Annexe C : Les traits dérivables

<!--
In various places in the book, we've discussed the `derive` attribute, which
you can apply to a struct or enum definition. The `derive` attribute generates
code that will implement a trait with its own default implementation on the
type you've annotated with the `derive` syntax.
-->
À plusieurs endroits dans ce livre, nous avons abordé l'attribut `derive`, que
vous pouvez appliquer à une définition de structure ou d'énumération. L'attribut
`derive` génère du code qui implémente un trait avec sa propre implémentation
par défaut sur le type que vous avez annoté avec la syntaxe `derive`.

<!--
In this appendix, we provide a reference of all the traits in the standard
library that you can use with `derive`. Each section covers:
-->
Dans cette annexe, nous fournissons une référence de tous les traits de la
bibliothèque standard que vous pouvez utiliser avec `derive`. Chaque section
couvre :

<!--
- What operators and methods deriving this trait will enable
- What the implementation of the trait provided by `derive` does
- What implementing the trait signifies about the type
- The conditions in which you're allowed or not allowed to implement the trait
- Examples of operations that require the trait
-->
- Quels opérateurs et méthodes la dérivation de ce trait activera
- Ce que fait l'implémentation du trait fournie par `derive`
- Ce que l'implémentation du trait signifie pour le type
- Les conditions dans lesquelles vous êtes autorisé ou non à implémenter le trait
- Des exemples d'opérations qui nécessitent le trait

<!--
If you want different behavior from that provided by the `derive` attribute,
consult the [standard library documentation](../std/index.html) ignore
-->
for each trait for details on how to manually implement them. -->
Si vous souhaitez un comportement différent de celui fourni par l'attribut
`derive`, consultez la [documentation de la bibliothèque
standard](../std/index.html)<!--
ignore
--> pour chaque trait afin d'obtenir
des détails sur la manière de les implémenter manuellement.

<!--
The traits listed here are the only ones defined by the standard library that
can be implemented on your types using `derive`. Other traits defined in the
standard library don't have sensible default behavior, so it's up to you to
implement them in the way that makes sense for what you're trying to accomplish.
-->
Les traits listés ici sont les seuls définis par la bibliothèque standard qui
peuvent être implémentés sur vos types en utilisant `derive`. Les autres traits
définis dans la bibliothèque standard n'ont pas de comportement par défaut
pertinent, c'est donc à vous de les implémenter de la manière qui correspond
à ce que vous essayez d'accomplir.

<!--
An example of a trait that can't be derived is `Display`, which handles
formatting for end users. You should always consider the appropriate way to
display a type to an end user. What parts of the type should an end user be
allowed to see? What parts would they find relevant? What format of the data
would be most relevant to them? The Rust compiler doesn't have this insight, so
it can't provide appropriate default behavior for you.
-->
Un exemple de trait qui ne peut pas être dérivé est `Display`, qui gère le
formatage pour les utilisateurs finaux. Vous devriez toujours réfléchir à la
manière appropriée d'afficher un type pour un utilisateur final. Quelles
parties du type un utilisateur final devrait-il pouvoir voir ? Quelles parties
trouverait-il pertinentes ? Quel format des données serait le plus pertinent
pour lui ? Le compilateur Rust n'a pas cette connaissance, il ne peut donc pas
fournir un comportement par défaut approprié pour vous.

<!--
The list of derivable traits provided in this appendix is not comprehensive:
Libraries can implement `derive` for their own traits, making the list of
traits you can use `derive` with truly open ended. Implementing `derive`
involves using a procedural macro, which is covered in the ["Custom `derive`
Macros"][custom-derive-macros] ignore
--> section in Chapter 20. -->
La liste des traits dérivables fournie dans cette annexe n'est pas exhaustive :
les bibliothèques peuvent implémenter `derive` pour leurs propres traits, ce
qui rend la liste des traits utilisables avec `derive` véritablement ouverte.
L'implémentation de `derive` implique l'utilisation d'une macro procédurale,
qui est traitée dans la section [« Les macros `derive`
personnalisées »][custom-derive-macros]<!--
ignore
--> du chapitre 20.

<!--
### `Debug` for Programmer Output
-->
### `Debug` pour l'affichage destiné aux développeurs

<!--
The `Debug` trait enables debug formatting in format strings, which you
indicate by adding `:?` within `{}` placeholders.
-->
Le trait `Debug` active le formatage de débogage dans les chaînes de formatage,
que vous indiquez en ajoutant `:?` à l'intérieur des espaces réservés `{}`.

<!--
The `Debug` trait allows you to print instances of a type for debugging
purposes, so you and other programmers using your type can inspect an instance
at a particular point in a program's execution.
-->
Le trait `Debug` vous permet d'afficher des instances d'un type à des fins de
débogage, afin que vous et les autres développeurs utilisant votre type puissiez
inspecter une instance à un moment précis de l'exécution d'un programme.

<!--
The `Debug` trait is required, for example, in the use of the `assert_eq!`
macro. This macro prints the values of instances given as arguments if the
equality assertion fails so that programmers can see why the two instances
weren't equal.
-->
Le trait `Debug` est requis, par exemple, lors de l'utilisation de la macro
`assert_eq!`. Cette macro affiche les valeurs des instances passées en arguments
si l'assertion d'égalité échoue, afin que les développeurs puissent voir
pourquoi les deux instances n'étaient pas égales.

<!--
### `PartialEq` and `Eq` for Equality Comparisons
-->
### `PartialEq` et `Eq` pour les comparaisons d'égalité

<!--
The `PartialEq` trait allows you to compare instances of a type to check for
equality and enables use of the `==` and `!=` operators.
-->
Le trait `PartialEq` vous permet de comparer des instances d'un type pour
vérifier l'égalité et active l'utilisation des opérateurs `==` et `!=`.

<!--
Deriving `PartialEq` implements the `eq` method. When `PartialEq` is derived on
structs, two instances are equal only if _all_ fields are equal, and the
instances are not equal if _any_ fields are not equal. When derived on enums,
each variant is equal to itself and not equal to the other variants.
-->
Dériver `PartialEq` implémente la méthode `eq`. Lorsque `PartialEq` est dérivé
sur des structures, deux instances sont égales uniquement si _tous_ les champs
sont égaux, et les instances ne sont pas égales si _un quelconque_ champ n'est
pas égal. Lorsqu'il est dérivé sur des énumérations, chaque variante est égale
à elle-même et différente des autres variantes.

<!--
The `PartialEq` trait is required, for example, with the use of the
`assert_eq!` macro, which needs to be able to compare two instances of a type
for equality.
-->
Le trait `PartialEq` est requis, par exemple, lors de l'utilisation de la macro
`assert_eq!`, qui doit pouvoir comparer deux instances d'un type pour vérifier
l'égalité.

<!--
The `Eq` trait has no methods. Its purpose is to signal that for every value of
the annotated type, the value is equal to itself. The `Eq` trait can only be
applied to types that also implement `PartialEq`, although not all types that
implement `PartialEq` can implement `Eq`. One example of this is floating-point
number types: The implementation of floating-point numbers states that two
instances of the not-a-number (`NaN`) value are not equal to each other.
-->
Le trait `Eq` n'a pas de méthodes. Son objectif est de signaler que pour chaque
valeur du type annoté, la valeur est égale à elle-même. Le trait `Eq` ne peut
être appliqué qu'aux types qui implémentent également `PartialEq`, bien que
tous les types qui implémentent `PartialEq` ne puissent pas implémenter `Eq`.
Un exemple est celui des types de nombres à virgule flottante : l'implémentation
des nombres à virgule flottante stipule que deux instances de la valeur
« pas un nombre » (`NaN`) ne sont pas égales entre elles.

<!--
An example of when `Eq` is required is for keys in a `HashMap<K, V>` so that
the `HashMap<K, V>` can tell whether two keys are the same.
-->
Un exemple où `Eq` est requis est pour les clés d'un `HashMap<K, V>`, afin que
le `HashMap<K, V>` puisse déterminer si deux clés sont identiques.

<!--
### `PartialOrd` and `Ord` for Ordering Comparisons
-->
### `PartialOrd` et `Ord` pour les comparaisons d'ordre

<!--
The `PartialOrd` trait allows you to compare instances of a type for sorting
purposes. A type that implements `PartialOrd` can be used with the `<`, `>`,
`<=`, and `>=` operators. You can only apply the `PartialOrd` trait to types
that also implement `PartialEq`.
-->
Le trait `PartialOrd` vous permet de comparer des instances d'un type à des
fins de tri. Un type qui implémente `PartialOrd` peut être utilisé avec les
opérateurs `<`, `>`, `<=` et `>=`. Vous ne pouvez appliquer le trait
`PartialOrd` qu'aux types qui implémentent également `PartialEq`.

<!--
Deriving `PartialOrd` implements the `partial_cmp` method, which returns an
`Option<Ordering>` that will be `None` when the values given don't produce an
ordering. An example of a value that doesn't produce an ordering, even though
most values of that type can be compared, is the `NaN` floating point value.
Calling `partial_cmp` with any floating-point number and the `NaN`
floating-point value will return `None`.
-->
Dériver `PartialOrd` implémente la méthode `partial_cmp`, qui retourne un
`Option<Ordering>` qui sera `None` lorsque les valeurs fournies ne produisent
pas d'ordre. Un exemple de valeur qui ne produit pas d'ordre, même si la
plupart des valeurs de ce type peuvent être comparées, est la valeur à virgule
flottante `NaN`. Appeler `partial_cmp` avec n'importe quel nombre à virgule
flottante et la valeur à virgule flottante `NaN` retournera `None`.

<!--
When derived on structs, `PartialOrd` compares two instances by comparing the
value in each field in the order in which the fields appear in the struct
definition. When derived on enums, variants of the enum declared earlier in the
enum definition are considered less than the variants listed later.
-->
Lorsqu'il est dérivé sur des structures, `PartialOrd` compare deux instances en
comparant la valeur de chaque champ dans l'ordre dans lequel les champs
apparaissent dans la définition de la structure. Lorsqu'il est dérivé sur des
énumérations, les variantes de l'énumération déclarées plus tôt dans la
définition sont considérées comme inférieures aux variantes listées après.

<!--
The `PartialOrd` trait is required, for example, for the `gen_range` method
from the `rand` crate that generates a random value in the range specified by a
range expression.
-->
Le trait `PartialOrd` est requis, par exemple, pour la méthode `gen_range` du
crate `rand` qui génère une valeur aléatoire dans l'intervalle spécifié par une
expression d'intervalle.

<!--
The `Ord` trait allows you to know that for any two values of the annotated
type, a valid ordering will exist. The `Ord` trait implements the `cmp` method,
which returns an `Ordering` rather than an `Option<Ordering>` because a valid
ordering will always be possible. You can only apply the `Ord` trait to types
that also implement `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`). When
derived on structs and enums, `cmp` behaves the same way as the derived
implementation for `partial_cmp` does with `PartialOrd`.
-->
Le trait `Ord` vous permet de savoir que pour deux valeurs quelconques du type
annoté, un ordre valide existera. Le trait `Ord` implémente la méthode `cmp`,
qui retourne un `Ordering` plutôt qu'un `Option<Ordering>` car un ordre valide
sera toujours possible. Vous ne pouvez appliquer le trait `Ord` qu'aux types
qui implémentent également `PartialOrd` et `Eq` (et `Eq` nécessite
`PartialEq`). Lorsqu'il est dérivé sur des structures et des énumérations,
`cmp` se comporte de la même manière que l'implémentation dérivée de
`partial_cmp` avec `PartialOrd`.

<!--
An example of when `Ord` is required is when storing values in a `BTreeSet<T>`,
a data structure that stores data based on the sort order of the values.
-->
Un exemple où `Ord` est requis est lors du stockage de valeurs dans un
`BTreeSet<T>`, une structure de données qui stocke les données en fonction de
l'ordre de tri des valeurs.

<!--
### `Clone` and `Copy` for Duplicating Values
-->
### `Clone` et `Copy` pour dupliquer des valeurs

<!--
The `Clone` trait allows you to explicitly create a deep copy of a value, and
the duplication process might involve running arbitrary code and copying heap
data. See the ["Variables and Data Interacting with
Clone"][variables-and-data-interacting-with-clone] ignore
--> section in
Chapter 4 for more information on `Clone`. -->
Le trait `Clone` vous permet de créer explicitement une copie en profondeur
d'une valeur, et le processus de duplication peut impliquer l'exécution de code
arbitraire et la copie de données du tas. Consultez la section [« Les variables
et les données interagissant avec
Clone »][variables-and-data-interacting-with-clone]<!--
ignore
--> du
chapitre 4 pour plus d'informations sur `Clone`.

<!--
Deriving `Clone` implements the `clone` method, which when implemented for the
whole type, calls `clone` on each of the parts of the type. This means all the
fields or values in the type must also implement `Clone` to derive `Clone`.
-->
Dériver `Clone` implémente la méthode `clone`, qui, lorsqu'elle est implémentée
pour le type entier, appelle `clone` sur chacune des parties du type. Cela
signifie que tous les champs ou valeurs du type doivent également implémenter
`Clone` pour pouvoir dériver `Clone`.

<!--
An example of when `Clone` is required is when calling the `to_vec` method on a
slice. The slice doesn't own the type instances it contains, but the vector
returned from `to_vec` will need to own its instances, so `to_vec` calls
`clone` on each item. Thus, the type stored in the slice must implement `Clone`.
-->
Un exemple où `Clone` est requis est lors de l'appel de la méthode `to_vec` sur
une slice. La slice ne possède pas les instances du type qu'elle contient, mais
le vecteur retourné par `to_vec` devra posséder ses instances, donc `to_vec`
appelle `clone` sur chaque élément. Ainsi, le type stocké dans la slice doit
implémenter `Clone`.

<!--
The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See the ["Stack-Only Data:
Copy"][stack-only-data-copy] ignore
--> section in Chapter 4 for more
information on `Copy`. -->
Le trait `Copy` vous permet de dupliquer une valeur en copiant uniquement les
bits stockés sur la pile ; aucun code arbitraire n'est nécessaire. Consultez la
section [« Les données uniquement sur la pile :
Copy »][stack-only-data-copy]<!--
ignore
--> du chapitre 4 pour plus
d'informations sur `Copy`.

<!--
The `Copy` trait doesn't define any methods to prevent programmers from
overloading those methods and violating the assumption that no arbitrary code
is being run. That way, all programmers can assume that copying a value will be
very fast.
-->
Le trait `Copy` ne définit aucune méthode afin d'empêcher les développeurs de
surcharger ces méthodes et de violer l'hypothèse qu'aucun code arbitraire n'est
exécuté. De cette façon, tous les développeurs peuvent supposer que la copie
d'une valeur sera très rapide.

<!--
You can derive `Copy` on any type whose parts all implement `Copy`. A type that
implements `Copy` must also implement `Clone` because a type that implements
`Copy` has a trivial implementation of `Clone` that performs the same task as
`Copy`.
-->
Vous pouvez dériver `Copy` sur tout type dont toutes les parties implémentent
`Copy`. Un type qui implémente `Copy` doit également implémenter `Clone` car
un type qui implémente `Copy` a une implémentation triviale de `Clone` qui
effectue la même tâche que `Copy`.

<!--
The `Copy` trait is rarely required; types that implement `Copy` have
optimizations available, meaning you don't have to call `clone`, which makes
the code more concise.
-->
Le trait `Copy` est rarement requis ; les types qui implémentent `Copy`
disposent d'optimisations, ce qui signifie que vous n'avez pas besoin d'appeler
`clone`, ce qui rend le code plus concis.

<!--
Everything possible with `Copy` you can also accomplish with `Clone`, but the
code might be slower or have to use `clone` in places.
-->
Tout ce qui est possible avec `Copy` peut également être accompli avec `Clone`,
mais le code pourrait être plus lent ou devoir utiliser `clone` par endroits.

<!--
### `Hash` for Mapping a Value to a Value of Fixed Size
-->
### `Hash` pour associer une valeur à une valeur de taille fixe

<!--
The `Hash` trait allows you to take an instance of a type of arbitrary size and
map that instance to a value of fixed size using a hash function. Deriving
`Hash` implements the `hash` method. The derived implementation of the `hash`
method combines the result of calling `hash` on each of the parts of the type,
meaning all fields or values must also implement `Hash` to derive `Hash`.
-->
Le trait `Hash` vous permet de prendre une instance d'un type de taille
arbitraire et d'associer cette instance à une valeur de taille fixe en
utilisant une fonction de hachage. Dériver `Hash` implémente la méthode `hash`.
L'implémentation dérivée de la méthode `hash` combine le résultat de l'appel
de `hash` sur chacune des parties du type, ce qui signifie que tous les champs
ou valeurs doivent également implémenter `Hash` pour pouvoir dériver `Hash`.

<!--
An example of when `Hash` is required is in storing keys in a `HashMap<K, V>`
to store data efficiently.
-->
Un exemple où `Hash` est requis est lors du stockage des clés dans un
`HashMap<K, V>` pour stocker des données efficacement.

<!--
### `Default` for Default Values
-->
### `Default` pour les valeurs par défaut

<!--
The `Default` trait allows you to create a default value for a type. Deriving
`Default` implements the `default` function. The derived implementation of the
`default` function calls the `default` function on each part of the type,
meaning all fields or values in the type must also implement `Default` to
derive `Default`.
-->
Le trait `Default` vous permet de créer une valeur par défaut pour un type.
Dériver `Default` implémente la fonction `default`. L'implémentation dérivée de
la fonction `default` appelle la fonction `default` sur chaque partie du type,
ce qui signifie que tous les champs ou valeurs du type doivent également
implémenter `Default` pour pouvoir dériver `Default`.

<!--
The `Default::default` function is commonly used in combination with the struct
update syntax discussed in the ["Creating Instances from Other Instances with
Struct Update
Syntax"][creating-instances-from-other-instances-with-struct-update-syntax]
ignore
--> section in Chapter 5. You can customize a few fields of a struct and
then set and use a default value for the rest of the fields by using
`..Default::default()`. -->
La fonction `Default::default` est couramment utilisée en combinaison avec la
syntaxe de mise à jour de structure abordée dans la section [« Créer des
instances à partir d'autres instances avec la syntaxe de mise à jour de
structure »][creating-instances-from-other-instances-with-struct-update-syntax]<!--
ignore
--> du chapitre 5. Vous pouvez personnaliser quelques champs d'une
structure puis définir et utiliser une valeur par défaut pour le reste des
champs en utilisant `..Default::default()`.

<!--
The `Default` trait is required when you use the method `unwrap_or_default` on
`Option<T>` instances, for example. If the `Option<T>` is `None`, the method
`unwrap_or_default` will return the result of `Default::default` for the type
`T` stored in the `Option<T>`.
-->
Le trait `Default` est requis lorsque vous utilisez la méthode
`unwrap_or_default` sur des instances d'`Option<T>`, par exemple. Si
l'`Option<T>` est `None`, la méthode `unwrap_or_default` retournera le résultat
de `Default::default` pour le type `T` stocké dans l'`Option<T>`.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[stack-only-data-copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
[variables-and-data-interacting-with-clone]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone
[custom-derive-macros]: ch20-05-macros.html#custom-derive-macros
