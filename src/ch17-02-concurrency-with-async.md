<!--
Old headings. Do not remove or links may break.
-->

<a id="concurrency-with-async"></a>

<!--
## Applying Concurrency with Async
-->

## Appliquer la concurrence avec l'async

<!--
In this section, we'll apply async to some of the same concurrency challenges
we tackled with threads in Chapter 16. Because we already talked about a lot of
the key ideas there, in this section we'll focus on what's different between
threads and futures.
-->

Dans cette section, nous appliquerons l'async à certains des mêmes défis de concurrence que nous avons abordés avec les threads au chapitre 16. Comme nous avons déjà discuté de beaucoup d'idées clés là-bas, dans cette section nous nous concentrerons sur ce qui diffère entre les threads et les futures.

<!--
In many cases, the APIs for working with concurrency using async are very
similar to those for using threads. In other cases, they end up being quite
different. Even when the APIs _look_ similar between threads and async, they
often have different behavior—and they nearly always have different performance
characteristics.
-->

Dans de nombreux cas, les API pour travailler avec la concurrence en utilisant l'async sont très similaires à celles utilisant les threads. Dans d'autres cas, elles finissent par être assez différentes. Même quand les API _semblent_ similaires entre les threads et l'async, elles ont souvent un comportement différent — et elles ont presque toujours des caractéristiques de performance différentes.

<!--
Old headings. Do not remove or links may break.
-->

<a id="counting"></a>

<!--
### Creating a New Task with `spawn_task`
-->

### Créer une nouvelle tâche avec `spawn_task`

<!--
The first operation we tackled in the ["Creating a New Thread with
`spawn`"][thread-spawn] ignore
--> section in Chapter 16 was counting up on
two separate threads. Let's do the same using async. The `trpl` crate supplies
a `spawn_task` function that looks very similar to the `thread::spawn` API, and
a `sleep` function that is an async version of the `thread::sleep` API. We can
use these together to implement the counting example, as shown in Listing 17-6.
-->

La première opération que nous avons abordée dans la section [« Créer un nouveau thread avec `spawn` »][thread-spawn]<!--
ignore
--> du chapitre 16 était de compter sur deux threads séparés. Faisons la même chose en utilisant l'async. Le crate `trpl` fournit une fonction `spawn_task` qui ressemble beaucoup à l'API `thread::spawn`, et une fonction `sleep` qui est une version async de l'API `thread::sleep`. Nous pouvons les utiliser ensemble pour implémenter l'exemple de comptage, comme montré dans l'encart 17-6.

<Listing number="17-6" caption="Création d'une nouvelle tâche pour afficher une chose pendant que la tâche principale affiche autre chose" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

<!--
As our starting point, we set up our `main` function with `trpl::block_on` so
that our top-level function can be async.
-->

Comme point de départ, nous configurons notre fonction `main` avec `trpl::block_on` pour que notre fonction de niveau supérieur puisse être async.

<!--
> Note: From this point forward in the chapter, every example will include this
> exact same wrapping code with `trpl::block_on` in `main`, so we'll often skip it
> just as we do with `main`. Remember to include it in your code!
-->

> Remarque : à partir de maintenant dans le chapitre, chaque exemple inclura ce même code d'enveloppe avec `trpl::block_on` dans `main`, donc nous l'omettrons souvent comme nous le faisons avec `main`. N'oubliez pas de l'inclure dans votre code !

<!--
Then we write two loops within that block, each containing a `trpl::sleep`
call, which waits for half a second (500 milliseconds) before sending the next
message. We put one loop in the body of a `trpl::spawn_task` and the other in a
top-level `for` loop. We also add an `await` after the `sleep` calls.
-->

Ensuite, nous écrivons deux boucles dans ce bloc, chacune contenant un appel à `trpl::sleep`, qui attend une demi-seconde (500 millisecondes) avant d'envoyer le message suivant. Nous mettons une boucle dans le corps d'un `trpl::spawn_task` et l'autre dans une boucle `for` de niveau supérieur. Nous ajoutons aussi un `await` après les appels à `sleep`.

<!--
This code behaves similarly to the thread-based implementation—including the
fact that you may see the messages appear in a different order in your own
terminal when you run it:
-->

Ce code se comporte de manière similaire à l'implémentation basée sur les threads — y compris le fait que vous pourriez voir les messages apparaître dans un ordre différent dans votre propre terminal quand vous l'exécutez :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

<!--
This version stops as soon as the `for` loop in the body of the main async
block finishes, because the task spawned by `spawn_task` is shut down when the
`main` function ends. If you want it to run all the way to the task's
completion, you will need to use a join handle to wait for the first task to
complete. With threads, we used the `join` method to "block" until the thread
was done running. In Listing 17-7, we can use `await` to do the same thing,
because the task handle itself is a future. Its `Output` type is a `Result`, so
we also unwrap it after awaiting it.
-->

Cette version s'arrête dès que la boucle `for` dans le corps du bloc async principal se termine, car la tâche lancée par `spawn_task` est arrêtée quand la fonction `main` se termine. Si vous voulez qu'elle s'exécute jusqu'à la fin de la tâche, vous devrez utiliser un handle de jointure pour attendre que la première tâche se termine. Avec les threads, nous utilisions la méthode `join` pour « bloquer » jusqu'à ce que le thread ait fini de s'exécuter. Dans l'encart 17-7, nous pouvons utiliser `await` pour faire la même chose, car le handle de tâche est lui-même une future. Son type `Output` est un `Result`, donc nous le déballons aussi après l'avoir attendu.

<Listing number="17-7" caption="Utiliser `await` avec un handle de jointure pour exécuter une tâche jusqu'à son achèvement" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

<!--
This updated version runs until _both_ loops finish:
-->

Cette version mise à jour s'exécute jusqu'à ce que _les deux_ boucles soient terminées :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

<!--
So far, it looks like async and threads give us similar outcomes, just with
different syntax: using `await` instead of calling `join` on the join handle,
and awaiting the `sleep` calls.
-->

Jusqu'ici, il semble que l'async et les threads nous donnent des résultats similaires, juste avec une syntaxe différente : utiliser `await` au lieu d'appeler `join` sur le handle de jointure, et attendre les appels à `sleep`.

<!--
The bigger difference is that we didn't need to spawn another operating system
thread to do this. In fact, we don't even need to spawn a task here. Because
async blocks compile to anonymous futures, we can put each loop in an async
block and have the runtime run them both to completion using the `trpl::join`
function.
-->

La plus grande différence est que nous n'avons pas eu besoin de créer un autre thread du système d'exploitation pour faire cela. En fait, nous n'avons même pas besoin de lancer une tâche ici. Comme les blocs async se compilent en futures anonymes, nous pouvons mettre chaque boucle dans un bloc async et laisser le runtime les exécuter toutes les deux jusqu'à leur achèvement en utilisant la fonction `trpl::join`.

<!--
In the ["Waiting for All Threads to Finish"][join-handles] ignore
-->
section in Chapter 16, we showed how to use the `join` method on the
`JoinHandle` type returned when you call `std::thread::spawn`. The `trpl::join`
function is similar, but for futures. When you give it two futures, it produces
a single new future whose output is a tuple containing the output of each
future you passed in once they _both_ complete. Thus, in Listing 17-8, we use
`trpl::join` to wait for both `fut1` and `fut2` to finish. We do _not_ await
`fut1` and `fut2` but instead the new future produced by `trpl::join`. We
ignore the output, because it's just a tuple containing two unit values.
-->

Dans la section [« Attendre que tous les threads se terminent »][join-handles]<!--
ignore
--> du chapitre 16, nous avons montré comment utiliser la méthode `join` sur le type `JoinHandle` retourné quand vous appelez `std::thread::spawn`. La fonction `trpl::join` est similaire, mais pour les futures. Quand vous lui donnez deux futures, elle produit une seule nouvelle future dont la sortie est un tuple contenant la sortie de chaque future que vous avez passée une fois qu'elles sont _toutes les deux_ terminées. Ainsi, dans l'encart 17-8, nous utilisons `trpl::join` pour attendre que `fut1` et `fut2` se terminent. Nous n'attendons _pas_ `fut1` et `fut2` mais plutôt la nouvelle future produite par `trpl::join`. Nous ignorons la sortie, car c'est juste un tuple contenant deux valeurs unit.

<Listing number="17-8" caption="Utiliser `trpl::join` pour attendre deux futures anonymes" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

<!--
When we run this, we see both futures run to completion:
-->

Quand nous exécutons cela, nous voyons les deux futures s'exécuter jusqu'à leur achèvement :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

<!--
Now, you'll see the exact same order every time, which is very different from
what we saw with threads and with `trpl::spawn_task` in Listing 17-7. That is
because the `trpl::join` function is _fair_, meaning it checks each future
equally often, alternating between them, and never lets one race ahead if the
other is ready. With threads, the operating system decides which thread to
check and how long to let it run. With async Rust, the runtime decides which
task to check. (In practice, the details get complicated because an async
runtime might use operating system threads under the hood as part of how it
manages concurrency, so guaranteeing fairness can be more work for a
runtime—but it's still possible!) Runtimes don't have to guarantee fairness for
any given operation, and they often offer different APIs to let you choose
whether or not you want fairness.
-->

Maintenant, vous verrez exactement le même ordre à chaque fois, ce qui est très différent de ce que nous avons vu avec les threads et avec `trpl::spawn_task` dans l'encart 17-7. C'est parce que la fonction `trpl::join` est _équitable_, ce qui signifie qu'elle vérifie chaque future aussi souvent, en alternant entre elles, et ne laisse jamais l'une prendre de l'avance si l'autre est prête. Avec les threads, le système d'exploitation décide quel thread vérifier et combien de temps le laisser s'exécuter. Avec le Rust async, c'est le runtime qui décide quelle tâche vérifier. (En pratique, les détails se compliquent car un runtime async peut utiliser des threads du système d'exploitation en coulisses dans le cadre de sa gestion de la concurrence, donc garantir l'équité peut demander plus de travail pour un runtime — mais c'est quand même possible !) Les runtimes n'ont pas à garantir l'équité pour une opération donnée, et ils offrent souvent différentes API pour vous permettre de choisir si vous voulez ou non l'équité.

<!--
Try some of these variations on awaiting the futures and see what they do:
-->

Essayez quelques-unes de ces variations sur l'attente des futures et voyez ce qu'elles font :

<!--
- Remove the async block from around either or both of the loops.
- Await each async block immediately after defining it.
- Wrap only the first loop in an async block, and await the resulting future
  after the body of second loop.
-->

- Retirez le bloc async autour de l'une ou des deux boucles.
- Attendez chaque bloc async immédiatement après l'avoir défini.
- N'encapsulez que la première boucle dans un bloc async, et attendez la future résultante après le corps de la deuxième boucle.

<!--
For an extra challenge, see if you can figure out what the output will be in
each case _before_ running the code!
-->

Pour un défi supplémentaire, essayez de deviner quelle sera la sortie dans chaque cas _avant_ d'exécuter le code !

<!--
Old headings. Do not remove or links may break.
-->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>

<!--
### Sending Data Between Two Tasks Using Message Passing
-->

### Envoyer des données entre deux tâches en utilisant le passage de messages

<!--
Sharing data between futures will also be familiar: we'll use message passing
again, but this time with async versions of the types and functions. We'll take
a slightly different path than we did in the ["Transfer Data Between Threads
with Message Passing"][message-passing-threads] ignore
--> section in
Chapter 16 to illustrate some of the key differences between thread-based and
futures-based concurrency. In Listing 17-9, we'll begin with just a single
async block—_not_ spawning a separate task as we spawned a separate thread.
-->

Le partage de données entre futures vous sera aussi familier : nous utiliserons à nouveau le passage de messages, mais cette fois avec des versions async des types et des fonctions. Nous prendrons un chemin légèrement différent de celui de la section [« Transférer des données entre threads avec le passage de messages »][message-passing-threads]<!--
ignore
--> du chapitre 16 pour illustrer certaines des différences clés entre la concurrence basée sur les threads et celle basée sur les futures. Dans l'encart 17-9, nous commencerons avec un seul bloc async — _sans_ lancer de tâche séparée comme nous avions lancé un thread séparé.

<Listing number="17-9" caption="Création d'un canal async et assignation des deux moitiés à `tx` et `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

<!--
Here, we use `trpl::channel`, an async version of the multiple-producer,
single-consumer channel API we used with threads back in Chapter 16. The async
version of the API is only a little different from the thread-based version: it
uses a mutable rather than an immutable receiver `rx`, and its `recv` method
produces a future we need to await rather than producing the value directly.
Now we can send messages from the sender to the receiver. Notice that we don't
have to spawn a separate thread or even a task; we merely need to await the
`rx.recv` call.
-->

Ici, nous utilisons `trpl::channel`, une version async de l'API de canal multiple-producteurs, unique-consommateur que nous avons utilisée avec les threads au chapitre 16. La version async de l'API est seulement un peu différente de la version basée sur les threads : elle utilise un récepteur `rx` mutable plutôt qu'immutable, et sa méthode `recv` produit une future que nous devons attendre au lieu de produire directement la valeur. Maintenant, nous pouvons envoyer des messages de l'émetteur au récepteur. Remarquez que nous n'avons pas besoin de lancer un thread séparé ni même une tâche ; nous avons simplement besoin d'attendre l'appel `rx.recv`.

<!--
The synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until it
receives a message. The `trpl::Receiver::recv` method does not, because it is
async. Instead of blocking, it hands control back to the runtime until either a
message is received or the send side of the channel closes. By contrast, we
don't await the `send` call, because it doesn't block. It doesn't need to,
because the channel we're sending it into is unbounded.
-->

La méthode synchrone `Receiver::recv` de `std::mpsc::channel` bloque jusqu'à recevoir un message. La méthode `trpl::Receiver::recv` ne le fait pas, car elle est async. Au lieu de bloquer, elle rend le contrôle au runtime jusqu'à ce qu'un message soit reçu ou que le côté émetteur du canal se ferme. En revanche, nous n'attendons pas l'appel `send`, car il ne bloque pas. Il n'a pas besoin de le faire, car le canal dans lequel nous envoyons est non borné.

<!--
> Note: Because all of this async code runs in an async block in a
> `trpl::block_on` call, everything within it can avoid blocking. However, the
> code _outside_ it will block on the `block_on` function returning. That's the
> whole point of the `trpl::block_on` function: it lets you _choose_ where to
> block on some set of async code, and thus where to transition between sync
> and async code.
-->

> Remarque : comme tout ce code async s'exécute dans un bloc async dans un appel à `trpl::block_on`, tout ce qui est à l'intérieur peut éviter de bloquer. Cependant, le code _à l'extérieur_ bloquera en attendant que la fonction `block_on` retourne. C'est tout l'intérêt de la fonction `trpl::block_on` : elle vous permet de _choisir_ où bloquer sur un ensemble de code async, et donc où faire la transition entre le code sync et async.

<!--
Notice two things about this example. First, the message will arrive right
away. Second, although we use a future here, there's no concurrency yet.
Everything in the listing happens in sequence, just as it would if there were
no futures involved.
-->

Remarquez deux choses à propos de cet exemple. Premièrement, le message arrivera immédiatement. Deuxièmement, bien que nous utilisions une future ici, il n'y a pas encore de concurrence. Tout dans le listing se passe en séquence, comme ce serait le cas s'il n'y avait pas de futures impliquées.

<!--
Let's address the first part by sending a series of messages and sleeping in
between them, as shown in Listing 17-10.
-->

Abordons la première partie en envoyant une série de messages et en dormant entre eux, comme montré dans l'encart 17-10.

<!--
We cannot test this one because it never stops!
-->

<Listing number="17-10" caption="Envoyer et recevoir plusieurs messages sur le canal async et dormir avec un `await` entre chaque message" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

<!--
In addition to sending the messages, we need to receive them. In this case,
because we know how many messages are coming in, we could do that manually by
calling `rx.recv().await` four times. In the real world, though, we'll generally
be waiting on some _unknown_ number of messages, so we need to keep waiting
until we determine that there are no more messages.
-->

En plus d'envoyer les messages, nous devons les recevoir. Dans ce cas, comme nous savons combien de messages arrivent, nous pourrions le faire manuellement en appelant `rx.recv().await` quatre fois. Dans le monde réel, cependant, nous attendrons généralement un nombre _inconnu_ de messages, donc nous devons continuer à attendre jusqu'à ce que nous déterminions qu'il n'y a plus de messages.

<!--
In Listing 16-10, we used a `for` loop to process all the items received from a
synchronous channel. Rust doesn't yet have a way to use a `for` loop with an
_asynchronously produced_ series of items, however, so we need to use a loop we
haven't seen before: the `while let` conditional loop. This is the loop version
of the `if let` construct we saw back in the ["Concise Control Flow with `if
let` and `let...else`"][if-let] ignore
--> section in Chapter 6. The loop
will continue executing as long as the pattern it specifies continues to match
the value.
-->

Dans l'encart 16-10, nous avons utilisé une boucle `for` pour traiter tous les éléments reçus d'un canal synchrone. Rust n'a pas encore de moyen d'utiliser une boucle `for` avec une série d'éléments _produits de manière asynchrone_, donc nous devons utiliser une boucle que nous n'avons pas vue avant : la boucle conditionnelle `while let`. C'est la version boucle de la construction `if let` que nous avons vue dans la section [« Flux de contrôle concis avec `if let` et `let...else` »][if-let]<!--
ignore
--> du chapitre 6. La boucle continuera à s'exécuter tant que le motif qu'elle spécifie continue de correspondre à la valeur.

<!--
The `rx.recv` call produces a future, which we await. The runtime will pause
the future until it is ready. Once a message arrives, the future will resolve
to `Some(message)` as many times as a message arrives. When the channel closes,
regardless of whether _any_ messages have arrived, the future will instead
resolve to `None` to indicate that there are no more values and thus we should
stop polling—that is, stop awaiting.
-->

L'appel `rx.recv` produit une future, que nous attendons. Le runtime mettra en pause la future jusqu'à ce qu'elle soit prête. Une fois qu'un message arrive, la future se résoudra en `Some(message)` autant de fois qu'un message arrive. Quand le canal se ferme, que des messages soient arrivés ou non, la future se résoudra plutôt en `None` pour indiquer qu'il n'y a plus de valeurs et que nous devons donc arrêter d'interroger — c'est-à-dire arrêter d'attendre.

<!--
The `while let` loop pulls all of this together. If the result of calling
`rx.recv().await` is `Some(message)`, we get access to the message and we can
use it in the loop body, just as we could with `if let`. If the result is
`None`, the loop ends. Every time the loop completes, it hits the await point
again, so the runtime pauses it again until another message arrives.
-->

La boucle `while let` rassemble tout cela. Si le résultat de l'appel à `rx.recv().await` est `Some(message)`, nous avons accès au message et pouvons l'utiliser dans le corps de la boucle, tout comme nous pourrions le faire avec `if let`. Si le résultat est `None`, la boucle se termine. Chaque fois que la boucle se complète, elle atteint à nouveau le point d'attente, donc le runtime la met à nouveau en pause jusqu'à ce qu'un autre message arrive.

<!--
The code now successfully sends and receives all of the messages.
Unfortunately, there are still a couple of problems. For one thing, the
messages do not arrive at half-second intervals. They arrive all at once, 2
seconds (2,000 milliseconds) after we start the program. For another, this
program also never exits! Instead, it waits forever for new messages. You will
need to shut it down using <kbd>ctrl</kbd>-<kbd>C</kbd>.
-->

Le code envoie et reçoit maintenant avec succès tous les messages. Malheureusement, il reste encore quelques problèmes. D'une part, les messages n'arrivent pas à intervalles d'une demi-seconde. Ils arrivent tous en même temps, 2 secondes (2 000 millisecondes) après le démarrage du programme. D'autre part, ce programme ne se termine jamais ! À la place, il attend indéfiniment de nouveaux messages. Vous devrez l'arrêter en utilisant <kbd>ctrl</kbd>-<kbd>C</kbd>.

<!--
#### Code Within One Async Block Executes Linearly
-->

#### Le code dans un seul bloc async s'exécute de manière linéaire

<!--
Let's start by examining why the messages come in all at once after the full
delay, rather than coming in with delays between each one. Within a given async
block, the order in which `await` keywords appear in the code is also the order
in which they're executed when the program runs.
-->

Commençons par examiner pourquoi les messages arrivent tous en même temps après le délai complet, plutôt que d'arriver avec des délais entre chacun. Dans un bloc async donné, l'ordre dans lequel les mots-clés `await` apparaissent dans le code est aussi l'ordre dans lequel ils sont exécutés quand le programme s'exécute.

<!--
There's only one async block in Listing 17-10, so everything in it runs
linearly. There's still no concurrency. All the `tx.send` calls happen,
interspersed with all of the `trpl::sleep` calls and their associated await
points. Only then does the `while let` loop get to go through any of the
`await` points on the `recv` calls.
-->

Il n'y a qu'un seul bloc async dans l'encart 17-10, donc tout s'exécute linéairement. Il n'y a toujours pas de concurrence. Tous les appels `tx.send` se produisent, entrecoupés de tous les appels `trpl::sleep` et de leurs points d'attente associés. Ce n'est qu'ensuite que la boucle `while let` passe par les points d'attente des appels `recv`.

<!--
To get the behavior we want, where the sleep delay happens between each
message, we need to put the `tx` and `rx` operations in their own async blocks,
as shown in Listing 17-11. Then the runtime can execute each of them separately
using `trpl::join`, just as in Listing 17-8. Once again, we await the result of
calling `trpl::join`, not the individual futures. If we awaited the individual
futures in sequence, we would just end up back in a sequential flow—exactly
what we're trying _not_ to do.
-->

Pour obtenir le comportement que nous voulons, où le délai de sommeil se produit entre chaque message, nous devons mettre les opérations `tx` et `rx` dans leurs propres blocs async, comme montré dans l'encart 17-11. Ensuite, le runtime peut exécuter chacun d'eux séparément en utilisant `trpl::join`, comme dans l'encart 17-8. Encore une fois, nous attendons le résultat de l'appel à `trpl::join`, pas les futures individuelles. Si nous attendions les futures individuelles en séquence, nous nous retrouverions dans un flux séquentiel — exactement ce que nous essayons de _ne pas_ faire.

<!--
We cannot test this one because it never stops!
-->

<Listing number="17-11" caption="Séparer `send` et `recv` dans leurs propres blocs `async` et attendre les futures de ces blocs" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

<!--
With the updated code in Listing 17-11, the messages get printed at
500-millisecond intervals, rather than all in a rush after 2 seconds.
-->

Avec le code mis à jour dans l'encart 17-11, les messages s'affichent à intervalles de 500 millisecondes, plutôt que tous en rafale après 2 secondes.

<!--
#### Moving Ownership Into an Async Block
-->

#### Déplacer la possession dans un bloc async

<!--
The program still never exits, though, because of the way the `while let` loop
interacts with `trpl::join`:
-->

Le programme ne se termine cependant toujours jamais, à cause de la façon dont la boucle `while let` interagit avec `trpl::join` :

<!--
- The future returned from `trpl::join` completes only once _both_ futures
  passed to it have completed.
- The `tx_fut` future completes once it finishes sleeping after sending the last
  message in `vals`.
- The `rx_fut` future won't complete until the `while let` loop ends.
- The `while let` loop won't end until awaiting `rx.recv` produces `None`.
- Awaiting `rx.recv` will return `None` only once the other end of the channel
  is closed.
- The channel will close only if we call `rx.close` or when the sender side,
  `tx`, is dropped.
- We don't call `rx.close` anywhere, and `tx` won't be dropped until the
  outermost async block passed to `trpl::block_on` ends.
- The block can't end because it is blocked on `trpl::join` completing, which
  takes us back to the top of this list.
-->

- La future retournée par `trpl::join` ne se termine que quand _les deux_ futures qui lui ont été passées sont terminées.
- La future `tx_fut` se termine une fois qu'elle a fini de dormir après l'envoi du dernier message dans `vals`.
- La future `rx_fut` ne se terminera pas tant que la boucle `while let` ne se terminera pas.
- La boucle `while let` ne se terminera pas tant que l'attente de `rx.recv` ne produira pas `None`.
- L'attente de `rx.recv` ne retournera `None` que quand l'autre extrémité du canal sera fermée.
- Le canal ne se fermera que si nous appelons `rx.close` ou quand le côté émetteur, `tx`, est droppé.
- Nous n'appelons `rx.close` nulle part, et `tx` ne sera pas droppé tant que le bloc async le plus externe passé à `trpl::block_on` ne se terminera pas.
- Le bloc ne peut pas se terminer car il est bloqué en attendant que `trpl::join` se termine, ce qui nous ramène en haut de cette liste.

<!--
Right now, the async block where we send the messages only _borrows_ `tx`
because sending a message doesn't require ownership, but if we could _move_
`tx` into that async block, it would be dropped once that block ends. In the
["Capturing References or Moving Ownership"][capture-or-move] ignore
-->
section in Chapter 13, you learned how to use the `move` keyword with closures,
and, as discussed in the ["Using `move` Closures with
Threads"][move-threads]<!--
ignore
--> section in Chapter 16, we often need to
move data into closures when working with threads. The same basic dynamics
apply to async blocks, so the `move` keyword works with async blocks just as it
does with closures.
-->

Actuellement, le bloc async où nous envoyons les messages ne fait qu'_emprunter_ `tx` car l'envoi d'un message ne nécessite pas la possession, mais si nous pouvions _déplacer_ `tx` dans ce bloc async, il serait droppé une fois que ce bloc se termine. Dans la section [« Capturer des références ou déplacer la possession »][capture-or-move]<!--
ignore
--> du chapitre 13, vous avez appris à utiliser le mot-clé `move` avec les closures, et comme discuté dans la section [« Utiliser les closures `move` avec les threads »][move-threads]<!--
ignore
--> du chapitre 16, nous devons souvent déplacer des données dans les closures quand nous travaillons avec les threads. Les mêmes dynamiques fondamentales s'appliquent aux blocs async, donc le mot-clé `move` fonctionne avec les blocs async comme il le fait avec les closures.

<!--
In Listing 17-12, we change the block used to send messages from `async` to
`async move`.
-->

Dans l'encart 17-12, nous changeons le bloc utilisé pour envoyer des messages de `async` à `async move`.

<Listing number="17-12" caption="Une révision du code de l'encart 17-11 qui se termine correctement une fois terminé" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

<!--
When we run _this_ version of the code, it shuts down gracefully after the last
message is sent and received. Next, let's see what would need to change to send
data from more than one future.
-->

Quand nous exécutons _cette_ version du code, il se termine proprement après l'envoi et la réception du dernier message. Voyons maintenant ce qui devrait changer pour envoyer des données depuis plus d'une future.

<!--
#### Joining a Number of Futures with the `join!` Macro
-->

#### Joindre un nombre de futures avec la macro `join!`

<!--
This async channel is also a multiple-producer channel, so we can call `clone`
on `tx` if we want to send messages from multiple futures, as shown in Listing
17-13.
-->

Ce canal async est aussi un canal à producteurs multiples, donc nous pouvons appeler `clone` sur `tx` si nous voulons envoyer des messages depuis plusieurs futures, comme montré dans l'encart 17-13.

<Listing number="17-13" caption="Utiliser plusieurs producteurs avec des blocs async" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

<!--
First, we clone `tx`, creating `tx1` outside the first async block. We move
`tx1` into that block just as we did before with `tx`. Then, later, we move the
original `tx` into a _new_ async block, where we send more messages on a
slightly slower delay. We happen to put this new async block after the async
block for receiving messages, but it could go before it just as well. The key is
the order in which the futures are awaited, not in which they're created.
-->

D'abord, nous clonons `tx`, créant `tx1` en dehors du premier bloc async. Nous déplaçons `tx1` dans ce bloc comme nous l'avons fait auparavant avec `tx`. Puis, plus tard, nous déplaçons le `tx` original dans un _nouveau_ bloc async, où nous envoyons plus de messages avec un délai légèrement plus lent. Il se trouve que nous mettons ce nouveau bloc async après le bloc async de réception des messages, mais il pourrait tout aussi bien être avant. Ce qui compte, c'est l'ordre dans lequel les futures sont attendues, pas l'ordre dans lequel elles sont créées.

<!--
Both of the async blocks for sending messages need to be `async move` blocks so
that both `tx` and `tx1` get dropped when those blocks finish. Otherwise, we'll
end up back in the same infinite loop we started out in.
-->

Les deux blocs async pour l'envoi de messages doivent être des blocs `async move` pour que `tx` et `tx1` soient tous les deux droppés quand ces blocs se terminent. Sinon, nous nous retrouverons dans la même boucle infinie qu'au départ.

<!--
Finally, we switch from `trpl::join` to `trpl::join!` to handle the additional
future: the `join!` macro awaits an arbitrary number of futures where we know
the number of futures at compile time. We'll discuss awaiting a collection of
an unknown number of futures later in this chapter.
-->

Enfin, nous passons de `trpl::join` à `trpl::join!` pour gérer la future supplémentaire : la macro `join!` attend un nombre arbitraire de futures dont nous connaissons le nombre au moment de la compilation. Nous discuterons de l'attente d'une collection d'un nombre inconnu de futures plus loin dans ce chapitre.

<!--
Now we see all the messages from both sending futures, and because the sending
futures use slightly different delays after sending, the messages are also
received at those different intervals:
-->

Maintenant nous voyons tous les messages des deux futures d'envoi, et comme les futures d'envoi utilisent des délais légèrement différents après l'envoi, les messages sont aussi reçus à ces différents intervalles :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

<!--
We've explored how to use message passing to send data between futures, how
code within an async block runs sequentially, how to move ownership into an
async block, and how to join multiple futures. Next, let's discuss how and why
to tell the runtime it can switch to another task.
-->

Nous avons exploré comment utiliser le passage de messages pour envoyer des données entre futures, comment le code dans un bloc async s'exécute séquentiellement, comment déplacer la possession dans un bloc async, et comment joindre plusieurs futures. Voyons maintenant comment et pourquoi indiquer au runtime qu'il peut passer à une autre tâche.

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads
