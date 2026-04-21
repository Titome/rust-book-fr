<!--
# An I/O Project: Building a Command Line Program
-->

# Un projet d'E/S : construire un programme en ligne de commande

<!--
This chapter is a recap of the many skills you've learned so far and an
exploration of a few more standard library features. We'll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.
-->

Ce chapitre est un récapitulatif des nombreuses compétences que vous avez acquises jusqu'ici, ainsi qu'une exploration de quelques fonctionnalités supplémentaires de la bibliothèque standard. Nous allons construire un outil en ligne de commande qui interagit avec les entrées/sorties de fichiers et de la ligne de commande pour mettre en pratique certains des concepts de Rust que vous maîtrisez désormais.

<!--
Rust's speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we'll
make our own version of the classic command line search tool `grep`
(**g**lobally search a **r**egular **e**xpression and **p**rint). In the
simplest use case, `grep` searches a specified file for a specified string. To
do so, `grep` takes as its arguments a file path and a string. Then, it reads
the file, finds lines in that file that contain the string argument, and prints
those lines.
-->

La rapidité, la sécurité, la production d'un binaire unique et la prise en charge multiplateforme de Rust en font un langage idéal pour créer des outils en ligne de commande. Pour notre projet, nous allons donc créer notre propre version du classique outil de recherche en ligne de commande `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint). Dans le cas d'utilisation le plus simple, `grep` recherche une chaîne de caractères spécifiée dans un fichier donné. Pour ce faire, `grep` prend en arguments un chemin de fichier et une chaîne de caractères. Ensuite, il lit le fichier, trouve les lignes qui contiennent la chaîne en argument, et affiche ces lignes.

<!--
Along the way, we'll show how to make our command line tool use the terminal
features that many other command line tools use. We'll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We'll also print error messages to the standard error console stream (`stderr`)
instead of standard output (`stdout`) so that, for example, the user can
redirect successful output to a file while still seeing error messages onscreen.
-->

En cours de route, nous montrerons comment faire en sorte que notre outil en ligne de commande utilise les fonctionnalités du terminal que de nombreux autres outils en ligne de commande utilisent. Nous lirons la valeur d'une variable d'environnement pour permettre à l'utilisateur de configurer le comportement de notre outil. Nous afficherons également les messages d'erreur sur le flux d'erreur standard (`stderr`) plutôt que sur la sortie standard (`stdout`), de sorte que, par exemple, l'utilisateur puisse rediriger la sortie réussie vers un fichier tout en continuant à voir les messages d'erreur à l'écran.

<!--
One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version will be fairly simple, but this chapter will give you some of the
background knowledge you need to understand a real-world project such as
`ripgrep`.
-->

Un membre de la communauté Rust, Andrew Gallant, a déjà créé une version complète et très rapide de `grep`, appelée `ripgrep`. En comparaison, notre version sera assez simple, mais ce chapitre vous donnera les connaissances de base nécessaires pour comprendre un projet concret tel que `ripgrep`.

<!--
Our `grep` project will combine a number of concepts you've learned so far:

- Organizing code ([Chapter 7][ch7] ignore
-->)
- Using vectors and strings ([Chapter 8][ch8]<!--
ignore
-->)
- Handling errors ([Chapter 9][ch9]<!--
ignore
-->)
- Using traits and lifetimes where appropriate ([Chapter 10][ch10]<!--
ignore
-->)
- Writing tests ([Chapter 11][ch11]<!--
ignore
-->)
-->

Notre projet `grep` combinera un certain nombre de concepts que vous avez appris jusqu'ici :

- Organiser le code ([Chapitre 7][ch7]<!--
ignore
-->)
- Utiliser les vecteurs et les chaînes de caractères ([Chapitre 8][ch8]<!--
ignore
-->)
- Gérer les erreurs ([Chapitre 9][ch9]<!--
ignore
-->)
- Utiliser les traits et les durées de vie là où c'est approprié ([Chapitre 10][ch10]<!--
ignore
-->)
- Écrire des tests ([Chapitre 11][ch11]<!--
ignore
-->)

<!--
We'll also briefly introduce closures, iterators, and trait objects, which
[Chapter 13][ch13] ignore
--> and [Chapter 18][ch18]<!--
ignore
--> will
cover in detail.
-->

Nous présenterons également brièvement les fermetures (closures), les itérateurs et les objets trait, qui seront couverts en détail dans le [Chapitre 13][ch13]<!--
ignore
--> et le [Chapitre 18][ch18]<!--
ignore
-->.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
