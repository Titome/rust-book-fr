<!--
## Working with Environment Variables
-->

## Travailler avec les variables d'environnement

<!--
We'll improve the `minigrep` binary by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but by instead making it an
environment variable, we allow our users to set the environment variable once
and have all their searches be case insensitive in that terminal session.
-->

Nous allons améliorer le binaire `minigrep` en ajoutant une fonctionnalité supplémentaire : une option de recherche insensible à la casse que l'utilisateur peut activer via une variable d'environnement. Nous pourrions faire de cette fonctionnalité une option de ligne de commande et exiger que les utilisateurs la saisissent à chaque fois qu'ils veulent l'appliquer, mais en en faisant plutôt une variable d'environnement, nous permettons à nos utilisateurs de définir la variable d'environnement une seule fois et d'avoir toutes leurs recherches insensibles à la casse dans cette session de terminal.

<!--
Old headings. Do not remove or links may break.
-->
<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>

<!--
### Writing a Failing Test for Case-Insensitive Search
-->

### Écrire un test qui échoue pour la recherche insensible à la casse

<!--
We first add a new `search_case_insensitive` function to the `minigrep` library
that will be called when the environment variable has a value. We'll continue
to follow the TDD process, so the first step is again to write a failing test.
We'll add a new test for the new `search_case_insensitive` function and rename
our old test from `one_result` to `case_sensitive` to clarify the differences
between the two tests, as shown in Listing 12-20.
-->

Nous ajoutons d'abord une nouvelle fonction `search_case_insensitive` à la bibliothèque `minigrep` qui sera appelée lorsque la variable d'environnement a une valeur. Nous continuerons à suivre le processus TDD, donc la première étape est à nouveau d'écrire un test qui échoue. Nous ajouterons un nouveau test pour la nouvelle fonction `search_case_insensitive` et renommerons notre ancien test de `one_result` en `case_sensitive` pour clarifier les différences entre les deux tests, comme illustré dans l'encart 12-20.

<Listing number="12-20" file-name="src/lib.rs" caption="Ajout d'un nouveau test qui échoue pour la fonction insensible à la casse que nous sommes sur le point d'ajouter">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

<!--
Note that we've edited the old test's `contents` too. We've added a new line
with the text `"Duct tape."` using a capital _D_ that shouldn't match the query
`"duct"` when we're searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don't accidentally break the case-sensitive
search functionality that we've already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.
-->

Notez que nous avons aussi modifié le `contents` de l'ancien test. Nous avons ajouté une nouvelle ligne avec le texte `"Duct tape."` utilisant un _D_ majuscule qui ne devrait pas correspondre à la requête `"duct"` lorsque nous cherchons de manière sensible à la casse. Modifier l'ancien test de cette façon permet de s'assurer que nous ne cassons pas accidentellement la fonctionnalité de recherche sensible à la casse que nous avons déjà implémentée. Ce test devrait passer maintenant et continuer à passer pendant que nous travaillons sur la recherche insensible à la casse.

<!--
The new test for the case-_insensitive_ search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we're about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital _R_ and match the
line `"Trust me."` even though both have different casing from the query. This
is our failing test, and it will fail to compile because we haven't yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.
-->

Le nouveau test pour la recherche _insensible_ à la casse utilise `"rUsT"` comme requête. Dans la fonction `search_case_insensitive` que nous sommes sur le point d'ajouter, la requête `"rUsT"` devrait correspondre à la ligne contenant `"Rust:"` avec un _R_ majuscule et correspondre à la ligne `"Trust me."` même si les deux ont une casse différente de la requête. C'est notre test qui échoue, et il ne compilera pas car nous n'avons pas encore défini la fonction `search_case_insensitive`. N'hésitez pas à ajouter une implémentation squelette qui renvoie toujours un vecteur vide, de la même manière que nous l'avons fait pour la fonction `search` dans l'encart 12-16, pour voir le test compiler et échouer.

<!--
### Implementing the `search_case_insensitive` Function
-->

### Implémenter la fonction `search_case_insensitive`

<!--
The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we'll lowercase
the `query` and each `line` so that whatever the case of the input arguments,
they'll be the same case when we check whether the line contains the query.
-->

La fonction `search_case_insensitive`, illustrée dans l'encart 12-21, sera presque identique à la fonction `search`. La seule différence est que nous mettrons en minuscules la `query` et chaque `line` de sorte que, quelle que soit la casse des arguments d'entrée, ils seront dans la même casse lorsque nous vérifierons si la ligne contient la requête.

<Listing number="12-21" file-name="src/lib.rs" caption="Définition de la fonction `search_case_insensitive` pour mettre en minuscules la requête et la ligne avant de les comparer">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

<!--
First, we lowercase the `query` string and store it in a new variable with the
same name, shadowing the original `query`. Calling `to_lowercase` on the query
is necessary so that no matter whether the user's query is `"rust"`, `"RUST"`,
`"Rust"`, or `"rUsT"`, we'll treat the query as if it were `"rust"` and be
insensitive to the case. While `to_lowercase` will handle basic Unicode, it
won't be 100 percent accurate. If we were writing a real application, we'd want
to do a bit more work here, but this section is about environment variables,
not Unicode, so we'll leave it at that here.
-->

Tout d'abord, nous mettons en minuscules la chaîne `query` et la stockons dans une nouvelle variable du même nom, masquant la `query` originale. Appeler `to_lowercase` sur la requête est nécessaire pour que, quelle que soit la requête de l'utilisateur — `"rust"`, `"RUST"`, `"Rust"` ou `"rUsT"` — nous traitions la requête comme si elle était `"rust"` et soyons insensibles à la casse. Bien que `to_lowercase` gère l'Unicode de base, ce ne sera pas précis à 100 %. Si nous écrivions une vraie application, nous voudrions faire un peu plus de travail ici, mais cette section porte sur les variables d'environnement, pas sur l'Unicode, donc nous en resterons là.

<!--
Note that `query` is now a `String` rather than a string slice because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: That string slice doesn't contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.
-->

Notez que `query` est maintenant un `String` plutôt qu'une tranche de chaîne, car l'appel à `to_lowercase` crée de nouvelles données plutôt que de référencer des données existantes. Prenons l'exemple de la requête `"rUsT"` : cette tranche de chaîne ne contient pas de `u` ou de `t` minuscule que nous pourrions utiliser, nous devons donc allouer un nouveau `String` contenant `"rust"`. Lorsque nous passons `query` comme argument à la méthode `contains` maintenant, nous devons ajouter une esperluette car la signature de `contains` est définie pour prendre une tranche de chaîne.

<!--
Next, we add a call to `to_lowercase` on each `line` to lowercase all
characters. Now that we've converted `line` and `query` to lowercase, we'll
find matches no matter what the case of the query is.
-->

Ensuite, nous ajoutons un appel à `to_lowercase` sur chaque `line` pour mettre tous les caractères en minuscules. Maintenant que nous avons converti `line` et `query` en minuscules, nous trouverons des correspondances quelle que soit la casse de la requête.

<!--
Let's see if this implementation passes the tests:
-->

Voyons si cette implémentation passe les tests :


```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

<!--
Great! They passed. Now let's call the new `search_case_insensitive` function
from the `run` function. First, we'll add a configuration option to the `Config`
struct to switch between case-sensitive and case-insensitive search. Adding
this field will cause compiler errors because we aren't initializing this field
anywhere yet:
-->

Parfait ! Ils sont passés. Maintenant, appelons la nouvelle fonction `search_case_insensitive` depuis la fonction `run`. Tout d'abord, nous ajouterons une option de configuration à la structure `Config` pour basculer entre la recherche sensible et insensible à la casse. L'ajout de ce champ provoquera des erreurs de compilation car nous n'initialisons ce champ nulle part pour le moment :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

<!--
We added the `ignore_case` field that holds a Boolean. Next, we need the `run`
function to check the `ignore_case` field's value and use that to decide
whether to call the `search` function or the `search_case_insensitive`
function, as shown in Listing 12-22. This still won't compile yet.
-->

Nous avons ajouté le champ `ignore_case` qui contient un booléen. Ensuite, nous avons besoin que la fonction `run` vérifie la valeur du champ `ignore_case` et l'utilise pour décider s'il faut appeler la fonction `search` ou la fonction `search_case_insensitive`, comme illustré dans l'encart 12-22. Cela ne compilera toujours pas encore.

<Listing number="12-22" file-name="src/main.rs" caption="Appel de `search` ou `search_case_insensitive` selon la valeur de `config.ignore_case`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

<!--
Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, which is already in scope at the top of _src/main.rs_. We'll use the
`var` function from the `env` module to check to see if any value has been set
for an environment variable named `IGNORE_CASE`, as shown in Listing 12-23.
-->

Enfin, nous devons vérifier la variable d'environnement. Les fonctions pour travailler avec les variables d'environnement sont dans le module `env` de la bibliothèque standard, qui est déjà dans la portée en haut de _src/main.rs_. Nous utiliserons la fonction `var` du module `env` pour vérifier si une valeur a été définie pour une variable d'environnement nommée `IGNORE_CASE`, comme illustré dans l'encart 12-23.

<Listing number="12-23" file-name="src/main.rs" caption="Vérification de toute valeur dans une variable d'environnement nommée `IGNORE_CASE`">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

<!--
Here, we create a new variable, `ignore_case`. To set its value, we call the
`env::var` function and pass it the name of the `IGNORE_CASE` environment
variable. The `env::var` function returns a `Result` that will be the
successful `Ok` variant that contains the value of the environment variable if
the environment variable is set to any value. It will return the `Err` variant
if the environment variable is not set.
-->

Ici, nous créons une nouvelle variable, `ignore_case`. Pour définir sa valeur, nous appelons la fonction `env::var` et lui passons le nom de la variable d'environnement `IGNORE_CASE`. La fonction `env::var` renvoie un `Result` qui sera le variant `Ok` de succès contenant la valeur de la variable d'environnement si celle-ci est définie à une quelconque valeur. Elle renverra le variant `Err` si la variable d'environnement n'est pas définie.

<!--
We're using the `is_ok` method on the `Result` to check whether the environment
variable is set, which means the program should do a case-insensitive search.
If the `IGNORE_CASE` environment variable isn't set to anything, `is_ok` will
return `false` and the program will perform a case-sensitive search. We don't
care about the _value_ of the environment variable, just whether it's set or
unset, so we're checking `is_ok` rather than using `unwrap`, `expect`, or any
of the other methods we've seen on `Result`.
-->

Nous utilisons la méthode `is_ok` sur le `Result` pour vérifier si la variable d'environnement est définie, ce qui signifie que le programme devrait effectuer une recherche insensible à la casse. Si la variable d'environnement `IGNORE_CASE` n'est définie à rien, `is_ok` renverra `false` et le programme effectuera une recherche sensible à la casse. Nous ne nous soucions pas de la _valeur_ de la variable d'environnement, juste de savoir si elle est définie ou non, donc nous vérifions `is_ok` plutôt que d'utiliser `unwrap`, `expect` ou l'une des autres méthodes que nous avons vues sur `Result`.

<!--
We pass the value in the `ignore_case` variable to the `Config` instance so
that the `run` function can read that value and decide whether to call
`search_case_insensitive` or `search`, as we implemented in Listing 12-22.
-->

Nous passons la valeur de la variable `ignore_case` à l'instance de `Config` afin que la fonction `run` puisse lire cette valeur et décider s'il faut appeler `search_case_insensitive` ou `search`, comme nous l'avons implémenté dans l'encart 12-22.

<!--
Let's give it a try! First, we'll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word _to_ in all lowercase:
-->

Essayons ! D'abord, nous exécuterons notre programme sans la variable d'environnement définie et avec la requête `to`, qui devrait correspondre à toute ligne contenant le mot _to_ entièrement en minuscules :


```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

<!--
Looks like that still works! Now let's run the program with `IGNORE_CASE` set
to `1` but with the same query `to`:
-->

Il semblerait que ça fonctionne toujours ! Maintenant, exécutons le programme avec `IGNORE_CASE` défini à `1` mais avec la même requête `to` :

<!--
```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```
-->

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

<!--
If you're using PowerShell, you will need to set the environment variable and
run the program as separate commands:
-->

Si vous utilisez PowerShell, vous devrez définir la variable d'environnement et exécuter le programme comme des commandes séparées :

<!--
```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```
-->

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

<!--
This will make `IGNORE_CASE` persist for the remainder of your shell session.
It can be unset with the `Remove-Item` cmdlet:
-->

Cela fera persister `IGNORE_CASE` pour le reste de votre session shell. Elle peut être supprimée avec le cmdlet `Remove-Item` :

<!--
```console
PS> Remove-Item Env:IGNORE_CASE
```
-->

```console
PS> Remove-Item Env:IGNORE_CASE
```

<!--
We should get lines that contain _to_ that might have uppercase letters:
-->

Nous devrions obtenir des lignes contenant _to_ qui pourraient avoir des lettres majuscules :

<!--
manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

<!--
```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

<!--
Excellent, we also got lines containing _To_! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.
-->

Excellent, nous avons aussi obtenu des lignes contenant _To_ ! Notre programme `minigrep` peut maintenant effectuer une recherche insensible à la casse contrôlée par une variable d'environnement. Vous savez maintenant comment gérer des options définies en utilisant soit des arguments de ligne de commande, soit des variables d'environnement.

<!--
Some programs allow arguments _and_ environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case sensitivity
through either a command line argument or an environment variable. Decide
whether the command line argument or the environment variable should take
precedence if the program is run with one set to case sensitive and one set to
ignore case.
-->

Certains programmes permettent des arguments _et_ des variables d'environnement pour la même configuration. Dans ces cas, les programmes décident que l'un ou l'autre a la priorité. Pour un autre exercice de votre côté, essayez de contrôler la sensibilité à la casse via un argument de ligne de commande ou une variable d'environnement. Décidez si l'argument de ligne de commande ou la variable d'environnement devrait avoir la priorité si le programme est exécuté avec l'un défini sur sensible à la casse et l'autre sur insensible à la casse.

<!--
The `std::env` module contains many more useful features for dealing with
environment variables: Check out its documentation to see what is available.
-->

Le module `std::env` contient de nombreuses autres fonctionnalités utiles pour travailler avec les variables d'environnement : consultez sa documentation pour voir ce qui est disponible.
