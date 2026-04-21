<!--
# Introduction
-->

# Introduction

<!--
> Note: This edition of the book is the same as [The Rust Programming
> Language][nsprust] available in print and ebook format from [No Starch
> Press][nsp].
-->

> Note : cette édition du livre est identique à [The Rust Programming
> Language][nsprust] disponible en format papier et numérique chez [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

<!--
Welcome to _The Rust Programming Language_, an introductory book about Rust.
The Rust programming language helps you write faster, more reliable software.
High-level ergonomics and low-level control are often at odds in programming
language design; Rust challenges that conflict. Through balancing powerful
technical capacity and a great developer experience, Rust gives you the option
to control low-level details (such as memory usage) without all the hassle
traditionally associated with such control.
-->

Bienvenue dans _Le langage de programmation Rust_, un livre d'introduction à
Rust. Le langage de programmation Rust vous aide à écrire des logiciels plus
rapides et plus fiables. L'ergonomie de haut niveau et le contrôle de bas niveau
sont souvent en opposition dans la conception des langages de programmation ;
Rust remet en question ce conflit. En équilibrant une puissante capacité
technique et une excellente expérience de développement, Rust vous donne la
possibilité de contrôler les détails de bas niveau (comme l'utilisation de la
mémoire) sans toutes les contraintes traditionnellement associées à ce type de
contrôle.

<!--
## Who Rust Is For
-->

## À qui s'adresse Rust

<!--
Rust is ideal for many people for a variety of reasons. Let's look at a few of
the most important groups.
-->

Rust est idéal pour de nombreuses personnes et pour des raisons variées.
Examinons quelques-uns des groupes les plus importants.

<!--
### Teams of Developers
-->

### Les équipes de développeurs

<!--
Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to various subtle bugs, which in most other languages can only be
caught through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these elusive bugs, including concurrency bugs. By working
alongside the compiler, the team can spend its time focusing on the program's
logic rather than chasing down bugs.
-->

Rust s'avère être un outil productif pour la collaboration au sein de grandes
équipes de développeurs avec des niveaux variés de connaissances en programmation
système. Le code de bas niveau est sujet à divers bugs subtils, qui dans la
plupart des autres langages ne peuvent être détectés qu'au travers de tests
approfondis et de revues de code minutieuses par des développeurs expérimentés.
En Rust, le compilateur joue un rôle de gardien en refusant de compiler le code
contenant ces bugs insaisissables, y compris les bugs de concurrence. En
travaillant avec le compilateur, l'équipe peut consacrer son temps à se
concentrer sur la logique du programme plutôt qu'à traquer des bugs.

<!--
Rust also brings contemporary developer tools to the systems programming world:
-->

Rust apporte également des outils de développement modernes au monde de la
programmation système :

<!--
- Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
- The `rustfmt` formatting tool ensures a consistent coding style across
  developers.
- The Rust Language Server powers integrated development environment (IDE)
  integration for code completion and inline error messages.
-->

- Cargo, le gestionnaire de dépendances et outil de compilation intégré, rend
  l'ajout, la compilation et la gestion des dépendances simples et cohérents à
  travers tout l'écosystème Rust.
- L'outil de formatage `rustfmt` assure un style de code cohérent entre les
  développeurs.
- Le Rust Language Server alimente l'intégration avec les environnements de
  développement intégrés (IDE) pour la complétion de code et les messages
  d'erreur en ligne.

<!--
By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.
-->

En utilisant ces outils et d'autres dans l'écosystème Rust, les développeurs
peuvent être productifs tout en écrivant du code au niveau système.

<!--
### Students
-->

### Les étudiants

<!--
Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
students' questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.
-->

Rust s'adresse aux étudiants et à ceux qui souhaitent apprendre les concepts
système. Grâce à Rust, de nombreuses personnes ont appris des sujets comme le
développement de systèmes d'exploitation. La communauté est très accueillante et
se fait un plaisir de répondre aux questions des étudiants. À travers des
initiatives comme ce livre, les équipes Rust souhaitent rendre les concepts
système plus accessibles à un plus grand nombre de personnes, en particulier
celles qui débutent en programmation.

<!--
### Companies
-->

### Les entreprises

<!--
Hundreds of companies, large and small, use Rust in production for a variety of
tasks, including command line tools, web services, DevOps tooling, embedded
devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.
-->

Des centaines d'entreprises, grandes et petites, utilisent Rust en production
pour une variété de tâches, notamment des outils en ligne de commande, des
services web, des outils DevOps, des systèmes embarqués, de l'analyse et du
transcodage audio et vidéo, des cryptomonnaies, de la bio-informatique, des
moteurs de recherche, des applications pour l'Internet des objets, de
l'apprentissage automatique, et même des parties majeures du navigateur web
Firefox.

<!--
### Open Source Developers
-->

### Les développeurs open source

<!--
Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We'd love to have you contribute to the Rust
language.
-->

Rust s'adresse aux personnes qui veulent contribuer au langage de programmation
Rust, à sa communauté, à ses outils de développement et à ses bibliothèques.
Nous serions ravis que vous contribuiez au langage Rust.

<!--
### People Who Value Speed and Stability
-->

### Les personnes qui valorisent la vitesse et la stabilité

<!--
Rust is for people who crave speed and stability in a language. By speed, we
mean both how quickly Rust code can run and the speed at which Rust lets you
write programs. The Rust compiler's checks ensure stability through feature
additions and refactoring. This is in contrast to the brittle legacy code in
languages without these checks, which developers are often afraid to modify. By
striving for zero-cost abstractions—higher-level features that compile to
lower-level code as fast as code written manually—Rust endeavors to make safe
code be fast code as well.
-->

Rust s'adresse aux personnes qui recherchent la vitesse et la stabilité dans un
langage. Par vitesse, nous entendons à la fois la rapidité d'exécution du code
Rust et la rapidité avec laquelle Rust vous permet d'écrire des programmes. Les
vérifications du compilateur Rust garantissent la stabilité lors de l'ajout de
fonctionnalités et du refactoring. Cela contraste avec le code hérité fragile
dans les langages dépourvus de ces vérifications, que les développeurs ont
souvent peur de modifier. En visant des abstractions à coût nul — des
fonctionnalités de haut niveau qui compilent en code de bas niveau aussi rapide
que du code écrit manuellement — Rust s'efforce de faire en sorte que le code sûr
soit également du code rapide.

<!--
The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust's greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety _and_ productivity, speed _and_ ergonomics. Give
Rust a try, and see if its choices work for you.
-->

Le langage Rust espère également prendre en charge de nombreux autres
utilisateurs ; ceux mentionnés ici ne sont que quelques-unes des parties
prenantes les plus importantes. Globalement, la plus grande ambition de Rust est
d'éliminer les compromis que les programmeurs ont acceptés pendant des décennies
en offrant la sûreté _et_ la productivité, la vitesse _et_ l'ergonomie. Essayez
Rust et voyez si ses choix vous conviennent.

<!--
## Who This Book Is For
-->

## À qui s'adresse ce livre

<!--
This book assumes that you've written code in another programming language, but
it doesn't make any assumptions about which one. We've tried to make the
material broadly accessible to those from a wide variety of programming
backgrounds. We don't spend a lot of time talking about what programming _is_
or how to think about it. If you're entirely new to programming, you would be
better served by reading a book that specifically provides an introduction to
programming.
-->

Ce livre suppose que vous avez déjà écrit du code dans un autre langage de
programmation, mais ne fait aucune supposition sur lequel. Nous avons essayé de
rendre le contenu largement accessible aux personnes venant d'horizons de
programmation variés. Nous ne passons pas beaucoup de temps à parler de ce qu'_est_
la programmation ou de comment y réfléchir. Si vous êtes complètement novice en
programmation, il serait préférable de lire un livre qui offre spécifiquement une
introduction à la programmation.

<!--
## How to Use This Book
-->

## Comment utiliser ce livre

<!--
In general, this book assumes that you're reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a particular topic but will revisit
the topic in a later chapter.
-->

En général, ce livre suppose que vous le lisez dans l'ordre, du début à la fin.
Les chapitres ultérieurs s'appuient sur les concepts des chapitres précédents,
et les chapitres précédents pourraient ne pas approfondir un sujet particulier
mais y reviendront dans un chapitre ultérieur.

<!--
You'll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you'll learn about an aspect of Rust. In project
chapters, we'll build small programs together, applying what you've learned so
far. Chapter 2, Chapter 12, and Chapter 21 are project chapters; the rest are
concept chapters.
-->

Vous trouverez deux types de chapitres dans ce livre : des chapitres
conceptuels et des chapitres de projet. Dans les chapitres conceptuels, vous
apprendrez un aspect de Rust. Dans les chapitres de projet, nous construirons
ensemble de petits programmes, en appliquant ce que vous avez appris jusque-là.
Les chapitres 2, 12 et 21 sont des chapitres de projet ; les autres sont des
chapitres conceptuels.

<!--
**Chapter 1** explains how to install Rust, how to write a "Hello, world!"
program, and how to use Cargo, Rust's package manager and build tool. **Chapter
2** is a hands-on introduction to writing a program in Rust, having you build
up a number-guessing game. Here, we cover concepts at a high level, and later
chapters will provide additional detail. If you want to get your hands dirty
right away, Chapter 2 is the place for that. If you're a particularly
meticulous learner who prefers to learn every detail before moving on to the
next, you might want to skip Chapter 2 and go straight to **Chapter 3**, which
covers Rust features that are similar to those of other programming languages;
then, you can return to Chapter 2 when you'd like to work on a project applying
the details you've learned.
-->

Le **chapitre 1** explique comment installer Rust, comment écrire un programme
"Hello, world!" et comment utiliser Cargo, le gestionnaire de paquets et outil
de compilation de Rust. Le **chapitre 2** est une introduction pratique à
l'écriture d'un programme en Rust, dans laquelle vous construirez un jeu de
devinette de nombre. Nous y abordons les concepts à un haut niveau, et les
chapitres ultérieurs fourniront des détails supplémentaires. Si vous voulez
mettre les mains dans le cambouis tout de suite, le chapitre 2 est l'endroit
idéal. Si vous êtes un apprenant particulièrement méticuleux qui préfère
apprendre chaque détail avant de passer au suivant, vous voudrez peut-être
sauter le chapitre 2 et aller directement au **chapitre 3**, qui couvre les
fonctionnalités de Rust similaires à celles d'autres langages de programmation ;
ensuite, vous pourrez revenir au chapitre 2 quand vous souhaiterez travailler
sur un projet en appliquant les détails que vous aurez appris.

<!--
In **Chapter 4**, you'll learn about Rust's ownership system. **Chapter 5**
discusses structs and methods. **Chapter 6** covers enums, `match` expressions,
and the `if let` and `let...else` control flow constructs. You'll use structs
and enums to make custom types.
-->

Au **chapitre 4**, vous découvrirez le système de possession (ownership) de
Rust. Le **chapitre 5** traite des structures (structs) et des méthodes. Le
**chapitre 6** couvre les énumérations (enums), les expressions `match` et les
constructions de flux de contrôle `if let` et `let...else`. Vous utiliserez les
structures et les énumérations pour créer des types personnalisés.

<!--
In **Chapter 7**, you'll learn about Rust's module system and about privacy
rules for organizing your code and its public application programming interface
(API). **Chapter 8** discusses some common collection data structures that the
standard library provides: vectors, strings, and hash maps. **Chapter 9**
explores Rust's error-handling philosophy and techniques.
-->

Au **chapitre 7**, vous apprendrez le système de modules de Rust et les règles
de visibilité pour organiser votre code et son interface de programmation
applicative (API) publique. Le **chapitre 8** présente certaines structures de
données de collection courantes fournies par la bibliothèque standard : les
vecteurs, les chaînes de caractères et les tables de hachage. Le **chapitre 9**
explore la philosophie et les techniques de gestion des erreurs en Rust.

<!--
**Chapter 10** digs into generics, traits, and lifetimes, which give you the
power to define code that applies to multiple types. **Chapter 11** is all
about testing, which even with Rust's safety guarantees is necessary to ensure
that your program's logic is correct. In **Chapter 12**, we'll build our own
implementation of a subset of functionality from the `grep` command line tool
that searches for text within files. For this, we'll use many of the concepts
we discussed in the previous chapters.
-->

Le **chapitre 10** approfondit les génériques, les traits et les durées de vie
(lifetimes), qui vous donnent le pouvoir de définir du code applicable à
plusieurs types. Le **chapitre 11** est entièrement consacré aux tests, qui même
avec les garanties de sûreté de Rust sont nécessaires pour s'assurer que la
logique de votre programme est correcte. Au **chapitre 12**, nous construirons
notre propre implémentation d'un sous-ensemble des fonctionnalités de l'outil en
ligne de commande `grep` qui recherche du texte dans des fichiers. Pour cela,
nous utiliserons de nombreux concepts abordés dans les chapitres précédents.

<!--
**Chapter 13** explores closures and iterators: features of Rust that come from
functional programming languages. In **Chapter 14**, we'll examine Cargo in
more depth and talk about best practices for sharing your libraries with
others. **Chapter 15** discusses smart pointers that the standard library
provides and the traits that enable their functionality.
-->

Le **chapitre 13** explore les fermetures (closures) et les itérateurs : des
fonctionnalités de Rust issues des langages de programmation fonctionnelle. Au
**chapitre 14**, nous examinerons Cargo plus en profondeur et parlerons des
bonnes pratiques pour partager vos bibliothèques avec d'autres. Le **chapitre
15** traite des pointeurs intelligents (smart pointers) fournis par la
bibliothèque standard et des traits qui permettent leur fonctionnement.

<!--
In **Chapter 16**, we'll walk through different models of concurrent
programming and talk about how Rust helps you program in multiple threads
fearlessly. In **Chapter 17**, we build on that by exploring Rust's async and
await syntax, along with tasks, futures, and streams, and the lightweight
concurrency model they enable.
-->

Au **chapitre 16**, nous parcourrons différents modèles de programmation
concurrente et verrons comment Rust vous aide à programmer avec plusieurs fils
d'exécution (threads) sans crainte. Au **chapitre 17**, nous poursuivrons en
explorant la syntaxe async et await de Rust, ainsi que les tâches (tasks), les
futures et les flux (streams), et le modèle de concurrence léger qu'ils
permettent.

<!--
**Chapter 18** looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with. **Chapter 19** is a reference on
patterns and pattern matching, which are powerful ways of expressing ideas
throughout Rust programs. **Chapter 20** contains a smorgasbord of advanced
topics of interest, including unsafe Rust, macros, and more about lifetimes,
traits, types, functions, and closures.
-->

Le **chapitre 18** examine comment les idiomes Rust se comparent aux principes
de programmation orientée objet que vous connaissez peut-être. Le **chapitre 19**
est une référence sur les motifs (patterns) et le filtrage par motif (pattern
matching), qui sont des moyens puissants d'exprimer des idées dans les programmes
Rust. Le **chapitre 20** contient un assortiment de sujets avancés intéressants,
incluant le Rust unsafe, les macros, et davantage sur les durées de vie, les
traits, les types, les fonctions et les fermetures.

<!--
In **Chapter 21**, we'll complete a project in which we'll implement a
low-level multithreaded web server!
-->

Au **chapitre 21**, nous terminerons un projet dans lequel nous implémenterons
un serveur web multithreadé de bas niveau !

<!--
Finally, some appendixes contain useful information about the language in a
more reference-like format. **Appendix A** covers Rust's keywords, **Appendix
B** covers Rust's operators and symbols, **Appendix C** covers derivable traits
provided by the standard library, **Appendix D** covers some useful development
tools, and **Appendix E** explains Rust editions. In **Appendix F**, you can
find translations of the book, and in **Appendix G** we'll cover how Rust is
made and what nightly Rust is.
-->

Enfin, certaines annexes contiennent des informations utiles sur le langage dans
un format plus proche d'une référence. L'**annexe A** couvre les mots-clés de
Rust, l'**annexe B** couvre les opérateurs et symboles de Rust, l'**annexe C**
couvre les traits dérivables fournis par la bibliothèque standard, l'**annexe D**
couvre certains outils de développement utiles, et l'**annexe E** explique les
éditions de Rust. Dans l'**annexe F**, vous trouverez des traductions du livre,
et dans l'**annexe G** nous verrons comment Rust est développé et ce qu'est
Rust nightly.

<!--
There is no wrong way to read this book: If you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.
-->

Il n'y a pas de mauvaise façon de lire ce livre : si vous voulez sauter des
chapitres, n'hésitez pas ! Vous devrez peut-être revenir aux chapitres
précédents si vous rencontrez des difficultés. Mais faites ce qui vous convient.

<span id="ferris"></span>

<!--
An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: These will guide you toward working code.
As such, we'll provide many examples that don't compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you're trying to run is meant to
error. In most situations, we'll lead you to the correct version of any code
that doesn't compile. Ferris will also help you distinguish code that isn't
meant to work:
-->

Une part importante du processus d'apprentissage de Rust est d'apprendre à lire
les messages d'erreur affichés par le compilateur : ils vous guideront vers un
code fonctionnel. Ainsi, nous fournirons de nombreux exemples qui ne compilent
pas, accompagnés du message d'erreur que le compilateur vous montrera dans chaque
situation. Sachez que si vous saisissez et exécutez un exemple au hasard, il
pourrait ne pas compiler ! Assurez-vous de lire le texte environnant pour voir
si l'exemple que vous essayez d'exécuter est censé produire une erreur. Dans la
plupart des situations, nous vous guiderons vers la version correcte de tout
code qui ne compile pas. Ferris vous aidera également à distinguer le code qui
n'est pas censé fonctionner :

<!--
| Ferris                                                                                                           | Meaning                                          |
| ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |
-->

| Ferris                                                                                                           | Signification                                              |
| ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris avec un point d'interrogation"/>   | Ce code ne compile pas !                                   |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris levant les mains en l'air"/>                 | Ce code panique !                                          |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris avec une pince levée, haussant les épaules"/> | Ce code ne produit pas le comportement souhaité. |

<!--
In most situations, we'll lead you to the correct version of any code that
doesn't compile.
-->

Dans la plupart des situations, nous vous guiderons vers la version correcte de
tout code qui ne compile pas.

<!--
## Source Code
-->

## Code source

<!--
The source files from which this book is generated can be found on
[GitHub][book].
-->

Les fichiers sources à partir desquels ce livre est généré se trouvent sur
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
