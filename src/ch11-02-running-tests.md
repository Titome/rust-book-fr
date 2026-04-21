<!--
## Controlling How Tests Are Run
-->

## Controler comment les tests sont executes

<!--
Just as `cargo run` compiles your code and then runs the resultant binary,
`cargo test` compiles your code in test mode and runs the resultant test
binary. The default behavior of the binary produced by `cargo test` is to run
all the tests in parallel and capture output generated during test runs,
preventing the output from being displayed and making it easier to read the
output related to the test results. You can, however, specify command line
options to change this default behavior.
-->

Tout comme `cargo run` compile votre code puis execute le binaire resultant, `cargo test` compile votre code en mode test et execute le binaire de test resultant. Le comportement par defaut du binaire produit par `cargo test` est d'executer tous les tests en parallele et de capturer la sortie generee pendant l'execution des tests, empechant la sortie d'etre affichee et facilitant la lecture de la sortie liee aux resultats des tests. Vous pouvez cependant specifier des options en ligne de commande pour modifier ce comportement par defaut.

<!--
Some command line options go to `cargo test`, and some go to the resultant test
binary. To separate these two types of arguments, you list the arguments that
go to `cargo test` followed by the separator `--` and then the ones that go to
the test binary. Running `cargo test --help` displays the options you can use
with `cargo test`, and running `cargo test -- --help` displays the options you
can use after the separator. These options are also documented in [the "Tests"
section of _The `rustc` Book_][tests].
-->

Certaines options en ligne de commande sont destinees a `cargo test`, et d'autres au binaire de test resultant. Pour separer ces deux types d'arguments, vous listez les arguments destines a `cargo test` suivis du separateur `--` puis ceux destines au binaire de test. Executer `cargo test --help` affiche les options utilisables avec `cargo test`, et executer `cargo test -- --help` affiche les options utilisables apres le separateur. Ces options sont egalement documentees dans [la section "Tests" du livre _The `rustc` Book_][tests].

[tests]: https://doc.rust-lang.org/rustc/tests/index.html

<!--
### Running Tests in Parallel or Consecutively
-->

### Executer les tests en parallele ou consecutivement

<!--
When you run multiple tests, by default they run in parallel using threads,
meaning they finish running more quickly and you get feedback sooner. Because
the tests are running at the same time, you must make sure your tests don't
depend on each other or on any shared state, including a shared environment,
such as the current working directory or environment variables.
-->

Quand vous executez plusieurs tests, par defaut ils s'executent en parallele en utilisant des threads, ce qui signifie qu'ils se terminent plus rapidement et que vous obtenez un retour plus tot. Comme les tests s'executent en meme temps, vous devez vous assurer que vos tests ne dependent pas les uns des autres ni d'un etat partage, y compris un environnement partage, comme le repertoire de travail courant ou les variables d'environnement.

<!--
For example, say each of your tests runs some code that creates a file on disk
named _test-output.txt_ and writes some data to that file. Then, each test
reads the data in that file and asserts that the file contains a particular
value, which is different in each test. Because the tests run at the same time,
one test might overwrite the file in the time between when another test is
writing and reading the file. The second test will then fail, not because the
code is incorrect but because the tests have interfered with each other while
running in parallel. One solution is to make sure each test writes to a
different file; another solution is to run the tests one at a time.
-->

Par exemple, imaginons que chacun de vos tests execute du code qui cree un fichier sur le disque nomme _test-output.txt_ et ecrit des donnees dans ce fichier. Ensuite, chaque test lit les donnees de ce fichier et verifie que le fichier contient une valeur particuliere, qui est differente dans chaque test. Comme les tests s'executent en meme temps, un test pourrait ecraser le fichier entre le moment ou un autre test ecrit et lit le fichier. Le second test echouera alors, non pas parce que le code est incorrect mais parce que les tests se sont interferes mutuellement en s'executant en parallele. Une solution est de s'assurer que chaque test ecrit dans un fichier different ; une autre solution est d'executer les tests un par un.

<!--
If you don't want to run the tests in parallel or if you want more fine-grained
control over the number of threads used, you can send the `--test-threads` flag
and the number of threads you want to use to the test binary. Take a look at
the following example:
-->

Si vous ne voulez pas executer les tests en parallele ou si vous souhaitez un controle plus fin sur le nombre de threads utilises, vous pouvez envoyer le drapeau `--test-threads` et le nombre de threads que vous souhaitez utiliser au binaire de test. Regardez l'exemple suivant :

<!--
```console
$ cargo test -- --test-threads=1
```
-->

```console
$ cargo test -- --test-threads=1
```

<!--
We set the number of test threads to `1`, telling the program not to use any
parallelism. Running the tests using one thread will take longer than running
them in parallel, but the tests won't interfere with each other if they share
state.
-->

Nous avons defini le nombre de threads de test a `1`, indiquant au programme de ne pas utiliser de parallelisme. Executer les tests avec un seul thread prendra plus de temps que de les executer en parallele, mais les tests ne s'interfereront pas les uns avec les autres s'ils partagent un etat.

<!--
### Showing Function Output
-->

### Afficher la sortie des fonctions

<!--
By default, if a test passes, Rust's test library captures anything printed to
standard output. For example, if we call `println!` in a test and the test
passes, we won't see the `println!` output in the terminal; we'll see only the
line that indicates the test passed. If a test fails, we'll see whatever was
printed to standard output with the rest of the failure message.
-->

Par defaut, si un test reussit, la bibliotheque de test de Rust capture tout ce qui est affiche sur la sortie standard. Par exemple, si nous appelons `println!` dans un test et que le test reussit, nous ne verrons pas la sortie de `println!` dans le terminal ; nous verrons seulement la ligne qui indique que le test a reussi. Si un test echoue, nous verrons tout ce qui a ete affiche sur la sortie standard avec le reste du message d'echec.

<!--
As an example, Listing 11-10 has a silly function that prints the value of its
parameter and returns 10, as well as a test that passes and a test that fails.
-->

A titre d'exemple, l'encart 11-10 contient une fonction simple qui affiche la valeur de son parametre et retourne 10, ainsi qu'un test qui reussit et un test qui echoue.

<Listing number="11-10" file-name="src/lib.rs" caption="Tests pour une fonction qui appelle `println!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

<!--
When we run these tests with `cargo test`, we'll see the following output:
-->

Quand nous executons ces tests avec `cargo test`, nous verrons la sortie suivante :

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

<!--
Note that nowhere in this output do we see `I got the value 4`, which is
printed when the test that passes runs. That output has been captured. The
output from the test that failed, `I got the value 8`, appears in the section
of the test summary output, which also shows the cause of the test failure.
-->

Notez que nulle part dans cette sortie nous ne voyons `I got the value 4`, qui est affiche quand le test qui reussit s'execute. Cette sortie a ete capturee. La sortie du test qui a echoue, `I got the value 8`, apparait dans la section du resume de sortie des tests, qui montre egalement la cause de l'echec du test.

<!--
If we want to see printed values for passing tests as well, we can tell Rust to
also show the output of successful tests with `--show-output`:
-->

Si nous voulons voir les valeurs affichees pour les tests qui reussissent egalement, nous pouvons demander a Rust d'afficher aussi la sortie des tests reussis avec `--show-output` :

<!--
```console
$ cargo test -- --show-output
```
-->

```console
$ cargo test -- --show-output
```

<!--
When we run the tests in Listing 11-10 again with the `--show-output` flag, we
see the following output:
-->

Quand nous executons les tests de l'encart 11-10 a nouveau avec le drapeau `--show-output`, nous voyons la sortie suivante :

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

<!--
### Running a Subset of Tests by Name
-->

### Executer un sous-ensemble de tests par nom

<!--
Running a full test suite can sometimes take a long time. If you're working on
code in a particular area, you might want to run only the tests pertaining to
that code. You can choose which tests to run by passing `cargo test` the name
or names of the test(s) you want to run as an argument.
-->

Executer une suite de tests complete peut parfois prendre beaucoup de temps. Si vous travaillez sur du code dans un domaine particulier, vous pourriez vouloir n'executer que les tests relatifs a ce code. Vous pouvez choisir quels tests executer en passant a `cargo test` le nom ou les noms du ou des tests que vous souhaitez executer en argument.

<!--
To demonstrate how to run a subset of tests, we'll first create three tests for
our `add_two` function, as shown in Listing 11-11, and choose which ones to run.
-->

Pour montrer comment executer un sous-ensemble de tests, nous allons d'abord creer trois tests pour notre fonction `add_two`, comme montre dans l'encart 11-11, et choisir lesquels executer.

<Listing number="11-11" file-name="src/lib.rs" caption="Trois tests avec trois noms differents">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

<!--
If we run the tests without passing any arguments, as we saw earlier, all the
tests will run in parallel:
-->

Si nous executons les tests sans passer d'arguments, comme nous l'avons vu precedemment, tous les tests s'executeront en parallele :

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

<!--
#### Running Single Tests
-->

#### Executer un seul test

<!--
We can pass the name of any test function to `cargo test` to run only that test:
-->

Nous pouvons passer le nom de n'importe quelle fonction de test a `cargo test` pour n'executer que ce test :

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

<!--
Only the test with the name `one_hundred` ran; the other two tests didn't match
that name. The test output lets us know we had more tests that didn't run by
displaying `2 filtered out` at the end.
-->

Seul le test nomme `one_hundred` a ete execute ; les deux autres tests ne correspondaient pas a ce nom. La sortie des tests nous indique qu'il y avait d'autres tests qui n'ont pas ete executes en affichant `2 filtered out` a la fin.

<!--
We can't specify the names of multiple tests in this way; only the first value
given to `cargo test` will be used. But there is a way to run multiple tests.
-->

Nous ne pouvons pas specifier les noms de plusieurs tests de cette facon ; seule la premiere valeur donnee a `cargo test` sera utilisee. Mais il existe un moyen d'executer plusieurs tests.

<!--
#### Filtering to Run Multiple Tests
-->

#### Filtrer pour executer plusieurs tests

<!--
We can specify part of a test name, and any test whose name matches that value
will be run. For example, because two of our tests' names contain `add`, we can
run those two by running `cargo test add`:
-->

Nous pouvons specifier une partie d'un nom de test, et tout test dont le nom correspond a cette valeur sera execute. Par exemple, comme deux de nos noms de tests contiennent `add`, nous pouvons executer ces deux-la en lancant `cargo test add` :

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

<!--
This command ran all tests with `add` in the name and filtered out the test
named `one_hundred`. Also note that the module in which a test appears becomes
part of the test's name, so we can run all the tests in a module by filtering
on the module's name.
-->

Cette commande a execute tous les tests contenant `add` dans leur nom et a filtre le test nomme `one_hundred`. Notez egalement que le module dans lequel un test apparait fait partie du nom du test, nous pouvons donc executer tous les tests d'un module en filtrant sur le nom du module.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ignoring-some-tests-unless-specifically-requested"></a>

<!--
### Ignoring Tests Unless Specifically Requested
-->

### Ignorer des tests sauf demande explicite

<!--
Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, you can instead annotate the
time-consuming tests using the `ignore` attribute to exclude them, as shown
here:
-->

Parfois, quelques tests specifiques peuvent etre tres longs a executer, vous pourriez donc vouloir les exclure lors de la plupart des executions de `cargo test`. Plutot que de lister en arguments tous les tests que vous souhaitez executer, vous pouvez annoter les tests chronophages avec l'attribut `ignore` pour les exclure, comme montre ici :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

<!--
After `#[test]`, we add the `#[ignore]` line to the test we want to exclude.
Now when we run our tests, `it_works` runs, but `expensive_test` doesn't:
-->

Apres `#[test]`, nous ajoutons la ligne `#[ignore]` au test que nous voulons exclure. Maintenant, quand nous executons nos tests, `it_works` s'execute, mais `expensive_test` ne s'execute pas :

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

<!--
The `expensive_test` function is listed as `ignored`. If we want to run only
the ignored tests, we can use `cargo test -- --ignored`:
-->

La fonction `expensive_test` est listee comme `ignored`. Si nous voulons n'executer que les tests ignores, nous pouvons utiliser `cargo test -- --ignored` :

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

<!--
By controlling which tests run, you can make sure your `cargo test` results
will be returned quickly. When you're at a point where it makes sense to check
the results of the `ignored` tests and you have time to wait for the results,
you can run `cargo test -- --ignored` instead. If you want to run all tests
whether they're ignored or not, you can run `cargo test -- --include-ignored`.
-->

En controlant quels tests s'executent, vous pouvez vous assurer que les resultats de `cargo test` seront retournes rapidement. Quand vous en etes a un point ou il est logique de verifier les resultats des tests `ignored` et que vous avez le temps d'attendre les resultats, vous pouvez executer `cargo test -- --ignored` a la place. Si vous voulez executer tous les tests, qu'ils soient ignores ou non, vous pouvez executer `cargo test -- --include-ignored`.
