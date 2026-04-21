<!--
Old headings. Do not remove or links may break.
-->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>
<a id="extensible-concurrency-with-the-send-and-sync-traits"></a>

<!--
## Extensible Concurrency with `Send` and `Sync`
-->

## La concurrence extensible avec `Send` et `Sync`

<!--
Interestingly, almost every concurrency feature we've talked about so far in
this chapter has been part of the standard library, not the language. Your
options for handling concurrency are not limited to the language or the
standard library; you can write your own concurrency features or use those
written by others.
-->

Fait intéressant, presque toutes les fonctionnalités de concurrence dont nous avons parlé jusqu'à présent dans ce chapitre font partie de la bibliothèque standard, pas du langage lui-même. Vos options pour gérer la concurrence ne se limitent pas au langage ou à la bibliothèque standard ; vous pouvez écrire vos propres fonctionnalités de concurrence ou utiliser celles écrites par d'autres.

<!--
However, among the key concurrency concepts that are embedded in the language
rather than the standard library are the `std::marker` traits `Send` and `Sync`.
-->

Cependant, parmi les concepts clés de concurrence qui sont intégrés dans le langage plutôt que dans la bibliothèque standard, on trouve les traits marqueurs `std::marker` `Send` et `Sync`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="allowing-transference-of-ownership-between-threads-with-send"></a>

<!--
### Transferring Ownership Between Threads
-->

### Transférer la possession entre les threads

<!--
The `Send` marker trait indicates that ownership of values of the type
implementing `Send` can be transferred between threads. Almost every Rust type
implements `Send`, but there are some exceptions, including `Rc<T>`: This
cannot implement `Send` because if you cloned an `Rc<T>` value and tried to
transfer ownership of the clone to another thread, both threads might update
the reference count at the same time. For this reason, `Rc<T>` is implemented
for use in single-threaded situations where you don't want to pay the
thread-safe performance penalty.
-->

Le trait marqueur `Send` indique que la possession des valeurs du type implémentant `Send` peut être transférée entre les threads. Presque tous les types Rust implémentent `Send`, mais il existe quelques exceptions, notamment `Rc<T>` : celui-ci ne peut pas implémenter `Send` car si vous cloniez une valeur `Rc<T>` et tentiez de transférer la possession du clone à un autre thread, les deux threads pourraient mettre à jour le compteur de références en même temps. Pour cette raison, `Rc<T>` est conçu pour être utilisé dans des situations mono-thread où vous ne voulez pas payer la pénalité de performance liée à la sécurité des threads.

<!--
Therefore, Rust's type system and trait bounds ensure that you can never
accidentally send an `Rc<T>` value across threads unsafely. When we tried to do
this in Listing 16-14, we got the error `` the trait `Send` is not implemented
for `Rc<Mutex<i32>>` ``. When we switched to `Arc<T>`, which does implement
`Send`, the code compiled.
-->

Par conséquent, le système de types de Rust et les contraintes de traits garantissent que vous ne pouvez jamais envoyer accidentellement une valeur `Rc<T>` entre les threads de manière non sûre. Lorsque nous avons essayé de le faire dans l'encart 16-14, nous avons obtenu l'erreur `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. Lorsque nous sommes passés à `Arc<T>`, qui implémente bien `Send`, le code a compilé.

<!--
Any type composed entirely of `Send` types is automatically marked as `Send` as
well. Almost all primitive types are `Send`, aside from raw pointers, which
we'll discuss in Chapter 20.
-->

Tout type composé entièrement de types `Send` est automatiquement marqué comme `Send` également. Presque tous les types primitifs sont `Send`, à l'exception des pointeurs bruts, dont nous parlerons au chapitre 20.

<!--
Old headings. Do not remove or links may break.
-->

<a id="allowing-access-from-multiple-threads-with-sync"></a>

<!--
### Accessing from Multiple Threads
-->

### Accéder depuis plusieurs threads

<!--
The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads. In other words, any type `T`
implements `Sync` if `&T` (an immutable reference to `T`) implements `Send`,
meaning the reference can be sent safely to another thread. Similar to `Send`,
primitive types all implement `Sync`, and types composed entirely of types that
implement `Sync` also implement `Sync`.
-->

Le trait marqueur `Sync` indique qu'il est sûr pour le type implémentant `Sync` d'être référencé depuis plusieurs threads. En d'autres termes, tout type `T` implémente `Sync` si `&T` (une référence immuable vers `T`) implémente `Send`, ce qui signifie que la référence peut être envoyée en toute sécurité vers un autre thread. De manière similaire à `Send`, les types primitifs implémentent tous `Sync`, et les types composés entièrement de types qui implémentent `Sync` implémentent également `Sync`.

<!--
The smart pointer `Rc<T>` also doesn't implement `Sync` for the same reasons
that it doesn't implement `Send`. The `RefCell<T>` type (which we talked about
in Chapter 15) and the family of related `Cell<T>` types don't implement
`Sync`. The implementation of borrow checking that `RefCell<T>` does at runtime
is not thread-safe. The smart pointer `Mutex<T>` implements `Sync` and can be
used to share access with multiple threads, as you saw in ["Shared Access to
`Mutex<T>`"][shared-access] ignore
-->.
-->

Le pointeur intelligent `Rc<T>` n'implémente pas non plus `Sync` pour les mêmes raisons qu'il n'implémente pas `Send`. Le type `RefCell<T>` (dont nous avons parlé au chapitre 15) et la famille de types `Cell<T>` associés n'implémentent pas `Sync`. L'implémentation de la vérification des emprunts que `RefCell<T>` effectue à l'exécution n'est pas sûre pour les threads. Le pointeur intelligent `Mutex<T>` implémente `Sync` et peut être utilisé pour partager l'accès avec plusieurs threads, comme vous l'avez vu dans ["Accès partagé à `Mutex<T>`"][shared-access]<!--
ignore
-->.

<!--
### Implementing `Send` and `Sync` Manually Is Unsafe
-->

### Implémenter `Send` et `Sync` manuellement n'est pas sûr

<!--
Because types composed entirely of other types that implement the `Send` and
`Sync` traits also automatically implement `Send` and `Sync`, we don't have to
implement those traits manually. As marker traits, they don't even have any
methods to implement. They're just useful for enforcing invariants related to
concurrency.
-->

Comme les types composés entièrement d'autres types qui implémentent les traits `Send` et `Sync` implémentent aussi automatiquement `Send` et `Sync`, nous n'avons pas besoin d'implémenter ces traits manuellement. En tant que traits marqueurs, ils n'ont même pas de méthodes à implémenter. Ils sont simplement utiles pour appliquer des invariants liés à la concurrence.

<!--
Manually implementing these traits involves implementing unsafe Rust code.
We'll talk about using unsafe Rust code in Chapter 20; for now, the important
information is that building new concurrent types not made up of `Send` and
`Sync` parts requires careful thought to uphold the safety guarantees. ["The
Rustonomicon"][nomicon] has more information about these guarantees and how to
uphold them.
-->

Implémenter ces traits manuellement implique d'écrire du code Rust non sûr (unsafe). Nous parlerons de l'utilisation du code Rust unsafe au chapitre 20 ; pour l'instant, l'information importante est que construire de nouveaux types concurrents qui ne sont pas composés de parties `Send` et `Sync` nécessite une réflexion approfondie pour respecter les garanties de sécurité. ["The Rustonomicon"][nomicon] contient plus d'informations sur ces garanties et comment les respecter.

<!--
## Summary
-->

## Résumé

<!--
This isn't the last you'll see of concurrency in this book: The next chapter
focuses on async programming, and the project in Chapter 21 will use the
concepts in this chapter in a more realistic situation than the smaller
examples discussed here.
-->

Ce n'est pas la dernière fois que vous verrez la concurrence dans ce livre : le prochain chapitre se concentre sur la programmation asynchrone, et le projet du chapitre 21 utilisera les concepts de ce chapitre dans une situation plus réaliste que les petits exemples abordés ici.

<!--
As mentioned earlier, because very little of how Rust handles concurrency is
part of the language, many concurrency solutions are implemented as crates.
These evolve more quickly than the standard library, so be sure to search
online for the current, state-of-the-art crates to use in multithreaded
situations.
-->

Comme mentionné précédemment, étant donné que très peu de la façon dont Rust gère la concurrence fait partie du langage, de nombreuses solutions de concurrence sont implémentées sous forme de crates. Celles-ci évoluent plus rapidement que la bibliothèque standard, alors n'hésitez pas à chercher en ligne les crates actuelles et à la pointe pour les situations multi-threads.

<!--
The Rust standard library provides channels for message passing and smart
pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in
concurrent contexts. The type system and the borrow checker ensure that the
code using these solutions won't end up with data races or invalid references.
Once you get your code to compile, you can rest assured that it will happily
run on multiple threads without the kinds of hard-to-track-down bugs common in
other languages. Concurrent programming is no longer a concept to be afraid of:
Go forth and make your programs concurrent, fearlessly!
-->

La bibliothèque standard de Rust fournit des canaux pour le passage de messages et des types de pointeurs intelligents, tels que `Mutex<T>` et `Arc<T>`, qui sont sûrs à utiliser dans des contextes concurrents. Le système de types et le vérificateur d'emprunts garantissent que le code utilisant ces solutions ne se retrouvera pas avec des courses de données ou des références invalides. Une fois que votre code compile, vous pouvez être assuré qu'il fonctionnera sans problème sur plusieurs threads, sans les types de bogues difficiles à traquer qui sont courants dans d'autres langages. La programmation concurrente n'est plus un concept à craindre : allez de l'avant et rendez vos programmes concurrents, sans crainte !

[shared-access]: ch16-03-shared-state.html#shared-access-to-mutext
[nomicon]: ../nomicon/index.html
