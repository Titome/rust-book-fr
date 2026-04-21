<!--
## Using Threads to Run Code Simultaneously
-->

## Utiliser les threads pour exécuter du code simultanément

<!--
In most current operating systems, an executed program's code is run in a
_process_, and the operating system will manage multiple processes at once.
Within a program, you can also have independent parts that run simultaneously.
The features that run these independent parts are called _threads_. For
example, a web server could have multiple threads so that it can respond to
more than one request at the same time.
-->

Dans la plupart des systèmes d'exploitation actuels, le code d'un programme exécuté s'exécute dans un _processus_, et le système d'exploitation gère plusieurs processus en même temps. Au sein d'un programme, vous pouvez aussi avoir des parties indépendantes qui s'exécutent simultanément. Les fonctionnalités qui exécutent ces parties indépendantes sont appelées _threads_. Par exemple, un serveur web pourrait avoir plusieurs threads pour pouvoir répondre à plus d'une requête en même temps.

<!--
Splitting the computation in your program into multiple threads to run multiple
tasks at the same time can improve performance, but it also adds complexity.
Because threads can run simultaneously, there's no inherent guarantee about the
order in which parts of your code on different threads will run. This can lead
to problems, such as:

- Race conditions, in which threads are accessing data or resources in an
  inconsistent order
- Deadlocks, in which two threads are waiting for each other, preventing both
  threads from continuing
- Bugs that only happen in certain situations and are hard to reproduce and fix
  reliably
-->

Diviser le calcul de votre programme en plusieurs threads pour exécuter plusieurs tâches en même temps peut améliorer les performances, mais cela ajoute aussi de la complexité. Comme les threads peuvent s'exécuter simultanément, il n'y a aucune garantie inhérente sur l'ordre dans lequel les parties de votre code sur différents threads s'exécuteront. Cela peut mener à des problèmes, tels que :

- Les conditions de course (race conditions), dans lesquelles les threads accèdent à des données ou des ressources dans un ordre incohérent
- Les interblocages (deadlocks), dans lesquels deux threads s'attendent mutuellement, empêchant les deux threads de continuer
- Les bogues qui ne se produisent que dans certaines situations et sont difficiles à reproduire et à corriger de manière fiable

<!--
Rust attempts to mitigate the negative effects of using threads, but
programming in a multithreaded context still takes careful thought and requires
a code structure that is different from that in programs running in a single
thread.
-->

Rust tente d'atténuer les effets négatifs de l'utilisation des threads, mais programmer dans un contexte multi-thread nécessite toujours une réflexion soigneuse et une structure de code différente de celle des programmes s'exécutant dans un seul thread.

<!--
Programming languages implement threads in a few different ways, and many
operating systems provide an API the programming language can call for creating
new threads. The Rust standard library uses a _1:1_ model of thread
implementation, whereby a program uses one operating system thread per one
language thread. There are crates that implement other models of threading that
make different trade-offs to the 1:1 model. (Rust's async system, which we will
see in the next chapter, provides another approach to concurrency as well.)
-->

Les langages de programmation implémentent les threads de différentes manières, et de nombreux systèmes d'exploitation fournissent une API que le langage de programmation peut appeler pour créer de nouveaux threads. La bibliothèque standard de Rust utilise un modèle d'implémentation de threads _1:1_, dans lequel un programme utilise un thread du système d'exploitation par thread du langage. Il existe des crates qui implémentent d'autres modèles de threading avec des compromis différents du modèle 1:1. (Le système async de Rust, que nous verrons dans le prochain chapitre, fournit aussi une autre approche de la concurrence.)

<!--
### Creating a New Thread with `spawn`
-->

### Créer un nouveau thread avec `spawn`

<!--
To create a new thread, we call the `thread::spawn` function and pass it a
closure (we talked about closures in Chapter 13) containing the code we want to
run in the new thread. The example in Listing 16-1 prints some text from a main
thread and other text from a new thread.
-->

Pour créer un nouveau thread, nous appelons la fonction `thread::spawn` et lui passons une closure (nous avons parlé des closures au chapitre 13) contenant le code que nous voulons exécuter dans le nouveau thread. L'exemple de l'encart 16-1 affiche du texte depuis le thread principal et d'autre texte depuis un nouveau thread.

<Listing number="16-1" file-name="src/main.rs" caption="Créer un nouveau thread pour afficher une chose pendant que le thread principal affiche autre chose">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

<!--
Note that when the main thread of a Rust program completes, all spawned threads
are shut down, whether or not they have finished running. The output from this
program might be a little different every time, but it will look similar to the
following:
-->

Notez que lorsque le thread principal d'un programme Rust se termine, tous les threads créés sont arrêtés, qu'ils aient fini ou non de s'exécuter. La sortie de ce programme peut être légèrement différente à chaque fois, mais elle ressemblera à ceci :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

<!--
The calls to `thread::sleep` force a thread to stop its execution for a short
duration, allowing a different thread to run. The threads will probably take
turns, but that isn't guaranteed: It depends on how your operating system
schedules the threads. In this run, the main thread printed first, even though
the print statement from the spawned thread appears first in the code. And even
though we told the spawned thread to print until `i` is `9`, it only got to `5`
before the main thread shut down.
-->

Les appels à `thread::sleep` forcent un thread à arrêter son exécution pendant une courte durée, permettant à un autre thread de s'exécuter. Les threads alterneront probablement, mais ce n'est pas garanti : cela dépend de la façon dont votre système d'exploitation planifie les threads. Dans cette exécution, le thread principal a affiché en premier, même si l'instruction d'affichage du thread créé apparaît en premier dans le code. Et même si nous avons demandé au thread créé d'afficher jusqu'à ce que `i` soit `9`, il n'a atteint que `5` avant que le thread principal ne s'arrête.

<!--
If you run this code and only see output from the main thread, or don't see any
overlap, try increasing the numbers in the ranges to create more opportunities
for the operating system to switch between the threads.
-->

Si vous exécutez ce code et ne voyez que la sortie du thread principal, ou ne voyez aucun chevauchement, essayez d'augmenter les nombres dans les plages pour créer plus d'opportunités pour le système d'exploitation de basculer entre les threads.

<!--
Old headings. Do not remove or links may break.
-->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>

<!--
### Waiting for All Threads to Finish
-->

### Attendre que tous les threads aient fini

<!--
The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time due to the main thread ending, but because there is no guarantee on
the order in which threads run, we also can't guarantee that the spawned thread
will get to run at all!
-->

Le code de l'encart 16-1 non seulement arrête prématurément le thread créé la plupart du temps en raison de la fin du thread principal, mais comme il n'y a aucune garantie sur l'ordre dans lequel les threads s'exécutent, nous ne pouvons pas non plus garantir que le thread créé s'exécutera du tout !

<!--
We can fix the problem of the spawned thread not running or of it ending
prematurely by saving the return value of `thread::spawn` in a variable. The
return type of `thread::spawn` is `JoinHandle<T>`. A `JoinHandle<T>` is an
owned value that, when we call the `join` method on it, will wait for its
thread to finish. Listing 16-2 shows how to use the `JoinHandle<T>` of the
thread we created in Listing 16-1 and how to call `join` to make sure the
spawned thread finishes before `main` exits.
-->

Nous pouvons corriger le problème du thread créé qui ne s'exécute pas ou qui se termine prématurément en sauvegardant la valeur de retour de `thread::spawn` dans une variable. Le type de retour de `thread::spawn` est `JoinHandle<T>`. Un `JoinHandle<T>` est une valeur possédée qui, quand nous appelons la méthode `join` dessus, attendra que son thread finisse. L'encart 16-2 montre comment utiliser le `JoinHandle<T>` du thread que nous avons créé dans l'encart 16-1 et comment appeler `join` pour s'assurer que le thread créé finisse avant que `main` ne se termine.

<Listing number="16-2" file-name="src/main.rs" caption="Sauvegarder un `JoinHandle<T>` de `thread::spawn` pour garantir que le thread s'exécute jusqu'à la fin">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

<!--
Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. _Blocking_ a thread means that
thread is prevented from performing work or exiting. Because we've put the call
to `join` after the main thread's `for` loop, running Listing 16-2 should
produce output similar to this:
-->

Appeler `join` sur le handle bloque le thread actuellement en cours d'exécution jusqu'à ce que le thread représenté par le handle se termine. _Bloquer_ un thread signifie que ce thread est empêché de travailler ou de se terminer. Comme nous avons placé l'appel à `join` après la boucle `for` du thread principal, l'exécution de l'encart 16-2 devrait produire une sortie similaire à ceci :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

<!--
The two threads continue alternating, but the main thread waits because of the
call to `handle.join()` and does not end until the spawned thread is finished.
-->

Les deux threads continuent d'alterner, mais le thread principal attend à cause de l'appel à `handle.join()` et ne se termine pas tant que le thread créé n'a pas fini.

<!--
But let's see what happens when we instead move `handle.join()` before the
`for` loop in `main`, like this:
-->

Mais voyons ce qui se passe quand nous déplaçons `handle.join()` avant la boucle `for` dans `main`, comme ceci :

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

<!--
The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won't be interleaved anymore, as shown here:
-->

Le thread principal attendra que le thread créé finisse puis exécutera sa boucle `for`, donc la sortie ne sera plus entrelacée, comme montré ici :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

<!--
Small details, such as where `join` is called, can affect whether or not your
threads run at the same time.
-->

De petits détails, comme l'endroit où `join` est appelé, peuvent affecter le fait que vos threads s'exécutent ou non en même temps.

<!--
### Using `move` Closures with Threads
-->

### Utiliser les closures `move` avec les threads

<!--
We'll often use the `move` keyword with closures passed to `thread::spawn`
because the closure will then take ownership of the values it uses from the
environment, thus transferring ownership of those values from one thread to
another. In ["Capturing References or Moving Ownership"][capture] ignore
--> in Chapter 13, we discussed `move` in the context of closures. Now we'll
concentrate more on the interaction between `move` and `thread::spawn`.
-->

Nous utiliserons souvent le mot-clé `move` avec les closures passées à `thread::spawn` car la closure prendra alors la possession des valeurs qu'elle utilise de l'environnement, transférant ainsi la possession de ces valeurs d'un thread à un autre. Dans ["Capturer des références ou transférer la possession"][capture]<!--
ignore
--> au chapitre 13, nous avons discuté de `move` dans le contexte des closures. Maintenant, nous allons nous concentrer davantage sur l'interaction entre `move` et `thread::spawn`.

<!--
Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: We're not using any data from the main thread in the spawned
thread's code. To use data from the main thread in the spawned thread, the
spawned thread's closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won't work yet, as you'll see in a moment.
-->

Remarquez dans l'encart 16-1 que la closure que nous passons à `thread::spawn` ne prend aucun argument : nous n'utilisons aucune donnée du thread principal dans le code du thread créé. Pour utiliser des données du thread principal dans le thread créé, la closure du thread créé doit capturer les valeurs dont elle a besoin. L'encart 16-3 montre une tentative de créer un vecteur dans le thread principal et de l'utiliser dans le thread créé. Cependant, cela ne fonctionnera pas encore, comme vous le verrez dans un instant.

<Listing number="16-3" file-name="src/main.rs" caption="Tentative d'utilisation d'un vecteur créé par le thread principal dans un autre thread">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

<!--
The closure uses `v`, so it will capture `v` and make it part of the closure's
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:
-->

La closure utilise `v`, donc elle va capturer `v` et en faire partie de l'environnement de la closure. Comme `thread::spawn` exécute cette closure dans un nouveau thread, nous devrions pouvoir accéder à `v` dans ce nouveau thread. Mais quand nous compilons cet exemple, nous obtenons l'erreur suivante :

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

<!--
Rust _infers_ how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there's a problem: Rust can't
tell how long the spawned thread will run, so it doesn't know whether the
reference to `v` will always be valid.
-->

Rust _infère_ comment capturer `v`, et comme `println!` n'a besoin que d'une référence vers `v`, la closure essaie d'emprunter `v`. Cependant, il y a un problème : Rust ne peut pas déterminer combien de temps le thread créé s'exécutera, donc il ne sait pas si la référence vers `v` sera toujours valide.

<!--
Listing 16-4 provides a scenario that's more likely to have a reference to `v`
that won't be valid.
-->

L'encart 16-4 fournit un scénario qui a plus de chances d'avoir une référence vers `v` qui ne sera pas valide.

<Listing number="16-4" file-name="src/main.rs" caption="Un thread avec une closure qui tente de capturer une référence vers `v` depuis un thread principal qui libère `v`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

<!--
If Rust allowed us to run this code, there's a possibility that the spawned
thread would be immediately put in the background without running at all. The
spawned thread has a reference to `v` inside, but the main thread immediately
drops `v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!
-->

Si Rust nous permettait d'exécuter ce code, il y aurait une possibilité que le thread créé soit immédiatement mis en arrière-plan sans s'exécuter du tout. Le thread créé contient une référence vers `v`, mais le thread principal libère immédiatement `v`, en utilisant la fonction `drop` dont nous avons discuté au chapitre 15. Ensuite, quand le thread créé commence à s'exécuter, `v` n'est plus valide, donc une référence vers lui est également invalide. Oh non !

<!--
To fix the compiler error in Listing 16-3, we can use the error message's
advice:
-->

Pour corriger l'erreur de compilation de l'encart 16-3, nous pouvons suivre le conseil du message d'erreur :

<!--
manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

<!--
By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it's using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend.
-->

En ajoutant le mot-clé `move` avant la closure, nous forçons la closure à prendre la possession des valeurs qu'elle utilise plutôt que de permettre à Rust d'inférer qu'elle devrait emprunter les valeurs. La modification de l'encart 16-3 montrée dans l'encart 16-5 compilera et s'exécutera comme nous le souhaitons.

<Listing number="16-5" file-name="src/main.rs" caption="Utiliser le mot-clé `move` pour forcer une closure à prendre la possession des valeurs qu'elle utilise">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

<!--
We might be tempted to try the same thing to fix the code in Listing 16-4 where
the main thread called `drop` by using a `move` closure. However, this fix will
not work because what Listing 16-4 is trying to do is disallowed for a
different reason. If we added `move` to the closure, we would move `v` into the
closure's environment, and we could no longer call `drop` on it in the main
thread. We would get this compiler error instead:
-->

Nous pourrions être tentés d'essayer la même chose pour corriger le code de l'encart 16-4 où le thread principal appelait `drop` en utilisant une closure `move`. Cependant, cette correction ne fonctionnera pas car ce que l'encart 16-4 essaie de faire est interdit pour une raison différente. Si nous ajoutions `move` à la closure, nous déplacerions `v` dans l'environnement de la closure, et nous ne pourrions plus appeler `drop` dessus dans le thread principal. Nous obtiendrions plutôt cette erreur de compilation :

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

<!--
Rust's ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread's reference. By telling Rust to move ownership of `v` to the spawned
thread, we're guaranteeing to Rust that the main thread won't use `v` anymore.
If we change Listing 16-4 in the same way, we're then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust's conservative default of borrowing; it doesn't let us violate the
ownership rules.
-->

Les règles de possession de Rust nous ont sauvés encore une fois ! Nous avons eu une erreur du code de l'encart 16-3 car Rust était conservateur et n'empruntait que `v` pour le thread, ce qui signifiait que le thread principal pouvait théoriquement invalider la référence du thread créé. En disant à Rust de déplacer la possession de `v` vers le thread créé, nous garantissons à Rust que le thread principal n'utilisera plus `v`. Si nous modifions l'encart 16-4 de la même manière, nous violons alors les règles de possession quand nous essayons d'utiliser `v` dans le thread principal. Le mot-clé `move` remplace le comportement conservateur par défaut de Rust qui est d'emprunter ; il ne nous permet pas de violer les règles de possession.

<!--
Now that we've covered what threads are and the methods supplied by the thread
API, let's look at some situations in which we can use threads.
-->

Maintenant que nous avons couvert ce que sont les threads et les méthodes fournies par l'API des threads, examinons quelques situations dans lesquelles nous pouvons utiliser les threads.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
