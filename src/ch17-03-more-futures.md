
<!--
Old headings. Do not remove or links may break.
-->

<a id="yielding"></a>

<!--
### Yielding Control to the Runtime
-->

### Céder le contrôle au runtime

<!--
Recall from the ["Our First Async Program"][async-program] ignore
-->
section that at each await point, Rust gives a runtime a chance to pause the
task and switch to another one if the future being awaited isn't ready. The
inverse is also true: Rust _only_ pauses async blocks and hands control back to
a runtime at an await point. Everything between await points is synchronous.
-->

Rappelez-vous de la section [« Notre premier programme async »][async-program]<!--
ignore
--> qu'à chaque point d'attente, Rust donne au runtime une chance de mettre en pause la tâche et de passer à une autre si la future attendue n'est pas prête. L'inverse est aussi vrai : Rust ne met en pause les blocs async et ne rend le contrôle au runtime _qu'à_ un point d'attente. Tout ce qui se trouve entre les points d'attente est synchrone.

<!--
That means if you do a bunch of work in an async block without an await point,
that future will block any other futures from making progress. You may sometimes
hear this referred to as one future _starving_ other futures. In some cases,
that may not be a big deal. However, if you are doing some kind of expensive
setup or long-running work, or if you have a future that will keep doing some
particular task indefinitely, you'll need to think about when and where to hand
control back to the runtime.
-->

Cela signifie que si vous effectuez beaucoup de travail dans un bloc async sans point d'attente, cette future bloquera toutes les autres futures et les empêchera de progresser. Vous entendrez parfois parler d'une future qui _affame_ les autres futures. Dans certains cas, cela peut ne pas être grave. Cependant, si vous effectuez une configuration coûteuse ou un travail de longue durée, ou si vous avez une future qui continuera à effectuer une tâche particulière indéfiniment, vous devrez réfléchir à quand et où rendre le contrôle au runtime.

<!--
Let's simulate a long-running operation to illustrate the starvation problem,
then explore how to solve it. Listing 17-14 introduces a `slow` function.
-->

Simulons une opération de longue durée pour illustrer le problème d'affamement, puis explorons comment le résoudre. L'encart 17-14 introduit une fonction `slow`.

<Listing number="17-14" caption="Utiliser `thread::sleep` pour simuler des opérations lentes" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

<!--
This code uses `std::thread::sleep` instead of `trpl::sleep` so that calling
`slow` will block the current thread for some number of milliseconds. We can
use `slow` to stand in for real-world operations that are both long-running and
blocking.
-->

Ce code utilise `std::thread::sleep` au lieu de `trpl::sleep` pour que l'appel à `slow` bloque le thread courant pendant un certain nombre de millisecondes. Nous pouvons utiliser `slow` pour représenter des opérations du monde réel qui sont à la fois longues et bloquantes.

<!--
In Listing 17-15, we use `slow` to emulate doing this kind of CPU-bound work in
a pair of futures.
-->

Dans l'encart 17-15, nous utilisons `slow` pour émuler ce type de travail limité par le CPU dans une paire de futures.

<Listing number="17-15" caption="Appel de la fonction `slow` pour simuler des opérations lentes" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

<!--
Each future hands control back to the runtime only _after_ carrying out a bunch
of slow operations. If you run this code, you will see this output:
-->

Chaque future ne rend le contrôle au runtime qu'_après_ avoir effectué une série d'opérations lentes. Si vous exécutez ce code, vous verrez cette sortie :

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

<!--
As with Listing 17-5 where we used `trpl::select` to race futures fetching two
URLs, `select` still finishes as soon as `a` is done. There's no interleaving
between the calls to `slow` in the two futures, though. The `a` future does all
of its work until the `trpl::sleep` call is awaited, then the `b` future does
all of its work until its own `trpl::sleep` call is awaited, and finally the
`a` future completes. To allow both futures to make progress between their slow
tasks, we need await points so we can hand control back to the runtime. That
means we need something we can await!
-->

Comme dans l'encart 17-5 où nous avons utilisé `trpl::select` pour mettre en compétition des futures récupérant deux URL, `select` se termine toujours dès que `a` est terminée. Il n'y a cependant pas d'entrelacement entre les appels à `slow` dans les deux futures. La future `a` fait tout son travail jusqu'à ce que l'appel `trpl::sleep` soit attendu, puis la future `b` fait tout son travail jusqu'à ce que son propre appel `trpl::sleep` soit attendu, et enfin la future `a` se termine. Pour permettre aux deux futures de progresser entre leurs tâches lentes, nous avons besoin de points d'attente pour pouvoir rendre le contrôle au runtime. Cela signifie que nous avons besoin de quelque chose que nous pouvons attendre !

<!--
We can already see this kind of handoff happening in Listing 17-15: if we
removed the `trpl::sleep` at the end of the `a` future, it would complete
without the `b` future running _at all_. Let's try using the `trpl::sleep`
function as a starting point for letting operations switch off making progress,
as shown in Listing 17-16.
-->

Nous pouvons déjà voir ce type de transfert se produire dans l'encart 17-15 : si nous supprimions le `trpl::sleep` à la fin de la future `a`, elle se terminerait sans que la future `b` ne s'exécute _du tout_. Essayons d'utiliser la fonction `trpl::sleep` comme point de départ pour laisser les opérations alterner leur progression, comme montré dans l'encart 17-16.

<Listing number="17-16" caption="Utiliser `trpl::sleep` pour laisser les opérations alterner leur progression" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

<!--
We've added `trpl::sleep` calls with await points between each call to `slow`.
Now the two futures' work is interleaved:
-->

Nous avons ajouté des appels `trpl::sleep` avec des points d'attente entre chaque appel à `slow`. Maintenant le travail des deux futures est entrelacé :

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

<!--
The `a` future still runs for a bit before handing off control to `b`, because
it calls `slow` before ever calling `trpl::sleep`, but after that the futures
swap back and forth each time one of them hits an await point. In this case, we
have done that after every call to `slow`, but we could break up the work in
whatever way makes the most sense to us.
-->

La future `a` s'exécute encore un peu avant de passer le contrôle à `b`, car elle appelle `slow` avant d'appeler `trpl::sleep`, mais après cela les futures alternent à chaque fois que l'une d'elles atteint un point d'attente. Dans ce cas, nous avons fait cela après chaque appel à `slow`, mais nous pourrions découper le travail de la manière qui a le plus de sens pour nous.

<!--
We don't really want to _sleep_ here, though: we want to make progress as fast
as we can. We just need to hand back control to the runtime. We can do that
directly, using the `trpl::yield_now` function. In Listing 17-17, we replace
all those `trpl::sleep` calls with `trpl::yield_now`.
-->

Nous ne voulons pas vraiment _dormir_ ici : nous voulons progresser aussi vite que possible. Nous avons juste besoin de rendre le contrôle au runtime. Nous pouvons le faire directement en utilisant la fonction `trpl::yield_now`. Dans l'encart 17-17, nous remplaçons tous ces appels `trpl::sleep` par `trpl::yield_now`.

<Listing number="17-17" caption="Utiliser `yield_now` pour laisser les opérations alterner leur progression" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

<!--
This code is both clearer about the actual intent and can be significantly
faster than using `sleep`, because timers such as the one used by `sleep` often
have limits on how granular they can be. The version of `sleep` we are using,
for example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are _fast_: they can do a
lot in one millisecond!
-->

Ce code est à la fois plus clair quant à l'intention réelle et peut être significativement plus rapide que l'utilisation de `sleep`, car les minuteries comme celle utilisée par `sleep` ont souvent des limites sur leur granularité. La version de `sleep` que nous utilisons, par exemple, dormira toujours pendant au moins une milliseconde, même si nous lui passons une `Duration` d'une nanoseconde. Encore une fois, les ordinateurs modernes sont _rapides_ : ils peuvent faire beaucoup en une milliseconde !

<!--
This means that async can be useful even for compute-bound tasks, depending on
what else your program is doing, because it provides a useful tool for
structuring the relationships between different parts of the program (but at a
cost of the overhead of the async state machine). This is a form of
_cooperative multitasking_, where each future has the power to determine when
it hands over control via await points. Each future therefore also has the
responsibility to avoid blocking for too long. In some Rust-based embedded
operating systems, this is the _only_ kind of multitasking!
-->

Cela signifie que l'async peut être utile même pour les tâches limitées par le calcul, en fonction de ce que fait d'autre votre programme, car il fournit un outil utile pour structurer les relations entre différentes parties du programme (mais au prix du surcoût de la machine à états async). C'est une forme de _multitâche coopératif_, où chaque future a le pouvoir de déterminer quand elle cède le contrôle via les points d'attente. Chaque future a donc aussi la responsabilité d'éviter de bloquer trop longtemps. Dans certains systèmes d'exploitation embarqués basés sur Rust, c'est le _seul_ type de multitâche !

<!--
In real-world code, you won't usually be alternating function calls with await
points on every single line, of course. While yielding control in this way is
relatively inexpensive, it's not free. In many cases, trying to break up a
compute-bound task might make it significantly slower, so sometimes it's better
for _overall_ performance to let an operation block briefly. Always
measure to see what your code's actual performance bottlenecks are. The
underlying dynamic is important to keep in mind, though, if you _are_ seeing a
lot of work happening in serial that you expected to happen concurrently!
-->

Dans le code du monde réel, vous n'alternerez bien sûr pas habituellement les appels de fonctions avec des points d'attente à chaque ligne. Bien que céder le contrôle de cette manière soit relativement peu coûteux, ce n'est pas gratuit. Dans de nombreux cas, essayer de découper une tâche limitée par le calcul pourrait la rendre significativement plus lente, donc parfois il est préférable pour les performances _globales_ de laisser une opération bloquer brièvement. Mesurez toujours pour voir quels sont les véritables goulots d'étranglement de performance de votre code. La dynamique sous-jacente est cependant importante à garder à l'esprit, si vous _constatez_ beaucoup de travail s'effectuant en série alors que vous vous attendiez à ce qu'il se fasse de manière concurrente !

<!--
### Building Our Own Async Abstractions
-->

### Construire nos propres abstractions async

<!--
We can also compose futures together to create new patterns. For example, we can
build a `timeout` function with async building blocks we already have. When
we're done, the result will be another building block we could use to create
still more async abstractions.
-->

Nous pouvons aussi composer des futures ensemble pour créer de nouveaux motifs. Par exemple, nous pouvons construire une fonction `timeout` avec les blocs de construction async que nous avons déjà. Quand nous aurons terminé, le résultat sera un autre bloc de construction que nous pourrons utiliser pour créer encore plus d'abstractions async.

<!--
Listing 17-18 shows how we would expect this `timeout` to work with a slow
future.
-->

L'encart 17-18 montre comment nous nous attendrions à ce que ce `timeout` fonctionne avec une future lente.

<Listing number="17-18" caption="Utiliser notre `timeout` imaginé pour exécuter une opération lente avec une limite de temps" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

<!--
Let's implement this! To begin, let's think about the API for `timeout`:
-->

Implémentons cela ! Pour commencer, réfléchissons à l'API de `timeout` :

<!--
- It needs to be an async function itself so we can await it.
- Its first parameter should be a future to run. We can make it generic to allow
  it to work with any future.
- Its second parameter will be the maximum time to wait. If we use a `Duration`,
  that will make it easy to pass along to `trpl::sleep`.
- It should return a `Result`. If the future completes successfully, the
  `Result` will be `Ok` with the value produced by the future. If the timeout
  elapses first, the `Result` will be `Err` with the duration that the timeout
  waited for.
-->

- Elle doit être une fonction async elle-même pour que nous puissions l'attendre.
- Son premier paramètre devrait être une future à exécuter. Nous pouvons la rendre générique pour qu'elle fonctionne avec n'importe quelle future.
- Son deuxième paramètre sera le temps maximum d'attente. Si nous utilisons une `Duration`, ce sera facile de la passer à `trpl::sleep`.
- Elle devrait retourner un `Result`. Si la future se termine avec succès, le `Result` sera `Ok` avec la valeur produite par la future. Si le timeout s'écoule en premier, le `Result` sera `Err` avec la durée pendant laquelle le timeout a attendu.

<!--
Listing 17-19 shows this declaration.
-->

L'encart 17-19 montre cette déclaration.

<!--
This is not tested because it intentionally does not compile.
-->

<Listing number="17-19" caption="Définition de la signature de `timeout`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

<!--
That satisfies our goals for the types. Now let's think about the _behavior_ we
need: we want to race the future passed in against the duration. We can use
`trpl::sleep` to make a timer future from the duration, and use `trpl::select`
to run that timer with the future the caller passes in.
-->

Cela satisfait nos objectifs pour les types. Maintenant, réfléchissons au _comportement_ dont nous avons besoin : nous voulons mettre en compétition la future passée en paramètre contre la durée. Nous pouvons utiliser `trpl::sleep` pour créer une future de minuterie à partir de la durée, et utiliser `trpl::select` pour exécuter cette minuterie avec la future que l'appelant passe.

<!--
In Listing 17-20, we implement `timeout` by matching on the result of awaiting
`trpl::select`.
-->

Dans l'encart 17-20, nous implémentons `timeout` en faisant un match sur le résultat de l'attente de `trpl::select`.

<Listing number="17-20" caption="Définition de `timeout` avec `select` et `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

<!--
The implementation of `trpl::select` is not fair: it always polls arguments in
the order in which they are passed (other `select` implementations will
randomly choose which argument to poll first). Thus, we pass `future_to_try` to
`select` first so it gets a chance to complete even if `max_time` is a very
short duration. If `future_to_try` finishes first, `select` will return `Left`
with the output from `future_to_try`. If `timer` finishes first, `select` will
return `Right` with the timer's output of `()`.
-->

L'implémentation de `trpl::select` n'est pas équitable : elle interroge toujours les arguments dans l'ordre dans lequel ils sont passés (d'autres implémentations de `select` choisiront aléatoirement quel argument interroger en premier). Ainsi, nous passons `future_to_try` à `select` en premier pour qu'elle ait une chance de se terminer même si `max_time` est une durée très courte. Si `future_to_try` se termine en premier, `select` retournera `Left` avec la sortie de `future_to_try`. Si `timer` se termine en premier, `select` retournera `Right` avec la sortie `()` de la minuterie.

<!--
If the `future_to_try` succeeds and we get a `Left(output)`, we return
`Ok(output)`. If the sleep timer elapses instead and we get a `Right(())`, we
ignore the `()` with `_` and return `Err(max_time)` instead.
-->

Si `future_to_try` réussit et que nous obtenons un `Left(output)`, nous retournons `Ok(output)`. Si la minuterie de sommeil s'écoule à la place et que nous obtenons un `Right(())`, nous ignorons le `()` avec `_` et retournons `Err(max_time)` à la place.

<!--
With that, we have a working `timeout` built out of two other async helpers. If
we run our code, it will print the failure mode after the timeout:
-->

Avec cela, nous avons un `timeout` fonctionnel construit à partir de deux autres aides async. Si nous exécutons notre code, il affichera le mode d'échec après le timeout :

```text
Failed after 2 seconds
```

<!--
Because futures compose with other futures, you can build really powerful tools
using smaller async building blocks. For example, you can use this same
approach to combine timeouts with retries, and in turn use those with
operations such as network calls (such as those in Listing 17-5).
-->

Comme les futures se composent avec d'autres futures, vous pouvez construire des outils vraiment puissants en utilisant de petits blocs de construction async. Par exemple, vous pouvez utiliser cette même approche pour combiner des timeouts avec des tentatives de reprise, et à leur tour les utiliser avec des opérations comme des appels réseau (comme ceux de l'encart 17-5).

<!--
In practice, you'll usually work directly with `async` and `await`, and
secondarily with functions such as `select` and macros such as the `join!`
macro to control how the outermost futures are executed.
-->

En pratique, vous travaillerez généralement directement avec `async` et `await`, et secondairement avec des fonctions comme `select` et des macros comme la macro `join!` pour contrôler comment les futures les plus extérieures sont exécutées.

<!--
We've now seen a number of ways to work with multiple futures at the same time.
Up next, we'll look at how we can work with multiple futures in a sequence over
time with _streams_.
-->

Nous avons maintenant vu plusieurs façons de travailler avec plusieurs futures en même temps. Ensuite, nous verrons comment nous pouvons travailler avec plusieurs futures en séquence au fil du temps avec les _streams_.

[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
