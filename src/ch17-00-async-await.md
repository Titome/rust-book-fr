<!--
# Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
-->

# Les fondamentaux de la programmation asynchrone : Async, Await, Futures et Streams

<!--
Many operations we ask the computer to do can take a while to finish. It would
be nice if we could do something else while we're waiting for those
long-running processes to complete. Modern computers offer two techniques for
working on more than one operation at a time: parallelism and concurrency. Our
programs' logic, however, is written in a mostly linear fashion. We'd like to
be able to specify the operations a program should perform and points at which
a function could pause and some other part of the program could run instead,
without needing to specify up front exactly the order and manner in which each
bit of code should run. _Asynchronous programming_ is an abstraction that lets
us express our code in terms of potential pausing points and eventual results
that takes care of the details of coordination for us.
-->

De nombreuses opérations que nous demandons à l'ordinateur d'effectuer peuvent prendre un certain temps. Ce serait bien de pouvoir faire autre chose en attendant que ces processus longs se terminent. Les ordinateurs modernes offrent deux techniques pour travailler sur plusieurs opérations à la fois : le parallélisme et la concurrence. Cependant, la logique de nos programmes est écrite de manière essentiellement linéaire. Nous aimerions pouvoir spécifier les opérations qu'un programme doit effectuer et les points auxquels une fonction pourrait se mettre en pause pour laisser une autre partie du programme s'exécuter à la place, sans avoir besoin de spécifier à l'avance l'ordre exact et la manière dont chaque morceau de code doit s'exécuter. La _programmation asynchrone_ est une abstraction qui nous permet d'exprimer notre code en termes de points de pause potentiels et de résultats finaux, tout en prenant en charge les détails de la coordination pour nous.

<!--
This chapter builds on Chapter 16's use of threads for parallelism and
concurrency by introducing an alternative approach to writing code: Rust's
futures, streams, and the `async` and `await` syntax that let us express how
operations could be asynchronous, and the third-party crates that implement
asynchronous runtimes: code that manages and coordinates the execution of
asynchronous operations.
-->

Ce chapitre s'appuie sur l'utilisation des threads du chapitre 16 pour le parallélisme et la concurrence, en présentant une approche alternative pour écrire du code : les futures, les streams, et la syntaxe `async` et `await` de Rust qui nous permettent d'exprimer comment les opérations pourraient être asynchrones, ainsi que les crates tierces qui implémentent des runtimes asynchrones : du code qui gère et coordonne l'exécution des opérations asynchrones.

<!--
Let's consider an example. Say you're exporting a video you've created of a
family celebration, an operation that could take anywhere from minutes to
hours. The video export will use as much CPU and GPU power as it can. If you
had only one CPU core and your operating system didn't pause that export until
it completed—that is, if it executed the export _synchronously_—you couldn't do
anything else on your computer while that task was running. That would be a
pretty frustrating experience. Fortunately, your computer's operating system
can, and does, invisibly interrupt the export often enough to let you get other
work done simultaneously.
-->

Prenons un exemple. Imaginons que vous exportez une vidéo d'une fête de famille, une opération qui peut prendre de quelques minutes à plusieurs heures. L'exportation vidéo utilisera autant de puissance CPU et GPU que possible. Si vous n'aviez qu'un seul cœur CPU et que votre système d'exploitation ne mettait pas en pause cette exportation avant qu'elle ne soit terminée — c'est-à-dire s'il exécutait l'exportation de manière _synchrone_ — vous ne pourriez rien faire d'autre sur votre ordinateur pendant l'exécution de cette tâche. Ce serait une expérience assez frustrante. Heureusement, le système d'exploitation de votre ordinateur peut, et le fait, interrompre invisiblement l'exportation assez souvent pour vous permettre de faire d'autres choses en même temps.

<!--
Now say you're downloading a video shared by someone else, which can also take
a while but does not take up as much CPU time. In this case, the CPU has to
wait for data to arrive from the network. While you can start reading the data
once it starts to arrive, it might take some time for all of it to show up.
Even once the data is all present, if the video is quite large, it could take
at least a second or two to load it all. That might not sound like much, but
it's a very long time for a modern processor, which can perform billions of
operations every second. Again, your operating system will invisibly interrupt
your program to allow the CPU to perform other work while waiting for the
network call to finish.
-->

Maintenant, imaginons que vous téléchargez une vidéo partagée par quelqu'un d'autre, ce qui peut aussi prendre du temps mais ne consomme pas autant de temps CPU. Dans ce cas, le CPU doit attendre que les données arrivent du réseau. Bien que vous puissiez commencer à lire les données dès qu'elles commencent à arriver, il peut falloir un certain temps pour que toutes les données soient disponibles. Même une fois que toutes les données sont présentes, si la vidéo est assez volumineuse, il pourrait falloir au moins une seconde ou deux pour tout charger. Cela peut sembler peu, mais c'est très long pour un processeur moderne qui peut effectuer des milliards d'opérations par seconde. Là encore, votre système d'exploitation interrompra invisiblement votre programme pour permettre au CPU d'effectuer d'autres tâches en attendant que l'appel réseau se termine.

<!--
The video export is an example of a _CPU-bound_ or _compute-bound_ operation.
It's limited by the computer's potential data processing speed within the CPU
or GPU, and how much of that speed it can dedicate to the operation. The video
download is an example of an _I/O-bound_ operation, because it's limited by the
speed of the computer's _input and output_; it can only go as fast as the data
can be sent across the network.
-->

L'exportation vidéo est un exemple d'opération _limitée par le CPU_ (_CPU-bound_) ou _limitée par le calcul_ (_compute-bound_). Elle est limitée par la vitesse potentielle de traitement des données du CPU ou du GPU, et par la part de cette vitesse qu'il peut consacrer à l'opération. Le téléchargement vidéo est un exemple d'opération _limitée par les E/S_ (_I/O-bound_), car elle est limitée par la vitesse des _entrées et sorties_ de l'ordinateur ; elle ne peut aller que aussi vite que les données peuvent être envoyées à travers le réseau.

<!--
In both of these examples, the operating system's invisible interrupts provide
a form of concurrency. That concurrency happens only at the level of the entire
program, though: the operating system interrupts one program to let other
programs get work done. In many cases, because we understand our programs at a
much more granular level than the operating system does, we can spot
opportunities for concurrency that the operating system can't see.
-->

Dans ces deux exemples, les interruptions invisibles du système d'exploitation fournissent une forme de concurrence. Cette concurrence se produit cependant uniquement au niveau du programme entier : le système d'exploitation interrompt un programme pour permettre à d'autres programmes de travailler. Dans de nombreux cas, comme nous comprenons nos programmes à un niveau bien plus granulaire que le système d'exploitation, nous pouvons repérer des opportunités de concurrence que le système d'exploitation ne peut pas voir.

<!--
For example, if we're building a tool to manage file downloads, we should be
able to write our program so that starting one download won't lock up the UI,
and users should be able to start multiple downloads at the same time. Many
operating system APIs for interacting with the network are _blocking_, though;
that is, they block the program's progress until the data they're processing is
completely ready.
-->

Par exemple, si nous construisons un outil pour gérer les téléchargements de fichiers, nous devrions pouvoir écrire notre programme de sorte que le démarrage d'un téléchargement ne bloque pas l'interface utilisateur, et les utilisateurs devraient pouvoir démarrer plusieurs téléchargements en même temps. Cependant, de nombreuses API de systèmes d'exploitation pour interagir avec le réseau sont _bloquantes_ ; c'est-à-dire qu'elles bloquent la progression du programme jusqu'à ce que les données qu'elles traitent soient complètement prêtes.

<!--
> Note: This is how _most_ function calls work, if you think about it. However,
> the term _blocking_ is usually reserved for function calls that interact with
> files, the network, or other resources on the computer, because those are the
> cases where an individual program would benefit from the operation being
> _non_-blocking.
-->

> Remarque : c'est ainsi que fonctionnent _la plupart_ des appels de fonctions, si vous y réfléchissez. Cependant, le terme _bloquant_ est généralement réservé aux appels de fonctions qui interagissent avec des fichiers, le réseau ou d'autres ressources de l'ordinateur, car ce sont les cas où un programme individuel bénéficierait d'une opération _non_ bloquante.

<!--
We could avoid blocking our main thread by spawning a dedicated thread to
download each file. However, the overhead of the system resources used by those
threads would eventually become a problem. It would be preferable if the call
didn't block in the first place, and instead we could define a number of tasks
that we'd like our program to complete and allow the runtime to choose the best
order and manner in which to run them.
-->

Nous pourrions éviter de bloquer notre thread principal en créant un thread dédié pour télécharger chaque fichier. Cependant, le surcoût des ressources système utilisées par ces threads finirait par devenir un problème. Il serait préférable que l'appel ne soit pas bloquant en premier lieu, et qu'à la place nous puissions définir un certain nombre de tâches que nous aimerions que notre programme accomplisse et laisser le runtime choisir le meilleur ordre et la meilleure manière de les exécuter.

<!--
That is exactly what Rust's _async_ (short for _asynchronous_) abstraction
gives us. In this chapter, you'll learn all about async as we cover the
following topics:
-->

C'est exactement ce que l'abstraction _async_ (abréviation d'_asynchrone_) de Rust nous offre. Dans ce chapitre, vous apprendrez tout sur l'async en couvrant les sujets suivants :

<!--
- How to use Rust's `async` and `await` syntax and execute asynchronous
  functions with a runtime
- How to use the async model to solve some of the same challenges we looked at
  in Chapter 16
- How multithreading and async provide complementary solutions that you can
  combine in many cases
-->

- Comment utiliser la syntaxe `async` et `await` de Rust et exécuter des fonctions asynchrones avec un runtime
- Comment utiliser le modèle async pour résoudre certains des mêmes défis que nous avons vus au chapitre 16
- Comment le multithreading et l'async fournissent des solutions complémentaires que vous pouvez combiner dans de nombreux cas

<!--
Before we see how async works in practice, though, we need to take a short
detour to discuss the differences between parallelism and concurrency.
-->

Avant de voir comment l'async fonctionne en pratique, nous devons cependant faire un petit détour pour discuter des différences entre le parallélisme et la concurrence.

<!--
## Parallelism and Concurrency
-->

## Parallélisme et concurrence

<!--
We've treated parallelism and concurrency as mostly interchangeable so far. Now
we need to distinguish between them more precisely, because the differences
will show up as we start working.
-->

Nous avons traité le parallélisme et la concurrence comme étant largement interchangeables jusqu'à présent. Maintenant, nous devons les distinguer plus précisément, car les différences vont apparaître lorsque nous commencerons à travailler.

<!--
Consider the different ways a team could split up work on a software project.
You could assign a single member multiple tasks, assign each member one task,
or use a mix of the two approaches.
-->

Considérez les différentes manières dont une équipe pourrait répartir le travail sur un projet logiciel. Vous pourriez assigner à un seul membre plusieurs tâches, assigner à chaque membre une seule tâche, ou utiliser un mélange des deux approches.

<!--
When an individual works on several different tasks before any of them is
complete, this is _concurrency_. One way to implement concurrency is similar to
having two different projects checked out on your computer, and when you get
bored or stuck on one project, you switch to the other. You're just one person,
so you can't make progress on both tasks at the exact same time, but you can
multitask, making progress on one at a time by switching between them (see
Figure 17-1).
-->

Quand un individu travaille sur plusieurs tâches différentes avant qu'aucune d'entre elles ne soit terminée, c'est de la _concurrence_. Une façon d'implémenter la concurrence est similaire au fait d'avoir deux projets différents ouverts sur votre ordinateur, et quand vous vous ennuyez ou êtes bloqué sur un projet, vous passez à l'autre. Vous n'êtes qu'une seule personne, donc vous ne pouvez pas progresser sur les deux tâches exactement en même temps, mais vous pouvez faire du multitâche, en progressant sur l'une à la fois en alternant entre elles (voir la figure 17-1).

<!--
<figure>

<img src="img/trpl17-01.svg" class="center" alt="A diagram with stacked boxes labeled Task A and Task B, with diamonds in them representing subtasks. Arrows point from A1 to B1, B1 to A2, A2 to B2, B2 to A3, A3 to A4, and A4 to B3. The arrows between the subtasks cross the boxes between Task A and Task B." />

<figcaption>Figure 17-1: A concurrent workflow, switching between Task A and Task B</figcaption>

</figure>
-->

<figure>

<img src="img/trpl17-01.svg" class="center" alt="Un diagramme avec des boîtes empilées intitulées Tâche A et Tâche B, avec des losanges représentant des sous-tâches. Les flèches pointent de A1 vers B1, B1 vers A2, A2 vers B2, B2 vers A3, A3 vers A4, et A4 vers B3. Les flèches entre les sous-tâches traversent les boîtes entre Tâche A et Tâche B." />

<figcaption>Figure 17-1 : Un flux de travail concurrent, alternant entre la Tâche A et la Tâche B</figcaption>

</figure>

<!--
When the team splits up a group of tasks by having each member take one task
and work on it alone, this is _parallelism_. Each person on the team can make
progress at the exact same time (see Figure 17-2).
-->

Quand l'équipe répartit un groupe de tâches en faisant prendre une tâche à chaque membre pour y travailler seul, c'est du _parallélisme_. Chaque personne de l'équipe peut progresser exactement en même temps (voir la figure 17-2).

<!--
<figure>

<img src="img/trpl17-02.svg" class="center" alt="A diagram with stacked boxes labeled Task A and Task B, with diamonds in them representing subtasks. Arrows point from A1 to A2, A2 to A3, A3 to A4, B1 to B2, and B2 to B3. No arrows cross between the boxes for Task A and Task B." />

<figcaption>Figure 17-2: A parallel workflow, where work happens on Task A and Task B independently</figcaption>

</figure>
-->

<figure>

<img src="img/trpl17-02.svg" class="center" alt="Un diagramme avec des boîtes empilées intitulées Tâche A et Tâche B, avec des losanges représentant des sous-tâches. Les flèches pointent de A1 vers A2, A2 vers A3, A3 vers A4, B1 vers B2, et B2 vers B3. Aucune flèche ne traverse entre les boîtes de Tâche A et Tâche B." />

<figcaption>Figure 17-2 : Un flux de travail parallèle, où le travail se fait sur la Tâche A et la Tâche B indépendamment</figcaption>

</figure>

<!--
In both of these workflows, you might have to coordinate between different
tasks. Maybe you thought the task assigned to one person was totally
independent from everyone else's work, but it actually requires another person
on the team to finish their task first. Some of the work could be done in
parallel, but some of it was actually _serial_: it could only happen in a
series, one task after the other, as in Figure 17-3.
-->

Dans ces deux flux de travail, vous pourriez devoir coordonner différentes tâches. Peut-être pensiez-vous que la tâche assignée à une personne était totalement indépendante du travail de tout le monde, mais elle nécessite en fait qu'une autre personne de l'équipe termine d'abord sa tâche. Une partie du travail pouvait être fait en parallèle, mais une autre partie était en réalité _sérielle_ : elle ne pouvait se faire qu'en série, une tâche après l'autre, comme dans la figure 17-3.

<!--
<figure>

<img src="img/trpl17-03.svg" class="center" alt="A diagram with stacked boxes labeled Task A and Task B, with diamonds in them representing subtasks. In Task A, arrows point from A1 to A2, from A2 to a pair of thick vertical lines like a "pause" symbol, and from that symbol to A3. In task B, arrows point from B1 to B2, from B2 to B3, from B3 to A3, and from B3 to B4." />

<figcaption>Figure 17-3: A partially parallel workflow, where work happens on Task A and Task B independently until Task A3 is blocked on the results of Task B3.</figcaption>

</figure>
-->

<figure>

<img src="img/trpl17-03.svg" class="center" alt="Un diagramme avec des boîtes empilées intitulées Tâche A et Tâche B, avec des losanges représentant des sous-tâches. Dans la Tâche A, les flèches pointent de A1 vers A2, de A2 vers une paire de lignes verticales épaisses comme un symbole « pause », et de ce symbole vers A3. Dans la Tâche B, les flèches pointent de B1 vers B2, de B2 vers B3, de B3 vers A3, et de B3 vers B4." />

<figcaption>Figure 17-3 : Un flux de travail partiellement parallèle, où le travail se fait sur la Tâche A et la Tâche B indépendamment jusqu'à ce que la Tâche A3 soit bloquée par les résultats de la Tâche B3.</figcaption>

</figure>

<!--
Likewise, you might realize that one of your own tasks depends on another of
your tasks. Now your concurrent work has also become serial.
-->

De même, vous pourriez réaliser qu'une de vos propres tâches dépend d'une autre de vos tâches. Votre travail concurrent est alors également devenu sériel.

<!--
Parallelism and concurrency can intersect with each other, too. If you learn
that a colleague is stuck until you finish one of your tasks, you'll probably
focus all your efforts on that task to "unblock" your colleague. You and your
coworker are no longer able to work in parallel, and you're also no longer able
to work concurrently on your own tasks.
-->

Le parallélisme et la concurrence peuvent également se croiser. Si vous apprenez qu'un collègue est bloqué tant que vous n'avez pas terminé une de vos tâches, vous concentrerez probablement tous vos efforts sur cette tâche pour « débloquer » votre collègue. Vous et votre collègue n'êtes plus en mesure de travailler en parallèle, et vous n'êtes plus non plus en mesure de travailler de manière concurrente sur vos propres tâches.

<!--
The same basic dynamics come into play with software and hardware. On a machine
with a single CPU core, the CPU can perform only one operation at a time, but
it can still work concurrently. Using tools such as threads, processes, and
async, the computer can pause one activity and switch to others before
eventually cycling back to that first activity again. On a machine with
multiple CPU cores, it can also do work in parallel. One core can be performing
one task while another core performs a completely unrelated one, and those
operations actually happen at the same time.
-->

Les mêmes dynamiques fondamentales entrent en jeu avec les logiciels et le matériel. Sur une machine avec un seul cœur CPU, le CPU ne peut effectuer qu'une seule opération à la fois, mais il peut quand même travailler de manière concurrente. En utilisant des outils comme les threads, les processus et l'async, l'ordinateur peut mettre en pause une activité et passer à d'autres avant de revenir éventuellement à la première activité. Sur une machine avec plusieurs cœurs CPU, il peut également travailler en parallèle. Un cœur peut exécuter une tâche pendant qu'un autre cœur en exécute une complètement différente, et ces opérations se produisent réellement en même temps.

<!--
Running async code in Rust usually happens concurrently. Depending on the
hardware, the operating system, and the async runtime we are using (more on
async runtimes shortly), that concurrency may also use parallelism under the
hood.
-->

L'exécution de code async en Rust se fait généralement de manière concurrente. En fonction du matériel, du système d'exploitation et du runtime async que nous utilisons (nous en parlerons bientôt), cette concurrence peut également utiliser le parallélisme en coulisses.

<!--
Now, let's dive into how async programming in Rust actually works.
-->

Maintenant, plongeons dans le fonctionnement réel de la programmation async en Rust.
