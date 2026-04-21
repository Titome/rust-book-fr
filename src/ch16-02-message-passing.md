<!--
Old headings. Do not remove or links may break.
-->

<a id="using-message-passing-to-transfer-data-between-threads"></a>

<!--
## Transfer Data Between Threads with Message Passing
-->

## Transférer des données entre les threads par passage de messages

<!--
One increasingly popular approach to ensuring safe concurrency is message
passing, where threads or actors communicate by sending each other messages
containing data. Here's the idea in a slogan from [the Go language documentation](https://golang.org/doc/effective_go.html#concurrency):
"Do not communicate by sharing memory; instead, share memory by communicating."
-->

Une approche de plus en plus populaire pour assurer une concurrence sûre est le passage de messages, où les threads ou les acteurs communiquent en s'envoyant mutuellement des messages contenant des données. Voici l'idée résumée dans un slogan de [la documentation du langage Go](https://golang.org/doc/effective_go.html#concurrency) : "Ne communiquez pas en partageant de la mémoire ; au contraire, partagez de la mémoire en communiquant."

<!--
To accomplish message-sending concurrency, Rust's standard library provides an
implementation of channels. A _channel_ is a general programming concept by
which data is sent from one thread to another.
-->

Pour accomplir la concurrence par envoi de messages, la bibliothèque standard de Rust fournit une implémentation de canaux (channels). Un _canal_ est un concept de programmation général par lequel des données sont envoyées d'un thread à un autre.

<!--
You can imagine a channel in programming as being like a directional channel of
water, such as a stream or a river. If you put something like a rubber duck
into a river, it will travel downstream to the end of the waterway.
-->

Vous pouvez imaginer un canal en programmation comme un canal d'eau directionnel, tel qu'un ruisseau ou une rivière. Si vous mettez quelque chose comme un canard en caoutchouc dans une rivière, il voyagera en aval jusqu'à la fin du cours d'eau.

<!--
A channel has two halves: a transmitter and a receiver. The transmitter half is
the upstream location where you put the rubber duck into the river, and the
receiver half is where the rubber duck ends up downstream. One part of your
code calls methods on the transmitter with the data you want to send, and
another part checks the receiving end for arriving messages. A channel is said
to be _closed_ if either the transmitter or receiver half is dropped.
-->

Un canal a deux moitiés : un émetteur (transmitter) et un récepteur (receiver). La moitié émettrice est l'emplacement en amont où vous mettez le canard en caoutchouc dans la rivière, et la moitié réceptrice est l'endroit où le canard en caoutchouc finit en aval. Une partie de votre code appelle des méthodes sur l'émetteur avec les données que vous voulez envoyer, et une autre partie vérifie l'extrémité réceptrice pour les messages arrivant. Un canal est dit _fermé_ si l'une ou l'autre des moitiés, émettrice ou réceptrice, est libérée.

<!--
Here, we'll work up to a program that has one thread to generate values and
send them down a channel, and another thread that will receive the values and
print them out. We'll be sending simple values between threads using a channel
to illustrate the feature. Once you're familiar with the technique, you could
use channels for any threads that need to communicate with each other, such as
a chat system or a system where many threads perform parts of a calculation and
send the parts to one thread that aggregates the results.
-->

Ici, nous allons construire un programme qui a un thread pour générer des valeurs et les envoyer dans un canal, et un autre thread qui recevra les valeurs et les affichera. Nous enverrons des valeurs simples entre les threads en utilisant un canal pour illustrer la fonctionnalité. Une fois que vous serez familier avec la technique, vous pourrez utiliser des canaux pour tous les threads qui doivent communiquer entre eux, comme un système de chat ou un système où de nombreux threads effectuent des parties d'un calcul et envoient les parties à un thread qui agrège les résultats.

<!--
First, in Listing 16-6, we'll create a channel but not do anything with it.
Note that this won't compile yet because Rust can't tell what type of values we
want to send over the channel.
-->

D'abord, dans l'encart 16-6, nous allons créer un canal mais ne rien faire avec. Notez que cela ne compilera pas encore car Rust ne peut pas déterminer quel type de valeurs nous voulons envoyer dans le canal.

<Listing number="16-6" file-name="src/main.rs" caption="Créer un canal et assigner les deux moitiés à `tx` et `rx`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

<!--
We create a new channel using the `mpsc::channel` function; `mpsc` stands for
_multiple producer, single consumer_. In short, the way Rust's standard library
implements channels means a channel can have multiple _sending_ ends that
produce values but only one _receiving_ end that consumes those values. Imagine
multiple streams flowing together into one big river: Everything sent down any
of the streams will end up in one river at the end. We'll start with a single
producer for now, but we'll add multiple producers when we get this example
working.
-->

Nous créons un nouveau canal en utilisant la fonction `mpsc::channel` ; `mpsc` signifie _multiple producer, single consumer_ (multiples producteurs, consommateur unique). En bref, la façon dont la bibliothèque standard de Rust implémente les canaux signifie qu'un canal peut avoir plusieurs extrémités _émettrices_ qui produisent des valeurs mais une seule extrémité _réceptrice_ qui consomme ces valeurs. Imaginez plusieurs ruisseaux se rejoignant dans une grande rivière : tout ce qui est envoyé dans n'importe lequel des ruisseaux finira dans une seule rivière à la fin. Nous commencerons avec un seul producteur pour l'instant, mais nous ajouterons plusieurs producteurs quand cet exemple fonctionnera.

<!--
The `mpsc::channel` function returns a tuple, the first element of which is the
sending end—the transmitter—and the second element of which is the receiving
end—the receiver. The abbreviations `tx` and `rx` are traditionally used in
many fields for _transmitter_ and _receiver_, respectively, so we name our
variables as such to indicate each end. We're using a `let` statement with a
pattern that destructures the tuples; we'll discuss the use of patterns in
`let` statements and destructuring in Chapter 19. For now, know that using a
`let` statement in this way is a convenient approach to extract the pieces of
the tuple returned by `mpsc::channel`.
-->

La fonction `mpsc::channel` retourne un tuple, dont le premier élément est l'extrémité émettrice -- l'émetteur -- et le second élément est l'extrémité réceptrice -- le récepteur. Les abréviations `tx` et `rx` sont traditionnellement utilisées dans de nombreux domaines pour _transmitter_ (émetteur) et _receiver_ (récepteur), respectivement, donc nous nommons nos variables ainsi pour indiquer chaque extrémité. Nous utilisons une instruction `let` avec un motif qui déstructure les tuples ; nous aborderons l'utilisation des motifs dans les instructions `let` et la déstructuration au chapitre 19. Pour l'instant, sachez qu'utiliser une instruction `let` de cette manière est une approche pratique pour extraire les éléments du tuple retourné par `mpsc::channel`.

<!--
Let's move the transmitting end into a spawned thread and have it send one
string so that the spawned thread is communicating with the main thread, as
shown in Listing 16-7. This is like putting a rubber duck in the river upstream
or sending a chat message from one thread to another.
-->

Déplaçons l'extrémité émettrice dans un thread créé et faisons-lui envoyer une chaîne de caractères pour que le thread créé communique avec le thread principal, comme montré dans l'encart 16-7. C'est comme mettre un canard en caoutchouc en amont de la rivière ou envoyer un message de chat d'un thread à un autre.

<Listing number="16-7" file-name="src/main.rs" caption='Déplacer `tx` dans un thread créé et envoyer `"hi"`'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

<!--
Again, we're using `thread::spawn` to create a new thread and then using `move`
to move `tx` into the closure so that the spawned thread owns `tx`. The spawned
thread needs to own the transmitter to be able to send messages through the
channel.
-->

Encore une fois, nous utilisons `thread::spawn` pour créer un nouveau thread puis `move` pour déplacer `tx` dans la closure afin que le thread créé possède `tx`. Le thread créé a besoin de posséder l'émetteur pour pouvoir envoyer des messages à travers le canal.

<!--
The transmitter has a `send` method that takes the value we want to send. The
`send` method returns a `Result<T, E>` type, so if the receiver has already
been dropped and there's nowhere to send a value, the send operation will
return an error. In this example, we're calling `unwrap` to panic in case of an
error. But in a real application, we would handle it properly: Return to
Chapter 9 to review strategies for proper error handling.
-->

L'émetteur a une méthode `send` qui prend la valeur que nous voulons envoyer. La méthode `send` retourne un type `Result<T, E>`, donc si le récepteur a déjà été libéré et qu'il n'y a nulle part où envoyer une valeur, l'opération d'envoi retournera une erreur. Dans cet exemple, nous appelons `unwrap` pour paniquer en cas d'erreur. Mais dans une vraie application, nous la gérerions correctement : retournez au chapitre 9 pour revoir les stratégies de gestion d'erreurs appropriées.

<!--
In Listing 16-8, we'll get the value from the receiver in the main thread. This
is like retrieving the rubber duck from the water at the end of the river or
receiving a chat message.
-->

Dans l'encart 16-8, nous récupérerons la valeur depuis le récepteur dans le thread principal. C'est comme récupérer le canard en caoutchouc de l'eau à la fin de la rivière ou recevoir un message de chat.

<Listing number="16-8" file-name="src/main.rs" caption='Recevoir la valeur `"hi"` dans le thread principal et l&apos;afficher'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

<!--
The receiver has two useful methods: `recv` and `try_recv`. We're using `recv`,
short for _receive_, which will block the main thread's execution and wait
until a value is sent down the channel. Once a value is sent, `recv` will
return it in a `Result<T, E>`. When the transmitter closes, `recv` will return
an error to signal that no more values will be coming.
-->

Le récepteur a deux méthodes utiles : `recv` et `try_recv`. Nous utilisons `recv`, abréviation de _receive_ (recevoir), qui bloquera l'exécution du thread principal et attendra jusqu'à ce qu'une valeur soit envoyée dans le canal. Une fois qu'une valeur est envoyée, `recv` la retournera dans un `Result<T, E>`. Quand l'émetteur se ferme, `recv` retournera une erreur pour signaler qu'il n'y aura plus de valeurs.

<!--
The `try_recv` method doesn't block, but will instead return a `Result<T, E>`
immediately: an `Ok` value holding a message if one is available and an `Err`
value if there aren't any messages this time. Using `try_recv` is useful if
this thread has other work to do while waiting for messages: We could write a
loop that calls `try_recv` every so often, handles a message if one is
available, and otherwise does other work for a little while until checking
again.
-->

La méthode `try_recv` ne bloque pas, mais retourne immédiatement un `Result<T, E>` : une valeur `Ok` contenant un message s'il y en a un disponible et une valeur `Err` s'il n'y a pas de messages cette fois. Utiliser `try_recv` est utile si ce thread a d'autres tâches à effectuer en attendant les messages : nous pourrions écrire une boucle qui appelle `try_recv` régulièrement, gère un message s'il y en a un disponible, et sinon fait d'autres tâches pendant un petit moment avant de vérifier à nouveau.

<!--
We've used `recv` in this example for simplicity; we don't have any other work
for the main thread to do other than wait for messages, so blocking the main
thread is appropriate.
-->

Nous avons utilisé `recv` dans cet exemple par simplicité ; nous n'avons pas d'autre travail à faire pour le thread principal que d'attendre des messages, donc bloquer le thread principal est approprié.

<!--
When we run the code in Listing 16-8, we'll see the value printed from the main
thread:
-->

Quand nous exécutons le code de l'encart 16-8, nous verrons la valeur affichée depuis le thread principal :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
Got: hi
```

<!--
Perfect!
-->

Parfait !

<!--
Old headings. Do not remove or links may break.
-->

<a id="channels-and-ownership-transference"></a>

<!--
### Transferring Ownership Through Channels
-->

### Transférer la possession via les canaux

<!--
The ownership rules play a vital role in message sending because they help you
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage of thinking about ownership throughout your Rust programs. Let's do
an experiment to show how channels and ownership work together to prevent
problems: We'll try to use a `val` value in the spawned thread _after_ we've
sent it down the channel. Try compiling the code in Listing 16-9 to see why
this code isn't allowed.
-->

Les règles de possession jouent un rôle vital dans l'envoi de messages car elles vous aident à écrire du code concurrent sûr. Prévenir les erreurs dans la programmation concurrente est l'avantage de penser à la possession tout au long de vos programmes Rust. Faisons une expérience pour montrer comment les canaux et la possession fonctionnent ensemble pour prévenir les problèmes : nous essaierons d'utiliser une valeur `val` dans le thread créé _après_ l'avoir envoyée dans le canal. Essayez de compiler le code de l'encart 16-9 pour voir pourquoi ce code n'est pas autorisé.

<Listing number="16-9" file-name="src/main.rs" caption="Tentative d'utilisation de `val` après l'avoir envoyé dans le canal">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

<!--
Here, we try to print `val` after we've sent it down the channel via `tx.send`.
Allowing this would be a bad idea: Once the value has been sent to another
thread, that thread could modify or drop it before we try to use the value
again. Potentially, the other thread's modifications could cause errors or
unexpected results due to inconsistent or nonexistent data. However, Rust gives
us an error if we try to compile the code in Listing 16-9:
-->

Ici, nous essayons d'afficher `val` après l'avoir envoyé dans le canal via `tx.send`. Permettre cela serait une mauvaise idée : une fois que la valeur a été envoyée à un autre thread, ce thread pourrait la modifier ou la libérer avant que nous essayions d'utiliser la valeur à nouveau. Potentiellement, les modifications de l'autre thread pourraient causer des erreurs ou des résultats inattendus en raison de données incohérentes ou inexistantes. Cependant, Rust nous donne une erreur si nous essayons de compiler le code de l'encart 16-9 :

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

<!--
Our concurrency mistake has caused a compile-time error. The `send` function
takes ownership of its parameter, and when the value is moved the receiver
takes ownership of it. This stops us from accidentally using the value again
after sending it; the ownership system checks that everything is okay.
-->

Notre erreur de concurrence a causé une erreur de compilation. La fonction `send` prend la possession de son paramètre, et quand la valeur est déplacée, le récepteur en prend la possession. Cela nous empêche d'utiliser accidentellement la valeur à nouveau après l'avoir envoyée ; le système de possession vérifie que tout est en ordre.

<!--
Old headings. Do not remove or links may break.
-->

<a id="sending-multiple-values-and-seeing-the-receiver-waiting"></a>

<!--
### Sending Multiple Values
-->

### Envoyer plusieurs valeurs

<!--
The code in Listing 16-8 compiled and ran, but it didn't clearly show us that
two separate threads were talking to each other over the channel.
-->

Le code de l'encart 16-8 a compilé et s'est exécuté, mais il ne nous a pas clairement montré que deux threads séparés communiquaient entre eux via le canal.

<!--
In Listing 16-10, we've made some modifications that will prove the code in
Listing 16-8 is running concurrently: The spawned thread will now send multiple
messages and pause for a second between each message.
-->

Dans l'encart 16-10, nous avons fait quelques modifications qui prouveront que le code de l'encart 16-8 s'exécute de manière concurrente : le thread créé enverra maintenant plusieurs messages et fera une pause d'une seconde entre chaque message.

<Listing number="16-10" file-name="src/main.rs" caption="Envoyer plusieurs messages et faire une pause entre chacun">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

<!--
This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
one second.
-->

Cette fois, le thread créé a un vecteur de chaînes de caractères que nous voulons envoyer au thread principal. Nous itérons dessus, en envoyant chacune individuellement, et faisons une pause entre chacune en appelant la fonction `thread::sleep` avec une valeur `Duration` d'une seconde.

<!--
In the main thread, we're not calling the `recv` function explicitly anymore:
Instead, we're treating `rx` as an iterator. For each value received, we're
printing it. When the channel is closed, iteration will end.
-->

Dans le thread principal, nous n'appelons plus la fonction `recv` explicitement : à la place, nous traitons `rx` comme un itérateur. Pour chaque valeur reçue, nous l'affichons. Quand le canal est fermé, l'itération se terminera.

<!--
When running the code in Listing 16-10, you should see the following output
with a one-second pause in between each line:
-->

Lors de l'exécution du code de l'encart 16-10, vous devriez voir la sortie suivante avec une pause d'une seconde entre chaque ligne :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
Got: hi
Got: from
Got: the
Got: thread
```

<!--
Because we don't have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.
-->

Comme nous n'avons aucun code qui fait des pauses ou des délais dans la boucle `for` du thread principal, nous pouvons en déduire que le thread principal attend de recevoir des valeurs du thread créé.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-multiple-producers-by-cloning-the-transmitter"></a>

<!--
### Creating Multiple Producers
-->

### Créer plusieurs producteurs

<!--
Earlier we mentioned that `mpsc` was an acronym for _multiple producer, single
consumer_. Let's put `mpsc` to use and expand the code in Listing 16-10 to
create multiple threads that all send values to the same receiver. We can do so
by cloning the transmitter, as shown in Listing 16-11.
-->

Plus tôt, nous avons mentionné que `mpsc` était un acronyme pour _multiple producer, single consumer_ (multiples producteurs, consommateur unique). Mettons `mpsc` en pratique et enrichissons le code de l'encart 16-10 pour créer plusieurs threads qui envoient tous des valeurs au même récepteur. Nous pouvons le faire en clonant l'émetteur, comme montré dans l'encart 16-11.

<Listing number="16-11" file-name="src/main.rs" caption="Envoyer plusieurs messages depuis plusieurs producteurs">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

<!--
This time, before we create the first spawned thread, we call `clone` on the
transmitter. This will give us a new transmitter we can pass to the first
spawned thread. We pass the original transmitter to a second spawned thread.
This gives us two threads, each sending different messages to the one receiver.
-->

Cette fois, avant de créer le premier thread, nous appelons `clone` sur l'émetteur. Cela nous donnera un nouvel émetteur que nous pouvons passer au premier thread créé. Nous passons l'émetteur original à un second thread créé. Cela nous donne deux threads, chacun envoyant des messages différents au même récepteur.

<!--
When you run the code, your output should look something like this:
-->

Quand vous exécutez le code, votre sortie devrait ressembler à ceci :

<!--
Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler
-->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

<!--
You might see the values in another order, depending on your system. This is
what makes concurrency interesting as well as difficult. If you experiment with
`thread::sleep`, giving it various values in the different threads, each run
will be more nondeterministic and create different output each time.
-->

Vous pourriez voir les valeurs dans un autre ordre, selon votre système. C'est ce qui rend la concurrence intéressante mais aussi difficile. Si vous expérimentez avec `thread::sleep`, en lui donnant des valeurs variées dans les différents threads, chaque exécution sera plus non déterministe et créera une sortie différente à chaque fois.

<!--
Now that we've looked at how channels work, let's look at a different method of
concurrency.
-->

Maintenant que nous avons vu comment les canaux fonctionnent, examinons une méthode différente de concurrence.
