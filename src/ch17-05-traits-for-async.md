<!--
Old headings. Do not remove or links may break.
-->

<a id="digging-into-the-traits-for-async"></a>

<!--
## A Closer Look at the Traits for Async
-->

## Un regard plus approfondi sur les traits pour l'async

<!--
Throughout the chapter, we've used the `Future`, `Stream`, and `StreamExt`
traits in various ways. So far, though, we've avoided getting too far into the
details of how they work or how they fit together, which is fine most of the
time for your day-to-day Rust work. Sometimes, though, you'll encounter
situations where you'll need to understand a few more of these traits' details,
along with the `Pin` type and the `Unpin` trait. In this section, we'll dig in
just enough to help in those scenarios, still leaving the _really_ deep dive
for other documentation.
-->

Tout au long du chapitre, nous avons utilisé les traits `Future`, `Stream` et `StreamExt` de diverses manières. Jusqu'ici, cependant, nous avons évité d'entrer trop dans les détails de leur fonctionnement ou de la façon dont ils s'articulent ensemble, ce qui convient la plupart du temps pour votre travail Rust quotidien. Parfois, cependant, vous rencontrerez des situations où vous devrez comprendre quelques détails supplémentaires de ces traits, ainsi que le type `Pin` et le trait `Unpin`. Dans cette section, nous approfondirons juste assez pour aider dans ces scénarios, en laissant la plongée _vraiment_ profonde à d'autre documentation.

<!--
Old headings. Do not remove or links may break.
-->

<a id="future"></a>

<!--
### The `Future` Trait
-->

### Le trait `Future`

<!--
Let's start by taking a closer look at how the `Future` trait works. Here's how
Rust defines it:
-->

Commençons par regarder de plus près comment fonctionne le trait `Future`. Voici comment Rust le définit :

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

<!--
That trait definition includes a bunch of new types and also some syntax we
haven't seen before, so let's walk through the definition piece by piece.
-->

Cette définition de trait inclut un tas de nouveaux types et aussi une syntaxe que nous n'avons pas vue auparavant, alors parcourons la définition morceau par morceau.

<!--
First, `Future`'s associated type `Output` says what the future resolves to.
This is analogous to the `Item` associated type for the `Iterator` trait.
Second, `Future` has the `poll` method, which takes a special `Pin` reference
for its `self` parameter and a mutable reference to a `Context` type, and
returns a `Poll<Self::Output>`. We'll talk more about `Pin` and `Context` in a
moment. For now, let's focus on what the method returns, the `Poll` type:
-->

Premièrement, le type associé `Output` de `Future` indique en quoi la future se résout. C'est analogue au type associé `Item` du trait `Iterator`. Deuxièmement, `Future` a la méthode `poll`, qui prend une référence spéciale `Pin` pour son paramètre `self` et une référence mutable vers un type `Context`, et retourne un `Poll<Self::Output>`. Nous parlerons davantage de `Pin` et de `Context` dans un moment. Pour l'instant, concentrons-nous sur ce que la méthode retourne, le type `Poll` :

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

<!--
This `Poll` type is similar to an `Option`. It has one variant that has a value,
`Ready(T)`, and one that does not, `Pending`. `Poll` means something quite
different from `Option`, though! The `Pending` variant indicates that the future
still has work to do, so the caller will need to check again later. The `Ready`
variant indicates that the `Future` has finished its work and the `T` value is
available.
-->

Ce type `Poll` est similaire à une `Option`. Il a une variante qui contient une valeur, `Ready(T)`, et une qui n'en contient pas, `Pending`. Cependant, `Poll` signifie quelque chose de très différent d'`Option` ! La variante `Pending` indique que la future a encore du travail à faire, donc l'appelant devra vérifier à nouveau plus tard. La variante `Ready` indique que la `Future` a terminé son travail et que la valeur `T` est disponible.

<!--
> Note: It's rare to need to call `poll` directly, but if you do need to, keep
> in mind that with most futures, the caller should not call `poll` again after
> the future has returned `Ready`. Many futures will panic if polled again after
> becoming ready. Futures that are safe to poll again will say so explicitly in
> their documentation. This is similar to how `Iterator::next` behaves.
-->

> Remarque : il est rare d'avoir besoin d'appeler `poll` directement, mais si vous devez le faire, gardez à l'esprit qu'avec la plupart des futures, l'appelant ne devrait pas appeler `poll` à nouveau après que la future a retourné `Ready`. Beaucoup de futures paniqueront si elles sont interrogées à nouveau après être devenues prêtes. Les futures qui peuvent être interrogées à nouveau en toute sécurité le diront explicitement dans leur documentation. C'est similaire au comportement de `Iterator::next`.

<!--
When you see code that uses `await`, Rust compiles it under the hood to code
that calls `poll`. If you look back at Listing 17-4, where we printed out the
page title for a single URL once it resolved, Rust compiles it into something
kind of (although not exactly) like this:
-->

Quand vous voyez du code qui utilise `await`, Rust le compile en coulisses en code qui appelle `poll`. Si vous revenez à l'encart 17-4, où nous avons affiché le titre de la page pour une seule URL une fois qu'elle s'est résolue, Rust le compile en quelque chose qui ressemble à peu près (bien que pas exactement) à ceci :

```rust,ignore
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
    }
}
```

<!--
What should we do when the future is still `Pending`? We need some way to try
again, and again, and again, until the future is finally ready. In other words,
we need a loop:
-->

Que devons-nous faire quand la future est encore `Pending` ? Nous avons besoin d'un moyen de réessayer, encore et encore, jusqu'à ce que la future soit enfin prête. En d'autres termes, nous avons besoin d'une boucle :

```rust,ignore
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continue
        }
    }
}
```

<!--
If Rust compiled it to exactly that code, though, every `await` would be
blocking—exactly the opposite of what we were going for! Instead, Rust ensures
that the loop can hand off control to something that can pause work on this
future to work on other futures and then check this one again later. As we've
seen, that something is an async runtime, and this scheduling and coordination
work is one of its main jobs.
-->

Si Rust le compilait exactement en ce code, cependant, chaque `await` serait bloquant — exactement le contraire de ce que nous recherchions ! À la place, Rust s'assure que la boucle peut passer le contrôle à quelque chose qui peut mettre en pause le travail sur cette future pour travailler sur d'autres futures puis revérifier celle-ci plus tard. Comme nous l'avons vu, ce quelque chose est un runtime async, et ce travail de planification et de coordination est l'un de ses principaux rôles.

<!--
In the ["Sending Data Between Two Tasks Using Message
Passing"][message-passing] ignore
--> section, we described waiting on
`rx.recv`. The `recv` call returns a future, and awaiting the future polls it.
We noted that a runtime will pause the future until it's ready with either
`Some(message)` or `None` when the channel closes. With our deeper
understanding of the `Future` trait, and specifically `Future::poll`, we can
see how that works. The runtime knows the future isn't ready when it returns
`Poll::Pending`. Conversely, the runtime knows the future _is_ ready and
advances it when `poll` returns `Poll::Ready(Some(message))` or
`Poll::Ready(None)`.
-->

Dans la section [« Envoyer des données entre deux tâches en utilisant le passage de messages »][message-passing]<!--
ignore
-->, nous avons décrit l'attente sur `rx.recv`. L'appel `recv` retourne une future, et attendre la future l'interroge. Nous avons noté qu'un runtime mettra en pause la future jusqu'à ce qu'elle soit prête avec soit `Some(message)` soit `None` quand le canal se ferme. Avec notre compréhension plus profonde du trait `Future`, et spécifiquement de `Future::poll`, nous pouvons voir comment cela fonctionne. Le runtime sait que la future n'est pas prête quand elle retourne `Poll::Pending`. Inversement, le runtime sait que la future _est_ prête et la fait avancer quand `poll` retourne `Poll::Ready(Some(message))` ou `Poll::Ready(None)`.

<!--
The exact details of how a runtime does that are beyond the scope of this book,
but the key is to see the basic mechanics of futures: a runtime _polls_ each
future it is responsible for, putting the future back to sleep when it is not
yet ready.
-->

Les détails exacts de la façon dont un runtime fait cela dépassent le cadre de ce livre, mais l'essentiel est de voir les mécanismes de base des futures : un runtime _interroge_ chaque future dont il est responsable, remettant la future en sommeil quand elle n'est pas encore prête.

<!--
Old headings. Do not remove or links may break.
-->

<a id="pinning-and-the-pin-and-unpin-traits"></a>
<a id="the-pin-and-unpin-traits"></a>

<!--
### The `Pin` Type and the `Unpin` Trait
-->

### Le type `Pin` et le trait `Unpin`

<!--
Back in Listing 17-13, we used the `trpl::join!` macro to await three
futures. However, it's common to have a collection such as a vector containing
some number futures that won't be known until runtime. Let's change Listing
17-13 to the code in Listing 17-23 that puts the three futures into a vector
and calls the `trpl::join_all` function instead, which won't compile yet.
-->

Dans l'encart 17-13, nous avons utilisé la macro `trpl::join!` pour attendre trois futures. Cependant, il est courant d'avoir une collection comme un vecteur contenant un certain nombre de futures qui ne sera pas connu avant l'exécution. Modifions l'encart 17-13 en le code de l'encart 17-23 qui met les trois futures dans un vecteur et appelle la fonction `trpl::join_all` à la place, ce qui ne compilera pas encore.

<Listing number="17-23" caption="Attendre des futures dans une collection" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:here}}
```

</Listing>

<!--
We put each future within a `Box` to make them into _trait objects_, just as
we did in the "Returning Errors from `run`" section in Chapter 12. (We'll cover
trait objects in detail in Chapter 18.) Using trait objects lets us treat each
of the anonymous futures produced by these types as the same type, because all
of them implement the `Future` trait.
-->

Nous mettons chaque future dans un `Box` pour en faire des _objets trait_, comme nous l'avons fait dans la section « Retourner les erreurs depuis `run` » au chapitre 12. (Nous couvrirons les objets trait en détail au chapitre 18.) Utiliser des objets trait nous permet de traiter chacune des futures anonymes produites par ces types comme le même type, car elles implémentent toutes le trait `Future`.

<!--
This might be surprising. After all, none of the async blocks returns anything,
so each one produces a `Future<Output = ()>`. Remember that `Future` is a
trait, though, and that the compiler creates a unique enum for each async
block, even when they have identical output types. Just as you can't put two
different handwritten structs in a `Vec`, you can't mix compiler-generated
enums.
-->

Cela peut être surprenant. Après tout, aucun des blocs async ne retourne quoi que ce soit, donc chacun produit une `Future<Output = ()>`. Rappelez-vous cependant que `Future` est un trait, et que le compilateur crée un enum unique pour chaque bloc async, même quand ils ont des types de sortie identiques. Tout comme vous ne pouvez pas mettre deux structs différentes écrites à la main dans un `Vec`, vous ne pouvez pas mélanger des enums générés par le compilateur.

<!--
Then we pass the collection of futures to the `trpl::join_all` function and
await the result. However, this doesn't compile; here's the relevant part of
the error messages.
-->

Ensuite, nous passons la collection de futures à la fonction `trpl::join_all` et attendons le résultat. Cependant, cela ne compile pas ; voici la partie pertinente des messages d'erreur.

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-23
cargo build
copy *only* the final `error` block from the errors
-->

```text
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:48:33
   |
48 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

<!--
The note in this error message tells us that we should use the `pin!` macro to
_pin_ the values, which means putting them inside the `Pin` type that
guarantees the values won't be moved in memory. The error message says pinning
is required because `dyn Future<Output = ()>` needs to implement the `Unpin`
trait and it currently does not.
-->

La note dans ce message d'erreur nous dit que nous devrions utiliser la macro `pin!` pour _épingler_ les valeurs, ce qui signifie les mettre à l'intérieur du type `Pin` qui garantit que les valeurs ne seront pas déplacées en mémoire. Le message d'erreur dit que l'épinglage est nécessaire car `dyn Future<Output = ()>` doit implémenter le trait `Unpin` et ne le fait pas actuellement.

<!--
The `trpl::join_all` function returns a struct called `JoinAll`. That struct is
generic over a type `F`, which is constrained to implement the `Future` trait.
Directly awaiting a future with `await` pins the future implicitly. That's why
we don't need to use `pin!` everywhere we want to await futures.
-->

La fonction `trpl::join_all` retourne une struct appelée `JoinAll`. Cette struct est générique sur un type `F`, qui est contraint d'implémenter le trait `Future`. Attendre directement une future avec `await` épingle la future implicitement. C'est pourquoi nous n'avons pas besoin d'utiliser `pin!` partout où nous voulons attendre des futures.

<!--
However, we're not directly awaiting a future here. Instead, we construct a new
future, JoinAll, by passing a collection of futures to the `join_all` function.
The signature for `join_all` requires that the types of the items in the
collection all implement the `Future` trait, and `Box<T>` implements `Future`
only if the `T` it wraps is a future that implements the `Unpin` trait.
-->

Cependant, nous n'attendons pas directement une future ici. À la place, nous construisons une nouvelle future, JoinAll, en passant une collection de futures à la fonction `join_all`. La signature de `join_all` requiert que les types des éléments de la collection implémentent tous le trait `Future`, et `Box<T>` n'implémente `Future` que si le `T` qu'il encapsule est une future qui implémente le trait `Unpin`.

<!--
That's a lot to absorb! To really understand it, let's dive a little further
into how the `Future` trait actually works, in particular around pinning. Look
again at the definition of the `Future` trait:
-->

C'est beaucoup à absorber ! Pour vraiment comprendre, plongeons un peu plus dans le fonctionnement réel du trait `Future`, en particulier autour de l'épinglage. Regardez à nouveau la définition du trait `Future` :

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

<!--
The `cx` parameter and its `Context` type are the key to how a runtime actually
knows when to check any given future while still being lazy. Again, the details
of how that works are beyond the scope of this chapter, and you generally only
need to think about this when writing a custom `Future` implementation. We'll
focus instead on the type for `self`, as this is the first time we've seen a
method where `self` has a type annotation. A type annotation for `self` works
like type annotations for other function parameters but with two key
differences:
-->

Le paramètre `cx` et son type `Context` sont la clé pour comprendre comment un runtime sait réellement quand vérifier une future donnée tout en restant paresseux. Encore une fois, les détails de ce fonctionnement dépassent le cadre de ce chapitre, et vous n'avez généralement besoin d'y penser que lorsque vous écrivez une implémentation personnalisée de `Future`. Nous nous concentrerons plutôt sur le type de `self`, car c'est la première fois que nous voyons une méthode où `self` a une annotation de type. Une annotation de type pour `self` fonctionne comme les annotations de type pour d'autres paramètres de fonction mais avec deux différences clés :

<!--
- It tells Rust what type `self` must be for the method to be called.
- It can't be just any type. It's restricted to the type on which the method is
  implemented, a reference or smart pointer to that type, or a `Pin` wrapping a
  reference to that type.
-->

- Elle indique à Rust quel type `self` doit avoir pour que la méthode puisse être appelée.
- Ce ne peut pas être n'importe quel type. C'est restreint au type sur lequel la méthode est implémentée, une référence ou un pointeur intelligent vers ce type, ou un `Pin` encapsulant une référence vers ce type.

<!--
We'll see more on this syntax in [Chapter 18][ch-18] ignore
-->. For now,
it's enough to know that if we want to poll a future to check whether it is
`Pending` or `Ready(Output)`, we need a `Pin`-wrapped mutable reference to the
type.
-->

Nous en verrons plus sur cette syntaxe au [chapitre 18][ch-18]<!--
ignore
-->. Pour l'instant, il suffit de savoir que si nous voulons interroger une future pour vérifier si elle est `Pending` ou `Ready(Output)`, nous avons besoin d'une référence mutable encapsulée dans `Pin` vers le type.

<!--
`Pin` is a wrapper for pointer-like types such as `&`, `&mut`, `Box`, and `Rc`.
(Technically, `Pin` works with types that implement the `Deref` or `DerefMut`
traits, but this is effectively equivalent to working only with references and
smart pointers.) `Pin` is not a pointer itself and doesn't have any behavior of
its own like `Rc` and `Arc` do with reference counting; it's purely a tool the
compiler can use to enforce constraints on pointer usage.
-->

`Pin` est un wrapper pour les types semblables à des pointeurs comme `&`, `&mut`, `Box` et `Rc`. (Techniquement, `Pin` fonctionne avec les types qui implémentent les traits `Deref` ou `DerefMut`, mais c'est effectivement équivalent à ne travailler qu'avec des références et des pointeurs intelligents.) `Pin` n'est pas un pointeur lui-même et n'a aucun comportement propre comme `Rc` et `Arc` avec le comptage de références ; c'est purement un outil que le compilateur peut utiliser pour imposer des contraintes sur l'utilisation des pointeurs.

<!--
Recalling that `await` is implemented in terms of calls to `poll` starts to
explain the error message we saw earlier, but that was in terms of `Unpin`, not
`Pin`. So how exactly does `Pin` relate to `Unpin`, and why does `Future` need
`self` to be in a `Pin` type to call `poll`?
-->

Se rappeler que `await` est implémenté en termes d'appels à `poll` commence à expliquer le message d'erreur que nous avons vu plus tôt, mais c'était en termes d'`Unpin`, pas de `Pin`. Alors comment exactement `Pin` est-il lié à `Unpin`, et pourquoi `Future` a-t-il besoin que `self` soit dans un type `Pin` pour appeler `poll` ?

<!--
Remember from earlier in this chapter that a series of await points in a future
get compiled into a state machine, and the compiler makes sure that state
machine follows all of Rust's normal rules around safety, including borrowing
and ownership. To make that work, Rust looks at what data is needed between one
await point and either the next await point or the end of the async block. It
then creates a corresponding variant in the compiled state machine. Each
variant gets the access it needs to the data that will be used in that section
of the source code, whether by taking ownership of that data or by getting a
mutable or immutable reference to it.
-->

Rappelez-vous du début de ce chapitre qu'une série de points d'attente dans une future est compilée en une machine à états, et le compilateur s'assure que cette machine à états respecte toutes les règles normales de Rust concernant la sécurité, y compris l'emprunt et la possession. Pour que cela fonctionne, Rust regarde quelles données sont nécessaires entre un point d'attente et soit le point d'attente suivant, soit la fin du bloc async. Il crée ensuite une variante correspondante dans la machine à états compilée. Chaque variante obtient l'accès dont elle a besoin aux données qui seront utilisées dans cette section du code source, que ce soit en prenant possession de ces données ou en obtenant une référence mutable ou immutable vers celles-ci.

<!--
So far, so good: if we get anything wrong about the ownership or references in
a given async block, the borrow checker will tell us. When we want to move
around the future that corresponds to that block—like moving it into a `Vec` to
pass to `join_all`—things get trickier.
-->

Jusqu'ici, tout va bien : si nous faisons quoi que ce soit de mal avec la possession ou les références dans un bloc async donné, le vérificateur d'emprunts nous le dira. Quand nous voulons déplacer la future qui correspond à ce bloc — comme la déplacer dans un `Vec` pour la passer à `join_all` — les choses se compliquent.

<!--
When we move a future—whether by pushing it into a data structure to use as an
iterator with `join_all` or by returning it from a function—that actually means
moving the state machine Rust creates for us. And unlike most other types in
Rust, the futures Rust creates for async blocks can end up with references to
themselves in the fields of any given variant, as shown in the simplified illustration in Figure 17-4.
-->

Quand nous déplaçons une future — que ce soit en la poussant dans une structure de données pour l'utiliser comme itérateur avec `join_all` ou en la retournant depuis une fonction — cela signifie en fait déplacer la machine à états que Rust crée pour nous. Et contrairement à la plupart des autres types en Rust, les futures que Rust crée pour les blocs async peuvent se retrouver avec des références vers elles-mêmes dans les champs de n'importe quelle variante donnée, comme montré dans l'illustration simplifiée de la figure 17-4.

<figure>

<img alt="A single-column, three-row table representing a future, fut1, which has data values 0 and 1 in the first two rows and an arrow pointing from the third row back to the second row, representing an internal reference within the future." src="img/trpl17-04.svg" class="center" />

<!--
<figcaption>Figure 17-4: A self-referential data type</figcaption>
-->

<figcaption>Figure 17-4 : Un type de données auto-référentiel</figcaption>

</figure>

<!--
By default, though, any object that has a reference to itself is unsafe to move,
because references always point to the actual memory address of whatever they
refer to (see Figure 17-5). If you move the data structure itself, those
internal references will be left pointing to the old location. However, that
memory location is now invalid. For one thing, its value will not be updated
when you make changes to the data structure. For another—more important—thing,
the computer is now free to reuse that memory for other purposes! You could end
up reading completely unrelated data later.
-->

Par défaut, cependant, tout objet qui a une référence vers lui-même est dangereux à déplacer, car les références pointent toujours vers l'adresse mémoire réelle de ce qu'elles référencent (voir la figure 17-5). Si vous déplacez la structure de données elle-même, ces références internes continueront de pointer vers l'ancien emplacement. Cependant, cet emplacement mémoire est maintenant invalide. D'une part, sa valeur ne sera pas mise à jour quand vous apporterez des modifications à la structure de données. D'autre part — et c'est plus important — l'ordinateur est maintenant libre de réutiliser cette mémoire à d'autres fins ! Vous pourriez finir par lire des données complètement sans rapport plus tard.

<figure>

<img alt="Two tables, depicting two futures, fut1 and fut2, each of which has one column and three rows, representing the result of having moved a future out of fut1 into fut2. The first, fut1, is grayed out, with a question mark in each index, representing unknown memory. The second, fut2, has 0 and 1 in the first and second rows and an arrow pointing from its third row back to the second row of fut1, representing a pointer that is referencing the old location in memory of the future before it was moved." src="img/trpl17-05.svg" class="center" />

<!--
<figcaption>Figure 17-5: The unsafe result of moving a self-referential data type</figcaption>
-->

<figcaption>Figure 17-5 : Le résultat dangereux du déplacement d'un type de données auto-référentiel</figcaption>

</figure>

<!--
Theoretically, the Rust compiler could try to update every reference to an
object whenever it gets moved, but that could add a lot of performance overhead,
especially if a whole web of references needs updating. If we could instead make
sure the data structure in question _doesn't move in memory_, we wouldn't have
to update any references. This is exactly what Rust's borrow checker is for:
in safe code, it prevents you from moving any item with an active reference to
it.
-->

Théoriquement, le compilateur Rust pourrait essayer de mettre à jour chaque référence vers un objet chaque fois qu'il est déplacé, mais cela pourrait ajouter beaucoup de surcoût en performance, surtout si tout un réseau de références doit être mis à jour. Si nous pouvions plutôt nous assurer que la structure de données en question _ne se déplace pas en mémoire_, nous n'aurions pas besoin de mettre à jour les références. C'est exactement le rôle du vérificateur d'emprunts de Rust : en code sûr, il vous empêche de déplacer tout élément qui a une référence active vers lui.

<!--
`Pin` builds on that to give us the exact guarantee we need. When we _pin_ a
value by wrapping a pointer to that value in `Pin`, it can no longer move. Thus,
if you have `Pin<Box<SomeType>>`, you actually pin the `SomeType` value, _not_
the `Box` pointer. Figure 17-6 illustrates this process.
-->

`Pin` s'appuie sur cela pour nous donner exactement la garantie dont nous avons besoin. Quand nous _épinglons_ une valeur en encapsulant un pointeur vers cette valeur dans `Pin`, elle ne peut plus se déplacer. Ainsi, si vous avez `Pin<Box<SomeType>>`, vous épinglez en fait la valeur `SomeType`, _pas_ le pointeur `Box`. La figure 17-6 illustre ce processus.

<figure>

<img alt="Three boxes laid out side by side. The first is labeled "Pin", the second "b1", and the third "pinned". Within "pinned" is a table labeled "fut", with a single column; it represents a future with cells for each part of the data structure. Its first cell has the value "0", its second cell has an arrow coming out of it and pointing to the fourth and final cell, which has the value "1" in it, and the third cell has dashed lines and an ellipsis to indicate there may be other parts to the data structure. All together, the "fut" table represents a future which is self-referential. An arrow leaves the box labeled "Pin", goes through the box labeled "b1" and terminates inside the "pinned" box at the "fut" table." src="img/trpl17-06.svg" class="center" />

<!--
<figcaption>Figure 17-6: Pinning a `Box` that points to a self-referential future type</figcaption>
-->

<figcaption>Figure 17-6 : Épingler un `Box` qui pointe vers un type de future auto-référentiel</figcaption>

</figure>

<!--
In fact, the `Box` pointer can still move around freely. Remember: we care about
making sure the data ultimately being referenced stays in place. If a pointer
moves around, _but the data it points to_ is in the same place, as in Figure
17-7, there's no potential problem. (As an independent exercise, look at the docs
for the types as well as the `std::pin` module and try to work out how you'd do
this with a `Pin` wrapping a `Box`.) The key is that the self-referential type
itself cannot move, because it is still pinned.
-->

En fait, le pointeur `Box` peut toujours se déplacer librement. Rappelez-vous : ce qui nous importe, c'est de nous assurer que les données finalement référencées restent en place. Si un pointeur se déplace, _mais les données vers lesquelles il pointe_ sont au même endroit, comme dans la figure 17-7, il n'y a pas de problème potentiel. (Comme exercice indépendant, regardez la documentation des types ainsi que le module `std::pin` et essayez de déterminer comment vous feriez cela avec un `Pin` encapsulant un `Box`.) L'essentiel est que le type auto-référentiel lui-même ne peut pas se déplacer, car il est toujours épinglé.

<figure>

<img alt="Four boxes laid out in three rough columns, identical to the previous diagram with a change to the second column. Now there are two boxes in the second column, labeled "b1" and "b2", "b1" is grayed out, and the arrow from "Pin" goes through "b2" instead of "b1", indicating that the pointer has moved from "b1" to "b2", but the data in "pinned" has not moved." src="img/trpl17-07.svg" class="center" />

<!--
<figcaption>Figure 17-7: Moving a `Box` which points to a self-referential future type</figcaption>
-->

<figcaption>Figure 17-7 : Déplacer un `Box` qui pointe vers un type de future auto-référentiel</figcaption>

</figure>

<!--
However, most types are perfectly safe to move around, even if they happen to be
behind a `Pin` pointer. We only need to think about pinning when items have
internal references. Primitive values such as numbers and Booleans are safe
because they obviously don't have any internal references.
Neither do most types you normally work with in Rust. You can move around
a `Vec`, for example, without worrying. Given what we have seen so far, if
you have a `Pin<Vec<String>>`, you'd have to do everything via the safe but
restrictive APIs provided by `Pin`, even though a `Vec<String>` is always safe
to move if there are no other references to it. We need a way to tell the
compiler that it's fine to move items around in cases like this—and that's
where `Unpin` comes into play.
-->

Cependant, la plupart des types sont parfaitement sûrs à déplacer, même s'ils se trouvent derrière un pointeur `Pin`. Nous n'avons besoin de penser à l'épinglage que quand les éléments ont des références internes. Les valeurs primitives comme les nombres et les booléens sont sûres car elles n'ont évidemment aucune référence interne. La plupart des types avec lesquels vous travaillez normalement en Rust non plus. Vous pouvez déplacer un `Vec`, par exemple, sans vous inquiéter. Étant donné ce que nous avons vu jusqu'ici, si vous avez un `Pin<Vec<String>>`, vous devriez tout faire via les API sûres mais restrictives fournies par `Pin`, même si un `Vec<String>` est toujours sûr à déplacer s'il n'y a pas d'autres références vers lui. Nous avons besoin d'un moyen de dire au compilateur qu'il est acceptable de déplacer des éléments dans des cas comme celui-ci — et c'est là qu'`Unpin` entre en jeu.

<!--
`Unpin` is a marker trait, similar to the `Send` and `Sync` traits we saw in
Chapter 16, and thus has no functionality of its own. Marker traits exist only
to tell the compiler it's safe to use the type implementing a given trait in a
particular context. `Unpin` informs the compiler that a given type does _not_
need to uphold any guarantees about whether the value in question can be safely
moved.
-->

`Unpin` est un trait marqueur, similaire aux traits `Send` et `Sync` que nous avons vus au chapitre 16, et n'a donc aucune fonctionnalité propre. Les traits marqueurs n'existent que pour dire au compilateur qu'il est sûr d'utiliser le type implémentant un trait donné dans un contexte particulier. `Unpin` informe le compilateur qu'un type donné n'a _pas_ besoin de respecter de garanties concernant le déplacement sûr de la valeur en question.

<!--
The inline `<code>` in the next block is to allow the inline `<em>` inside it,
  matching what NoStarch does style-wise, and emphasizing within the text here
  that it is something distinct from a normal type.
-->

<!--
Just as with `Send` and `Sync`, the compiler implements `Unpin` automatically
for all types where it can prove it is safe. A special case, again similar to
`Send` and `Sync`, is where `Unpin` is _not_ implemented for a type. The
notation for this is <code>impl !Unpin for <em>SomeType</em></code>, where
<code><em>SomeType</em></code> is the name of a type that _does_ need to uphold
those guarantees to be safe whenever a pointer to that type is used in a `Pin`.
-->

Tout comme avec `Send` et `Sync`, le compilateur implémente `Unpin` automatiquement pour tous les types pour lesquels il peut prouver que c'est sûr. Un cas spécial, encore similaire à `Send` et `Sync`, est quand `Unpin` n'est _pas_ implémenté pour un type. La notation pour cela est <code>impl !Unpin for <em>SomeType</em></code>, où <code><em>SomeType</em></code> est le nom d'un type qui _doit_ respecter ces garanties pour être sûr chaque fois qu'un pointeur vers ce type est utilisé dans un `Pin`.

<!--
In other words, there are two things to keep in mind about the relationship
between `Pin` and `Unpin`. First, `Unpin` is the "normal" case, and `!Unpin` is
the special case. Second, whether a type implements `Unpin` or `!Unpin` _only_
matters when you're using a pinned pointer to that type like <code>Pin<&mut
<em>SomeType</em>></code>.
-->

En d'autres termes, il y a deux choses à garder à l'esprit concernant la relation entre `Pin` et `Unpin`. Premièrement, `Unpin` est le cas « normal », et `!Unpin` est le cas spécial. Deuxièmement, qu'un type implémente `Unpin` ou `!Unpin` n'a d'importance _que_ quand vous utilisez un pointeur épinglé vers ce type comme <code>Pin<&mut <em>SomeType</em>></code>.

<!--
To make that concrete, think about a `String`: it has a length and the Unicode
characters that make it up. We can wrap a `String` in `Pin`, as seen in Figure
17-8. However, `String` automatically implements `Unpin`, as do most other types
in Rust.
-->

Pour rendre cela concret, pensez à une `String` : elle a une longueur et les caractères Unicode qui la composent. Nous pouvons encapsuler une `String` dans `Pin`, comme vu dans la figure 17-8. Cependant, `String` implémente automatiquement `Unpin`, comme la plupart des autres types en Rust.

<figure>

<img alt="A box labeled "Pin" on the left with an arrow going from it to a box labeled "String" on the right. The "String" box contains the data 5usize, representing the length of the string, and the letters "h", "e", "l", "l", and "o" representing the characters of the string "hello" stored in this String instance. A dotted rectangle surrounds the "String" box and its label, but not the "Pin" box." src="img/trpl17-08.svg" class="center" />

<!--
<figcaption>Figure 17-8: Pinning a `String`; the dotted line indicates that the `String` implements the `Unpin` trait and thus is not pinned</figcaption>
-->

<figcaption>Figure 17-8 : Épingler une `String` ; la ligne pointillée indique que la `String` implémente le trait `Unpin` et n'est donc pas épinglée</figcaption>

</figure>

<!--
As a result, we can do things that would be illegal if `String` implemented
`!Unpin` instead, such as replacing one string with another at the exact same
location in memory as in Figure 17-9. This doesn't violate the `Pin` contract,
because `String` has no internal references that make it unsafe to move around.
That is precisely why it implements `Unpin` rather than `!Unpin`.
-->

En conséquence, nous pouvons faire des choses qui seraient illégales si `String` implémentait `!Unpin` à la place, comme remplacer une chaîne par une autre exactement au même emplacement en mémoire comme dans la figure 17-9. Cela ne viole pas le contrat de `Pin`, car `String` n'a pas de références internes qui rendraient son déplacement dangereux. C'est précisément pourquoi elle implémente `Unpin` plutôt que `!Unpin`.

<figure>

<img alt="The same "hello" string data from the previous example, now labeled "s1" and grayed out. The "Pin" box from the previous example now points to a different String instance, one that is labeled "s2", is valid, has a length of 7usize, and contains the characters of the string "goodbye". s2 is surrounded by a dotted rectangle because it, too, implements the Unpin trait." src="img/trpl17-09.svg" class="center" />

<!--
<figcaption>Figure 17-9: Replacing the `String` with an entirely different `String` in memory</figcaption>
-->

<figcaption>Figure 17-9 : Remplacer la `String` par une `String` entièrement différente en mémoire</figcaption>

</figure>

<!--
Now we know enough to understand the errors reported for that `join_all` call
from back in Listing 17-23. We originally tried to move the futures produced by
async blocks into a `Vec<Box<dyn Future<Output = ()>>>`, but as we've seen,
those futures may have internal references, so they don't automatically
implement `Unpin`. Once we pin them, we can pass the resulting `Pin` type into
the `Vec`, confident that the underlying data in the futures will _not_ be
moved. Listing 17-24 shows how to fix the code by calling the `pin!` macro
where each of the three futures are defined and adjusting the trait object type.
-->

Nous en savons maintenant assez pour comprendre les erreurs signalées pour cet appel à `join_all` dans l'encart 17-23. Nous avons initialement essayé de déplacer les futures produites par les blocs async dans un `Vec<Box<dyn Future<Output = ()>>>`, mais comme nous l'avons vu, ces futures peuvent avoir des références internes, donc elles n'implémentent pas automatiquement `Unpin`. Une fois que nous les épinglons, nous pouvons passer le type `Pin` résultant dans le `Vec`, confiants que les données sous-jacentes dans les futures ne seront _pas_ déplacées. L'encart 17-24 montre comment corriger le code en appelant la macro `pin!` là où chacune des trois futures est définie et en ajustant le type d'objet trait.

<Listing number="17-24" caption="Épingler les futures pour permettre de les déplacer dans le vecteur">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

<!--
This example now compiles and runs, and we could add or remove futures from the
vector at runtime and join them all.
-->

Cet exemple compile et s'exécute maintenant, et nous pourrions ajouter ou retirer des futures du vecteur à l'exécution et les joindre toutes.

<!--
`Pin` and `Unpin` are mostly important for building lower-level libraries, or
when you're building a runtime itself, rather than for day-to-day Rust code.
When you see these traits in error messages, though, now you'll have a better
idea of how to fix your code!
-->

`Pin` et `Unpin` sont surtout importants pour construire des bibliothèques de bas niveau, ou quand vous construisez un runtime lui-même, plutôt que pour le code Rust quotidien. Quand vous verrez ces traits dans les messages d'erreur, cependant, vous aurez maintenant une meilleure idée de comment corriger votre code !

<!--
> Note: This combination of `Pin` and `Unpin` makes it possible to safely
> implement a whole class of complex types in Rust that would otherwise prove
> challenging because they're self-referential. Types that require `Pin` show up
> most commonly in async Rust today, but every once in a while, you might see
> them in other contexts, too.
>
> The specifics of how `Pin` and `Unpin` work, and the rules they're required
> to uphold, are covered extensively in the API documentation for `std::pin`, so
> if you're interested in learning more, that's a great place to start.
>
> If you want to understand how things work under the hood in even more detail,
> see Chapters [2][under-the-hood] ignore
--> and
> [4][pinning]<!--
ignore
--> of
> [_Asynchronous Programming in Rust_][async-book].
-->

> Remarque : cette combinaison de `Pin` et `Unpin` rend possible l'implémentation sûre de toute une classe de types complexes en Rust qui seraient autrement difficiles car ils sont auto-référentiels. Les types qui nécessitent `Pin` apparaissent le plus souvent dans le Rust async aujourd'hui, mais de temps en temps, vous pourriez aussi les voir dans d'autres contextes.
>
> Les spécificités du fonctionnement de `Pin` et `Unpin`, et les règles qu'ils doivent respecter, sont couvertes en détail dans la documentation de l'API pour `std::pin`, donc si vous souhaitez en savoir plus, c'est un excellent point de départ.
>
> Si vous voulez comprendre comment les choses fonctionnent en coulisses encore plus en détail, voyez les chapitres [2][under-the-hood]<!--
ignore
--> et [4][pinning]<!--
ignore
--> de [_Asynchronous Programming in Rust_][async-book].

<!--
### The `Stream` Trait
-->

### Le trait `Stream`

<!--
Now that you have a deeper grasp on the `Future`, `Pin`, and `Unpin` traits, we
can turn our attention to the `Stream` trait. As you learned earlier in the
chapter, streams are similar to asynchronous iterators. Unlike `Iterator` and
`Future`, however, `Stream` has no definition in the standard library as of
this writing, but there _is_ a very common definition from the `futures` crate
used throughout the ecosystem.
-->

Maintenant que vous avez une compréhension plus profonde des traits `Future`, `Pin` et `Unpin`, nous pouvons tourner notre attention vers le trait `Stream`. Comme vous l'avez appris plus tôt dans le chapitre, les streams sont similaires aux itérateurs asynchrones. Contrairement à `Iterator` et `Future`, cependant, `Stream` n'a pas de définition dans la bibliothèque standard au moment de la rédaction, mais il _existe_ une définition très courante du crate `futures` utilisée dans tout l'écosystème.

<!--
Let's review the definitions of the `Iterator` and `Future` traits before
looking at how a `Stream` trait might merge them together. From `Iterator`, we
have the idea of a sequence: its `next` method provides an
`Option<Self::Item>`. From `Future`, we have the idea of readiness over time:
its `poll` method provides a `Poll<Self::Output>`. To represent a sequence of
items that become ready over time, we define a `Stream` trait that puts those
features together:
-->

Revoyons les définitions des traits `Iterator` et `Future` avant de regarder comment un trait `Stream` pourrait les fusionner. De `Iterator`, nous avons l'idée d'une séquence : sa méthode `next` fournit un `Option<Self::Item>`. De `Future`, nous avons l'idée de disponibilité au fil du temps : sa méthode `poll` fournit un `Poll<Self::Output>`. Pour représenter une séquence d'éléments qui deviennent disponibles au fil du temps, nous définissons un trait `Stream` qui rassemble ces fonctionnalités :

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

<!--
The `Stream` trait defines an associated type called `Item` for the type of the
items produced by the stream. This is similar to `Iterator`, where there may be
zero to many items, and unlike `Future`, where there is always a single
`Output`, even if it's the unit type `()`.
-->

Le trait `Stream` définit un type associé appelé `Item` pour le type des éléments produits par le stream. C'est similaire à `Iterator`, où il peut y avoir de zéro à plusieurs éléments, et contrairement à `Future`, où il y a toujours un seul `Output`, même si c'est le type unit `()`.

<!--
`Stream` also defines a method to get those items. We call it `poll_next`, to
make it clear that it polls in the same way `Future::poll` does and produces a
sequence of items in the same way `Iterator::next` does. Its return type
combines `Poll` with `Option`. The outer type is `Poll`, because it has to be
checked for readiness, just as a future does. The inner type is `Option`,
because it needs to signal whether there are more messages, just as an iterator
does.
-->

`Stream` définit aussi une méthode pour obtenir ces éléments. Nous l'appelons `poll_next`, pour rendre clair qu'elle interroge de la même manière que `Future::poll` et produit une séquence d'éléments de la même manière que `Iterator::next`. Son type de retour combine `Poll` avec `Option`. Le type extérieur est `Poll`, car il doit être vérifié pour la disponibilité, tout comme une future. Le type intérieur est `Option`, car il doit signaler s'il y a plus de messages, tout comme un itérateur.

<!--
Something very similar to this definition will likely end up as part of Rust's
standard library. In the meantime, it's part of the toolkit of most runtimes,
so you can rely on it, and everything we cover next should generally apply!
-->

Quelque chose de très similaire à cette définition finira probablement par faire partie de la bibliothèque standard de Rust. En attendant, cela fait partie de la boîte à outils de la plupart des runtimes, donc vous pouvez vous y fier, et tout ce que nous couvrons ensuite devrait généralement s'appliquer !

<!--
In the examples we saw in the ["Streams: Futures in Sequence"][streams]
ignore
--> section, though, we didn't use `poll_next` _or_ `Stream`, but
instead used `next` and `StreamExt`. We _could_ work directly in terms of the
`poll_next` API by hand-writing our own `Stream` state machines, of course,
just as we _could_ work with futures directly via their `poll` method. Using
`await` is much nicer, though, and the `StreamExt` trait supplies the `next`
method so we can do just that:
-->

Dans les exemples que nous avons vus dans la section [« Streams : des futures en séquence »][streams]<!--
ignore
-->, cependant, nous n'avons pas utilisé `poll_next` _ni_ `Stream`, mais plutôt `next` et `StreamExt`. Nous _pourrions_ travailler directement avec l'API `poll_next` en écrivant manuellement nos propres machines à états `Stream`, bien sûr, tout comme nous _pourrions_ travailler avec les futures directement via leur méthode `poll`. Utiliser `await` est beaucoup plus agréable, cependant, et le trait `StreamExt` fournit la méthode `next` pour que nous puissions faire exactement cela :

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using async functions
in traits, since the lack thereof is the reason they do not yet have this.
-->

<!--
> Note: The actual definition we used earlier in the chapter looks slightly
> different than this, because it supports versions of Rust that did not yet
> support using async functions in traits. As a result, it looks like this:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> That `Next` type is a `struct` that implements `Future` and allows us to name
> the lifetime of the reference to `self` with `Next<'_, Self>`, so that `await`
> can work with this method.
-->

> Remarque : la définition réelle que nous avons utilisée plus tôt dans le chapitre est légèrement différente de celle-ci, car elle prend en charge les versions de Rust qui ne supportaient pas encore l'utilisation de fonctions async dans les traits. En conséquence, elle ressemble à ceci :
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> Ce type `Next` est une `struct` qui implémente `Future` et nous permet de nommer la durée de vie de la référence vers `self` avec `Next<'_, Self>`, pour que `await` puisse fonctionner avec cette méthode.

<!--
The `StreamExt` trait is also the home of all the interesting methods available
to use with streams. `StreamExt` is automatically implemented for every type
that implements `Stream`, but these traits are defined separately to enable the
community to iterate on convenience APIs without affecting the foundational
trait.
-->

Le trait `StreamExt` est aussi le lieu de toutes les méthodes intéressantes disponibles pour utiliser avec les streams. `StreamExt` est automatiquement implémenté pour chaque type qui implémente `Stream`, mais ces traits sont définis séparément pour permettre à la communauté d'itérer sur les API de commodité sans affecter le trait fondamental.

<!--
In the version of `StreamExt` used in the `trpl` crate, the trait not only
defines the `next` method but also supplies a default implementation of `next`
that correctly handles the details of calling `Stream::poll_next`. This means
that even when you need to write your own streaming data type, you _only_ have
to implement `Stream`, and then anyone who uses your data type can use
`StreamExt` and its methods with it automatically.
-->

Dans la version de `StreamExt` utilisée dans le crate `trpl`, le trait non seulement définit la méthode `next` mais fournit aussi une implémentation par défaut de `next` qui gère correctement les détails de l'appel à `Stream::poll_next`. Cela signifie que même quand vous devez écrire votre propre type de données de streaming, vous n'avez _qu'à_ implémenter `Stream`, et ensuite quiconque utilise votre type de données peut utiliser `StreamExt` et ses méthodes automatiquement.

<!--
That's all we're going to cover for the lower-level details on these traits. To
wrap up, let's consider how futures (including streams), tasks, and threads all
fit together!
-->

C'est tout ce que nous allons couvrir pour les détails de bas niveau de ces traits. Pour conclure, voyons comment les futures (y compris les streams), les tâches et les threads s'articulent ensemble !

[message-passing]: ch17-02-concurrency-with-async.md#sending-data-between-two-tasks-using-message-passing
[ch-18]: ch18-00-oop.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program
[any-number-futures]: ch17-03-more-futures.html#working-with-any-number-of-futures
[streams]: ch17-04-streams.html
