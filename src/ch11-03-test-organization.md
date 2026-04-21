<!--
## Test Organization
-->

## Organisation des tests

<!--
As mentioned at the start of the chapter, testing is a complex discipline, and
different people use different terminology and organization. The Rust community
thinks about tests in terms of two main categories: unit tests and integration
tests. _Unit tests_ are small and more focused, testing one module in isolation
at a time, and can test private interfaces. _Integration tests_ are entirely
external to your library and use your code in the same way any other external
code would, using only the public interface and potentially exercising multiple
modules per test.
-->

Comme mentionne au debut du chapitre, les tests sont une discipline complexe, et differentes personnes utilisent une terminologie et une organisation differentes. La communaute Rust pense aux tests en termes de deux categories principales : les tests unitaires et les tests d'integration. Les _tests unitaires_ sont petits et plus cibles, testant un module a la fois de maniere isolee, et peuvent tester les interfaces privees. Les _tests d'integration_ sont entierement externes a votre bibliotheque et utilisent votre code de la meme facon que n'importe quel autre code externe le ferait, en n'utilisant que l'interface publique et en exercant potentiellement plusieurs modules par test.

<!--
Writing both kinds of tests is important to ensure that the pieces of your
library are doing what you expect them to, separately and together.
-->

Ecrire les deux types de tests est important pour s'assurer que les composants de votre bibliotheque font ce que vous attendez d'eux, separement et ensemble.

<!--
### Unit Tests
-->

### Tests unitaires

<!--
The purpose of unit tests is to test each unit of code in isolation from the
rest of the code to quickly pinpoint where code is and isn't working as
expected. You'll put unit tests in the _src_ directory in each file with the
code that they're testing. The convention is to create a module named `tests`
in each file to contain the test functions and to annotate the module with
`cfg(test)`.
-->

L'objectif des tests unitaires est de tester chaque unite de code isolement du reste du code pour identifier rapidement ou le code fonctionne ou ne fonctionne pas comme prevu. Vous placerez les tests unitaires dans le repertoire _src_ dans chaque fichier avec le code qu'ils testent. La convention est de creer un module nomme `tests` dans chaque fichier pour contenir les fonctions de test et d'annoter le module avec `cfg(test)`.

<!--
#### The `tests` Module and `#[cfg(test)]`
-->

#### Le module `tests` et `#[cfg(test)]`

<!--
The `#[cfg(test)]` annotation on the `tests` module tells Rust to compile and
run the test code only when you run `cargo test`, not when you run `cargo
build`. This saves compile time when you only want to build the library and
saves space in the resultant compiled artifact because the tests are not
included. You'll see that because integration tests go in a different
directory, they don't need the `#[cfg(test)]` annotation. However, because unit
tests go in the same files as the code, you'll use `#[cfg(test)]` to specify
that they shouldn't be included in the compiled result.
-->

L'annotation `#[cfg(test)]` sur le module `tests` indique a Rust de compiler et d'executer le code de test uniquement quand vous executez `cargo test`, et non quand vous executez `cargo build`. Cela economise du temps de compilation quand vous voulez seulement compiler la bibliotheque et economise de l'espace dans l'artefact compile resultant parce que les tests ne sont pas inclus. Vous verrez que comme les tests d'integration sont dans un repertoire different, ils n'ont pas besoin de l'annotation `#[cfg(test)]`. Cependant, comme les tests unitaires sont dans les memes fichiers que le code, vous utiliserez `#[cfg(test)]` pour specifier qu'ils ne doivent pas etre inclus dans le resultat compile.

<!--
Recall that when we generated the new `adder` project in the first section of
this chapter, Cargo generated this code for us:
-->

Rappelez-vous que lorsque nous avons genere le nouveau projet `adder` dans la premiere section de ce chapitre, Cargo a genere ce code pour nous :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<!--
On the automatically generated `tests` module, the attribute `cfg` stands for
_configuration_ and tells Rust that the following item should only be included
given a certain configuration option. In this case, the configuration option is
`test`, which is provided by Rust for compiling and running tests. By using the
`cfg` attribute, Cargo compiles our test code only if we actively run the tests
with `cargo test`. This includes any helper functions that might be within this
module, in addition to the functions annotated with `#[test]`.
-->

Sur le module `tests` genere automatiquement, l'attribut `cfg` signifie _configuration_ et indique a Rust que l'element suivant ne doit etre inclus que pour une certaine option de configuration. Dans ce cas, l'option de configuration est `test`, qui est fournie par Rust pour la compilation et l'execution des tests. En utilisant l'attribut `cfg`, Cargo ne compile notre code de test que si nous executons activement les tests avec `cargo test`. Cela inclut toutes les fonctions utilitaires qui pourraient se trouver dans ce module, en plus des fonctions annotees avec `#[test]`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="testing-private-functions"></a>

<!--
#### Private Function Tests
-->

#### Tests de fonctions privees

<!--
There's debate within the testing community about whether or not private
functions should be tested directly, and other languages make it difficult or
impossible to test private functions. Regardless of which testing ideology you
adhere to, Rust's privacy rules do allow you to test private functions.
Consider the code in Listing 11-12 with the private function `internal_adder`.
-->

Il y a un debat au sein de la communaute des testeurs pour savoir si les fonctions privees doivent etre testees directement ou non, et d'autres langages rendent difficile ou impossible le test des fonctions privees. Quelle que soit l'ideologie de test a laquelle vous adherez, les regles de confidentialite de Rust vous permettent de tester les fonctions privees. Considerez le code de l'encart 11-12 avec la fonction privee `internal_adder`.

<Listing number="11-12" file-name="src/lib.rs" caption="Tester une fonction privee">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

<!--
Note that the `internal_adder` function is not marked as `pub`. Tests are just
Rust code, and the `tests` module is just another module. As we discussed in
["Paths for Referring to an Item in the Module Tree"][paths] ignore
-->,
items in child modules can use the items in their ancestor modules. In this
test, we bring all of the items belonging to the `tests` module's parent into
scope with `use super::*`, and then the test can call `internal_adder`. If you
don't think private functions should be tested, there's nothing in Rust that
will compel you to do so.
-->

Notez que la fonction `internal_adder` n'est pas marquee comme `pub`. Les tests ne sont que du code Rust, et le module `tests` n'est qu'un autre module. Comme nous l'avons vu dans ["Les chemins pour faire reference a un element dans l'arborescence des modules"][paths]<!--
ignore
-->, les elements des modules enfants peuvent utiliser les elements de leurs modules ancetres. Dans ce test, nous importons tous les elements appartenant au parent du module `tests` dans la portee avec `use super::*`, et le test peut alors appeler `internal_adder`. Si vous pensez que les fonctions privees ne devraient pas etre testees, rien dans Rust ne vous obligera a le faire.

<!--
### Integration Tests
-->

### Tests d'integration

<!--
In Rust, integration tests are entirely external to your library. They use your
library in the same way any other code would, which means they can only call
functions that are part of your library's public API. Their purpose is to test
whether many parts of your library work together correctly. Units of code that
work correctly on their own could have problems when integrated, so test
coverage of the integrated code is important as well. To create integration
tests, you first need a _tests_ directory.
-->

En Rust, les tests d'integration sont entierement externes a votre bibliotheque. Ils utilisent votre bibliotheque de la meme facon que n'importe quel autre code le ferait, ce qui signifie qu'ils ne peuvent appeler que les fonctions faisant partie de l'API publique de votre bibliotheque. Leur objectif est de tester si de nombreuses parties de votre bibliotheque fonctionnent correctement ensemble. Des unites de code qui fonctionnent correctement individuellement pourraient avoir des problemes une fois integrees, donc la couverture de test du code integre est egalement importante. Pour creer des tests d'integration, vous avez d'abord besoin d'un repertoire _tests_.

<!--
#### The _tests_ Directory
-->

#### Le repertoire _tests_

<!--
We create a _tests_ directory at the top level of our project directory, next
to _src_. Cargo knows to look for integration test files in this directory. We
can then make as many test files as we want, and Cargo will compile each of the
files as an individual crate.
-->

Nous creons un repertoire _tests_ au niveau superieur de notre repertoire de projet, a cote de _src_. Cargo sait qu'il doit chercher les fichiers de tests d'integration dans ce repertoire. Nous pouvons alors creer autant de fichiers de test que nous le souhaitons, et Cargo compilera chacun des fichiers comme un crate individuel.

<!--
Let's create an integration test. With the code in Listing 11-12 still in the
_src/lib.rs_ file, make a _tests_ directory, and create a new file named
_tests/integration_test.rs_. Your directory structure should look like this:
-->

Creons un test d'integration. Avec le code de l'encart 11-12 toujours dans le fichier _src/lib.rs_, creez un repertoire _tests_ et creez un nouveau fichier nomme _tests/integration_test.rs_. Votre structure de repertoire devrait ressembler a ceci :

<!--
```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
-->

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

<!--
Enter the code in Listing 11-13 into the _tests/integration_test.rs_ file.
-->

Entrez le code de l'encart 11-13 dans le fichier _tests/integration_test.rs_.

<Listing number="11-13" file-name="tests/integration_test.rs" caption="Un test d'integration d'une fonction du crate `adder`">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

</Listing>

<!--
Each file in the _tests_ directory is a separate crate, so we need to bring our
library into each test crate's scope. For that reason, we add `use
adder::add_two;` at the top of the code, which we didn't need in the unit tests.
-->

Chaque fichier dans le repertoire _tests_ est un crate separe, nous devons donc importer notre bibliotheque dans la portee de chaque crate de test. C'est pourquoi nous ajoutons `use adder::add_two;` en haut du code, ce que nous n'avions pas besoin de faire dans les tests unitaires.

<!--
We don't need to annotate any code in _tests/integration_test.rs_ with
`#[cfg(test)]`. Cargo treats the _tests_ directory specially and compiles files
in this directory only when we run `cargo test`. Run `cargo test` now:
-->

Nous n'avons pas besoin d'annoter le code dans _tests/integration_test.rs_ avec `#[cfg(test)]`. Cargo traite le repertoire _tests_ de maniere speciale et ne compile les fichiers de ce repertoire que lorsque nous executons `cargo test`. Executez `cargo test` maintenant :

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

<!--
The three sections of output include the unit tests, the integration test, and
the doc tests. Note that if any test in a section fails, the following sections
will not be run. For example, if a unit test fails, there won't be any output
for integration and doc tests, because those tests will only be run if all unit
tests are passing.
-->

Les trois sections de sortie comprennent les tests unitaires, le test d'integration et les tests de documentation. Notez que si un test d'une section echoue, les sections suivantes ne seront pas executees. Par exemple, si un test unitaire echoue, il n'y aura pas de sortie pour les tests d'integration et de documentation, car ces tests ne seront executes que si tous les tests unitaires reussissent.

<!--
The first section for the unit tests is the same as we've been seeing: one line
for each unit test (one named `internal` that we added in Listing 11-12) and
then a summary line for the unit tests.
-->

La premiere section pour les tests unitaires est la meme que ce que nous avons vu : une ligne pour chaque test unitaire (un nomme `internal` que nous avons ajoute dans l'encart 11-12) puis une ligne de resume pour les tests unitaires.

<!--
The integration tests section starts with the line `Running
tests/integration_test.rs`. Next, there is a line for each test function in
that integration test and a summary line for the results of the integration
test just before the `Doc-tests adder` section starts.
-->

La section des tests d'integration commence par la ligne `Running tests/integration_test.rs`. Ensuite, il y a une ligne pour chaque fonction de test dans ce test d'integration et une ligne de resume pour les resultats du test d'integration juste avant le debut de la section `Doc-tests adder`.

<!--
Each integration test file has its own section, so if we add more files in the
_tests_ directory, there will be more integration test sections.
-->

Chaque fichier de test d'integration a sa propre section, donc si nous ajoutons plus de fichiers dans le repertoire _tests_, il y aura plus de sections de tests d'integration.

<!--
We can still run a particular integration test function by specifying the test
function's name as an argument to `cargo test`. To run all the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file:
-->

Nous pouvons toujours executer une fonction de test d'integration particuliere en specifiant le nom de la fonction de test comme argument de `cargo test`. Pour executer tous les tests d'un fichier de test d'integration particulier, utilisez l'argument `--test` de `cargo test` suivi du nom du fichier :

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

<!--
This command runs only the tests in the _tests/integration_test.rs_ file.
-->

Cette commande n'execute que les tests du fichier _tests/integration_test.rs_.

<!--
#### Submodules in Integration Tests
-->

#### Sous-modules dans les tests d'integration

<!--
As you add more integration tests, you might want to make more files in the
_tests_ directory to help organize them; for example, you can group the test
functions by the functionality they're testing. As mentioned earlier, each file
in the _tests_ directory is compiled as its own separate crate, which is useful
for creating separate scopes to more closely imitate the way end users will be
using your crate. However, this means files in the _tests_ directory don't
share the same behavior as files in _src_ do, as you learned in Chapter 7
regarding how to separate code into modules and files.
-->

Au fur et a mesure que vous ajoutez des tests d'integration, vous pourriez vouloir creer plus de fichiers dans le repertoire _tests_ pour les organiser ; par exemple, vous pouvez regrouper les fonctions de test par la fonctionnalite qu'elles testent. Comme mentionne precedemment, chaque fichier dans le repertoire _tests_ est compile comme son propre crate separe, ce qui est utile pour creer des portees separees afin d'imiter plus fidèlement la facon dont les utilisateurs finaux utiliseront votre crate. Cependant, cela signifie que les fichiers dans le repertoire _tests_ ne partagent pas le meme comportement que les fichiers dans _src_, comme vous l'avez appris au chapitre 7 concernant la separation du code en modules et fichiers.

<!--
The different behavior of _tests_ directory files is most noticeable when you
have a set of helper functions to use in multiple integration test files, and
you try to follow the steps in the ["Separating Modules into Different
Files"][separating-modules-into-files] ignore
--> section of Chapter 7 to
extract them into a common module. For example, if we create _tests/common.rs_
and place a function named `setup` in it, we can add some code to `setup` that
we want to call from multiple test functions in multiple test files:
-->

Le comportement different des fichiers du repertoire _tests_ est plus visible quand vous avez un ensemble de fonctions utilitaires a utiliser dans plusieurs fichiers de tests d'integration, et que vous essayez de suivre les etapes de la section ["Separer les modules dans differents fichiers"][separating-modules-into-files]<!--
ignore
--> du chapitre 7 pour les extraire dans un module commun. Par exemple, si nous creons _tests/common.rs_ et y placons une fonction nommee `setup`, nous pouvons ajouter du code a `setup` que nous voulons appeler depuis plusieurs fonctions de test dans plusieurs fichiers de test :

<!--
<span class="filename">Filename: tests/common.rs</span>
-->

<span class="filename">Fichier : tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

<!--
When we run the tests again, we'll see a new section in the test output for the
_common.rs_ file, even though this file doesn't contain any test functions nor
did we call the `setup` function from anywhere:
-->

Quand nous executons les tests a nouveau, nous verrons une nouvelle section dans la sortie des tests pour le fichier _common.rs_, meme si ce fichier ne contient aucune fonction de test et que nous n'avons appele la fonction `setup` depuis nulle part :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

<!--
Having `common` appear in the test results with `running 0 tests` displayed for
it is not what we wanted. We just wanted to share some code with the other
integration test files. To avoid having `common` appear in the test output,
instead of creating _tests/common.rs_, we'll create _tests/common/mod.rs_. The
project directory now looks like this:
-->

Voir `common` apparaitre dans les resultats de test avec `running 0 tests` affiche n'est pas ce que nous voulions. Nous voulions simplement partager du code avec les autres fichiers de tests d'integration. Pour eviter que `common` apparaisse dans la sortie des tests, au lieu de creer _tests/common.rs_, nous allons creer _tests/common/mod.rs_. Le repertoire du projet ressemble maintenant a ceci :

<!--
```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
-->

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

<!--
This is the older naming convention that Rust also understands that we mentioned
in ["Alternate File Paths"][alt-paths] ignore
--> in Chapter 7. Naming the
file this way tells Rust not to treat the `common` module as an integration test
file. When we move the `setup` function code into _tests/common/mod.rs_ and
delete the _tests/common.rs_ file, the section in the test output will no longer
appear. Files in subdirectories of the _tests_ directory don't get compiled as
separate crates or have sections in the test output.
-->

C'est l'ancienne convention de nommage que Rust comprend egalement et que nous avons mentionnee dans ["Chemins de fichiers alternatifs"][alt-paths]<!--
ignore
--> au chapitre 7. Nommer le fichier de cette facon indique a Rust de ne pas traiter le module `common` comme un fichier de test d'integration. Quand nous deplacons le code de la fonction `setup` dans _tests/common/mod.rs_ et supprimons le fichier _tests/common.rs_, la section dans la sortie des tests n'apparaitra plus. Les fichiers dans les sous-repertoires du repertoire _tests_ ne sont pas compiles comme des crates separes et n'ont pas de sections dans la sortie des tests.

<!--
After we've created _tests/common/mod.rs_, we can use it from any of the
integration test files as a module. Here's an example of calling the `setup`
function from the `it_adds_two` test in _tests/integration_test.rs_:
-->

Apres avoir cree _tests/common/mod.rs_, nous pouvons l'utiliser depuis n'importe quel fichier de test d'integration en tant que module. Voici un exemple d'appel de la fonction `setup` depuis le test `it_adds_two` dans _tests/integration_test.rs_ :

<!--
<span class="filename">Filename: tests/integration_test.rs</span>
-->

<span class="filename">Fichier : tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

<!--
Note that the `mod common;` declaration is the same as the module declaration
we demonstrated in Listing 7-21. Then, in the test function, we can call the
`common::setup()` function.
-->

Notez que la declaration `mod common;` est la meme que la declaration de module que nous avons montree dans l'encart 7-21. Ensuite, dans la fonction de test, nous pouvons appeler la fonction `common::setup()`.

<!--
#### Integration Tests for Binary Crates
-->

#### Tests d'integration pour les crates binaires

<!--
If our project is a binary crate that only contains a _src/main.rs_ file and
doesn't have a _src/lib.rs_ file, we can't create integration tests in the
_tests_ directory and bring functions defined in the _src/main.rs_ file into
scope with a `use` statement. Only library crates expose functions that other
crates can use; binary crates are meant to be run on their own.
-->

Si notre projet est un crate binaire qui ne contient qu'un fichier _src/main.rs_ et n'a pas de fichier _src/lib.rs_, nous ne pouvons pas creer de tests d'integration dans le repertoire _tests_ et importer les fonctions definies dans le fichier _src/main.rs_ dans la portee avec une instruction `use`. Seuls les crates de bibliotheque exposent des fonctions que d'autres crates peuvent utiliser ; les crates binaires sont destines a etre executes de maniere autonome.

<!--
This is one of the reasons Rust projects that provide a binary have a
straightforward _src/main.rs_ file that calls logic that lives in the
_src/lib.rs_ file. Using that structure, integration tests _can_ test the
library crate with `use` to make the important functionality available. If the
important functionality works, the small amount of code in the _src/main.rs_
file will work as well, and that small amount of code doesn't need to be tested.
-->

C'est l'une des raisons pour lesquelles les projets Rust qui fournissent un binaire ont un fichier _src/main.rs_ simple qui appelle la logique qui reside dans le fichier _src/lib.rs_. Avec cette structure, les tests d'integration _peuvent_ tester le crate de bibliotheque avec `use` pour rendre la fonctionnalite importante disponible. Si la fonctionnalite importante fonctionne, la petite quantite de code dans le fichier _src/main.rs_ fonctionnera egalement, et cette petite quantite de code n'a pas besoin d'etre testee.

<!--
## Summary
-->

## Resume

<!--
Rust's testing features provide a way to specify how code should function to
ensure that it continues to work as you expect, even as you make changes. Unit
tests exercise different parts of a library separately and can test private
implementation details. Integration tests check that many parts of the library
work together correctly, and they use the library's public API to test the code
in the same way external code will use it. Even though Rust's type system and
ownership rules help prevent some kinds of bugs, tests are still important to
reduce logic bugs having to do with how your code is expected to behave.
-->

Les fonctionnalites de test de Rust fournissent un moyen de specifier comment le code devrait fonctionner pour s'assurer qu'il continue a fonctionner comme vous l'attendez, meme lorsque vous apportez des modifications. Les tests unitaires exercent differentes parties d'une bibliotheque separement et peuvent tester les details d'implementation prives. Les tests d'integration verifient que de nombreuses parties de la bibliotheque fonctionnent correctement ensemble, et ils utilisent l'API publique de la bibliotheque pour tester le code de la meme maniere que le code externe l'utilisera. Meme si le systeme de types et les regles de possession de Rust aident a prevenir certains types de bogues, les tests restent importants pour reduire les bogues logiques lies a la facon dont votre code est cense se comporter.

<!--
Let's combine the knowledge you learned in this chapter and in previous
chapters to work on a project!
-->

Combinons les connaissances que vous avez acquises dans ce chapitre et dans les chapitres precedents pour travailler sur un projet !

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#alternate-file-paths
