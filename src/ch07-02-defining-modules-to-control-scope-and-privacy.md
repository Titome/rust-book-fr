<!--
Old headings. Do not remove or links may break.
-->

<a id="defining-modules-to-control-scope-and-privacy"></a>

<!--
## Control Scope and Privacy with Modules
-->

## Contrôler la portée et la confidentialité avec les modules

<!--
In this section, we'll talk about modules and other parts of the module system,
namely _paths_, which allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We'll also discuss
the `as` keyword, external packages, and the glob operator.
-->

Dans cette section, nous parlerons des modules et d'autres parties du système
de modules, à savoir les *chemins* (*paths*), qui vous permettent de nommer des
éléments ; le mot-clé `use` qui amène un chemin dans la portée ; et le mot-clé
`pub` pour rendre des éléments publics. Nous aborderons également le mot-clé
`as`, les packages externes et l'opérateur glob.

<!--
### Modules Cheat Sheet
-->

### Aide-mémoire des modules

<!--
Before we get to the details of modules and paths, here we provide a quick
reference on how modules, paths, the `use` keyword, and the `pub` keyword work
in the compiler, and how most developers organize their code. We'll be going
through examples of each of these rules throughout this chapter, but this is a
great place to refer to as a reminder of how modules work.
-->

Avant d'entrer dans les détails des modules et des chemins, voici une référence
rapide sur le fonctionnement des modules, des chemins, du mot-clé `use` et du
mot-clé `pub` dans le compilateur, et sur la façon dont la plupart des
développeurs organisent leur code. Nous passerons en revue des exemples pour
chacune de ces règles tout au long de ce chapitre, mais c'est un excellent
endroit auquel se référer pour se rappeler le fonctionnement des modules.

<!--
- **Start from the crate root**: When compiling a crate, the compiler first
  looks in the crate root file (usually _src/lib.rs_ for a library crate and
  _src/main.rs_ for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules;
  say you declare a "garden" module with `mod garden;`. The compiler will look
  for the module's code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod
    garden`
  - In the file _src/garden.rs_
  - In the file _src/garden/mod.rs_
- **Declaring submodules**: In any file other than the crate root, you can
  declare submodules. For example, you might declare `mod vegetables;` in
  _src/garden.rs_. The compiler will look for the submodule's code within the
  directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
  - In the file _src/garden/vegetables.rs_
  - In the file _src/garden/vegetables/mod.rs_
- **Paths to code in modules**: Once a module is part of your crate, you can
  refer to code in that module from anywhere else in that same crate, as long
  as the privacy rules allow, using the path to the code. For example, an
  `Asparagus` type in the garden vegetables module would be found at
  `crate::garden::vegetables::Asparagus`.
- **Private vs. public**: Code within a module is private from its parent
  modules by default. To make a module public, declare it with `pub mod`
  instead of `mod`. To make items within a public module public as well, use
  `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
  items to reduce repetition of long paths. In any scope that can refer to
  `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
  crate::garden::vegetables::Asparagus;`, and from then on you only need to
  write `Asparagus` to make use of that type in the scope.
-->

- **Commencer par la racine de la crate** : lors de la compilation d'une crate,
  le compilateur cherche d'abord dans le fichier racine de la crate
  (généralement *src/lib.rs* pour une crate de bibliothèque et *src/main.rs*
  pour une crate binaire) le code à compiler.
- **Déclarer des modules** : dans le fichier racine de la crate, vous pouvez
  déclarer de nouveaux modules ; par exemple, vous déclarez un module « garden »
  avec `mod garden;`. Le compilateur cherchera le code du module aux endroits
  suivants :
  - En ligne, entre des accolades qui remplacent le point-virgule après
    `mod garden`
  - Dans le fichier *src/garden.rs*
  - Dans le fichier *src/garden/mod.rs*
- **Déclarer des sous-modules** : dans tout fichier autre que la racine de la
  crate, vous pouvez déclarer des sous-modules. Par exemple, vous pourriez
  déclarer `mod vegetables;` dans *src/garden.rs*. Le compilateur cherchera le
  code du sous-module dans le répertoire portant le nom du module parent, aux
  endroits suivants :
  - En ligne, directement après `mod vegetables`, entre des accolades au lieu
    du point-virgule
  - Dans le fichier *src/garden/vegetables.rs*
  - Dans le fichier *src/garden/vegetables/mod.rs*
- **Chemins vers le code dans les modules** : une fois qu'un module fait partie
  de votre crate, vous pouvez faire référence au code de ce module depuis
  n'importe quel autre endroit de la même crate, tant que les règles de
  confidentialité le permettent, en utilisant le chemin vers le code. Par
  exemple, un type `Asparagus` dans le module garden vegetables se trouverait à
  `crate::garden::vegetables::Asparagus`.
- **Privé vs. public** : le code au sein d'un module est privé par rapport à
  ses modules parents par défaut. Pour rendre un module public, déclarez-le avec
  `pub mod` au lieu de `mod`. Pour rendre également publics les éléments au sein
  d'un module public, utilisez `pub` avant leurs déclarations.
- **Le mot-clé `use`** : au sein d'une portée, le mot-clé `use` crée des
  raccourcis vers des éléments pour réduire la répétition de chemins longs. Dans
  toute portée pouvant faire référence à
  `crate::garden::vegetables::Asparagus`, vous pouvez créer un raccourci avec
  `use crate::garden::vegetables::Asparagus;`, et à partir de là, il vous suffit
  d'écrire `Asparagus` pour utiliser ce type dans la portée.

<!--
Here, we create a binary crate named `backyard` that illustrates these rules.
The crate's directory, also named _backyard_, contains these files and
directories:
-->

Ici, nous créons une crate binaire nommée `backyard` qui illustre ces règles.
Le répertoire de la crate, également nommé *backyard*, contient ces fichiers et
répertoires :

<!--
```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
-->

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

<!--
The crate root file in this case is _src/main.rs_, and it contains:
-->

Le fichier racine de la crate dans ce cas est *src/main.rs*, et il contient :


<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

<!--
The `pub mod garden;` line tells the compiler to include the code it finds in
_src/garden.rs_, which is:
-->

La ligne `pub mod garden;` indique au compilateur d'inclure le code qu'il trouve
dans *src/garden.rs*, qui est :


<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

<!--
Here, `pub mod vegetables;` means the code in _src/garden/vegetables.rs_ is
included too. That code is:
-->

Ici, `pub mod vegetables;` signifie que le code dans *src/garden/vegetables.rs*
est également inclus. Ce code est :


```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

<!--
Now let's get into the details of these rules and demonstrate them in action!
-->

Maintenant, entrons dans les détails de ces règles et voyons-les en action !

<!--
### Grouping Related Code in Modules
-->

### Regrouper le code associé dans des modules

<!--
_Modules_ let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the _privacy_ of items because code within a
module is private by default. Private items are internal implementation details
not available for outside use. We can choose to make modules and the items
within them public, which exposes them to allow external code to use and depend
on them.
-->

Les *modules* nous permettent d'organiser le code au sein d'une crate pour la
lisibilité et la facilité de réutilisation. Les modules nous permettent
également de contrôler la *confidentialité* des éléments, car le code au sein
d'un module est privé par défaut. Les éléments privés sont des détails
d'implémentation internes non disponibles pour une utilisation externe. Nous
pouvons choisir de rendre les modules et les éléments qu'ils contiennent
publics, ce qui les expose pour permettre au code externe de les utiliser et
d'en dépendre.

<!--
As an example, let's write a library crate that provides the functionality of a
restaurant. We'll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code rather than the
implementation of a restaurant.
-->

À titre d'exemple, écrivons une crate de bibliothèque qui fournit les
fonctionnalités d'un restaurant. Nous définirons les signatures des fonctions
mais laisserons leurs corps vides pour nous concentrer sur l'organisation du
code plutôt que sur l'implémentation d'un restaurant.

<!--
In the restaurant industry, some parts of a restaurant are referred to as front
of house and others as back of house. _Front of house_ is where customers are;
this encompasses where the hosts seat customers, servers take orders and
payment, and bartenders make drinks. _Back of house_ is where the chefs and
cooks work in the kitchen, dishwashers clean up, and managers do administrative
work.
-->

Dans le secteur de la restauration, certaines parties d'un restaurant sont
désignées comme la salle (*front of house*) et d'autres comme les cuisines
(*back of house*). La *salle* est l'endroit où se trouvent les clients ; cela
englobe l'endroit où les hôtes placent les clients, où les serveurs prennent les
commandes et les paiements, et où les barmans préparent les boissons. Les
*cuisines* sont l'endroit où les chefs et les cuisiniers travaillent, où les
plongeurs nettoient et où les responsables effectuent le travail administratif.

<!--
To structure our crate in this way, we can organize its functions into nested
modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`. Then, enter the code in Listing 7-1 into _src/lib.rs_ to
define some modules and function signatures; this code is the front of house
section.
-->

Pour structurer notre crate de cette façon, nous pouvons organiser ses fonctions
en modules imbriqués. Créez une nouvelle bibliothèque nommée `restaurant` en
exécutant `cargo new restaurant --lib`. Ensuite, entrez le code du listing 7-1
dans *src/lib.rs* pour définir quelques modules et signatures de fonctions ; ce
code est la section de la salle.


<Listing number="7-1" file-name="src/lib.rs" caption="Un module `front_of_house` contenant d'autres modules qui contiennent eux-mêmes des fonctions">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

<!--
We define a module with the `mod` keyword followed by the name of the module
(in this case, `front_of_house`). The body of the module then goes inside curly
brackets. Inside modules, we can place other modules, as in this case with the
modules `hosting` and `serving`. Modules can also hold definitions for other
items, such as structs, enums, constants, traits, and as in Listing 7-1,
functions.
-->

Nous définissons un module avec le mot-clé `mod` suivi du nom du module (dans
ce cas, `front_of_house`). Le corps du module se place ensuite entre des
accolades. À l'intérieur des modules, nous pouvons placer d'autres modules,
comme dans ce cas avec les modules `hosting` et `serving`. Les modules peuvent
également contenir des définitions pour d'autres éléments, tels que des structs,
des enums, des constantes, des traits et, comme dans le listing 7-1, des
fonctions.

<!--
By using modules, we can group related definitions together and name why
they're related. Programmers using this code can navigate the code based on the
groups rather than having to read through all the definitions, making it easier
to find the definitions relevant to them. Programmers adding new functionality
to this code would know where to place the code to keep the program organized.
-->

En utilisant des modules, nous pouvons regrouper les définitions associées
ensemble et nommer pourquoi elles sont liées. Les développeurs utilisant ce code
peuvent naviguer dans le code en se basant sur les groupes plutôt que de devoir
lire toutes les définitions, ce qui facilite la recherche des définitions qui
les intéressent. Les développeurs ajoutant de nouvelles fonctionnalités à ce
code sauraient où placer le code pour garder le programme organisé.

<!--
Earlier, we mentioned that _src/main.rs_ and _src/lib.rs_ are called _crate
roots_. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate's module structure,
known as the _module tree_.
-->

Plus tôt, nous avons mentionné que *src/main.rs* et *src/lib.rs* sont appelés
*racines de crate*. La raison de ce nom est que le contenu de l'un ou l'autre
de ces deux fichiers forme un module nommé `crate` à la racine de la structure
de modules de la crate, connue sous le nom d'*arbre de modules*.

<!--
Listing 7-2 shows the module tree for the structure in Listing 7-1.
-->

Le listing 7-2 montre l'arbre de modules pour la structure du listing 7-1.

<!--
```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
-->

<Listing number="7-2" caption="L'arbre de modules pour le code du listing 7-1">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

<!--
This tree shows how some of the modules nest inside other modules; for example,
`hosting` nests inside `front_of_house`. The tree also shows that some modules
are _siblings_, meaning they're defined in the same module; `hosting` and
`serving` are siblings defined within `front_of_house`. If module A is
contained inside module B, we say that module A is the _child_ of module B and
that module B is the _parent_ of module A. Notice that the entire module tree
is rooted under the implicit module named `crate`.
-->

Cet arbre montre comment certains modules s'imbriquent dans d'autres modules ;
par exemple, `hosting` s'imbrique dans `front_of_house`. L'arbre montre
également que certains modules sont *frères et sœurs* (*siblings*), ce qui
signifie qu'ils sont définis dans le même module ; `hosting` et `serving` sont
des modules frères définis au sein de `front_of_house`. Si le module A est
contenu dans le module B, nous disons que le module A est l'*enfant* du module B
et que le module B est le *parent* du module A. Notez que l'arbre de modules
entier est enraciné sous le module implicite nommé `crate`.

<!--
The module tree might remind you of the filesystem's directory tree on your
computer; this is a very apt comparison! Just like directories in a filesystem,
you use modules to organize your code. And just like files in a directory, we
need a way to find our modules.
-->

L'arbre de modules pourrait vous rappeler l'arborescence de répertoires du
système de fichiers de votre ordinateur ; c'est une comparaison très pertinente !
Tout comme les répertoires dans un système de fichiers, vous utilisez les
modules pour organiser votre code. Et tout comme les fichiers dans un
répertoire, nous avons besoin d'un moyen de trouver nos modules.
