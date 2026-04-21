<!--
# The Rust Programming Language
-->

# Le langage de programmation Rust

<!--
[The Rust Programming Language](title-page.md)
-->

[Le langage de programmation Rust](title-page.md)

<!--
[Foreword](foreword.md)
-->

[Avant-propos](foreword.md)

<!--
[Introduction](ch00-00-introduction.md)
-->

[Introduction](ch00-00-introduction.md)

<!--
- [Getting Started](ch01-00-getting-started.md)
  - [Installation](ch01-01-installation.md)
  - [Hello, World!](ch01-02-hello-world.md)
  - [Hello, Cargo!](ch01-03-hello-cargo.md)
-->

- [Mise en route](ch01-00-getting-started.md)
  - [Installation](ch01-01-installation.md)
  - [Hello, World!](ch01-02-hello-world.md)
  - [Hello, Cargo!](ch01-03-hello-cargo.md)

<!--
- [Programming a Guessing Game](ch02-00-guessing-game-tutorial.md)
-->

- [Programmer un jeu de devinettes](ch02-00-guessing-game-tutorial.md)

<!--
- [Common Programming Concepts](ch03-00-common-programming-concepts.md)
  - [Variables and Mutability](ch03-01-variables-and-mutability.md)
  - [Data Types](ch03-02-data-types.md)
  - [Functions](ch03-03-how-functions-work.md)
  - [Comments](ch03-04-comments.md)
  - [Control Flow](ch03-05-control-flow.md)
-->

- [Les concepts courants de programmation](ch03-00-common-programming-concepts.md)
  - [Les variables et la mutabilité](ch03-01-variables-and-mutability.md)
  - [Les types de données](ch03-02-data-types.md)
  - [Les fonctions](ch03-03-how-functions-work.md)
  - [Les commentaires](ch03-04-comments.md)
  - [Les structures de contrôle](ch03-05-control-flow.md)

<!--
- [Understanding Ownership](ch04-00-understanding-ownership.md)
  - [What is Ownership?](ch04-01-what-is-ownership.md)
  - [References and Borrowing](ch04-02-references-and-borrowing.md)
  - [The Slice Type](ch04-03-slices.md)
-->

- [Comprendre l'ownership](ch04-00-understanding-ownership.md)
  - [Qu'est-ce que l'ownership ?](ch04-01-what-is-ownership.md)
  - [Les références et l'emprunt](ch04-02-references-and-borrowing.md)
  - [Le type slice](ch04-03-slices.md)

<!--
- [Using Structs to Structure Related Data](ch05-00-structs.md)
  - [Defining and Instantiating Structs](ch05-01-defining-structs.md)
  - [An Example Program Using Structs](ch05-02-example-structs.md)
  - [Methods](ch05-03-method-syntax.md)
-->

- [Utiliser les structs pour structurer des données apparentées](ch05-00-structs.md)
  - [Définir et instancier des structs](ch05-01-defining-structs.md)
  - [Un exemple de programme utilisant des structs](ch05-02-example-structs.md)
  - [Les méthodes](ch05-03-method-syntax.md)

<!--
- [Enums and Pattern Matching](ch06-00-enums.md)
  - [Defining an Enum](ch06-01-defining-an-enum.md)
  - [The `match` Control Flow Construct](ch06-02-match.md)
  - [Concise Control Flow with `if let` and `let...else`](ch06-03-if-let.md)
-->

- [Les enums et le filtrage par motif](ch06-00-enums.md)
  - [Définir une enum](ch06-01-defining-an-enum.md)
  - [La structure de contrôle `match`](ch06-02-match.md)
  - [Un contrôle de flux concis avec `if let` et `let...else`](ch06-03-if-let.md)

<!--
- [Packages, Crates, and Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Packages and Crates](ch07-01-packages-and-crates.md)
  - [Control Scope and Privacy with Modules](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Paths for Referring to an Item in the Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [Bringing Paths Into Scope with the `use` Keyword](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.md)
-->

- [Les packages, les crates et les modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Les packages et les crates](ch07-01-packages-and-crates.md)
  - [Contrôler la portée et la visibilité avec les modules](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Les chemins pour désigner un élément dans l'arborescence des modules](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [Importer des chemins dans la portée avec le mot-clé `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [Séparer les modules dans différents fichiers](ch07-05-separating-modules-into-different-files.md)

<!--
- [Common Collections](ch08-00-common-collections.md)
  - [Storing Lists of Values with Vectors](ch08-01-vectors.md)
  - [Storing UTF-8 Encoded Text with Strings](ch08-02-strings.md)
  - [Storing Keys with Associated Values in Hash Maps](ch08-03-hash-maps.md)
-->

- [Les collections courantes](ch08-00-common-collections.md)
  - [Stocker des listes de valeurs avec les vecteurs](ch08-01-vectors.md)
  - [Stocker du texte encodé en UTF-8 avec les String](ch08-02-strings.md)
  - [Stocker des clés associées à des valeurs dans des tables de hachage](ch08-03-hash-maps.md)

<!--
- [Error Handling](ch09-00-error-handling.md)
  - [Unrecoverable Errors with `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [Recoverable Errors with `Result`](ch09-02-recoverable-errors-with-result.md)
  - [To `panic!` or Not to `panic!`](ch09-03-to-panic-or-not-to-panic.md)
-->

- [La gestion des erreurs](ch09-00-error-handling.md)
  - [Les erreurs irrécupérables avec `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [Les erreurs récupérables avec `Result`](ch09-02-recoverable-errors-with-result.md)
  - [`panic!` ou ne pas `panic!`](ch09-03-to-panic-or-not-to-panic.md)

<!--
- [Generic Types, Traits, and Lifetimes](ch10-00-generics.md)
  - [Generic Data Types](ch10-01-syntax.md)
  - [Defining Shared Behavior with Traits](ch10-02-traits.md)
  - [Validating References with Lifetimes](ch10-03-lifetime-syntax.md)
-->

- [Les types génériques, les traits et les durées de vie](ch10-00-generics.md)
  - [Les types de données génériques](ch10-01-syntax.md)
  - [Définir un comportement partagé avec les traits](ch10-02-traits.md)
  - [Valider les références avec les durées de vie](ch10-03-lifetime-syntax.md)

<!--
- [Writing Automated Tests](ch11-00-testing.md)
  - [How to Write Tests](ch11-01-writing-tests.md)
  - [Controlling How Tests Are Run](ch11-02-running-tests.md)
  - [Test Organization](ch11-03-test-organization.md)
-->

- [Écrire des tests automatisés](ch11-00-testing.md)
  - [Comment écrire des tests](ch11-01-writing-tests.md)
  - [Contrôler l'exécution des tests](ch11-02-running-tests.md)
  - [L'organisation des tests](ch11-03-test-organization.md)

<!--
- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md)
  - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md)
  - [Reading a File](ch12-02-reading-a-file.md)
  - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md)
  - [Adding Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
  - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
  - [Redirecting Errors to Standard Error](ch12-06-writing-to-stderr-instead-of-stdout.md)
-->

- [Un projet d'E/S : construire un programme en ligne de commande](ch12-00-an-io-project.md)
  - [Accepter des arguments en ligne de commande](ch12-01-accepting-command-line-arguments.md)
  - [Lire un fichier](ch12-02-reading-a-file.md)
  - [Refactoriser pour améliorer la modularité et la gestion des erreurs](ch12-03-improving-error-handling-and-modularity.md)
  - [Ajouter des fonctionnalités avec le développement piloté par les tests](ch12-04-testing-the-librarys-functionality.md)
  - [Travailler avec les variables d'environnement](ch12-05-working-with-environment-variables.md)
  - [Rediriger les erreurs vers la sortie d'erreur standard](ch12-06-writing-to-stderr-instead-of-stdout.md)

<!--
- [Functional Language Features: Iterators and Closures](ch13-00-functional-features.md)
  - [Closures](ch13-01-closures.md)
  - [Processing a Series of Items with Iterators](ch13-02-iterators.md)
  - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
  - [Performance in Loops vs. Iterators](ch13-04-performance.md)
-->

- [Les fonctionnalités de langage fonctionnel : les itérateurs et les closures](ch13-00-functional-features.md)
  - [Les closures](ch13-01-closures.md)
  - [Traiter une série d'éléments avec les itérateurs](ch13-02-iterators.md)
  - [Améliorer notre projet d'E/S](ch13-03-improving-our-io-project.md)
  - [Performances : boucles vs. itérateurs](ch13-04-performance.md)

<!--
- [More about Cargo and Crates.io](ch14-00-more-about-cargo.md)
  - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
  - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
  - [Installing Binaries with `cargo install`](ch14-04-installing-binaries.md)
  - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md)
-->

- [En savoir plus sur Cargo et Crates.io](ch14-00-more-about-cargo.md)
  - [Personnaliser les compilations avec les profils de publication](ch14-01-release-profiles.md)
  - [Publier une crate sur Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Les espaces de travail Cargo](ch14-03-cargo-workspaces.md)
  - [Installer des binaires avec `cargo install`](ch14-04-installing-binaries.md)
  - [Étendre Cargo avec des commandes personnalisées](ch14-05-extending-cargo.md)

<!--
- [Smart Pointers](ch15-00-smart-pointers.md)
  - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
  - [Treating Smart Pointers Like Regular References](ch15-02-deref.md)
  - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
  - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
  - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
  - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)
-->

- [Les pointeurs intelligents](ch15-00-smart-pointers.md)
  - [Utiliser `Box<T>` pour pointer vers des données sur le tas](ch15-01-box.md)
  - [Traiter les pointeurs intelligents comme des références classiques](ch15-02-deref.md)
  - [Exécuter du code au nettoyage avec le trait `Drop`](ch15-03-drop.md)
  - [`Rc<T>`, le pointeur intelligent à compteur de références](ch15-04-rc.md)
  - [`RefCell<T>` et le patron de mutabilité intérieure](ch15-05-interior-mutability.md)
  - [Les cycles de références peuvent provoquer des fuites de mémoire](ch15-06-reference-cycles.md)

<!--
- [Fearless Concurrency](ch16-00-concurrency.md)
  - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
  - [Transfer Data Between Threads with Message Passing](ch16-02-message-passing.md)
  - [Shared-State Concurrency](ch16-03-shared-state.md)
  - [Extensible Concurrency with `Send` and `Sync`](ch16-04-extensible-concurrency-sync-and-send.md)
-->

- [La concurrence sans crainte](ch16-00-concurrency.md)
  - [Utiliser les tâches pour exécuter du code simultanément](ch16-01-threads.md)
  - [Transférer des données entre les tâches avec le passage de messages](ch16-02-message-passing.md)
  - [La concurrence à état partagé](ch16-03-shared-state.md)
  - [La concurrence extensible avec `Send` et `Sync`](ch16-04-extensible-concurrency-sync-and-send.md)

<!--
- [Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams](ch17-00-async-await.md)
  - [Futures and the Async Syntax](ch17-01-futures-and-syntax.md)
  - [Applying Concurrency with Async](ch17-02-concurrency-with-async.md)
  - [Working With Any Number of Futures](ch17-03-more-futures.md)
  - [Streams: Futures in Sequence](ch17-04-streams.md)
  - [A Closer Look at the Traits for Async](ch17-05-traits-for-async.md)
  - [Futures, Tasks, and Threads](ch17-06-futures-tasks-threads.md)
-->

- [Les fondamentaux de la programmation asynchrone : Async, Await, Futures et Streams](ch17-00-async-await.md)
  - [Les futures et la syntaxe async](ch17-01-futures-and-syntax.md)
  - [Appliquer la concurrence avec async](ch17-02-concurrency-with-async.md)
  - [Travailler avec un nombre quelconque de futures](ch17-03-more-futures.md)
  - [Les streams : des futures en séquence](ch17-04-streams.md)
  - [Un regard approfondi sur les traits pour l'async](ch17-05-traits-for-async.md)
  - [Les futures, les tâches et les tâches de fond](ch17-06-futures-tasks-threads.md)

<!--
- [Object Oriented Programming Features](ch18-00-oop.md)
  - [Characteristics of Object-Oriented Languages](ch18-01-what-is-oo.md)
  - [Using Trait Objects to Abstract over Shared Behavior](ch18-02-trait-objects.md)
  - [Implementing an Object-Oriented Design Pattern](ch18-03-oo-design-patterns.md)
-->

- [Les fonctionnalités de la programmation orientée objet](ch18-00-oop.md)
  - [Les caractéristiques des langages orientés objet](ch18-01-what-is-oo.md)
  - [Utiliser des objets trait pour abstraire un comportement commun](ch18-02-trait-objects.md)
  - [Implémenter un patron de conception orienté objet](ch18-03-oo-design-patterns.md)

<!--
- [Patterns and Matching](ch19-00-patterns.md)
  - [All the Places Patterns Can Be Used](ch19-01-all-the-places-for-patterns.md)
  - [Refutability: Whether a Pattern Might Fail to Match](ch19-02-refutability.md)
  - [Pattern Syntax](ch19-03-pattern-syntax.md)
-->

- [Les motifs et le filtrage](ch19-00-patterns.md)
  - [Tous les endroits où les motifs peuvent être utilisés](ch19-01-all-the-places-for-patterns.md)
  - [La réfutabilité : quand un motif pourrait ne pas correspondre](ch19-02-refutability.md)
  - [La syntaxe des motifs](ch19-03-pattern-syntax.md)

<!--
- [Advanced Features](ch20-00-advanced-features.md)
  - [Unsafe Rust](ch20-01-unsafe-rust.md)
  - [Advanced Traits](ch20-02-advanced-traits.md)
  - [Advanced Types](ch20-03-advanced-types.md)
  - [Advanced Functions and Closures](ch20-04-advanced-functions-and-closures.md)
  - [Macros](ch20-05-macros.md)
-->

- [Les fonctionnalités avancées](ch20-00-advanced-features.md)
  - [Le Rust unsafe](ch20-01-unsafe-rust.md)
  - [Les traits avancés](ch20-02-advanced-traits.md)
  - [Les types avancés](ch20-03-advanced-types.md)
  - [Les fonctions et les closures avancées](ch20-04-advanced-functions-and-closures.md)
  - [Les macros](ch20-05-macros.md)

<!--
- [Final Project: Building a Multithreaded Web Server](ch21-00-final-project-a-web-server.md)
  - [Building a Single-Threaded Web Server](ch21-01-single-threaded.md)
  - [From Single-Threaded to Multithreaded Server](ch21-02-multithreaded.md)
  - [Graceful Shutdown and Cleanup](ch21-03-graceful-shutdown-and-cleanup.md)
-->

- [Projet final : construire un serveur web multitâche](ch21-00-final-project-a-web-server.md)
  - [Construire un serveur web monotâche](ch21-01-single-threaded.md)
  - [Du serveur monotâche au serveur multitâche](ch21-02-multithreaded.md)
  - [Arrêt gracieux et nettoyage](ch21-03-graceful-shutdown-and-cleanup.md)

<!--
- [Appendix](appendix-00.md)
  - [A - Keywords](appendix-01-keywords.md)
  - [B - Operators and Symbols](appendix-02-operators.md)
  - [C - Derivable Traits](appendix-03-derivable-traits.md)
  - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
  - [E - Editions](appendix-05-editions.md)
  - [F - Translations of the Book](appendix-06-translation.md)
  - [G - How Rust is Made and "Nightly Rust"](appendix-07-nightly-rust.md)
-->

- [Annexes](appendix-00.md)
  - [A - Les mots-clés](appendix-01-keywords.md)
  - [B - Les opérateurs et les symboles](appendix-02-operators.md)
  - [C - Les traits dérivables](appendix-03-derivable-traits.md)
  - [D - Les outils de développement utiles](appendix-04-useful-development-tools.md)
  - [E - Les éditions](appendix-05-editions.md)
  - [F - Les traductions du livre](appendix-06-translation.md)
  - [G - Comment Rust est conçu et "Rust Nightly"](appendix-07-nightly-rust.md)
