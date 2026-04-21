<!--
Old headings. Do not remove or links may break.
-->

<a id="managing-growing-projects-with-packages-crates-and-modules"></a>

<!--
# Packages, Crates, and Modules
-->

# Les packages, les crates et les modules

<!--
As you write large programs, organizing your code will become increasingly
important. By grouping related functionality and separating code with distinct
features, you'll clarify where to find code that implements a particular
feature and where to go to change how a feature works.
-->

Au fur et à mesure que vous écrivez des programmes de plus en plus grands,
l'organisation de votre code deviendra de plus en plus importante. En regroupant
les fonctionnalités associées et en séparant le code selon ses différentes
responsabilités, vous clarifierez où trouver le code qui implémente une
fonctionnalité particulière et où intervenir pour modifier le comportement
d'une fonctionnalité.

<!--
The programs we've written so far have been in one module in one file. As a
project grows, you should organize code by splitting it into multiple modules
and then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. This chapter covers all
these techniques. For very large projects comprising a set of interrelated
packages that evolve together, Cargo provides workspaces, which we'll cover in
["Cargo Workspaces"][workspaces] ignore
--> in Chapter 14.
-->

Les programmes que nous avons écrits jusqu'à présent se trouvaient dans un seul
module dans un seul fichier. À mesure qu'un projet grandit, vous devriez
organiser le code en le divisant en plusieurs modules puis en plusieurs fichiers.
Un package peut contenir plusieurs crates binaires et éventuellement une crate
de bibliothèque. Au fur et à mesure qu'un package grandit, vous pouvez en
extraire des parties dans des crates séparées qui deviennent des dépendances
externes. Ce chapitre couvre toutes ces techniques. Pour les très grands projets
comprenant un ensemble de packages interdépendants qui évoluent ensemble, Cargo
fournit les espaces de travail (*workspaces*), que nous aborderons dans
[« Les espaces de travail Cargo »][workspaces]<!--
ignore
--> au chapitre 14.

<!--
We'll also discuss encapsulating implementation details, which lets you reuse
code at a higher level: Once you've implemented an operation, other code can
call your code via its public interface without having to know how the
implementation works. The way you write code defines which parts are public for
other code to use and which parts are private implementation details that you
reserve the right to change. This is another way to limit the amount of detail
you have to keep in your head.
-->

Nous aborderons également l'encapsulation des détails d'implémentation, qui
vous permet de réutiliser du code à un niveau plus élevé : une fois que vous
avez implémenté une opération, d'autre code peut appeler votre code via son
interface publique sans avoir à connaître le fonctionnement de l'implémentation.
La façon dont vous écrivez le code définit quelles parties sont publiques pour
que d'autre code puisse les utiliser et quelles parties sont des détails
d'implémentation privés que vous vous réservez le droit de modifier. C'est une
autre façon de limiter la quantité de détails que vous devez garder en tête.

<!--
A related concept is scope: The nested context in which code is written has a
set of names that are defined as "in scope." When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can't have two items with the
same name in the same scope; tools are available to resolve name conflicts.
-->

Un concept associé est la portée (*scope*) : le contexte imbriqué dans lequel le
code est écrit possède un ensemble de noms définis comme étant « dans la
portée ». Lors de la lecture, de l'écriture et de la compilation du code, les
développeurs et les compilateurs doivent savoir si un nom particulier à un
endroit particulier fait référence à une variable, une fonction, une struct, une
enum, un module, une constante ou un autre élément, et ce que cet élément
signifie. Vous pouvez créer des portées et modifier quels noms sont dans ou hors
de la portée. Vous ne pouvez pas avoir deux éléments portant le même nom dans la
même portée ; des outils sont disponibles pour résoudre les conflits de noms.

<!--
Rust has a number of features that allow you to manage your code's
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the _module system_, include:
-->

Rust dispose d'un certain nombre de fonctionnalités qui vous permettent de gérer
l'organisation de votre code, notamment quels détails sont exposés, quels
détails sont privés et quels noms se trouvent dans chaque portée de vos
programmes. Ces fonctionnalités, parfois collectivement appelées le *système de
modules*, comprennent :

<!--
* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules and use**: Let you control the organization, scope, and privacy of
paths
* **Paths**: A way of naming an item, such as a struct, function, or module
-->

* **Les packages** : une fonctionnalité de Cargo qui vous permet de compiler,
tester et partager des crates
* **Les crates** : un arbre de modules qui produit une bibliothèque ou un
exécutable
* **Les modules et `use`** : vous permettent de contrôler l'organisation, la
portée et la confidentialité des chemins
* **Les chemins (*paths*)** : une façon de nommer un élément, comme une struct,
une fonction ou un module

<!--
In this chapter, we'll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!
-->

Dans ce chapitre, nous aborderons toutes ces fonctionnalités, discuterons de
la façon dont elles interagissent et expliquerons comment les utiliser pour
gérer la portée. À la fin, vous devriez avoir une compréhension solide du
système de modules et être capable de travailler avec les portées comme un pro !

[workspaces]: ch14-03-cargo-workspaces.html
