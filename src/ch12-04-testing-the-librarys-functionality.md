<!--
Old headings. Do not remove or links may break.
-->
<a id="developing-the-librarys-functionality-with-test-driven-development"></a>

<!--
## Adding Functionality with Test-Driven Development
-->

## Ajouter des fonctionnalités avec le développement piloté par les tests

<!--
Now that we have the search logic in _src/lib.rs_ separate from the `main`
function, it's much easier to write tests for the core functionality of our
code. We can call functions directly with various arguments and check return
values without having to call our binary from the command line.
-->

Maintenant que la logique de recherche se trouve dans _src/lib.rs_, séparée de la fonction `main`, il est beaucoup plus facile d'écrire des tests pour les fonctionnalités principales de notre code. Nous pouvons appeler les fonctions directement avec divers arguments et vérifier les valeurs de retour sans avoir à appeler notre binaire depuis la ligne de commande.

<!--
In this section, we'll add the searching logic to the `minigrep` program using
the test-driven development (TDD) process with the following steps:

1. Write a test that fails and run it to make sure it fails for the reason you
   expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue
   to pass.
4. Repeat from step 1!
-->

Dans cette section, nous allons ajouter la logique de recherche au programme `minigrep` en utilisant le processus de développement piloté par les tests (TDD) avec les étapes suivantes :

1. Écrire un test qui échoue et l'exécuter pour s'assurer qu'il échoue pour la raison attendue.
2. Écrire ou modifier juste assez de code pour faire passer le nouveau test.
3. Refactoriser le code que vous venez d'ajouter ou de modifier et vous assurer que les tests continuent de passer.
4. Recommencer à l'étape 1 !

<!--
Though it's just one of many ways to write software, TDD can help drive code
design. Writing the test before you write the code that makes the test pass
helps maintain high test coverage throughout the process.
-->

Bien que ce ne soit qu'une des nombreuses façons d'écrire du logiciel, le TDD peut aider à orienter la conception du code. Écrire le test avant d'écrire le code qui fait passer le test aide à maintenir une couverture de test élevée tout au long du processus.

<!--
We'll test-drive the implementation of the functionality that will actually do
the searching for the query string in the file contents and produce a list of
lines that match the query. We'll add this functionality in a function called
`search`.
-->

Nous allons piloter par les tests l'implémentation de la fonctionnalité qui effectuera réellement la recherche de la chaîne de requête dans le contenu du fichier et produira une liste de lignes correspondant à la requête. Nous ajouterons cette fonctionnalité dans une fonction appelée `search`.

<!--
### Writing a Failing Test
-->

### Écrire un test qui échoue

<!--
In _src/lib.rs_, we'll add a `tests` module with a test function, as we did in
[Chapter 11][ch11-anatomy] ignore
-->. The test function specifies the
behavior we want the `search` function to have: It will take a query and the
text to search, and it will return only the lines from the text that contain
the query. Listing 12-15 shows this test.
-->

Dans _src/lib.rs_, nous ajouterons un module `tests` avec une fonction de test, comme nous l'avons fait dans le [Chapitre 11][ch11-anatomy]<!--
ignore
-->. La fonction de test spécifie le comportement que nous voulons que la fonction `search` ait : elle prendra une requête et le texte à rechercher, et ne renverra que les lignes du texte qui contiennent la requête. L'encart 12-15 montre ce test.

<Listing number="12-15" file-name="src/lib.rs" caption="Création d'un test qui échoue pour la fonction `search` avec la fonctionnalité que nous souhaitons avoir">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

<!--
This test searches for the string `"duct"`. The text we're searching is three
lines, only one of which contains `"duct"` (note that the backslash after the
opening double quote tells Rust not to put a newline character at the beginning
of the contents of this string literal). We assert that the value returned from
the `search` function contains only the line we expect.
-->

Ce test recherche la chaîne `"duct"`. Le texte dans lequel nous cherchons fait trois lignes, dont une seule contient `"duct"` (notez que la barre oblique inverse après le guillemet double ouvrant indique à Rust de ne pas mettre de caractère de nouvelle ligne au début du contenu de ce littéral de chaîne). Nous vérifions que la valeur renvoyée par la fonction `search` contient uniquement la ligne attendue.

<!--
If we run this test, it will currently fail because the `unimplemented!` macro
panics with the message "not implemented". In accordance with TDD principles,
we'll take a small step of adding just enough code to get the test to not panic
when calling the function by defining the `search` function to always return an
empty vector, as shown in Listing 12-16. Then, the test should compile and fail
because an empty vector doesn't match a vector containing the line `"safe,
fast, productive."`.
-->

Si nous exécutons ce test, il échouera actuellement car la macro `unimplemented!` panique avec le message "not implemented". Conformément aux principes du TDD, nous ferons un petit pas en ajoutant juste assez de code pour que le test ne panique pas lors de l'appel de la fonction, en définissant la fonction `search` pour qu'elle renvoie toujours un vecteur vide, comme illustré dans l'encart 12-16. Ensuite, le test devrait compiler et échouer car un vecteur vide ne correspond pas à un vecteur contenant la ligne `"safe, fast, productive."`.

<Listing number="12-16" file-name="src/lib.rs" caption="Définition du minimum de la fonction `search` pour que son appel ne panique pas">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

<!--
Now let's discuss why we need to define an explicit lifetime `'a` in the
signature of `search` and use that lifetime with the `contents` argument and
the return value. Recall in [Chapter 10][ch10-lifetimes] ignore
--> that
the lifetime parameters specify which argument lifetime is connected to the
lifetime of the return value. In this case, we indicate that the returned
vector should contain string slices that reference slices of the argument
`contents` (rather than the argument `query`).
-->

Voyons maintenant pourquoi nous devons définir une durée de vie explicite `'a` dans la signature de `search` et utiliser cette durée de vie avec l'argument `contents` et la valeur de retour. Rappelez-vous que dans le [Chapitre 10][ch10-lifetimes]<!--
ignore
-->, les paramètres de durée de vie spécifient quelle durée de vie d'argument est connectée à la durée de vie de la valeur de retour. Dans ce cas, nous indiquons que le vecteur renvoyé doit contenir des tranches de chaîne qui référencent des tranches de l'argument `contents` (plutôt que de l'argument `query`).

<!--
In other words, we tell Rust that the data returned by the `search` function
will live as long as the data passed into the `search` function in the
`contents` argument. This is important! The data referenced _by_ a slice needs
to be valid for the reference to be valid; if the compiler assumes we're making
string slices of `query` rather than `contents`, it will do its safety checking
incorrectly.
-->

Autrement dit, nous indiquons à Rust que les données renvoyées par la fonction `search` vivront aussi longtemps que les données passées à la fonction `search` dans l'argument `contents`. C'est important ! Les données référencées _par_ une tranche doivent être valides pour que la référence soit valide ; si le compilateur suppose que nous créons des tranches de chaîne de `query` plutôt que de `contents`, il effectuera ses vérifications de sécurité de manière incorrecte.

<!--
If we forget the lifetime annotations and try to compile this function, we'll
get this error:
-->

Si nous oublions les annotations de durée de vie et essayons de compiler cette fonction, nous obtiendrons cette erreur :


```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

<!--
Rust can't know which of the two parameters we need for the output, so we need
to tell it explicitly. Note that the help text suggests specifying the same
lifetime parameter for all the parameters and the output type, which is
incorrect! Because `contents` is the parameter that contains all of our text
and we want to return the parts of that text that match, we know `contents` is
the only parameter that should be connected to the return value using the
lifetime syntax.
-->

Rust ne peut pas savoir lequel des deux paramètres nous avons besoin pour la sortie, nous devons donc le lui indiquer explicitement. Notez que le texte d'aide suggère de spécifier le même paramètre de durée de vie pour tous les paramètres et le type de sortie, ce qui est incorrect ! Comme `contents` est le paramètre qui contient tout notre texte et que nous voulons renvoyer les parties de ce texte qui correspondent, nous savons que `contents` est le seul paramètre qui doit être connecté à la valeur de retour en utilisant la syntaxe de durée de vie.

<!--
Other programming languages don't require you to connect arguments to return
values in the signature, but this practice will get easier over time. You might
want to compare this example with the examples in the ["Validating References
with Lifetimes"][validating-references-with-lifetimes] ignore
--> section
in Chapter 10.
-->

D'autres langages de programmation ne vous obligent pas à connecter les arguments aux valeurs de retour dans la signature, mais cette pratique deviendra plus facile avec le temps. Vous voudrez peut-être comparer cet exemple avec les exemples de la section [« Valider les références avec les durées de vie »][validating-references-with-lifetimes]<!--
ignore
--> du Chapitre 10.

<!--
### Writing Code to Pass the Test
-->

### Écrire du code pour faire passer le test

<!--
Currently, our test is failing because we always return an empty vector. To fix
that and implement `search`, our program needs to follow these steps:

1. Iterate through each line of the contents.
2. Check whether the line contains our query string.
3. If it does, add it to the list of values we're returning.
4. If it doesn't, do nothing.
5. Return the list of results that match.
-->

Actuellement, notre test échoue car nous renvoyons toujours un vecteur vide. Pour corriger cela et implémenter `search`, notre programme doit suivre ces étapes :

1. Itérer sur chaque ligne du contenu.
2. Vérifier si la ligne contient notre chaîne de requête.
3. Si c'est le cas, l'ajouter à la liste des valeurs que nous renvoyons.
4. Si ce n'est pas le cas, ne rien faire.
5. Renvoyer la liste des résultats correspondants.

<!--
Let's work through each step, starting with iterating through lines.
-->

Travaillons sur chaque étape, en commençant par l'itération sur les lignes.

<!--
#### Iterating Through Lines with the `lines` Method
-->

#### Itérer sur les lignes avec la méthode `lines`

<!--
Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works as shown in Listing 12-17. Note that
this won't compile yet.
-->

Rust dispose d'une méthode utile pour gérer l'itération ligne par ligne des chaînes de caractères, judicieusement nommée `lines`, qui fonctionne comme illustré dans l'encart 12-17. Notez que cela ne compilera pas encore.

<Listing number="12-17" file-name="src/lib.rs" caption="Itération sur chaque ligne de `contents`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

<!--
The `lines` method returns an iterator. We'll talk about iterators in depth in
[Chapter 13][ch13-iterators] ignore
-->. But recall that you saw this way
of using an iterator in [Listing 3-5][ch3-iter]<!--
ignore
-->, where we used a
`for` loop with an iterator to run some code on each item in a collection.
-->

La méthode `lines` renvoie un itérateur. Nous parlerons des itérateurs en profondeur dans le [Chapitre 13][ch13-iterators]<!--
ignore
-->. Mais rappelez-vous que vous avez vu cette façon d'utiliser un itérateur dans l'[encart 3-5][ch3-iter]<!--
ignore
-->, où nous avons utilisé une boucle `for` avec un itérateur pour exécuter du code sur chaque élément d'une collection.

<!--
#### Searching Each Line for the Query
-->

#### Rechercher la requête dans chaque ligne

<!--
Next, we'll check whether the current line contains our query string.
Fortunately, strings have a helpful method named `contains` that does this for
us! Add a call to the `contains` method in the `search` function, as shown in
Listing 12-18. Note that this still won't compile yet.
-->

Ensuite, nous vérifierons si la ligne courante contient notre chaîne de requête. Heureusement, les chaînes de caractères ont une méthode utile nommée `contains` qui fait cela pour nous ! Ajoutez un appel à la méthode `contains` dans la fonction `search`, comme illustré dans l'encart 12-18. Notez que cela ne compilera toujours pas encore.

<Listing number="12-18" file-name="src/lib.rs" caption="Ajout de la fonctionnalité pour voir si la ligne contient la chaîne de `query`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

<!--
At the moment, we're building up functionality. To get the code to compile, we
need to return a value from the body as we indicated we would in the function
signature.
-->

Pour le moment, nous construisons la fonctionnalité progressivement. Pour que le code compile, nous devons renvoyer une valeur depuis le corps de la fonction comme nous l'avons indiqué dans la signature de la fonction.

<!--
#### Storing Matching Lines
-->

#### Stocker les lignes correspondantes

<!--
To finish this function, we need a way to store the matching lines that we want
to return. For that, we can make a mutable vector before the `for` loop and
call the `push` method to store a `line` in the vector. After the `for` loop,
we return the vector, as shown in Listing 12-19.
-->

Pour terminer cette fonction, nous avons besoin d'un moyen de stocker les lignes correspondantes que nous voulons renvoyer. Pour cela, nous pouvons créer un vecteur mutable avant la boucle `for` et appeler la méthode `push` pour stocker une `line` dans le vecteur. Après la boucle `for`, nous renvoyons le vecteur, comme illustré dans l'encart 12-19.

<Listing number="12-19" file-name="src/lib.rs" caption="Stockage des lignes correspondantes pour pouvoir les renvoyer">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

<!--
Now the `search` function should return only the lines that contain `query`,
and our test should pass. Let's run the test:
-->

Maintenant, la fonction `search` ne devrait renvoyer que les lignes qui contiennent `query`, et notre test devrait passer. Exécutons le test :


```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

<!--
Our test passed, so we know it works!
-->

Notre test est passé, donc nous savons que ça fonctionne !

<!--
At this point, we could consider opportunities for refactoring the
implementation of the search function while keeping the tests passing to
maintain the same functionality. The code in the search function isn't too bad,
but it doesn't take advantage of some useful features of iterators. We'll
return to this example in [Chapter 13][ch13-iterators] ignore
-->, where
we'll explore iterators in detail, and look at how to improve it.
-->

À ce stade, nous pourrions envisager des opportunités de refactorisation de l'implémentation de la fonction de recherche tout en gardant les tests passants pour maintenir la même fonctionnalité. Le code de la fonction de recherche n'est pas trop mal, mais il ne tire pas parti de certaines fonctionnalités utiles des itérateurs. Nous reviendrons à cet exemple dans le [Chapitre 13][ch13-iterators]<!--
ignore
-->, où nous explorerons les itérateurs en détail, et verrons comment l'améliorer.

<!--
Now the entire program should work! Let's try it out, first with a word that
should return exactly one line from the Emily Dickinson poem: _frog_.
-->

Maintenant, le programme entier devrait fonctionner ! Essayons-le, d'abord avec un mot qui devrait renvoyer exactement une ligne du poème d'Emily Dickinson : _frog_.


```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

<!--
Cool! Now let's try a word that will match multiple lines, like _body_:
-->

Super ! Essayons maintenant un mot qui correspondra à plusieurs lignes, comme _body_ :


```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

<!--
And finally, let's make sure that we don't get any lines when we search for a
word that isn't anywhere in the poem, such as _monomorphization_:
-->

Et enfin, assurons-nous que nous n'obtenons aucune ligne lorsque nous recherchons un mot qui n'apparaît nulle part dans le poème, comme _monomorphization_ :


```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

<!--
Excellent! We've built our own mini version of a classic tool and learned a lot
about how to structure applications. We've also learned a bit about file input
and output, lifetimes, testing, and command line parsing.
-->

Excellent ! Nous avons construit notre propre mini version d'un outil classique et avons beaucoup appris sur la structuration des applications. Nous avons aussi appris un peu sur les entrées/sorties de fichiers, les durées de vie, les tests et l'analyse de la ligne de commande.

<!--
To round out this project, we'll briefly demonstrate how to work with
environment variables and how to print to standard error, both of which are
useful when you're writing command line programs.
-->

Pour compléter ce projet, nous montrerons brièvement comment travailler avec les variables d'environnement et comment écrire sur la sortie d'erreur standard, deux choses utiles lorsque vous écrivez des programmes en ligne de commande.

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
