<!--
## Macros
-->

## Les macros

<!--
We've used macros like `println!` throughout this book, but we haven't fully
explored what a macro is and how it works. The term _macro_ refers to a family
of features in Rust—declarative macros with `macro_rules!` and three kinds of
procedural macros:
-->

Nous avons utilisé des macros comme `println!` tout au long de ce livre, mais nous n'avons pas exploré en détail ce qu'est une macro et comment elle fonctionne. Le terme _macro_ fait référence à une famille de fonctionnalités en Rust -- les macros déclaratives avec `macro_rules!` et trois types de macros procédurales :

<!--
- Custom `#[derive]` macros that specify code added with the `derive` attribute
  used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens
  specified as their argument
-->

- Les macros `#[derive]` personnalisées qui spécifient du code ajouté avec l'attribut `derive` utilisé sur les structures et les enums
- Les macros de type attribut qui définissent des attributs personnalisés utilisables sur n'importe quel élément
- Les macros de type fonction qui ressemblent à des appels de fonction mais opèrent sur les tokens spécifiés comme arguments

<!--
We'll talk about each of these in turn, but first, let's look at why we even
need macros when we already have functions.
-->

Nous allons parler de chacune d'elles à tour de rôle, mais d'abord, voyons pourquoi nous avons même besoin de macros alors que nous avons déjà des fonctions.

<!--
### The Difference Between Macros and Functions
-->

### La différence entre les macros et les fonctions

<!--
Fundamentally, macros are a way of writing code that writes other code, which
is known as _metaprogramming_. In Appendix C, we discuss the `derive`
attribute, which generates an implementation of various traits for you. We've
also used the `println!` and `vec!` macros throughout the book. All of these
macros _expand_ to produce more code than the code you've written manually.
-->

Fondamentalement, les macros sont un moyen d'écrire du code qui écrit d'autre code, ce qu'on appelle la _métaprogrammation_. Dans l'annexe C, nous discutons de l'attribut `derive`, qui génère une implémentation de divers traits pour vous. Nous avons aussi utilisé les macros `println!` et `vec!` tout au long du livre. Toutes ces macros _se développent_ pour produire plus de code que le code que vous avez écrit manuellement.

<!--
Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don't have.
-->

La métaprogrammation est utile pour réduire la quantité de code que vous devez écrire et maintenir, ce qui est aussi l'un des rôles des fonctions. Cependant, les macros possèdent des pouvoirs supplémentaires que les fonctions n'ont pas.

<!--
A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: We can call `println!("hello")` with one argument or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can't, because it gets
called at runtime and a trait needs to be implemented at compile time.
-->

Une signature de fonction doit déclarer le nombre et le type de paramètres de la fonction. Les macros, en revanche, peuvent prendre un nombre variable de paramètres : nous pouvons appeler `println!("hello")` avec un argument ou `println!("hello {}", name)` avec deux arguments. De plus, les macros sont développées avant que le compilateur n'interprète la signification du code, donc une macro peut, par exemple, implémenter un trait sur un type donné. Une fonction ne le peut pas, car elle est appelée à l'exécution et un trait doit être implémenté au moment de la compilation.

<!--
The downside to implementing a macro instead of a function is that macro
definitions are more complex than function definitions because you're writing
Rust code that writes Rust code. Due to this indirection, macro definitions are
generally more difficult to read, understand, and maintain than function
definitions.
-->

L'inconvénient d'implémenter une macro au lieu d'une fonction est que les définitions de macros sont plus complexes que les définitions de fonctions car vous écrivez du code Rust qui écrit du code Rust. En raison de cette indirection, les définitions de macros sont généralement plus difficiles à lire, comprendre et maintenir que les définitions de fonctions.

<!--
Another important difference between macros and functions is that you must
define macros or bring them into scope _before_ you call them in a file, as
opposed to functions you can define anywhere and call anywhere.
-->

Une autre différence importante entre les macros et les fonctions est que vous devez définir les macros ou les importer dans la portée _avant_ de les appeler dans un fichier, contrairement aux fonctions que vous pouvez définir n'importe où et appeler n'importe où.

<!--
Old headings. Do not remove or links may break.
-->

<a id="declarative-macros-with-macro_rules-for-general-metaprogramming"></a>

<!--
### Declarative Macros for General Metaprogramming
-->

### Les macros déclaratives pour la métaprogrammation générale

<!--
The most widely used form of macros in Rust is the _declarative macro_. These
are also sometimes referred to as "macros by example," "`macro_rules!` macros,"
or just plain "macros." At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resultant value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that are
associated with particular code: In this situation, the value is the literal
Rust source code passed to the macro; the patterns are compared with the
structure of that source code; and the code associated with each pattern, when
matched, replaces the code passed to the macro. This all happens during
compilation.
-->

La forme de macros la plus largement utilisée en Rust est la _macro déclarative_. Celles-ci sont aussi parfois appelées "macros par l'exemple", "macros `macro_rules!`" ou simplement "macros". En leur coeur, les macros déclaratives vous permettent d'écrire quelque chose de similaire à une expression `match` de Rust. Comme discuté au chapitre 6, les expressions `match` sont des structures de contrôle qui prennent une expression, comparent la valeur résultante de l'expression à des motifs, puis exécutent le code associé au motif correspondant. Les macros comparent aussi une valeur à des motifs associés à du code particulier : dans cette situation, la valeur est le code source Rust littéral passé à la macro ; les motifs sont comparés à la structure de ce code source ; et le code associé à chaque motif, lorsqu'il correspond, remplace le code passé à la macro. Tout cela se produit pendant la compilation.

<!--
To define a macro, you use the `macro_rules!` construct. Let's explore how to
use `macro_rules!` by looking at how the `vec!` macro is defined. Chapter 8
covered how we can use the `vec!` macro to create a new vector with particular
values. For example, the following macro creates a new vector containing three
integers:
-->

Pour définir une macro, vous utilisez la construction `macro_rules!`. Explorons comment utiliser `macro_rules!` en regardant comment la macro `vec!` est définie. Le chapitre 8 a couvert comment nous pouvons utiliser la macro `vec!` pour créer un nouveau vecteur avec des valeurs particulières. Par exemple, la macro suivante crée un nouveau vecteur contenant trois entiers :

<!--
```rust
let v: Vec<u32> = vec![1, 2, 3];
```
-->

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

<!--
We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices. We wouldn't be able to use a function to do the same
because we wouldn't know the number or type of values up front.
-->

Nous pourrions aussi utiliser la macro `vec!` pour créer un vecteur de deux entiers ou un vecteur de cinq slices de chaînes. Nous ne pourrions pas utiliser une fonction pour faire la même chose car nous ne connaîtrions pas le nombre ou le type de valeurs à l'avance.

<!--
Listing 20-35 shows a slightly simplified definition of the `vec!` macro.
-->

L'encart 20-35 montre une définition légèrement simplifiée de la macro `vec!`.

<Listing number="20-35" file-name="src/lib.rs" caption="Une version simplifiée de la définition de la macro `vec!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

<!--
> Note: The actual definition of the `vec!` macro in the standard library
> includes code to pre-allocate the correct amount of memory up front. That code
> is an optimization that we don't include here, to make the example simpler.
-->

> Remarque : la définition réelle de la macro `vec!` dans la bibliothèque standard inclut du code pour pré-allouer la bonne quantité de mémoire à l'avance. Ce code est une optimisation que nous n'incluons pas ici, pour simplifier l'exemple.

<!--
The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which the macro is defined is brought into
scope. Without this annotation, the macro can't be brought into scope.
-->

L'annotation `#[macro_export]` indique que cette macro devrait être rendue disponible chaque fois que le crate dans lequel la macro est définie est importé dans la portée. Sans cette annotation, la macro ne peut pas être importée dans la portée.

<!--
We then start the macro definition with `macro_rules!` and the name of the
macro we're defining _without_ the exclamation mark. The name, in this case
`vec`, is followed by curly brackets denoting the body of the macro definition.
-->

Nous commençons ensuite la définition de la macro avec `macro_rules!` et le nom de la macro que nous définissons _sans_ le point d'exclamation. Le nom, dans ce cas `vec`, est suivi d'accolades indiquant le corps de la définition de la macro.

<!--
The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there is only one valid way to match; any
other pattern will result in an error. More complex macros will have more than
one arm.
-->

La structure dans le corps de `vec!` est similaire à la structure d'une expression `match`. Ici nous avons une branche avec le motif `( $( $x:expr ),* )`, suivi de `=>` et du bloc de code associé à ce motif. Si le motif correspond, le bloc de code associé sera émis. Étant donné que c'est le seul motif dans cette macro, il n'y a qu'une seule manière valide de correspondre ; tout autre motif entraînera une erreur. Les macros plus complexes auront plus d'une branche.

<!--
Valid pattern syntax in macro definitions is different from the pattern syntax
covered in Chapter 19 because macro patterns are matched against Rust code
structure rather than values. Let's walk through what the pattern pieces in
Listing 20-29 mean; for the full macro pattern syntax, see the [Rust
Reference][ref].
-->

La syntaxe de motifs valide dans les définitions de macros est différente de la syntaxe de motifs couverte au chapitre 19 car les motifs de macros sont comparés à la structure du code Rust plutôt qu'à des valeurs. Parcourons ce que signifient les éléments du motif dans l'encart 20-29 ; pour la syntaxe complète des motifs de macros, consultez la [Référence Rust][ref].

<!--
First, we use a set of parentheses to encompass the whole pattern. We use a
dollar sign (`$`) to declare a variable in the macro system that will contain
the Rust code matching the pattern. The dollar sign makes it clear this is a
macro variable as opposed to a regular Rust variable. Next comes a set of
parentheses that captures values that match the pattern within the parentheses
for use in the replacement code. Within `$()` is `$x:expr`, which matches any
Rust expression and gives the expression the name `$x`.
-->

D'abord, nous utilisons un jeu de parenthèses pour englober tout le motif. Nous utilisons un signe dollar (`$`) pour déclarer une variable dans le système de macros qui contiendra le code Rust correspondant au motif. Le signe dollar indique clairement qu'il s'agit d'une variable de macro par opposition à une variable Rust normale. Ensuite vient un jeu de parenthèses qui capture les valeurs correspondant au motif à l'intérieur des parenthèses pour utilisation dans le code de remplacement. À l'intérieur de `$()` se trouve `$x:expr`, qui correspond à n'importe quelle expression Rust et donne à l'expression le nom `$x`.

<!--
The comma following `$()` indicates that a literal comma separator character
must appear between each instance of the code that matches the code in `$()`.
The `*` specifies that the pattern matches zero or more of whatever precedes
the `*`.
-->

La virgule suivant `$()` indique qu'un caractère séparateur virgule littéral doit apparaître entre chaque instance du code correspondant au code dans `$()`. Le `*` spécifie que le motif correspond à zéro ou plusieurs occurrences de ce qui précède le `*`.

<!--
When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.
-->

Lorsque nous appelons cette macro avec `vec![1, 2, 3];`, le motif `$x` correspond trois fois avec les trois expressions `1`, `2` et `3`.

<!--
Now let's look at the pattern in the body of the code associated with this arm:
`temp_vec.push()` within `$()*` is generated for each part that matches `$()`
in the pattern zero or more times depending on how many times the pattern
matches. The `$x` is replaced with each expression matched. When we call this
macro with `vec![1, 2, 3];`, the code generated that replaces this macro call
will be the following:
-->

Maintenant regardons le motif dans le corps du code associé à cette branche : `temp_vec.push()` à l'intérieur de `$()*` est généré pour chaque partie qui correspond à `$()` dans le motif zéro ou plusieurs fois selon le nombre de fois que le motif correspond. Le `$x` est remplacé par chaque expression correspondante. Lorsque nous appelons cette macro avec `vec![1, 2, 3];`, le code généré qui remplace cet appel de macro sera le suivant :

<!--
```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```
-->

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

<!--
We've defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.
-->

Nous avons défini une macro qui peut prendre n'importe quel nombre d'arguments de n'importe quel type et peut générer du code pour créer un vecteur contenant les éléments spécifiés.

<!--
To learn more about how to write macros, consult the online documentation or
other resources, such as ["The Little Book of Rust Macros"][tlborm] started by
Daniel Keep and continued by Lukas Wirth.
-->

Pour en apprendre plus sur l'écriture de macros, consultez la documentation en ligne ou d'autres ressources, comme ["The Little Book of Rust Macros"][tlborm] commencé par Daniel Keep et continué par Lukas Wirth.

<!--
### Procedural Macros for Generating Code from Attributes
-->

### Les macros procédurales pour générer du code à partir d'attributs

<!--
The second form of macros is the procedural macro, which acts more like a
function (and is a type of procedure). _Procedural macros_ accept some code as
an input, operate on that code, and produce some code as an output rather than
matching against patterns and replacing the code with other code as declarative
macros do. The three kinds of procedural macros are custom `derive`,
attribute-like, and function-like, and all work in a similar fashion.
-->

La deuxième forme de macros est la macro procédurale, qui agit davantage comme une fonction (et est un type de procédure). Les _macros procédurales_ acceptent du code en entrée, opèrent sur ce code et produisent du code en sortie plutôt que de faire correspondre des motifs et de remplacer le code par un autre code comme le font les macros déclaratives. Les trois types de macros procédurales sont les macros `derive` personnalisées, les macros de type attribut et les macros de type fonction, et toutes fonctionnent de manière similaire.

<!--
When creating procedural macros, the definitions must reside in their own crate
with a special crate type. This is for complex technical reasons that we hope
to eliminate in the future. In Listing 20-36, we show how to define a
procedural macro, where `some_attribute` is a placeholder for using a specific
macro variety.
-->

Lors de la création de macros procédurales, les définitions doivent résider dans leur propre crate avec un type de crate spécial. C'est pour des raisons techniques complexes que nous espérons éliminer à l'avenir. Dans l'encart 20-36, nous montrons comment définir une macro procédurale, où `some_attribute` est un espace réservé pour l'utilisation d'une variété spécifique de macro.

<Listing number="20-36" file-name="src/lib.rs" caption="Un exemple de définition d'une macro procédurale">

```rust,ignore
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

</Listing>

<!--
The function that defines a procedural macro takes a `TokenStream` as an input
and produces a `TokenStream` as an output. The `TokenStream` type is defined by
the `proc_macro` crate that is included with Rust and represents a sequence of
tokens. This is the core of the macro: The source code that the macro is
operating on makes up the input `TokenStream`, and the code the macro produces
is the output `TokenStream`. The function also has an attribute attached to it
that specifies which kind of procedural macro we're creating. We can have
multiple kinds of procedural macros in the same crate.
-->

La fonction qui définit une macro procédurale prend un `TokenStream` en entrée et produit un `TokenStream` en sortie. Le type `TokenStream` est défini par le crate `proc_macro` inclus avec Rust et représente une séquence de tokens. C'est le coeur de la macro : le code source sur lequel la macro opère constitue le `TokenStream` d'entrée, et le code que la macro produit est le `TokenStream` de sortie. La fonction a aussi un attribut attaché qui spécifie quel type de macro procédurale nous créons. Nous pouvons avoir plusieurs types de macros procédurales dans le même crate.

<!--
Let's look at the different kinds of procedural macros. We'll start with a
custom `derive` macro and then explain the small dissimilarities that make the
other forms different.
-->

Regardons les différents types de macros procédurales. Nous commencerons par une macro `derive` personnalisée puis expliquerons les petites différences qui distinguent les autres formes.

<!--
Old headings. Do not remove or links may break.
-->

<a id="how-to-write-a-custom-derive-macro"></a>

<!--
### Custom `derive` Macros
-->

### Les macros `derive` personnalisées

<!--
Let's create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making our users implement the `HelloMacro` trait for each of their types,
we'll provide a procedural macro so that users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined. In other words, we'll write a crate that enables another
programmer to write code like Listing 20-37 using our crate.
-->

Créons un crate nommé `hello_macro` qui définit un trait nommé `HelloMacro` avec une fonction associée nommée `hello_macro`. Plutôt que de demander à nos utilisateurs d'implémenter le trait `HelloMacro` pour chacun de leurs types, nous fournirons une macro procédurale pour que les utilisateurs puissent annoter leur type avec `#[derive(HelloMacro)]` pour obtenir une implémentation par défaut de la fonction `hello_macro`. L'implémentation par défaut affichera `Hello, Macro! My name is TypeName!` où `TypeName` est le nom du type sur lequel ce trait a été défini. En d'autres termes, nous écrirons un crate qui permettra à un autre programmeur d'écrire du code comme l'encart 20-37 en utilisant notre crate.

<Listing number="20-37" file-name="src/main.rs" caption="Le code qu'un utilisateur de notre crate pourra écrire en utilisant notre macro procédurale">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

<!--
This code will print `Hello, Macro! My name is Pancakes!` when we're done. The
first step is to make a new library crate, like this:
-->

Ce code affichera `Hello, Macro! My name is Pancakes!` lorsque nous aurons terminé. La première étape est de créer un nouveau crate de bibliothèque, comme ceci :

<!--
```console
$ cargo new hello_macro --lib
```
-->

```console
$ cargo new hello_macro --lib
```

<!--
Next, in Listing 20-38, we'll define the `HelloMacro` trait and its associated
function.
-->

Ensuite, dans l'encart 20-38, nous définirons le trait `HelloMacro` et sa fonction associée.

<Listing file-name="src/lib.rs" number="20-38" caption="Un trait simple que nous utiliserons avec la macro `derive`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

</Listing>

<!--
We have a trait and its function. At this point, our crate user could implement
the trait to achieve the desired functionality, as in Listing 20-39.
-->

Nous avons un trait et sa fonction. À ce stade, l'utilisateur de notre crate pourrait implémenter le trait pour obtenir la fonctionnalité souhaitée, comme dans l'encart 20-39.

<Listing number="20-39" file-name="src/main.rs" caption="À quoi cela ressemblerait si les utilisateurs écrivaient une implémentation manuelle du trait `HelloMacro`">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

</Listing>

<!--
However, they would need to write the implementation block for each type they
wanted to use with `hello_macro`; we want to spare them from having to do this
work.
-->

Cependant, ils devraient écrire le bloc d'implémentation pour chaque type qu'ils voudraient utiliser avec `hello_macro` ; nous voulons leur épargner ce travail.

<!--
Additionally, we can't yet provide the `hello_macro` function with default
implementation that will print the name of the type the trait is implemented
on: Rust doesn't have reflection capabilities, so it can't look up the type's
name at runtime. We need a macro to generate code at compile time.
-->

De plus, nous ne pouvons pas encore fournir à la fonction `hello_macro` une implémentation par défaut qui afficherait le nom du type sur lequel le trait est implémenté : Rust n'a pas de capacités de réflexion, il ne peut donc pas rechercher le nom du type à l'exécution. Nous avons besoin d'une macro pour générer du code au moment de la compilation.

<!--
The next step is to define the procedural macro. At the time of this writing,
procedural macros need to be in their own crate. Eventually, this restriction
might be lifted. The convention for structuring crates and macro crates is as
follows: For a crate named `foo`, a custom `derive` procedural macro crate is
called `foo_derive`. Let's start a new crate called `hello_macro_derive` inside
our `hello_macro` project:
-->

L'étape suivante est de définir la macro procédurale. Au moment de l'écriture, les macros procédurales doivent être dans leur propre crate. À terme, cette restriction pourrait être levée. La convention pour structurer les crates et les crates de macros est la suivante : pour un crate nommé `foo`, un crate de macro procédurale `derive` personnalisée s'appelle `foo_derive`. Commençons un nouveau crate appelé `hello_macro_derive` dans notre projet `hello_macro` :

<!--
```console
$ cargo new hello_macro_derive --lib
```
-->

```console
$ cargo new hello_macro_derive --lib
```

<!--
Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we'll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, and programmers using these crates will need to add
both as dependencies and bring them both into scope. We could instead have the
`hello_macro` crate use `hello_macro_derive` as a dependency and re-export the
procedural macro code. However, the way we've structured the project makes it
possible for programmers to use `hello_macro` even if they don't want the
`derive` functionality.
-->

Nos deux crates sont étroitement liés, nous créons donc le crate de macro procédurale dans le répertoire de notre crate `hello_macro`. Si nous changeons la définition du trait dans `hello_macro`, nous devrons aussi changer l'implémentation de la macro procédurale dans `hello_macro_derive`. Les deux crates devront être publiés séparément, et les programmeurs utilisant ces crates devront ajouter les deux comme dépendances et les importer tous les deux dans la portée. Nous pourrions plutôt faire en sorte que le crate `hello_macro` utilise `hello_macro_derive` comme dépendance et ré-exporte le code de la macro procédurale. Cependant, la manière dont nous avons structuré le projet permet aux programmeurs d'utiliser `hello_macro` même s'ils ne veulent pas la fonctionnalité `derive`.

<!--
We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We'll also need functionality from the `syn` and `quote` crates, as you'll see
in a moment, so we need to add them as dependencies. Add the following to the
_Cargo.toml_ file for `hello_macro_derive`:
-->

Nous devons déclarer le crate `hello_macro_derive` comme un crate de macro procédurale. Nous aurons aussi besoin des fonctionnalités des crates `syn` et `quote`, comme vous le verrez dans un instant, nous devons donc les ajouter comme dépendances. Ajoutez ce qui suit au fichier _Cargo.toml_ de `hello_macro_derive` :

<Listing file-name="hello_macro_derive/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

</Listing>

<!--
To start defining the procedural macro, place the code in Listing 20-40 into
your _src/lib.rs_ file for the `hello_macro_derive` crate. Note that this code
won't compile until we add a definition for the `impl_hello_macro` function.
-->

Pour commencer à définir la macro procédurale, placez le code de l'encart 20-40 dans votre fichier _src/lib.rs_ du crate `hello_macro_derive`. Notez que ce code ne compilera pas tant que nous n'ajoutons pas une définition pour la fonction `impl_hello_macro`.

<Listing number="20-40" file-name="hello_macro_derive/src/lib.rs" caption="Code que la plupart des crates de macros procédurales nécessiteront pour traiter du code Rust">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

</Listing>

<!--
Notice that we've split the code into the `hello_macro_derive` function, which
is responsible for parsing the `TokenStream`, and the `impl_hello_macro`
function, which is responsible for transforming the syntax tree: This makes
writing a procedural macro more convenient. The code in the outer function
(`hello_macro_derive` in this case) will be the same for almost every
procedural macro crate you see or create. The code you specify in the body of
the inner function (`impl_hello_macro` in this case) will be different
depending on your procedural macro's purpose.
-->

Remarquez que nous avons séparé le code en la fonction `hello_macro_derive`, qui est responsable de l'analyse du `TokenStream`, et la fonction `impl_hello_macro`, qui est responsable de la transformation de l'arbre syntaxique : cela rend l'écriture d'une macro procédurale plus pratique. Le code dans la fonction externe (`hello_macro_derive` dans ce cas) sera le même pour presque chaque crate de macro procédurale que vous verrez ou créerez. Le code que vous spécifiez dans le corps de la fonction interne (`impl_hello_macro` dans ce cas) sera différent selon l'objectif de votre macro procédurale.

<!--
We've introduced three new crates: `proc_macro`, [`syn`][syn] ignore
-->,
and [`quote`][quote]<!--
ignore
-->. The `proc_macro` crate comes with Rust,
so we didn't need to add that to the dependencies in _Cargo.toml_. The
`proc_macro` crate is the compiler's API that allows us to read and manipulate
Rust code from our code.
-->

Nous avons introduit trois nouveaux crates : `proc_macro`, [`syn`][syn]<!--
ignore
--> et [`quote`][quote]<!--
ignore
-->. Le crate `proc_macro` est fourni avec Rust, nous n'avons donc pas eu besoin de l'ajouter aux dépendances dans _Cargo.toml_. Le crate `proc_macro` est l'API du compilateur qui nous permet de lire et manipuler du code Rust depuis notre code.

<!--
The `syn` crate parses Rust code from a string into a data structure that we
can perform operations on. The `quote` crate turns `syn` data structures back
into Rust code. These crates make it much simpler to parse any sort of Rust
code we might want to handle: Writing a full parser for Rust code is no simple
task.
-->

Le crate `syn` analyse le code Rust depuis une chaîne de caractères vers une structure de données sur laquelle nous pouvons effectuer des opérations. Le crate `quote` retransforme les structures de données `syn` en code Rust. Ces crates rendent beaucoup plus simple l'analyse de tout type de code Rust que nous pourrions vouloir traiter : écrire un analyseur complet pour le code Rust n'est pas une tâche simple.

<!--
The `hello_macro_derive` function will be called when a user of our library
specifies `#[derive(HelloMacro)]` on a type. This is possible because we've
annotated the `hello_macro_derive` function here with `proc_macro_derive` and
specified the name `HelloMacro`, which matches our trait name; this is the
convention most procedural macros follow.
-->

La fonction `hello_macro_derive` sera appelée lorsqu'un utilisateur de notre bibliothèque spécifie `#[derive(HelloMacro)]` sur un type. C'est possible car nous avons annoté la fonction `hello_macro_derive` ici avec `proc_macro_derive` et spécifié le nom `HelloMacro`, qui correspond au nom de notre trait ; c'est la convention que la plupart des macros procédurales suivent.

<!--
The `hello_macro_derive` function first converts the `input` from a
`TokenStream` to a data structure that we can then interpret and perform
operations on. This is where `syn` comes into play. The `parse` function in
`syn` takes a `TokenStream` and returns a `DeriveInput` struct representing the
parsed Rust code. Listing 20-41 shows the relevant parts of the `DeriveInput`
struct we get from parsing the `struct Pancakes;` string.
-->

La fonction `hello_macro_derive` convertit d'abord l'`input` d'un `TokenStream` vers une structure de données que nous pouvons ensuite interpréter et sur laquelle nous pouvons effectuer des opérations. C'est là que `syn` entre en jeu. La fonction `parse` de `syn` prend un `TokenStream` et retourne une structure `DeriveInput` représentant le code Rust analysé. L'encart 20-41 montre les parties pertinentes de la structure `DeriveInput` que nous obtenons de l'analyse de la chaîne `struct Pancakes;`.

<Listing number="20-41" caption="L'instance `DeriveInput` que nous obtenons en analysant le code qui possède l'attribut de la macro dans l'encart 20-37">

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

</Listing>

<!--
The fields of this struct show that the Rust code we've parsed is a unit struct
with the `ident` (_identifier_, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
documentation for `DeriveInput`][syn-docs] for more information.
-->

Les champs de cette structure montrent que le code Rust que nous avons analysé est une structure unitaire avec l'`ident` (_identifiant_, c'est-à-dire le nom) `Pancakes`. Il y a d'autres champs sur cette structure pour décrire toutes sortes de code Rust ; consultez la [documentation de `syn` pour `DeriveInput`][syn-docs] pour plus d'informations.

<!--
Soon we'll define the `impl_hello_macro` function, which is where we'll build
the new Rust code we want to include. But before we do, note that the output
for our `derive` macro is also a `TokenStream`. The returned `TokenStream` is
added to the code that our crate users write, so when they compile their crate,
they'll get the extra functionality that we provide in the modified
`TokenStream`.
-->

Bientôt nous définirons la fonction `impl_hello_macro`, qui est l'endroit où nous construirons le nouveau code Rust que nous voulons inclure. Mais avant cela, notez que la sortie de notre macro `derive` est aussi un `TokenStream`. Le `TokenStream` retourné est ajouté au code que les utilisateurs de notre crate écrivent, donc lorsqu'ils compilent leur crate, ils obtiendront la fonctionnalité supplémentaire que nous fournissons dans le `TokenStream` modifié.

<!--
You might have noticed that we're calling `unwrap` to cause the
`hello_macro_derive` function to panic if the call to the `syn::parse` function
fails here. It's necessary for our procedural macro to panic on errors because
`proc_macro_derive` functions must return `TokenStream` rather than `Result` to
conform to the procedural macro API. We've simplified this example by using
`unwrap`; in production code, you should provide more specific error messages
about what went wrong by using `panic!` or `expect`.
-->

Vous avez peut-être remarqué que nous appelons `unwrap` pour faire paniquer la fonction `hello_macro_derive` si l'appel à la fonction `syn::parse` échoue ici. Il est nécessaire que notre macro procédurale panique en cas d'erreur car les fonctions `proc_macro_derive` doivent retourner un `TokenStream` plutôt qu'un `Result` pour se conformer à l'API des macros procédurales. Nous avons simplifié cet exemple en utilisant `unwrap` ; dans du code de production, vous devriez fournir des messages d'erreur plus spécifiques sur ce qui s'est mal passé en utilisant `panic!` ou `expect`.

<!--
Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `DeriveInput` instance, let's generate the code that implements the
`HelloMacro` trait on the annotated type, as shown in Listing 20-42.
-->

Maintenant que nous avons le code pour transformer le code Rust annoté d'un `TokenStream` en une instance `DeriveInput`, générons le code qui implémente le trait `HelloMacro` sur le type annoté, comme montré dans l'encart 20-42.

<Listing number="20-42" file-name="hello_macro_derive/src/lib.rs" caption="Implémenter le trait `HelloMacro` en utilisant le code Rust analysé">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

</Listing>

<!--
We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. The struct in Listing 20-41 shows that when
we run the `impl_hello_macro` function on the code in Listing 20-37, the
`ident` we get will have the `ident` field with a value of `"Pancakes"`. Thus,
the `name` variable in Listing 20-42 will contain an `Ident` struct instance
that, when printed, will be the string `"Pancakes"`, the name of the struct in
Listing 20-37.
-->

Nous obtenons une instance de la structure `Ident` contenant le nom (identifiant) du type annoté en utilisant `ast.ident`. La structure dans l'encart 20-41 montre que lorsque nous exécutons la fonction `impl_hello_macro` sur le code de l'encart 20-37, l'`ident` que nous obtenons aura le champ `ident` avec une valeur de `"Pancakes"`. Ainsi, la variable `name` dans l'encart 20-42 contiendra une instance de la structure `Ident` qui, lorsqu'elle sera affichée, sera la chaîne `"Pancakes"`, le nom de la structure dans l'encart 20-37.

<!--
The `quote!` macro lets us define the Rust code that we want to return. The
compiler expects something different from the direct result of the `quote!`
macro's execution, so we need to convert it to a `TokenStream`. We do this by
calling the `into` method, which consumes this intermediate representation and
returns a value of the required `TokenStream` type.
-->

La macro `quote!` nous permet de définir le code Rust que nous voulons retourner. Le compilateur attend quelque chose de différent du résultat direct de l'exécution de la macro `quote!`, nous devons donc le convertir en un `TokenStream`. Nous faisons cela en appelant la méthode `into`, qui consomme cette représentation intermédiaire et retourne une valeur du type `TokenStream` requis.

<!--
The `quote!` macro also provides some very cool templating mechanics: We can
enter `#name`, and `quote!` will replace it with the value in the variable
`name`. You can even do some repetition similar to the way regular macros work.
Check out [the `quote` crate's docs][quote-docs] for a thorough introduction.
-->

La macro `quote!` fournit aussi des mécanismes de modèles très pratiques : nous pouvons entrer `#name`, et `quote!` le remplacera par la valeur de la variable `name`. Vous pouvez même faire de la répétition de manière similaire au fonctionnement des macros normales. Consultez [la documentation du crate `quote`][quote-docs] pour une introduction approfondie.

<!--
We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has the one function `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.
-->

Nous voulons que notre macro procédurale génère une implémentation de notre trait `HelloMacro` pour le type que l'utilisateur a annoté, que nous pouvons obtenir en utilisant `#name`. L'implémentation du trait a une seule fonction `hello_macro`, dont le corps contient la fonctionnalité que nous voulons fournir : afficher `Hello, Macro! My name is` suivi du nom du type annoté.

<!--
The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different from `format!` or
`println!`, which are macros that evaluate the expression and then turn the
result into a `String`. There is a possibility that the `#name` input might be
an expression to print literally, so we use `stringify!`. Using `stringify!`
also saves an allocation by converting `#name` to a string literal at compile
time.
-->

La macro `stringify!` utilisée ici est intégrée à Rust. Elle prend une expression Rust, comme `1 + 2`, et au moment de la compilation transforme l'expression en un littéral de chaîne, comme `"1 + 2"`. C'est différent de `format!` ou `println!`, qui sont des macros qui évaluent l'expression puis transforment le résultat en une `String`. Il est possible que l'entrée `#name` soit une expression à afficher littéralement, c'est pourquoi nous utilisons `stringify!`. L'utilisation de `stringify!` économise aussi une allocation en convertissant `#name` en un littéral de chaîne au moment de la compilation.

<!--
At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let's hook up these crates to the code in Listing
20-37 to see the procedural macro in action! Create a new binary project in
your _projects_ directory using `cargo new pancakes`. We need to add
`hello_macro` and `hello_macro_derive` as dependencies in the `pancakes`
crate's _Cargo.toml_. If you're publishing your versions of `hello_macro` and
`hello_macro_derive` to [crates.io](https://crates.io/) ignore
-->, they
would be regular dependencies; if not, you can specify them as `path`
dependencies as follows:
-->

À ce stade, `cargo build` devrait se terminer avec succès dans `hello_macro` et `hello_macro_derive`. Connectons ces crates au code de l'encart 20-37 pour voir la macro procédurale en action ! Créez un nouveau projet binaire dans votre répertoire _projects_ en utilisant `cargo new pancakes`. Nous devons ajouter `hello_macro` et `hello_macro_derive` comme dépendances dans le _Cargo.toml_ du crate `pancakes`. Si vous publiez vos versions de `hello_macro` et `hello_macro_derive` sur [crates.io](https://crates.io/)<!--
ignore
-->, ce seraient des dépendances normales ; sinon, vous pouvez les spécifier comme dépendances `path` comme suit :


```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:6:8}}
```

<!--
Put the code in Listing 20-37 into _src/main.rs_, and run `cargo run`: It
should print `Hello, Macro! My name is Pancakes!`. The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the
trait implementation.
-->

Placez le code de l'encart 20-37 dans _src/main.rs_ et exécutez `cargo run` : il devrait afficher `Hello, Macro! My name is Pancakes!`. L'implémentation du trait `HelloMacro` par la macro procédurale a été incluse sans que le crate `pancakes` n'ait besoin de l'implémenter ; le `#[derive(HelloMacro)]` a ajouté l'implémentation du trait.

<!--
Next, let's explore how the other kinds of procedural macros differ from custom
`derive` macros.
-->

Ensuite, explorons en quoi les autres types de macros procédurales diffèrent des macros `derive` personnalisées.

<!--
### Attribute-Like Macros
-->

### Les macros de type attribut

<!--
Attribute-like macros are similar to custom `derive` macros, but instead of
generating code for the `derive` attribute, they allow you to create new
attributes. They're also more flexible: `derive` only works for structs and
enums; attributes can be applied to other items as well, such as functions.
Here's an example of using an attribute-like macro. Say you have an attribute
named `route` that annotates functions when using a web application framework:
-->

Les macros de type attribut sont similaires aux macros `derive` personnalisées, mais au lieu de générer du code pour l'attribut `derive`, elles vous permettent de créer de nouveaux attributs. Elles sont aussi plus flexibles : `derive` ne fonctionne que pour les structures et les enums ; les attributs peuvent être appliqués à d'autres éléments aussi, comme les fonctions. Voici un exemple d'utilisation d'une macro de type attribut. Imaginons que vous ayez un attribut nommé `route` qui annote des fonctions lors de l'utilisation d'un framework d'application web :

<!--
```rust,ignore
#[route(GET, "/")]
fn index() {
```
-->

```rust,ignore
#[route(GET, "/")]
fn index() {
```

<!--
This `#[route]` attribute would be defined by the framework as a procedural
macro. The signature of the macro definition function would look like this:
-->

Cet attribut `#[route]` serait défini par le framework comme une macro procédurale. La signature de la fonction de définition de la macro ressemblerait à ceci :

<!--
```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```
-->

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

<!--
Here, we have two parameters of type `TokenStream`. The first is for the
contents of the attribute: the `GET, "/"` part. The second is the body of the
item the attribute is attached to: in this case, `fn index() {}` and the rest
of the function's body.
-->

Ici, nous avons deux paramètres de type `TokenStream`. Le premier est pour le contenu de l'attribut : la partie `GET, "/"`. Le second est le corps de l'élément auquel l'attribut est attaché : dans ce cas, `fn index() {}` et le reste du corps de la fonction.

<!--
Other than that, attribute-like macros work the same way as custom `derive`
macros: You create a crate with the `proc-macro` crate type and implement a
function that generates the code you want!
-->

À part cela, les macros de type attribut fonctionnent de la même manière que les macros `derive` personnalisées : vous créez un crate avec le type de crate `proc-macro` et implémentez une fonction qui génère le code que vous voulez !

<!--
### Function-Like Macros
-->

### Les macros de type fonction

<!--
Function-like macros define macros that look like function calls. Similarly to
`macro_rules!` macros, they're more flexible than functions; for example, they
can take an unknown number of arguments. However, `macro_rules!` macros can
only be defined using the match-like syntax we discussed in the ["Declarative
Macros for General Metaprogramming"][decl] ignore
--> section earlier.
Function-like macros take a `TokenStream` parameter, and their definition
manipulates that `TokenStream` using Rust code as the other two types of
procedural macros do. An example of a function-like macro is an `sql!` macro
that might be called like so:
-->

Les macros de type fonction définissent des macros qui ressemblent à des appels de fonction. Comme les macros `macro_rules!`, elles sont plus flexibles que les fonctions ; par exemple, elles peuvent prendre un nombre inconnu d'arguments. Cependant, les macros `macro_rules!` ne peuvent être définies qu'en utilisant la syntaxe de type match que nous avons discutée dans la section ["Les macros déclaratives pour la métaprogrammation générale"][decl]<!--
ignore
--> plus tôt. Les macros de type fonction prennent un paramètre `TokenStream`, et leur définition manipule ce `TokenStream` en utilisant du code Rust comme le font les deux autres types de macros procédurales. Un exemple de macro de type fonction est une macro `sql!` qui pourrait être appelée comme suit :

<!--
```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
-->

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

<!--
This macro would parse the SQL statement inside it and check that it's
syntactically correct, which is much more complex processing than a
`macro_rules!` macro can do. The `sql!` macro would be defined like this:
-->

Cette macro analyserait l'instruction SQL à l'intérieur et vérifierait qu'elle est syntaxiquement correcte, ce qui est un traitement beaucoup plus complexe que ce qu'une macro `macro_rules!` peut faire. La macro `sql!` serait définie comme ceci :

<!--
```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
-->

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

<!--
This definition is similar to the custom `derive` macro's signature: We receive
the tokens that are inside the parentheses and return the code we wanted to
generate.
-->

Cette définition est similaire à la signature de la macro `derive` personnalisée : nous recevons les tokens qui sont à l'intérieur des parenthèses et retournons le code que nous voulions générer.

<!--
## Summary
-->

## Résumé

<!--
Whew! Now you have some Rust features in your toolbox that you likely won't use
often, but you'll know they're available in very particular circumstances.
We've introduced several complex topics so that when you encounter them in
error message suggestions or in other people's code, you'll be able to
recognize these concepts and syntax. Use this chapter as a reference to guide
you to solutions.
-->

Ouf ! Vous avez maintenant quelques fonctionnalités Rust dans votre boîte à outils que vous n'utiliserez probablement pas souvent, mais vous saurez qu'elles sont disponibles dans des circonstances très particulières. Nous avons introduit plusieurs sujets complexes afin que lorsque vous les rencontrez dans les suggestions de messages d'erreur ou dans le code d'autres personnes, vous puissiez reconnaître ces concepts et cette syntaxe. Utilisez ce chapitre comme référence pour vous guider vers des solutions.

<!--
Next, we'll put everything we've discussed throughout the book into practice
and do one more project!
-->

Ensuite, nous mettrons en pratique tout ce que nous avons discuté tout au long du livre et réaliserons un dernier projet !

[ref]: ../reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[syn]: https://crates.io/crates/syn
[quote]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
