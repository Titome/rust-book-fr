<!--
## An Example Program Using Structs
-->

## Un exemple de programme utilisant les structs

<!--
To understand when we might want to use structs, let's write a program that
calculates the area of a rectangle. We'll start by using single variables and
then refactor the program until we're using structs instead.
-->

Pour comprendre quand nous pourrions vouloir utiliser des structs, écrivons un programme qui calcule l'aire d'un rectangle. Nous commencerons par utiliser des variables simples, puis nous refactoriserons le programme jusqu'à utiliser des structs à la place.

<!--
Let's make a new binary project with Cargo called _rectangles_ that will take
the width and height of a rectangle specified in pixels and calculate the area
of the rectangle. Listing 5-8 shows a short program with one way of doing
exactly that in our project's _src/main.rs_.
-->

Créons un nouveau projet binaire avec Cargo appelé _rectangles_ qui prendra la largeur et la hauteur d'un rectangle spécifiées en pixels et calculera l'aire du rectangle. Le listing 5-8 montre un court programme avec une manière de faire exactement cela dans le fichier _src/main.rs_ de notre projet.

<Listing number="5-8" file-name="src/main.rs" caption="Calcul de l'aire d'un rectangle spécifié par des variables séparées de largeur et de hauteur">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

<!--
Now, run this program using `cargo run`:
-->

Maintenant, exécutez ce programme avec `cargo run` :


```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

<!--
This code succeeds in figuring out the area of the rectangle by calling the
`area` function with each dimension, but we can do more to make this code clear
and readable.
-->

Ce code réussit à déterminer l'aire du rectangle en appelant la fonction `area` avec chaque dimension, mais nous pouvons faire davantage pour rendre ce code clair et lisible.

<!--
The issue with this code is evident in the signature of `area`:
-->

Le problème avec ce code est évident dans la signature de `area` :


```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

<!--
The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters, and it's not clear anywhere in our
program that the parameters are related. It would be more readable and more
manageable to group width and height together. We've already discussed one way
we might do that in ["The Tuple Type"][the-tuple-type] ignore
--> section
of Chapter 3: by using tuples.
-->

La fonction `area` est censée calculer l'aire d'un seul rectangle, mais la fonction que nous avons écrite a deux paramètres, et rien dans notre programme n'indique clairement que ces paramètres sont liés. Il serait plus lisible et plus facile à gérer de regrouper la largeur et la hauteur. Nous avons déjà abordé une façon de le faire dans la section [« Le type tuple »][the-tuple-type]<!--
ignore
--> du chapitre 3 : en utilisant des tuples.

<!--
### Refactoring with Tuples
-->

### Refactorisation avec des tuples

<!--
Listing 5-9 shows another version of our program that uses tuples.
-->

Le listing 5-9 montre une autre version de notre programme qui utilise des tuples.

<Listing number="5-9" file-name="src/main.rs" caption="Spécification de la largeur et de la hauteur du rectangle avec un tuple">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

<!--
In one way, this program is better. Tuples let us add a bit of structure, and
we're now passing just one argument. But in another way, this version is less
clear: Tuples don't name their elements, so we have to index into the parts of
the tuple, making our calculation less obvious.
-->

D'un côté, ce programme est meilleur. Les tuples nous permettent d'ajouter un peu de structure, et nous ne passons maintenant qu'un seul argument. Mais d'un autre côté, cette version est moins claire : les tuples ne nomment pas leurs éléments, donc nous devons accéder aux parties du tuple par index, ce qui rend notre calcul moins évident.

<!--
Mixing up the width and height wouldn't matter for the area calculation, but if
we want to draw the rectangle on the screen, it would matter! We would have to
keep in mind that `width` is the tuple index `0` and `height` is the tuple
index `1`. This would be even harder for someone else to figure out and keep in
mind if they were to use our code. Because we haven't conveyed the meaning of
our data in our code, it's now easier to introduce errors.
-->

Confondre la largeur et la hauteur n'aurait pas d'importance pour le calcul de l'aire, mais si nous voulions dessiner le rectangle à l'écran, cela compterait ! Nous devrions garder en tête que `width` est l'index `0` du tuple et `height` est l'index `1`. Ce serait encore plus difficile pour quelqu'un d'autre de comprendre et de retenir s'il devait utiliser notre code. Comme nous n'avons pas exprimé la signification de nos données dans notre code, il est maintenant plus facile d'introduire des erreurs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="refactoring-with-structs-adding-more-meaning"></a>

<!--
### Refactoring with Structs
-->

### Refactorisation avec des structs

<!--
We use structs to add meaning by labeling the data. We can transform the tuple
we're using into a struct with a name for the whole as well as names for the
parts, as shown in Listing 5-10.
-->

Nous utilisons les structs pour ajouter du sens en étiquetant les données. Nous pouvons transformer le tuple que nous utilisons en une struct avec un nom pour l'ensemble ainsi que des noms pour les parties, comme le montre le listing 5-10.

<Listing number="5-10" file-name="src/main.rs" caption="Définition d'une struct `Rectangle`">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

<!--
Here, we've defined a struct and named it `Rectangle`. Inside the curly
brackets, we defined the fields as `width` and `height`, both of which have
type `u32`. Then, in `main`, we created a particular instance of `Rectangle`
that has a width of `30` and a height of `50`.
-->

Ici, nous avons défini une struct et l'avons nommée `Rectangle`. À l'intérieur des accolades, nous avons défini les champs `width` et `height`, qui sont tous les deux de type `u32`. Ensuite, dans `main`, nous avons créé une instance particulière de `Rectangle` avec une largeur de `30` et une hauteur de `50`.

<!--
Our `area` function is now defined with one parameter, which we've named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.
-->

Notre fonction `area` est maintenant définie avec un seul paramètre, que nous avons nommé `rectangle`, dont le type est un emprunt immutable d'une instance de la struct `Rectangle`. Comme mentionné au chapitre 4, nous voulons emprunter la struct plutôt que d'en prendre possession. De cette façon, `main` conserve la possession et peut continuer à utiliser `rect1`, c'est pourquoi nous utilisons le `&` dans la signature de la fonction et à l'endroit où nous appelons la fonction.

<!--
The `area` function accesses the `width` and `height` fields of the `Rectangle`
instance (note that accessing fields of a borrowed struct instance does not
move the field values, which is why you often see borrows of structs). Our
function signature for `area` now says exactly what we mean: Calculate the area
of `Rectangle`, using its `width` and `height` fields. This conveys that the
width and height are related to each other, and it gives descriptive names to
the values rather than using the tuple index values of `0` and `1`. This is a
win for clarity.
-->

La fonction `area` accède aux champs `width` et `height` de l'instance de `Rectangle` (notez que l'accès aux champs d'une instance de struct empruntée ne déplace pas les valeurs des champs, c'est pourquoi vous voyez souvent des emprunts de structs). Notre signature de fonction pour `area` dit maintenant exactement ce que nous voulons dire : calculer l'aire d'un `Rectangle` en utilisant ses champs `width` et `height`. Cela exprime que la largeur et la hauteur sont liées entre elles, et cela donne des noms descriptifs aux valeurs plutôt que d'utiliser les valeurs d'index de tuple `0` et `1`. C'est un gain de clarté.

<!--
Old headings. Do not remove or links may break.
-->

<a id="adding-useful-functionality-with-derived-traits"></a>

<!--
### Adding Functionality with Derived Traits
-->

### Ajouter des fonctionnalités avec les traits dérivés

<!--
It'd be useful to be able to print an instance of `Rectangle` while we're
debugging our program and see the values for all its fields. Listing 5-11 tries
using the [`println!` macro][println] ignore
--> as we have used in
previous chapters. This won't work, however.
-->

Il serait utile de pouvoir afficher une instance de `Rectangle` pendant que nous déboguons notre programme et de voir les valeurs de tous ses champs. Le listing 5-11 essaie d'utiliser la [macro `println!`][println]<!--
ignore
--> comme nous l'avons fait dans les chapitres précédents. Cependant, cela ne fonctionnera pas.

<Listing number="5-11" file-name="src/main.rs" caption="Tentative d'affichage d'une instance de `Rectangle`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

<!--
When we compile this code, we get an error with this core message:
-->

Lorsque nous compilons ce code, nous obtenons une erreur avec ce message principal :


```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

<!--
The `println!` macro can do many kinds of formatting, and by default, the curly
brackets tell `println!` to use formatting known as `Display`: output intended
for direct end user consumption. The primitive types we've seen so far
implement `Display` by default because there's only one way you'd want to show
a `1` or any other primitive type to a user. But with structs, the way
`println!` should format the output is less clear because there are more
display possibilities: Do you want commas or not? Do you want to print the
curly brackets? Should all the fields be shown? Due to this ambiguity, Rust
doesn't try to guess what we want, and structs don't have a provided
implementation of `Display` to use with `println!` and the `{}` placeholder.
-->

La macro `println!` peut effectuer de nombreux types de formatage, et par défaut, les accolades indiquent à `println!` d'utiliser le formatage connu sous le nom de `Display` : une sortie destinée à la consommation directe par l'utilisateur final. Les types primitifs que nous avons vus jusqu'ici implémentent `Display` par défaut car il n'y a qu'une seule façon de montrer un `1` ou tout autre type primitif à un utilisateur. Mais avec les structs, la façon dont `println!` devrait formater la sortie est moins claire car il y a plus de possibilités d'affichage : voulez-vous des virgules ou non ? Voulez-vous afficher les accolades ? Tous les champs doivent-ils être affichés ? En raison de cette ambiguïté, Rust n'essaie pas de deviner ce que nous voulons, et les structs n'ont pas d'implémentation fournie de `Display` à utiliser avec `println!` et le placeholder `{}`.

<!--
If we continue reading the errors, we'll find this helpful note:
-->

Si nous continuons à lire les erreurs, nous trouverons cette note utile :


```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

<!--
Let's try it! The `println!` macro call will now look like `println!("rect1 is
{rect1:?}");`. Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`. The `Debug` trait
enables us to print our struct in a way that is useful for developers so that
we can see its value while we're debugging our code.
-->

Essayons ! L'appel à la macro `println!` ressemblera maintenant à `println!("rect1 is {rect1:?}");`. Mettre le spécificateur `:?` à l'intérieur des accolades indique à `println!` que nous voulons utiliser un format de sortie appelé `Debug`. Le trait `Debug` nous permet d'afficher notre struct d'une manière utile pour les développeurs afin que nous puissions voir sa valeur pendant que nous déboguons notre code.

<!--
Compile the code with this change. Drat! We still get an error:
-->

Compilez le code avec cette modification. Zut ! Nous obtenons toujours une erreur :


```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

<!--
But again, the compiler gives us a helpful note:
-->

Mais encore une fois, le compilateur nous donne une note utile :


```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

<!--
Rust _does_ include functionality to print out debugging information, but we
have to explicitly opt in to make that functionality available for our struct.
To do that, we add the outer attribute `#[derive(Debug)]` just before the
struct definition, as shown in Listing 5-12.
-->

Rust _inclut bien_ la fonctionnalité pour afficher des informations de débogage, mais nous devons explicitement l'activer pour rendre cette fonctionnalité disponible pour notre struct. Pour ce faire, nous ajoutons l'attribut externe `#[derive(Debug)]` juste avant la définition de la struct, comme le montre le listing 5-12.

<Listing number="5-12" file-name="src/main.rs" caption="Ajout de l'attribut pour dériver le trait `Debug` et affichage de l'instance de `Rectangle` en utilisant le formatage de débogage">


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

<!--
Now when we run the program, we won't get any errors, and we'll see the
following output:
-->

Maintenant, quand nous exécutons le programme, nous n'obtiendrons aucune erreur, et nous verrons la sortie suivante :


```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

<!--
Nice! It's not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it's useful to have output that's a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string. In
this example, using the `{:#?}` style will output the following:
-->

Bien ! Ce n'est pas la plus jolie sortie, mais elle montre les valeurs de tous les champs pour cette instance, ce qui aiderait certainement lors du débogage. Quand nous avons des structs plus grandes, il est utile d'avoir une sortie un peu plus facile à lire ; dans ces cas-là, nous pouvons utiliser `{:#?}` au lieu de `{:?}` dans la chaîne de `println!`. Dans cet exemple, utiliser le style `{:#?}` produira la sortie suivante :


```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

<!--
Another way to print out a value using the `Debug` format is to use the [`dbg!`
macro][dbg] ignore
-->, which takes ownership of an expression (as opposed
to `println!`, which takes a reference), prints the file and line number of
where that `dbg!` macro call occurs in your code along with the resultant value
of that expression, and returns ownership of the value.
-->

Une autre façon d'afficher une valeur en utilisant le format `Debug` est d'utiliser la [macro `dbg!`][dbg]<!--
ignore
-->, qui prend possession d'une expression (contrairement à `println!`, qui prend une référence), affiche le fichier et le numéro de ligne où l'appel à la macro `dbg!` se produit dans votre code ainsi que la valeur résultante de cette expression, et retourne la possession de la valeur.

<!--
> Note: Calling the `dbg!` macro prints to the standard error console stream
> (`stderr`), as opposed to `println!`, which prints to the standard output
> console stream (`stdout`). We'll talk more about `stderr` and `stdout` in the
> ["Redirecting Errors to Standard Error" section in Chapter
> 12][err] ignore
-->.
-->

> Note : L'appel à la macro `dbg!` affiche sur le flux d'erreur standard de la
> console (`stderr`), contrairement à `println!`, qui affiche sur le flux de
> sortie standard de la console (`stdout`). Nous parlerons davantage de `stderr`
> et `stdout` dans la section [« Rediriger les erreurs vers la sortie d'erreur
> standard » du chapitre 12][err]<!--
ignore
-->.

<!--
Here's an example where we're interested in the value that gets assigned to the
`width` field, as well as the value of the whole struct in `rect1`:
-->

Voici un exemple où nous nous intéressons à la valeur assignée au champ `width`, ainsi qu'à la valeur de la struct entière dans `rect1` :


```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

<!--
We can put `dbg!` around the expression `30 * scale` and, because `dbg!`
returns ownership of the expression's value, the `width` field will get the
same value as if we didn't have the `dbg!` call there. We don't want `dbg!` to
take ownership of `rect1`, so we use a reference to `rect1` in the next call.
Here's what the output of this example looks like:
-->

Nous pouvons placer `dbg!` autour de l'expression `30 * scale` et, comme `dbg!` retourne la possession de la valeur de l'expression, le champ `width` recevra la même valeur que si nous n'avions pas l'appel à `dbg!` à cet endroit. Nous ne voulons pas que `dbg!` prenne possession de `rect1`, donc nous utilisons une référence vers `rect1` dans l'appel suivant. Voici à quoi ressemble la sortie de cet exemple :


```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

<!--
We can see the first bit of output came from _src/main.rs_ line 10 where we're
debugging the expression `30 * scale`, and its resultant value is `60` (the
`Debug` formatting implemented for integers is to print only their value). The
`dbg!` call on line 14 of _src/main.rs_ outputs the value of `&rect1`, which is
the `Rectangle` struct. This output uses the pretty `Debug` formatting of the
`Rectangle` type. The `dbg!` macro can be really helpful when you're trying to
figure out what your code is doing!
-->

Nous pouvons voir que la première partie de la sortie provient de _src/main.rs_ ligne 10 où nous déboguons l'expression `30 * scale`, et sa valeur résultante est `60` (le formatage `Debug` implémenté pour les entiers consiste à afficher uniquement leur valeur). L'appel à `dbg!` à la ligne 14 de _src/main.rs_ affiche la valeur de `&rect1`, qui est la struct `Rectangle`. Cette sortie utilise le joli formatage `Debug` du type `Rectangle`. La macro `dbg!` peut être vraiment utile quand vous essayez de comprendre ce que fait votre code !

<!--
In addition to the `Debug` trait, Rust has provided a number of traits for us
to use with the `derive` attribute that can add useful behavior to our custom
types. Those traits and their behaviors are listed in [Appendix C][app-c]
ignore
-->. We'll cover how to implement these traits with custom behavior as
well as how to create your own traits in Chapter 10. There are also many
attributes other than `derive`; for more information, see [the "Attributes"
section of the Rust Reference][attributes].
-->

En plus du trait `Debug`, Rust fournit un certain nombre de traits à utiliser avec l'attribut `derive` qui peuvent ajouter des comportements utiles à nos types personnalisés. Ces traits et leurs comportements sont listés dans l'[annexe C][app-c]<!--
ignore
-->. Nous verrons comment implémenter ces traits avec un comportement personnalisé ainsi que comment créer vos propres traits au chapitre 10. Il existe également de nombreux attributs autres que `derive` ; pour plus d'informations, consultez [la section « Attributes » de la référence Rust][attributes].

<!--
Our `area` function is very specific: It only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle` struct
because it won't work with any other type. Let's look at how we can continue to
refactor this code by turning the `area` function into an `area` method
defined on our `Rectangle` type.
-->

Notre fonction `area` est très spécifique : elle ne calcule que l'aire de rectangles. Il serait utile de lier ce comportement plus étroitement à notre struct `Rectangle` car il ne fonctionnera avec aucun autre type. Voyons comment nous pouvons continuer à refactoriser ce code en transformant la fonction `area` en une méthode `area` définie sur notre type `Rectangle`.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
