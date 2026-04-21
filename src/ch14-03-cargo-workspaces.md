<!--
## Cargo Workspaces
-->

## Les espaces de travail Cargo

<!--
In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split your package further into
multiple library crates. Cargo offers a feature called _workspaces_ that can
help manage multiple related packages that are developed in tandem.
-->

Dans le chapitre 12, nous avons construit un paquet qui incluait un crate binaire et un crate de bibliotheque. Au fur et a mesure que votre projet se developpe, vous pourriez constater que le crate de bibliotheque continue de grossir et que vous souhaitez diviser votre paquet en plusieurs crates de bibliotheque. Cargo offre une fonctionnalite appelee _workspaces_ (espaces de travail) qui peut aider a gerer plusieurs paquets lies developpes en tandem.

<!--
### Creating a Workspace
-->

### Creer un espace de travail

<!--
A _workspace_ is a set of packages that share the same _Cargo.lock_ and output
directory. Let's make a project using a workspace—we'll use trivial code so
that we can concentrate on the structure of the workspace. There are multiple
ways to structure a workspace, so we'll just show one common way. We'll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function and the other library an `add_two` function.
These three crates will be part of the same workspace. We'll start by creating
a new directory for the workspace:
-->

Un _workspace_ est un ensemble de paquets qui partagent le meme _Cargo.lock_ et le meme repertoire de sortie. Creons un projet utilisant un workspace -- nous utiliserons du code trivial pour pouvoir nous concentrer sur la structure du workspace. Il y a plusieurs facons de structurer un workspace, donc nous n'en montrerons qu'une courante. Nous aurons un workspace contenant un binaire et deux bibliotheques. Le binaire, qui fournira la fonctionnalite principale, dependra des deux bibliotheques. Une bibliotheque fournira une fonction `add_one` et l'autre bibliotheque une fonction `add_two`. Ces trois crates feront partie du meme workspace. Nous commencerons par creer un nouveau repertoire pour le workspace :

<!--
```console
$ mkdir add
$ cd add
```
-->

```console
$ mkdir add
$ cd add
```

<!--
Next, in the _add_ directory, we create the _Cargo.toml_ file that will
configure the entire workspace. This file won't have a `[package]` section.
Instead, it will start with a `[workspace]` section that will allow us to add
members to the workspace. We also make a point to use the latest and greatest
version of Cargo's resolver algorithm in our workspace by setting the
`resolver` value to `"3"`:
-->

Ensuite, dans le repertoire _add_, nous creons le fichier _Cargo.toml_ qui configurera l'ensemble du workspace. Ce fichier n'aura pas de section `[package]`. A la place, il commencera par une section `[workspace]` qui nous permettra d'ajouter des membres au workspace. Nous prenons egalement soin d'utiliser la version la plus recente de l'algorithme de resolution de Cargo dans notre workspace en definissant la valeur `resolver` a `"3"` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>


```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

<!--
Next, we'll create the `adder` binary crate by running `cargo new` within the
_add_ directory:
-->

Ensuite, nous allons creer le crate binaire `adder` en executant `cargo new` dans le repertoire _add_ :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

<!--
```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

<!--
Running `cargo new` inside a workspace also automatically adds the newly created
package to the `members` key in the `[workspace]` definition in the workspace
_Cargo.toml_, like this:
-->

Executer `cargo new` a l'interieur d'un workspace ajoute egalement automatiquement le paquet nouvellement cree a la cle `members` dans la definition `[workspace]` du _Cargo.toml_ du workspace, comme ceci :


```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

<!--
At this point, we can build the workspace by running `cargo build`. The files
in your _add_ directory should look like this:
-->

A ce stade, nous pouvons compiler le workspace en executant `cargo build`. Les fichiers dans votre repertoire _add_ devraient ressembler a ceci :

<!--
```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
-->

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
The workspace has one _target_ directory at the top level that the compiled
artifacts will be placed into; the `adder` package doesn't have its own
_target_ directory. Even if we were to run `cargo build` from inside the
_adder_ directory, the compiled artifacts would still end up in _add/target_
rather than _add/adder/target_. Cargo structures the _target_ directory in a
workspace like this because the crates in a workspace are meant to depend on
each other. If each crate had its own _target_ directory, each crate would have
to recompile each of the other crates in the workspace to place the artifacts
in its own _target_ directory. By sharing one _target_ directory, the crates
can avoid unnecessary rebuilding.
-->

Le workspace a un seul repertoire _target_ au niveau superieur dans lequel les artefacts compiles seront places ; le paquet `adder` n'a pas son propre repertoire _target_. Meme si nous executons `cargo build` depuis l'interieur du repertoire _adder_, les artefacts compiles finiraient quand meme dans _add/target_ plutot que dans _add/adder/target_. Cargo structure le repertoire _target_ dans un workspace de cette facon parce que les crates dans un workspace sont destines a dependre les uns des autres. Si chaque crate avait son propre repertoire _target_, chaque crate devrait recompiler chacun des autres crates du workspace pour placer les artefacts dans son propre repertoire _target_. En partageant un seul repertoire _target_, les crates peuvent eviter les recompilations inutiles.

<!--
### Creating the Second Package in the Workspace
-->

### Creer le deuxieme paquet dans le workspace

<!--
Next, let's create another member package in the workspace and call it
`add_one`. Generate a new library crate named `add_one`:
-->

Ensuite, creons un autre paquet membre dans le workspace et appelons-le `add_one`. Generez un nouveau crate de bibliotheque nomme `add_one` :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

<!--
```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

<!--
The top-level _Cargo.toml_ will now include the _add_one_ path in the `members`
list:
-->

Le _Cargo.toml_ de niveau superieur inclura maintenant le chemin _add_one_ dans la liste `members` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>


```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

<!--
Your _add_ directory should now have these directories and files:
-->

Votre repertoire _add_ devrait maintenant avoir ces repertoires et fichiers :

<!--
```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```
-->

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

<!--
In the _add_one/src/lib.rs_ file, let's add an `add_one` function:
-->

Dans le fichier _add_one/src/lib.rs_, ajoutons une fonction `add_one` :

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">Fichier : add_one/src/lib.rs</span>


```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

<!--
Now we can have the `adder` package with our binary depend on the `add_one`
package that has our library. First, we'll need to add a path dependency on
`add_one` to _adder/Cargo.toml_.
-->

Maintenant, nous pouvons faire dependre le paquet `adder` avec notre binaire du paquet `add_one` qui contient notre bibliotheque. D'abord, nous devrons ajouter une dependance de chemin vers `add_one` dans _adder/Cargo.toml_.

<!--
<span class="filename">Filename: adder/Cargo.toml</span>
-->

<span class="filename">Fichier : adder/Cargo.toml</span>


```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

<!--
Cargo doesn't assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships.
-->

Cargo ne suppose pas que les crates dans un workspace dependront les uns des autres, nous devons donc etre explicites sur les relations de dependance.

<!--
Next, let's use the `add_one` function (from the `add_one` crate) in the
`adder` crate. Open the _adder/src/main.rs_ file and change the `main`
function to call the `add_one` function, as in Listing 14-7.
-->

Ensuite, utilisons la fonction `add_one` (du crate `add_one`) dans le crate `adder`. Ouvrez le fichier _adder/src/main.rs_ et modifiez la fonction `main` pour appeler la fonction `add_one`, comme dans l'encart 14-7.

<Listing number="14-7" file-name="adder/src/main.rs" caption="Utilisation du crate de bibliotheque `add_one` depuis le crate `adder`">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

<!--
Let's build the workspace by running `cargo build` in the top-level _add_
directory!
-->

Compilons le workspace en executant `cargo build` dans le repertoire _add_ de niveau superieur !

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

<!--
```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

<!--
To run the binary crate from the _add_ directory, we can specify which package
in the workspace we want to run by using the `-p` argument and the package name
with `cargo run`:
-->

Pour executer le crate binaire depuis le repertoire _add_, nous pouvons specifier quel paquet dans le workspace nous voulons executer en utilisant l'argument `-p` et le nom du paquet avec `cargo run` :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

<!--
```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

<!--
This runs the code in _adder/src/main.rs_, which depends on the `add_one` crate.
-->

Cela execute le code dans _adder/src/main.rs_, qui depend du crate `add_one`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="depending-on-an-external-package-in-a-workspace"></a>

<!--
### Depending on an External Package
-->

### Dependre d'un paquet externe

<!--
Notice that the workspace has only one _Cargo.lock_ file at the top level,
rather than having a _Cargo.lock_ in each crate's directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`
package to the _adder/Cargo.toml_ and _add_one/Cargo.toml_ files, Cargo will
resolve both of those to one version of `rand` and record that in the one
_Cargo.lock_. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let's add the
`rand` crate to the `[dependencies]` section in the _add_one/Cargo.toml_ file
so that we can use the `rand` crate in the `add_one` crate:
-->

Remarquez que le workspace n'a qu'un seul fichier _Cargo.lock_ au niveau superieur, plutot qu'un _Cargo.lock_ dans le repertoire de chaque crate. Cela garantit que tous les crates utilisent la meme version de toutes les dependances. Si nous ajoutons le paquet `rand` aux fichiers _adder/Cargo.toml_ et _add_one/Cargo.toml_, Cargo resoudra les deux vers une seule version de `rand` et l'enregistrera dans l'unique _Cargo.lock_. Faire en sorte que tous les crates du workspace utilisent les memes dependances signifie que les crates seront toujours compatibles entre eux. Ajoutons le crate `rand` a la section `[dependencies]` dans le fichier _add_one/Cargo.toml_ afin de pouvoir utiliser le crate `rand` dans le crate `add_one` :

<!--
When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<!--
<span class="filename">Filename: add_one/Cargo.toml</span>
-->

<span class="filename">Fichier : add_one/Cargo.toml</span>


```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

<!--
We can now add `use rand;` to the _add_one/src/lib.rs_ file, and building the
whole workspace by running `cargo build` in the _add_ directory will bring in
and compile the `rand` crate. We will get one warning because we aren't
referring to the `rand` we brought into scope:
-->

Nous pouvons maintenant ajouter `use rand;` au fichier _add_one/src/lib.rs_, et compiler l'ensemble du workspace en executant `cargo build` dans le repertoire _add_ integrera et compilera le crate `rand`. Nous obtiendrons un avertissement car nous ne faisons pas reference au `rand` que nous avons amene dans la portee :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

<!--
```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
--> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

<!--
The top-level _Cargo.lock_ now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can't use it in other crates in the workspace unless we add
`rand` to their _Cargo.toml_ files as well. For example, if we add `use rand;`
to the _adder/src/main.rs_ file for the `adder` package, we'll get an error:
-->

Le _Cargo.lock_ de niveau superieur contient maintenant des informations sur la dependance d'`add_one` envers `rand`. Cependant, meme si `rand` est utilise quelque part dans le workspace, nous ne pouvons pas l'utiliser dans d'autres crates du workspace a moins d'ajouter egalement `rand` a leurs fichiers _Cargo.toml_. Par exemple, si nous ajoutons `use rand;` au fichier _adder/src/main.rs_ pour le paquet `adder`, nous obtiendrons une erreur :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

<!--
```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
--> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

<!--
To fix this, edit the _Cargo.toml_ file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in _Cargo.lock_, but no
additional copies of `rand` will be downloaded. Cargo will ensure that every
crate in every package in the workspace using the `rand` package will use the
same version as long as they specify compatible versions of `rand`, saving us
space and ensuring that the crates in the workspace will be compatible with
each other.
-->

Pour corriger cela, editez le fichier _Cargo.toml_ du paquet `adder` et indiquez que `rand` est egalement une dependance pour celui-ci. Compiler le paquet `adder` ajoutera `rand` a la liste des dependances pour `adder` dans _Cargo.lock_, mais aucune copie supplementaire de `rand` ne sera telechargee. Cargo s'assurera que chaque crate dans chaque paquet du workspace utilisant le paquet `rand` utilisera la meme version tant qu'ils specifient des versions compatibles de `rand`, nous faisant economiser de l'espace et garantissant que les crates du workspace seront compatibles entre eux.

<!--
If crates in the workspace specify incompatible versions of the same
dependency, Cargo will resolve each of them but will still try to resolve as
few versions as possible.
-->

Si les crates du workspace specifient des versions incompatibles de la meme dependance, Cargo resoudra chacune d'entre elles mais essaiera quand meme de resoudre le moins de versions possible.

<!--
### Adding a Test to a Workspace
-->

### Ajouter un test a un workspace

<!--
For another enhancement, let's add a test of the `add_one::add_one` function
within the `add_one` crate:
-->

Pour une autre amelioration, ajoutons un test de la fonction `add_one::add_one` dans le crate `add_one` :

<!--
<span class="filename">Filename: add_one/src/lib.rs</span>
-->

<span class="filename">Fichier : add_one/src/lib.rs</span>


```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

<!--
Now run `cargo test` in the top-level _add_ directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:
-->

Maintenant, executez `cargo test` dans le repertoire _add_ de niveau superieur. Executer `cargo test` dans un workspace structure comme celui-ci executera les tests pour tous les crates du workspace :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

<!--
```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows that zero documentation tests were found
in the `add_one` crate.
-->

La premiere section de la sortie montre que le test `it_works` dans le crate `add_one` a reussi. La section suivante montre que zero test a ete trouve dans le crate `adder`, puis la derniere section montre que zero test de documentation a ete trouve dans le crate `add_one`.

<!--
We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:
-->

Nous pouvons egalement executer les tests pour un crate particulier dans un workspace depuis le repertoire de niveau superieur en utilisant le drapeau `-p` et en specifiant le nom du crate que nous voulons tester :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

<!--
```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<!--
This output shows `cargo test` only ran the tests for the `add_one` crate and
didn't run the `adder` crate tests.
-->

Cette sortie montre que `cargo test` n'a execute que les tests du crate `add_one` et n'a pas execute les tests du crate `adder`.

<!--
If you publish the crates in the workspace to
[crates.io](https://crates.io/) ignore
-->, each crate in the workspace
will need to be published separately. Like `cargo test`, we can publish a
particular crate in our workspace by using the `-p` flag and specifying the
name of the crate we want to publish.
-->

Si vous publiez les crates du workspace sur [crates.io](https://crates.io/)<!--
ignore
-->, chaque crate du workspace devra etre publie separement. Comme pour `cargo test`, nous pouvons publier un crate particulier dans notre workspace en utilisant le drapeau `-p` et en specifiant le nom du crate que nous voulons publier.

<!--
For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!
-->

Pour vous exercer davantage, ajoutez un crate `add_two` a ce workspace de la meme maniere que le crate `add_one` !

<!--
As your project grows, consider using a workspace: It enables you to work with
smaller, easier-to-understand components than one big blob of code.
Furthermore, keeping the crates in a workspace can make coordination between
crates easier if they are often changed at the same time.
-->

Au fur et a mesure que votre projet grandit, envisagez d'utiliser un workspace : il vous permet de travailler avec des composants plus petits et plus faciles a comprendre plutot qu'un seul gros bloc de code. De plus, garder les crates dans un workspace peut faciliter la coordination entre les crates s'ils sont souvent modifies en meme temps.
