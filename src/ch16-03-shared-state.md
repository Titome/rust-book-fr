<!--
## Shared-State Concurrency
-->

## La concurrence par état partagé

<!--
Message passing is a fine way to handle concurrency, but it's not the only way.
Another method would be for multiple threads to access the same shared data.
Consider this part of the slogan from the Go language documentation again: "Do
not communicate by sharing memory."
-->

Le passage de messages est un bon moyen de gérer la concurrence, mais ce n'est pas le seul. Une autre méthode serait que plusieurs threads accèdent aux mêmes données partagées. Considérez à nouveau cette partie du slogan de la documentation du langage Go : "Ne communiquez pas en partageant de la mémoire."

<!--
What would communicating by sharing memory look like? In addition, why would
message-passing enthusiasts caution not to use memory sharing?
-->

À quoi ressemblerait la communication par partage de mémoire ? Et pourquoi les partisans du passage de messages recommanderaient-ils de ne pas utiliser le partage de mémoire ?

<!--
In a way, channels in any programming language are similar to single ownership
because once you transfer a value down a channel, you should no longer use that
value. Shared-memory concurrency is like multiple ownership: Multiple threads
can access the same memory location at the same time. As you saw in Chapter 15,
where smart pointers made multiple ownership possible, multiple ownership can
add complexity because these different owners need managing. Rust's type system
and ownership rules greatly assist in getting this management correct. For an
example, let's look at mutexes, one of the more common concurrency primitives
for shared memory.
-->

D'une certaine manière, les canaux dans n'importe quel langage de programmation sont similaires à la possession unique car une fois que vous transférez une valeur dans un canal, vous ne devriez plus utiliser cette valeur. La concurrence par mémoire partagée est comme la possession multiple : plusieurs threads peuvent accéder au même emplacement mémoire en même temps. Comme vous l'avez vu au chapitre 15, où les pointeurs intelligents rendaient possible la possession multiple, la possession multiple peut ajouter de la complexité car ces différents propriétaires doivent être gérés. Le système de types de Rust et les règles de possession aident grandement à effectuer cette gestion correctement. Pour un exemple, examinons les mutex, l'une des primitives de concurrence les plus courantes pour la mémoire partagée.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time"></a>

<!--
### Controlling Access with Mutexes
-->

### Contrôler l'accès avec les mutex

<!--
_Mutex_ is an abbreviation for _mutual exclusion_, as in a mutex allows only
one thread to access some data at any given time. To access the data in a
mutex, a thread must first signal that it wants access by asking to acquire the
mutex's lock. The _lock_ is a data structure that is part of the mutex that
keeps track of who currently has exclusive access to the data. Therefore, the
mutex is described as _guarding_ the data it holds via the locking system.
-->

_Mutex_ est une abréviation de _mutual exclusion_ (exclusion mutuelle), car un mutex ne permet qu'à un seul thread d'accéder à certaines données à un moment donné. Pour accéder aux données dans un mutex, un thread doit d'abord signaler qu'il veut y accéder en demandant à acquérir le verrou du mutex. Le _verrou_ (lock) est une structure de données qui fait partie du mutex et qui suit qui a actuellement l'accès exclusif aux données. Par conséquent, le mutex est décrit comme _gardant_ les données qu'il contient via le système de verrouillage.

<!--
Mutexes have a reputation for being difficult to use because you have to
remember two rules:

1. You must attempt to acquire the lock before using the data.
2. When you're done with the data that the mutex guards, you must unlock the
   data so that other threads can acquire the lock.
-->

Les mutex ont la réputation d'être difficiles à utiliser car vous devez vous souvenir de deux règles :

1. Vous devez tenter d'acquérir le verrou avant d'utiliser les données.
2. Quand vous avez fini avec les données que le mutex garde, vous devez déverrouiller les données pour que d'autres threads puissent acquérir le verrou.

<!--
For a real-world metaphor for a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist can speak, they have to
ask or signal that they want to use the microphone. When they get the
microphone, they can talk for as long as they want to and then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they're finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel won't work
as planned!
-->

Pour une métaphore du monde réel d'un mutex, imaginez une table ronde lors d'une conférence avec un seul microphone. Avant qu'un panéliste puisse parler, il doit demander ou signaler qu'il veut utiliser le microphone. Quand il obtient le microphone, il peut parler aussi longtemps qu'il le souhaite puis passer le microphone au prochain panéliste qui demande à parler. Si un panéliste oublie de passer le microphone quand il a fini, personne d'autre ne peut parler. Si la gestion du microphone partagé se passe mal, la table ronde ne fonctionnera pas comme prévu !

<!--
Management of mutexes can be incredibly tricky to get right, which is why so
many people are enthusiastic about channels. However, thanks to Rust's type
system and ownership rules, you can't get locking and unlocking wrong.
-->

La gestion des mutex peut être incroyablement délicate à réaliser correctement, c'est pourquoi tant de gens sont enthousiastes à propos des canaux. Cependant, grâce au système de types de Rust et aux règles de possession, vous ne pouvez pas vous tromper dans le verrouillage et le déverrouillage.

<!--
#### The API of `Mutex<T>`
-->

#### L'API de `Mutex<T>`

<!--
As an example of how to use a mutex, let's start by using a mutex in a
single-threaded context, as shown in Listing 16-12.
-->

Comme exemple d'utilisation d'un mutex, commençons par utiliser un mutex dans un contexte mono-thread, comme montré dans l'encart 16-12.

<Listing number="16-12" file-name="src/main.rs" caption="Explorer l'API de `Mutex<T>` dans un contexte mono-thread par simplicité">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

<!--
As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so that it can't do any work
until it's our turn to have the lock.
-->

Comme avec beaucoup de types, nous créons un `Mutex<T>` en utilisant la fonction associée `new`. Pour accéder aux données à l'intérieur du mutex, nous utilisons la méthode `lock` pour acquérir le verrou. Cet appel bloquera le thread courant pour qu'il ne puisse faire aucun travail jusqu'à ce que ce soit notre tour d'avoir le verrou.

<!--
The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we've chosen to
`unwrap` and have this thread panic if we're in that situation.
-->

L'appel à `lock` échouerait si un autre thread détenant le verrou a paniqué. Dans ce cas, personne ne pourrait jamais obtenir le verrou, donc nous avons choisi d'utiliser `unwrap` et de faire paniquer ce thread si nous sommes dans cette situation.

<!--
After we've acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using the value in `m`. The type of `m` is
`Mutex<i32>`, not `i32`, so we _must_ call `lock` to be able to use the `i32`
value. We can't forget; the type system won't let us access the inner `i32`
otherwise.
-->

Après avoir acquis le verrou, nous pouvons traiter la valeur de retour, nommée `num` dans ce cas, comme une référence mutable vers les données à l'intérieur. Le système de types garantit que nous acquérons un verrou avant d'utiliser la valeur dans `m`. Le type de `m` est `Mutex<i32>`, pas `i32`, donc nous _devons_ appeler `lock` pour pouvoir utiliser la valeur `i32`. Nous ne pouvons pas oublier ; le système de types ne nous laissera pas accéder au `i32` intérieur autrement.

<!--
The call to `lock` returns a type called `MutexGuard`, wrapped in a
`LockResult` that we handled with the call to `unwrap`. The `MutexGuard` type
implements `Deref` to point at our inner data; the type also has a `Drop`
implementation that releases the lock automatically when a `MutexGuard` goes
out of scope, which happens at the end of the inner scope. As a result, we
don't risk forgetting to release the lock and blocking the mutex from being
used by other threads because the lock release happens automatically.
-->

L'appel à `lock` retourne un type appelé `MutexGuard`, enveloppé dans un `LockResult` que nous avons géré avec l'appel à `unwrap`. Le type `MutexGuard` implémente `Deref` pour pointer vers nos données internes ; le type a aussi une implémentation de `Drop` qui libère le verrou automatiquement quand un `MutexGuard` sort de la portée, ce qui se produit à la fin de la portée intérieure. En conséquence, nous ne risquons pas d'oublier de libérer le verrou et de bloquer le mutex pour les autres threads car la libération du verrou se fait automatiquement.

<!--
After dropping the lock, we can print the mutex value and see that we were able
to change the inner `i32` to `6`.
-->

Après avoir libéré le verrou, nous pouvons afficher la valeur du mutex et voir que nous avons pu changer le `i32` intérieur à `6`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="sharing-a-mutext-between-multiple-threads"></a>

<!--
#### Shared Access to `Mutex<T>`
-->

#### Accès partagé à `Mutex<T>`

<!--
Now let's try to share a value between multiple threads using `Mutex<T>`. We'll
spin up 10 threads and have them each increment a counter value by 1, so the
counter goes from 0 to 10. The example in Listing 16-13 will have a compiler
error, and we'll use that error to learn more about using `Mutex<T>` and how
Rust helps us use it correctly.
-->

Maintenant, essayons de partager une valeur entre plusieurs threads en utilisant `Mutex<T>`. Nous allons lancer 10 threads et faire en sorte que chacun incrémente un compteur de 1, de sorte que le compteur passe de 0 à 10. L'exemple de l'encart 16-13 aura une erreur de compilation, et nous utiliserons cette erreur pour en apprendre davantage sur l'utilisation de `Mutex<T>` et comment Rust nous aide à l'utiliser correctement.

<Listing number="16-13" file-name="src/main.rs" caption="Dix threads, chacun incrémentant un compteur gardé par un `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

<!--
We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we did
in Listing 16-12. Next, we create 10 threads by iterating over a range of
numbers. We use `thread::spawn` and give all the threads the same closure: one
that moves the counter into the thread, acquires a lock on the `Mutex<T>` by
calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so that another thread can acquire it.
-->

Nous créons une variable `counter` pour contenir un `i32` dans un `Mutex<T>`, comme nous l'avons fait dans l'encart 16-12. Ensuite, nous créons 10 threads en itérant sur une plage de nombres. Nous utilisons `thread::spawn` et donnons à tous les threads la même closure : une qui déplace le compteur dans le thread, acquiert un verrou sur le `Mutex<T>` en appelant la méthode `lock`, puis ajoute 1 à la valeur dans le mutex. Quand un thread finit d'exécuter sa closure, `num` sortira de la portée et libérera le verrou pour qu'un autre thread puisse l'acquérir.

<!--
In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each handle to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.
-->

Dans le thread principal, nous collectons tous les handles de jointure. Ensuite, comme nous l'avons fait dans l'encart 16-2, nous appelons `join` sur chaque handle pour nous assurer que tous les threads finissent. À ce stade, le thread principal acquerra le verrou et affichera le résultat de ce programme.

<!--
We hinted that this example wouldn't compile. Now let's find out why!
-->

Nous avons suggéré que cet exemple ne compilerait pas. Maintenant, découvrons pourquoi !

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

<!--
The error message states that the `counter` value was moved in the previous
iteration of the loop. Rust is telling us that we can't move the ownership of
lock `counter` into multiple threads. Let's fix the compiler error with the
multiple-ownership method we discussed in Chapter 15.
-->

Le message d'erreur indique que la valeur `counter` a été déplacée dans l'itération précédente de la boucle. Rust nous dit que nous ne pouvons pas déplacer la possession du verrou `counter` dans plusieurs threads. Corrigeons l'erreur de compilation avec la méthode de possession multiple dont nous avons discuté au chapitre 15.

<!--
#### Multiple Ownership with Multiple Threads
-->

#### Possession multiple avec plusieurs threads

<!--
In Chapter 15, we gave a value to multiple owners by using the smart pointer
`Rc<T>` to create a reference-counted value. Let's do the same here and see
what happens. We'll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread.
-->

Au chapitre 15, nous avons donné une valeur à plusieurs propriétaires en utilisant le pointeur intelligent `Rc<T>` pour créer une valeur à comptage de références. Faisons la même chose ici et voyons ce qui se passe. Nous enveloppons le `Mutex<T>` dans un `Rc<T>` dans l'encart 16-14 et clonons le `Rc<T>` avant de transférer la possession au thread.

<Listing number="16-14" file-name="src/main.rs" caption="Tentative d'utilisation de `Rc<T>` pour permettre à plusieurs threads de posséder le `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

<!--
Once again, we compile and get... different errors! The compiler is teaching us
a lot:
-->

Encore une fois, nous compilons et obtenons... des erreurs différentes ! Le compilateur nous en apprend beaucoup :

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

<!--
Wow, that error message is very wordy! Here's the important part to focus on:
`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. The compiler is
also telling us the reason why: `` the trait `Send` is not implemented for
`Rc<Mutex<i32>>` ``. We'll talk about `Send` in the next section: It's one of
the traits that ensures that the types we use with threads are meant for use in
concurrent situations.
-->

Le message d'erreur est très verbeux ! Voici la partie importante sur laquelle se concentrer : `` `Rc<Mutex<i32>>` cannot be sent between threads safely `` (ne peut pas être envoyé entre les threads de manière sûre). Le compilateur nous dit aussi pourquoi : `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. Nous parlerons de `Send` dans la prochaine section : c'est l'un des traits qui garantit que les types que nous utilisons avec les threads sont destinés à être utilisés dans des situations concurrentes.

<!--
Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn't use any
concurrency primitives to make sure that changes to the count can't be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we're done
with it. What we need is a type that is exactly like `Rc<T>`, but that makes
changes to the reference count in a thread-safe way.
-->

Malheureusement, `Rc<T>` n'est pas sûr à partager entre les threads. Quand `Rc<T>` gère le compteur de références, il ajoute au compteur pour chaque appel à `clone` et soustrait du compteur quand chaque clone est libéré. Mais il n'utilise aucune primitive de concurrence pour s'assurer que les modifications du compteur ne peuvent pas être interrompues par un autre thread. Cela pourrait mener à des compteurs erronés -- des bogues subtils qui pourraient à leur tour provoquer des fuites de mémoire ou une valeur libérée avant que nous en ayons fini avec elle. Ce dont nous avons besoin est un type exactement comme `Rc<T>`, mais qui modifie le compteur de références de manière sûre pour les threads.

<!--
#### Atomic Reference Counting with `Arc<T>`
-->

#### Comptage de références atomique avec `Arc<T>`

<!--
Fortunately, `Arc<T>` _is_ a type like `Rc<T>` that is safe to use in
concurrent situations. The _a_ stands for _atomic_, meaning it's an _atomically
reference-counted_ type. Atomics are an additional kind of concurrency
primitive that we won't cover in detail here: See the standard library
documentation for [`std::sync::atomic`][atomic] ignore
--> for more
details. At this point, you just need to know that atomics work like primitive
types but are safe to share across threads.
-->

Heureusement, `Arc<T>` _est_ un type comme `Rc<T>` qui est sûr à utiliser dans des situations concurrentes. Le _a_ signifie _atomic_ (atomique), ce qui signifie que c'est un type à _comptage de références atomique_. Les atomiques sont un type supplémentaire de primitive de concurrence que nous ne couvrirons pas en détail ici : consultez la documentation de la bibliothèque standard pour [`std::sync::atomic`][atomic]<!--
ignore
--> pour plus de détails. À ce stade, vous avez juste besoin de savoir que les atomiques fonctionnent comme les types primitifs mais sont sûrs à partager entre les threads.

<!--
You might then wonder why all primitive types aren't atomic and why standard
library types aren't implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you're just performing operations on values within a
single thread, your code can run faster if it doesn't have to enforce the
guarantees atomics provide.
-->

Vous pourriez alors vous demander pourquoi tous les types primitifs ne sont pas atomiques et pourquoi les types de la bibliothèque standard ne sont pas implémentés pour utiliser `Arc<T>` par défaut. La raison est que la sécurité des threads a un coût en performance que vous ne voulez payer que quand vous en avez vraiment besoin. Si vous effectuez simplement des opérations sur des valeurs dans un seul thread, votre code peut s'exécuter plus rapidement s'il n'a pas à appliquer les garanties que fournissent les atomiques.

<!--
Let's return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run.
-->

Revenons à notre exemple : `Arc<T>` et `Rc<T>` ont la même API, donc nous corrigeons notre programme en changeant la ligne `use`, l'appel à `new`, et l'appel à `clone`. Le code de l'encart 16-15 compilera et s'exécutera enfin.

<Listing number="16-15" file-name="src/main.rs" caption="Utiliser un `Arc<T>` pour envelopper le `Mutex<T>` afin de pouvoir partager la possession entre plusieurs threads">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

<!--
This code will print the following:
-->

Ce code affichera ce qui suit :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
Result: 10
```

<!--
We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program's structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.
-->

Nous avons réussi ! Nous avons compté de 0 à 10, ce qui peut ne pas sembler très impressionnant, mais cela nous a beaucoup appris sur `Mutex<T>` et la sécurité des threads. Vous pourriez aussi utiliser la structure de ce programme pour effectuer des opérations plus complexes que simplement incrémenter un compteur. En utilisant cette stratégie, vous pouvez diviser un calcul en parties indépendantes, répartir ces parties entre les threads, puis utiliser un `Mutex<T>` pour que chaque thread mette à jour le résultat final avec sa partie.

<!--
Note that if you are doing simple numerical operations, there are types simpler
than `Mutex<T>` types provided by the [`std::sync::atomic` module of the
standard library][atomic] ignore
-->. These types provide safe, concurrent,
atomic access to primitive types. We chose to use `Mutex<T>` with a primitive
type for this example so that we could concentrate on how `Mutex<T>` works.
-->

Notez que si vous effectuez des opérations numériques simples, il existe des types plus simples que les types `Mutex<T>` fournis par le [module `std::sync::atomic` de la bibliothèque standard][atomic]<!--
ignore
-->. Ces types fournissent un accès sûr, concurrent et atomique aux types primitifs. Nous avons choisi d'utiliser `Mutex<T>` avec un type primitif pour cet exemple afin de pouvoir nous concentrer sur le fonctionnement de `Mutex<T>`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="similarities-between-refcelltrct-and-mutextarct"></a>

<!--
### Comparing `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
-->

### Comparer `RefCell<T>`/`Rc<T>` et `Mutex<T>`/`Arc<T>`

<!--
You might have noticed that `counter` is immutable but that we could get a
mutable reference to the value inside it; this means `Mutex<T>` provides
interior mutability, as the `Cell` family does. In the same way we used
`RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we
use `Mutex<T>` to mutate contents inside an `Arc<T>`.
-->

Vous avez peut-être remarqué que `counter` est immuable mais que nous pouvions obtenir une référence mutable vers la valeur à l'intérieur ; cela signifie que `Mutex<T>` fournit la mutabilité intérieure, comme la famille `Cell`. De la même manière que nous avons utilisé `RefCell<T>` au chapitre 15 pour nous permettre de modifier le contenu à l'intérieur d'un `Rc<T>`, nous utilisons `Mutex<T>` pour modifier le contenu à l'intérieur d'un `Arc<T>`.

<!--
Another detail to note is that Rust can't protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall from Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating _deadlocks_. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you're interested in deadlocks, try creating a Rust
program that has a deadlock; then, research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.
-->

Un autre détail à noter est que Rust ne peut pas vous protéger de tous les types d'erreurs logiques quand vous utilisez `Mutex<T>`. Rappelez-vous du chapitre 15 que l'utilisation de `Rc<T>` comportait le risque de créer des cycles de références, où deux valeurs `Rc<T>` se réfèrent mutuellement, provoquant des fuites de mémoire. De même, `Mutex<T>` comporte le risque de créer des _interblocages_ (deadlocks). Ceux-ci se produisent quand une opération a besoin de verrouiller deux ressources et que deux threads ont chacun acquis l'un des verrous, les faisant s'attendre mutuellement pour toujours. Si vous êtes intéressé par les interblocages, essayez de créer un programme Rust qui a un interblocage ; puis, recherchez les stratégies d'atténuation des interblocages pour les mutex dans n'importe quel langage et essayez de les implémenter en Rust. La documentation de l'API de la bibliothèque standard pour `Mutex<T>` et `MutexGuard` offre des informations utiles.

<!--
We'll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.
-->

Nous terminerons ce chapitre en parlant des traits `Send` et `Sync` et de la façon dont nous pouvons les utiliser avec des types personnalisés.

[atomic]: ../std/sync/atomic/index.html
