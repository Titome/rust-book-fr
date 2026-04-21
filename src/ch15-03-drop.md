<!--
## Running Code on Cleanup with the `Drop` Trait
-->

## Exécuter du code au nettoyage avec le trait `Drop`

<!--
The second trait important to the smart pointer pattern is `Drop`, which lets
you customize what happens when a value is about to go out of scope. You can
provide an implementation for the `Drop` trait on any type, and that code can
be used to release resources like files or network connections.
-->

Le deuxième trait important pour le patron des pointeurs intelligents est `Drop`, qui vous permet de personnaliser ce qui se passe quand une valeur est sur le point de sortir de la portée. Vous pouvez fournir une implémentation du trait `Drop` sur n'importe quel type, et ce code peut être utilisé pour libérer des ressources comme des fichiers ou des connexions réseau.

<!--
We're introducing `Drop` in the context of smart pointers because the
functionality of the `Drop` trait is almost always used when implementing a
smart pointer. For example, when a `Box<T>` is dropped, it will deallocate the
space on the heap that the box points to.
-->

Nous introduisons `Drop` dans le contexte des pointeurs intelligents car la fonctionnalité du trait `Drop` est presque toujours utilisée lors de l'implémentation d'un pointeur intelligent. Par exemple, quand un `Box<T>` est libéré (dropped), il désallouera l'espace sur le tas vers lequel la boîte pointe.

<!--
In some languages, for some types, the programmer must call code to free memory
or resources every time they finish using an instance of those types. Examples
include file handles, sockets, and locks. If the programmer forgets, the system
might become overloaded and crash. In Rust, you can specify that a particular
bit of code be run whenever a value goes out of scope, and the compiler will
insert this code automatically. As a result, you don't need to be careful about
placing cleanup code everywhere in a program that an instance of a particular
type is finished with—you still won't leak resources!
-->

Dans certains langages, pour certains types, le programmeur doit appeler du code pour libérer la mémoire ou les ressources chaque fois qu'il a fini d'utiliser une instance de ces types. Les exemples incluent les descripteurs de fichiers, les sockets et les verrous. Si le programmeur oublie, le système peut devenir surchargé et planter. En Rust, vous pouvez spécifier qu'un morceau de code particulier soit exécuté chaque fois qu'une valeur sort de la portée, et le compilateur insérera ce code automatiquement. En conséquence, vous n'avez pas besoin de faire attention à placer du code de nettoyage partout dans le programme quand vous avez fini avec une instance d'un type particulier -- vous ne fuirez toujours pas de ressources !

<!--
You specify the code to run when a value goes out of scope by implementing the
`Drop` trait. The `Drop` trait requires you to implement one method named
`drop` that takes a mutable reference to `self`. To see when Rust calls `drop`,
let's implement `drop` with `println!` statements for now.
-->

Vous spécifiez le code à exécuter quand une valeur sort de la portée en implémentant le trait `Drop`. Le trait `Drop` vous demande d'implémenter une méthode nommée `drop` qui prend une référence mutable vers `self`. Pour voir quand Rust appelle `drop`, implémentons `drop` avec des instructions `println!` pour l'instant.

<!--
Listing 15-14 shows a `CustomSmartPointer` struct whose only custom
functionality is that it will print `Dropping CustomSmartPointer!` when the
instance goes out of scope, to show when Rust runs the `drop` method.
-->

L'encart 15-14 montre une struct `CustomSmartPointer` dont la seule fonctionnalité personnalisée est qu'elle affichera `Dropping CustomSmartPointer!` quand l'instance sort de la portée, pour montrer quand Rust exécute la méthode `drop`.

<Listing number="15-14" file-name="src/main.rs" caption="Une struct `CustomSmartPointer` qui implémente le trait `Drop` où nous placerions notre code de nettoyage">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

<!--
The `Drop` trait is included in the prelude, so we don't need to bring it into
scope. We implement the `Drop` trait on `CustomSmartPointer` and provide an
implementation for the `drop` method that calls `println!`. The body of the
`drop` method is where you would place any logic that you wanted to run when an
instance of your type goes out of scope. We're printing some text here to
demonstrate visually when Rust will call `drop`.
-->

Le trait `Drop` est inclus dans le prélude, nous n'avons donc pas besoin de l'importer dans la portée. Nous implémentons le trait `Drop` sur `CustomSmartPointer` et fournissons une implémentation de la méthode `drop` qui appelle `println!`. Le corps de la méthode `drop` est l'endroit où vous placeriez toute logique que vous souhaitez exécuter quand une instance de votre type sort de la portée. Nous affichons du texte ici pour démontrer visuellement quand Rust appellera `drop`.

<!--
In `main`, we create two instances of `CustomSmartPointer` and then print
`CustomSmartPointers created`. At the end of `main`, our instances of
`CustomSmartPointer` will go out of scope, and Rust will call the code we put
in the `drop` method, printing our final message. Note that we didn't need to
call the `drop` method explicitly.
-->

Dans `main`, nous créons deux instances de `CustomSmartPointer` puis affichons `CustomSmartPointers created`. À la fin de `main`, nos instances de `CustomSmartPointer` sortiront de la portée, et Rust appellera le code que nous avons mis dans la méthode `drop`, affichant notre message final. Notez que nous n'avons pas eu besoin d'appeler la méthode `drop` explicitement.

<!--
When we run this program, we'll see the following output:
-->

Lorsque nous exécutons ce programme, nous verrons la sortie suivante :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

<!--
Rust automatically called `drop` for us when our instances went out of scope,
calling the code we specified. Variables are dropped in the reverse order of
their creation, so `d` was dropped before `c`. This example's purpose is to
give you a visual guide to how the `drop` method works; usually you would
specify the cleanup code that your type needs to run rather than a print
message.
-->

Rust a automatiquement appelé `drop` pour nous quand nos instances sont sorties de la portée, en appelant le code que nous avons spécifié. Les variables sont libérées dans l'ordre inverse de leur création, donc `d` a été libéré avant `c`. Le but de cet exemple est de vous donner un guide visuel de comment la méthode `drop` fonctionne ; habituellement, vous spécifieriez le code de nettoyage que votre type doit exécuter plutôt qu'un message d'affichage.

<!--
Old headings. Do not remove or links may break.
-->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

<!--
Unfortunately, it's not straightforward to disable the automatic `drop`
functionality. Disabling `drop` isn't usually necessary; the whole point of the
`Drop` trait is that it's taken care of automatically. Occasionally, however,
you might want to clean up a value early. One example is when using smart
pointers that manage locks: You might want to force the `drop` method that
releases the lock so that other code in the same scope can acquire the lock.
Rust doesn't let you call the `Drop` trait's `drop` method manually; instead,
you have to call the `std::mem::drop` function provided by the standard library
if you want to force a value to be dropped before the end of its scope.
-->

Malheureusement, il n'est pas simple de désactiver la fonctionnalité automatique de `drop`. Désactiver `drop` n'est généralement pas nécessaire ; tout l'intérêt du trait `Drop` est qu'il est géré automatiquement. Cependant, il arrive parfois que vous souhaitiez nettoyer une valeur plus tôt. Un exemple est lorsque vous utilisez des pointeurs intelligents qui gèrent des verrous : vous pourriez vouloir forcer la méthode `drop` qui libère le verrou afin que d'autre code dans la même portée puisse acquérir le verrou. Rust ne vous permet pas d'appeler manuellement la méthode `drop` du trait `Drop` ; à la place, vous devez appeler la fonction `std::mem::drop` fournie par la bibliothèque standard si vous voulez forcer la libération d'une valeur avant la fin de sa portée.

<!--
Trying to call the `Drop` trait's `drop` method manually by modifying the
`main` function from Listing 15-14 won't work, as shown in Listing 15-15.
-->

Essayer d'appeler manuellement la méthode `drop` du trait `Drop` en modifiant la fonction `main` de l'encart 15-14 ne fonctionnera pas, comme montré dans l'encart 15-15.

<Listing number="15-15" file-name="src/main.rs" caption="Tentative d'appel manuel de la méthode `drop` du trait `Drop` pour nettoyer plus tôt">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

<!--
When we try to compile this code, we'll get this error:
-->

Lorsque nous essayons de compiler ce code, nous obtenons cette erreur :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

<!--
This error message states that we're not allowed to explicitly call `drop`. The
error message uses the term _destructor_, which is the general programming term
for a function that cleans up an instance. A _destructor_ is analogous to a
_constructor_, which creates an instance. The `drop` function in Rust is one
particular destructor.
-->

Ce message d'erreur indique que nous ne sommes pas autorisés à appeler explicitement `drop`. Le message d'erreur utilise le terme _destructeur_, qui est le terme de programmation générale pour une fonction qui nettoie une instance. Un _destructeur_ est l'analogue d'un _constructeur_, qui crée une instance. La fonction `drop` en Rust est un destructeur particulier.

<!--
Rust doesn't let us call `drop` explicitly, because Rust would still
automatically call `drop` on the value at the end of `main`. This would cause a
double free error because Rust would be trying to clean up the same value twice.
-->

Rust ne nous permet pas d'appeler `drop` explicitement, car Rust appellerait quand même automatiquement `drop` sur la valeur à la fin de `main`. Cela provoquerait une erreur de double libération car Rust essaierait de nettoyer la même valeur deux fois.

<!--
We can't disable the automatic insertion of `drop` when a value goes out of
scope, and we can't call the `drop` method explicitly. So, if we need to force
a value to be cleaned up early, we use the `std::mem::drop` function.
-->

Nous ne pouvons pas désactiver l'insertion automatique de `drop` quand une valeur sort de la portée, et nous ne pouvons pas appeler la méthode `drop` explicitement. Donc, si nous avons besoin de forcer le nettoyage anticipé d'une valeur, nous utilisons la fonction `std::mem::drop`.

<!--
The `std::mem::drop` function is different from the `drop` method in the `Drop`
trait. We call it by passing as an argument the value we want to force-drop.
The function is in the prelude, so we can modify `main` in Listing 15-15 to
call the `drop` function, as shown in Listing 15-16.
-->

La fonction `std::mem::drop` est différente de la méthode `drop` du trait `Drop`. Nous l'appelons en passant comme argument la valeur que nous voulons forcer à libérer. La fonction est dans le prélude, donc nous pouvons modifier `main` dans l'encart 15-15 pour appeler la fonction `drop`, comme montré dans l'encart 15-16.

<Listing number="15-16" file-name="src/main.rs" caption="Appeler `std::mem::drop` pour libérer explicitement une valeur avant qu'elle ne sorte de la portée">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

<!--
Running this code will print the following:
-->

L'exécution de ce code affichera ce qui suit :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

<!--
The text ``Dropping CustomSmartPointer with data `some data`!`` is printed
between the `CustomSmartPointer created` and `CustomSmartPointer dropped before
the end of main` text, showing that the `drop` method code is called to drop
`c` at that point.
-->

Le texte ``Dropping CustomSmartPointer with data `some data`!`` est affiché entre les textes `CustomSmartPointer created` et `CustomSmartPointer dropped before the end of main`, montrant que le code de la méthode `drop` est appelé pour libérer `c` à ce moment-là.

<!--
You can use code specified in a `Drop` trait implementation in many ways to
make cleanup convenient and safe: For instance, you could use it to create your
own memory allocator! With the `Drop` trait and Rust's ownership system, you
don't have to remember to clean up, because Rust does it automatically.
-->

Vous pouvez utiliser le code spécifié dans une implémentation du trait `Drop` de nombreuses manières pour rendre le nettoyage pratique et sûr : par exemple, vous pourriez l'utiliser pour créer votre propre allocateur de mémoire ! Avec le trait `Drop` et le système de possession de Rust, vous n'avez pas à vous souvenir de nettoyer, car Rust le fait automatiquement.

<!--
You also don't have to worry about problems resulting from accidentally
cleaning up values still in use: The ownership system that makes sure
references are always valid also ensures that `drop` gets called only once when
the value is no longer being used.
-->

Vous n'avez pas non plus à vous soucier des problèmes résultant du nettoyage accidentel de valeurs encore utilisées : le système de possession qui s'assure que les références sont toujours valides garantit également que `drop` n'est appelé qu'une seule fois lorsque la valeur n'est plus utilisée.

<!--
Now that we've examined `Box<T>` and some of the characteristics of smart
pointers, let's look at a few other smart pointers defined in the standard
library.
-->

Maintenant que nous avons examiné `Box<T>` et certaines des caractéristiques des pointeurs intelligents, examinons quelques autres pointeurs intelligents définis dans la bibliothèque standard.
