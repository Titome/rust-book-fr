<!--
## Putting It All Together: Futures, Tasks, and Threads
-->

## Tout assembler : futures, tâches et threads

<!--
As we saw in [Chapter 16][ch16] ignore
-->, threads provide one approach to
concurrency. We've seen another approach in this chapter: using async with
futures and streams. If you're wondering when to choose one method over the other,
the answer is: it depends! And in many cases, the choice isn't threads _or_
async but rather threads _and_ async.
-->

Comme nous l'avons vu au [chapitre 16][ch16]<!--
ignore
-->, les threads fournissent une approche de la concurrence. Nous avons vu une autre approche dans ce chapitre : utiliser l'async avec les futures et les streams. Si vous vous demandez quand choisir une méthode plutôt que l'autre, la réponse est : ça dépend ! Et dans de nombreux cas, le choix n'est pas entre les threads _ou_ l'async mais plutôt les threads _et_ l'async.

<!--
Many operating systems have supplied threading-based concurrency models for
decades now, and many programming languages support them as a result. However,
these models are not without their tradeoffs. On many operating systems, they
use a fair bit of memory for each thread. Threads are also only an option when
your operating system and hardware support them. Unlike mainstream desktop and
mobile computers, some embedded systems don't have an OS at all, so they also
don't have threads.
-->

De nombreux systèmes d'exploitation fournissent des modèles de concurrence basés sur les threads depuis des décennies maintenant, et de nombreux langages de programmation les supportent en conséquence. Cependant, ces modèles ne sont pas sans compromis. Sur de nombreux systèmes d'exploitation, ils utilisent pas mal de mémoire pour chaque thread. Les threads ne sont aussi une option que quand votre système d'exploitation et votre matériel les supportent. Contrairement aux ordinateurs de bureau et mobiles grand public, certains systèmes embarqués n'ont pas du tout de système d'exploitation, donc ils n'ont pas non plus de threads.

<!--
The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations don't require their own
threads. Instead, they can run on tasks, as when we used `trpl::spawn_task` to
kick off work from a synchronous function in the streams section. A task is
similar to a thread, but instead of being managed by the operating system, it's
managed by library-level code: the runtime.
-->

Le modèle async fournit un ensemble de compromis différent — et finalement complémentaire. Dans le modèle async, les opérations concurrentes ne nécessitent pas leurs propres threads. À la place, elles peuvent s'exécuter sur des tâches, comme quand nous avons utilisé `trpl::spawn_task` pour lancer du travail depuis une fonction synchrone dans la section sur les streams. Une tâche est similaire à un thread, mais au lieu d'être gérée par le système d'exploitation, elle est gérée par du code au niveau de la bibliothèque : le runtime.

<!--
There's a reason the APIs for spawning threads and spawning tasks are so
similar. Threads act as a boundary for sets of synchronous operations;
concurrency is possible _between_ threads. Tasks act as a boundary for sets of
_asynchronous_ operations; concurrency is possible both _between_ and _within_
tasks, because a task can switch between futures in its body. Finally, futures
are Rust's most granular unit of concurrency, and each future may represent a
tree of other futures. The runtime—specifically, its executor—manages tasks,
and tasks manage futures. In that regard, tasks are similar to lightweight,
runtime-managed threads with added capabilities that come from being managed by
a runtime instead of by the operating system.
-->

Il y a une raison pour laquelle les API pour lancer des threads et lancer des tâches sont si similaires. Les threads agissent comme une frontière pour des ensembles d'opérations synchrones ; la concurrence est possible _entre_ les threads. Les tâches agissent comme une frontière pour des ensembles d'opérations _asynchrones_ ; la concurrence est possible à la fois _entre_ et _au sein_ des tâches, car une tâche peut alterner entre les futures dans son corps. Enfin, les futures sont l'unité de concurrence la plus granulaire de Rust, et chaque future peut représenter un arbre d'autres futures. Le runtime — spécifiquement, son exécuteur — gère les tâches, et les tâches gèrent les futures. À cet égard, les tâches sont similaires à des threads légers gérés par le runtime avec des capacités supplémentaires qui viennent du fait d'être gérées par un runtime plutôt que par le système d'exploitation.

<!--
This doesn't mean that async tasks are always better than threads (or vice
versa). Concurrency with threads is in some ways a simpler programming model
than concurrency with `async`. That can be a strength or a weakness. Threads are
somewhat "fire and forget"; they have no native equivalent to a future, so they
simply run to completion without being interrupted except by the operating
system itself.
-->

Cela ne signifie pas que les tâches async sont toujours meilleures que les threads (ou vice versa). La concurrence avec les threads est à certains égards un modèle de programmation plus simple que la concurrence avec `async`. Cela peut être une force ou une faiblesse. Les threads sont quelque peu « lancer et oublier » ; ils n'ont pas d'équivalent natif à une future, donc ils s'exécutent simplement jusqu'à la fin sans être interrompus sauf par le système d'exploitation lui-même.

<!--
And it turns out that threads and tasks often work
very well together, because tasks can (at least in some runtimes) be moved
around between threads. In fact, under the hood, the runtime we've been
using—including the `spawn_blocking` and `spawn_task` functions—is multithreaded
by default! Many runtimes use an approach called _work stealing_ to
transparently move tasks around between threads, based on how the threads are
currently being utilized, to improve the system's overall performance. That
approach actually requires threads _and_ tasks, and therefore futures.
-->

Et il s'avère que les threads et les tâches fonctionnent souvent très bien ensemble, car les tâches peuvent (au moins dans certains runtimes) être déplacées entre les threads. En fait, en coulisses, le runtime que nous avons utilisé — y compris les fonctions `spawn_blocking` et `spawn_task` — est multithread par défaut ! Beaucoup de runtimes utilisent une approche appelée _vol de travail_ (_work stealing_) pour déplacer de manière transparente les tâches entre les threads, en fonction de la façon dont les threads sont actuellement utilisés, pour améliorer les performances globales du système. Cette approche nécessite en fait des threads _et_ des tâches, et donc des futures.

<!--
When thinking about which method to use when, consider these rules of thumb:
-->

Quand vous réfléchissez à quelle méthode utiliser et quand, considérez ces règles empiriques :

<!--
- If the work is _very parallelizable_ (that is, CPU-bound), such as processing
  a bunch of data where each part can be processed separately, threads are a
  better choice.
- If the work is _very concurrent_ (that is, I/O-bound), such as handling
  messages from a bunch of different sources that may come in at different
  intervals or different rates, async is a better choice.
-->

- Si le travail est _très parallélisable_ (c'est-à-dire limité par le CPU), comme traiter un ensemble de données où chaque partie peut être traitée séparément, les threads sont un meilleur choix.
- Si le travail est _très concurrent_ (c'est-à-dire limité par les E/S), comme gérer des messages provenant de nombreuses sources différentes qui peuvent arriver à différents intervalles ou à différentes fréquences, l'async est un meilleur choix.

<!--
And if you need both parallelism and concurrency, you don't have to choose
between threads and async. You can use them together freely, letting each
play the part it's best at. For example, Listing 17-25 shows a fairly common
example of this kind of mix in real-world Rust code.
-->

Et si vous avez besoin à la fois du parallélisme et de la concurrence, vous n'avez pas à choisir entre les threads et l'async. Vous pouvez les utiliser ensemble librement, en laissant chacun jouer le rôle dans lequel il excelle. Par exemple, l'encart 17-25 montre un exemple assez courant de ce type de mélange dans le code Rust du monde réel.

<Listing number="17-25" caption="Envoyer des messages avec du code bloquant dans un thread et attendre les messages dans un bloc async" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

<!--
We begin by creating an async channel, then spawning a thread that takes
ownership of the sender side of the channel using the `move` keyword. Within
the thread, we send the numbers 1 through 10, sleeping for a second between
each. Finally, we run a future created with an async block passed to
`trpl::block_on` just as we have throughout the chapter. In that future, we
await those messages, just as in the other message-passing examples we have
seen.
-->

Nous commençons par créer un canal async, puis lançons un thread qui prend possession du côté émetteur du canal en utilisant le mot-clé `move`. Dans le thread, nous envoyons les nombres de 1 à 10, en dormant une seconde entre chaque. Enfin, nous exécutons une future créée avec un bloc async passé à `trpl::block_on` comme nous l'avons fait tout au long du chapitre. Dans cette future, nous attendons ces messages, comme dans les autres exemples de passage de messages que nous avons vus.

<!--
To return to the scenario we opened the chapter with, imagine running a set of
video encoding tasks using a dedicated thread (because video encoding is
compute-bound) but notifying the UI that those operations are done with an
async channel. There are countless examples of these kinds of combinations in
real-world use cases.
-->

Pour revenir au scénario avec lequel nous avons ouvert le chapitre, imaginez exécuter un ensemble de tâches d'encodage vidéo en utilisant un thread dédié (parce que l'encodage vidéo est limité par le calcul) mais notifier l'interface utilisateur que ces opérations sont terminées avec un canal async. Il existe d'innombrables exemples de ce genre de combinaisons dans les cas d'utilisation du monde réel.

<!--
## Summary
-->

## Résumé

<!--
This isn't the last you'll see of concurrency in this book. The project in
[Chapter 21][ch21] ignore
--> will apply these concepts in a more realistic
situation than the simpler examples discussed here and compare problem-solving
with threading versus tasks and futures more directly.
-->

Ce n'est pas la dernière fois que vous verrez la concurrence dans ce livre. Le projet du [chapitre 21][ch21]<!--
ignore
--> appliquera ces concepts dans une situation plus réaliste que les exemples simples discutés ici et comparera plus directement la résolution de problèmes avec les threads versus les tâches et les futures.

<!--
No matter which of these approaches you choose, Rust gives you the tools you
need to write safe, fast, concurrent code—whether for a high-throughput web
server or an embedded operating system.
-->

Quelle que soit l'approche que vous choisissez, Rust vous donne les outils dont vous avez besoin pour écrire du code concurrent sûr et rapide — que ce soit pour un serveur web à haut débit ou un système d'exploitation embarqué.

<!--
Next, we'll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we'll discuss how Rust's idioms
relate to those you might be familiar with from object-oriented programming.
-->

Ensuite, nous parlerons des façons idiomatiques de modéliser les problèmes et de structurer les solutions à mesure que vos programmes Rust grandissent. De plus, nous discuterons de la relation entre les idiomes de Rust et ceux que vous pourriez connaître de la programmation orientée objet.

[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
[ch21]: ch21-00-final-project-a-web-server.html
