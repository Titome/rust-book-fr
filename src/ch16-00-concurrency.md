<!--
# Fearless Concurrency
-->

# La concurrence sans crainte

<!--
Handling concurrent programming safely and efficiently is another of Rust's
major goals. _Concurrent programming_, in which different parts of a program
execute independently, and _parallel programming_, in which different parts of
a program execute at the same time, are becoming increasingly important as more
computers take advantage of their multiple processors. Historically,
programming in these contexts has been difficult and error-prone. Rust hopes to
change that.
-->

Gérer la programmation concurrente de manière sûre et efficace est un autre des objectifs majeurs de Rust. La _programmation concurrente_, dans laquelle différentes parties d'un programme s'exécutent indépendamment, et la _programmation parallèle_, dans laquelle différentes parties d'un programme s'exécutent en même temps, deviennent de plus en plus importantes à mesure que les ordinateurs tirent parti de leurs multiples processeurs. Historiquement, programmer dans ces contextes a été difficile et source d'erreurs. Rust espère changer cela.

<!--
Initially, the Rust team thought that ensuring memory safety and preventing
concurrency problems were two separate challenges to be solved with different
methods. Over time, the team discovered that the ownership and type systems are
a powerful set of tools to help manage memory safety _and_ concurrency
problems! By leveraging ownership and type checking, many concurrency errors
are compile-time errors in Rust rather than runtime errors. Therefore, rather
than making you spend lots of time trying to reproduce the exact circumstances
under which a runtime concurrency bug occurs, incorrect code will refuse to
compile and present an error explaining the problem. As a result, you can fix
your code while you're working on it rather than potentially after it has been
shipped to production. We've nicknamed this aspect of Rust _fearless
concurrency_. Fearless concurrency allows you to write code that is free of
subtle bugs and is easy to refactor without introducing new bugs.
-->

Au départ, l'équipe Rust pensait que garantir la sécurité de la mémoire et prévenir les problèmes de concurrence étaient deux défis distincts à résoudre avec des méthodes différentes. Au fil du temps, l'équipe a découvert que les systèmes de possession et de types constituent un ensemble d'outils puissants pour gérer la sécurité de la mémoire _et_ les problèmes de concurrence ! En tirant parti de la possession et de la vérification de types, de nombreuses erreurs de concurrence sont des erreurs de compilation en Rust plutôt que des erreurs d'exécution. Par conséquent, plutôt que de vous faire passer beaucoup de temps à essayer de reproduire les circonstances exactes dans lesquelles un bogue de concurrence à l'exécution se produit, le code incorrect refusera de compiler et présentera une erreur expliquant le problème. En conséquence, vous pouvez corriger votre code pendant que vous travaillez dessus plutôt que potentiellement après qu'il a été déployé en production. Nous avons surnommé cet aspect de Rust _la concurrence sans crainte_ (fearless concurrency). La concurrence sans crainte vous permet d'écrire du code exempt de bogues subtils et facile à refactoriser sans introduire de nouveaux bogues.

<!--
> Note: For simplicity's sake, we'll refer to many of the problems as
> _concurrent_ rather than being more precise by saying _concurrent and/or
> parallel_. For this chapter, please mentally substitute _concurrent and/or
> parallel_ whenever we use _concurrent_. In the next chapter, where the
> distinction matters more, we'll be more specific.
-->

> Remarque : par souci de simplicité, nous désignerons bon nombre de problèmes comme
> _concurrents_ plutôt que d'être plus précis en disant _concurrents et/ou
> parallèles_. Pour ce chapitre, veuillez mentalement substituer _concurrent et/ou
> parallèle_ chaque fois que nous utilisons _concurrent_. Dans le chapitre suivant, où
> la distinction est plus importante, nous serons plus précis.

<!--
Many languages are dogmatic about the solutions they offer for handling
concurrent problems. For example, Erlang has elegant functionality for
message-passing concurrency but has only obscure ways to share state between
threads. Supporting only a subset of possible solutions is a reasonable
strategy for higher-level languages because a higher-level language promises
benefits from giving up some control to gain abstractions. However, lower-level
languages are expected to provide the solution with the best performance in any
given situation and have fewer abstractions over the hardware. Therefore, Rust
offers a variety of tools for modeling problems in whatever way is appropriate
for your situation and requirements.
-->

De nombreux langages sont dogmatiques quant aux solutions qu'ils proposent pour gérer les problèmes de concurrence. Par exemple, Erlang dispose de fonctionnalités élégantes pour la concurrence par passage de messages mais n'offre que des moyens obscurs de partager l'état entre les threads. Ne supporter qu'un sous-ensemble de solutions possibles est une stratégie raisonnable pour les langages de haut niveau car un langage de haut niveau promet des avantages en abandonnant un certain contrôle pour gagner en abstraction. Cependant, les langages de bas niveau sont censés fournir la solution offrant les meilleures performances dans n'importe quelle situation et disposent de moins d'abstractions par rapport au matériel. Par conséquent, Rust offre une variété d'outils pour modéliser les problèmes de la manière la plus appropriée à votre situation et vos besoins.

<!--
Here are the topics we'll cover in this chapter:

- How to create threads to run multiple pieces of code at the same time
- _Message-passing_ concurrency, where channels send messages between threads
- _Shared-state_ concurrency, where multiple threads have access to some piece
  of data
- The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to
  user-defined types as well as types provided by the standard library
-->

Voici les sujets que nous aborderons dans ce chapitre :

- Comment créer des threads pour exécuter plusieurs morceaux de code en même temps
- La concurrence par _passage de messages_, où des canaux envoient des messages entre les threads
- La concurrence par _état partagé_, où plusieurs threads ont accès à une même donnée
- Les traits `Sync` et `Send`, qui étendent les garanties de concurrence de Rust aux types définis par l'utilisateur ainsi qu'aux types fournis par la bibliothèque standard
