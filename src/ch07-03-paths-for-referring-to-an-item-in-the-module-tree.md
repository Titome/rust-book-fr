<!--
## Paths for Referring to an Item in the Module Tree
-->

## Les chemins pour faire référence à un élément dans l'arbre de modules

<!--
To show Rust where to find an item in a module tree, we use a path in the same
way we use a path when navigating a filesystem. To call a function, we need to
know its path.
-->

Pour indiquer à Rust où trouver un élément dans un arbre de modules, nous
utilisons un chemin de la même manière que nous utilisons un chemin pour
naviguer dans un système de fichiers. Pour appeler une fonction, nous devons
connaître son chemin.

<!--
A path can take two forms:

- An _absolute path_ is the full path starting from a crate root; for code
  from an external crate, the absolute path begins with the crate name, and for
  code from the current crate, it starts with the literal `crate`.
- A _relative path_ starts from the current module and uses `self`, `super`, or
  an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).
-->

Un chemin peut prendre deux formes :

- Un *chemin absolu* est le chemin complet à partir de la racine d'une crate ;
  pour du code provenant d'une crate externe, le chemin absolu commence par le
  nom de la crate, et pour du code provenant de la crate courante, il commence
  par le mot littéral `crate`.
- Un *chemin relatif* commence à partir du module courant et utilise `self`,
  `super` ou un identifiant dans le module courant.

Les chemins absolus et relatifs sont suivis d'un ou plusieurs identifiants
séparés par des doubles deux-points (`::`).

<!--
Returning to Listing 7-1, say we want to call the `add_to_waitlist` function.
This is the same as asking: What's the path of the `add_to_waitlist` function?
Listing 7-3 contains Listing 7-1 with some of the modules and functions removed.
-->

Revenons au listing 7-1, et supposons que nous voulions appeler la fonction
`add_to_waitlist`. C'est comme demander : quel est le chemin de la fonction
`add_to_waitlist` ? Le listing 7-3 contient le listing 7-1 avec certains modules
et fonctions supprimés.

<!--
We'll show two ways to call the `add_to_waitlist` function from a new function,
`eat_at_restaurant`, defined in the crate root. These paths are correct, but
there's another problem remaining that will prevent this example from compiling
as is. We'll explain why in a bit.
-->

Nous montrerons deux façons d'appeler la fonction `add_to_waitlist` depuis une
nouvelle fonction, `eat_at_restaurant`, définie à la racine de la crate. Ces
chemins sont corrects, mais il reste un autre problème qui empêchera cet exemple
de compiler en l'état. Nous expliquerons pourquoi dans un instant.

<!--
The `eat_at_restaurant` function is part of our library crate's public API, so
we mark it with the `pub` keyword. In the ["Exposing Paths with the `pub`
Keyword"][pub] ignore
--> section, we'll go into more detail about `pub`.
-->

La fonction `eat_at_restaurant` fait partie de l'API publique de notre crate de
bibliothèque, nous la marquons donc avec le mot-clé `pub`. Dans la section
[« Exposer les chemins avec le mot-clé `pub` »][pub]<!--
ignore
-->, nous
entrerons plus en détail sur `pub`.


<Listing number="7-3" file-name="src/lib.rs" caption="Appel de la fonction `add_to_waitlist` en utilisant des chemins absolus et relatifs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

<!--
The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path. We then include each of the successive modules until we
make our way to `add_to_waitlist`. You can imagine a filesystem with the same
structure: We'd specify the path `/front_of_house/hosting/add_to_waitlist` to
run the `add_to_waitlist` program; using the `crate` name to start from the
crate root is like using `/` to start from the filesystem root in your shell.
-->

La première fois que nous appelons la fonction `add_to_waitlist` dans
`eat_at_restaurant`, nous utilisons un chemin absolu. La fonction
`add_to_waitlist` est définie dans la même crate que `eat_at_restaurant`, ce qui
signifie que nous pouvons utiliser le mot-clé `crate` pour commencer un chemin
absolu. Nous incluons ensuite chacun des modules successifs jusqu'à atteindre
`add_to_waitlist`. Vous pouvez imaginer un système de fichiers avec la même
structure : nous spécifierions le chemin
`/front_of_house/hosting/add_to_waitlist` pour exécuter le programme
`add_to_waitlist` ; utiliser le nom `crate` pour partir de la racine de la crate
revient à utiliser `/` pour partir de la racine du système de fichiers dans
votre terminal.

<!--
The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. Starting with a module name means
that the path is relative.
-->

La deuxième fois que nous appelons `add_to_waitlist` dans `eat_at_restaurant`,
nous utilisons un chemin relatif. Le chemin commence par `front_of_house`, le
nom du module défini au même niveau de l'arbre de modules que
`eat_at_restaurant`. Ici, l'équivalent dans le système de fichiers serait
d'utiliser le chemin `front_of_house/hosting/add_to_waitlist`. Commencer par un
nom de module signifie que le chemin est relatif.

<!--
Choosing whether to use a relative or absolute path is a decision you'll make
based on your project, and it depends on whether you're more likely to move
item definition code separately from or together with the code that uses the
item. For example, if we moved the `front_of_house` module and the
`eat_at_restaurant` function into a module named `customer_experience`, we'd
need to update the absolute path to `add_to_waitlist`, but the relative path
would still be valid. However, if we moved the `eat_at_restaurant` function
separately into a module named `dining`, the absolute path to the
`add_to_waitlist` call would stay the same, but the relative path would need to
be updated. Our preference in general is to specify absolute paths because it's
more likely we'll want to move code definitions and item calls independently of
each other.
-->

Choisir d'utiliser un chemin relatif ou absolu est une décision que vous
prendrez en fonction de votre projet, et cela dépend de la probabilité que vous
déplaciez le code de définition de l'élément séparément ou ensemble avec le code
qui utilise l'élément. Par exemple, si nous déplacions le module
`front_of_house` et la fonction `eat_at_restaurant` dans un module nommé
`customer_experience`, nous devrions mettre à jour le chemin absolu vers
`add_to_waitlist`, mais le chemin relatif serait toujours valide. En revanche,
si nous déplacions la fonction `eat_at_restaurant` séparément dans un module
nommé `dining`, le chemin absolu vers l'appel `add_to_waitlist` resterait le
même, mais le chemin relatif devrait être mis à jour. Notre préférence en
général est de spécifier des chemins absolus, car il est plus probable que nous
voudrons déplacer les définitions de code et les appels d'éléments
indépendamment les uns des autres.

<!--
Let's try to compile Listing 7-3 and find out why it won't compile yet! The
errors we get are shown in Listing 7-4.
-->

Essayons de compiler le listing 7-3 et découvrons pourquoi il ne compile pas
encore ! Les erreurs que nous obtenons sont présentées dans le listing 7-4.


<Listing number="7-4" caption="Erreurs du compilateur lors de la compilation du code du listing 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

<!--
The error messages say that module `hosting` is private. In other words, we
have the correct paths for the `hosting` module and the `add_to_waitlist`
function, but Rust won't let us use them because it doesn't have access to the
private sections. In Rust, all items (functions, methods, structs, enums,
modules, and constants) are private to parent modules by default. If you want
to make an item like a function or struct private, you put it in a module.
-->

Les messages d'erreur indiquent que le module `hosting` est privé. En d'autres
termes, nous avons les chemins corrects pour le module `hosting` et la fonction
`add_to_waitlist`, mais Rust ne nous permet pas de les utiliser car il n'a pas
accès aux sections privées. En Rust, tous les éléments (fonctions, méthodes,
structs, enums, modules et constantes) sont privés par rapport aux modules
parents par défaut. Si vous voulez rendre un élément comme une fonction ou une
struct privé, vous le placez dans un module.

<!--
Items in a parent module can't use the private items inside child modules, but
items in child modules can use the items in their ancestor modules. This is
because child modules wrap and hide their implementation details, but the child
modules can see the context in which they're defined. To continue with our
metaphor, think of the privacy rules as being like the back office of a
restaurant: What goes on in there is private to restaurant customers, but
office managers can see and do everything in the restaurant they operate.
-->

Les éléments d'un module parent ne peuvent pas utiliser les éléments privés à
l'intérieur des modules enfants, mais les éléments des modules enfants peuvent
utiliser les éléments de leurs modules ancêtres. C'est parce que les modules
enfants enveloppent et cachent leurs détails d'implémentation, mais les modules
enfants peuvent voir le contexte dans lequel ils sont définis. Pour continuer
avec notre métaphore, pensez aux règles de confidentialité comme au bureau de
direction d'un restaurant : ce qui s'y passe est privé pour les clients du
restaurant, mais les responsables peuvent voir et faire tout dans le restaurant
qu'ils gèrent.

<!--
Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without breaking the outer code. However, Rust does
give you the option to expose inner parts of child modules' code to outer
ancestor modules by using the `pub` keyword to make an item public.
-->

Rust a choisi de faire fonctionner le système de modules de cette manière afin
que masquer les détails d'implémentation internes soit le comportement par
défaut. De cette façon, vous savez quelles parties du code interne vous pouvez
modifier sans casser le code externe. Cependant, Rust vous donne la possibilité
d'exposer les parties internes du code des modules enfants aux modules ancêtres
externes en utilisant le mot-clé `pub` pour rendre un élément public.

<!--
### Exposing Paths with the `pub` Keyword
-->

### Exposer les chemins avec le mot-clé `pub`

<!--
Let's return to the error in Listing 7-4 that told us the `hosting` module is
private. We want the `eat_at_restaurant` function in the parent module to have
access to the `add_to_waitlist` function in the child module, so we mark the
`hosting` module with the `pub` keyword, as shown in Listing 7-5.
-->

Revenons à l'erreur du listing 7-4 qui nous indiquait que le module `hosting`
est privé. Nous voulons que la fonction `eat_at_restaurant` dans le module
parent ait accès à la fonction `add_to_waitlist` dans le module enfant, donc
nous marquons le module `hosting` avec le mot-clé `pub`, comme montré dans le
listing 7-5.


<Listing number="7-5" file-name="src/lib.rs" caption="Déclarer le module `hosting` comme `pub` pour l'utiliser depuis `eat_at_restaurant`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

<!--
Unfortunately, the code in Listing 7-5 still results in compiler errors, as
shown in Listing 7-6.
-->

Malheureusement, le code du listing 7-5 produit toujours des erreurs de
compilation, comme montré dans le listing 7-6.


<Listing number="7-6" caption="Erreurs du compilateur lors de la compilation du code du listing 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

<!--
What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the _contents_ of `hosting` are still private; making the
module public doesn't make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it, not access its inner code.
Because modules are containers, there's not much we can do by only making the
module public; we need to go further and choose to make one or more of the
items within the module public as well.
-->

Que s'est-il passé ? Ajouter le mot-clé `pub` devant `mod hosting` rend le
module public. Avec ce changement, si nous pouvons accéder à `front_of_house`,
nous pouvons accéder à `hosting`. Mais le *contenu* de `hosting` est toujours
privé ; rendre le module public ne rend pas son contenu public. Le mot-clé `pub`
sur un module permet uniquement au code de ses modules ancêtres de s'y référer,
pas d'accéder à son code interne. Puisque les modules sont des conteneurs, nous
ne pouvons pas faire grand-chose en rendant uniquement le module public ; nous
devons aller plus loin et choisir de rendre un ou plusieurs des éléments du
module publics également.

<!--
The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.
-->

Les erreurs du listing 7-6 indiquent que la fonction `add_to_waitlist` est
privée. Les règles de confidentialité s'appliquent aux structs, enums,
fonctions et méthodes ainsi qu'aux modules.

<!--
Let's also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.
-->

Rendons également la fonction `add_to_waitlist` publique en ajoutant le mot-clé
`pub` avant sa définition, comme dans le listing 7-7.


<Listing number="7-7" file-name="src/lib.rs" caption="L'ajout du mot-clé `pub` à `mod hosting` et `fn add_to_waitlist` nous permet d'appeler la fonction depuis `eat_at_restaurant`.">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

<!--
Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `eat_at_restaurant` with respect to the privacy rules, let's
look at the absolute and the relative paths.
-->

Maintenant le code va compiler ! Pour comprendre pourquoi l'ajout du mot-clé
`pub` nous permet d'utiliser ces chemins dans `eat_at_restaurant` conformément
aux règles de confidentialité, examinons les chemins absolus et relatifs.

<!--
In the absolute path, we start with `crate`, the root of our crate's module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn't public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub`, and we can access its parent
module, so this function call works!
-->

Dans le chemin absolu, nous commençons par `crate`, la racine de l'arbre de
modules de notre crate. Le module `front_of_house` est défini à la racine de la
crate. Bien que `front_of_house` ne soit pas public, comme la fonction
`eat_at_restaurant` est définie dans le même module que `front_of_house`
(c'est-à-dire que `eat_at_restaurant` et `front_of_house` sont frères), nous
pouvons faire référence à `front_of_house` depuis `eat_at_restaurant`. Ensuite
vient le module `hosting` marqué avec `pub`. Nous pouvons accéder au module
parent de `hosting`, donc nous pouvons accéder à `hosting`. Enfin, la fonction
`add_to_waitlist` est marquée avec `pub`, et nous pouvons accéder à son module
parent, donc cet appel de fonction fonctionne !

<!--
In the relative path, the logic is the same as the absolute path except for the
first step: Rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!
-->

Dans le chemin relatif, la logique est la même que pour le chemin absolu sauf
pour la première étape : plutôt que de commencer à la racine de la crate, le
chemin commence à partir de `front_of_house`. Le module `front_of_house` est
défini dans le même module que `eat_at_restaurant`, donc le chemin relatif
commençant depuis le module dans lequel `eat_at_restaurant` est défini
fonctionne. Ensuite, puisque `hosting` et `add_to_waitlist` sont marqués avec
`pub`, le reste du chemin fonctionne, et cet appel de fonction est valide !

<!--
If you plan to share your library crate so that other projects can use your
code, your public API is your contract with users of your crate that determines
how they can interact with your code. There are many considerations around
managing changes to your public API to make it easier for people to depend on
your crate. These considerations are beyond the scope of this book; if you're
interested in this topic, see [the Rust API Guidelines][api-guidelines].
-->

Si vous prévoyez de partager votre crate de bibliothèque pour que d'autres
projets puissent utiliser votre code, votre API publique est votre contrat avec
les utilisateurs de votre crate qui détermine comment ils peuvent interagir avec
votre code. Il y a de nombreuses considérations autour de la gestion des
modifications de votre API publique pour faciliter la dépendance des gens envers
votre crate. Ces considérations dépassent le cadre de ce livre ; si vous êtes
intéressé par ce sujet, consultez [les directives de l'API Rust][api-guidelines].

<!--
> #### Best Practices for Packages with a Binary and a Library
>
> We mentioned that a package can contain both a _src/main.rs_ binary crate
> root as well as a _src/lib.rs_ library crate root, and both crates will have
> the package name by default. Typically, packages with this pattern of
> containing both a library and a binary crate will have just enough code in the
> binary crate to start an executable that calls code defined in the library
> crate. This lets other projects benefit from the most functionality that the
> package provides because the library crate's code can be shared.
>
> The module tree should be defined in _src/lib.rs_. Then, any public items can
> be used in the binary crate by starting paths with the name of the package.
> The binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: It can only use the public API.
> This helps you design a good API; not only are you the author, but you're
> also a client!
>
> In [Chapter 12][ch12] ignore
-->, we'll demonstrate this organizational
> practice with a command line program that will contain both a binary crate
> and a library crate.
-->

> #### Bonnes pratiques pour les packages avec un binaire et une bibliothèque
>
> Nous avons mentionné qu'un package peut contenir à la fois une racine de crate
> binaire *src/main.rs* et une racine de crate de bibliothèque *src/lib.rs*, et
> les deux crates porteront le nom du package par défaut. Typiquement, les
> packages suivant ce schéma de contenir à la fois une crate de bibliothèque et
> une crate binaire auront juste assez de code dans la crate binaire pour
> démarrer un exécutable qui appelle du code défini dans la crate de
> bibliothèque. Cela permet à d'autres projets de bénéficier du maximum de
> fonctionnalités que le package fournit, car le code de la crate de
> bibliothèque peut être partagé.
>
> L'arbre de modules devrait être défini dans *src/lib.rs*. Ensuite, tous les
> éléments publics peuvent être utilisés dans la crate binaire en commençant les
> chemins par le nom du package. La crate binaire devient un utilisateur de la
> crate de bibliothèque, tout comme une crate complètement externe utiliserait
> la crate de bibliothèque : elle ne peut utiliser que l'API publique. Cela vous
> aide à concevoir une bonne API ; vous n'êtes pas seulement l'auteur, mais
> aussi un client !
>
> Au [chapitre 12][ch12]<!--
ignore
-->, nous démontrerons cette pratique
> d'organisation avec un programme en ligne de commande qui contiendra à la fois
> une crate binaire et une crate de bibliothèque.

<!--
### Starting Relative Paths with `super`
-->

### Commencer les chemins relatifs avec `super`

<!--
We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax that means
to go to the parent directory. Using `super` allows us to reference an item
that we know is in the parent module, which can make rearranging the module
tree easier when the module is closely related to the parent but the parent
might be moved elsewhere in the module tree someday.
-->

Nous pouvons construire des chemins relatifs qui commencent dans le module
parent, plutôt que dans le module courant ou la racine de la crate, en utilisant
`super` au début du chemin. C'est comme commencer un chemin de système de
fichiers avec la syntaxe `..` qui signifie aller au répertoire parent. Utiliser
`super` nous permet de faire référence à un élément que nous savons être dans le
module parent, ce qui peut faciliter la réorganisation de l'arbre de modules
lorsque le module est étroitement lié au parent mais que le parent pourrait être
déplacé ailleurs dans l'arbre de modules un jour.

<!--
Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order`, starting with `super`.
-->

Considérons le code du listing 7-8 qui modélise la situation dans laquelle un
chef corrige une commande incorrecte et l'apporte personnellement au client. La
fonction `fix_incorrect_order` définie dans le module `back_of_house` appelle la
fonction `deliver_order` définie dans le module parent en spécifiant le chemin
vers `deliver_order`, en commençant par `super`.


<Listing number="7-8" file-name="src/lib.rs" caption="Appel d'une fonction en utilisant un chemin relatif commençant par `super`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

<!--
The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it.
Success! We think the `back_of_house` module and the `deliver_order` function
are likely to stay in the same relationship to each other and get moved
together should we decide to reorganize the crate's module tree. Therefore, we
used `super` so that we'll have fewer places to update code in the future if
this code gets moved to a different module.
-->

La fonction `fix_incorrect_order` se trouve dans le module `back_of_house`, donc
nous pouvons utiliser `super` pour aller au module parent de `back_of_house`,
qui dans ce cas est `crate`, la racine. De là, nous cherchons `deliver_order` et
le trouvons. Succès ! Nous pensons que le module `back_of_house` et la fonction
`deliver_order` sont susceptibles de rester dans la même relation l'un avec
l'autre et d'être déplacés ensemble si nous décidons de réorganiser l'arbre de
modules de la crate. Par conséquent, nous avons utilisé `super` afin d'avoir
moins d'endroits où mettre à jour le code à l'avenir si ce code est déplacé vers
un autre module.

<!--
### Making Structs and Enums Public
-->

### Rendre les structs et les enums publics

<!--
We can also use `pub` to designate structs and enums as public, but there are a
few extra details to the usage of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct's fields
will still be private. We can make each field public or not on a case-by-case
basis. In Listing 7-9, we've defined a public `back_of_house::Breakfast` struct
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what's in season and in stock. The available fruit changes quickly, so
customers can't choose the fruit or even see which fruit they'll get.
-->

Nous pouvons également utiliser `pub` pour désigner les structs et les enums
comme publics, mais il y a quelques détails supplémentaires concernant
l'utilisation de `pub` avec les structs et les enums. Si nous utilisons `pub`
avant une définition de struct, nous rendons la struct publique, mais les champs
de la struct resteront privés. Nous pouvons rendre chaque champ public ou non au
cas par cas. Dans le listing 7-9, nous avons défini une struct publique
`back_of_house::Breakfast` avec un champ public `toast` mais un champ privé
`seasonal_fruit`. Cela modélise le cas d'un restaurant où le client peut choisir
le type de pain qui accompagne un repas, mais le chef décide quel fruit
accompagne le repas en fonction de ce qui est de saison et en stock. Les fruits
disponibles changent rapidement, donc les clients ne peuvent pas choisir le
fruit ni même voir quel fruit ils recevront.


<Listing number="7-9" file-name="src/lib.rs" caption="Une struct avec certains champs publics et certains champs privés">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

<!--
Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can't use the `seasonal_fruit` field in
`eat_at_restaurant`, because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!
-->

Puisque le champ `toast` de la struct `back_of_house::Breakfast` est public,
dans `eat_at_restaurant` nous pouvons écrire et lire le champ `toast` en
utilisant la notation avec point. Remarquez que nous ne pouvons pas utiliser le
champ `seasonal_fruit` dans `eat_at_restaurant`, car `seasonal_fruit` est privé.
Essayez de décommenter la ligne modifiant la valeur du champ `seasonal_fruit`
pour voir quelle erreur vous obtenez !

<!--
Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we've named it `summer` here). If `Breakfast` didn't
have such a function, we couldn't create an instance of `Breakfast` in
`eat_at_restaurant`, because we couldn't set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.
-->

Notez également que, puisque `back_of_house::Breakfast` a un champ privé, la
struct doit fournir une fonction associée publique qui construit une instance de
`Breakfast` (nous l'avons nommée `summer` ici). Si `Breakfast` n'avait pas une
telle fonction, nous ne pourrions pas créer une instance de `Breakfast` dans
`eat_at_restaurant`, car nous ne pourrions pas définir la valeur du champ privé
`seasonal_fruit` dans `eat_at_restaurant`.

<!--
In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.
-->

En revanche, si nous rendons une enum publique, tous ses variants sont alors
publics. Nous n'avons besoin du `pub` que devant le mot-clé `enum`, comme montré
dans le listing 7-10.


<Listing number="7-10" file-name="src/lib.rs" caption="Désigner une enum comme publique rend tous ses variants publics.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

<!--
Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`.
-->

Puisque nous avons rendu l'enum `Appetizer` publique, nous pouvons utiliser les
variants `Soup` et `Salad` dans `eat_at_restaurant`.

<!--
Enums aren't very useful unless their variants are public; it would be annoying
to have to annotate all enum variants with `pub` in every case, so the default
for enum variants is to be public. Structs are often useful without their
fields being public, so struct fields follow the general rule of everything
being private by default unless annotated with `pub`.
-->

Les enums ne sont pas très utiles à moins que leurs variants soient publics ; il
serait pénible de devoir annoter tous les variants d'enum avec `pub` dans chaque
cas, donc le comportement par défaut pour les variants d'enum est d'être
publics. Les structs sont souvent utiles sans que leurs champs soient publics,
donc les champs de struct suivent la règle générale selon laquelle tout est
privé par défaut sauf annotation avec `pub`.

<!--
There's one more situation involving `pub` that we haven't covered, and that is
our last module system feature: the `use` keyword. We'll cover `use` by itself
first, and then we'll show how to combine `pub` and `use`.
-->

Il y a une dernière situation impliquant `pub` que nous n'avons pas couverte, et
c'est notre dernière fonctionnalité du système de modules : le mot-clé `use`.
Nous couvrirons d'abord `use` seul, puis nous montrerons comment combiner `pub`
et `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
