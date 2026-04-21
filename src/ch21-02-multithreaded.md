<!--
Old headings. Do not remove or links may break.
-->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>

<!--
## From a Single-Threaded to a Multithreaded Server
-->

## D'un serveur monothread à un serveur multithreadé

<!--
Right now, the server will process each request in turn, meaning it won't
process a second connection until the first connection is finished processing.
If the server received more and more requests, this serial execution would be
less and less optimal. If the server receives a request that takes a long time
to process, subsequent requests will have to wait until the long request is
finished, even if the new requests can be processed quickly. We'll need to fix
this, but first we'll look at the problem in action.
-->

Pour l'instant, le serveur traitera chaque requête à tour de rôle, ce qui signifie qu'il ne traitera pas une deuxième connexion tant que la première connexion n'aura pas fini d'être traitée. Si le serveur recevait de plus en plus de requêtes, cette exécution séquentielle serait de moins en moins optimale. Si le serveur reçoit une requête qui prend beaucoup de temps à traiter, les requêtes suivantes devront attendre que la longue requête soit terminée, même si les nouvelles requêtes peuvent être traitées rapidement. Nous devons corriger cela, mais d'abord nous allons observer le problème en action.

<!--
Old headings. Do not remove or links may break.
-->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>

<!--
### Simulating a Slow Request
-->

### Simuler une requête lente

<!--
We'll look at how a slowly processing request can affect other requests made to
our current server implementation. Listing 21-10 implements handling a request
to _/sleep_ with a simulated slow response that will cause the server to sleep
for five seconds before responding.
-->

Nous allons voir comment une requête traitée lentement peut affecter les autres requêtes faites à notre implémentation actuelle du serveur. L'encart 21-10 implémente la gestion d'une requête vers _/sleep_ avec une réponse lente simulée qui fera dormir le serveur pendant cinq secondes avant de répondre.

<Listing number="21-10" file-name="src/main.rs" caption="Simuler une requête lente en mettant le serveur en veille pendant cinq secondes">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

<!--
We switched from `if` to `match` now that we have three cases. We need to
explicitly match on a slice of `request_line` to pattern-match against the
string literal values; `match` doesn't do automatic referencing and
dereferencing, like the equality method does.
-->

Nous sommes passés de `if` à `match` maintenant que nous avons trois cas. Nous devons faire correspondre explicitement sur une slice de `request_line` pour faire du pattern matching avec les valeurs littérales de chaîne ; `match` ne fait pas de référencement et déréférencement automatique, contrairement à la méthode d'égalité.

<!--
The first arm is the same as the `if` block from Listing 21-9. The second arm
matches a request to _/sleep_. When that request is received, the server will
sleep for five seconds before rendering the successful HTML page. The third arm
is the same as the `else` block from Listing 21-9.
-->

Le premier bras est le même que le bloc `if` de l'encart 21-9. Le deuxième bras correspond à une requête vers _/sleep_. Quand cette requête est reçue, le serveur dormira pendant cinq secondes avant d'afficher la page HTML de succès. Le troisième bras est le même que le bloc `else` de l'encart 21-9.

<!--
You can see how primitive our server is: Real libraries would handle the
recognition of multiple requests in a much less verbose way!
-->

Vous pouvez voir à quel point notre serveur est primitif : de vraies bibliothèques géreraient la reconnaissance de multiples requêtes de manière bien moins verbeuse !

<!--
Start the server using `cargo run`. Then, open two browser windows: one for
_http://127.0.0.1:7878_ and the other for _http://127.0.0.1:7878/sleep_. If you
enter the _/_ URI a few times, as before, you'll see it respond quickly. But if
you enter _/sleep_ and then load _/_, you'll see that _/_ waits until `sleep`
has slept for its full five seconds before loading.
-->

Démarrez le serveur en utilisant `cargo run`. Ensuite, ouvrez deux fenêtres de navigateur : une pour _http://127.0.0.1:7878_ et l'autre pour _http://127.0.0.1:7878/sleep_. Si vous entrez l'URI _/_ quelques fois, comme avant, vous verrez qu'il répond rapidement. Mais si vous entrez _/sleep_ puis chargez _/_, vous verrez que _/_ attend que `sleep` ait dormi pendant ses cinq secondes complètes avant de se charger.

<!--
There are multiple techniques we could use to avoid requests backing up behind
a slow request, including using async as we did Chapter 17; the one we'll
implement is a thread pool.
-->

Il existe plusieurs techniques que nous pourrions utiliser pour éviter que les requêtes ne s'accumulent derrière une requête lente, notamment l'utilisation d'async comme nous l'avons fait au chapitre 17 ; celle que nous allons implémenter est un groupe de threads (thread pool).

<!--
### Improving Throughput with a Thread Pool
-->

### Améliorer le débit avec un groupe de threads

<!--
A _thread pool_ is a group of spawned threads that are ready and waiting to
handle a task. When the program receives a new task, it assigns one of the
threads in the pool to the task, and that thread will process the task. The
remaining threads in the pool are available to handle any other tasks that come
in while the first thread is processing. When the first thread is done
processing its task, it's returned to the pool of idle threads, ready to handle
a new task. A thread pool allows you to process connections concurrently,
increasing the throughput of your server.
-->

Un _thread pool_ (groupe de threads) est un groupe de threads créés qui sont prêts et en attente de traiter une tâche. Quand le programme reçoit une nouvelle tâche, il assigne l'un des threads du pool à la tâche, et ce thread traitera la tâche. Les threads restants dans le pool sont disponibles pour gérer toute autre tâche qui arrive pendant que le premier thread est en cours de traitement. Quand le premier thread a terminé de traiter sa tâche, il est renvoyé dans le pool de threads inactifs, prêt à traiter une nouvelle tâche. Un thread pool vous permet de traiter des connexions de manière concurrente, augmentant le débit de votre serveur.

<!--
We'll limit the number of threads in the pool to a small number to protect us
from DoS attacks; if we had our program create a new thread for each request as
it came in, someone making 10 million requests to our server could wreak havoc
by using up all our server's resources and grinding the processing of requests
to a halt.
-->

Nous limiterons le nombre de threads dans le pool à un petit nombre pour nous protéger des attaques DoS ; si notre programme créait un nouveau thread pour chaque requête entrante, quelqu'un faisant 10 millions de requêtes à notre serveur pourrait semer le chaos en épuisant toutes les ressources de notre serveur et en paralysant le traitement des requêtes.

<!--
Rather than spawning unlimited threads, then, we'll have a fixed number of
threads waiting in the pool. Requests that come in are sent to the pool for
processing. The pool will maintain a queue of incoming requests. Each of the
threads in the pool will pop off a request from this queue, handle the request,
and then ask the queue for another request. With this design, we can process up
to _`N`_ requests concurrently, where _`N`_ is the number of threads. If each
thread is responding to a long-running request, subsequent requests can still
back up in the queue, but we've increased the number of long-running requests
we can handle before reaching that point.
-->

Plutôt que de créer un nombre illimité de threads, nous aurons donc un nombre fixe de threads en attente dans le pool. Les requêtes entrantes sont envoyées au pool pour traitement. Le pool maintiendra une file d'attente de requêtes entrantes. Chaque thread du pool retirera une requête de cette file, traitera la requête, puis demandera une autre requête à la file. Avec cette conception, nous pouvons traiter jusqu'à _`N`_ requêtes de manière concurrente, où _`N`_ est le nombre de threads. Si chaque thread répond à une requête de longue durée, les requêtes suivantes peuvent toujours s'accumuler dans la file, mais nous avons augmenté le nombre de requêtes de longue durée que nous pouvons gérer avant d'atteindre ce point.

<!--
This technique is just one of many ways to improve the throughput of a web
server. Other options you might explore are the fork/join model, the
single-threaded async I/O model, and the multithreaded async I/O model. If
you're interested in this topic, you can read more about other solutions and
try to implement them; with a low-level language like Rust, all of these
options are possible.
-->

Cette technique n'est qu'un des nombreux moyens d'améliorer le débit d'un serveur web. D'autres options que vous pourriez explorer sont le modèle fork/join, le modèle d'E/S async monothread et le modèle d'E/S async multithreadé. Si ce sujet vous intéresse, vous pouvez en lire davantage sur les autres solutions et essayer de les implémenter ; avec un langage de bas niveau comme Rust, toutes ces options sont possibles.

<!--
Before we begin implementing a thread pool, let's talk about what using the
pool should look like. When you're trying to design code, writing the client
interface first can help guide your design. Write the API of the code so that
it's structured in the way you want to call it; then, implement the
functionality within that structure rather than implementing the functionality
and then designing the public API.
-->

Avant de commencer à implémenter un thread pool, parlons de ce à quoi l'utilisation du pool devrait ressembler. Quand vous essayez de concevoir du code, écrire l'interface client en premier peut aider à guider votre conception. Écrivez l'API du code de sorte qu'elle soit structurée de la manière dont vous voulez l'appeler ; ensuite, implémentez la fonctionnalité au sein de cette structure plutôt que d'implémenter la fonctionnalité puis de concevoir l'API publique.

<!--
Similar to how we used test-driven development in the project in Chapter 12,
we'll use compiler-driven development here. We'll write the code that calls the
functions we want, and then we'll look at errors from the compiler to determine
what we should change next to get the code to work. Before we do that, however,
we'll explore the technique we're not going to use as a starting point.
-->

De manière similaire à la façon dont nous avons utilisé le développement piloté par les tests dans le projet du chapitre 12, nous utiliserons ici le développement piloté par le compilateur. Nous écrirons le code qui appelle les fonctions que nous voulons, puis nous examinerons les erreurs du compilateur pour déterminer ce que nous devons changer ensuite pour que le code fonctionne. Avant de faire cela, cependant, nous explorerons la technique que nous n'allons pas utiliser comme point de départ.

<!--
Old headings. Do not remove or links may break.
-->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

<!--
#### Spawning a Thread for Each Request
-->

#### Créer un thread pour chaque requête

<!--
First, let's explore how our code might look if it did create a new thread for
every connection. As mentioned earlier, this isn't our final plan due to the
problems with potentially spawning an unlimited number of threads, but it is a
starting point to get a working multithreaded server first. Then, we'll add the
thread pool as an improvement, and contrasting the two solutions will be easier.
-->

D'abord, explorons à quoi notre code pourrait ressembler s'il créait un nouveau thread pour chaque connexion. Comme mentionné précédemment, ce n'est pas notre plan final en raison des problèmes liés à la création potentiellement illimitée de threads, mais c'est un point de départ pour obtenir d'abord un serveur multithreadé fonctionnel. Ensuite, nous ajouterons le thread pool comme amélioration, et comparer les deux solutions sera plus facile.

<!--
Listing 21-11 shows the changes to make to `main` to spawn a new thread to
handle each stream within the `for` loop.
-->

L'encart 21-11 montre les modifications à apporter à `main` pour créer un nouveau thread afin de gérer chaque flux dans la boucle `for`.

<Listing number="21-11" file-name="src/main.rs" caption="Créer un nouveau thread pour chaque flux">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

<!--
As you learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in the new thread. If you run this code and load
_/sleep_ in your browser, then _/_ in two more browser tabs, you'll indeed see
that the requests to _/_ don't have to wait for _/sleep_ to finish. However, as
we mentioned, this will eventually overwhelm the system because you'd be making
new threads without any limit.
-->

Comme vous l'avez appris au chapitre 16, `thread::spawn` créera un nouveau thread puis exécutera le code de la closure dans le nouveau thread. Si vous exécutez ce code et chargez _/sleep_ dans votre navigateur, puis _/_ dans deux autres onglets du navigateur, vous verrez effectivement que les requêtes vers _/_ n'ont pas besoin d'attendre que _/sleep_ se termine. Cependant, comme nous l'avons mentionné, cela finira par submerger le système car vous créeriez de nouveaux threads sans aucune limite.

<!--
You may also recall from Chapter 17 that this is exactly the kind of situation
where async and await really shine! Keep that in mind as we build the thread
pool and think about how things would look different or the same with async.
-->

Vous vous souvenez peut-être aussi du chapitre 17 que c'est exactement le type de situation où async et await brillent vraiment ! Gardez cela à l'esprit pendant que nous construisons le thread pool et réfléchissez à ce qui serait différent ou identique avec async.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

<!--
#### Creating a Finite Number of Threads
-->

#### Créer un nombre fini de threads

<!--
We want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn't require large changes to the code that
uses our API. Listing 21-12 shows the hypothetical interface for a `ThreadPool`
struct we want to use instead of `thread::spawn`.
-->

Nous voulons que notre thread pool fonctionne de manière similaire et familière afin que passer des threads à un thread pool ne nécessite pas de grands changements dans le code qui utilise notre API. L'encart 21-12 montre l'interface hypothétique d'une structure `ThreadPool` que nous voulons utiliser à la place de `thread::spawn`.

<Listing number="21-12" file-name="src/main.rs" caption="Notre interface `ThreadPool` idéale">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

<!--
We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` has a
similar interface as `thread::spawn` in that it takes a closure that the pool
should run for each stream. We need to implement `pool.execute` so that it
takes the closure and gives it to a thread in the pool to run. This code won't
yet compile, but we'll try so that the compiler can guide us in how to fix it.
-->

Nous utilisons `ThreadPool::new` pour créer un nouveau thread pool avec un nombre configurable de threads, dans ce cas quatre. Ensuite, dans la boucle `for`, `pool.execute` a une interface similaire à `thread::spawn` en ce qu'il prend une closure que le pool devrait exécuter pour chaque flux. Nous devons implémenter `pool.execute` de sorte qu'il prenne la closure et la donne à un thread du pool pour l'exécuter. Ce code ne compilera pas encore, mais nous allons essayer pour que le compilateur puisse nous guider dans la correction.

<!--
Old headings. Do not remove or links may break.
-->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

<!--
#### Building `ThreadPool` Using Compiler-Driven Development
-->

#### Construire `ThreadPool` en utilisant le développement piloté par le compilateur

<!--
Make the changes in Listing 21-12 to _src/main.rs_, and then let's use the
compiler errors from `cargo check` to drive our development. Here is the first
error we get:
-->

Effectuez les modifications de l'encart 21-12 dans _src/main.rs_, puis utilisons les erreurs du compilateur de `cargo check` pour piloter notre développement. Voici la première erreur que nous obtenons :


```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

<!--
Great! This error tells us we need a `ThreadPool` type or module, so we'll
build one now. Our `ThreadPool` implementation will be independent of the kind
of work our web server is doing. So, let's switch the `hello` crate from a
binary crate to a library crate to hold our `ThreadPool` implementation. After
we change to a library crate, we could also use the separate thread pool
library for any work we want to do using a thread pool, not just for serving
web requests.
-->

Parfait ! Cette erreur nous dit que nous avons besoin d'un type ou module `ThreadPool`, donc nous allons en construire un maintenant. Notre implémentation de `ThreadPool` sera indépendante du type de travail que notre serveur web effectue. Donc, transformons le crate `hello` d'un crate binaire en un crate de bibliothèque pour héberger notre implémentation de `ThreadPool`. Après être passé à un crate de bibliothèque, nous pourrons aussi utiliser la bibliothèque de thread pool séparée pour tout travail que nous voulons faire en utilisant un thread pool, pas seulement pour servir des requêtes web.

<!--
Create a _src/lib.rs_ file that contains the following, which is the simplest
definition of a `ThreadPool` struct that we can have for now:
-->

Créez un fichier _src/lib.rs_ qui contient ce qui suit, qui est la définition la plus simple d'une structure `ThreadPool` que nous puissions avoir pour le moment :

<Listing file-name="src/lib.rs">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

<!--
Then, edit the _main.rs_ file to bring `ThreadPool` into scope from the library
crate by adding the following code to the top of _src/main.rs_:
-->

Ensuite, modifiez le fichier _main.rs_ pour importer `ThreadPool` dans la portée depuis le crate de bibliothèque en ajoutant le code suivant en haut de _src/main.rs_ :

<Listing file-name="src/main.rs">


```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

<!--
This code still won't work, but let's check it again to get the next error that
we need to address:
-->

Ce code ne fonctionnera toujours pas, mais vérifions-le à nouveau pour obtenir la prochaine erreur que nous devons traiter :


```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

<!--
This error indicates that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument and should return a `ThreadPool` instance.
Let's implement the simplest `new` function that will have those
characteristics:
-->

Cette erreur indique qu'ensuite nous devons créer une fonction associée nommée `new` pour `ThreadPool`. Nous savons aussi que `new` doit avoir un paramètre qui peut accepter `4` comme argument et devrait retourner une instance de `ThreadPool`. Implémentons la fonction `new` la plus simple qui aura ces caractéristiques :

<Listing file-name="src/lib.rs">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

<!--
We chose `usize` as the type of the `size` parameter because we know that a
negative number of threads doesn't make any sense. We also know we'll use this
`4` as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the ["Integer Types"][integer-types]
ignore
--> section in Chapter 3.
-->

Nous avons choisi `usize` comme type du paramètre `size` car nous savons qu'un nombre négatif de threads n'a aucun sens. Nous savons aussi que nous utiliserons ce `4` comme nombre d'éléments dans une collection de threads, ce qui est la raison d'être du type `usize`, comme discuté dans la section [« Les types d'entiers »][integer-types]<!--
ignore
--> du chapitre 3.

<!--
Let's check the code again:
-->

Vérifions le code à nouveau :


```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

<!--
Now the error occurs because we don't have an `execute` method on `ThreadPool`.
Recall from the ["Creating a Finite Number of
Threads"](#creating-a-finite-number-of-threads) ignore
--> section that we
decided our thread pool should have an interface similar to `thread::spawn`. In
addition, we'll implement the `execute` function so that it takes the closure
it's given and gives it to an idle thread in the pool to run.
-->

Maintenant l'erreur survient parce que nous n'avons pas de méthode `execute` sur `ThreadPool`. Rappelez-vous de la section [« Créer un nombre fini de threads »](#créer-un-nombre-fini-de-threads)<!--
ignore
--> que nous avions décidé que notre thread pool devrait avoir une interface similaire à `thread::spawn`. De plus, nous allons implémenter la fonction `execute` de sorte qu'elle prenne la closure qu'on lui donne et la transmette à un thread inactif du pool pour l'exécuter.

<!--
We'll define the `execute` method on `ThreadPool` to take a closure as a
parameter. Recall from the ["Moving Captured Values Out of
Closures"][moving-out-of-closures] ignore
--> in Chapter 13 that we can
take closures as parameters with three different traits: `Fn`, `FnMut`, and
`FnOnce`. We need to decide which kind of closure to use here. We know we'll
end up doing something similar to the standard library `thread::spawn`
implementation, so we can look at what bounds the signature of `thread::spawn`
has on its parameter. The documentation shows us the following:
-->

Nous allons définir la méthode `execute` sur `ThreadPool` pour prendre une closure comme paramètre. Rappelez-vous de la section [« Déplacer les valeurs capturées hors des closures »][moving-out-of-closures]<!--
ignore
--> du chapitre 13 que nous pouvons prendre des closures comme paramètres avec trois traits différents : `Fn`, `FnMut` et `FnOnce`. Nous devons décider quel type de closure utiliser ici. Nous savons que nous finirons par faire quelque chose de similaire à l'implémentation de `thread::spawn` de la bibliothèque standard, donc nous pouvons regarder quelles contraintes la signature de `thread::spawn` a sur son paramètre. La documentation nous montre ce qui suit :

<!--
```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```
-->

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `F` type parameter is the one we're concerned with here; the `T` type
parameter is related to the return value, and we're not concerned with that. We
can see that `spawn` uses `FnOnce` as the trait bound on `F`. This is probably
what we want as well, because we'll eventually pass the argument we get in
`execute` to `spawn`. We can be further confident that `FnOnce` is the trait we
want to use because the thread for running a request will only execute that
request's closure one time, which matches the `Once` in `FnOnce`.
-->

Le paramètre de type `F` est celui qui nous intéresse ici ; le paramètre de type `T` est lié à la valeur de retour, et cela ne nous concerne pas. Nous pouvons voir que `spawn` utilise `FnOnce` comme contrainte de trait sur `F`. C'est probablement ce que nous voulons aussi, car nous finirons par passer l'argument que nous recevons dans `execute` à `spawn`. Nous pouvons être encore plus confiants que `FnOnce` est le trait que nous voulons utiliser parce que le thread exécutant une requête n'exécutera la closure de cette requête qu'une seule fois, ce qui correspond au `Once` de `FnOnce`.

<!--
The `F` type parameter also has the trait bound `Send` and the lifetime bound
`'static`, which are useful in our situation: We need `Send` to transfer the
closure from one thread to another and `'static` because we don't know how long
the thread will take to execute. Let's create an `execute` method on
`ThreadPool` that will take a generic parameter of type `F` with these bounds:
-->

Le paramètre de type `F` a aussi la contrainte de trait `Send` et la contrainte de durée de vie `'static`, qui sont utiles dans notre situation : nous avons besoin de `Send` pour transférer la closure d'un thread à un autre et de `'static` parce que nous ne savons pas combien de temps le thread prendra pour s'exécuter. Créons une méthode `execute` sur `ThreadPool` qui prendra un paramètre générique de type `F` avec ces contraintes :

<Listing file-name="src/lib.rs">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

<!--
We still use the `()` after `FnOnce` because this `FnOnce` represents a closure
that takes no parameters and returns the unit type `()`. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.
-->

Nous utilisons toujours `()` après `FnOnce` parce que ce `FnOnce` représente une closure qui ne prend pas de paramètres et retourne le type unitaire `()`. Tout comme les définitions de fonctions, le type de retour peut être omis de la signature, mais même si nous n'avons pas de paramètres, nous avons toujours besoin des parenthèses.

<!--
Again, this is the simplest implementation of the `execute` method: It does
nothing, but we're only trying to make our code compile. Let's check it again:
-->

Encore une fois, c'est l'implémentation la plus simple de la méthode `execute` : elle ne fait rien, mais nous essayons seulement de faire compiler notre code. Vérifions-le à nouveau :


```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

<!--
It compiles! But note that if you try `cargo run` and make a request in the
browser, you'll see the errors in the browser that we saw at the beginning of
the chapter. Our library isn't actually calling the closure passed to `execute`
yet!
-->

Ça compile ! Mais notez que si vous essayez `cargo run` et faites une requête dans le navigateur, vous verrez les erreurs dans le navigateur que nous avons vues au début du chapitre. Notre bibliothèque n'appelle pas encore réellement la closure passée à `execute` !

<!--
> Note: A saying you might hear about languages with strict compilers, such as
> Haskell and Rust, is "If the code compiles, it works." But this saying is not
> universally true. Our project compiles, but it does absolutely nothing! If we
> were building a real, complete project, this would be a good time to start
> writing unit tests to check that the code compiles _and_ has the behavior we
> want.
-->

> Remarque : un dicton que vous pourriez entendre à propos des langages avec des compilateurs stricts, comme Haskell et Rust, est « Si le code compile, il fonctionne. » Mais ce dicton n'est pas universellement vrai. Notre projet compile, mais il ne fait absolument rien ! Si nous construisions un vrai projet complet, ce serait le bon moment pour commencer à écrire des tests unitaires pour vérifier que le code compile _et_ a le comportement que nous voulons.

<!--
Consider: What would be different here if we were going to execute a future
instead of a closure?
-->

Réfléchissez : qu'est-ce qui serait différent ici si nous allions exécuter une future au lieu d'une closure ?

<!--
#### Validating the Number of Threads in `new`
-->

#### Valider le nombre de threads dans `new`

<!--
We aren't doing anything with the parameters to `new` and `execute`. Let's
implement the bodies of these functions with the behavior we want. To start,
let's think about `new`. Earlier we chose an unsigned type for the `size`
parameter because a pool with a negative number of threads makes no sense.
However, a pool with zero threads also makes no sense, yet zero is a perfectly
valid `usize`. We'll add code to check that `size` is greater than zero before
we return a `ThreadPool` instance, and we'll have the program panic if it
receives a zero by using the `assert!` macro, as shown in Listing 21-13.
-->

Nous ne faisons rien avec les paramètres de `new` et `execute`. Implémentons les corps de ces fonctions avec le comportement que nous voulons. Pour commencer, pensons à `new`. Plus tôt, nous avons choisi un type non signé pour le paramètre `size` parce qu'un pool avec un nombre négatif de threads n'a aucun sens. Cependant, un pool avec zéro thread n'a pas de sens non plus, pourtant zéro est une valeur `usize` parfaitement valide. Nous ajouterons du code pour vérifier que `size` est supérieur à zéro avant de retourner une instance de `ThreadPool`, et nous ferons paniquer le programme s'il reçoit zéro en utilisant la macro `assert!`, comme montré dans l'encart 21-13.

<Listing number="21-13" file-name="src/lib.rs" caption="Implémenter `ThreadPool::new` pour paniquer si `size` est zéro">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

<!--
We've also added some documentation for our `ThreadPool` with doc comments.
Note that we followed good documentation practices by adding a section that
calls out the situations in which our function can panic, as discussed in
Chapter 14. Try running `cargo doc --open` and clicking the `ThreadPool` struct
to see what the generated docs for `new` look like!
-->

Nous avons aussi ajouté de la documentation pour notre `ThreadPool` avec des commentaires de documentation. Notez que nous avons suivi les bonnes pratiques de documentation en ajoutant une section qui signale les situations dans lesquelles notre fonction peut paniquer, comme discuté au chapitre 14. Essayez d'exécuter `cargo doc --open` et de cliquer sur la structure `ThreadPool` pour voir à quoi ressemble la documentation générée pour `new` !

<!--
Instead of adding the `assert!` macro as we've done here, we could change `new`
into `build` and return a `Result` like we did with `Config::build` in the I/O
project in Listing 12-9. But we've decided in this case that trying to create a
thread pool without any threads should be an unrecoverable error. If you're
feeling ambitious, try to write a function named `build` with the following
signature to compare with the `new` function:
-->

Au lieu d'ajouter la macro `assert!` comme nous l'avons fait ici, nous pourrions transformer `new` en `build` et retourner un `Result` comme nous l'avons fait avec `Config::build` dans le projet d'E/S de l'encart 12-9. Mais nous avons décidé dans ce cas qu'essayer de créer un thread pool sans aucun thread devrait être une erreur irrécupérable. Si vous êtes ambitieux, essayez d'écrire une fonction nommée `build` avec la signature suivante pour la comparer avec la fonction `new` :

<!--
```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```
-->

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

<!--
#### Creating Space to Store the Threads
-->

#### Créer l'espace pour stocker les threads

<!--
Now that we have a way to know we have a valid number of threads to store in
the pool, we can create those threads and store them in the `ThreadPool` struct
before returning the struct. But how do we "store" a thread? Let's take another
look at the `thread::spawn` signature:
-->

Maintenant que nous avons un moyen de savoir que nous avons un nombre valide de threads à stocker dans le pool, nous pouvons créer ces threads et les stocker dans la structure `ThreadPool` avant de retourner la structure. Mais comment « stocker » un thread ? Regardons à nouveau la signature de `thread::spawn` :

<!--
```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```
-->

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `spawn` function returns a `JoinHandle<T>`, where `T` is the type that the
closure returns. Let's try using `JoinHandle` too and see what happens. In our
case, the closures we're passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.
-->

La fonction `spawn` retourne un `JoinHandle<T>`, où `T` est le type que la closure retourne. Essayons d'utiliser `JoinHandle` aussi et voyons ce qui se passe. Dans notre cas, les closures que nous passons au thread pool géreront la connexion et ne retourneront rien, donc `T` sera le type unitaire `()`.

<!--
The code in Listing 21-14 will compile, but it doesn't create any threads yet.
We've changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them.
-->

Le code de l'encart 21-14 compilera, mais il ne crée pas encore de threads. Nous avons changé la définition de `ThreadPool` pour contenir un vecteur d'instances `thread::JoinHandle<()>`, initialisé le vecteur avec une capacité de `size`, mis en place une boucle `for` qui exécutera du code pour créer les threads, et retourné une instance de `ThreadPool` les contenant.

<Listing number="21-14" file-name="src/lib.rs" caption="Créer un vecteur pour que `ThreadPool` contienne les threads">


```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

<!--
We've brought `std::thread` into scope in the library crate because we're
using `thread::JoinHandle` as the type of the items in the vector in
`ThreadPool`.
-->

Nous avons importé `std::thread` dans la portée du crate de bibliothèque car nous utilisons `thread::JoinHandle` comme type des éléments du vecteur dans `ThreadPool`.

<!--
Once a valid size is received, our `ThreadPool` creates a new vector that can
hold `size` items. The `with_capacity` function performs the same task as
`Vec::new` but with an important difference: It pre-allocates space in the
vector. Because we know we need to store `size` elements in the vector, doing
this allocation up front is slightly more efficient than using `Vec::new`,
which resizes itself as elements are inserted.
-->

Une fois qu'une taille valide est reçue, notre `ThreadPool` crée un nouveau vecteur qui peut contenir `size` éléments. La fonction `with_capacity` effectue la même tâche que `Vec::new` mais avec une différence importante : elle pré-alloue l'espace dans le vecteur. Parce que nous savons que nous devons stocker `size` éléments dans le vecteur, faire cette allocation à l'avance est légèrement plus efficace que d'utiliser `Vec::new`, qui se redimensionne au fur et à mesure que les éléments sont insérés.

<!--
When you run `cargo check` again, it should succeed.
-->

Quand vous exécutez `cargo check` à nouveau, cela devrait réussir.

<!--
Old headings. Do not remove or links may break.
-->
<a id ="a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread"></a>

<!--
#### Sending Code from the `ThreadPool` to a Thread
-->

#### Envoyer du code du `ThreadPool` vers un thread

<!--
We left a comment in the `for` loop in Listing 21-14 regarding the creation of
threads. Here, we'll look at how we actually create threads. The standard
library provides `thread::spawn` as a way to create threads, and
`thread::spawn` expects to get some code the thread should run as soon as the
thread is created. However, in our case, we want to create the threads and have
them _wait_ for code that we'll send later. The standard library's
implementation of threads doesn't include any way to do that; we have to
implement it manually.
-->

Nous avons laissé un commentaire dans la boucle `for` de l'encart 21-14 concernant la création de threads. Ici, nous allons voir comment nous créons réellement les threads. La bibliothèque standard fournit `thread::spawn` comme moyen de créer des threads, et `thread::spawn` s'attend à recevoir du code que le thread devrait exécuter dès que le thread est créé. Cependant, dans notre cas, nous voulons créer les threads et les faire _attendre_ du code que nous enverrons plus tard. L'implémentation des threads de la bibliothèque standard n'inclut aucun moyen de faire cela ; nous devons l'implémenter manuellement.

<!--
We'll implement this behavior by introducing a new data structure between the
`ThreadPool` and the threads that will manage this new behavior. We'll call
this data structure _Worker_, which is a common term in pooling
implementations. The `Worker` picks up code that needs to be run and runs the
code in its thread.
-->

Nous allons implémenter ce comportement en introduisant une nouvelle structure de données entre le `ThreadPool` et les threads qui gérera ce nouveau comportement. Nous appellerons cette structure de données _Worker_, qui est un terme courant dans les implémentations de pools. Le `Worker` récupère le code qui doit être exécuté et l'exécute dans son thread.

<!--
Think of people working in the kitchen at a restaurant: The workers wait until
orders come in from customers, and then they're responsible for taking those
orders and filling them.
-->

Pensez aux personnes travaillant en cuisine dans un restaurant : les workers attendent que les commandes arrivent des clients, puis ils sont responsables de prendre ces commandes et de les exécuter.

<!--
Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we'll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then, we'll implement a method on `Worker` that will
take a closure of code to run and send it to the already running thread for
execution. We'll also give each `Worker` an `id` so that we can distinguish
between the different instances of `Worker` in the pool when logging or
debugging.
-->

Au lieu de stocker un vecteur d'instances `JoinHandle<()>` dans le thread pool, nous stockerons des instances de la structure `Worker`. Chaque `Worker` stockera une seule instance de `JoinHandle<()>`. Ensuite, nous implémenterons une méthode sur `Worker` qui prendra une closure de code à exécuter et l'enverra au thread déjà en cours d'exécution pour traitement. Nous donnerons aussi à chaque `Worker` un `id` pour que nous puissions distinguer les différentes instances de `Worker` dans le pool lors de la journalisation ou du débogage.

<!--
Here is the new process that will happen when we create a `ThreadPool`. We'll
implement the code that sends the closure to the thread after we have `Worker`
set up in this way:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`.
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the `id` and a thread spawned with an empty
   closure.
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the `Worker` in the vector.
-->

Voici le nouveau processus qui se produira quand nous créerons un `ThreadPool`. Nous implémenterons le code qui envoie la closure au thread après avoir mis en place le `Worker` de cette manière :

1. Définir une structure `Worker` qui contient un `id` et un `JoinHandle<()>`.
2. Modifier `ThreadPool` pour contenir un vecteur d'instances de `Worker`.
3. Définir une fonction `Worker::new` qui prend un numéro d'`id` et retourne une instance de `Worker` qui contient l'`id` et un thread créé avec une closure vide.
4. Dans `ThreadPool::new`, utiliser le compteur de la boucle `for` pour générer un `id`, créer un nouveau `Worker` avec cet `id`, et stocker le `Worker` dans le vecteur.

<!--
If you're up for a challenge, try implementing these changes on your own before
looking at the code in Listing 21-15.
-->

Si vous aimez les défis, essayez d'implémenter ces changements par vous-même avant de regarder le code de l'encart 21-15.

<!--
Ready? Here is Listing 21-15 with one way to make the preceding modifications.
-->

Prêt ? Voici l'encart 21-15 avec une façon de faire les modifications précédentes.

<Listing number="21-15" file-name="src/lib.rs" caption="Modifier `ThreadPool` pour contenir des instances de `Worker` au lieu de contenir directement des threads">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

<!--
We've changed the name of the field on `ThreadPool` from `threads` to `workers`
because it's now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.
-->

Nous avons changé le nom du champ sur `ThreadPool` de `threads` à `workers` car il contient maintenant des instances de `Worker` au lieu d'instances de `JoinHandle<()>`. Nous utilisons le compteur dans la boucle `for` comme argument pour `Worker::new`, et nous stockons chaque nouveau `Worker` dans le vecteur nommé `workers`.

<!--
External code (like our server in _src/main.rs_) doesn't need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that is created by spawning a new thread using an empty closure.
-->

Le code externe (comme notre serveur dans _src/main.rs_) n'a pas besoin de connaître les détails d'implémentation concernant l'utilisation d'une structure `Worker` au sein de `ThreadPool`, donc nous rendons la structure `Worker` et sa fonction `new` privées. La fonction `Worker::new` utilise l'`id` que nous lui donnons et stocke une instance de `JoinHandle<()>` qui est créée en lançant un nouveau thread avec une closure vide.

<!--
> Note: If the operating system can't create a thread because there aren't
> enough system resources, `thread::spawn` will panic. That will cause our
> whole server to panic, even though the creation of some threads might
> succeed. For simplicity's sake, this behavior is fine, but in a production
> thread pool implementation, you'd likely want to use
> `std::thread::Builder` and its `spawn` method that returns `Result` instead.
-->

> Remarque : si le système d'exploitation ne peut pas créer un thread parce qu'il n'y a pas assez de ressources système, `thread::spawn` paniquera. Cela fera paniquer tout notre serveur, même si la création de certains threads aurait pu réussir. Par souci de simplicité, ce comportement est acceptable, mais dans une implémentation de thread pool en production, vous voudriez probablement utiliser [`std::thread::Builder`][builder]<!--
ignore
--> et sa méthode [`spawn`][builder-spawn]<!--
ignore
--> qui retourne un `Result` à la place.

<!--
This code will compile and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`. But we're _still_ not processing
the closure that we get in `execute`. Let's look at how to do that next.
-->

Ce code compilera et stockera le nombre d'instances de `Worker` que nous avons spécifié comme argument de `ThreadPool::new`. Mais nous ne traitons _toujours_ pas la closure que nous recevons dans `execute`. Voyons comment faire cela ensuite.

<!--
#### Sending Requests to Threads via Channels
-->

#### Envoyer des requêtes aux threads via des canaux

<!--
The next problem we'll tackle is that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method. But we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.
-->

Le prochain problème que nous allons aborder est que les closures données à `thread::spawn` ne font absolument rien. Actuellement, nous recevons la closure que nous voulons exécuter dans la méthode `execute`. Mais nous devons donner à `thread::spawn` une closure à exécuter quand nous créons chaque `Worker` lors de la création du `ThreadPool`.

<!--
We want the `Worker` structs that we just created to fetch the code to run from
a queue held in the `ThreadPool` and send that code to its thread to run.
-->

Nous voulons que les structures `Worker` que nous venons de créer récupèrent le code à exécuter depuis une file d'attente détenue par le `ThreadPool` et envoient ce code à leur thread pour l'exécuter.

<!--
The channels we learned about in Chapter 16—a simple way to communicate between
two threads—would be perfect for this use case. We'll use a channel to function
as the queue of jobs, and `execute` will send a job from the `ThreadPool` to
the `Worker` instances, which will send the job to its thread. Here is the plan:

1. The `ThreadPool` will create a channel and hold on to the sender.
2. Each `Worker` will hold on to the receiver.
3. We'll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute through the
   sender.
5. In its thread, the `Worker` will loop over its receiver and execute the
   closures of any jobs it receives.
-->

Les canaux que nous avons appris au chapitre 16 -- un moyen simple de communiquer entre deux threads -- seraient parfaits pour ce cas d'utilisation. Nous utiliserons un canal pour fonctionner comme la file d'attente de tâches, et `execute` enverra une tâche du `ThreadPool` aux instances de `Worker`, qui enverront la tâche à leur thread. Voici le plan :

1. Le `ThreadPool` créera un canal et conservera l'émetteur (sender).
2. Chaque `Worker` conservera le récepteur (receiver).
3. Nous créerons une nouvelle structure `Job` qui contiendra les closures que nous voulons envoyer à travers le canal.
4. La méthode `execute` enverra la tâche qu'elle veut exécuter à travers l'émetteur.
5. Dans son thread, le `Worker` bouclera sur son récepteur et exécutera les closures de toutes les tâches qu'il reçoit.

<!--
Let's start by creating a channel in `ThreadPool::new` and holding the sender
in the `ThreadPool` instance, as shown in Listing 21-16. The `Job` struct
doesn't hold anything for now but will be the type of item we're sending down
the channel.
-->

Commençons par créer un canal dans `ThreadPool::new` et en conservant l'émetteur dans l'instance de `ThreadPool`, comme montré dans l'encart 21-16. La structure `Job` ne contient rien pour le moment mais sera le type d'élément que nous enverrons à travers le canal.

<Listing number="21-16" file-name="src/lib.rs" caption="Modifier `ThreadPool` pour stocker l'émetteur d'un canal qui transmet des instances de `Job`">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

<!--
In `ThreadPool::new`, we create our new channel and have the pool hold the
sender. This will successfully compile.
-->

Dans `ThreadPool::new`, nous créons notre nouveau canal et faisons en sorte que le pool conserve l'émetteur. Cela compilera avec succès.

<!--
Let's try passing a receiver of the channel into each `Worker` as the thread
pool creates the channel. We know we want to use the receiver in the thread that
the `Worker` instances spawn, so we'll reference the `receiver` parameter in the
closure. The code in Listing 21-17 won't quite compile yet.
-->

Essayons de passer un récepteur du canal à chaque `Worker` au moment où le thread pool crée le canal. Nous savons que nous voulons utiliser le récepteur dans le thread que les instances de `Worker` créent, donc nous référencerons le paramètre `receiver` dans la closure. Le code de l'encart 21-17 ne compilera pas tout à fait encore.

<Listing number="21-17" file-name="src/lib.rs" caption="Passer le récepteur à chaque `Worker`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

<!--
We've made some small and straightforward changes: We pass the receiver into
`Worker::new`, and then we use it inside the closure.
-->

Nous avons fait quelques petits changements simples : nous passons le récepteur à `Worker::new`, puis nous l'utilisons à l'intérieur de la closure.

<!--
When we try to check this code, we get this error:
-->

Quand nous essayons de vérifier ce code, nous obtenons cette erreur :


```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

<!--
The code is trying to pass `receiver` to multiple `Worker` instances. This
won't work, as you'll recall from Chapter 16: The channel implementation that
Rust provides is multiple _producer_, single _consumer_. This means we can't
just clone the consuming end of the channel to fix this code. We also don't
want to send a message multiple times to multiple consumers; we want one list
of messages with multiple `Worker` instances such that each message gets
processed once.
-->

Le code essaie de passer `receiver` à plusieurs instances de `Worker`. Cela ne fonctionnera pas, comme vous vous en souviendrez du chapitre 16 : l'implémentation de canal que Rust fournit est à _producteurs_ multiples, _consommateur_ unique. Cela signifie que nous ne pouvons pas simplement cloner l'extrémité consommatrice du canal pour corriger ce code. Nous ne voulons pas non plus envoyer un message plusieurs fois à plusieurs consommateurs ; nous voulons une liste de messages avec plusieurs instances de `Worker` de sorte que chaque message soit traité une seule fois.

<!--
Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`;
otherwise, we might get race conditions (as covered in Chapter 16).
-->

De plus, retirer une tâche de la file d'attente du canal implique de muter le `receiver`, donc les threads ont besoin d'un moyen sûr de partager et modifier `receiver` ; sinon, nous pourrions obtenir des conditions de concurrence (comme abordé au chapitre 16).

<!--
Recall the thread-safe smart pointers discussed in Chapter 16: To share
ownership across multiple threads and allow the threads to mutate the value, we
need to use `Arc<Mutex<T>>`. The `Arc` type will let multiple `Worker` instances
own the receiver, and `Mutex` will ensure that only one `Worker` gets a job from
the receiver at a time. Listing 21-18 shows the changes we need to make.
-->

Rappelez-vous les pointeurs intelligents thread-safe discutés au chapitre 16 : pour partager la propriété entre plusieurs threads et permettre aux threads de muter la valeur, nous devons utiliser `Arc<Mutex<T>>`. Le type `Arc` permettra à plusieurs instances de `Worker` de posséder le récepteur, et `Mutex` garantira qu'un seul `Worker` obtient une tâche du récepteur à la fois. L'encart 21-18 montre les changements que nous devons effectuer.

<Listing number="21-18" file-name="src/lib.rs" caption="Partager le récepteur entre les instances de `Worker` en utilisant `Arc` et `Mutex`">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

<!--
In `ThreadPool::new`, we put the receiver in an `Arc` and a `Mutex`. For each
new `Worker`, we clone the `Arc` to bump the reference count so that the
`Worker` instances can share ownership of the receiver.
-->

Dans `ThreadPool::new`, nous plaçons le récepteur dans un `Arc` et un `Mutex`. Pour chaque nouveau `Worker`, nous clonons l'`Arc` pour incrémenter le compteur de références afin que les instances de `Worker` puissent partager la propriété du récepteur.

<!--
With these changes, the code compiles! We're getting there!
-->

Avec ces changements, le code compile ! Nous y arrivons !

<!--
#### Implementing the `execute` Method
-->

#### Implémenter la méthode `execute`

<!--
Let's finally implement the `execute` method on `ThreadPool`. We'll also change
`Job` from a struct to a type alias for a trait object that holds the type of
closure that `execute` receives. As discussed in the ["Type Synonyms and Type
Aliases"][type-aliases] ignore
--> section in Chapter 20, type aliases
allow us to make long types shorter for ease of use. Look at Listing 21-19.
-->

Implémentons enfin la méthode `execute` sur `ThreadPool`. Nous allons aussi changer `Job` d'une structure vers un alias de type pour un objet trait qui contient le type de closure que `execute` reçoit. Comme discuté dans la section [« Synonymes de types et alias de types »][type-aliases]<!--
ignore
--> du chapitre 20, les alias de types nous permettent de raccourcir les types longs pour faciliter leur utilisation. Regardez l'encart 21-19.

<Listing number="21-19" file-name="src/lib.rs" caption="Créer un alias de type `Job` pour une `Box` qui contient chaque closure puis envoyer la tâche à travers le canal">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

<!--
After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We're calling `unwrap` on
`send` for the case that sending fails. This might happen if, for example, we
stop all our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, we can't stop our threads from
executing: Our threads continue executing as long as the pool exists. The
reason we use `unwrap` is that we know the failure case won't happen, but the
compiler doesn't know that.
-->

Après avoir créé une nouvelle instance de `Job` en utilisant la closure que nous recevons dans `execute`, nous envoyons cette tâche par l'extrémité émettrice du canal. Nous appelons `unwrap` sur `send` pour le cas où l'envoi échouerait. Cela pourrait se produire si, par exemple, nous arrêtions tous nos threads, ce qui signifierait que l'extrémité réceptrice a cessé de recevoir de nouveaux messages. Pour le moment, nous ne pouvons pas arrêter nos threads : nos threads continuent de s'exécuter tant que le pool existe. La raison pour laquelle nous utilisons `unwrap` est que nous savons que le cas d'échec ne se produira pas, mais le compilateur ne le sait pas.

<!--
But we're not quite done yet! In the `Worker`, our closure being passed to
`thread::spawn` still only _references_ the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job and running the job when it gets one. Let's make the change
shown in Listing 21-20 to `Worker::new`.
-->

Mais nous n'avons pas tout à fait fini ! Dans le `Worker`, notre closure passée à `thread::spawn` ne fait encore que _référencer_ l'extrémité réceptrice du canal. Au lieu de cela, nous avons besoin que la closure boucle indéfiniment, en demandant à l'extrémité réceptrice du canal une tâche et en exécutant la tâche quand elle en obtient une. Effectuons la modification montrée dans l'encart 21-20 sur `Worker::new`.

<Listing number="21-20" file-name="src/lib.rs" caption="Recevoir et exécuter les tâches dans le thread de l'instance de `Worker`">


```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

<!--
Here, we first call `lock` on the `receiver` to acquire the mutex, and then we
call `unwrap` to panic on any errors. Acquiring a lock might fail if the mutex
is in a _poisoned_ state, which can happen if some other thread panicked while
holding the lock rather than releasing the lock. In this situation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you.
-->

Ici, nous appelons d'abord `lock` sur le `receiver` pour acquérir le mutex, puis nous appelons `unwrap` pour paniquer en cas d'erreur. L'acquisition d'un verrou peut échouer si le mutex est dans un état _empoisonné_ (poisoned), ce qui peut se produire si un autre thread a paniqué tout en détenant le verrou au lieu de le libérer. Dans cette situation, appeler `unwrap` pour faire paniquer ce thread est l'action correcte à prendre. N'hésitez pas à remplacer cet `unwrap` par un `expect` avec un message d'erreur qui a du sens pour vous.

<!--
If we get the lock on the mutex, we call `recv` to receive a `Job` from the
channel. A final `unwrap` moves past any errors here as well, which might occur
if the thread holding the sender has shut down, similar to how the `send`
method returns `Err` if the receiver shuts down.
-->

Si nous obtenons le verrou sur le mutex, nous appelons `recv` pour recevoir un `Job` du canal. Un dernier `unwrap` passe outre toute erreur ici aussi, qui pourrait survenir si le thread détenant l'émetteur s'est arrêté, de manière similaire à la façon dont la méthode `send` retourne `Err` si le récepteur s'arrête.

<!--
The call to `recv` blocks, so if there is no job yet, the current thread will
wait until a job becomes available. The `Mutex<T>` ensures that only one
`Worker` thread at a time is trying to request a job.
-->

L'appel à `recv` bloque, donc s'il n'y a pas encore de tâche, le thread courant attendra qu'une tâche devienne disponible. Le `Mutex<T>` garantit qu'un seul thread `Worker` à la fois essaie de demander une tâche.

<!--
Our thread pool is now in a working state! Give it a `cargo run` and make some
requests:
-->

Notre thread pool est maintenant en état de fonctionnement ! Lancez un `cargo run` et faites quelques requêtes :

<!--
manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
--> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

<!--
Success! We now have a thread pool that executes connections asynchronously.
There are never more than four threads created, so our system won't get
overloaded if the server receives a lot of requests. If we make a request to
_/sleep_, the server will be able to serve other requests by having another
thread run them.
-->

Succès ! Nous avons maintenant un thread pool qui exécute les connexions de manière asynchrone. Il n'y a jamais plus de quatre threads créés, donc notre système ne sera pas surchargé si le serveur reçoit beaucoup de requêtes. Si nous faisons une requête vers _/sleep_, le serveur pourra servir d'autres requêtes en faisant exécuter celles-ci par un autre thread.

<!--
> Note: If you open _/sleep_ in multiple browser windows simultaneously, they
> might load one at a time in five-second intervals. Some web browsers execute
> multiple instances of the same request sequentially for caching reasons. This
> limitation is not caused by our web server.
-->

> Remarque : si vous ouvrez _/sleep_ dans plusieurs fenêtres de navigateur simultanément, elles pourraient se charger une à la fois à des intervalles de cinq secondes. Certains navigateurs web exécutent séquentiellement plusieurs instances de la même requête pour des raisons de mise en cache. Cette limitation n'est pas causée par notre serveur web.

<!--
This is a good time to pause and consider how the code in Listings 21-18, 21-19,
and 21-20 would be different if we were using futures instead of a closure for
the work to be done. What types would change? How would the method signatures be
different, if at all? What parts of the code would stay the same?
-->

C'est un bon moment pour faire une pause et considérer comment le code des encarts 21-18, 21-19 et 21-20 serait différent si nous utilisions des futures au lieu d'une closure pour le travail à effectuer. Quels types changeraient ? Comment les signatures des méthodes seraient-elles différentes, si elles l'étaient ? Quelles parties du code resteraient les mêmes ?

<!--
After learning about the `while let` loop in Chapter 17 and Chapter 19, you
might be wondering why we didn't write the `Worker` thread code as shown in
Listing 21-21.
-->

Après avoir appris la boucle `while let` aux chapitres 17 et 19, vous vous demandez peut-être pourquoi nous n'avons pas écrit le code du thread `Worker` comme montré dans l'encart 21-21.

<Listing number="21-21" file-name="src/lib.rs" caption="Une implémentation alternative de `Worker::new` utilisant `while let`">


```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

<!--
This code compiles and runs but doesn't result in the desired threading
behavior: A slow request will still cause other requests to wait to be
processed. The reason is somewhat subtle: The `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. At compile time, the borrow checker can then enforce the rule
that a resource guarded by a `Mutex` cannot be accessed unless we hold the
lock. However, this implementation can also result in the lock being held
longer than intended if we aren't mindful of the lifetime of the
`MutexGuard<T>`.
-->

Ce code compile et s'exécute mais ne produit pas le comportement de threading souhaité : une requête lente fera toujours attendre les autres requêtes pour être traitées. La raison est assez subtile : la structure `Mutex` n'a pas de méthode publique `unlock` car la propriété du verrou est basée sur la durée de vie du `MutexGuard<T>` au sein du `LockResult<MutexGuard<T>>` que la méthode `lock` retourne. Au moment de la compilation, le vérificateur d'emprunt peut alors appliquer la règle selon laquelle une ressource protégée par un `Mutex` ne peut pas être accédée à moins que nous détenions le verrou. Cependant, cette implémentation peut aussi faire en sorte que le verrou soit détenu plus longtemps que prévu si nous ne sommes pas attentifs à la durée de vie du `MutexGuard<T>`.

<!--
The code in Listing 21-20 that uses `let job =
receiver.lock().unwrap().recv().unwrap();` works because with `let`, any
temporary values used in the expression on the right-hand side of the equal
sign are immediately dropped when the `let` statement ends. However, `while
let` (and `if let` and `match`) does not drop temporary values until the end of
the associated block. In Listing 21-21, the lock remains held for the duration
of the call to `job()`, meaning other `Worker` instances cannot receive jobs.
-->

Le code de l'encart 21-20 qui utilise `let job = receiver.lock().unwrap().recv().unwrap();` fonctionne car avec `let`, toutes les valeurs temporaires utilisées dans l'expression du côté droit du signe égal sont immédiatement libérées quand l'instruction `let` se termine. Cependant, `while let` (et `if let` et `match`) ne libère pas les valeurs temporaires avant la fin du bloc associé. Dans l'encart 21-21, le verrou reste détenu pendant toute la durée de l'appel à `job()`, ce qui signifie que les autres instances de `Worker` ne peuvent pas recevoir de tâches.

[type-aliases]: ch20-03-advanced-types.html#type-synonyms-and-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[moving-out-of-closures]: ch13-01-closures.html#moving-captured-values-out-of-closures
[builder]: ../std/thread/struct.Builder.html
[builder-spawn]: ../std/thread/struct.Builder.html#method.spawn
