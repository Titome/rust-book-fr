<!--
## Separating Modules into Different Files
-->

## Séparer les modules dans différents fichiers

<!--
So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate
file to make the code easier to navigate.
-->

Jusqu'à présent, tous les exemples de ce chapitre définissaient plusieurs
modules dans un seul fichier. Lorsque les modules deviennent volumineux, vous
pourriez vouloir déplacer leurs définitions dans un fichier séparé pour rendre
le code plus facile à naviguer.

<!--
For example, let's start from the code in Listing 7-17 that had multiple
restaurant modules. We'll extract modules into files instead of having all the
modules defined in the crate root file. In this case, the crate root file is
_src/lib.rs_, but this procedure also works with binary crates whose crate root
file is _src/main.rs_.
-->

Par exemple, partons du code du listing 7-17 qui avait plusieurs modules de
restaurant. Nous allons extraire les modules dans des fichiers au lieu d'avoir
tous les modules définis dans le fichier racine de la crate. Dans ce cas, le
fichier racine de la crate est *src/lib.rs*, mais cette procédure fonctionne
aussi avec les crates binaires dont le fichier racine est *src/main.rs*.

<!--
First, we'll extract the `front_of_house` module to its own file. Remove the
code inside the curly brackets for the `front_of_house` module, leaving only
the `mod front_of_house;` declaration, so that _src/lib.rs_ contains the code
shown in Listing 7-21. Note that this won't compile until we create the
_src/front_of_house.rs_ file in Listing 7-22.
-->

Tout d'abord, nous allons extraire le module `front_of_house` dans son propre
fichier. Supprimez le code à l'intérieur des accolades du module
`front_of_house`, en ne laissant que la déclaration `mod front_of_house;`, de
sorte que *src/lib.rs* contienne le code montré dans le listing 7-21. Notez que
cela ne compilera pas tant que nous n'aurons pas créé le fichier
*src/front_of_house.rs* dans le listing 7-22.


<Listing number="7-21" file-name="src/lib.rs" caption="Déclarer le module `front_of_house` dont le corps sera dans *src/front_of_house.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

<!--
Next, place the code that was in the curly brackets into a new file named
_src/front_of_house.rs_, as shown in Listing 7-22. The compiler knows to look
in this file because it came across the module declaration in the crate root
with the name `front_of_house`.
-->

Ensuite, placez le code qui se trouvait entre les accolades dans un nouveau
fichier nommé *src/front_of_house.rs*, comme montré dans le listing 7-22. Le
compilateur sait qu'il doit chercher dans ce fichier car il a rencontré la
déclaration de module dans la racine de la crate avec le nom `front_of_house`.


<Listing number="7-22" file-name="src/front_of_house.rs" caption="Définitions à l'intérieur du module `front_of_house` dans *src/front_of_house.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

</Listing>

<!--
Note that you only need to load a file using a `mod` declaration _once_ in your
module tree. Once the compiler knows the file is part of the project (and knows
where in the module tree the code resides because of where you've put the `mod`
statement), other files in your project should refer to the loaded file's code
using a path to where it was declared, as covered in the ["Paths for Referring
to an Item in the Module Tree"][paths] ignore
--> section. In other words,
`mod` is _not_ an "include" operation that you may have seen in other
programming languages.
-->

Notez que vous n'avez besoin de charger un fichier avec une déclaration `mod`
qu'*une seule fois* dans votre arbre de modules. Une fois que le compilateur
sait que le fichier fait partie du projet (et sait où dans l'arbre de modules le
code réside grâce à l'endroit où vous avez placé l'instruction `mod`), les
autres fichiers de votre projet doivent faire référence au code du fichier
chargé en utilisant un chemin vers l'endroit où il a été déclaré, comme couvert
dans la section [« Les chemins pour faire référence à un élément dans l'arbre de
modules »][paths]<!--
ignore
-->. En d'autres termes, `mod` n'est *pas* une
opération « include » que vous avez peut-être vue dans d'autres langages de
programmation.

<!--
Next, we'll extract the `hosting` module to its own file. The process is a bit
different because `hosting` is a child module of `front_of_house`, not of the
root module. We'll place the file for `hosting` in a new directory that will be
named for its ancestors in the module tree, in this case _src/front_of_house_.
-->

Ensuite, nous allons extraire le module `hosting` dans son propre fichier. Le
processus est un peu différent car `hosting` est un module enfant de
`front_of_house`, et non du module racine. Nous placerons le fichier pour
`hosting` dans un nouveau répertoire qui sera nommé d'après ses ancêtres dans
l'arbre de modules, dans ce cas *src/front_of_house*.

<!--
To start moving `hosting`, we change _src/front_of_house.rs_ to contain only
the declaration of the `hosting` module:
-->

Pour commencer à déplacer `hosting`, nous modifions *src/front_of_house.rs*
pour qu'il ne contienne que la déclaration du module `hosting` :


<Listing file-name="src/front_of_house.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

</Listing>

<!--
Then, we create a _src/front_of_house_ directory and a _hosting.rs_ file to
contain the definitions made in the `hosting` module:
-->

Ensuite, nous créons un répertoire *src/front_of_house* et un fichier
*hosting.rs* pour contenir les définitions faites dans le module `hosting` :


<Listing file-name="src/front_of_house/hosting.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

</Listing>

<!--
If we instead put _hosting.rs_ in the _src_ directory, the compiler would
expect the _hosting.rs_ code to be in a `hosting` module declared in the crate
root and not declared as a child of the `front_of_house` module. The
compiler's rules for which files to check for which modules' code mean the
directories and files more closely match the module tree.
-->

Si au contraire nous placions *hosting.rs* dans le répertoire *src*, le
compilateur s'attendrait à ce que le code de *hosting.rs* soit dans un module
`hosting` déclaré à la racine de la crate et non déclaré comme enfant du module
`front_of_house`. Les règles du compilateur concernant les fichiers à vérifier
pour le code de chaque module font que les répertoires et les fichiers
correspondent plus étroitement à l'arbre de modules.

<!--
> ### Alternate File Paths
>
> So far we've covered the most idiomatic file paths the Rust compiler uses,
> but Rust also supports an older style of file path. For a module named
> `front_of_house` declared in the crate root, the compiler will look for the
> module's code in:
>
> - _src/front_of_house.rs_ (what we covered)
> - _src/front_of_house/mod.rs_ (older style, still supported path)
>
> For a module named `hosting` that is a submodule of `front_of_house`, the
> compiler will look for the module's code in:
>
> - _src/front_of_house/hosting.rs_ (what we covered)
> - _src/front_of_house/hosting/mod.rs_ (older style, still supported path)
>
> If you use both styles for the same module, you'll get a compiler error.
> Using a mix of both styles for different modules in the same project is
> allowed but might be confusing for people navigating your project.
>
> The main downside to the style that uses files named _mod.rs_ is that your
> project can end up with many files named _mod.rs_, which can get confusing
> when you have them open in your editor at the same time.
-->

> ### Chemins de fichiers alternatifs
>
> Jusqu'ici, nous avons couvert les chemins de fichiers les plus idiomatiques
> que le compilateur Rust utilise, mais Rust prend aussi en charge un ancien
> style de chemin de fichier. Pour un module nommé `front_of_house` déclaré à
> la racine de la crate, le compilateur cherchera le code du module dans :
>
> - *src/front_of_house.rs* (ce que nous avons couvert)
> - *src/front_of_house/mod.rs* (ancien style, chemin toujours pris en charge)
>
> Pour un module nommé `hosting` qui est un sous-module de `front_of_house`, le
> compilateur cherchera le code du module dans :
>
> - *src/front_of_house/hosting.rs* (ce que nous avons couvert)
> - *src/front_of_house/hosting/mod.rs* (ancien style, chemin toujours pris en
>   charge)
>
> Si vous utilisez les deux styles pour le même module, vous obtiendrez une
> erreur de compilation. Utiliser un mélange des deux styles pour différents
> modules dans le même projet est autorisé mais pourrait être déroutant pour les
> personnes naviguant dans votre projet.
>
> Le principal inconvénient du style utilisant des fichiers nommés *mod.rs* est
> que votre projet peut se retrouver avec de nombreux fichiers nommés *mod.rs*,
> ce qui peut prêter à confusion lorsque vous les avez ouverts en même temps
> dans votre éditeur.

<!--
We've moved each module's code to a separate file, and the module tree remains
the same. The function calls in `eat_at_restaurant` will work without any
modification, even though the definitions live in different files. This
technique lets you move modules to new files as they grow in size.
-->

Nous avons déplacé le code de chaque module dans un fichier séparé, et l'arbre
de modules reste le même. Les appels de fonction dans `eat_at_restaurant`
fonctionneront sans aucune modification, même si les définitions se trouvent
dans des fichiers différents. Cette technique vous permet de déplacer les modules
vers de nouveaux fichiers à mesure qu'ils grandissent en taille.

<!--
Note that the `pub use crate::front_of_house::hosting` statement in
_src/lib.rs_ also hasn't changed, nor does `use` have any impact on what files
are compiled as part of the crate. The `mod` keyword declares modules, and Rust
looks in a file with the same name as the module for the code that goes into
that module.
-->

Notez que l'instruction `pub use crate::front_of_house::hosting` dans
*src/lib.rs* n'a pas changé non plus, et `use` n'a aucun impact sur les fichiers
qui sont compilés en tant que partie de la crate. Le mot-clé `mod` déclare les
modules, et Rust cherche dans un fichier portant le même nom que le module le
code qui va dans ce module.

<!--
## Summary
-->

## Résumé

<!--
Rust lets you split a package into multiple crates and a crate into modules so
that you can refer to items defined in one module from another module. You can
do this by specifying absolute or relative paths. These paths can be brought
into scope with a `use` statement so that you can use a shorter path for
multiple uses of the item in that scope. Module code is private by default, but
you can make definitions public by adding the `pub` keyword.
-->

Rust vous permet de diviser un package en plusieurs crates et une crate en
modules afin que vous puissiez faire référence à des éléments définis dans un
module depuis un autre module. Vous pouvez le faire en spécifiant des chemins
absolus ou relatifs. Ces chemins peuvent être amenés dans la portée avec une
instruction `use` afin que vous puissiez utiliser un chemin plus court pour
plusieurs utilisations de l'élément dans cette portée. Le code des modules est
privé par défaut, mais vous pouvez rendre les définitions publiques en ajoutant
le mot-clé `pub`.

<!--
In the next chapter, we'll look at some collection data structures in the
standard library that you can use in your neatly organized code.
-->

Dans le prochain chapitre, nous examinerons quelques structures de données de
collections dans la bibliothèque standard que vous pouvez utiliser dans votre
code bien organisé.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
