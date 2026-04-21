<!--
## `RefCell<T>` and the Interior Mutability Pattern
-->

## `RefCell<T>` et le patron de mutabilité intérieure

<!--
_Interior mutability_ is a design pattern in Rust that allows you to mutate
data even when there are immutable references to that data; normally, this
action is disallowed by the borrowing rules. To mutate data, the pattern uses
`unsafe` code inside a data structure to bend Rust's usual rules that govern
mutation and borrowing. Unsafe code indicates to the compiler that we're
checking the rules manually instead of relying on the compiler to check them
for us; we will discuss unsafe code more in Chapter 20.
-->

La _mutabilité intérieure_ est un patron de conception en Rust qui vous permet de modifier des données même lorsqu'il existe des références immuables vers ces données ; normalement, cette action est interdite par les règles d'emprunt. Pour modifier des données, le patron utilise du code `unsafe` à l'intérieur d'une structure de données pour contourner les règles habituelles de Rust qui régissent la mutation et l'emprunt. Le code unsafe indique au compilateur que nous vérifions les règles manuellement au lieu de compter sur le compilateur pour les vérifier à notre place ; nous aborderons le code unsafe plus en détail au chapitre 20.

<!--
We can use types that use the interior mutability pattern only when we can
ensure that the borrowing rules will be followed at runtime, even though the
compiler can't guarantee that. The `unsafe` code involved is then wrapped in a
safe API, and the outer type is still immutable.
-->

Nous ne pouvons utiliser les types qui emploient le patron de mutabilité intérieure que lorsque nous pouvons nous assurer que les règles d'emprunt seront respectées à l'exécution, même si le compilateur ne peut pas le garantir. Le code `unsafe` impliqué est alors enveloppé dans une API sûre, et le type externe reste immuable.

<!--
Let's explore this concept by looking at the `RefCell<T>` type that follows the
interior mutability pattern.
-->

Explorons ce concept en examinant le type `RefCell<T>` qui suit le patron de mutabilité intérieure.

<!--
Old headings. Do not remove or links may break.
-->

<a id="enforcing-borrowing-rules-at-runtime-with-refcellt"></a>

<!--
### Enforcing Borrowing Rules at Runtime
-->

### Appliquer les règles d'emprunt à l'exécution

<!--
Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
it holds. So, what makes `RefCell<T>` different from a type like `Box<T>`?
Recall the borrowing rules you learned in Chapter 4:

- At any given time, you can have _either_ one mutable reference or any number
  of immutable references (but not both).
- References must always be valid.
-->

Contrairement à `Rc<T>`, le type `RefCell<T>` représente une possession unique des données qu'il contient. Alors, qu'est-ce qui rend `RefCell<T>` différent d'un type comme `Box<T>` ? Rappelez-vous les règles d'emprunt que vous avez apprises au chapitre 4 :

- À tout moment, vous pouvez avoir _soit_ une référence mutable, _soit_ un nombre quelconque de références immuables (mais pas les deux).
- Les références doivent toujours être valides.

<!--
With references and `Box<T>`, the borrowing rules' invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced _at runtime_.
With references, if you break these rules, you'll get a compiler error. With
`RefCell<T>`, if you break these rules, your program will panic and exit.
-->

Avec les références et `Box<T>`, les invariants des règles d'emprunt sont appliqués à la compilation. Avec `RefCell<T>`, ces invariants sont appliqués _à l'exécution_. Avec les références, si vous enfreignez ces règles, vous obtiendrez une erreur de compilation. Avec `RefCell<T>`, si vous enfreignez ces règles, votre programme paniquera et se terminera.

<!--
The advantages of checking the borrowing rules at compile time are that errors
will be caught sooner in the development process, and there is no impact on
runtime performance because all the analysis is completed beforehand. For those
reasons, checking the borrowing rules at compile time is the best choice in the
majority of cases, which is why this is Rust's default.
-->

Les avantages de vérifier les règles d'emprunt à la compilation sont que les erreurs seront détectées plus tôt dans le processus de développement, et qu'il n'y a aucun impact sur les performances à l'exécution car toute l'analyse est effectuée au préalable. Pour ces raisons, vérifier les règles d'emprunt à la compilation est le meilleur choix dans la majorité des cas, c'est pourquoi c'est le comportement par défaut de Rust.

<!--
The advantage of checking the borrowing rules at runtime instead is that
certain memory-safe scenarios are then allowed, where they would've been
disallowed by the compile-time checks. Static analysis, like the Rust compiler,
is inherently conservative. Some properties of code are impossible to detect by
analyzing the code: The most famous example is the Halting Problem, which is
beyond the scope of this book but is an interesting topic to research.
-->

L'avantage de vérifier les règles d'emprunt à l'exécution à la place est que certains scénarios sûrs en mémoire sont alors autorisés, là où ils auraient été interdits par les vérifications à la compilation. L'analyse statique, comme celle du compilateur Rust, est intrinsèquement conservatrice. Certaines propriétés du code sont impossibles à détecter en analysant le code : l'exemple le plus célèbre est le problème de l'arrêt, qui dépasse le cadre de ce livre mais constitue un sujet de recherche intéressant.

<!--
Because some analysis is impossible, if the Rust compiler can't be sure the
code complies with the ownership rules, it might reject a correct program; in
this way, it's conservative. If Rust accepted an incorrect program, users
wouldn't be able to trust the guarantees Rust makes. However, if Rust rejects a
correct program, the programmer will be inconvenienced, but nothing
catastrophic can occur. The `RefCell<T>` type is useful when you're sure your
code follows the borrowing rules but the compiler is unable to understand and
guarantee that.
-->

Comme certaines analyses sont impossibles, si le compilateur Rust ne peut pas être sûr que le code respecte les règles de possession, il pourrait rejeter un programme correct ; en ce sens, il est conservateur. Si Rust acceptait un programme incorrect, les utilisateurs ne pourraient pas faire confiance aux garanties que Rust offre. Cependant, si Rust rejette un programme correct, le programmeur sera gêné, mais rien de catastrophique ne peut se produire. Le type `RefCell<T>` est utile quand vous êtes sûr que votre code suit les règles d'emprunt mais que le compilateur est incapable de le comprendre et de le garantir.

<!--
Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios
and will give you a compile-time error if you try using it in a multithreaded
context. We'll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in Chapter 16.
-->

De manière similaire à `Rc<T>`, `RefCell<T>` est uniquement destiné aux scénarios mono-thread et vous donnera une erreur de compilation si vous essayez de l'utiliser dans un contexte multi-thread. Nous parlerons de comment obtenir la fonctionnalité de `RefCell<T>` dans un programme multi-thread au chapitre 16.

<!--
Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
  have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
  allows only immutable borrows checked at compile time; `RefCell<T>` allows
  immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can
  mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is
  immutable.
-->

Voici un récapitulatif des raisons de choisir `Box<T>`, `Rc<T>` ou `RefCell<T>` :

- `Rc<T>` permet d'avoir plusieurs propriétaires des mêmes données ; `Box<T>` et `RefCell<T>` ont un seul propriétaire.
- `Box<T>` permet des emprunts immuables ou mutables vérifiés à la compilation ; `Rc<T>` ne permet que des emprunts immuables vérifiés à la compilation ; `RefCell<T>` permet des emprunts immuables ou mutables vérifiés à l'exécution.
- Comme `RefCell<T>` permet des emprunts mutables vérifiés à l'exécution, vous pouvez modifier la valeur à l'intérieur du `RefCell<T>` même quand le `RefCell<T>` est immuable.

<!--
Mutating the value inside an immutable value is the interior mutability
pattern. Let's look at a situation in which interior mutability is useful and
examine how it's possible.
-->

Modifier la valeur à l'intérieur d'une valeur immuable est le patron de mutabilité intérieure. Examinons une situation dans laquelle la mutabilité intérieure est utile et voyons comment c'est possible.

<!--
Old headings. Do not remove or links may break.
-->

<a id="interior-mutability-a-mutable-borrow-to-an-immutable-value"></a>

<!--
### Using Interior Mutability
-->

### Utiliser la mutabilité intérieure

<!--
A consequence of the borrowing rules is that when you have an immutable value,
you can't borrow it mutably. For example, this code won't compile:
-->

Une conséquence des règles d'emprunt est que quand vous avez une valeur immuable, vous ne pouvez pas l'emprunter de manière mutable. Par exemple, ce code ne compilera pas :

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

<!--
If you tried to compile this code, you'd get the following error:
-->

Si vous tentiez de compiler ce code, vous obtiendriez l'erreur suivante :

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

<!--
However, there are situations in which it would be useful for a value to mutate
itself in its methods but appear immutable to other code. Code outside the
value's methods would not be able to mutate the value. Using `RefCell<T>` is
one way to get the ability to have interior mutability, but `RefCell<T>`
doesn't get around the borrowing rules completely: The borrow checker in the
compiler allows this interior mutability, and the borrowing rules are checked
at runtime instead. If you violate the rules, you'll get a `panic!` instead of
a compiler error.
-->

Cependant, il existe des situations dans lesquelles il serait utile qu'une valeur se modifie elle-même dans ses méthodes mais apparaisse immuable au code extérieur. Le code en dehors des méthodes de la valeur ne pourrait pas modifier la valeur. Utiliser `RefCell<T>` est un moyen d'obtenir la capacité d'avoir une mutabilité intérieure, mais `RefCell<T>` ne contourne pas complètement les règles d'emprunt : le vérificateur d'emprunts dans le compilateur autorise cette mutabilité intérieure, et les règles d'emprunt sont vérifiées à l'exécution à la place. Si vous violez les règles, vous obtiendrez un `panic!` au lieu d'une erreur de compilation.

<!--
Let's work through a practical example where we can use `RefCell<T>` to mutate
an immutable value and see why that is useful.
-->

Travaillons sur un exemple pratique où nous pouvons utiliser `RefCell<T>` pour modifier une valeur immuable et voir pourquoi c'est utile.

<!--
Old headings. Do not remove or links may break.
-->

<a id="a-use-case-for-interior-mutability-mock-objects"></a>

<!--
#### Testing with Mock Objects
-->

#### Tester avec des objets simulés

<!--
Sometimes during testing a programmer will use a type in place of another type,
in order to observe particular behavior and assert that it's implemented
correctly. This placeholder type is called a _test double_. Think of it in the
sense of a stunt double in filmmaking, where a person steps in and substitutes
for an actor to do a particularly tricky scene. Test doubles stand in for other
types when we're running tests. _Mock objects_ are specific types of test
doubles that record what happens during a test so that you can assert that the
correct actions took place.
-->

Parfois, lors des tests, un programmeur utilise un type à la place d'un autre type, afin d'observer un comportement particulier et de vérifier qu'il est correctement implémenté. Ce type de substitution s'appelle un _doublure de test_ (test double). Pensez-y comme une doublure cascade au cinéma, où une personne se substitue à un acteur pour réaliser une scène particulièrement périlleuse. Les doublures de test remplacent d'autres types lorsque nous exécutons des tests. Les _objets simulés_ (mock objects) sont des types spécifiques de doublures de test qui enregistrent ce qui se passe pendant un test afin que vous puissiez vérifier que les actions correctes ont eu lieu.

<!--
Rust doesn't have objects in the same sense as other languages have objects,
and Rust doesn't have mock object functionality built into the standard library
as some other languages do. However, you can definitely create a struct that
will serve the same purposes as a mock object.
-->

Rust n'a pas d'objets au même sens que d'autres langages, et Rust n'a pas de fonctionnalité d'objets simulés intégrée dans la bibliothèque standard comme le font certains autres langages. Cependant, vous pouvez tout à fait créer une struct qui servira les mêmes objectifs qu'un objet simulé.

<!--
Here's the scenario we'll test: We'll create a library that tracks a value
against a maximum value and sends messages based on how close to the maximum
value the current value is. This library could be used to keep track of a
user's quota for the number of API calls they're allowed to make, for example.
-->

Voici le scénario que nous allons tester : nous allons créer une bibliothèque qui suit une valeur par rapport à une valeur maximale et envoie des messages en fonction de la proximité de la valeur courante par rapport à la valeur maximale. Cette bibliothèque pourrait être utilisée pour suivre le quota d'un utilisateur pour le nombre d'appels API qu'il est autorisé à effectuer, par exemple.

<!--
Our library will only provide the functionality of tracking how close to the
maximum a value is and what the messages should be at what times. Applications
that use our library will be expected to provide the mechanism for sending the
messages: The application could show the message to the user directly, send an
email, send a text message, or do something else. The library doesn't need to
know that detail. All it needs is something that implements a trait we'll
provide, called `Messenger`. Listing 15-20 shows the library code.
-->

Notre bibliothèque ne fournira que la fonctionnalité de suivi de la proximité d'une valeur par rapport au maximum et les messages à envoyer à quels moments. Les applications qui utilisent notre bibliothèque devront fournir le mécanisme d'envoi des messages : l'application pourrait afficher le message directement à l'utilisateur, envoyer un email, envoyer un SMS, ou faire autre chose. La bibliothèque n'a pas besoin de connaître ce détail. Tout ce dont elle a besoin est quelque chose qui implémente un trait que nous fournirons, appelé `Messenger`. L'encart 15-20 montre le code de la bibliothèque.

<Listing number="15-20" file-name="src/lib.rs" caption="Une bibliothèque pour suivre la proximité d'une valeur par rapport à une valeur maximale et avertir quand la valeur atteint certains niveaux">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

<!--
One important part of this code is that the `Messenger` trait has one method
called `send` that takes an immutable reference to `self` and the text of the
message. This trait is the interface our mock object needs to implement so that
the mock can be used in the same way a real object is. The other important part
is that we want to test the behavior of the `set_value` method on the
`LimitTracker`. We can change what we pass in for the `value` parameter, but
`set_value` doesn't return anything for us to make assertions on. We want to be
able to say that if we create a `LimitTracker` with something that implements
the `Messenger` trait and a particular value for `max`, the messenger is told
to send the appropriate messages when we pass different numbers for `value`.
-->

Une partie importante de ce code est que le trait `Messenger` a une méthode appelée `send` qui prend une référence immuable vers `self` et le texte du message. Ce trait est l'interface que notre objet simulé doit implémenter pour que le simulacre puisse être utilisé de la même manière qu'un vrai objet. L'autre partie importante est que nous voulons tester le comportement de la méthode `set_value` sur le `LimitTracker`. Nous pouvons changer ce que nous passons pour le paramètre `value`, mais `set_value` ne retourne rien sur quoi faire des assertions. Nous voulons pouvoir dire que si nous créons un `LimitTracker` avec quelque chose qui implémente le trait `Messenger` et une valeur particulière pour `max`, le messager reçoit l'instruction d'envoyer les messages appropriés quand nous passons différents nombres pour `value`.

<!--
We need a mock object that, instead of sending an email or text message when we
call `send`, will only keep track of the messages it's told to send. We can
create a new instance of the mock object, create a `LimitTracker` that uses the
mock object, call the `set_value` method on `LimitTracker`, and then check that
the mock object has the messages we expect. Listing 15-21 shows an attempt to
implement a mock object to do just that, but the borrow checker won't allow it.
-->

Nous avons besoin d'un objet simulé qui, au lieu d'envoyer un email ou un SMS quand nous appelons `send`, ne fera que garder trace des messages qu'on lui demande d'envoyer. Nous pouvons créer une nouvelle instance de l'objet simulé, créer un `LimitTracker` qui utilise l'objet simulé, appeler la méthode `set_value` sur `LimitTracker`, puis vérifier que l'objet simulé contient les messages attendus. L'encart 15-21 montre une tentative d'implémentation d'un objet simulé pour faire exactement cela, mais le vérificateur d'emprunts ne le permettra pas.

<Listing number="15-21" file-name="src/lib.rs" caption="Une tentative d'implémentation d'un `MockMessenger` qui n'est pas autorisée par le vérificateur d'emprunts">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

<!--
This test code defines a `MockMessenger` struct that has a `sent_messages`
field with a `Vec` of `String` values to keep track of the messages it's told
to send. We also define an associated function `new` to make it convenient to
create new `MockMessenger` values that start with an empty list of messages. We
then implement the `Messenger` trait for `MockMessenger` so that we can give a
`MockMessenger` to a `LimitTracker`. In the definition of the `send` method, we
take the message passed in as a parameter and store it in the `MockMessenger`
list of `sent_messages`.
-->

Ce code de test définit une struct `MockMessenger` qui a un champ `sent_messages` avec un `Vec` de valeurs `String` pour garder trace des messages qu'on lui demande d'envoyer. Nous définissons aussi une fonction associée `new` pour faciliter la création de nouvelles valeurs `MockMessenger` qui commencent avec une liste vide de messages. Nous implémentons ensuite le trait `Messenger` pour `MockMessenger` afin de pouvoir donner un `MockMessenger` à un `LimitTracker`. Dans la définition de la méthode `send`, nous prenons le message passé en paramètre et le stockons dans la liste `sent_messages` du `MockMessenger`.

<!--
In the test, we're testing what happens when the `LimitTracker` is told to set
`value` to something that is more than 75 percent of the `max` value. First, we
create a new `MockMessenger`, which will start with an empty list of messages.
Then, we create a new `LimitTracker` and give it a reference to the new
`MockMessenger` and a `max` value of `100`. We call the `set_value` method on
the `LimitTracker` with a value of `80`, which is more than 75 percent of 100.
Then, we assert that the list of messages that the `MockMessenger` is keeping
track of should now have one message in it.
-->

Dans le test, nous testons ce qui se passe quand on demande au `LimitTracker` de définir `value` à quelque chose qui représente plus de 75 pour cent de la valeur `max`. D'abord, nous créons un nouveau `MockMessenger`, qui commencera avec une liste vide de messages. Ensuite, nous créons un nouveau `LimitTracker` et lui donnons une référence vers le nouveau `MockMessenger` et une valeur `max` de `100`. Nous appelons la méthode `set_value` sur le `LimitTracker` avec une valeur de `80`, qui est plus de 75 pour cent de 100. Puis, nous vérifions que la liste de messages que le `MockMessenger` suit devrait maintenant contenir un message.

<!--
However, there's one problem with this test, as shown here:
-->

Cependant, il y a un problème avec ce test, comme montré ici :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

<!--
We can't modify the `MockMessenger` to keep track of the messages, because the
`send` method takes an immutable reference to `self`. We also can't take the
suggestion from the error text to use `&mut self` in both the `impl` method and
the trait definition. We do not want to change the `Messenger` trait solely for
the sake of testing. Instead, we need to find a way to make our test code work
correctly with our existing design.
-->

Nous ne pouvons pas modifier le `MockMessenger` pour garder trace des messages, car la méthode `send` prend une référence immuable vers `self`. Nous ne pouvons pas non plus suivre la suggestion du texte d'erreur d'utiliser `&mut self` à la fois dans la méthode `impl` et la définition du trait. Nous ne voulons pas changer le trait `Messenger` uniquement pour les besoins des tests. Au lieu de cela, nous devons trouver un moyen de faire fonctionner correctement notre code de test avec notre conception existante.

<!--
This is a situation in which interior mutability can help! We'll store the
`sent_messages` within a `RefCell<T>`, and then the `send` method will be able
to modify `sent_messages` to store the messages we've seen. Listing 15-22 shows
what that looks like.
-->

C'est une situation dans laquelle la mutabilité intérieure peut aider ! Nous allons stocker les `sent_messages` dans un `RefCell<T>`, et alors la méthode `send` pourra modifier `sent_messages` pour stocker les messages que nous avons vus. L'encart 15-22 montre à quoi cela ressemble.

<Listing number="15-22" file-name="src/lib.rs" caption="Utiliser `RefCell<T>` pour modifier une valeur intérieure alors que la valeur extérieure est considérée comme immuable">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

<!--
The `sent_messages` field is now of type `RefCell<Vec<String>>` instead of
`Vec<String>`. In the `new` function, we create a new `RefCell<Vec<String>>`
instance around the empty vector.
-->

Le champ `sent_messages` est maintenant de type `RefCell<Vec<String>>` au lieu de `Vec<String>`. Dans la fonction `new`, nous créons une nouvelle instance `RefCell<Vec<String>>` autour du vecteur vide.

<!--
For the implementation of the `send` method, the first parameter is still an
immutable borrow of `self`, which matches the trait definition. We call
`borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a
mutable reference to the value inside the `RefCell<Vec<String>>`, which is the
vector. Then, we can call `push` on the mutable reference to the vector to keep
track of the messages sent during the test.
-->

Pour l'implémentation de la méthode `send`, le premier paramètre est toujours un emprunt immuable de `self`, ce qui correspond à la définition du trait. Nous appelons `borrow_mut` sur le `RefCell<Vec<String>>` dans `self.sent_messages` pour obtenir une référence mutable vers la valeur à l'intérieur du `RefCell<Vec<String>>`, qui est le vecteur. Ensuite, nous pouvons appeler `push` sur la référence mutable vers le vecteur pour garder trace des messages envoyés pendant le test.

<!--
The last change we have to make is in the assertion: To see how many items are
in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an
immutable reference to the vector.
-->

Le dernier changement que nous devons faire est dans l'assertion : pour voir combien d'éléments se trouvent dans le vecteur intérieur, nous appelons `borrow` sur le `RefCell<Vec<String>>` pour obtenir une référence immuable vers le vecteur.

<!--
Now that you've seen how to use `RefCell<T>`, let's dig into how it works!
-->

Maintenant que vous avez vu comment utiliser `RefCell<T>`, examinons comment il fonctionne !

<!--
Old headings. Do not remove or links may break.
-->

<a id="keeping-track-of-borrows-at-runtime-with-refcellt"></a>

<!--
#### Tracking Borrows at Runtime
-->

#### Suivre les emprunts à l'exécution

<!--
When creating immutable and mutable references, we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut`
returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we
can treat them like regular references.
-->

Lors de la création de références immuables et mutables, nous utilisons respectivement la syntaxe `&` et `&mut`. Avec `RefCell<T>`, nous utilisons les méthodes `borrow` et `borrow_mut`, qui font partie de l'API sûre de `RefCell<T>`. La méthode `borrow` retourne le type de pointeur intelligent `Ref<T>`, et `borrow_mut` retourne le type de pointeur intelligent `RefMut<T>`. Les deux types implémentent `Deref`, nous pouvons donc les traiter comme des références classiques.

<!--
The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart
pointers are currently active. Every time we call `borrow`, the `RefCell<T>`
increases its count of how many immutable borrows are active. When a `Ref<T>`
value goes out of scope, the count of immutable borrows goes down by 1. Just
like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable
borrows or one mutable borrow at any point in time.
-->

Le `RefCell<T>` suit le nombre de pointeurs intelligents `Ref<T>` et `RefMut<T>` actuellement actifs. Chaque fois que nous appelons `borrow`, le `RefCell<T>` augmente son compteur d'emprunts immuables actifs. Quand une valeur `Ref<T>` sort de la portée, le compteur d'emprunts immuables diminue de 1. Tout comme les règles d'emprunt à la compilation, `RefCell<T>` nous permet d'avoir de nombreux emprunts immuables ou un seul emprunt mutable à tout moment.

<!--
If we try to violate these rules, rather than getting a compiler error as we
would with references, the implementation of `RefCell<T>` will panic at
runtime. Listing 15-23 shows a modification of the implementation of `send` in
Listing 15-22. We're deliberately trying to create two mutable borrows active
for the same scope to illustrate that `RefCell<T>` prevents us from doing this
at runtime.
-->

Si nous essayons de violer ces règles, plutôt que d'obtenir une erreur de compilation comme ce serait le cas avec les références, l'implémentation de `RefCell<T>` paniquera à l'exécution. L'encart 15-23 montre une modification de l'implémentation de `send` de l'encart 15-22. Nous essayons délibérément de créer deux emprunts mutables actifs dans la même portée pour illustrer que `RefCell<T>` nous empêche de le faire à l'exécution.

<Listing number="15-23" file-name="src/lib.rs" caption="Créer deux références mutables dans la même portée pour voir que `RefCell<T>` paniquera">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

<!--
We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned
from `borrow_mut`. Then, we create another mutable borrow in the same way in
the variable `two_borrow`. This makes two mutable references in the same scope,
which isn't allowed. When we run the tests for our library, the code in Listing
15-23 will compile without any errors, but the test will fail:
-->

Nous créons une variable `one_borrow` pour le pointeur intelligent `RefMut<T>` retourné par `borrow_mut`. Ensuite, nous créons un autre emprunt mutable de la même manière dans la variable `two_borrow`. Cela crée deux références mutables dans la même portée, ce qui n'est pas autorisé. Quand nous exécutons les tests de notre bibliothèque, le code de l'encart 15-23 compilera sans aucune erreur, mais le test échouera :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

<!--
Notice that the code panicked with the message `already borrowed:
BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing
rules at runtime.
-->

Remarquez que le code a paniqué avec le message `already borrowed: BorrowMutError`. C'est ainsi que `RefCell<T>` gère les violations des règles d'emprunt à l'exécution.

<!--
Choosing to catch borrowing errors at runtime rather than compile time, as
we've done here, means you'd potentially be finding mistakes in your code later
in the development process: possibly not until your code was deployed to
production. Also, your code would incur a small runtime performance penalty as
a result of keeping track of the borrows at runtime rather than compile time.
However, using `RefCell<T>` makes it possible to write a mock object that can
modify itself to keep track of the messages it has seen while you're using it
in a context where only immutable values are allowed. You can use `RefCell<T>`
despite its trade-offs to get more functionality than regular references
provide.
-->

Choisir de capturer les erreurs d'emprunt à l'exécution plutôt qu'à la compilation, comme nous l'avons fait ici, signifie que vous trouveriez potentiellement des erreurs dans votre code plus tard dans le processus de développement : peut-être pas avant que votre code ne soit déployé en production. De plus, votre code subirait une légère pénalité de performance à l'exécution du fait du suivi des emprunts à l'exécution plutôt qu'à la compilation. Cependant, utiliser `RefCell<T>` rend possible l'écriture d'un objet simulé qui peut se modifier lui-même pour garder trace des messages qu'il a vus tout en l'utilisant dans un contexte où seules les valeurs immuables sont autorisées. Vous pouvez utiliser `RefCell<T>` malgré ses compromis pour obtenir plus de fonctionnalités que ce que les références classiques fournissent.

<!--
Old headings. Do not remove or links may break.
-->

<a id="having-multiple-owners-of-mutable-data-by-combining-rc-t-and-ref-cell-t"></a>
<a id="allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt"></a>

<!--
### Allowing Multiple Owners of Mutable Data
-->

### Permettre plusieurs propriétaires de données mutables

<!--
A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that
`Rc<T>` lets you have multiple owners of some data, but it only gives immutable
access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can
get a value that can have multiple owners _and_ that you can mutate!
-->

Une façon courante d'utiliser `RefCell<T>` est en combinaison avec `Rc<T>`. Rappelez-vous que `Rc<T>` vous permet d'avoir plusieurs propriétaires de certaines données, mais il ne donne qu'un accès immuable à ces données. Si vous avez un `Rc<T>` qui contient un `RefCell<T>`, vous pouvez obtenir une valeur qui peut avoir plusieurs propriétaires _et_ que vous pouvez modifier !

<!--
For example, recall the cons list example in Listing 15-18 where we used
`Rc<T>` to allow multiple lists to share ownership of another list. Because
`Rc<T>` holds only immutable values, we can't change any of the values in the
list once we've created them. Let's add in `RefCell<T>` for its ability to
change the values in the lists. Listing 15-24 shows that by using a
`RefCell<T>` in the `Cons` definition, we can modify the value stored in all
the lists.
-->

Par exemple, rappelez-vous l'exemple de la liste cons de l'encart 15-18 où nous avons utilisé `Rc<T>` pour permettre à plusieurs listes de partager la possession d'une autre liste. Comme `Rc<T>` ne contient que des valeurs immuables, nous ne pouvons pas changer les valeurs de la liste une fois que nous les avons créées. Ajoutons `RefCell<T>` pour sa capacité à changer les valeurs dans les listes. L'encart 15-24 montre qu'en utilisant un `RefCell<T>` dans la définition de `Cons`, nous pouvons modifier la valeur stockée dans toutes les listes.

<Listing number="15-24" file-name="src/main.rs" caption="Utiliser `Rc<RefCell<i32>>` pour créer une `List` que nous pouvons modifier">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

<!--
We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a
variable named `value` so that we can access it directly later. Then, we create
a `List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value` so that both `a` and `value` have ownership of the inner `5` value
rather than transferring ownership from `value` to `a` or having `a` borrow
from `value`.
-->

Nous créons une valeur qui est une instance de `Rc<RefCell<i32>>` et la stockons dans une variable nommée `value` afin de pouvoir y accéder directement plus tard. Ensuite, nous créons une `List` dans `a` avec une variante `Cons` qui contient `value`. Nous devons cloner `value` pour que `a` et `value` aient tous les deux la possession de la valeur intérieure `5` plutôt que de transférer la possession de `value` vers `a` ou de faire emprunter `a` depuis `value`.

<!--
We wrap the list `a` in an `Rc<T>` so that when we create lists `b` and `c`,
they can both refer to `a`, which is what we did in Listing 15-18.
-->

Nous enveloppons la liste `a` dans un `Rc<T>` pour que quand nous créons les listes `b` et `c`, elles puissent toutes les deux faire référence à `a`, ce que nous avons fait dans l'encart 15-18.

<!--
After we've created the lists in `a`, `b`, and `c`, we want to add 10 to the
value in `value`. We do this by calling `borrow_mut` on `value`, which uses the
automatic dereferencing feature we discussed in ["Where's the `->`
Operator?"][wheres-the---operator] ignore
--> in Chapter 5 to dereference
the `Rc<T>` to the inner `RefCell<T>` value. The `borrow_mut` method returns a
`RefMut<T>` smart pointer, and we use the dereference operator on it and change
the inner value.
-->

Après avoir créé les listes dans `a`, `b` et `c`, nous voulons ajouter 10 à la valeur dans `value`. Nous le faisons en appelant `borrow_mut` sur `value`, qui utilise la fonctionnalité de déréférencement automatique dont nous avons discuté dans ["Où est l'opérateur `->` ?"][wheres-the---operator]<!--
ignore
--> au chapitre 5 pour déréférencer le `Rc<T>` vers la valeur intérieure `RefCell<T>`. La méthode `borrow_mut` retourne un pointeur intelligent `RefMut<T>`, et nous utilisons l'opérateur de déréférencement dessus pour changer la valeur intérieure.

<!--
When we print `a`, `b`, and `c`, we can see that they all have the modified
value of `15` rather than `5`:
-->

Quand nous affichons `a`, `b` et `c`, nous pouvons voir qu'ils ont tous la valeur modifiée de `15` plutôt que `5` :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

<!--
This technique is pretty neat! By using `RefCell<T>`, we have an outwardly
immutable `List` value. But we can use the methods on `RefCell<T>` that provide
access to its interior mutability so that we can modify our data when we need
to. The runtime checks of the borrowing rules protect us from data races, and
it's sometimes worth trading a bit of speed for this flexibility in our data
structures. Note that `RefCell<T>` does not work for multithreaded code!
`Mutex<T>` is the thread-safe version of `RefCell<T>`, and we'll discuss
`Mutex<T>` in Chapter 16.
-->

Cette technique est plutôt élégante ! En utilisant `RefCell<T>`, nous avons une valeur `List` extérieurement immuable. Mais nous pouvons utiliser les méthodes de `RefCell<T>` qui fournissent l'accès à sa mutabilité intérieure afin de pouvoir modifier nos données quand nous en avons besoin. Les vérifications à l'exécution des règles d'emprunt nous protègent des courses de données, et il vaut parfois la peine d'échanger un peu de vitesse contre cette flexibilité dans nos structures de données. Notez que `RefCell<T>` ne fonctionne pas pour le code multi-thread ! `Mutex<T>` est la version sûre pour les threads de `RefCell<T>`, et nous aborderons `Mutex<T>` au chapitre 16.

[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
