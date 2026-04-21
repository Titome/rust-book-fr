<!--
## Futures and the Async Syntax
-->

## Les futures et la syntaxe async

<!--
The key elements of asynchronous programming in Rust are _futures_ and Rust's
`async` and `await` keywords.
-->

Les éléments clés de la programmation asynchrone en Rust sont les _futures_ et les mots-clés `async` et `await` de Rust.

<!--
A _future_ is a value that may not be ready now but will become ready at some
point in the future. (This same concept shows up in many languages, sometimes
under other names such as _task_ or _promise_.) Rust provides a `Future` trait
as a building block so that different async operations can be implemented with
different data structures but with a common interface. In Rust, futures are
types that implement the `Future` trait. Each future holds its own information
about the progress that has been made and what "ready" means.
-->

Une _future_ est une valeur qui n'est peut-être pas prête maintenant mais qui le deviendra à un moment donné dans le futur. (Ce même concept apparaît dans de nombreux langages, parfois sous d'autres noms comme _task_ ou _promise_.) Rust fournit un trait `Future` comme brique de base pour que différentes opérations async puissent être implémentées avec différentes structures de données mais avec une interface commune. En Rust, les futures sont des types qui implémentent le trait `Future`. Chaque future contient ses propres informations sur la progression réalisée et sur ce que signifie « prêt ».

<!--
You can apply the `async` keyword to blocks and functions to specify that they
can be interrupted and resumed. Within an async block or async function, you
can use the `await` keyword to _await a future_ (that is, wait for it to become
ready). Any point where you await a future within an async block or function is
a potential spot for that block or function to pause and resume. The process of
checking with a future to see if its value is available yet is called _polling_.
-->

Vous pouvez appliquer le mot-clé `async` aux blocs et aux fonctions pour spécifier qu'ils peuvent être interrompus et repris. Au sein d'un bloc async ou d'une fonction async, vous pouvez utiliser le mot-clé `await` pour _attendre une future_ (c'est-à-dire attendre qu'elle devienne prête). Tout point où vous attendez une future dans un bloc ou une fonction async est un endroit potentiel pour que ce bloc ou cette fonction se mette en pause et reprenne. Le processus de vérification auprès d'une future pour voir si sa valeur est disponible s'appelle le _polling_ (ou _interrogation_).

<!--
Some other languages, such as C# and JavaScript, also use `async` and `await`
keywords for async programming. If you're familiar with those languages, you
may notice some significant differences in how Rust handles the syntax. That's
for good reason, as we'll see!
-->

Certains autres langages, comme C# et JavaScript, utilisent également les mots-clés `async` et `await` pour la programmation asynchrone. Si vous êtes familier avec ces langages, vous remarquerez peut-être des différences significatives dans la façon dont Rust gère la syntaxe. C'est pour de bonnes raisons, comme nous le verrons !

<!--
When writing async Rust, we use the `async` and `await` keywords most of the
time. Rust compiles them into equivalent code using the `Future` trait, much as
it compiles `for` loops into equivalent code using the `Iterator` trait.
Because Rust provides the `Future` trait, though, you can also implement it for
your own data types when you need to. Many of the functions we'll see
throughout this chapter return types with their own implementations of
`Future`. We'll return to the definition of the trait at the end of the chapter
and dig into more of how it works, but this is enough detail to keep us moving
forward.
-->

Lorsque nous écrivons du Rust async, nous utilisons les mots-clés `async` et `await` la plupart du temps. Rust les compile en code équivalent utilisant le trait `Future`, de la même manière qu'il compile les boucles `for` en code équivalent utilisant le trait `Iterator`. Comme Rust fournit le trait `Future`, vous pouvez aussi l'implémenter pour vos propres types de données quand vous en avez besoin. Beaucoup de fonctions que nous verrons tout au long de ce chapitre retournent des types avec leurs propres implémentations de `Future`. Nous reviendrons à la définition du trait à la fin du chapitre pour approfondir son fonctionnement, mais ces détails suffisent pour avancer.

<!--
This may all feel a bit abstract, so let's write our first async program: a
little web scraper. We'll pass in two URLs from the command line, fetch both of
them concurrently, and return the result of whichever one finishes first. This
example will have a fair bit of new syntax, but don't worry—we'll explain
everything you need to know as we go.
-->

Tout cela peut sembler un peu abstrait, alors écrivons notre premier programme async : un petit web scraper. Nous passerons deux URL depuis la ligne de commande, les récupérerons toutes les deux de manière concurrente, et retournerons le résultat de celle qui finit en premier. Cet exemple comportera pas mal de nouvelle syntaxe, mais ne vous inquiétez pas — nous expliquerons tout ce que vous devez savoir au fur et à mesure.

<!--
## Our First Async Program
-->

## Notre premier programme async

<!--
To keep the focus of this chapter on learning async rather than juggling parts
of the ecosystem, we've created the `trpl` crate (`trpl` is short for "The Rust
Programming Language"). It re-exports all the types, traits, and functions
you'll need, primarily from the [`futures`][futures-crate] ignore
--> and
[`tokio`][tokio]<!--
ignore
--> crates. The `futures` crate is an official home
for Rust experimentation for async code, and it's actually where the `Future`
trait was originally designed. Tokio is the most widely used async runtime in
Rust today, especially for web applications. There are other great runtimes out
there, and they may be more suitable for your purposes. We use the `tokio`
crate under the hood for `trpl` because it's well tested and widely used.
-->

Pour garder l'attention de ce chapitre sur l'apprentissage de l'async plutôt que sur la gestion de l'écosystème, nous avons créé le crate `trpl` (`trpl` est l'abréviation de « The Rust Programming Language »). Il ré-exporte tous les types, traits et fonctions dont vous aurez besoin, principalement depuis les crates [`futures`][futures-crate]<!--
ignore
--> et [`tokio`][tokio]<!--
ignore
-->. Le crate `futures` est le lieu officiel d'expérimentation de Rust pour le code async, et c'est en fait là que le trait `Future` a été conçu à l'origine. Tokio est le runtime async le plus largement utilisé en Rust aujourd'hui, surtout pour les applications web. Il existe d'autres excellents runtimes qui pourraient être plus adaptés à vos besoins. Nous utilisons le crate `tokio` en coulisses pour `trpl` car il est bien testé et largement utilisé.

<!--
In some cases, `trpl` also renames or wraps the original APIs to keep you
focused on the details relevant to this chapter. If you want to understand what
the crate does, we encourage you to check out [its source code][crate-source].
You'll be able to see what crate each re-export comes from, and we've left
extensive comments explaining what the crate does.
-->

Dans certains cas, `trpl` renomme ou encapsule les API d'origine pour vous permettre de vous concentrer sur les détails pertinents pour ce chapitre. Si vous voulez comprendre ce que fait le crate, nous vous encourageons à consulter [son code source][crate-source]. Vous pourrez voir de quel crate provient chaque ré-exportation, et nous avons laissé des commentaires détaillés expliquant ce que fait le crate.

<!--
Create a new binary project named `hello-async` and add the `trpl` crate as a
dependency:
-->

Créez un nouveau projet binaire nommé `hello-async` et ajoutez le crate `trpl` comme dépendance :

<!--
```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```
-->

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

<!--
Now we can use the various pieces provided by `trpl` to write our first async
program. We'll build a little command line tool that fetches two web pages,
pulls the `<title>` element from each, and prints out the title of whichever
page finishes that whole process first.
-->

Nous pouvons maintenant utiliser les différents éléments fournis par `trpl` pour écrire notre premier programme async. Nous allons construire un petit outil en ligne de commande qui récupère deux pages web, extrait l'élément `<title>` de chacune, et affiche le titre de la page qui termine ce processus en premier.

<!--
### Defining the page_title Function
-->

### Définition de la fonction page_title

<!--
Let's start by writing a function that takes one page URL as a parameter, makes
a request to it, and returns the text of the `<title>` element (see Listing
17-1).
-->

Commençons par écrire une fonction qui prend une URL de page en paramètre, effectue une requête vers celle-ci, et retourne le texte de l'élément `<title>` (voir l'encart 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Définition d'une fonction async pour obtenir l'élément title d'une page HTML">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

<!--
First, we define a function named `page_title` and mark it with the `async`
keyword. Then we use the `trpl::get` function to fetch whatever URL is passed
in and add the `await` keyword to await the response. To get the text of the
`response`, we call its `text` method and once again await it with the `await`
keyword. Both of these steps are asynchronous. For the `get` function, we have
to wait for the server to send back the first part of its response, which will
include HTTP headers, cookies, and so on and can be delivered separately from
the response body. Especially if the body is very large, it can take some time
for it all to arrive. Because we have to wait for the _entirety_ of the
response to arrive, the `text` method is also async.
-->

Premièrement, nous définissons une fonction nommée `page_title` et la marquons avec le mot-clé `async`. Ensuite, nous utilisons la fonction `trpl::get` pour récupérer l'URL passée en paramètre et ajoutons le mot-clé `await` pour attendre la réponse. Pour obtenir le texte de la réponse, nous appelons sa méthode `text` et l'attendons à nouveau avec le mot-clé `await`. Ces deux étapes sont asynchrones. Pour la fonction `get`, nous devons attendre que le serveur renvoie la première partie de sa réponse, qui inclura les en-têtes HTTP, les cookies, etc., et qui peut être délivrée séparément du corps de la réponse. Surtout si le corps est très volumineux, il peut falloir un certain temps pour qu'il arrive en entier. Comme nous devons attendre l'_intégralité_ de la réponse, la méthode `text` est également async.

<!--
We have to explicitly await both of these futures, because futures in Rust are
_lazy_: they don't do anything until you ask them to with the `await` keyword.
(In fact, Rust will show a compiler warning if you don't use a future.) This
might remind you of the discussion of iterators in the ["Processing a Series of
Items with Iterators"][iterators-lazy] ignore
--> section in Chapter 13.
Iterators do nothing unless you call their `next` method—whether directly or by
using `for` loops or methods such as `map` that use `next` under the hood.
Likewise, futures do nothing unless you explicitly ask them to. This laziness
allows Rust to avoid running async code until it's actually needed.
-->

Nous devons explicitement attendre ces deux futures, car les futures en Rust sont _paresseuses_ : elles ne font rien tant que vous ne leur demandez pas avec le mot-clé `await`. (En fait, Rust affichera un avertissement du compilateur si vous n'utilisez pas une future.) Cela pourrait vous rappeler la discussion sur les itérateurs dans la section [« Traitement d'une série d'éléments avec les itérateurs »][iterators-lazy]<!--
ignore
--> du chapitre 13. Les itérateurs ne font rien tant que vous n'appelez pas leur méthode `next` — que ce soit directement ou en utilisant des boucles `for` ou des méthodes comme `map` qui utilisent `next` en coulisses. De même, les futures ne font rien tant que vous ne leur demandez pas explicitement. Cette paresse permet à Rust d'éviter d'exécuter du code async tant que ce n'est pas réellement nécessaire.

<!--
> Note: This is different from the behavior we saw when using `thread::spawn`
> in the ["Creating a New Thread with spawn"][thread-spawn] ignore
-->
> section in Chapter 16, where the closure we passed to another thread started
> running immediately. It's also different from how many other languages
> approach async. But it's important for Rust to be able to provide its
> performance guarantees, just as it is with iterators.
-->

> Remarque : cela est différent du comportement que nous avons vu en utilisant `thread::spawn` dans la section [« Créer un nouveau thread avec spawn »][thread-spawn]<!--
ignore
--> du chapitre 16, où la closure que nous avons passée à un autre thread commençait à s'exécuter immédiatement. C'est aussi différent de la façon dont beaucoup d'autres langages abordent l'async. Mais il est important que Rust puisse fournir ses garanties de performance, tout comme avec les itérateurs.

<!--
Once we have `response_text`, we can parse it into an instance of the `Html`
type using `Html::parse`. Instead of a raw string, we now have a data type we
can use to work with the HTML as a richer data structure. In particular, we can
use the `select_first` method to find the first instance of a given CSS
selector. By passing the string `"title"`, we'll get the first `<title>`
element in the document, if there is one. Because there may not be any matching
element, `select_first` returns an `Option<ElementRef>`. Finally, we use the
`Option::map` method, which lets us work with the item in the `Option` if it's
present, and do nothing if it isn't. (We could also use a `match` expression
here, but `map` is more idiomatic.) In the body of the function we supply to
`map`, we call `inner_html` on the `title` to get its content, which is a
`String`. When all is said and done, we have an `Option<String>`.
-->

Une fois que nous avons `response_text`, nous pouvons l'analyser en une instance du type `Html` en utilisant `Html::parse`. Au lieu d'une chaîne brute, nous avons maintenant un type de données que nous pouvons utiliser pour travailler avec le HTML comme une structure de données plus riche. En particulier, nous pouvons utiliser la méthode `select_first` pour trouver la première instance d'un sélecteur CSS donné. En passant la chaîne `"title"`, nous obtiendrons le premier élément `<title>` du document, s'il y en a un. Comme il peut ne pas y avoir d'élément correspondant, `select_first` retourne un `Option<ElementRef>`. Enfin, nous utilisons la méthode `Option::map`, qui nous permet de travailler avec l'élément dans l'`Option` s'il est présent, et de ne rien faire sinon. (Nous pourrions aussi utiliser une expression `match` ici, mais `map` est plus idiomatique.) Dans le corps de la fonction que nous fournissons à `map`, nous appelons `inner_html` sur le `title` pour obtenir son contenu, qui est une `String`. Au final, nous avons un `Option<String>`.

<!--
Notice that Rust's `await` keyword goes _after_ the expression you're awaiting,
not before it. That is, it's a _postfix_ keyword. This may differ from what
you're used to if you've used `async` in other languages, but in Rust it makes
chains of methods much nicer to work with. As a result, we could change the
body of `page_title` to chain the `trpl::get` and `text` function calls
together with `await` between them, as shown in Listing 17-2.
-->

Remarquez que le mot-clé `await` de Rust vient _après_ l'expression que vous attendez, et non avant. C'est-à-dire que c'est un mot-clé _postfixe_. Cela peut différer de ce à quoi vous êtes habitué si vous avez utilisé `async` dans d'autres langages, mais en Rust, cela rend les chaînes de méthodes beaucoup plus agréables à utiliser. En conséquence, nous pourrions modifier le corps de `page_title` pour chaîner les appels de fonctions `trpl::get` et `text` avec `await` entre eux, comme montré dans l'encart 17-2.

<Listing number="17-2" file-name="src/main.rs" caption="Chaînage avec le mot-clé `await`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

<!--
With that, we have successfully written our first async function! Before we add
some code in `main` to call it, let's talk a little more about what we've
written and what it means.
-->

Avec cela, nous avons écrit avec succès notre première fonction async ! Avant d'ajouter du code dans `main` pour l'appeler, parlons un peu plus de ce que nous avons écrit et de ce que cela signifie.

<!--
When Rust sees a _block_ marked with the `async` keyword, it compiles it into a
unique, anonymous data type that implements the `Future` trait. When Rust sees
a _function_ marked with `async`, it compiles it into a non-async function
whose body is an async block. An async function's return type is the type of
the anonymous data type the compiler creates for that async block.
-->

Quand Rust voit un _bloc_ marqué avec le mot-clé `async`, il le compile en un type de données unique et anonyme qui implémente le trait `Future`. Quand Rust voit une _fonction_ marquée avec `async`, il la compile en une fonction non-async dont le corps est un bloc async. Le type de retour d'une fonction async est le type du type de données anonyme que le compilateur crée pour ce bloc async.

<!--
Thus, writing `async fn` is equivalent to writing a function that returns a
_future_ of the return type. To the compiler, a function definition such as the
`async fn page_title` in Listing 17-1 is roughly equivalent to a non-async
function defined like this:
-->

Ainsi, écrire `async fn` est équivalent à écrire une fonction qui retourne une _future_ du type de retour. Pour le compilateur, une définition de fonction telle que `async fn page_title` dans l'encart 17-1 est à peu près équivalente à une fonction non-async définie comme ceci :

```rust
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

<!--
Let's walk through each part of the transformed version:
-->

Parcourons chaque partie de la version transformée :

<!--
- It uses the `impl Trait` syntax we discussed back in Chapter 10 in the
  ["Traits as Parameters"][impl-trait] ignore
--> section.
- The returned value implements the `Future` trait with an associated type of
  `Output`. Notice that the `Output` type is `Option<String>`, which is the
  same as the original return type from the `async fn` version of `page_title`.
- All of the code called in the body of the original function is wrapped in
  an `async move` block. Remember that blocks are expressions. This whole block
  is the expression returned from the function.
- This async block produces a value with the type `Option<String>`, as just
  described. That value matches the `Output` type in the return type. This is
  just like other blocks you have seen.
- The new function body is an `async move` block because of how it uses the
  `url` parameter. (We'll talk much more about `async` versus `async move`
  later in the chapter.)
-->

- Elle utilise la syntaxe `impl Trait` que nous avons abordée au chapitre 10 dans la section [« Les traits comme paramètres »][impl-trait]<!--
ignore
-->.
- La valeur retournée implémente le trait `Future` avec un type associé `Output`. Remarquez que le type `Output` est `Option<String>`, qui est le même que le type de retour original de la version `async fn` de `page_title`.
- Tout le code appelé dans le corps de la fonction originale est encapsulé dans un bloc `async move`. Rappelez-vous que les blocs sont des expressions. Ce bloc entier est l'expression retournée par la fonction.
- Ce bloc async produit une valeur de type `Option<String>`, comme décrit précédemment. Cette valeur correspond au type `Output` dans le type de retour. C'est comme les autres blocs que vous avez vus.
- Le nouveau corps de la fonction est un bloc `async move` en raison de la façon dont il utilise le paramètre `url`. (Nous parlerons beaucoup plus de `async` versus `async move` plus loin dans le chapitre.)

<!--
Now we can call `page_title` in `main`.
-->

Nous pouvons maintenant appeler `page_title` dans `main`.

<!--
Old headings. Do not remove or links may break.
-->

<a id ="determining-a-single-pages-title"></a>

<!--
### Executing an Async Function with a Runtime
-->

### Exécuter une fonction async avec un runtime

<!--
To start, we'll get the title for a single page, shown in Listing 17-3.
Unfortunately, this code doesn't compile yet.
-->

Pour commencer, nous allons obtenir le titre d'une seule page, comme montré dans l'encart 17-3. Malheureusement, ce code ne compile pas encore.

<Listing number="17-3" file-name="src/main.rs" caption="Appel de la fonction `page_title` depuis `main` avec un argument fourni par l'utilisateur">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

<!--
We follow the same pattern we used to get command line arguments in the
["Accepting Command Line Arguments"][cli-args] ignore
--> section in
Chapter 12. Then we pass the URL argument to `page_title` and await the result.
Because the value produced by the future is an `Option<String>`, we use a
`match` expression to print different messages to account for whether the page
had a `<title>`.
-->

Nous suivons le même schéma que celui utilisé pour obtenir les arguments de la ligne de commande dans la section [« Accepter les arguments de la ligne de commande »][cli-args]<!--
ignore
--> du chapitre 12. Ensuite, nous passons l'argument URL à `page_title` et attendons le résultat. Comme la valeur produite par la future est un `Option<String>`, nous utilisons une expression `match` pour afficher différents messages selon que la page avait ou non un `<title>`.

<!--
The only place we can use the `await` keyword is in async functions or blocks,
and Rust won't let us mark the special `main` function as `async`.
-->

Le seul endroit où nous pouvons utiliser le mot-clé `await` est dans les fonctions ou blocs async, et Rust ne nous laisse pas marquer la fonction spéciale `main` comme `async`.

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

<!--
The reason `main` can't be marked `async` is that async code needs a _runtime_:
a Rust crate that manages the details of executing asynchronous code. A
program's `main` function can _initialize_ a runtime, but it's not a runtime
_itself_. (We'll see more about why this is the case in a bit.) Every Rust
program that executes async code has at least one place where it sets up a
runtime that executes the futures.
-->

La raison pour laquelle `main` ne peut pas être marquée `async` est que le code async a besoin d'un _runtime_ : un crate Rust qui gère les détails de l'exécution du code asynchrone. La fonction `main` d'un programme peut _initialiser_ un runtime, mais elle n'est pas un runtime _elle-même_. (Nous verrons bientôt pourquoi c'est le cas.) Tout programme Rust qui exécute du code async a au moins un endroit où il configure un runtime qui exécute les futures.

<!--
Most languages that support async bundle a runtime, but Rust does not. Instead,
there are many different async runtimes available, each of which makes different
tradeoffs suitable to the use case it targets. For example, a high-throughput
web server with many CPU cores and a large amount of RAM has very different
needs than a microcontroller with a single core, a small amount of RAM, and no
heap allocation ability. The crates that provide those runtimes also often
supply async versions of common functionality such as file or network I/O.
-->

La plupart des langages qui supportent l'async intègrent un runtime, mais pas Rust. À la place, il existe de nombreux runtimes async différents disponibles, chacun faisant des compromis différents adaptés au cas d'utilisation qu'il cible. Par exemple, un serveur web à haut débit avec de nombreux cœurs CPU et une grande quantité de RAM a des besoins très différents d'un microcontrôleur avec un seul cœur, une petite quantité de RAM et aucune capacité d'allocation sur le tas. Les crates qui fournissent ces runtimes offrent souvent aussi des versions async de fonctionnalités courantes comme les E/S de fichiers ou réseau.

<!--
Here, and throughout the rest of this chapter, we'll use the `block_on`
function from the `trpl` crate, which takes a future as an argument and blocks
the current thread until this future runs to completion. Behind the scenes,
calling `block_on` sets up a runtime using the `tokio` crate that's used to run
the future passed in (the `trpl` crate's `block_on` behavior is similar to
other runtime crates' `block_on` functions). Once the future completes,
`block_on` returns whatever value the future produced.
-->

Ici, et tout au long du reste de ce chapitre, nous utiliserons la fonction `block_on` du crate `trpl`, qui prend une future en argument et bloque le thread courant jusqu'à ce que cette future s'exécute jusqu'à la fin. En coulisses, appeler `block_on` configure un runtime en utilisant le crate `tokio` qui est utilisé pour exécuter la future passée en paramètre (le comportement de `block_on` du crate `trpl` est similaire aux fonctions `block_on` d'autres crates de runtime). Une fois que la future est terminée, `block_on` retourne la valeur que la future a produite.

<!--
We could pass the future returned by `page_title` directly to `block_on` and,
once it completed, we could match on the resulting `Option<String>` as we tried
to do in Listing 17-3. However, for most of the examples in the chapter (and
most async code in the real world), we'll be doing more than just one async
function call, so instead we'll pass an `async` block and explicitly await the
result of the `page_title` call, as in Listing 17-4.
-->

Nous pourrions passer la future retournée par `page_title` directement à `block_on` et, une fois terminée, faire un match sur le `Option<String>` résultant comme nous avons essayé de le faire dans l'encart 17-3. Cependant, pour la plupart des exemples du chapitre (et la plupart du code async dans le monde réel), nous ferons plus qu'un simple appel de fonction async, donc à la place nous passerons un bloc `async` et attendrons explicitement le résultat de l'appel à `page_title`, comme dans l'encart 17-4.

<Listing number="17-4" caption="Attendre un bloc async avec `trpl::block_on`" file-name="src/main.rs">

<!--
should_panic,noplayground because mdbook test does not pass args
-->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

<!--
When we run this code, we get the behavior we expected initially:
-->

Quand nous exécutons ce code, nous obtenons le comportement attendu initialement :

<!--
manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run -- "https://www.rust-lang.org"
# copy the output here
-->

```console
$ cargo run -- "https://www.rust-lang.org"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

<!--
Phew—we finally have some working async code! But before we add the code to
race two sites against each other, let's briefly turn our attention back to how
futures work.
-->

Ouf — nous avons enfin du code async qui fonctionne ! Mais avant d'ajouter le code pour mettre en compétition deux sites l'un contre l'autre, tournons brièvement notre attention vers le fonctionnement des futures.

<!--
Each _await point_—that is, every place where the code uses the `await`
keyword—represents a place where control is handed back to the runtime. To make
that work, Rust needs to keep track of the state involved in the async block so
that the runtime could kick off some other work and then come back when it's
ready to try advancing the first one again. This is an invisible state machine,
as if you'd written an enum like this to save the current state at each await
point:
-->

Chaque _point d'attente_ — c'est-à-dire chaque endroit où le code utilise le mot-clé `await` — représente un endroit où le contrôle est rendu au runtime. Pour que cela fonctionne, Rust doit garder une trace de l'état impliqué dans le bloc async afin que le runtime puisse lancer un autre travail puis revenir quand il est prêt à essayer de faire avancer le premier à nouveau. C'est une machine à états invisible, comme si vous aviez écrit un enum comme celui-ci pour sauvegarder l'état courant à chaque point d'attente :

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

<!--
Writing the code to transition between each state by hand would be tedious and
error-prone, however, especially when you need to add more functionality and
more states to the code later. Fortunately, the Rust compiler creates and
manages the state machine data structures for async code automatically. The
normal borrowing and ownership rules around data structures all still apply,
and happily, the compiler also handles checking those for us and provides
useful error messages. We'll work through a few of those later in the chapter.
-->

Écrire le code pour effectuer la transition entre chaque état à la main serait fastidieux et source d'erreurs, surtout quand vous devez ajouter plus de fonctionnalités et plus d'états au code par la suite. Heureusement, le compilateur Rust crée et gère automatiquement les structures de données de la machine à états pour le code async. Les règles normales d'emprunt et de possession autour des structures de données s'appliquent toujours, et heureusement, le compilateur gère aussi la vérification de celles-ci pour nous et fournit des messages d'erreur utiles. Nous en verrons quelques-uns plus loin dans le chapitre.

<!--
Ultimately, something has to execute this state machine, and that something is
a runtime. (This is why you may come across mentions of _executors_ when
looking into runtimes: an executor is the part of a runtime responsible for
executing the async code.)
-->

Au final, quelque chose doit exécuter cette machine à états, et ce quelque chose est un runtime. (C'est pourquoi vous pouvez rencontrer des mentions d'_exécuteurs_ quand vous vous renseignez sur les runtimes : un exécuteur est la partie d'un runtime responsable de l'exécution du code async.)

<!--
Now you can see why the compiler stopped us from making `main` itself an async
function back in Listing 17-3. If `main` were an async function, something else
would need to manage the state machine for whatever future `main` returned, but
`main` is the starting point for the program! Instead, we called the
`trpl::block_on` function in `main` to set up a runtime and run the future
returned by the `async` block until it's done.
-->

Vous pouvez maintenant voir pourquoi le compilateur nous a empêchés de faire de `main` elle-même une fonction async dans l'encart 17-3. Si `main` était une fonction async, quelque chose d'autre devrait gérer la machine à états pour la future que `main` retournerait, mais `main` est le point d'entrée du programme ! À la place, nous avons appelé la fonction `trpl::block_on` dans `main` pour configurer un runtime et exécuter la future retournée par le bloc `async` jusqu'à ce qu'elle soit terminée.

<!--
> Note: Some runtimes provide macros so you _can_ write an async `main`
> function. Those macros rewrite `async fn main() { ... }` to be a normal `fn
> main`, which does the same thing we did by hand in Listing 17-4: call a
> function that runs a future to completion the way `trpl::block_on` does.
-->

> Remarque : certains runtimes fournissent des macros pour que vous _puissiez_ écrire une fonction `main` async. Ces macros réécrivent `async fn main() { ... }` en un `fn main` normal, qui fait la même chose que ce que nous avons fait manuellement dans l'encart 17-4 : appeler une fonction qui exécute une future jusqu'à son achèvement de la même manière que `trpl::block_on`.

<!--
Now let's put these pieces together and see how we can write concurrent code.
-->

Maintenant, assemblons ces éléments et voyons comment nous pouvons écrire du code concurrent.

<!--
Old headings. Do not remove or links may break.
-->

<a id="racing-our-two-urls-against-each-other"></a>

<!--
### Racing Two URLs Against Each Other Concurrently
-->

### Mettre en compétition deux URL de manière concurrente

<!--
In Listing 17-5, we call `page_title` with two different URLs passed in from the
command line and race them by selecting whichever future finishes first.
-->

Dans l'encart 17-5, nous appelons `page_title` avec deux URL différentes passées depuis la ligne de commande et les mettons en compétition en sélectionnant la future qui termine en premier.

<Listing number="17-5" caption="Appel de `page_title` pour deux URL pour voir laquelle retourne en premier" file-name="src/main.rs">

<!--
should_panic,noplayground because mdbook does not pass args
-->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

<!--
We begin by calling `page_title` for each of the user-supplied URLs. We save
the resulting futures as `title_fut_1` and `title_fut_2`. Remember, these don't
do anything yet, because futures are lazy and we haven't yet awaited them. Then
we pass the futures to `trpl::select`, which returns a value to indicate which
of the futures passed to it finishes first.
-->

Nous commençons par appeler `page_title` pour chacune des URL fournies par l'utilisateur. Nous sauvegardons les futures résultantes sous les noms `title_fut_1` et `title_fut_2`. Rappelez-vous, elles ne font encore rien, car les futures sont paresseuses et nous ne les avons pas encore attendues. Ensuite, nous passons les futures à `trpl::select`, qui retourne une valeur indiquant laquelle des futures passées en paramètre se termine en premier.

<!--
> Note: Under the hood, `trpl::select` is built on a more general `select`
> function defined in the `futures` crate. The `futures` crate's `select`
> function can do a lot of things that the `trpl::select` function can't, but
> it also has some additional complexity that we can skip over for now.
-->

> Remarque : en coulisses, `trpl::select` est construit sur une fonction `select` plus générale définie dans le crate `futures`. La fonction `select` du crate `futures` peut faire beaucoup de choses que la fonction `trpl::select` ne peut pas, mais elle a aussi une complexité supplémentaire que nous pouvons ignorer pour l'instant.

<!--
Either future can legitimately "win," so it doesn't make sense to return a
`Result`. Instead, `trpl::select` returns a type we haven't seen before,
`trpl::Either`. The `Either` type is somewhat similar to a `Result` in that it
has two cases. Unlike `Result`, though, there is no notion of success or
failure baked into `Either`. Instead, it uses `Left` and `Right` to indicate
"one or the other":
-->

L'une ou l'autre des futures peut légitimement « gagner », donc il n'est pas logique de retourner un `Result`. À la place, `trpl::select` retourne un type que nous n'avons pas vu auparavant, `trpl::Either`. Le type `Either` est quelque peu similaire à un `Result` en ce qu'il a deux cas. Contrairement à `Result`, cependant, il n'y a aucune notion de succès ou d'échec intégrée dans `Either`. À la place, il utilise `Left` et `Right` pour indiquer « l'un ou l'autre » :

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

<!--
The `select` function returns `Left` with that future's output if the first
argument wins, and `Right` with the second future argument's output if _that_
one wins. This matches the order the arguments appear in when calling the
function: the first argument is to the left of the second argument.
-->

La fonction `select` retourne `Left` avec la sortie de cette future si le premier argument gagne, et `Right` avec la sortie du deuxième argument future si _celui-là_ gagne. Cela correspond à l'ordre dans lequel les arguments apparaissent lors de l'appel de la fonction : le premier argument est à gauche du deuxième argument.

<!--
We also update `page_title` to return the same URL passed in. That way, if the
page that returns first does not have a `<title>` we can resolve, we can still
print a meaningful message. With that information available, we wrap up by
updating our `println!` output to indicate both which URL finished first and
what, if any, the `<title>` is for the web page at that URL.
-->

Nous mettons aussi à jour `page_title` pour retourner la même URL passée en paramètre. De cette façon, si la page qui retourne en premier n'a pas de `<title>` que nous pouvons résoudre, nous pouvons quand même afficher un message significatif. Avec ces informations disponibles, nous terminons en mettant à jour notre sortie `println!` pour indiquer à la fois quelle URL a terminé en premier et quel est, le cas échéant, le `<title>` de la page web à cette URL.

<!--
You have built a small working web scraper now! Pick a couple URLs and run the
command line tool. You may discover that some sites are consistently faster
than others, while in other cases the faster site varies from run to run. More
importantly, you've learned the basics of working with futures, so now we can
dig deeper into what we can do with async.
-->

Vous avez maintenant construit un petit web scraper fonctionnel ! Choisissez quelques URL et exécutez l'outil en ligne de commande. Vous découvrirez peut-être que certains sites sont systématiquement plus rapides que d'autres, tandis que dans d'autres cas, le site le plus rapide varie d'une exécution à l'autre. Plus important encore, vous avez appris les bases du travail avec les futures, et nous pouvons maintenant approfondir ce que nous pouvons faire avec l'async.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html

<!--
TODO: map source link version to version of Rust?
-->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
