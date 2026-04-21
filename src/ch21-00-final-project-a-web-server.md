<!--
# Final Project: Building a Multithreaded Web Server
-->

# Projet final : construire un serveur web multithreadé

<!--
It's been a long journey, but we've reached the end of the book. In this
chapter, we'll build one more project together to demonstrate some of the
concepts we covered in the final chapters, as well as recap some earlier
lessons.
-->

Ce fut un long voyage, mais nous avons atteint la fin du livre. Dans ce chapitre, nous allons construire un dernier projet ensemble pour illustrer certains des concepts que nous avons abordés dans les derniers chapitres, ainsi que pour récapituler certaines leçons précédentes.

<!--
For our final project, we'll make a web server that says "Hello!" and looks like
Figure 21-1 in a web browser.
-->

Pour notre projet final, nous allons créer un serveur web qui affiche "Hello!" et ressemble à la Figure 21-1 dans un navigateur web.

<!--
Here is our plan for building the web server:

1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.
-->

Voici notre plan pour construire le serveur web :

1. Apprendre un peu sur TCP et HTTP.
2. Écouter les connexions TCP sur un socket.
3. Analyser un petit nombre de requêtes HTTP.
4. Créer une réponse HTTP correcte.
5. Améliorer le débit de notre serveur avec un groupe de threads (thread pool).

<!--
<img alt="Screenshot of a web browser visiting the address 127.0.0.1:8080 displaying a webpage with the text content "Hello! Hi from Rust"" src="img/trpl21-01.png" class="center" style="width: 50%;" />
-->

<img alt="Capture d'écran d'un navigateur web visitant l'adresse 127.0.0.1:8080 affichant une page web avec le contenu texte "Hello! Hi from Rust"" src="img/trpl21-01.png" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 21-1: Our final shared project</span>
-->

<span class="caption">Figure 21-1 : Notre dernier projet partagé</span>

<!--
Before we get started, we should mention two details. First, the method we'll
use won't be the best way to build a web server with Rust. Community members
have published a number of production-ready crates available at
[crates.io](https://crates.io/) that provide more complete web server and
thread pool implementations than we'll build. However, our intention in this
chapter is to help you learn, not to take the easy route. Because Rust is a
systems programming language, we can choose the level of abstraction we want to
work with and can go to a lower level than is possible or practical in other
languages.
-->

Avant de commencer, nous devons mentionner deux détails. Premièrement, la méthode que nous utiliserons ne sera pas la meilleure façon de construire un serveur web avec Rust. Des membres de la communauté ont publié un certain nombre de crates prêtes pour la production, disponibles sur [crates.io](https://crates.io/), qui fournissent des implémentations de serveur web et de groupe de threads plus complètes que ce que nous allons construire. Cependant, notre intention dans ce chapitre est de vous aider à apprendre, pas de prendre le chemin le plus facile. Comme Rust est un langage de programmation système, nous pouvons choisir le niveau d'abstraction avec lequel nous voulons travailler et descendre à un niveau plus bas que ce qui est possible ou pratique dans d'autres langages.

<!--
Second, we will not be using async and await here. Building a thread pool is a
big enough challenge on its own, without adding in building an async runtime!
However, we will note how async and await might be applicable to some of the
same problems we will see in this chapter. Ultimately, as we noted back in
Chapter 17, many async runtimes use thread pools for managing their work.
-->

Deuxièmement, nous n'utiliserons pas async et await ici. Construire un groupe de threads est déjà un défi suffisamment important en soi, sans y ajouter la construction d'un runtime async ! Cependant, nous noterons comment async et await pourraient s'appliquer à certains des mêmes problèmes que nous verrons dans ce chapitre. En fin de compte, comme nous l'avons noté au chapitre 17, de nombreux runtimes async utilisent des groupes de threads pour gérer leur travail.

<!--
We'll therefore write the basic HTTP server and thread pool manually so that
you can learn the general ideas and techniques behind the crates you might use
in the future.
-->

Nous allons donc écrire le serveur HTTP basique et le groupe de threads manuellement afin que vous puissiez apprendre les idées générales et les techniques derrière les crates que vous pourriez utiliser à l'avenir.
