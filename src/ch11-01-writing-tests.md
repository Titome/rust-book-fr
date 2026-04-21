<!--
## How to Write Tests
-->

## Comment ecrire des tests

<!--
_Tests_ are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform these three
actions:
-->

Les _tests_ sont des fonctions Rust qui verifient que le code non-test fonctionne de la maniere attendue. Le corps des fonctions de test effectue generalement ces trois actions :

<!--
- Set up any needed data or state.
- Run the code you want to test.
- Assert that the results are what you expect.
-->

- Mettre en place les donnees ou l'etat necessaires.
- Executer le code que vous souhaitez tester.
- Verifier que les resultats sont ceux que vous attendez.

<!--
Let's look at the features Rust provides specifically for writing tests that
take these actions, which include the `test` attribute, a few macros, and the
`should_panic` attribute.
-->

Examinons les fonctionnalites que Rust fournit specifiquement pour ecrire des tests qui effectuent ces actions, notamment l'attribut `test`, quelques macros et l'attribut `should_panic`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="the-anatomy-of-a-test-function"></a>

<!--
### Structuring Test Functions
-->

### Structurer les fonctions de test

<!--
At its simplest, a test in Rust is a function that's annotated with the `test`
attribute. Attributes are metadata about pieces of Rust code; one example is
the `derive` attribute we used with structs in Chapter 5. To change a function
into a test function, add `#[test]` on the line before `fn`. When you run your
tests with the `cargo test` command, Rust builds a test runner binary that runs
the annotated functions and reports on whether each test function passes or
fails.
-->

Dans sa forme la plus simple, un test en Rust est une fonction annotee avec l'attribut `test`. Les attributs sont des metadonnees sur des portions de code Rust ; un exemple est l'attribut `derive` que nous avons utilise avec les structures au chapitre 5. Pour transformer une fonction en fonction de test, ajoutez `#[test]` sur la ligne avant `fn`. Quand vous executez vos tests avec la commande `cargo test`, Rust compile un binaire d'execution de tests qui lance les fonctions annotees et rapporte si chaque fonction de test reussit ou echoue.

<!--
Whenever we make a new library project with Cargo, a test module with a test
function in it is automatically generated for us. This module gives you a
template for writing your tests so that you don't have to look up the exact
structure and syntax every time you start a new project. You can add as many
additional test functions and as many test modules as you want!
-->

Chaque fois que nous creons un nouveau projet de bibliotheque avec Cargo, un module de test contenant une fonction de test est automatiquement genere pour nous. Ce module vous fournit un modele pour ecrire vos tests afin que vous n'ayez pas a rechercher la structure et la syntaxe exactes chaque fois que vous demarrez un nouveau projet. Vous pouvez ajouter autant de fonctions de test et de modules de test supplementaires que vous le souhaitez !

<!--
We'll explore some aspects of how tests work by experimenting with the template
test before we actually test any code. Then, we'll write some real-world tests
that call some code that we've written and assert that its behavior is correct.
-->

Nous allons explorer certains aspects du fonctionnement des tests en experimentant avec le test modele avant de tester reellement du code. Ensuite, nous ecrirons des tests concrets qui appellent du code que nous avons ecrit et verifient que son comportement est correct.

<!--
Let's create a new library project called `adder` that will add two numbers:
-->

Creons un nouveau projet de bibliotheque appele `adder` qui additionnera deux nombres :

<!--
```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```
-->

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

<!--
The contents of the _src/lib.rs_ file in your `adder` library should look like
Listing 11-1.
-->

Le contenu du fichier _src/lib.rs_ de votre bibliotheque `adder` devrait ressembler a l'encart 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="Le code genere automatiquement par `cargo new`">

<!--
manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

<!--
The file starts with an example `add` function so that we have something to
test.
-->

Le fichier commence par un exemple de fonction `add` afin que nous ayons quelque chose a tester.

<!--
For now, let's focus solely on the `it_works` function. Note the `#[test]`
annotation: This attribute indicates this is a test function, so the test
runner knows to treat this function as a test. We might also have non-test
functions in the `tests` module to help set up common scenarios or perform
common operations, so we always need to indicate which functions are tests.
-->

Pour l'instant, concentrons-nous uniquement sur la fonction `it_works`. Notez l'annotation `#[test]` : cet attribut indique qu'il s'agit d'une fonction de test, de sorte que l'executeur de tests sait qu'il doit traiter cette fonction comme un test. Nous pourrions aussi avoir des fonctions non-test dans le module `tests` pour aider a mettre en place des scenarios communs ou effectuer des operations courantes, c'est pourquoi nous devons toujours indiquer quelles fonctions sont des tests.

<!--
The example function body uses the `assert_eq!` macro to assert that `result`,
which contains the result of calling `add` with 2 and 2, equals 4. This
assertion serves as an example of the format for a typical test. Let's run it
to see that this test passes.
-->

Le corps de la fonction d'exemple utilise la macro `assert_eq!` pour verifier que `result`, qui contient le resultat de l'appel de `add` avec 2 et 2, est egal a 4. Cette assertion sert d'exemple de format pour un test typique. Executons-le pour voir que ce test reussit.

<!--
The `cargo test` command runs all tests in our project, as shown in Listing
11-2.
-->

La commande `cargo test` execute tous les tests de notre projet, comme montre dans l'encart 11-2.

<Listing number="11-2" caption="La sortie de l'execution du test genere automatiquement">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

<!--
Cargo compiled and ran the test. We see the line `running 1 test`. The next
line shows the name of the generated test function, called `tests::it_works`,
and that the result of running that test is `ok`. The overall summary `test
result: ok.` means that all the tests passed, and the portion that reads `1
passed; 0 failed` totals the number of tests that passed or failed.
-->

Cargo a compile et execute le test. Nous voyons la ligne `running 1 test`. La ligne suivante montre le nom de la fonction de test generee, appelee `tests::it_works`, et que le resultat de l'execution de ce test est `ok`. Le resume global `test result: ok.` signifie que tous les tests ont reussi, et la partie qui indique `1 passed; 0 failed` totalise le nombre de tests qui ont reussi ou echoue.

<!--
It's possible to mark a test as ignored so that it doesn't run in a particular
instance; we'll cover that in the ["Ignoring Tests Unless Specifically
Requested"][ignoring] ignore
--> section later in this chapter. Because we
haven't done that here, the summary shows `0 ignored`. We can also pass an
argument to the `cargo test` command to run only tests whose name matches a
string; this is called _filtering_, and we'll cover it in the ["Running a
Subset of Tests by Name"][subset]<!--
ignore
--> section. Here, we haven't
filtered the tests being run, so the end of the summary shows `0 filtered out`.
-->

Il est possible de marquer un test comme ignore afin qu'il ne s'execute pas dans un cas particulier ; nous aborderons cela dans la section ["Ignorer des tests sauf demande explicite"][ignoring]<!--
ignore
--> plus loin dans ce chapitre. Comme nous ne l'avons pas fait ici, le resume affiche `0 ignored`. Nous pouvons egalement passer un argument a la commande `cargo test` pour n'executer que les tests dont le nom correspond a une chaine de caracteres ; c'est ce qu'on appelle le _filtrage_, et nous l'aborderons dans la section ["Executer un sous-ensemble de tests par nom"][subset]<!--
ignore
-->. Ici, nous n'avons pas filtre les tests executes, donc la fin du resume affiche `0 filtered out`.

<!--
The `0 measured` statistic is for benchmark tests that measure performance.
Benchmark tests are, as of this writing, only available in nightly Rust. See
[the documentation about benchmark tests][bench] to learn more.
-->

La statistique `0 measured` concerne les tests de performance (benchmarks) qui mesurent les performances. Les tests de performance ne sont, au moment de la redaction de ce livre, disponibles que dans la version nightly de Rust. Consultez [la documentation sur les tests de performance][bench] pour en savoir plus.

<!--
The next part of the test output starting at `Doc-tests adder` is for the
results of any documentation tests. We don't have any documentation tests yet,
but Rust can compile any code examples that appear in our API documentation.
This feature helps keep your docs and your code in sync! We'll discuss how to
write documentation tests in the ["Documentation Comments as
Tests"][doc-comments] ignore
--> section of Chapter 14. For now, we'll
ignore the `Doc-tests` output.
-->

La partie suivante de la sortie des tests, commencant par `Doc-tests adder`, concerne les resultats des tests de documentation. Nous n'avons pas encore de tests de documentation, mais Rust peut compiler tous les exemples de code qui apparaissent dans notre documentation d'API. Cette fonctionnalite aide a garder votre documentation et votre code synchronises ! Nous verrons comment ecrire des tests de documentation dans la section ["Les commentaires de documentation comme tests"][doc-comments]<!--
ignore
--> du chapitre 14. Pour l'instant, nous ignorerons la sortie `Doc-tests`.

<!--
Let's start to customize the test to our own needs. First, change the name of
the `it_works` function to a different name, such as `exploration`, like so:
-->

Commencons a personnaliser le test selon nos besoins. D'abord, changez le nom de la fonction `it_works` pour un nom different, comme `exploration`, comme ceci :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

<!--
Then, run `cargo test` again. The output now shows `exploration` instead of
`it_works`:
-->

Ensuite, executez `cargo test` a nouveau. La sortie affiche maintenant `exploration` au lieu de `it_works` :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

<!--
Now we'll add another test, but this time we'll make a test that fails! Tests
fail when something in the test function panics. Each test is run in a new
thread, and when the main thread sees that a test thread has died, the test is
marked as failed. In Chapter 9, we talked about how the simplest way to panic
is to call the `panic!` macro. Enter the new test as a function named
`another`, so your _src/lib.rs_ file looks like Listing 11-3.
-->

Maintenant, nous allons ajouter un autre test, mais cette fois nous allons creer un test qui echoue ! Les tests echouent quand quelque chose dans la fonction de test provoque un panic. Chaque test est execute dans un nouveau thread, et quand le thread principal voit qu'un thread de test est mort, le test est marque comme echoue. Au chapitre 9, nous avons parle de la facon la plus simple de provoquer un panic : appeler la macro `panic!`. Entrez le nouveau test sous forme de fonction nommee `another`, pour que votre fichier _src/lib.rs_ ressemble a l'encart 11-3.

<Listing number="11-3" file-name="src/lib.rs" caption="Ajout d'un second test qui echouera parce que nous appelons la macro `panic!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

<!--
Run the tests again using `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed.
-->

Executez les tests a nouveau avec `cargo test`. La sortie devrait ressembler a l'encart 11-4, qui montre que notre test `exploration` a reussi et que `another` a echoue.

<Listing number="11-4" caption="Resultats des tests quand un test reussit et un autre echoue">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!--
manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
-->

<!--
Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new
sections appear between the individual results and the summary: The first
displays the detailed reason for each test failure. In this case, we get the
details that `tests::another` failed because it panicked with the message `Make
this test fail` on line 17 in the _src/lib.rs_ file. The next section lists
just the names of all the failing tests, which is useful when there are lots of
tests and lots of detailed failing test output. We can use the name of a
failing test to run just that test to debug it more easily; we'll talk more
about ways to run tests in the ["Controlling How Tests Are
Run"][controlling-how-tests-are-run] ignore
--> section.
-->

Au lieu de `ok`, la ligne `test tests::another` affiche `FAILED`. Deux nouvelles sections apparaissent entre les resultats individuels et le resume : la premiere affiche la raison detaillee de chaque echec de test. Dans ce cas, nous obtenons les details indiquant que `tests::another` a echoue parce qu'il a provoque un panic avec le message `Make this test fail` a la ligne 17 du fichier _src/lib.rs_. La section suivante liste uniquement les noms de tous les tests en echec, ce qui est utile quand il y a beaucoup de tests et beaucoup de sorties detaillees de tests en echec. Nous pouvons utiliser le nom d'un test en echec pour n'executer que ce test et le deboguer plus facilement ; nous parlerons davantage des facons d'executer les tests dans la section ["Controler comment les tests sont executes"][controlling-how-tests-are-run]<!--
ignore
-->.

<!--
The summary line displays at the end: Overall, our test result is `FAILED`. We
had one test pass and one test fail.
-->

La ligne de resume s'affiche a la fin : globalement, notre resultat de test est `FAILED`. Nous avions un test reussi et un test en echec.

<!--
Now that you've seen what the test results look like in different scenarios,
let's look at some macros other than `panic!` that are useful in tests.
-->

Maintenant que vous avez vu a quoi ressemblent les resultats des tests dans differents scenarios, examinons quelques macros autres que `panic!` qui sont utiles dans les tests.

<!--
Old headings. Do not remove or links may break.
-->

<a id="checking-results-with-the-assert-macro"></a>

<!--
### Checking Results with `assert!`
-->

### Verifier les resultats avec `assert!`

<!--
The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a Boolean. If the value is
`true`, nothing happens and the test passes. If the value is `false`, the
`assert!` macro calls `panic!` to cause the test to fail. Using the `assert!`
macro helps us check that our code is functioning in the way we intend.
-->

La macro `assert!`, fournie par la bibliotheque standard, est utile quand vous voulez vous assurer qu'une certaine condition dans un test s'evalue a `true`. Nous donnons a la macro `assert!` un argument qui s'evalue en un booleen. Si la valeur est `true`, rien ne se passe et le test reussit. Si la valeur est `false`, la macro `assert!` appelle `panic!` pour faire echouer le test. Utiliser la macro `assert!` nous aide a verifier que notre code fonctionne de la maniere prevue.

<!--
In Chapter 5, Listing 5-15, we used a `Rectangle` struct and a `can_hold`
method, which are repeated here in Listing 11-5. Let's put this code in the
_src/lib.rs_ file, then write some tests for it using the `assert!` macro.
-->

Au chapitre 5, dans l'encart 5-15, nous avons utilise une structure `Rectangle` et une methode `can_hold`, qui sont reproduites ici dans l'encart 11-5. Mettons ce code dans le fichier _src/lib.rs_, puis ecrivons quelques tests en utilisant la macro `assert!`.

<Listing number="11-5" file-name="src/lib.rs" caption="La structure `Rectangle` et sa methode `can_hold` du chapitre 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

<!--
The `can_hold` method returns a Boolean, which means it's a perfect use case
for the `assert!` macro. In Listing 11-6, we write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a width of 8 and
a height of 7 and asserting that it can hold another `Rectangle` instance that
has a width of 5 and a height of 1.
-->

La methode `can_hold` retourne un booleen, ce qui en fait un cas d'utilisation parfait pour la macro `assert!`. Dans l'encart 11-6, nous ecrivons un test qui exerce la methode `can_hold` en creant une instance de `Rectangle` ayant une largeur de 8 et une hauteur de 7, et en verifiant qu'elle peut contenir une autre instance de `Rectangle` ayant une largeur de 5 et une hauteur de 1.

<Listing number="11-6" file-name="src/lib.rs" caption="Un test pour `can_hold` qui verifie si un plus grand rectangle peut effectivement contenir un plus petit rectangle">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

<!--
Note the `use super::*;` line inside the `tests` module. The `tests` module is
a regular module that follows the usual visibility rules we covered in Chapter
7 in the ["Paths for Referring to an Item in the Module
Tree"][paths-for-referring-to-an-item-in-the-module-tree] ignore
-->
section. Because the `tests` module is an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We use
a glob here, so anything we define in the outer module is available to this
`tests` module.
-->

Notez la ligne `use super::*;` a l'interieur du module `tests`. Le module `tests` est un module ordinaire qui suit les regles de visibilite habituelles que nous avons couvertes au chapitre 7 dans la section ["Les chemins pour faire reference a un element dans l'arborescence des modules"][paths-for-referring-to-an-item-in-the-module-tree]<!--
ignore
-->. Comme le module `tests` est un module interne, nous devons importer le code a tester du module externe dans la portee du module interne. Nous utilisons un glob ici, de sorte que tout ce que nous definissons dans le module externe est disponible dans ce module `tests`.

<!--
We've named our test `larger_can_hold_smaller`, and we've created the two
`Rectangle` instances that we need. Then, we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression is
supposed to return `true`, so our test should pass. Let's find out!
-->

Nous avons nomme notre test `larger_can_hold_smaller`, et nous avons cree les deux instances de `Rectangle` dont nous avons besoin. Ensuite, nous avons appele la macro `assert!` en lui passant le resultat de l'appel `larger.can_hold(&smaller)`. Cette expression est censee retourner `true`, donc notre test devrait reussir. Voyons cela !

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

<!--
It does pass! Let's add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:
-->

Il reussit ! Ajoutons un autre test, cette fois en verifiant qu'un plus petit rectangle ne peut pas contenir un plus grand rectangle :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

<!--
Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. As a
result, our test will pass if `can_hold` returns `false`:
-->

Comme le resultat correct de la fonction `can_hold` dans ce cas est `false`, nous devons inverser ce resultat avant de le passer a la macro `assert!`. En consequence, notre test reussira si `can_hold` retourne `false` :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

<!--
Two tests that pass! Now let's see what happens to our test results when we
introduce a bug in our code. We'll change the implementation of the `can_hold`
method by replacing the greater-than sign (`>`) with a less-than sign (`<`)
when it compares the widths:
-->

Deux tests qui reussissent ! Maintenant, voyons ce qui arrive a nos resultats de tests quand nous introduisons un bogue dans notre code. Nous allons changer l'implementation de la methode `can_hold` en remplacant le signe superieur (`>`) par un signe inferieur (`<`) lors de la comparaison des largeurs :

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

<!--
Running the tests now produces the following:
-->

L'execution des tests produit maintenant le resultat suivant :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

<!--
Our tests caught the bug! Because `larger.width` is `8` and `smaller.width` is
`5`, the comparison of the widths in `can_hold` now returns `false`: 8 is not
less than 5.
-->

Nos tests ont detecte le bogue ! Comme `larger.width` vaut `8` et `smaller.width` vaut `5`, la comparaison des largeurs dans `can_hold` retourne maintenant `false` : 8 n'est pas inferieur a 5.

<!--
Old headings. Do not remove or links may break.
-->

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>

<!--
### Testing Equality with `assert_eq!` and `assert_ne!`
-->

### Tester l'egalite avec `assert_eq!` et `assert_ne!`

<!--
A common way to verify functionality is to test for equality between the result
of the code under test and the value you expect the code to return. You could
do this by using the `assert!` macro and passing it an expression using the
`==` operator. However, this is such a common test that the standard library
provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test
more conveniently. These macros compare two arguments for equality or
inequality, respectively. They'll also print the two values if the assertion
fails, which makes it easier to see _why_ the test failed; conversely, the
`assert!` macro only indicates that it got a `false` value for the `==`
expression, without printing the values that led to the `false` value.
-->

Une facon courante de verifier le fonctionnement est de tester l'egalite entre le resultat du code teste et la valeur que vous attendez du code. Vous pourriez le faire en utilisant la macro `assert!` et en lui passant une expression utilisant l'operateur `==`. Cependant, c'est un test si courant que la bibliotheque standard fournit une paire de macros -- `assert_eq!` et `assert_ne!` -- pour effectuer ce test plus facilement. Ces macros comparent deux arguments pour l'egalite ou l'inegalite, respectivement. Elles affichent egalement les deux valeurs si l'assertion echoue, ce qui permet de voir plus facilement _pourquoi_ le test a echoue ; a l'inverse, la macro `assert!` indique seulement qu'elle a obtenu une valeur `false` pour l'expression `==`, sans afficher les valeurs qui ont mene a la valeur `false`.

<!--
In Listing 11-7, we write a function named `add_two` that adds `2` to its
parameter, and then we test this function using the `assert_eq!` macro.
-->

Dans l'encart 11-7, nous ecrivons une fonction nommee `add_two` qui ajoute `2` a son parametre, puis nous testons cette fonction en utilisant la macro `assert_eq!`.

<Listing number="11-7" file-name="src/lib.rs" caption="Test de la fonction `add_two` en utilisant la macro `assert_eq!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

<!--
Let's check that it passes!
-->

Verifions que ca reussit !

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

<!--
We create a variable named `result` that holds the result of calling
`add_two(2)`. Then, we pass `result` and `4` as the arguments to the
`assert_eq!` macro. The output line for this test is `test tests::it_adds_two
... ok`, and the `ok` text indicates that our test passed!
-->

Nous creons une variable nommee `result` qui contient le resultat de l'appel de `add_two(2)`. Ensuite, nous passons `result` et `4` comme arguments a la macro `assert_eq!`. La ligne de sortie pour ce test est `test tests::it_adds_two ... ok`, et le texte `ok` indique que notre test a reussi !

<!--
Let's introduce a bug into our code to see what `assert_eq!` looks like when it
fails. Change the implementation of the `add_two` function to instead add `3`:
-->

Introduisons un bogue dans notre code pour voir a quoi ressemble `assert_eq!` quand il echoue. Changez l'implementation de la fonction `add_two` pour qu'elle ajoute `3` a la place :

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

<!--
Run the tests again:
-->

Executez les tests a nouveau :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

<!--
Our test caught the bug! The `tests::it_adds_two` test failed, and the message
tells us that the assertion that failed was `left == right` and what the `left`
and `right` values are. This message helps us start debugging: The `left`
argument, where we had the result of calling `add_two(2)`, was `5`, but the
`right` argument was `4`. You can imagine that this would be especially helpful
when we have a lot of tests going on.
-->

Notre test a detecte le bogue ! Le test `tests::it_adds_two` a echoue, et le message nous indique que l'assertion qui a echoue etait `left == right` et quelles sont les valeurs de `left` et `right`. Ce message nous aide a commencer le debogage : l'argument `left`, ou nous avions le resultat de l'appel de `add_two(2)`, etait `5`, mais l'argument `right` etait `4`. Vous pouvez imaginer que cela serait particulierement utile quand nous avons beaucoup de tests en cours.

<!--
Note that in some languages and test frameworks, the parameters to equality
assertion functions are called `expected` and `actual`, and the order in which
we specify the arguments matters. However, in Rust, they're called `left` and
`right`, and the order in which we specify the value we expect and the value
the code produces doesn't matter. We could write the assertion in this test as
`assert_eq!(4, result)`, which would result in the same failure message that
displays `` assertion `left == right` failed ``.
-->

Notez que dans certains langages et frameworks de test, les parametres des fonctions d'assertion d'egalite sont appeles `expected` et `actual`, et l'ordre dans lequel nous specifiions les arguments a de l'importance. Cependant, en Rust, ils sont appeles `left` et `right`, et l'ordre dans lequel nous specifiions la valeur attendue et la valeur produite par le code n'a pas d'importance. Nous pourrions ecrire l'assertion dans ce test comme `assert_eq!(4, result)`, ce qui produirait le meme message d'echec affichant `` assertion `left == right` failed ``.

<!--
The `assert_ne!` macro will pass if the two values we give it are not equal and
will fail if they are equal. This macro is most useful for cases when we're not
sure what a value _will_ be, but we know what the value definitely _shouldn't_
be. For example, if we're testing a function that is guaranteed to change its
input in some way, but the way in which the input is changed depends on the day
of the week that we run our tests, the best thing to assert might be that the
output of the function is not equal to the input.
-->

La macro `assert_ne!` reussira si les deux valeurs que nous lui donnons ne sont pas egales et echouera si elles sont egales. Cette macro est surtout utile dans les cas ou nous ne savons pas quelle valeur quelque chose _aura_, mais nous savons quelle valeur elle ne _devrait_ definitivement _pas_ avoir. Par exemple, si nous testons une fonction qui est garantie de modifier son entree d'une certaine facon, mais la facon dont l'entree est modifiee depend du jour de la semaine ou nous executons nos tests, la meilleure chose a verifier pourrait etre que la sortie de la fonction n'est pas egale a l'entree.

<!--
Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All primitive types and most of
the standard library types implement these traits. For structs and enums that
you define yourself, you'll need to implement `PartialEq` to assert equality of
those types. You'll also need to implement `Debug` to print the values when the
assertion fails. Because both traits are derivable traits, as mentioned in
Listing 5-12 in Chapter 5, this is usually as straightforward as adding the
`#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See
Appendix C, ["Derivable Traits,"][derivable-traits] ignore
--> for more
details about these and other derivable traits.
-->

Sous la surface, les macros `assert_eq!` et `assert_ne!` utilisent respectivement les operateurs `==` et `!=`. Quand les assertions echouent, ces macros affichent leurs arguments en utilisant le formatage de debogage, ce qui signifie que les valeurs comparees doivent implementer les traits `PartialEq` et `Debug`. Tous les types primitifs et la plupart des types de la bibliotheque standard implementent ces traits. Pour les structures et les enumerations que vous definissez vous-meme, vous devrez implementer `PartialEq` pour verifier l'egalite de ces types. Vous devrez aussi implementer `Debug` pour afficher les valeurs quand l'assertion echoue. Comme les deux traits sont derivables, comme mentionne dans l'encart 5-12 au chapitre 5, c'est generalement aussi simple que d'ajouter l'annotation `#[derive(PartialEq, Debug)]` a votre definition de structure ou d'enumeration. Voir l'annexe C, ["Les traits derivables"][derivable-traits]<!--
ignore
-->, pour plus de details sur ces traits derivables et les autres.

<!--
### Adding Custom Failure Messages
-->

### Ajouter des messages d'echec personnalises

<!--
You can also add a custom message to be printed with the failure message as
optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any
arguments specified after the required arguments are passed along to the
`format!` macro (discussed in ["Concatenating with `+` or
`format!`"][concatenating]
ignore
--> in Chapter 8), so you can pass a format string that contains `{}`
placeholders and values to go in those placeholders. Custom messages are useful
for documenting what an assertion means; when a test fails, you'll have a better
idea of what the problem is with the code.
-->

Vous pouvez aussi ajouter un message personnalise a afficher avec le message d'echec en tant qu'arguments optionnels des macros `assert!`, `assert_eq!` et `assert_ne!`. Tous les arguments specifies apres les arguments requis sont passes a la macro `format!` (discutee dans ["La concatenation avec `+` ou `format!`"][concatenating]<!--
ignore
--> au chapitre 8), vous pouvez donc passer une chaine de format contenant des emplacements `{}` et des valeurs a y inserer. Les messages personnalises sont utiles pour documenter ce que signifie une assertion ; quand un test echoue, vous aurez une meilleure idee du probleme dans le code.

<!--
For example, let's say we have a function that greets people by name and we
want to test that the name we pass into the function appears in the output:
-->

Par exemple, disons que nous avons une fonction qui salue les personnes par leur nom et que nous voulons tester que le nom que nous passons a la fonction apparait dans la sortie :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

<!--
The requirements for this program haven't been agreed upon yet, and we're
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don't want to have to update the test when the requirements change,
so instead of checking for exact equality to the value returned from the
`greeting` function, we'll just assert that the output contains the text of the
input parameter.
-->

Les exigences de ce programme n'ont pas encore ete definies, et nous sommes assez surs que le texte `Hello` au debut du message de bienvenue changera. Nous avons decide que nous ne voulons pas avoir a mettre a jour le test quand les exigences changent, donc au lieu de verifier l'egalite exacte avec la valeur retournee par la fonction `greeting`, nous allons simplement verifier que la sortie contient le texte du parametre d'entree.

<!--
Now let's introduce a bug into this code by changing `greeting` to exclude
`name` to see what the default test failure looks like:
-->

Maintenant, introduisons un bogue dans ce code en modifiant `greeting` pour exclure `name` afin de voir a quoi ressemble un echec de test par defaut :

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

<!--
Running this test produces the following:
-->

L'execution de ce test produit le resultat suivant :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

<!--
This result just indicates that the assertion failed and which line the
assertion is on. A more useful failure message would print the value from the
`greeting` function. Let's add a custom failure message composed of a format
string with a placeholder filled in with the actual value we got from the
`greeting` function:
-->

Ce resultat indique seulement que l'assertion a echoue et sur quelle ligne se trouve l'assertion. Un message d'echec plus utile afficherait la valeur de la fonction `greeting`. Ajoutons un message d'echec personnalise compose d'une chaine de format avec un emplacement rempli par la valeur reelle que nous avons obtenue de la fonction `greeting` :

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

<!--
Now when we run the test, we'll get a more informative error message:
-->

Maintenant, quand nous executons le test, nous obtenons un message d'erreur plus informatif :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

<!--
We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.
-->

Nous pouvons voir la valeur que nous avons reellement obtenue dans la sortie du test, ce qui nous aide a deboguer ce qui s'est passe au lieu de ce que nous nous attendions a voir se produire.

<!--
### Checking for Panics with `should_panic`
-->

### Verifier les panics avec `should_panic`

<!--
In addition to checking return values, it's important to check that our code
handles error conditions as we expect. For example, consider the `Guess` type
that we created in Chapter 9, Listing 9-13. Other code that uses `Guess`
depends on the guarantee that `Guess` instances will contain only values
between 1 and 100. We can write a test that ensures that attempting to create a
`Guess` instance with a value outside that range panics.
-->

En plus de verifier les valeurs de retour, il est important de verifier que notre code gere les conditions d'erreur comme nous l'attendons. Par exemple, considerez le type `Guess` que nous avons cree au chapitre 9, dans l'encart 9-13. Le code qui utilise `Guess` depend de la garantie que les instances de `Guess` ne contiendront que des valeurs entre 1 et 100. Nous pouvons ecrire un test qui s'assure que tenter de creer une instance de `Guess` avec une valeur en dehors de cette plage provoque un panic.

<!--
We do this by adding the attribute `should_panic` to our test function. The
test passes if the code inside the function panics; the test fails if the code
inside the function doesn't panic.
-->

Nous faisons cela en ajoutant l'attribut `should_panic` a notre fonction de test. Le test reussit si le code a l'interieur de la fonction provoque un panic ; le test echoue si le code a l'interieur de la fonction ne provoque pas de panic.

<!--
Listing 11-8 shows a test that checks that the error conditions of `Guess::new`
happen when we expect them to.
-->

L'encart 11-8 montre un test qui verifie que les conditions d'erreur de `Guess::new` se produisent quand nous nous y attendons.

<Listing number="11-8" file-name="src/lib.rs" caption="Tester qu'une condition provoquera un `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

<!--
We place the `#[should_panic]` attribute after the `#[test]` attribute and
before the test function it applies to. Let's look at the result when this test
passes:
-->

Nous placons l'attribut `#[should_panic]` apres l'attribut `#[test]` et avant la fonction de test a laquelle il s'applique. Regardons le resultat quand ce test reussit :

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

<!--
Looks good! Now let's introduce a bug in our code by removing the condition
that the `new` function will panic if the value is greater than 100:
-->

Ca a l'air bon ! Maintenant, introduisons un bogue dans notre code en supprimant la condition qui fait que la fonction `new` provoque un panic si la valeur est superieure a 100 :

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

<!--
When we run the test in Listing 11-8, it will fail:
-->

Quand nous executons le test de l'encart 11-8, il echouera :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

<!--
We don't get a very helpful message in this case, but when we look at the test
function, we see that it's annotated with `#[should_panic]`. The failure we got
means that the code in the test function did not cause a panic.
-->

Nous n'obtenons pas un message tres utile dans ce cas, mais quand nous regardons la fonction de test, nous voyons qu'elle est annotee avec `#[should_panic]`. L'echec que nous avons obtenu signifie que le code dans la fonction de test n'a pas provoque de panic.

<!--
Tests that use `should_panic` can be imprecise. A `should_panic` test would
pass even if the test panics for a different reason from the one we were
expecting. To make `should_panic` tests more precise, we can add an optional
`expected` parameter to the `should_panic` attribute. The test harness will
make sure that the failure message contains the provided text. For example,
consider the modified code for `Guess` in Listing 11-9 where the `new` function
panics with different messages depending on whether the value is too small or
too large.
-->

Les tests utilisant `should_panic` peuvent etre imprecis. Un test `should_panic` reussirait meme si le test provoque un panic pour une raison differente de celle que nous attendions. Pour rendre les tests `should_panic` plus precis, nous pouvons ajouter un parametre optionnel `expected` a l'attribut `should_panic`. Le harnais de test s'assurera que le message d'echec contient le texte fourni. Par exemple, considerez le code modifie pour `Guess` dans l'encart 11-9 ou la fonction `new` provoque un panic avec des messages differents selon que la valeur est trop petite ou trop grande.

<Listing number="11-9" file-name="src/lib.rs" caption="Tester un `panic!` avec un message de panic contenant une sous-chaine specifiee">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

<!--
This test will pass because the value we put in the `should_panic` attribute's
`expected` parameter is a substring of the message that the `Guess::new`
function panics with. We could have specified the entire panic message that we
expect, which in this case would be `Guess value must be less than or equal to
100, got 200`. What you choose to specify depends on how much of the panic
message is unique or dynamic and how precise you want your test to be. In this
case, a substring of the panic message is enough to ensure that the code in the
test function executes the `else if value > 100` case.
-->

Ce test reussira parce que la valeur que nous avons mise dans le parametre `expected` de l'attribut `should_panic` est une sous-chaine du message avec lequel la fonction `Guess::new` provoque un panic. Nous aurions pu specifier le message de panic complet que nous attendions, qui dans ce cas serait `Guess value must be less than or equal to 100, got 200`. Ce que vous choisissez de specifier depend de la quantite du message de panic qui est unique ou dynamique et de la precision que vous souhaitez pour votre test. Dans ce cas, une sous-chaine du message de panic suffit pour s'assurer que le code dans la fonction de test execute le cas `else if value > 100`.

<!--
To see what happens when a `should_panic` test with an `expected` message
fails, let's again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:
-->

Pour voir ce qui se passe quand un test `should_panic` avec un message `expected` echoue, introduisons a nouveau un bogue dans notre code en echangeant les corps des blocs `if value < 1` et `else if value > 100` :

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

<!--
This time when we run the `should_panic` test, it will fail:
-->

Cette fois, quand nous executons le test `should_panic`, il echouera :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

<!--
The failure message indicates that this test did indeed panic as we expected,
but the panic message did not include the expected string `less than or equal
to 100`. The panic message that we did get in this case was `Guess value must
be greater than or equal to 1, got 200`. Now we can start figuring out where
our bug is!
-->

Le message d'echec indique que ce test a bien provoque un panic comme nous l'attendions, mais le message de panic ne contenait pas la chaine attendue `less than or equal to 100`. Le message de panic que nous avons obtenu dans ce cas etait `Guess value must be greater than or equal to 1, got 200`. Nous pouvons maintenant commencer a chercher ou se trouve notre bogue !

<!--
### Using `Result<T, E>` in Tests
-->

### Utiliser `Result<T, E>` dans les tests

<!--
All of our tests so far panic when they fail. We can also write tests that use
`Result<T, E>`! Here's the test from Listing 11-1, rewritten to use `Result<T,
E>` and return an `Err` instead of panicking:
-->

Tous nos tests jusqu'ici provoquent un panic quand ils echouent. Nous pouvons aussi ecrire des tests qui utilisent `Result<T, E>` ! Voici le test de l'encart 11-1, reecrit pour utiliser `Result<T, E>` et retourner un `Err` au lieu de provoquer un panic :

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

<!--
The `it_works` function now has the `Result<(), String>` return type. In the
body of the function, rather than calling the `assert_eq!` macro, we return
`Ok(())` when the test passes and an `Err` with a `String` inside when the test
fails.
-->

La fonction `it_works` a maintenant le type de retour `Result<(), String>`. Dans le corps de la fonction, au lieu d'appeler la macro `assert_eq!`, nous retournons `Ok(())` quand le test reussit et un `Err` contenant un `String` quand le test echoue.

<!--
Writing tests so that they return a `Result<T, E>` enables you to use the
question mark operator in the body of tests, which can be a convenient way to
write tests that should fail if any operation within them returns an `Err`
variant.
-->

Ecrire des tests qui retournent un `Result<T, E>` vous permet d'utiliser l'operateur point d'interrogation dans le corps des tests, ce qui peut etre une facon pratique d'ecrire des tests qui devraient echouer si une operation en leur sein retourne une variante `Err`.

<!--
You can't use the `#[should_panic]` annotation on tests that use `Result<T,
E>`. To assert that an operation returns an `Err` variant, _don't_ use the
question mark operator on the `Result<T, E>` value. Instead, use
`assert!(value.is_err())`.
-->

Vous ne pouvez pas utiliser l'annotation `#[should_panic]` sur les tests qui utilisent `Result<T, E>`. Pour verifier qu'une operation retourne une variante `Err`, _n'utilisez pas_ l'operateur point d'interrogation sur la valeur `Result<T, E>`. Utilisez plutot `assert!(value.is_err())`.

<!--
Now that you know several ways to write tests, let's look at what is happening
when we run our tests and explore the different options we can use with `cargo
test`.
-->

Maintenant que vous connaissez plusieurs facons d'ecrire des tests, voyons ce qui se passe quand nous executons nos tests et explorons les differentes options que nous pouvons utiliser avec `cargo test`.

[concatenating]: ch08-02-strings.html#concatenating-with--or-format
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]: ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
