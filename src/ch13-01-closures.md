<!--
Old headings. Do not remove or links may break.
-->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

<!--
## Closures
-->

## Les fermetures (closures)

<!--
Rust's closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure elsewhere to evaluate it in a different context. Unlike
functions, closures can capture values from the scope in which they're defined.
We'll demonstrate how these closure features allow for code reuse and behavior
customization.
-->

Les fermetures en Rust sont des fonctions anonymes que vous pouvez enregistrer dans une variable ou passer en arguments a d'autres fonctions. Vous pouvez creer la fermeture a un endroit puis l'appeler ailleurs pour l'evaluer dans un contexte different. Contrairement aux fonctions, les fermetures peuvent capturer des valeurs de la portee dans laquelle elles sont definies. Nous allons montrer comment ces caracteristiques des fermetures permettent la reutilisation du code et la personnalisation du comportement.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

<!--
### Capturing the Environment
-->

### Capturer l'environnement

<!--
We'll first examine how we can use closures to capture values from the
environment they're defined in for later use. Here's the scenario: Every so
often, our T-shirt company gives away an exclusive, limited-edition shirt to
someone on our mailing list as a promotion. People on the mailing list can
optionally add their favorite color to their profile. If the person chosen for
a free shirt has their favorite color set, they get that color shirt. If the
person hasn't specified a favorite color, they get whatever color the company
currently has the most of.
-->

Nous allons d'abord examiner comment nous pouvons utiliser les fermetures pour capturer des valeurs de l'environnement dans lequel elles sont definies pour une utilisation ulterieure. Voici le scenario : de temps en temps, notre entreprise de T-shirts offre un T-shirt exclusif en edition limitee a quelqu'un de notre liste de diffusion comme promotion. Les personnes inscrites sur la liste de diffusion peuvent optionnellement ajouter leur couleur preferee a leur profil. Si la personne choisie pour un T-shirt gratuit a defini sa couleur preferee, elle recoit un T-shirt de cette couleur. Si la personne n'a pas specifie de couleur preferee, elle recoit la couleur dont l'entreprise a le plus en stock.

<!--
There are many ways to implement this. For this example, we're going to use an
enum called `ShirtColor` that has the variants `Red` and `Blue` (limiting the
number of colors available for simplicity). We represent the company's
inventory with an `Inventory` struct that has a field named `shirts` that
contains a `Vec<ShirtColor>` representing the shirt colors currently in stock.
The method `giveaway` defined on `Inventory` gets the optional shirt color
preference of the free-shirt winner, and it returns the shirt color the
person will get. This setup is shown in Listing 13-1.
-->

Il existe de nombreuses facons d'implementer cela. Pour cet exemple, nous allons utiliser un enum appele `ShirtColor` qui a les variantes `Red` et `Blue` (limitant le nombre de couleurs disponibles par souci de simplicite). Nous representons l'inventaire de l'entreprise avec une struct `Inventory` qui a un champ nomme `shirts` contenant un `Vec<ShirtColor>` representant les couleurs de T-shirts actuellement en stock. La methode `giveaway` definie sur `Inventory` obtient la preference optionnelle de couleur de T-shirt du gagnant, et retourne la couleur de T-shirt que la personne recevra. Cette configuration est presentee dans l'encart 13-1.

<Listing number="13-1" file-name="src/main.rs" caption="Situation de distribution de T-shirts de l'entreprise">


```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

<!--
The `store` defined in `main` has two blue shirts and one red shirt remaining
to distribute for this limited-edition promotion. We call the `giveaway` method
for a user with a preference for a red shirt and a user without any preference.
-->

Le `store` defini dans `main` a deux T-shirts bleus et un T-shirt rouge restants a distribuer pour cette promotion en edition limitee. Nous appelons la methode `giveaway` pour un utilisateur avec une preference pour un T-shirt rouge et un utilisateur sans aucune preference.

<!--
Again, this code could be implemented in many ways, and here, to focus on
closures, we've stuck to concepts you've already learned, except for the body of
the `giveaway` method that uses a closure. In the `giveaway` method, we get the
user preference as a parameter of type `Option<ShirtColor>` and call the
`unwrap_or_else` method on `user_preference`. The [`unwrap_or_else` method on
`Option<T>`][unwrap-or-else] ignore
--> is defined by the standard library.
It takes one argument: a closure without any arguments that returns a value `T`
(the same type stored in the `Some` variant of the `Option<T>`, in this case
`ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else`
returns the value from within the `Some`. If the `Option<T>` is the `None`
variant, `unwrap_or_else` calls the closure and returns the value returned by
the closure.
-->

Encore une fois, ce code pourrait etre implemente de nombreuses facons, et ici, pour nous concentrer sur les fermetures, nous nous sommes limites aux concepts que vous avez deja appris, a l'exception du corps de la methode `giveaway` qui utilise une fermeture. Dans la methode `giveaway`, nous obtenons la preference de l'utilisateur sous forme de parametre de type `Option<ShirtColor>` et appelons la methode `unwrap_or_else` sur `user_preference`. La methode [`unwrap_or_else` sur `Option<T>`][unwrap-or-else]<!--
ignore
--> est definie par la bibliotheque standard. Elle prend un argument : une fermeture sans aucun argument qui retourne une valeur `T` (le meme type stocke dans la variante `Some` de `Option<T>`, dans ce cas `ShirtColor`). Si `Option<T>` est la variante `Some`, `unwrap_or_else` retourne la valeur contenue dans le `Some`. Si `Option<T>` est la variante `None`, `unwrap_or_else` appelle la fermeture et retourne la valeur renvoyee par la fermeture.

<!--
We specify the closure expression `|| self.most_stocked()` as the argument to
`unwrap_or_else`. This is a closure that takes no parameters itself (if the
closure had parameters, they would appear between the two vertical pipes). The
body of the closure calls `self.most_stocked()`. We're defining the closure
here, and the implementation of `unwrap_or_else` will evaluate the closure
later if the result is needed.
-->

Nous specifions l'expression de fermeture `|| self.most_stocked()` comme argument de `unwrap_or_else`. C'est une fermeture qui ne prend aucun parametre (si la fermeture avait des parametres, ils apparaitraient entre les deux barres verticales). Le corps de la fermeture appelle `self.most_stocked()`. Nous definissons la fermeture ici, et l'implementation de `unwrap_or_else` evaluera la fermeture plus tard si le resultat est necessaire.

<!--
Running this code prints the following:
-->

L'execution de ce code affiche le resultat suivant :


```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

<!--
One interesting aspect here is that we've passed a closure that calls
`self.most_stocked()` on the current `Inventory` instance. The standard library
didn't need to know anything about the `Inventory` or `ShirtColor` types we
defined, or the logic we want to use in this scenario. The closure captures an
immutable reference to the `self` `Inventory` instance and passes it with the
code we specify to the `unwrap_or_else` method. Functions, on the other hand,
are not able to capture their environment in this way.
-->

Un aspect interessant ici est que nous avons passe une fermeture qui appelle `self.most_stocked()` sur l'instance actuelle d'`Inventory`. La bibliotheque standard n'avait pas besoin de connaitre quoi que ce soit sur les types `Inventory` ou `ShirtColor` que nous avons definis, ni sur la logique que nous voulons utiliser dans ce scenario. La fermeture capture une reference immuable vers l'instance `self` d'`Inventory` et la transmet avec le code que nous specifions a la methode `unwrap_or_else`. Les fonctions, en revanche, ne sont pas capables de capturer leur environnement de cette maniere.

<!--
Old headings. Do not remove or links may break.
-->

<a id="closure-type-inference-and-annotation"></a>

<!--
### Inferring and Annotating Closure Types
-->

### Inference et annotation des types de fermetures

<!--
There are more differences between functions and closures. Closures don't
usually require you to annotate the types of the parameters or the return value
like `fn` functions do. Type annotations are required on functions because the
types are part of an explicit interface exposed to your users. Defining this
interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. Closures, on the other hand, aren't used
in an exposed interface like this: They're stored in variables, and they're
used without naming them and exposing them to users of our library.
-->

Il y a d'autres differences entre les fonctions et les fermetures. Les fermetures ne necessitent generalement pas d'annoter les types des parametres ou de la valeur de retour comme le font les fonctions `fn`. Les annotations de types sont requises sur les fonctions car les types font partie d'une interface explicite exposee a vos utilisateurs. Definir cette interface de maniere rigide est important pour s'assurer que tout le monde est d'accord sur les types de valeurs qu'une fonction utilise et retourne. Les fermetures, en revanche, ne sont pas utilisees dans une interface exposee de cette facon : elles sont stockees dans des variables et utilisees sans etre nommees ni exposees aux utilisateurs de notre bibliotheque.

<!--
Closures are typically short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler can
infer the types of the parameters and the return type, similar to how it's able
to infer the types of most variables (there are rare cases where the compiler
needs closure type annotations too).
-->

Les fermetures sont generalement courtes et pertinentes uniquement dans un contexte restreint plutot que dans n'importe quel scenario arbitraire. Dans ces contextes limites, le compilateur peut inferer les types des parametres et le type de retour, de la meme maniere qu'il est capable d'inferer les types de la plupart des variables (il y a de rares cas ou le compilateur a egalement besoin d'annotations de types pour les fermetures).

<!--
As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for a closure would look like the definition
shown in Listing 13-2. In this example, we're defining a closure and storing it
in a variable rather than defining the closure in the spot we pass it as an
argument, as we did in Listing 13-1.
-->

Comme avec les variables, nous pouvons ajouter des annotations de types si nous voulons augmenter l'explicite et la clarte au prix d'etre plus verbeux que strictement necessaire. Annoter les types pour une fermeture ressemblerait a la definition montree dans l'encart 13-2. Dans cet exemple, nous definissons une fermeture et la stockons dans une variable plutot que de definir la fermeture a l'endroit ou nous la passons en argument, comme nous l'avons fait dans l'encart 13-1.

<Listing number="13-2" file-name="src/main.rs" caption="Ajout d'annotations de types optionnelles pour les parametres et la valeur de retour de la fermeture">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

<!--
With type annotations added, the syntax of closures looks more similar to the
syntax of functions. Here, we define a function that adds 1 to its parameter and
a closure that has the same behavior, for comparison. We've added some spaces
to line up the relevant parts. This illustrates how closure syntax is similar
to function syntax except for the use of pipes and the amount of syntax that is
optional:
-->

Avec les annotations de types ajoutees, la syntaxe des fermetures ressemble davantage a la syntaxe des fonctions. Ici, nous definissons une fonction qui ajoute 1 a son parametre et une fermeture qui a le meme comportement, a titre de comparaison. Nous avons ajoute des espaces pour aligner les parties pertinentes. Cela illustre comment la syntaxe des fermetures est similaire a la syntaxe des fonctions, a l'exception de l'utilisation des barres verticales et de la quantite de syntaxe qui est optionnelle :

<!--
```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
-->

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

<!--
The first line shows a function definition and the second line shows a fully
annotated closure definition. In the third line, we remove the type annotations
from the closure definition. In the fourth line, we remove the brackets, which
are optional because the closure body has only one expression. These are all
valid definitions that will produce the same behavior when they're called. The
`add_one_v3` and `add_one_v4` lines require the closures to be evaluated to be
able to compile because the types will be inferred from their usage. This is
similar to `let v = Vec::new();` needing either type annotations or values of
some type to be inserted into the `Vec` for Rust to be able to infer the type.
-->

La premiere ligne montre une definition de fonction et la deuxieme ligne montre une definition de fermeture entierement annotee. A la troisieme ligne, nous retirons les annotations de types de la definition de la fermeture. A la quatrieme ligne, nous retirons les accolades, qui sont optionnelles car le corps de la fermeture n'a qu'une seule expression. Ce sont toutes des definitions valides qui produiront le meme comportement lorsqu'elles seront appelees. Les lignes `add_one_v3` et `add_one_v4` necessitent que les fermetures soient evaluees pour pouvoir compiler car les types seront inferes a partir de leur utilisation. C'est similaire a `let v = Vec::new();` qui necessite soit des annotations de types, soit des valeurs d'un certain type a inserer dans le `Vec` pour que Rust puisse inferer le type.

<!--
For closure definitions, the compiler will infer one concrete type for each of
their parameters and for their return value. For instance, Listing 13-3 shows
the definition of a short closure that just returns the value it receives as a
parameter. This closure isn't very useful except for the purposes of this
example. Note that we haven't added any type annotations to the definition.
Because there are no type annotations, we can call the closure with any type,
which we've done here with `String` the first time. If we then try to call
`example_closure` with an integer, we'll get an error.
-->

Pour les definitions de fermetures, le compilateur inferera un type concret pour chacun de leurs parametres et pour leur valeur de retour. Par exemple, l'encart 13-3 montre la definition d'une courte fermeture qui retourne simplement la valeur qu'elle recoit en parametre. Cette fermeture n'est pas tres utile sauf dans le cadre de cet exemple. Notez que nous n'avons ajoute aucune annotation de type a la definition. Parce qu'il n'y a pas d'annotations de types, nous pouvons appeler la fermeture avec n'importe quel type, ce que nous avons fait ici avec `String` la premiere fois. Si nous essayons ensuite d'appeler `example_closure` avec un entier, nous obtiendrons une erreur.

<Listing number="13-3" file-name="src/main.rs" caption="Tentative d'appel d'une fermeture dont les types sont inferes avec deux types differents">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

<!--
The compiler gives us this error:
-->

Le compilateur nous donne cette erreur :


```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

<!--
The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error when we next try to use a different type with the same closure.
-->

La premiere fois que nous appelons `example_closure` avec la valeur `String`, le compilateur infere que le type de `x` et le type de retour de la fermeture sont `String`. Ces types sont alors verrouilles dans la fermeture `example_closure`, et nous obtenons une erreur de type lorsque nous essayons ensuite d'utiliser un type different avec la meme fermeture.

<!--
### Capturing References or Moving Ownership
-->

### Capturer des references ou transferer la possession

<!--
Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership. The closure will decide
which of these to use based on what the body of the function does with the
captured values.
-->

Les fermetures peuvent capturer des valeurs de leur environnement de trois facons, qui correspondent directement aux trois facons dont une fonction peut prendre un parametre : emprunter de maniere immuable, emprunter de maniere mutable et prendre la possession. La fermeture decidera laquelle de ces methodes utiliser en fonction de ce que le corps de la fonction fait avec les valeurs capturees.

<!--
In Listing 13-4, we define a closure that captures an immutable reference to
the vector named `list` because it only needs an immutable reference to print
the value.
-->

Dans l'encart 13-4, nous definissons une fermeture qui capture une reference immuable vers le vecteur nomme `list` car elle n'a besoin que d'une reference immuable pour afficher la valeur.

<Listing number="13-4" file-name="src/main.rs" caption="Definition et appel d'une fermeture qui capture une reference immuable">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

<!--
This example also illustrates that a variable can bind to a closure definition,
and we can later call the closure by using the variable name and parentheses as
if the variable name were a function name.
-->

Cet exemple illustre egalement qu'une variable peut etre liee a une definition de fermeture, et nous pouvons ensuite appeler la fermeture en utilisant le nom de la variable et des parentheses comme si le nom de la variable etait un nom de fonction.

<!--
Because we can have multiple immutable references to `list` at the same time,
`list` is still accessible from the code before the closure definition, after
the closure definition but before the closure is called, and after the closure
is called. This code compiles, runs, and prints:
-->

Parce que nous pouvons avoir plusieurs references immuables vers `list` en meme temps, `list` est toujours accessible depuis le code avant la definition de la fermeture, apres la definition de la fermeture mais avant que la fermeture ne soit appelee, et apres que la fermeture ait ete appelee. Ce code compile, s'execute et affiche :


```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

<!--
Next, in Listing 13-5, we change the closure body so that it adds an element to
the `list` vector. The closure now captures a mutable reference.
-->

Ensuite, dans l'encart 13-5, nous modifions le corps de la fermeture pour qu'il ajoute un element au vecteur `list`. La fermeture capture maintenant une reference mutable.

<Listing number="13-5" file-name="src/main.rs" caption="Definition et appel d'une fermeture qui capture une reference mutable">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

<!--
This code compiles, runs, and prints:
-->

Ce code compile, s'execute et affiche :


```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

<!--
Note that there's no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: When `borrows_mutably` is defined, it captures a
mutable reference to `list`. We don't use the closure again after the closure
is called, so the mutable borrow ends. Between the closure definition and the
closure call, an immutable borrow to print isn't allowed, because no other
borrows are allowed when there's a mutable borrow. Try adding a `println!`
there to see what error message you get!
-->

Notez qu'il n'y a plus de `println!` entre la definition et l'appel de la fermeture `borrows_mutably` : quand `borrows_mutably` est definie, elle capture une reference mutable vers `list`. Nous n'utilisons plus la fermeture apres son appel, donc l'emprunt mutable se termine. Entre la definition de la fermeture et l'appel de la fermeture, un emprunt immuable pour afficher n'est pas autorise, car aucun autre emprunt n'est permis lorsqu'il y a un emprunt mutable. Essayez d'ajouter un `println!` a cet endroit pour voir quel message d'erreur vous obtenez !

<!--
If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn't strictly need
ownership, you can use the `move` keyword before the parameter list.
-->

Si vous voulez forcer la fermeture a prendre la possession des valeurs qu'elle utilise dans l'environnement meme si le corps de la fermeture n'a pas strictement besoin de la possession, vous pouvez utiliser le mot-cle `move` avant la liste de parametres.

<!--
This technique is mostly useful when passing a closure to a new thread to move
the data so that it's owned by the new thread. We'll discuss threads and why
you would want to use them in detail in Chapter 16 when we talk about
concurrency, but for now, let's briefly explore spawning a new thread using a
closure that needs the `move` keyword. Listing 13-6 shows Listing 13-4 modified
to print the vector in a new thread rather than in the main thread.
-->

Cette technique est surtout utile lorsque vous passez une fermeture a un nouveau thread pour deplacer les donnees afin qu'elles soient possedees par le nouveau thread. Nous discuterons des threads et de pourquoi vous voudriez les utiliser en detail dans le chapitre 16 lorsque nous parlerons de la concurrence, mais pour l'instant, explorons brievement la creation d'un nouveau thread en utilisant une fermeture qui necessite le mot-cle `move`. L'encart 13-6 montre l'encart 13-4 modifie pour afficher le vecteur dans un nouveau thread plutot que dans le thread principal.

<Listing number="13-6" file-name="src/main.rs" caption="Utilisation de `move` pour forcer la fermeture du thread a prendre la possession de `list`">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

<!--
We spawn a new thread, giving the thread a closure to run as an argument. The
closure body prints out the list. In Listing 13-4, the closure only captured
`list` using an immutable reference because that's the least amount of access
to `list` needed to print it. In this example, even though the closure body
still only needs an immutable reference, we need to specify that `list` should
be moved into the closure by putting the `move` keyword at the beginning of the
closure definition. If the main thread performed more operations before calling
`join` on the new thread, the new thread might finish before the rest of the
main thread finishes, or the main thread might finish first. If the main thread
maintained ownership of `list` but ended before the new thread and drops
`list`, the immutable reference in the thread would be invalid. Therefore, the
compiler requires that `list` be moved into the closure given to the new thread
so that the reference will be valid. Try removing the `move` keyword or using
`list` in the main thread after the closure is defined to see what compiler
errors you get!
-->

Nous creons un nouveau thread en lui donnant une fermeture a executer comme argument. Le corps de la fermeture affiche la liste. Dans l'encart 13-4, la fermeture ne capturait `list` qu'avec une reference immuable car c'est le minimum d'acces a `list` necessaire pour l'afficher. Dans cet exemple, meme si le corps de la fermeture n'a toujours besoin que d'une reference immuable, nous devons specifier que `list` doit etre deplacee dans la fermeture en mettant le mot-cle `move` au debut de la definition de la fermeture. Si le thread principal effectuait d'autres operations avant d'appeler `join` sur le nouveau thread, le nouveau thread pourrait se terminer avant que le reste du thread principal ne se termine, ou le thread principal pourrait se terminer en premier. Si le thread principal conservait la possession de `list` mais se terminait avant le nouveau thread et liberait `list`, la reference immuable dans le thread serait invalide. Par consequent, le compilateur exige que `list` soit deplacee dans la fermeture donnee au nouveau thread pour que la reference soit valide. Essayez de retirer le mot-cle `move` ou d'utiliser `list` dans le thread principal apres que la fermeture ait ete definie pour voir quelles erreurs du compilateur vous obtenez !

<!--
Old headings. Do not remove or links may break.
-->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>

<!--
### Moving Captured Values Out of Closures
-->

### Deplacer les valeurs capturees hors des fermetures

<!--
Once a closure has captured a reference or captured ownership of a value from
the environment where the closure is defined (thus affecting what, if anything,
is moved _into_ the closure), the code in the body of the closure defines what
happens to the references or values when the closure is evaluated later (thus
affecting what, if anything, is moved _out of_ the closure).
-->

Une fois qu'une fermeture a capture une reference ou pris la possession d'une valeur de l'environnement ou la fermeture est definie (affectant ainsi ce qui, le cas echeant, est deplace _dans_ la fermeture), le code dans le corps de la fermeture definit ce qui arrive aux references ou aux valeurs lorsque la fermeture est evaluee plus tard (affectant ainsi ce qui, le cas echeant, est deplace _hors de_ la fermeture).

<!--
A closure body can do any of the following: Move a captured value out of the
closure, mutate the captured value, neither move nor mutate the value, or
capture nothing from the environment to begin with.
-->

Le corps d'une fermeture peut faire l'une des choses suivantes : deplacer une valeur capturee hors de la fermeture, muter la valeur capturee, ne ni deplacer ni muter la valeur, ou ne rien capturer de l'environnement au depart.

<!--
The way a closure captures and handles values from the environment affects
which traits the closure implements, and traits are how functions and structs
can specify what kinds of closures they can use. Closures will automatically
implement one, two, or all three of these `Fn` traits, in an additive fashion,
depending on how the closure's body handles the values:
-->

La facon dont une fermeture capture et gere les valeurs de l'environnement affecte les traits que la fermeture implemente, et les traits sont la maniere dont les fonctions et les structs peuvent specifier quels types de fermetures elles peuvent utiliser. Les fermetures implementeront automatiquement un, deux ou les trois traits `Fn`, de maniere additive, selon la facon dont le corps de la fermeture gere les valeurs :

<!--
* `FnOnce` applies to closures that can be called once. All closures implement
  at least this trait because all closures can be called. A closure that moves
  captured values out of its body will only implement `FnOnce` and none of the
  other `Fn` traits because it can only be called once.
* `FnMut` applies to closures that don't move captured values out of their body
  but might mutate the captured values. These closures can be called more than
  once.
* `Fn` applies to closures that don't move captured values out of their body
  and don't mutate captured values, as well as closures that capture nothing
  from their environment. These closures can be called more than once without
  mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
-->

* `FnOnce` s'applique aux fermetures qui peuvent etre appelees une seule fois. Toutes les fermetures implementent au moins ce trait car toutes les fermetures peuvent etre appelees. Une fermeture qui deplace les valeurs capturees hors de son corps n'implementera que `FnOnce` et aucun des autres traits `Fn` car elle ne peut etre appelee qu'une seule fois.
* `FnMut` s'applique aux fermetures qui ne deplacent pas les valeurs capturees hors de leur corps mais qui pourraient muter les valeurs capturees. Ces fermetures peuvent etre appelees plusieurs fois.
* `Fn` s'applique aux fermetures qui ne deplacent pas les valeurs capturees hors de leur corps et qui ne mutent pas les valeurs capturees, ainsi qu'aux fermetures qui ne capturent rien de leur environnement. Ces fermetures peuvent etre appelees plusieurs fois sans muter leur environnement, ce qui est important dans des cas comme l'appel concurrent d'une fermeture plusieurs fois.

<!--
Let's look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-1:
-->

Examinons la definition de la methode `unwrap_or_else` sur `Option<T>` que nous avons utilisee dans l'encart 13-1 :

<!--
```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
-->

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

<!--
Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: Code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.
-->

Rappelez-vous que `T` est le type generique representant le type de la valeur dans la variante `Some` d'une `Option`. Ce type `T` est egalement le type de retour de la fonction `unwrap_or_else` : du code qui appelle `unwrap_or_else` sur une `Option<String>`, par exemple, obtiendra une `String`.

<!--
Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.
-->

Ensuite, remarquez que la fonction `unwrap_or_else` a le parametre de type generique supplementaire `F`. Le type `F` est le type du parametre nomme `f`, qui est la fermeture que nous fournissons lors de l'appel a `unwrap_or_else`.

<!--
The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called once, take no arguments, and return a `T`.
Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` will not call `f` more than once. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won't be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts all three kinds of
closures and is as flexible as it can be.
-->

La contrainte de trait specifiee sur le type generique `F` est `FnOnce() -> T`, ce qui signifie que `F` doit pouvoir etre appele une fois, ne prendre aucun argument et retourner un `T`. Utiliser `FnOnce` dans la contrainte de trait exprime la contrainte que `unwrap_or_else` n'appellera pas `f` plus d'une fois. Dans le corps de `unwrap_or_else`, nous pouvons voir que si `Option` est `Some`, `f` ne sera pas appelee. Si `Option` est `None`, `f` sera appelee une fois. Parce que toutes les fermetures implementent `FnOnce`, `unwrap_or_else` accepte les trois types de fermetures et est aussi flexible que possible.

<!--
> Note: If what we want to do doesn't require capturing a value from the
> environment, we can use the name of a function rather than a closure where we
> need something that implements one of the `Fn` traits. For example, on an
> `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a
> new, empty vector if the value is `None`. The compiler automatically
> implements whichever of the `Fn` traits is applicable for a function
> definition.
-->

> Remarque : si ce que nous voulons faire ne necessite pas de capturer une valeur de l'environnement, nous pouvons utiliser le nom d'une fonction plutot qu'une fermeture la ou nous avons besoin de quelque chose qui implemente l'un des traits `Fn`. Par exemple, sur une valeur `Option<Vec<T>>`, nous pourrions appeler `unwrap_or_else(Vec::new)` pour obtenir un nouveau vecteur vide si la valeur est `None`. Le compilateur implemente automatiquement celui des traits `Fn` qui est applicable pour une definition de fonction.

<!--
Now let's look at the standard library method `sort_by_key`, defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound. The closure gets one argument
in the form of a reference to the current item in the slice being considered,
and it returns a value of type `K` that can be ordered. This function is useful
when you want to sort a slice by a particular attribute of each item. In
Listing 13-7, we have a list of `Rectangle` instances, and we use `sort_by_key`
to order them by their `width` attribute from low to high.
-->

Examinons maintenant la methode de la bibliotheque standard `sort_by_key`, definie sur les slices, pour voir en quoi elle differe de `unwrap_or_else` et pourquoi `sort_by_key` utilise `FnMut` au lieu de `FnOnce` pour la contrainte de trait. La fermeture recoit un argument sous la forme d'une reference vers l'element actuel de la slice consideree, et retourne une valeur de type `K` qui peut etre ordonnee. Cette fonction est utile lorsque vous voulez trier une slice selon un attribut particulier de chaque element. Dans l'encart 13-7, nous avons une liste d'instances de `Rectangle`, et nous utilisons `sort_by_key` pour les ordonner par leur attribut `width` du plus petit au plus grand.

<Listing number="13-7" file-name="src/main.rs" caption="Utilisation de `sort_by_key` pour ordonner les rectangles par largeur">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

<!--
This code prints:
-->

Ce code affiche :


```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

<!--
The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn't capture, mutate, or move anything out from its environment, so
it meets the trait bound requirements.
-->

La raison pour laquelle `sort_by_key` est definie pour prendre une fermeture `FnMut` est qu'elle appelle la fermeture plusieurs fois : une fois pour chaque element de la slice. La fermeture `|r| r.width` ne capture, ne mute ni ne deplace rien hors de son environnement, donc elle respecte les exigences de la contrainte de trait.

<!--
In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won't let us use this closure with `sort_by_key`.
-->

En revanche, l'encart 13-8 montre un exemple de fermeture qui n'implemente que le trait `FnOnce`, car elle deplace une valeur hors de l'environnement. Le compilateur ne nous laissera pas utiliser cette fermeture avec `sort_by_key`.

<Listing number="13-8" file-name="src/main.rs" caption="Tentative d'utilisation d'une fermeture `FnOnce` avec `sort_by_key`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

<!--
This is a contrived, convoluted way (that doesn't work) to try to count the
number of times `sort_by_key` calls the closure when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure's
environment—into the `sort_operations` vector. The closure captures `value` and
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn't work, because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can't be moved out of the closure because the closure must
implement `FnMut`:
-->

Il s'agit d'une facon artificielle et alambiquee (qui ne fonctionne pas) d'essayer de compter le nombre de fois que `sort_by_key` appelle la fermeture lors du tri de `list`. Ce code tente de faire ce comptage en poussant `value` -- une `String` de l'environnement de la fermeture -- dans le vecteur `sort_operations`. La fermeture capture `value` puis deplace `value` hors de la fermeture en transferant la possession de `value` au vecteur `sort_operations`. Cette fermeture ne peut etre appelee qu'une seule fois ; essayer de l'appeler une deuxieme fois ne fonctionnerait pas, car `value` ne serait plus dans l'environnement pour etre poussee dans `sort_operations` a nouveau ! Par consequent, cette fermeture n'implemente que `FnOnce`. Lorsque nous essayons de compiler ce code, nous obtenons cette erreur indiquant que `value` ne peut pas etre deplacee hors de la fermeture car la fermeture doit implementer `FnMut` :


```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

<!--
The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn't
move values out of the environment. Keeping a counter in the environment and
incrementing its value in the closure body is a more straightforward way to
count the number of times the closure is called. The closure in Listing 13-9
works with `sort_by_key` because it is only capturing a mutable reference to the
`num_sort_operations` counter and can therefore be called more than once.
-->

L'erreur pointe vers la ligne dans le corps de la fermeture qui deplace `value` hors de l'environnement. Pour corriger cela, nous devons modifier le corps de la fermeture pour qu'il ne deplace pas de valeurs hors de l'environnement. Garder un compteur dans l'environnement et incrementer sa valeur dans le corps de la fermeture est une facon plus directe de compter le nombre de fois que la fermeture est appelee. La fermeture de l'encart 13-9 fonctionne avec `sort_by_key` car elle ne capture qu'une reference mutable vers le compteur `num_sort_operations` et peut donc etre appelee plus d'une fois.

<Listing number="13-9" file-name="src/main.rs" caption="L'utilisation d'une fermeture `FnMut` avec `sort_by_key` est autorisee.">


```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

<!--
The `Fn` traits are important when defining or using functions or types that
make use of closures. In the next section, we'll discuss iterators. Many
iterator methods take closure arguments, so keep these closure details in mind
as we continue!
-->

Les traits `Fn` sont importants lors de la definition ou de l'utilisation de fonctions ou de types qui utilisent des fermetures. Dans la prochaine section, nous aborderons les iterateurs. De nombreuses methodes d'iterateurs prennent des fermetures en arguments, alors gardez ces details sur les fermetures a l'esprit pendant que nous continuons !

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
