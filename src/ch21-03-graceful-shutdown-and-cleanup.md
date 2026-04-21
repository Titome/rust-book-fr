<!--
## Graceful Shutdown and Cleanup
-->

## Arrêt propre et nettoyage

<!--
The code in Listing 21-20 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we're not using in a direct way that reminds us
we're not cleaning up anything. When we use the less elegant
<kbd>ctrl</kbd>-<kbd>C</kbd> method to halt the main thread, all other threads
are stopped immediately as well, even if they're in the middle of serving a
request.
-->

Le code de l'encart 21-20 répond aux requêtes de manière asynchrone grâce à l'utilisation d'un thread pool, comme nous le souhaitions. Nous recevons des avertissements concernant les champs `workers`, `id` et `thread` que nous n'utilisons pas directement, ce qui nous rappelle que nous ne nettoyons rien. Quand nous utilisons la méthode peu élégante <kbd>ctrl</kbd>-<kbd>C</kbd> pour arrêter le thread principal, tous les autres threads sont arrêtés immédiatement aussi, même s'ils sont en train de traiter une requête.

<!--
Next, then, we'll implement the `Drop` trait to call `join` on each of the
threads in the pool so that they can finish the requests they're working on
before closing. Then, we'll implement a way to tell the threads they should
stop accepting new requests and shut down. To see this code in action, we'll
modify our server to accept only two requests before gracefully shutting down
its thread pool.
-->

Ensuite, nous allons implémenter le trait `Drop` pour appeler `join` sur chacun des threads du pool afin qu'ils puissent terminer les requêtes sur lesquelles ils travaillent avant de se fermer. Puis, nous implémenterons un moyen de dire aux threads qu'ils devraient arrêter d'accepter de nouvelles requêtes et s'arrêter. Pour voir ce code en action, nous modifierons notre serveur pour n'accepter que deux requêtes avant d'arrêter proprement son thread pool.

<!--
One thing to notice as we go: None of this affects the parts of the code that
handle executing the closures, so everything here would be the same if we were
using a thread pool for an async runtime.
-->

Une chose à remarquer au passage : rien de tout cela n'affecte les parties du code qui gèrent l'exécution des closures, donc tout ce qui est ici serait identique si nous utilisions un thread pool pour un runtime async.

<!--
### Implementing the `Drop` Trait on `ThreadPool`
-->

### Implémenter le trait `Drop` sur `ThreadPool`

<!--
Let's start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 21-22 shows a first attempt at a `Drop` implementation; this code won't
quite work yet.
-->

Commençons par implémenter `Drop` sur notre thread pool. Quand le pool est libéré, tous nos threads devraient se joindre (join) pour s'assurer qu'ils terminent leur travail. L'encart 21-22 montre une première tentative d'implémentation de `Drop` ; ce code ne fonctionnera pas tout à fait encore.

<Listing number="21-22" file-name="src/lib.rs" caption="Joindre chaque thread quand le thread pool sort de la portée">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

<!--
First, we loop through each of the thread pool `workers`. We use `&mut` for this
because `self` is a mutable reference, and we also need to be able to mutate
`worker`. For each `worker`, we print a message saying that this particular
`Worker` instance is shutting down, and then we call `join` on that `Worker`
instance's thread. If the call to `join` fails, we use `unwrap` to make Rust
panic and go into an ungraceful shutdown.
-->

D'abord, nous bouclons sur chaque `worker` du thread pool. Nous utilisons `&mut` pour cela car `self` est une référence mutable, et nous devons aussi pouvoir muter `worker`. Pour chaque `worker`, nous affichons un message disant que cette instance particulière de `Worker` s'arrête, puis nous appelons `join` sur le thread de cette instance de `Worker`. Si l'appel à `join` échoue, nous utilisons `unwrap` pour faire paniquer Rust et entrer dans un arrêt non propre.

<!--
Here is the error we get when we compile this code:
-->

Voici l'erreur que nous obtenons quand nous compilons ce code :


```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

<!--
The error tells us we can't call `join` because we only have a mutable borrow
of each `worker` and `join` takes ownership of its argument. To solve this
issue, we need to move the thread out of the `Worker` instance that owns
`thread` so that `join` can consume the thread. One way to do this is to take
the same approach we took in Listing 18-15. If `Worker` held an
`Option<thread::JoinHandle<()>>`, we could call the `take` method on the
`Option` to move the value out of the `Some` variant and leave a `None` variant
in its place. In other words, a `Worker` that is running would have a `Some`
variant in `thread`, and when we wanted to clean up a `Worker`, we'd replace
`Some` with `None` so that the `Worker` wouldn't have a thread to run.
-->

L'erreur nous dit que nous ne pouvons pas appeler `join` parce que nous n'avons qu'un emprunt mutable de chaque `worker` et que `join` prend la propriété de son argument. Pour résoudre ce problème, nous devons déplacer le thread hors de l'instance de `Worker` qui possède `thread` afin que `join` puisse consommer le thread. Une façon de faire cela est de prendre la même approche que celle de l'encart 18-15. Si `Worker` contenait un `Option<thread::JoinHandle<()>>`, nous pourrions appeler la méthode `take` sur l'`Option` pour déplacer la valeur hors de la variante `Some` et laisser une variante `None` à sa place. En d'autres termes, un `Worker` en cours d'exécution aurait une variante `Some` dans `thread`, et quand nous voudrions nettoyer un `Worker`, nous remplacerions `Some` par `None` pour que le `Worker` n'ait plus de thread à exécuter.

<!--
However, the _only_ time this would come up would be when dropping the
`Worker`. In exchange, we'd have to deal with an
`Option<thread::JoinHandle<()>>` anywhere we accessed `worker.thread`.
Idiomatic Rust uses `Option` quite a bit, but when you find yourself wrapping
something you know will always be present in an `Option` as a workaround like
this, it's a good idea to look for alternative approaches to make your code
cleaner and less error-prone.
-->

Cependant, le _seul_ moment où cela se produirait serait lors de la libération du `Worker`. En contrepartie, nous devrions gérer un `Option<thread::JoinHandle<()>>` partout où nous accéderions à `worker.thread`. Le Rust idiomatique utilise beaucoup `Option`, mais quand vous vous retrouvez à envelopper quelque chose que vous savez toujours présent dans un `Option` comme solution de contournement comme celle-ci, c'est une bonne idée de chercher des approches alternatives pour rendre votre code plus propre et moins sujet aux erreurs.

<!--
In this case, a better alternative exists: the `Vec::drain` method. It accepts
a range parameter to specify which items to remove from the vector and returns
an iterator of those items. Passing the `..` range syntax will remove *every*
value from the vector.
-->

Dans ce cas, une meilleure alternative existe : la méthode `Vec::drain`. Elle accepte un paramètre de plage pour spécifier quels éléments retirer du vecteur et retourne un itérateur de ces éléments. Passer la syntaxe de plage `..` retirera *chaque* valeur du vecteur.

<!--
So, we need to update the `ThreadPool` `drop` implementation like this:
-->

Donc, nous devons mettre à jour l'implémentation de `drop` du `ThreadPool` comme ceci :

<Listing file-name="src/lib.rs">


```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

<!--
This resolves the compiler error and does not require any other changes to our
code. Note that, because drop can be called when panicking, the unwrap
could also panic and cause a double panic, which immediately crashes the
program and ends any cleanup in progress. This is fine for an example program,
but it isn't recommended for production code.
-->

Cela résout l'erreur du compilateur et ne nécessite aucune autre modification de notre code. Notez que, comme drop peut être appelé lors d'un panic, le unwrap pourrait aussi paniquer et causer un double panic, ce qui fait immédiatement planter le programme et met fin à tout nettoyage en cours. C'est acceptable pour un programme d'exemple, mais ce n'est pas recommandé pour du code en production.

<!--
### Signaling to the Threads to Stop Listening for Jobs
-->

### Signaler aux threads d'arrêter d'écouter les tâches

<!--
With all the changes we've made, our code compiles without any warnings.
However, the bad news is that this code doesn't function the way we want it to
yet. The key is the logic in the closures run by the threads of the `Worker`
instances: At the moment, we call `join`, but that won't shut down the threads,
because they `loop` forever looking for jobs. If we try to drop our
`ThreadPool` with our current implementation of `drop`, the main thread will
block forever, waiting for the first thread to finish.
-->

Avec toutes les modifications que nous avons faites, notre code compile sans aucun avertissement. Cependant, la mauvaise nouvelle est que ce code ne fonctionne pas encore comme nous le voulons. La clé est la logique dans les closures exécutées par les threads des instances de `Worker` : pour le moment, nous appelons `join`, mais cela n'arrêtera pas les threads, car ils bouclent (`loop`) indéfiniment à la recherche de tâches. Si nous essayons de libérer notre `ThreadPool` avec notre implémentation actuelle de `drop`, le thread principal se bloquera indéfiniment, en attendant que le premier thread se termine.

<!--
To fix this problem, we'll need a change in the `ThreadPool` `drop`
implementation and then a change in the `Worker` loop.
-->

Pour corriger ce problème, nous aurons besoin d'un changement dans l'implémentation de `drop` du `ThreadPool` puis d'un changement dans la boucle du `Worker`.

<!--
First, we'll change the `ThreadPool` `drop` implementation to explicitly drop
the `sender` before waiting for the threads to finish. Listing 21-23 shows the
changes to `ThreadPool` to explicitly drop `sender`. Unlike with the thread,
here we _do_ need to use an `Option` to be able to move `sender` out of
`ThreadPool` with `Option::take`.
-->

D'abord, nous allons modifier l'implémentation de `drop` du `ThreadPool` pour libérer explicitement le `sender` avant d'attendre que les threads se terminent. L'encart 21-23 montre les modifications apportées au `ThreadPool` pour libérer explicitement `sender`. Contrairement au thread, ici nous _avons_ besoin d'utiliser un `Option` pour pouvoir déplacer `sender` hors du `ThreadPool` avec `Option::take`.

<Listing number="21-23" file-name="src/lib.rs" caption="Libérer explicitement `sender` avant de joindre les threads `Worker`">


```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

<!--
Dropping `sender` closes the channel, which indicates no more messages will be
sent. When that happens, all the calls to `recv` that the `Worker` instances do
in the infinite loop will return an error. In Listing 21-24, we change the
`Worker` loop to gracefully exit the loop in that case, which means the threads
will finish when the `ThreadPool` `drop` implementation calls `join` on them.
-->

Libérer `sender` ferme le canal, ce qui indique qu'aucun autre message ne sera envoyé. Quand cela se produit, tous les appels à `recv` que les instances de `Worker` font dans la boucle infinie retourneront une erreur. Dans l'encart 21-24, nous modifions la boucle du `Worker` pour sortir proprement de la boucle dans ce cas, ce qui signifie que les threads se termineront quand l'implémentation de `drop` du `ThreadPool` appellera `join` sur eux.

<Listing number="21-24" file-name="src/lib.rs" caption="Sortir explicitement de la boucle quand `recv` retourne une erreur">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

<!--
To see this code in action, let's modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 21-25.
-->

Pour voir ce code en action, modifions `main` pour n'accepter que deux requêtes avant d'arrêter proprement le serveur, comme montré dans l'encart 21-25.

<Listing number="21-25" file-name="src/main.rs" caption="Arrêter le serveur après avoir servi deux requêtes en sortant de la boucle">


```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

<!--
You wouldn't want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.
-->

Vous ne voudriez pas qu'un serveur web réel s'arrête après avoir servi seulement deux requêtes. Ce code démontre simplement que l'arrêt propre et le nettoyage fonctionnent correctement.

<!--
The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.
-->

La méthode `take` est définie dans le trait `Iterator` et limite l'itération aux deux premiers éléments au maximum. Le `ThreadPool` sortira de la portée à la fin de `main`, et l'implémentation de `drop` s'exécutera.

<!--
Start the server with `cargo run` and make three requests. The third request
should error, and in your terminal, you should see output similar to this:
-->

Démarrez le serveur avec `cargo run` et faites trois requêtes. La troisième requête devrait échouer, et dans votre terminal, vous devriez voir une sortie similaire à ceci :

<!--
manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

<!--
You might see a different ordering of `Worker` IDs and messages printed. We can
see how this code works from the messages: `Worker` instances 0 and 3 got the
first two requests. The server stopped accepting connections after the second
connection, and the `Drop` implementation on `ThreadPool` starts executing
before `Worker 3` even starts its job. Dropping the `sender` disconnects all the
`Worker` instances and tells them to shut down. The `Worker` instances each
print a message when they disconnect, and then the thread pool calls `join` to
wait for each `Worker` thread to finish.
-->

Vous pourriez voir un ordre différent des identifiants de `Worker` et des messages affichés. Nous pouvons voir comment ce code fonctionne d'après les messages : les instances de `Worker` 0 et 3 ont reçu les deux premières requêtes. Le serveur a arrêté d'accepter les connexions après la deuxième connexion, et l'implémentation de `Drop` sur `ThreadPool` commence à s'exécuter avant même que `Worker 3` ne commence sa tâche. La libération du `sender` déconnecte toutes les instances de `Worker` et leur dit de s'arrêter. Les instances de `Worker` affichent chacune un message quand elles se déconnectent, puis le thread pool appelle `join` pour attendre que chaque thread `Worker` se termine.

<!--
Notice one interesting aspect of this particular execution: The `ThreadPool`
dropped the `sender`, and before any `Worker` received an error, we tried to
join `Worker 0`. `Worker 0` had not yet gotten an error from `recv`, so the main
thread blocked, waiting for `Worker 0` to finish. In the meantime, `Worker 3`
received a job and then all threads received an error. When `Worker 0` finished,
the main thread waited for the rest of the `Worker` instances to finish. At that
point, they had all exited their loops and stopped.
-->

Remarquez un aspect intéressant de cette exécution particulière : le `ThreadPool` a libéré le `sender`, et avant qu'aucun `Worker` n'ait reçu d'erreur, nous avons essayé de joindre `Worker 0`. `Worker 0` n'avait pas encore reçu d'erreur de `recv`, donc le thread principal s'est bloqué, en attendant que `Worker 0` se termine. Entre-temps, `Worker 3` a reçu une tâche et ensuite tous les threads ont reçu une erreur. Quand `Worker 0` a terminé, le thread principal a attendu que le reste des instances de `Worker` se termine. À ce moment-là, elles avaient toutes quitté leurs boucles et s'étaient arrêtées.

<!--
Congrats! We've now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We're able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.
-->

Félicitations ! Nous avons maintenant terminé notre projet ; nous avons un serveur web basique qui utilise un thread pool pour répondre de manière asynchrone. Nous sommes capables d'effectuer un arrêt propre du serveur, qui nettoie tous les threads du pool.

<!--
Here's the full code for reference:
-->

Voici le code complet pour référence :

<Listing file-name="src/main.rs">


```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

<!--
We could do more here! If you want to continue enhancing this project, here are
some ideas:

- Add more documentation to `ThreadPool` and its public methods.
- Add tests of the library's functionality.
- Change calls to `unwrap` to more robust error handling.
- Use `ThreadPool` to perform some task other than serving web requests.
- Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then, compare its API and
  robustness to the thread pool we implemented.
-->

Nous pourrions faire plus ici ! Si vous voulez continuer à améliorer ce projet, voici quelques idées :

- Ajouter plus de documentation à `ThreadPool` et à ses méthodes publiques.
- Ajouter des tests de la fonctionnalité de la bibliothèque.
- Remplacer les appels à `unwrap` par une gestion d'erreurs plus robuste.
- Utiliser `ThreadPool` pour effectuer une tâche autre que servir des requêtes web.
- Trouver un crate de thread pool sur [crates.io](https://crates.io/) et implémenter un serveur web similaire en utilisant ce crate à la place. Ensuite, comparer son API et sa robustesse au thread pool que nous avons implémenté.

<!--
## Summary
-->

## Résumé

<!--
Well done! You've made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You're now ready to implement your own Rust
projects and help with other people's projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
-->

Bien joué ! Vous êtes arrivé à la fin du livre ! Nous voulons vous remercier de nous avoir accompagnés dans cette visite de Rust. Vous êtes maintenant prêt à implémenter vos propres projets Rust et à aider dans les projets des autres. Gardez à l'esprit qu'il existe une communauté accueillante d'autres Rustacés qui seraient ravis de vous aider avec tous les défis que vous rencontrerez dans votre parcours Rust.
