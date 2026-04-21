<!--
## What Is Ownership?
-->

## Qu'est-ce que la possession ?

<!--
_Ownership_ is a set of rules that govern how a Rust program manages memory.
All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection that regularly looks for no-longer-used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free the memory. Rust uses a third approach: Memory is managed
through a system of ownership with a set of rules that the compiler checks. If
any of the rules are violated, the program won't compile. None of the features
of ownership will slow down your program while it's running.
-->

La _possession_ (ownership) est un ensemble de regles qui regissent la facon
dont un programme Rust gere la memoire. Tous les programmes doivent gerer la
facon dont ils utilisent la memoire d'un ordinateur pendant leur execution.
Certains langages disposent d'un ramasse-miettes (garbage collector) qui
recherche regulierement la memoire inutilisee pendant l'execution du programme ;
dans d'autres langages, le programmeur doit explicitement allouer et liberer la
memoire. Rust utilise une troisieme approche : la memoire est geree par un
systeme de possession avec un ensemble de regles que le compilateur verifie. Si
l'une des regles est violee, le programme ne compilera pas. Aucune des
fonctionnalites de la possession ne ralentira votre programme pendant son
execution.

<!--
Because ownership is a new concept for many programmers, it does take some time
to get used to. The good news is that the more experienced you become with Rust
and the rules of the ownership system, the easier you'll find it to naturally
develop code that is safe and efficient. Keep at it!
-->

Comme la possession est un concept nouveau pour de nombreux programmeurs, il
faut un certain temps pour s'y habituer. La bonne nouvelle, c'est que plus vous
acquerrez de l'experience avec Rust et les regles du systeme de possession, plus
vous trouverez facile de developper naturellement du code sur et efficace.
Perseverez !

<!--
When you understand ownership, you'll have a solid foundation for understanding
the features that make Rust unique. In this chapter, you'll learn ownership by
working through some examples that focus on a very common data structure:
strings.
-->

Lorsque vous comprendrez la possession, vous disposerez d'une base solide pour
comprendre les fonctionnalites qui rendent Rust unique. Dans ce chapitre, vous
apprendrez la possession en travaillant sur des exemples qui se concentrent sur
une structure de donnees tres courante : les chaines de caracteres (strings).

<!--
> ### The Stack and the Heap
>
> Many programming languages don't require you to think about the stack and the
> heap very often. But in a systems programming language like Rust, whether a
> value is on the stack or the heap affects how the language behaves and why
> you have to make certain decisions. Parts of ownership will be described in
> relation to the stack and the heap later in this chapter, so here is a brief
> explanation in preparation.
>
> Both the stack and the heap are parts of memory available to your code to use
> at runtime, but they are structured in different ways. The stack stores
> values in the order it gets them and removes the values in the opposite
> order. This is referred to as _last in, first out (LIFO)_. Think of a stack of
> plates: When you add more plates, you put them on top of the pile, and when
> you need a plate, you take one off the top. Adding or removing plates from
> the middle or bottom wouldn't work as well! Adding data is called _pushing
> onto the stack_, and removing data is called _popping off the stack_. All
> data stored on the stack must have a known, fixed size. Data with an unknown
> size at compile time or a size that might change must be stored on the heap
> instead.
>
> The heap is less organized: When you put data on the heap, you request a
> certain amount of space. The memory allocator finds an empty spot in the heap
> that is big enough, marks it as being in use, and returns a _pointer_, which
> is the address of that location. This process is called _allocating on the
> heap_ and is sometimes abbreviated as just _allocating_ (pushing values onto
> the stack is not considered allocating). Because the pointer to the heap is a
> known, fixed size, you can store the pointer on the stack, but when you want
> the actual data, you must follow the pointer. Think of being seated at a
> restaurant. When you enter, you state the number of people in your group, and
> the host finds an empty table that fits everyone and leads you there. If
> someone in your group comes late, they can ask where you've been seated to
> find you.
>
> Pushing to the stack is faster than allocating on the heap because the
> allocator never has to search for a place to store new data; that location is
> always at the top of the stack. Comparatively, allocating space on the heap
> requires more work because the allocator must first find a big enough space
> to hold the data and then perform bookkeeping to prepare for the next
> allocation.
>
> Accessing data in the heap is generally slower than accessing data on the
> stack because you have to follow a pointer to get there. Contemporary
> processors are faster if they jump around less in memory. Continuing the
> analogy, consider a server at a restaurant taking orders from many tables.
> It's most efficient to get all the orders at one table before moving on to
> the next table. Taking an order from table A, then an order from table B,
> then one from A again, and then one from B again would be a much slower
> process. By the same token, a processor can usually do its job better if it
> works on data that's close to other data (as it is on the stack) rather than
> farther away (as it can be on the heap).
>
> When your code calls a function, the values passed into the function
> (including, potentially, pointers to data on the heap) and the function's
> local variables get pushed onto the stack. When the function is over, those
> values get popped off the stack.
>
> Keeping track of what parts of code are using what data on the heap,
> minimizing the amount of duplicate data on the heap, and cleaning up unused
> data on the heap so that you don't run out of space are all problems that
> ownership addresses. Once you understand ownership, you won't need to think
> about the stack and the heap very often. But knowing that the main purpose of
> ownership is to manage heap data can help explain why it works the way it
> does.
-->

> ### La pile et le tas
>
> De nombreux langages de programmation ne vous demandent pas de penser souvent
> a la pile (stack) et au tas (heap). Mais dans un langage de programmation
> systeme comme Rust, le fait qu'une valeur soit sur la pile ou sur le tas
> affecte le comportement du langage et explique pourquoi vous devez prendre
> certaines decisions. Des aspects de la possession seront decrits en relation
> avec la pile et le tas plus tard dans ce chapitre, voici donc une breve
> explication en guise de preparation.
>
> La pile et le tas sont tous deux des parties de la memoire disponibles pour
> votre code a l'execution, mais ils sont structures de manieres differentes.
> La pile stocke les valeurs dans l'ordre ou elle les recoit et les retire dans
> l'ordre inverse. C'est ce qu'on appelle _dernier entre, premier sorti
> (LIFO, last in, first out)_. Pensez a une pile d'assiettes : quand vous en
> ajoutez, vous les posez sur le dessus, et quand vous avez besoin d'une
> assiette, vous en prenez une sur le dessus. Ajouter ou retirer des assiettes
> au milieu ou en bas ne fonctionnerait pas aussi bien ! Ajouter des donnees
> s'appelle _empiler_ (pushing onto the stack), et retirer des donnees
> s'appelle _depiler_ (popping off the stack). Toutes les donnees stockees sur
> la pile doivent avoir une taille connue et fixe. Les donnees dont la taille
> est inconnue a la compilation ou susceptible de changer doivent etre stockees
> sur le tas.
>
> Le tas est moins organise : quand vous placez des donnees sur le tas, vous
> demandez une certaine quantite d'espace. L'allocateur de memoire trouve un
> emplacement vide dans le tas qui est assez grand, le marque comme etant
> utilise, et renvoie un _pointeur_, qui est l'adresse de cet emplacement. Ce
> processus s'appelle _allouer sur le tas_ et est parfois abrege en simplement
> _allouer_ (empiler des valeurs sur la pile n'est pas considere comme une
> allocation). Comme le pointeur vers le tas a une taille connue et fixe, vous
> pouvez stocker le pointeur sur la pile, mais quand vous voulez les donnees
> reelles, vous devez suivre le pointeur. Imaginez que vous arriviez dans un
> restaurant. A l'entree, vous indiquez le nombre de personnes dans votre
> groupe, et l'hotesse trouve une table vide qui convient a tout le monde et
> vous y conduit. Si quelqu'un de votre groupe arrive en retard, il peut
> demander ou vous etes installes pour vous retrouver.
>
> Empiler sur la pile est plus rapide qu'allouer sur le tas car l'allocateur n'a
> jamais besoin de chercher un endroit pour stocker de nouvelles donnees ; cet
> endroit est toujours au sommet de la pile. En comparaison, allouer de l'espace
> sur le tas demande plus de travail car l'allocateur doit d'abord trouver un
> espace assez grand pour contenir les donnees, puis effectuer de la
> comptabilite pour preparer la prochaine allocation.
>
> Acceder aux donnees sur le tas est generalement plus lent qu'acceder aux
> donnees sur la pile car il faut suivre un pointeur pour y arriver. Les
> processeurs contemporains sont plus rapides s'ils effectuent moins de sauts
> en memoire. Pour continuer l'analogie, imaginez un serveur dans un restaurant
> qui prend les commandes de plusieurs tables. Il est plus efficace de prendre
> toutes les commandes a une table avant de passer a la suivante. Prendre une
> commande a la table A, puis une a la table B, puis revenir a la table A, puis
> a la table B serait un processus bien plus lent. De la meme facon, un
> processeur peut generalement mieux faire son travail s'il travaille sur des
> donnees proches les unes des autres (comme sur la pile) plutot qu'eloignees
> (comme cela peut etre le cas sur le tas).
>
> Quand votre code appelle une fonction, les valeurs passees a la fonction (y
> compris, potentiellement, des pointeurs vers des donnees sur le tas) et les
> variables locales de la fonction sont empilees sur la pile. Quand la fonction
> se termine, ces valeurs sont depilees.
>
> Garder la trace de quelles parties du code utilisent quelles donnees sur le
> tas, minimiser la quantite de donnees dupliquees sur le tas, et nettoyer les
> donnees inutilisees sur le tas pour ne pas manquer d'espace sont tous des
> problemes que la possession resout. Une fois que vous comprendrez la
> possession, vous n'aurez plus besoin de penser souvent a la pile et au tas.
> Mais savoir que l'objectif principal de la possession est de gerer les donnees
> du tas peut aider a expliquer pourquoi elle fonctionne ainsi.

<!--
### Ownership Rules
-->

### Les regles de la possession

<!--
First, let's take a look at the ownership rules. Keep these rules in mind as we
work through the examples that illustrate them:
-->

Tout d'abord, jetons un coup d'oeil aux regles de la possession. Gardez ces
regles a l'esprit pendant que nous travaillons sur les exemples qui les
illustrent :

<!--
- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
-->

- Chaque valeur en Rust a un _proprietaire_ (owner).
- Il ne peut y avoir qu'un seul proprietaire a la fois.
- Quand le proprietaire sort de la portee, la valeur est supprimee (dropped).

<!--
### Variable Scope
-->

### La portee des variables

<!--
Now that we're past basic Rust syntax, we won't include all the `fn main() {`
code in the examples, so if you're following along, make sure to put the
following examples inside a `main` function manually. As a result, our examples
will be a bit more concise, letting us focus on the actual details rather than
boilerplate code.
-->

Maintenant que nous avons depasse la syntaxe de base de Rust, nous n'inclurons
pas tout le code `fn main() {` dans les exemples, donc si vous suivez, assurez-
vous de placer les exemples suivants a l'interieur d'une fonction `main`
manuellement. En consequence, nos exemples seront un peu plus concis, ce qui
nous permettra de nous concentrer sur les details importants plutot que sur le
code repetitif.

<!--
As a first example of ownership, we'll look at the scope of some variables. A
_scope_ is the range within a program for which an item is valid. Take the
following variable:
-->

Comme premier exemple de possession, nous allons examiner la portee de certaines
variables. Une _portee_ (scope) est la zone au sein d'un programme dans laquelle
un element est valide. Prenons la variable suivante :

<!--
```rust
let s = "hello";
```
-->

```rust
let s = "hello";
```

<!--
The variable `s` refers to a string literal, where the value of the string is
hardcoded into the text of our program. The variable is valid from the point at
which it's declared until the end of the current scope. Listing 4-1 shows a
program with comments annotating where the variable `s` would be valid.
-->

La variable `s` fait reference a un litteral de chaine de caracteres, dont la
valeur est codee en dur dans le texte de notre programme. La variable est valide
a partir du moment ou elle est declaree jusqu'a la fin de la portee courante. Le
listing 4-1 montre un programme avec des commentaires indiquant ou la variable
`s` serait valide.

<Listing number="4-1" caption="Une variable et la portee dans laquelle elle est valide">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

</Listing>

<!--
In other words, there are two important points in time here:
-->

En d'autres termes, il y a ici deux moments importants :

<!--
- When `s` comes _into_ scope, it is valid.
- It remains valid until it goes _out of_ scope.
-->

- Quand `s` _entre_ dans la portee, elle est valide.
- Elle reste valide jusqu'a ce qu'elle _sorte_ de la portee.

<!--
At this point, the relationship between scopes and when variables are valid is
similar to that in other programming languages. Now we'll build on top of this
understanding by introducing the `String` type.
-->

A ce stade, la relation entre les portees et la validite des variables est
similaire a celle des autres langages de programmation. Nous allons maintenant
approfondir cette comprehension en introduisant le type `String`.

<!--
### The `String` Type
-->

### Le type `String`

<!--
To illustrate the rules of ownership, we need a data type that is more complex
than those we covered in the ["Data Types"][data-types] ignore
--> section
of Chapter 3. The types covered previously are of a known size, can be stored
on the stack and popped off the stack when their scope is over, and can be
quickly and trivially copied to make a new, independent instance if another
part of code needs to use the same value in a different scope. But we want to
look at data that is stored on the heap and explore how Rust knows when to
clean up that data, and the `String` type is a great example.
-->

Pour illustrer les regles de la possession, nous avons besoin d'un type de
donnees plus complexe que ceux que nous avons couverts dans la section ["Les
types de donnees"][data-types]<!--
ignore
--> du chapitre 3. Les types couverts
precedemment ont une taille connue, peuvent etre stockes sur la pile et depiles
quand leur portee est terminee, et peuvent etre copies rapidement et
trivialement pour creer une nouvelle instance independante si une autre partie
du code a besoin d'utiliser la meme valeur dans une portee differente. Mais nous
voulons examiner des donnees stockees sur le tas et explorer comment Rust sait
quand nettoyer ces donnees, et le type `String` en est un excellent exemple.

<!--
We'll concentrate on the parts of `String` that relate to ownership. These
aspects also apply to other complex data types, whether they are provided by
the standard library or created by you. We'll discuss non-ownership aspects of
`String` in [Chapter 8][ch8] ignore
-->.
-->

Nous nous concentrerons sur les aspects de `String` qui sont lies a la
possession. Ces aspects s'appliquent egalement a d'autres types de donnees
complexes, qu'ils soient fournis par la bibliotheque standard ou crees par vous.
Nous aborderons les aspects de `String` non lies a la possession au
[chapitre 8][ch8]<!--
ignore
-->.

<!--
We've already seen string literals, where a string value is hardcoded into our
program. String literals are convenient, but they aren't suitable for every
situation in which we may want to use text. One reason is that they're
immutable. Another is that not every string value can be known when we write
our code: For example, what if we want to take user input and store it? It is
for these situations that Rust has the `String` type. This type manages
data allocated on the heap and as such is able to store an amount of text that
is unknown to us at compile time. You can create a `String` from a string
literal using the `from` function, like so:
-->

Nous avons deja vu les litteraux de chaines de caracteres, ou une valeur de
chaine est codee en dur dans notre programme. Les litteraux de chaines sont
pratiques, mais ils ne conviennent pas a toutes les situations ou nous pourrions
vouloir utiliser du texte. L'une des raisons est qu'ils sont immuables. Une
autre est que toutes les valeurs de chaines ne peuvent pas etre connues au
moment ou nous ecrivons notre code : par exemple, que faire si nous voulons
prendre une saisie utilisateur et la stocker ? C'est pour ces situations que
Rust dispose du type `String`. Ce type gere des donnees allouees sur le tas et
est donc capable de stocker une quantite de texte qui nous est inconnue a la
compilation. Vous pouvez creer une `String` a partir d'un litteral de chaine en
utilisant la fonction `from`, comme ceci :

<!--
```rust
let s = String::from("hello");
```
-->

```rust
let s = String::from("hello");
```

<!--
The double colon `::` operator allows us to namespace this particular `from`
function under the `String` type rather than using some sort of name like
`string_from`. We'll discuss this syntax more in the ["Methods"][methods]
ignore
--> section of Chapter 5, and when we talk about namespacing with
modules in ["Paths for Referring to an Item in the Module
Tree"][paths-module-tree]<!--
ignore
--> in Chapter 7.
-->

L'operateur double deux-points `::` nous permet d'espacer cette fonction `from`
particuliere sous le type `String` plutot que d'utiliser un nom comme
`string_from`. Nous discuterons de cette syntaxe plus en detail dans la section
["Les methodes"][methods]<!--
ignore
--> du chapitre 5, et quand nous parlerons
de l'espacement de noms avec les modules dans ["Les chemins pour faire reference
a un element dans l'arborescence des modules"][paths-module-tree]<!--
ignore
-->
au chapitre 7.

<!--
This kind of string _can_ be mutated:
-->

Ce type de chaine _peut_ etre modifie :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

<!--
So, what's the difference here? Why can `String` be mutated but literals
cannot? The difference is in how these two types deal with memory.
-->

Alors, quelle est la difference ici ? Pourquoi `String` peut-elle etre modifiee
mais pas les litteraux ? La difference reside dans la facon dont ces deux types
gerent la memoire.

<!--
### Memory and Allocation
-->

### Memoire et allocation

<!--
In the case of a string literal, we know the contents at compile time, so the
text is hardcoded directly into the final executable. This is why string
literals are fast and efficient. But these properties only come from the string
literal's immutability. Unfortunately, we can't put a blob of memory into the
binary for each piece of text whose size is unknown at compile time and whose
size might change while running the program.
-->

Dans le cas d'un litteral de chaine, nous connaissons le contenu a la
compilation, donc le texte est code en dur directement dans l'executable final.
C'est pourquoi les litteraux de chaines sont rapides et efficaces. Mais ces
proprietes ne viennent que de l'immutabilite du litteral de chaine.
Malheureusement, nous ne pouvons pas placer un bloc de memoire dans le binaire
pour chaque morceau de texte dont la taille est inconnue a la compilation et
dont la taille pourrait changer pendant l'execution du programme.

<!--
With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:
-->

Avec le type `String`, pour prendre en charge un morceau de texte modifiable et
extensible, nous devons allouer une quantite de memoire sur le tas, inconnue a
la compilation, pour contenir le contenu. Cela signifie :

<!--
- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we're done with
  our `String`.
-->

- La memoire doit etre demandee a l'allocateur de memoire a l'execution.
- Nous avons besoin d'un moyen de rendre cette memoire a l'allocateur quand nous
  avons termine avec notre `String`.

<!--
That first part is done by us: When we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.
-->

La premiere partie est faite par nous : quand nous appelons `String::from`, son
implementation demande la memoire dont elle a besoin. C'est a peu pres universel
dans les langages de programmation.

<!--
However, the second part is different. In languages with a _garbage collector
(GC)_, the GC keeps track of and cleans up memory that isn't being used
anymore, and we don't need to think about it. In most languages without a GC,
it's our responsibility to identify when memory is no longer being used and to
call code to explicitly free it, just as we did to request it. Doing this
correctly has historically been a difficult programming problem. If we forget,
we'll waste memory. If we do it too early, we'll have an invalid variable. If
we do it twice, that's a bug too. We need to pair exactly one `allocate` with
exactly one `free`.
-->

Cependant, la seconde partie est differente. Dans les langages avec un
_ramasse-miettes (GC)_, le GC garde la trace de la memoire qui n'est plus
utilisee et la nettoie, et nous n'avons pas besoin d'y penser. Dans la plupart
des langages sans GC, c'est notre responsabilite d'identifier quand la memoire
n'est plus utilisee et d'appeler du code pour la liberer explicitement, tout
comme nous l'avons fait pour la demander. Faire cela correctement a
historiquement ete un probleme de programmation difficile. Si nous oublions, nous
gaspillons de la memoire. Si nous le faisons trop tot, nous aurons une variable
invalide. Si nous le faisons deux fois, c'est aussi un bug. Nous devons associer
exactement un `allocate` avec exactement un `free`.

<!--
Rust takes a different path: The memory is automatically returned once the
variable that owns it goes out of scope. Here's a version of our scope example
from Listing 4-1 using a `String` instead of a string literal:
-->

Rust prend un chemin different : la memoire est automatiquement rendue une fois
que la variable qui la possede sort de la portee. Voici une version de notre
exemple de portee du listing 4-1 utilisant une `String` au lieu d'un litteral de
chaine :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

<!--
There is a natural point at which we can return the memory our `String` needs
to the allocator: when `s` goes out of scope. When a variable goes out of
scope, Rust calls a special function for us. This function is called
`drop`, and it's where the author of `String` can put
the code to return the memory. Rust calls `drop` automatically at the closing
curly bracket.
-->

Il y a un moment naturel auquel nous pouvons rendre la memoire dont notre
`String` a besoin a l'allocateur : quand `s` sort de la portee. Quand une
variable sort de la portee, Rust appelle une fonction speciale pour nous. Cette
fonction s'appelle `drop`, et c'est la que l'auteur de `String` peut placer le
code pour rendre la memoire. Rust appelle `drop` automatiquement a l'accolade
fermante.

<!--
> Note: In C++, this pattern of deallocating resources at the end of an item's
> lifetime is sometimes called _Resource Acquisition Is Initialization (RAII)_.
> The `drop` function in Rust will be familiar to you if you've used RAII
> patterns.
-->

> Note : En C++, ce patron de desallocation des ressources a la fin de la duree
> de vie d'un element est parfois appele _Resource Acquisition Is Initialization
> (RAII)_. La fonction `drop` en Rust vous sera familiere si vous avez utilise
> des patrons RAII.

<!--
This pattern has a profound impact on the way Rust code is written. It may seem
simple right now, but the behavior of code can be unexpected in more
complicated situations when we want to have multiple variables use the data
we've allocated on the heap. Let's explore some of those situations now.
-->

Ce patron a un impact profond sur la facon dont le code Rust est ecrit. Cela
peut sembler simple pour l'instant, mais le comportement du code peut etre
inattendu dans des situations plus complexes quand nous voulons que plusieurs
variables utilisent les donnees que nous avons allouees sur le tas. Explorons
certaines de ces situations maintenant.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ways-variables-and-data-interact-move"></a>

<!--
#### Variables and Data Interacting with Move
-->

#### Interaction entre les variables et les donnees avec le deplacement (Move)

<!--
Multiple variables can interact with the same data in different ways in Rust.
Listing 4-2 shows an example using an integer.
-->

Plusieurs variables peuvent interagir avec les memes donnees de differentes
facons en Rust. Le listing 4-2 montre un exemple utilisant un entier.

<Listing number="4-2" caption="Assignation de la valeur entiere de la variable `x` a `y`">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

</Listing>

<!--
We can probably guess what this is doing: "Bind the value `5` to `x`; then, make
a copy of the value in `x` and bind it to `y`." We now have two variables, `x`
and `y`, and both equal `5`. This is indeed what is happening, because integers
are simple values with a known, fixed size, and these two `5` values are pushed
onto the stack.
-->

Nous pouvons probablement deviner ce que cela fait : "Lier la valeur `5` a `x` ;
puis, faire une copie de la valeur dans `x` et la lier a `y`." Nous avons
maintenant deux variables, `x` et `y`, et les deux valent `5`. C'est en effet ce
qui se passe, car les entiers sont des valeurs simples avec une taille connue et
fixe, et ces deux valeurs `5` sont empilees sur la pile.

<!--
Now let's look at the `String` version:
-->

Maintenant, regardons la version avec `String` :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

<!--
This looks very similar, so we might assume that the way it works would be the
same: That is, the second line would make a copy of the value in `s1` and bind
it to `s2`. But this isn't quite what happens.
-->

Cela semble tres similaire, donc nous pourrions supposer que le fonctionnement
serait le meme : c'est-a-dire que la deuxieme ligne ferait une copie de la
valeur dans `s1` et la lierait a `s2`. Mais ce n'est pas tout a fait ce qui se
passe.

<!--
Take a look at Figure 4-1 to see what is happening to `String` under the
covers. A `String` is made up of three parts, shown on the left: a pointer to
the memory that holds the contents of the string, a length, and a capacity.
This group of data is stored on the stack. On the right is the memory on the
heap that holds the contents.
-->

Jetez un oeil a la figure 4-1 pour voir ce qui se passe sous le capot avec
`String`. Une `String` est composee de trois parties, montrees a gauche : un
pointeur vers la memoire qui contient le contenu de la chaine, une longueur, et
une capacite. Ce groupe de donnees est stocke sur la pile. A droite se trouve la
memoire sur le tas qui contient le contenu.

<!--
<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />
-->

<img alt="Deux tableaux : le premier tableau contient la representation de s1 sur la
pile, composee de sa longueur (5), sa capacite (5), et un pointeur vers la
premiere valeur du second tableau. Le second tableau contient la representation
des donnees de la chaine sur le tas, octet par octet." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<!--
<span class="caption">Figure 4-1: The representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>
-->

<span class="caption">Figure 4-1 : La representation en memoire d'une `String`
contenant la valeur `"hello"` liee a `s1`</span>

<!--
The length is how much memory, in bytes, the contents of the `String` are
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the allocator. The difference between length and
capacity matters, but not in this context, so for now, it's fine to ignore the
capacity.
-->

La longueur est la quantite de memoire, en octets, que le contenu de la `String`
utilise actuellement. La capacite est la quantite totale de memoire, en octets,
que la `String` a recue de l'allocateur. La difference entre la longueur et la
capacite est importante, mais pas dans ce contexte, donc pour l'instant, il est
acceptable d'ignorer la capacite.

<!--
When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-2.
-->

Quand nous assignons `s1` a `s2`, les donnees de la `String` sont copiees, ce
qui signifie que nous copions le pointeur, la longueur, et la capacite qui sont
sur la pile. Nous ne copions pas les donnees sur le tas auxquelles le pointeur
fait reference. En d'autres termes, la representation des donnees en memoire
ressemble a la figure 4-2.

<!--
<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />
-->

<img alt="Trois tableaux : les tableaux s1 et s2 representant ces chaines sur la
pile, respectivement, et pointant tous deux vers les memes donnees de chaine
sur le tas." src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-2: The representation in memory of the variable
`s2` that has a copy of the pointer, length, and capacity of `s1`</span>
-->

<span class="caption">Figure 4-2 : La representation en memoire de la variable
`s2` qui a une copie du pointeur, de la longueur et de la capacite de
`s1`</span>

<!--
The representation does _not_ look like Figure 4-3, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could be very expensive in terms of runtime performance if
the data on the heap were large.
-->

La representation ne ressemble _pas_ a la figure 4-3, qui est ce a quoi la
memoire ressemblerait si Rust copiait egalement les donnees du tas. Si Rust
faisait cela, l'operation `s2 = s1` pourrait etre tres couteuse en termes de
performances a l'execution si les donnees sur le tas etaient volumineuses.

<!--
<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />
-->

<img alt="Quatre tableaux : deux tableaux representant les donnees de la pile
pour s1 et s2, et chacun pointe vers sa propre copie des donnees de chaine
sur le tas." src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-3: Another possibility for what `s2 = s1` might
do if Rust copied the heap data as well</span>
-->

<span class="caption">Figure 4-3 : Une autre possibilite de ce que `s2 = s1`
pourrait faire si Rust copiait egalement les donnees du tas</span>

<!--
Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-2 shows both data pointers pointing to the same location. This is a
problem: When `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a _double free_ error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.
-->

Plus tot, nous avons dit que quand une variable sort de la portee, Rust appelle
automatiquement la fonction `drop` et nettoie la memoire du tas pour cette
variable. Mais la figure 4-2 montre les deux pointeurs de donnees pointant vers
le meme emplacement. C'est un probleme : quand `s2` et `s1` sortent de la
portee, elles essaieront toutes les deux de liberer la meme memoire. C'est ce
qu'on appelle une erreur de _double liberation_ (double free) et c'est l'un des
bugs de securite memoire que nous avons mentionnes precedemment. Liberer la
memoire deux fois peut entrainer une corruption de la memoire, ce qui peut
potentiellement mener a des vulnerabilites de securite.

<!--
To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as
no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes
out of scope. Check out what happens when you try to use `s1` after `s2` is
created; it won't work:
-->

Pour garantir la securite de la memoire, apres la ligne `let s2 = s1;`, Rust
considere `s1` comme n'etant plus valide. Par consequent, Rust n'a pas besoin de
liberer quoi que ce soit quand `s1` sort de la portee. Regardez ce qui se passe
quand vous essayez d'utiliser `s1` apres que `s2` a ete creee ; cela ne
fonctionnera pas :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

<!--
You'll get an error like this because Rust prevents you from using the
invalidated reference:
-->

Vous obtiendrez une erreur comme celle-ci car Rust vous empeche d'utiliser la
reference invalidee :


```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

<!--
If you've heard the terms _shallow copy_ and _deep copy_ while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like making a shallow copy. But
because Rust also invalidates the first variable, instead of being called a
shallow copy, it's known as a _move_. In this example, we would say that `s1`
was _moved_ into `s2`. So, what actually happens is shown in Figure 4-4.
-->

Si vous avez entendu les termes _copie superficielle_ (shallow copy) et _copie
profonde_ (deep copy) en travaillant avec d'autres langages, le concept de
copier le pointeur, la longueur et la capacite sans copier les donnees ressemble
probablement a une copie superficielle. Mais comme Rust invalide aussi la
premiere variable, au lieu d'etre appelee une copie superficielle, c'est connu
sous le nom de _deplacement_ (move). Dans cet exemple, nous dirions que `s1` a
ete _deplacee_ dans `s2`. Donc, ce qui se passe reellement est illustre dans la
figure 4-4.

<!--
<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out because s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />
-->

<img alt="Trois tableaux : les tableaux s1 et s2 representant ces chaines sur la
pile, respectivement, et pointant tous deux vers les memes donnees de chaine
sur le tas. Le tableau s1 est grise car s1 n'est plus valide ; seul s2 peut
etre utilise pour acceder aux donnees du tas." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<!--
<span class="caption">Figure 4-4: The representation in memory after `s1` has
been invalidated</span>
-->

<span class="caption">Figure 4-4 : La representation en memoire apres que `s1`
a ete invalidee</span>

<!--
That solves our problem! With only `s2` valid, when it goes out of scope it
alone will free the memory, and we're done.
-->

Cela resout notre probleme ! Avec seulement `s2` valide, quand elle sort de la
portee, elle seule liberera la memoire, et c'est termine.

<!--
In addition, there's a design choice that's implied by this: Rust will never
automatically create "deep" copies of your data. Therefore, any _automatic_
copying can be assumed to be inexpensive in terms of runtime performance.
-->

De plus, il y a un choix de conception qui est implique par cela : Rust ne creera
jamais automatiquement de copies "profondes" de vos donnees. Par consequent,
toute copie _automatique_ peut etre consideree comme peu couteuse en termes de
performances a l'execution.

<!--
#### Scope and Assignment
-->

#### Portee et assignation

<!--
The inverse of this is true for the relationship between scoping, ownership, and
memory being freed via the `drop` function as well. When you assign a completely
new value to an existing variable, Rust will call `drop` and free the original
value's memory immediately. Consider this code, for example:
-->

L'inverse est egalement vrai pour la relation entre la portee, la possession et
la liberation de la memoire via la fonction `drop`. Quand vous assignez une
valeur completement nouvelle a une variable existante, Rust appellera `drop` et
liberera immediatement la memoire de la valeur d'origine. Considerez ce code,
par exemple :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04b-replacement-drop/src/main.rs:here}}
```

<!--
We initially declare a variable `s` and bind it to a `String` with the value
`"hello"`. Then, we immediately create a new `String` with the value `"ahoy"`
and assign it to `s`. At this point, nothing is referring to the original value
on the heap at all. Figure 4-5 illustrates the stack and heap data now:
-->

Nous declarons d'abord une variable `s` et la lions a une `String` avec la
valeur `"hello"`. Ensuite, nous creons immediatement une nouvelle `String` avec
la valeur `"ahoy"` et l'assignons a `s`. A ce stade, plus rien ne fait reference
a la valeur originale sur le tas. La figure 4-5 illustre les donnees de la pile
et du tas maintenant :

<!--
<img alt="One table representing the string value on the stack, pointing to
the second piece of string data (ahoy) on the heap, with the original string
data (hello) grayed out because it cannot be accessed anymore."
src="img/trpl04-05.svg" class="center" style="width: 50%;" />
-->

<img alt="Un tableau representant la valeur de la chaine sur la pile, pointant vers
la deuxieme donnee de chaine (ahoy) sur le tas, avec la donnee de chaine
originale (hello) grisee car elle n'est plus accessible."
src="img/trpl04-05.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-5: The representation in memory after the initial
value has been replaced in its entirety</span>
-->

<span class="caption">Figure 4-5 : La representation en memoire apres que la
valeur initiale a ete entierement remplacee</span>

<!--
The original string thus immediately goes out of scope. Rust will run the `drop`
function on it and its memory will be freed right away. When we print the value
at the end, it will be `"ahoy, world!"`.
-->

La chaine originale sort donc immediatement de la portee. Rust executera la
fonction `drop` sur celle-ci et sa memoire sera liberee immediatement. Quand
nous affichons la valeur a la fin, ce sera `"ahoy, world!"`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ways-variables-and-data-interact-clone"></a>

<!--
#### Variables and Data Interacting with Clone
-->

#### Interaction entre les variables et les donnees avec Clone

<!--
If we _do_ want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We'll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you've probably seen them before.
-->

Si nous _voulons_ vraiment copier en profondeur les donnees du tas de la
`String`, et pas seulement les donnees de la pile, nous pouvons utiliser une
methode courante appelee `clone`. Nous aborderons la syntaxe des methodes au
chapitre 5, mais comme les methodes sont une fonctionnalite courante dans de
nombreux langages de programmation, vous les avez probablement deja vues.

<!--
Here's an example of the `clone` method in action:
-->

Voici un exemple de la methode `clone` en action :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

<!--
This works just fine and explicitly produces the behavior shown in Figure 4-3,
where the heap data _does_ get copied.
-->

Cela fonctionne parfaitement et produit explicitement le comportement illustre
dans la figure 4-3, ou les donnees du tas _sont_ effectivement copiees.

<!--
When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It's a visual indicator that something
different is going on.
-->

Quand vous voyez un appel a `clone`, vous savez qu'un code arbitraire est
execute et que ce code peut etre couteux. C'est un indicateur visuel que quelque
chose de different se passe.

<!--
#### Stack-Only Data: Copy
-->

#### Donnees uniquement sur la pile : Copy

<!--
There's another wrinkle we haven't talked about yet. This code using
integers—part of which was shown in Listing 4-2—works and is valid:
-->

Il y a une autre subtilite dont nous n'avons pas encore parle. Ce code utilisant
des entiers -- dont une partie a ete montree dans le listing 4-2 -- fonctionne
et est valide :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

<!--
But this code seems to contradict what we just learned: We don't have a call to
`clone`, but `x` is still valid and wasn't moved into `y`.
-->

Mais ce code semble contredire ce que nous venons d'apprendre : nous n'avons pas
d'appel a `clone`, mais `x` est toujours valide et n'a pas ete deplace dans `y`.

<!--
The reason is that types such as integers that have a known size at compile
time are stored entirely on the stack, so copies of the actual values are quick
to make. That means there's no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there's no difference
between deep and shallow copying here, so calling `clone` wouldn't do anything
different from the usual shallow copying, and we can leave it out.
-->

La raison est que les types tels que les entiers qui ont une taille connue a la
compilation sont entierement stockes sur la pile, donc les copies des valeurs
reelles sont rapides a effectuer. Cela signifie qu'il n'y a aucune raison de
vouloir empecher `x` d'etre valide apres avoir cree la variable `y`. En
d'autres termes, il n'y a pas de difference entre copie profonde et copie
superficielle ici, donc appeler `clone` ne ferait rien de different de la copie
superficielle habituelle, et nous pouvons l'omettre.

<!--
Rust has a special annotation called the `Copy` trait that we can place on
types that are stored on the stack, as integers are (we'll talk more about
traits in [Chapter 10][traits] ignore
-->). If a type implements the `Copy`
trait, variables that use it do not move, but rather are trivially copied,
making them still valid after assignment to another variable.
-->

Rust a une annotation speciale appelee le trait `Copy` que nous pouvons placer
sur les types stockes sur la pile, comme le sont les entiers (nous parlerons
davantage des traits au [chapitre 10][traits]<!--
ignore
-->). Si un type
implemente le trait `Copy`, les variables qui l'utilisent ne sont pas deplacees,
mais sont plutot copiees de maniere triviale, ce qui les rend toujours valides
apres l'assignation a une autre variable.

<!--
Rust won't let us annotate a type with `Copy` if the type, or any of its parts,
has implemented the `Drop` trait. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we'll get a compile-time error. To learn about how to add the `Copy` annotation
to your type to implement the trait, see ["Derivable
Traits"][derivable-traits] ignore
--> in Appendix C.
-->

Rust ne nous laissera pas annoter un type avec `Copy` si le type, ou l'une de
ses parties, a implemente le trait `Drop`. Si le type necessite qu'un traitement
special se produise quand la valeur sort de la portee et que nous ajoutons
l'annotation `Copy` a ce type, nous obtiendrons une erreur de compilation. Pour
apprendre comment ajouter l'annotation `Copy` a votre type pour implementer le
trait, consultez ["Les traits derivables"][derivable-traits]<!--
ignore
--> dans
l'annexe C.

<!--
So, what types implement the `Copy` trait? You can check the documentation for
the given type to be sure, but as a general rule, any group of simple scalar
values can implement `Copy`, and nothing that requires allocation or is some
form of resource can implement `Copy`. Here are some of the types that
implement `Copy`:
-->

Alors, quels types implementent le trait `Copy` ? Vous pouvez consulter la
documentation du type en question pour en etre sur, mais en regle generale, tout
groupe de valeurs scalaires simples peut implementer `Copy`, et rien qui
necessite une allocation ou qui constitue une forme de ressource ne peut
implementer `Copy`. Voici quelques-uns des types qui implementent `Copy` :

<!--
- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example,
  `(i32, i32)` implements `Copy`, but `(i32, String)` does not.
-->

- Tous les types d'entiers, comme `u32`.
- Le type booleen, `bool`, avec les valeurs `true` et `false`.
- Tous les types a virgule flottante, comme `f64`.
- Le type caractere, `char`.
- Les tuples, s'ils ne contiennent que des types qui implementent egalement
  `Copy`. Par exemple, `(i32, i32)` implemente `Copy`, mais `(i32, String)` ne
  l'implemente pas.

<!--
### Ownership and Functions
-->

### La possession et les fonctions

<!--
The mechanics of passing a value to a function are similar to those when
assigning a value to a variable. Passing a variable to a function will move or
copy, just as assignment does. Listing 4-3 has an example with some annotations
showing where variables go into and out of scope.
-->

Le mecanisme de passage d'une valeur a une fonction est similaire a celui de
l'assignation d'une valeur a une variable. Passer une variable a une fonction
la deplacera ou la copiera, tout comme l'assignation. Le listing 4-3 contient un
exemple avec des annotations montrant ou les variables entrent et sortent de la
portee.

<Listing number="4-3" file-name="src/main.rs" caption="Fonctions avec la possession et la portee annotees">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

</Listing>

<!--
If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile-time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.
-->

Si nous essayions d'utiliser `s` apres l'appel a `takes_ownership`, Rust
lancerait une erreur de compilation. Ces verifications statiques nous protegent
des erreurs. Essayez d'ajouter du code a `main` qui utilise `s` et `x` pour
voir ou vous pouvez les utiliser et ou les regles de la possession vous en
empechent.

<!--
### Return Values and Scope
-->

### Les valeurs de retour et la portee

<!--
Returning values can also transfer ownership. Listing 4-4 shows an example of a
function that returns some value, with similar annotations as those in Listing
4-3.
-->

Renvoyer des valeurs peut egalement transferer la possession. Le listing 4-4
montre un exemple de fonction qui renvoie une valeur, avec des annotations
similaires a celles du listing 4-3.

<Listing number="4-4" file-name="src/main.rs" caption="Transfert de la possession des valeurs de retour">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

</Listing>

<!--
The ownership of a variable follows the same pattern every time: Assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.
-->

La possession d'une variable suit le meme schema a chaque fois : assigner une
valeur a une autre variable la deplace. Quand une variable qui inclut des donnees
sur le tas sort de la portee, la valeur sera nettoyee par `drop` a moins que la
possession des donnees n'ait ete transferee a une autre variable.

<!--
While this works, taking ownership and then returning ownership with every
function is a bit tedious. What if we want to let a function use a value but
not take ownership? It's quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting
from the body of the function that we might want to return as well.
-->

Bien que cela fonctionne, prendre la possession puis la rendre avec chaque
fonction est un peu fastidieux. Que faire si nous voulons laisser une fonction
utiliser une valeur sans en prendre la possession ? C'est assez agacant que tout
ce que nous passons doive aussi etre renvoye si nous voulons le reutiliser, en
plus de toutes les donnees resultant du corps de la fonction que nous pourrions
egalement vouloir renvoyer.

<!--
Rust does let us return multiple values using a tuple, as shown in Listing 4-5.
-->

Rust nous permet de renvoyer plusieurs valeurs en utilisant un tuple, comme le
montre le listing 4-5.

<Listing number="4-5" file-name="src/main.rs" caption="Renvoi de la possession des parametres">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

</Listing>

<!--
But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for using a value without
transferring ownership: references.
-->

Mais c'est beaucoup trop de ceremonie et de travail pour un concept qui devrait
etre courant. Heureusement pour nous, Rust dispose d'une fonctionnalite pour
utiliser une valeur sans transferer la possession : les references.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[methods]: ch05-03-method-syntax.html#methods
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
