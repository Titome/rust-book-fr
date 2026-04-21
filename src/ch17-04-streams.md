<!--
Old headings. Do not remove or links may break.
-->

<a id="streams"></a>

<!--
## Streams: Futures in Sequence
-->

## Streams : des futures en séquence

<!--
Recall how we used the receiver for our async channel earlier in this chapter
in the ["Message Passing"][17-02-messages] ignore
--> section. The async
`recv` method produces a sequence of items over time. This is an instance of a
much more general pattern known as a _stream_. Many concepts are naturally
represented as streams: items becoming available in a queue, chunks of data
being pulled incrementally from the filesystem when the full data set is too
large for the computer's memory, or data arriving over the network over time.
Because streams are futures, we can use them with any other kind of future and
combine them in interesting ways. For example, we can batch up events to avoid
triggering too many network calls, set timeouts on sequences of long-running
operations, or throttle user interface events to avoid doing needless work.
-->

Rappelez-vous comment nous avons utilisé le récepteur de notre canal async plus tôt dans ce chapitre dans la section [« Passage de messages »][17-02-messages]<!--
ignore
-->. La méthode async `recv` produit une séquence d'éléments au fil du temps. C'est une instance d'un motif beaucoup plus général connu sous le nom de _stream_. De nombreux concepts sont naturellement représentés sous forme de streams : des éléments devenant disponibles dans une file d'attente, des morceaux de données extraits progressivement du système de fichiers quand l'ensemble complet de données est trop volumineux pour la mémoire de l'ordinateur, ou des données arrivant par le réseau au fil du temps. Comme les streams sont des futures, nous pouvons les utiliser avec tout autre type de future et les combiner de manières intéressantes. Par exemple, nous pouvons regrouper des événements pour éviter de déclencher trop d'appels réseau, définir des timeouts sur des séquences d'opérations longues, ou limiter les événements de l'interface utilisateur pour éviter de faire un travail inutile.

<!--
We saw a sequence of items back in Chapter 13, when we looked at the Iterator
trait in ["The Iterator Trait and the `next` Method"][iterator-trait]
ignore
--> section, but there are two differences between iterators and the
async channel receiver. The first difference is time: iterators are
synchronous, while the channel receiver is asynchronous. The second difference
is the API. When working directly with `Iterator`, we call its synchronous
`next` method. With the `trpl::Receiver` stream in particular, we called an
asynchronous `recv` method instead. Otherwise, these APIs feel very similar,
and that similarity isn't a coincidence. A stream is like an asynchronous form
of iteration. Whereas the `trpl::Receiver` specifically waits to receive
messages, though, the general-purpose stream API is much broader: it provides
the next item the way `Iterator` does, but asynchronously.
-->

Nous avons vu une séquence d'éléments au chapitre 13, quand nous avons examiné le trait Iterator dans la section [« Le trait Iterator et la méthode `next` »][iterator-trait]<!--
ignore
-->, mais il y a deux différences entre les itérateurs et le récepteur de canal async. La première différence est le temps : les itérateurs sont synchrones, tandis que le récepteur de canal est asynchrone. La deuxième différence est l'API. En travaillant directement avec `Iterator`, nous appelons sa méthode synchrone `next`. Avec le stream `trpl::Receiver` en particulier, nous avons appelé une méthode asynchrone `recv` à la place. Sinon, ces API sont très similaires, et cette similarité n'est pas une coïncidence. Un stream est comme une forme asynchrone d'itération. Alors que `trpl::Receiver` attend spécifiquement de recevoir des messages, l'API de stream à usage général est beaucoup plus large : elle fournit l'élément suivant comme le fait `Iterator`, mais de manière asynchrone.

<!--
The similarity between iterators and streams in Rust means we can actually
create a stream from any iterator. As with an iterator, we can work with a
stream by calling its `next` method and then awaiting the output, as in Listing
17-21, which won't compile yet.
-->

La similarité entre les itérateurs et les streams en Rust signifie que nous pouvons réellement créer un stream à partir de n'importe quel itérateur. Comme avec un itérateur, nous pouvons travailler avec un stream en appelant sa méthode `next` puis en attendant la sortie, comme dans l'encart 17-21, qui ne compilera pas encore.

<Listing number="17-21" caption="Créer un stream à partir d'un itérateur et afficher ses valeurs" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

<!--
We start with an array of numbers, which we convert to an iterator and then
call `map` on to double all the values. Then we convert the iterator into a
stream using the `trpl::stream_from_iter` function. Next, we loop over the
items in the stream as they arrive with the `while let` loop.
-->

Nous commençons avec un tableau de nombres, que nous convertissons en itérateur puis sur lequel nous appelons `map` pour doubler toutes les valeurs. Ensuite, nous convertissons l'itérateur en stream en utilisant la fonction `trpl::stream_from_iter`. Puis, nous bouclons sur les éléments du stream à mesure qu'ils arrivent avec la boucle `while let`.

<!--
Unfortunately, when we try to run the code, it doesn't compile but instead
reports that there's no `next` method available:
-->

Malheureusement, quand nous essayons d'exécuter le code, il ne compile pas et signale qu'il n'y a pas de méthode `next` disponible :

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

<!--
As this output explains, the reason for the compiler error is that we need the
right trait in scope to be able to use the `next` method. Given our discussion
so far, you might reasonably expect that trait to be `Stream`, but it's
actually `StreamExt`. Short for _extension_, `Ext` is a common pattern in the
Rust community for extending one trait with another.
-->

Comme cette sortie l'explique, la raison de l'erreur du compilateur est que nous avons besoin du bon trait dans la portée pour pouvoir utiliser la méthode `next`. Compte tenu de notre discussion jusqu'à présent, vous pourriez raisonnablement vous attendre à ce que ce trait soit `Stream`, mais c'est en fait `StreamExt`. Abréviation d'_extension_, `Ext` est un motif courant dans la communauté Rust pour étendre un trait avec un autre.

<!--
The `Stream` trait defines a low-level interface that effectively combines the
`Iterator` and `Future` traits. `StreamExt` supplies a higher-level set of APIs
on top of `Stream`, including the `next` method as well as other utility
methods similar to those provided by the `Iterator` trait. `Stream` and
`StreamExt` are not yet part of Rust's standard library, but most ecosystem
crates use similar definitions.
-->

Le trait `Stream` définit une interface de bas niveau qui combine effectivement les traits `Iterator` et `Future`. `StreamExt` fournit un ensemble d'API de plus haut niveau par-dessus `Stream`, incluant la méthode `next` ainsi que d'autres méthodes utilitaires similaires à celles fournies par le trait `Iterator`. `Stream` et `StreamExt` ne font pas encore partie de la bibliothèque standard de Rust, mais la plupart des crates de l'écosystème utilisent des définitions similaires.

<!--
The fix to the compiler error is to add a `use` statement for
`trpl::StreamExt`, as in Listing 17-22.
-->

La solution à l'erreur du compilateur est d'ajouter une instruction `use` pour `trpl::StreamExt`, comme dans l'encart 17-22.

<Listing number="17-22" caption="Utiliser avec succès un itérateur comme base pour un stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

<!--
With all those pieces put together, this code works the way we want! What's
more, now that we have `StreamExt` in scope, we can use all of its utility
methods, just as with iterators.
-->

Avec tous ces éléments assemblés, ce code fonctionne comme nous le voulons ! De plus, maintenant que nous avons `StreamExt` dans la portée, nous pouvons utiliser toutes ses méthodes utilitaires, tout comme avec les itérateurs.

[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
