<!--
## Refactoring to Improve Modularity and Error Handling
-->

## Refactoriser pour améliorer la modularité et la gestion des erreurs

<!--
To improve our program, we'll fix four problems that have to do with the
program's structure and how it's handling potential errors. First, our `main`
function now performs two tasks: It parses arguments and reads files. As our
program grows, the number of separate tasks the `main` function handles will
increase. As a function gains responsibilities, it becomes more difficult to
reason about, harder to test, and harder to change without breaking one of its
parts. It's best to separate functionality so that each function is responsible
for one task.
-->

Pour améliorer notre programme, nous allons corriger quatre problèmes liés à la structure du programme et à la façon dont il gère les erreurs potentielles. Premièrement, notre fonction `main` effectue désormais deux tâches : elle analyse les arguments et lit les fichiers. À mesure que notre programme grandit, le nombre de tâches distinctes que la fonction `main` gère augmentera. Quand une fonction accumule les responsabilités, elle devient plus difficile à comprendre, plus difficile à tester et plus difficile à modifier sans casser l'une de ses parties. Il est préférable de séparer les fonctionnalités de sorte que chaque fonction soit responsable d'une seule tâche.

<!--
This issue also ties into the second problem: Although `query` and `file_path`
are configuration variables to our program, variables like `contents` are used
to perform the program's logic. The longer `main` becomes, the more variables
we'll need to bring into scope; the more variables we have in scope, the harder
it will be to keep track of the purpose of each. It's best to group the
configuration variables into one structure to make their purpose clear.
-->

Ce problème est aussi lié au deuxième : bien que `query` et `file_path` soient des variables de configuration de notre programme, des variables comme `contents` sont utilisées pour exécuter la logique du programme. Plus `main` s'allonge, plus nous aurons de variables à mettre en portée ; plus nous avons de variables en portée, plus il sera difficile de suivre l'objectif de chacune. Il est préférable de regrouper les variables de configuration dans une seule structure pour clarifier leur rôle.

<!--
The third problem is that we've used `expect` to print an error message when
reading the file fails, but the error message just prints `Should have been
able to read the file`. Reading a file can fail in a number of ways: For
example, the file could be missing, or we might not have permission to open it.
Right now, regardless of the situation, we'd print the same error message for
everything, which wouldn't give the user any information!
-->

Le troisième problème est que nous avons utilisé `expect` pour afficher un message d'erreur lorsque la lecture du fichier échoue, mais le message d'erreur affiche simplement `Should have been able to read the file`. La lecture d'un fichier peut échouer de nombreuses façons : par exemple, le fichier pourrait être manquant, ou nous pourrions ne pas avoir la permission de l'ouvrir. Actuellement, quelle que soit la situation, nous afficherions le même message d'erreur pour tout, ce qui ne donnerait aucune information à l'utilisateur !

<!--
Fourth, we use `expect` to handle an error, and if the user runs our program
without specifying enough arguments, they'll get an `index out of bounds` error
from Rust that doesn't clearly explain the problem. It would be best if all the
error-handling code were in one place so that future maintainers had only one
place to consult the code if the error-handling logic needed to change. Having
all the error-handling code in one place will also ensure that we're printing
messages that will be meaningful to our end users.
-->

Quatrièmement, nous utilisons `expect` pour gérer une erreur, et si l'utilisateur exécute notre programme sans spécifier suffisamment d'arguments, il obtiendra une erreur `index out of bounds` de Rust qui n'explique pas clairement le problème. Il serait préférable que tout le code de gestion des erreurs soit au même endroit, de sorte que les futurs mainteneurs n'aient qu'un seul endroit à consulter si la logique de gestion des erreurs devait changer. Avoir tout le code de gestion des erreurs au même endroit garantira également que nous affichons des messages significatifs pour nos utilisateurs finaux.

<!--
Let's address these four problems by refactoring our project.
-->

Résolvons ces quatre problèmes en refactorisant notre projet.

<!--
Old headings. Do not remove or links may break.
-->

<a id="separation-of-concerns-for-binary-projects"></a>

<!--
### Separating Concerns in Binary Projects
-->

### Séparer les responsabilités dans les projets binaires

<!--
The organizational problem of allocating responsibility for multiple tasks to
the `main` function is common to many binary projects. As a result, many Rust
programmers find it useful to split up the separate concerns of a binary
program when the `main` function starts getting large. This process has the
following steps:

- Split your program into a _main.rs_ file and a _lib.rs_ file and move your
  program's logic to _lib.rs_.
- As long as your command line parsing logic is small, it can remain in
  the `main` function.
- When the command line parsing logic starts getting complicated, extract it
  from the `main` function into other functions or types.
-->

Le problème organisationnel consistant à attribuer la responsabilité de plusieurs tâches à la fonction `main` est commun à de nombreux projets binaires. Par conséquent, de nombreux programmeurs Rust trouvent utile de séparer les différentes responsabilités d'un programme binaire lorsque la fonction `main` commence à devenir volumineuse. Ce processus comprend les étapes suivantes :

- Diviser votre programme en un fichier _main.rs_ et un fichier _lib.rs_, et déplacer la logique de votre programme dans _lib.rs_.
- Tant que votre logique d'analyse de la ligne de commande est petite, elle peut rester dans la fonction `main`.
- Lorsque la logique d'analyse de la ligne de commande commence à devenir complexe, l'extraire de la fonction `main` dans d'autres fonctions ou types.

<!--
The responsibilities that remain in the `main` function after this process
should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a `run` function in _lib.rs_
- Handling the error if `run` returns an error
-->

Les responsabilités qui restent dans la fonction `main` après ce processus devraient se limiter aux suivantes :

- Appeler la logique d'analyse de la ligne de commande avec les valeurs des arguments
- Mettre en place toute autre configuration
- Appeler une fonction `run` dans _lib.rs_
- Gérer l'erreur si `run` renvoie une erreur

<!--
This pattern is about separating concerns: _main.rs_ handles running the
program and _lib.rs_ handles all the logic of the task at hand. Because you
can't test the `main` function directly, this structure lets you test all of
your program's logic by moving it out of the `main` function. The code that
remains in the `main` function will be small enough to verify its correctness
by reading it. Let's rework our program by following this process.
-->

Ce patron consiste à séparer les responsabilités : _main.rs_ gère l'exécution du programme et _lib.rs_ gère toute la logique de la tâche en cours. Comme vous ne pouvez pas tester directement la fonction `main`, cette structure vous permet de tester toute la logique de votre programme en la déplaçant hors de la fonction `main`. Le code qui reste dans la fonction `main` sera suffisamment petit pour vérifier son exactitude en le lisant. Retravaillons notre programme en suivant ce processus.

<!--
#### Extracting the Argument Parser
-->

#### Extraire l'analyseur d'arguments

<!--
We'll extract the functionality for parsing arguments into a function that
`main` will call. Listing 12-5 shows the new start of the `main` function that
calls a new function `parse_config`, which we'll define in _src/main.rs_.
-->

Nous allons extraire la fonctionnalité d'analyse des arguments dans une fonction que `main` appellera. L'encart 12-5 montre le nouveau début de la fonction `main` qui appelle une nouvelle fonction `parse_config`, que nous définirons dans _src/main.rs_.

<Listing number="12-5" file-name="src/main.rs" caption="Extraction d'une fonction `parse_config` depuis `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

<!--
We're still collecting the command line arguments into a vector, but instead of
assigning the argument value at index 1 to the variable `query` and the
argument value at index 2 to the variable `file_path` within the `main`
function, we pass the whole vector to the `parse_config` function. The
`parse_config` function then holds the logic that determines which argument
goes in which variable and passes the values back to `main`. We still create
the `query` and `file_path` variables in `main`, but `main` no longer has the
responsibility of determining how the command line arguments and variables
correspond.
-->

Nous collectons toujours les arguments de ligne de commande dans un vecteur, mais au lieu d'assigner la valeur de l'argument à l'index 1 à la variable `query` et la valeur de l'argument à l'index 2 à la variable `file_path` dans la fonction `main`, nous passons le vecteur entier à la fonction `parse_config`. La fonction `parse_config` contient alors la logique qui détermine quel argument va dans quelle variable et renvoie les valeurs à `main`. Nous créons toujours les variables `query` et `file_path` dans `main`, mais `main` n'a plus la responsabilité de déterminer comment les arguments de ligne de commande et les variables correspondent.

<!--
This rework may seem like overkill for our small program, but we're refactoring
in small, incremental steps. After making this change, run the program again to
verify that the argument parsing still works. It's good to check your progress
often, to help identify the cause of problems when they occur.
-->

Cette refonte peut sembler excessive pour notre petit programme, mais nous refactorisons par petites étapes incrémentales. Après avoir effectué ce changement, exécutez à nouveau le programme pour vérifier que l'analyse des arguments fonctionne toujours. Il est bon de vérifier vos progrès fréquemment, pour aider à identifier la cause des problèmes lorsqu'ils surviennent.

<!--
#### Grouping Configuration Values
-->

#### Regrouper les valeurs de configuration

<!--
We can take another small step to improve the `parse_config` function further.
At the moment, we're returning a tuple, but then we immediately break that
tuple into individual parts again. This is a sign that perhaps we don't have
the right abstraction yet.
-->

Nous pouvons faire un autre petit pas pour améliorer davantage la fonction `parse_config`. Pour le moment, nous renvoyons un tuple, mais ensuite nous décomposons immédiatement ce tuple en parties individuelles. C'est un signe que nous n'avons peut-être pas encore la bonne abstraction.

<!--
Another indicator that shows there's room for improvement is the `config` part
of `parse_config`, which implies that the two values we return are related and
are both part of one configuration value. We're not currently conveying this
meaning in the structure of the data other than by grouping the two values into
a tuple; we'll instead put the two values into one struct and give each of the
struct fields a meaningful name. Doing so will make it easier for future
maintainers of this code to understand how the different values relate to each
other and what their purpose is.
-->

Un autre indicateur qui montre qu'il y a une marge d'amélioration est la partie `config` de `parse_config`, qui implique que les deux valeurs que nous renvoyons sont liées et font toutes deux partie d'une seule valeur de configuration. Nous ne transmettons pas actuellement cette signification dans la structure des données autrement qu'en regroupant les deux valeurs dans un tuple ; nous allons plutôt placer les deux valeurs dans une structure et donner à chaque champ de la structure un nom significatif. Cela permettra aux futurs mainteneurs de ce code de comprendre plus facilement comment les différentes valeurs sont liées entre elles et quel est leur rôle.

<!--
Listing 12-6 shows the improvements to the `parse_config` function.
-->

L'encart 12-6 montre les améliorations apportées à la fonction `parse_config`.

<Listing number="12-6" file-name="src/main.rs" caption="Refactorisation de `parse_config` pour renvoyer une instance d'une structure `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

<!--
We've added a struct named `Config` defined to have fields named `query` and
`file_path`. The signature of `parse_config` now indicates that it returns a
`Config` value. In the body of `parse_config`, where we used to return
string slices that reference `String` values in `args`, we now define `Config`
to contain owned `String` values. The `args` variable in `main` is the owner of
the argument values and is only letting the `parse_config` function borrow
them, which means we'd violate Rust's borrowing rules if `Config` tried to take
ownership of the values in `args`.
-->

Nous avons ajouté une structure nommée `Config` définie avec des champs nommés `query` et `file_path`. La signature de `parse_config` indique maintenant qu'elle renvoie une valeur `Config`. Dans le corps de `parse_config`, là où nous renvoyions auparavant des tranches de chaînes qui référençaient des valeurs `String` dans `args`, nous définissons maintenant `Config` pour contenir des valeurs `String` possédées. La variable `args` dans `main` est la propriétaire des valeurs d'arguments et ne fait que les prêter à la fonction `parse_config`, ce qui signifie que nous violerions les règles d'emprunt de Rust si `Config` essayait de prendre possession des valeurs dans `args`.

<!--
There are a number of ways we could manage the `String` data; the easiest,
though somewhat inefficient, route is to call the `clone` method on the values.
This will make a full copy of the data for the `Config` instance to own, which
takes more time and memory than storing a reference to the string data.
However, cloning the data also makes our code very straightforward because we
don't have to manage the lifetimes of the references; in this circumstance,
giving up a little performance to gain simplicity is a worthwhile trade-off.
-->

Il existe plusieurs façons de gérer les données `String` ; la plus simple, bien que quelque peu inefficace, est d'appeler la méthode `clone` sur les valeurs. Cela créera une copie complète des données que l'instance de `Config` pourra posséder, ce qui prend plus de temps et de mémoire que de stocker une référence aux données de la chaîne. Cependant, cloner les données rend aussi notre code très simple car nous n'avons pas à gérer les durées de vie des références ; dans ce contexte, sacrifier un peu de performance pour gagner en simplicité est un compromis qui en vaut la peine.

<!--
> ### The Trade-Offs of Using `clone`
>
> There's a tendency among many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In
> [Chapter 13][ch13] ignore
-->, you'll learn how to use more efficient
> methods in this type of situation. But for now, it's okay to copy a few
> strings to continue making progress because you'll make these copies only
> once and your file path and query string are very small. It's better to have
> a working program that's a bit inefficient than to try to hyperoptimize code
> on your first pass. As you become more experienced with Rust, it'll be
> easier to start with the most efficient solution, but for now, it's
> perfectly acceptable to call `clone`.
-->

> ### Les compromis de l'utilisation de `clone`
>
> Il y a une tendance parmi les Rustacés à éviter d'utiliser `clone` pour
> résoudre les problèmes de possession en raison de son coût à l'exécution.
> Dans le [Chapitre 13][ch13]<!--
ignore
-->, vous apprendrez à utiliser des
> méthodes plus efficaces dans ce type de situation. Mais pour l'instant, il
> est acceptable de copier quelques chaînes pour continuer à progresser, car
> vous ne ferez ces copies qu'une seule fois et votre chemin de fichier et
> votre chaîne de requête sont très petits. Il vaut mieux avoir un programme
> fonctionnel qui est un peu inefficace que d'essayer d'hyper-optimiser le code
> dès le premier passage. À mesure que vous acquerrez de l'expérience avec
> Rust, il sera plus facile de commencer par la solution la plus efficace,
> mais pour l'instant, il est tout à fait acceptable d'appeler `clone`.

<!--
We've updated `main` so that it places the instance of `Config` returned by
`parse_config` into a variable named `config`, and we updated the code that
previously used the separate `query` and `file_path` variables so that it now
uses the fields on the `Config` struct instead.
-->

Nous avons mis à jour `main` de sorte qu'il place l'instance de `Config` renvoyée par `parse_config` dans une variable nommée `config`, et nous avons mis à jour le code qui utilisait auparavant les variables séparées `query` et `file_path` pour qu'il utilise désormais les champs de la structure `Config`.

<!--
Now our code more clearly conveys that `query` and `file_path` are related and
that their purpose is to configure how the program will work. Any code that
uses these values knows to find them in the `config` instance in the fields
named for their purpose.
-->

Maintenant, notre code exprime plus clairement que `query` et `file_path` sont liés et que leur objectif est de configurer le fonctionnement du programme. Tout code qui utilise ces valeurs sait qu'il les trouvera dans l'instance `config`, dans les champs nommés selon leur fonction.

<!--
#### Creating a Constructor for `Config`
-->

#### Créer un constructeur pour `Config`

<!--
So far, we've extracted the logic responsible for parsing the command line
arguments from `main` and placed it in the `parse_config` function. Doing so
helped us see that the `query` and `file_path` values were related, and that
relationship should be conveyed in our code. We then added a `Config` struct to
name the related purpose of `query` and `file_path` and to be able to return the
values' names as struct field names from the `parse_config` function.
-->

Jusqu'ici, nous avons extrait la logique responsable de l'analyse des arguments de ligne de commande de `main` et l'avons placée dans la fonction `parse_config`. Cela nous a aidés à voir que les valeurs `query` et `file_path` étaient liées, et que cette relation devait être exprimée dans notre code. Nous avons ensuite ajouté une structure `Config` pour nommer la relation entre `query` et `file_path` et pouvoir renvoyer les noms des valeurs comme noms de champs de la structure depuis la fonction `parse_config`.

<!--
So, now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from a plain function to a function
named `new` that is associated with the `Config` struct. Making this change
will make the code more idiomatic. We can create instances of types in the
standard library, such as `String`, by calling `String::new`. Similarly, by
changing `parse_config` into a `new` function associated with `Config`, we'll
be able to create instances of `Config` by calling `Config::new`. Listing 12-7
shows the changes we need to make.
-->

Maintenant que l'objectif de la fonction `parse_config` est de créer une instance de `Config`, nous pouvons transformer `parse_config` d'une simple fonction en une fonction nommée `new` associée à la structure `Config`. Ce changement rendra le code plus idiomatique. Nous pouvons créer des instances de types de la bibliothèque standard, comme `String`, en appelant `String::new`. De même, en transformant `parse_config` en une fonction `new` associée à `Config`, nous pourrons créer des instances de `Config` en appelant `Config::new`. L'encart 12-7 montre les changements que nous devons apporter.

<Listing number="12-7" file-name="src/main.rs" caption="Transformation de `parse_config` en `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

<!--
We've updated `main` where we were calling `parse_config` to instead call
`Config::new`. We've changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.
-->

Nous avons mis à jour `main` là où nous appelions `parse_config` pour appeler à la place `Config::new`. Nous avons changé le nom de `parse_config` en `new` et l'avons déplacée dans un bloc `impl`, ce qui associe la fonction `new` à `Config`. Essayez de compiler ce code à nouveau pour vous assurer qu'il fonctionne.

<!--
### Fixing the Error Handling
-->

### Corriger la gestion des erreurs

<!--
Now we'll work on fixing our error handling. Recall that attempting to access
the values in the `args` vector at index 1 or index 2 will cause the program to
panic if the vector contains fewer than three items. Try running the program
without any arguments; it will look like this:
-->

Nous allons maintenant travailler à la correction de notre gestion des erreurs. Rappelez-vous que tenter d'accéder aux valeurs du vecteur `args` à l'index 1 ou à l'index 2 provoquera un panic du programme si le vecteur contient moins de trois éléments. Essayez d'exécuter le programme sans aucun argument ; voici ce que cela donnera :


```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

<!--
The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won't help our end users understand what
they should do instead. Let's fix that now.
-->

La ligne `index out of bounds: the len is 1 but the index is 1` est un message d'erreur destiné aux programmeurs. Il n'aidera pas nos utilisateurs finaux à comprendre ce qu'ils devraient faire à la place. Corrigeons cela maintenant.

<!--
#### Improving the Error Message
-->

#### Améliorer le message d'erreur

<!--
In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index 1 and index 2. If the slice isn't
long enough, the program panics and displays a better error message.
-->

Dans l'encart 12-8, nous ajoutons une vérification dans la fonction `new` qui vérifiera que la tranche est suffisamment longue avant d'accéder aux index 1 et 2. Si la tranche n'est pas assez longue, le programme panique et affiche un meilleur message d'erreur.

<Listing number="12-8" file-name="src/main.rs" caption="Ajout d'une vérification du nombre d'arguments">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

<!--
This code is similar to [the `Guess::new` function we wrote in Listing
9-13][ch9-custom-types] ignore
-->, where we called `panic!` when the
`value` argument was out of the range of valid values. Instead of checking for
a range of values here, we're checking that the length of `args` is at least
`3` and the rest of the function can operate under the assumption that this
condition has been met. If `args` has fewer than three items, this condition
will be `true`, and we call the `panic!` macro to end the program immediately.
-->

Ce code est similaire à [la fonction `Guess::new` que nous avons écrite dans l'encart 9-13][ch9-custom-types]<!--
ignore
-->, où nous appelions `panic!` lorsque l'argument `value` était en dehors de la plage de valeurs valides. Au lieu de vérifier une plage de valeurs ici, nous vérifions que la longueur de `args` est d'au moins `3` et le reste de la fonction peut fonctionner en supposant que cette condition est remplie. Si `args` contient moins de trois éléments, cette condition sera `true`, et nous appelons la macro `panic!` pour mettre fin au programme immédiatement.

<!--
With these extra few lines of code in `new`, let's run the program without any
arguments again to see what the error looks like now:
-->

Avec ces quelques lignes de code supplémentaires dans `new`, exécutons à nouveau le programme sans aucun argument pour voir à quoi ressemble l'erreur maintenant :


```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

<!--
This output is better: We now have a reasonable error message. However, we also
have extraneous information we don't want to give to our users. Perhaps the
technique we used in Listing 9-13 isn't the best one to use here: A call to
`panic!` is more appropriate for a programming problem than a usage problem,
[as discussed in Chapter 9][ch9-error-guidelines] ignore
-->. Instead,
we'll use the other technique you learned about in Chapter 9—[returning a
`Result`][ch9-result]<!--
ignore
--> that indicates either success or an error.
-->

Cette sortie est meilleure : nous avons maintenant un message d'erreur raisonnable. Cependant, nous avons aussi des informations superflues que nous ne voulons pas donner à nos utilisateurs. La technique que nous avons utilisée dans l'encart 9-13 n'est peut-être pas la meilleure à utiliser ici : un appel à `panic!` est plus approprié pour un problème de programmation que pour un problème d'utilisation, [comme discuté au Chapitre 9][ch9-error-guidelines]<!--
ignore
-->. À la place, nous utiliserons l'autre technique que vous avez apprise au Chapitre 9 — [renvoyer un `Result`][ch9-result]<!--
ignore
--> qui indique soit un succès, soit une erreur.

<!--
Old headings. Do not remove or links may break.
-->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

<!--
#### Returning a `Result` Instead of Calling `panic!`
-->

#### Renvoyer un `Result` au lieu d'appeler `panic!`

<!--
We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. We're also
going to change the function name from `new` to `build` because many
programmers expect `new` functions to never fail. When `Config::build` is
communicating to `main`, we can use the `Result` type to signal there was a
problem. Then, we can change `main` to convert an `Err` variant into a more
practical error for our users without the surrounding text about `thread
'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.
-->

Nous pouvons à la place renvoyer une valeur `Result` qui contiendra une instance de `Config` en cas de succès et décrira le problème en cas d'erreur. Nous allons aussi changer le nom de la fonction de `new` à `build` car de nombreux programmeurs s'attendent à ce que les fonctions `new` n'échouent jamais. Lorsque `Config::build` communique avec `main`, nous pouvons utiliser le type `Result` pour signaler qu'il y a eu un problème. Ensuite, nous pouvons modifier `main` pour convertir un variant `Err` en une erreur plus pratique pour nos utilisateurs, sans le texte environnant sur `thread 'main'` et `RUST_BACKTRACE` qu'un appel à `panic!` provoque.

<!--
Listing 12-9 shows the changes we need to make to the return value of the
function we're now calling `Config::build` and the body of the function needed
to return a `Result`. Note that this won't compile until we update `main` as
well, which we'll do in the next listing.
-->

L'encart 12-9 montre les changements que nous devons apporter à la valeur de retour de la fonction que nous appelons maintenant `Config::build` et au corps de la fonction pour renvoyer un `Result`. Notez que cela ne compilera pas tant que nous n'aurons pas aussi mis à jour `main`, ce que nous ferons dans le prochain encart.

<Listing number="12-9" file-name="src/main.rs" caption="Renvoyer un `Result` depuis `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

<!--
Our `build` function returns a `Result` with a `Config` instance in the success
case and a string literal in the error case. Our error values will always be
string literals that have the `'static` lifetime.
-->

Notre fonction `build` renvoie un `Result` avec une instance de `Config` en cas de succès et un littéral de chaîne en cas d'erreur. Nos valeurs d'erreur seront toujours des littéraux de chaîne qui ont la durée de vie `'static`.

<!--
We've made two changes in the body of the function: Instead of calling `panic!`
when the user doesn't pass enough arguments, we now return an `Err` value, and
we've wrapped the `Config` return value in an `Ok`. These changes make the
function conform to its new type signature.
-->

Nous avons apporté deux modifications au corps de la fonction : au lieu d'appeler `panic!` lorsque l'utilisateur ne passe pas assez d'arguments, nous renvoyons maintenant une valeur `Err`, et nous avons enveloppé la valeur de retour `Config` dans un `Ok`. Ces changements font que la fonction se conforme à sa nouvelle signature de type.

<!--
Returning an `Err` value from `Config::build` allows the `main` function to
handle the `Result` value returned from the `build` function and exit the
process more cleanly in the error case.
-->

Renvoyer une valeur `Err` depuis `Config::build` permet à la fonction `main` de gérer la valeur `Result` renvoyée par la fonction `build` et de quitter le processus plus proprement en cas d'erreur.

<!--
Old headings. Do not remove or links may break.
-->

<a id="calling-confignew-and-handling-errors"></a>

<!--
#### Calling `Config::build` and Handling Errors
-->

#### Appeler `Config::build` et gérer les erreurs

<!--
To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::build`, as shown in
Listing 12-10. We'll also take the responsibility of exiting the command line
tool with a nonzero error code away from `panic!` and instead implement it by
hand. A nonzero exit status is a convention to signal to the process that
called our program that the program exited with an error state.
-->

Pour gérer le cas d'erreur et afficher un message convivial, nous devons mettre à jour `main` pour gérer le `Result` renvoyé par `Config::build`, comme illustré dans l'encart 12-10. Nous prendrons également la responsabilité de quitter l'outil en ligne de commande avec un code d'erreur non nul, en l'implémentant nous-mêmes au lieu de laisser `panic!` s'en charger. Un code de sortie non nul est une convention pour signaler au processus qui a appelé notre programme que celui-ci s'est terminé dans un état d'erreur.

<Listing number="12-10" file-name="src/main.rs" caption="Quitter avec un code d'erreur si la construction d'un `Config` échoue">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

<!--
In this listing, we've used a method we haven't covered in detail yet:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method's behavior is similar
to `unwrap`: It returns the inner value that `Ok` is wrapping. However, if the
value is an `Err` value, this method calls the code in the closure, which is
an anonymous function we define and pass as an argument to `unwrap_or_else`.
We'll cover closures in more detail in [Chapter 13][ch13] ignore
-->. For
now, you just need to know that `unwrap_or_else` will pass the inner value of
the `Err`, which in this case is the static string `"not enough arguments"`
that we added in Listing 12-9, to our closure in the argument `err` that
appears between the vertical pipes. The code in the closure can then use the
`err` value when it runs.
-->

Dans cet encart, nous avons utilisé une méthode que nous n'avons pas encore couverte en détail : `unwrap_or_else`, qui est définie sur `Result<T, E>` par la bibliothèque standard. Utiliser `unwrap_or_else` nous permet de définir une gestion d'erreur personnalisée, sans `panic!`. Si le `Result` est une valeur `Ok`, le comportement de cette méthode est similaire à `unwrap` : elle renvoie la valeur interne que `Ok` enveloppe. Cependant, si la valeur est un `Err`, cette méthode appelle le code dans la fermeture, qui est une fonction anonyme que nous définissons et passons en argument à `unwrap_or_else`. Nous couvrirons les fermetures plus en détail dans le [Chapitre 13][ch13]<!--
ignore
-->. Pour l'instant, vous devez simplement savoir que `unwrap_or_else` passera la valeur interne de l'`Err`, qui dans ce cas est la chaîne statique `"not enough arguments"` que nous avons ajoutée dans l'encart 12-9, à notre fermeture dans l'argument `err` qui apparaît entre les barres verticales. Le code dans la fermeture peut alors utiliser la valeur `err` lorsqu'il s'exécute.

<!--
We've added a new `use` line to bring `process` from the standard library into
scope. The code in the closure that will be run in the error case is only two
lines: We print the `err` value and then call `process::exit`. The
`process::exit` function will stop the program immediately and return the
number that was passed as the exit status code. This is similar to the
`panic!`-based handling we used in Listing 12-8, but we no longer get all the
extra output. Let's try it:
-->

Nous avons ajouté une nouvelle ligne `use` pour importer `process` de la bibliothèque standard dans la portée. Le code dans la fermeture qui sera exécuté en cas d'erreur ne fait que deux lignes : nous affichons la valeur `err` puis appelons `process::exit`. La fonction `process::exit` arrêtera le programme immédiatement et renverra le nombre passé comme code de sortie. C'est similaire à la gestion basée sur `panic!` que nous avons utilisée dans l'encart 12-8, mais nous n'obtenons plus toute la sortie supplémentaire. Essayons :


```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

<!--
Great! This output is much friendlier for our users.
-->

Parfait ! Cette sortie est beaucoup plus conviviale pour nos utilisateurs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="extracting-logic-from-the-main-function"></a>

<!--
### Extracting Logic from `main`
-->

### Extraire la logique de `main`

<!--
Now that we've finished refactoring the configuration parsing, let's turn to
the program's logic. As we stated in ["Separating Concerns in Binary
Projects"](#separation-of-concerns-for-binary-projects) ignore
-->, we'll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn't involved with setting up configuration or handling
errors. When we're done, the `main` function will be concise and easy to verify
by inspection, and we'll be able to write tests for all the other logic.
-->

Maintenant que nous avons fini de refactoriser l'analyse de la configuration, tournons-nous vers la logique du programme. Comme nous l'avons indiqué dans [« Séparer les responsabilités dans les projets binaires »](#separation-of-concerns-for-binary-projects)<!--
ignore
-->, nous allons extraire une fonction nommée `run` qui contiendra toute la logique actuellement dans la fonction `main` qui n'est pas impliquée dans la mise en place de la configuration ou la gestion des erreurs. Une fois terminé, la fonction `main` sera concise et facile à vérifier par inspection, et nous pourrons écrire des tests pour toute la logique restante.

<!--
Listing 12-11 shows the small, incremental improvement of extracting a `run`
function.
-->

L'encart 12-11 montre la petite amélioration incrémentale consistant à extraire une fonction `run`.

<Listing number="12-11" file-name="src/main.rs" caption="Extraction d'une fonction `run` contenant le reste de la logique du programme">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

<!--
The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.
-->

La fonction `run` contient maintenant toute la logique restante de `main`, à partir de la lecture du fichier. La fonction `run` prend l'instance de `Config` comme argument.

<!--
Old headings. Do not remove or links may break.
-->

<a id="returning-errors-from-the-run-function"></a>

<!--
#### Returning Errors from `run`
-->

#### Renvoyer les erreurs depuis `run`

<!--
With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::build` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate the logic around handling errors into `main` in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`.
-->

Avec la logique restante du programme séparée dans la fonction `run`, nous pouvons améliorer la gestion des erreurs, comme nous l'avons fait avec `Config::build` dans l'encart 12-9. Au lieu de permettre au programme de paniquer en appelant `expect`, la fonction `run` renverra un `Result<T, E>` quand quelque chose se passe mal. Cela nous permettra de consolider davantage la logique de gestion des erreurs dans `main` de manière conviviale. L'encart 12-12 montre les changements que nous devons apporter à la signature et au corps de `run`.

<Listing number="12-12" file-name="src/main.rs" caption="Modification de la fonction `run` pour renvoyer un `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

<!--
We've made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.
-->

Nous avons apporté trois changements significatifs ici. Premièrement, nous avons changé le type de retour de la fonction `run` en `Result<(), Box<dyn Error>>`. Cette fonction renvoyait auparavant le type unité, `()`, et nous gardons cela comme valeur renvoyée dans le cas `Ok`.

<!--
For the error type, we used the trait object `Box<dyn Error>` (and we brought
`std::error::Error` into scope with a `use` statement at the top). We'll cover
trait objects in [Chapter 18][ch18] ignore
-->. For now, just know that
`Box<dyn Error>` means the function will return a type that implements the
`Error` trait, but we don't have to specify what particular type the return
value will be. This gives us flexibility to return error values that may be of
different types in different error cases. The `dyn` keyword is short for
_dynamic_.
-->

Pour le type d'erreur, nous avons utilisé l'objet trait `Box<dyn Error>` (et nous avons importé `std::error::Error` dans la portée avec une instruction `use` en haut). Nous couvrirons les objets trait dans le [Chapitre 18][ch18]<!--
ignore
-->. Pour l'instant, sachez simplement que `Box<dyn Error>` signifie que la fonction renverra un type qui implémente le trait `Error`, mais nous n'avons pas à spécifier quel type particulier sera la valeur de retour. Cela nous donne la flexibilité de renvoyer des valeurs d'erreur qui peuvent être de types différents dans différents cas d'erreur. Le mot-clé `dyn` est l'abréviation de _dynamic_ (dynamique).

<!--
Second, we've removed the call to `expect` in favor of the `?` operator, as we
talked about in [Chapter 9][ch9-question-mark] ignore
-->. Rather than
`panic!` on an error, `?` will return the error value from the current function
for the caller to handle.
-->

Deuxièmement, nous avons supprimé l'appel à `expect` en faveur de l'opérateur `?`, comme nous en avons parlé dans le [Chapitre 9][ch9-question-mark]<!--
ignore
-->. Au lieu de faire un `panic!` sur une erreur, `?` renverra la valeur d'erreur depuis la fonction courante pour que l'appelant la gère.

<!--
Third, the `run` function now returns an `Ok` value in the success case.
We've declared the `run` function's success type as `()` in the signature,
which means we need to wrap the unit type value in the `Ok` value. This
`Ok(())` syntax might look a bit strange at first. But using `()` like this is
the idiomatic way to indicate that we're calling `run` for its side effects
only; it doesn't return a value we need.
-->

Troisièmement, la fonction `run` renvoie maintenant une valeur `Ok` en cas de succès. Nous avons déclaré le type de succès de la fonction `run` comme `()` dans la signature, ce qui signifie que nous devons envelopper la valeur de type unité dans la valeur `Ok`. Cette syntaxe `Ok(())` peut sembler un peu étrange au premier abord. Mais utiliser `()` de cette façon est la manière idiomatique d'indiquer que nous appelons `run` uniquement pour ses effets de bord ; elle ne renvoie pas de valeur dont nous avons besoin.

<!--
When you run this code, it will compile but will display a warning:
-->

Lorsque vous exécuterez ce code, il compilera mais affichera un avertissement :


```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

<!--
Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we're not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error-handling code here! Let's rectify that problem now.
-->

Rust nous dit que notre code a ignoré la valeur `Result` et que la valeur `Result` pourrait indiquer qu'une erreur s'est produite. Mais nous ne vérifions pas s'il y a eu une erreur, et le compilateur nous rappelle que nous avions probablement l'intention d'avoir du code de gestion d'erreurs ici ! Rectifions ce problème maintenant.

<!--
#### Handling Errors Returned from `run` in `main`
-->

#### Gérer les erreurs renvoyées par `run` dans `main`

<!--
We'll check for errors and handle them using a technique similar to one we used
with `Config::build` in Listing 12-10, but with a slight difference:
-->

Nous vérifierons les erreurs et les gérerons en utilisant une technique similaire à celle que nous avons utilisée avec `Config::build` dans l'encart 12-10, mais avec une légère différence :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

<!--
We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and to call `process::exit(1)` if it does. The `run` function
doesn't return a value that we want to `unwrap` in the same way that
`Config::build` returns the `Config` instance. Because `run` returns `()` in
the success case, we only care about detecting an error, so we don't need
`unwrap_or_else` to return the unwrapped value, which would only be `()`.
-->

Nous utilisons `if let` plutôt que `unwrap_or_else` pour vérifier si `run` renvoie une valeur `Err` et appeler `process::exit(1)` si c'est le cas. La fonction `run` ne renvoie pas de valeur que nous voulons « déballer » (unwrap) de la même manière que `Config::build` renvoie l'instance de `Config`. Comme `run` renvoie `()` en cas de succès, nous ne nous soucions que de détecter une erreur, nous n'avons donc pas besoin de `unwrap_or_else` pour renvoyer la valeur déballée, qui ne serait que `()`.

<!--
The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: We print the error and exit.
-->

Les corps de `if let` et des fonctions `unwrap_or_else` sont les mêmes dans les deux cas : nous affichons l'erreur et quittons.

<!--
### Splitting Code into a Library Crate
-->

### Séparer le code dans un crate de bibliothèque

<!--
Our `minigrep` project is looking good so far! Now we'll split the
_src/main.rs_ file and put some code into the _src/lib.rs_ file. That way, we
can test the code and have a _src/main.rs_ file with fewer responsibilities.
-->

Notre projet `minigrep` a bonne allure jusqu'ici ! Maintenant, nous allons diviser le fichier _src/main.rs_ et mettre du code dans le fichier _src/lib.rs_. De cette façon, nous pourrons tester le code et avoir un fichier _src/main.rs_ avec moins de responsabilités.

<!--
Let's define the code responsible for searching text in _src/lib.rs_ rather
than in _src/main.rs_, which will let us (or anyone else using our
`minigrep` library) call the searching function from more contexts than our
`minigrep` binary.
-->

Définissons le code responsable de la recherche de texte dans _src/lib.rs_ plutôt que dans _src/main.rs_, ce qui nous permettra (ou permettra à quiconque utilise notre bibliothèque `minigrep`) d'appeler la fonction de recherche depuis plus de contextes que notre binaire `minigrep`.

<!--
First, let's define the `search` function signature in _src/lib.rs_ as shown in
Listing 12-13, with a body that calls the `unimplemented!` macro. We'll explain
the signature in more detail when we fill in the implementation.
-->

Tout d'abord, définissons la signature de la fonction `search` dans _src/lib.rs_ comme illustré dans l'encart 12-13, avec un corps qui appelle la macro `unimplemented!`. Nous expliquerons la signature plus en détail lorsque nous remplirons l'implémentation.

<Listing number="12-13" file-name="src/lib.rs" caption="Définition de la fonction `search` dans *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

<!--
We've used the `pub` keyword on the function definition to designate `search`
as part of our library crate's public API. We now have a library crate that we
can use from our binary crate and that we can test!
-->

Nous avons utilisé le mot-clé `pub` sur la définition de la fonction pour désigner `search` comme faisant partie de l'API publique de notre crate de bibliothèque. Nous avons maintenant un crate de bibliothèque que nous pouvons utiliser depuis notre crate binaire et que nous pouvons tester !

<!--
Now we need to bring the code defined in _src/lib.rs_ into the scope of the
binary crate in _src/main.rs_ and call it, as shown in Listing 12-14.
-->

Nous devons maintenant importer le code défini dans _src/lib.rs_ dans la portée du crate binaire dans _src/main.rs_ et l'appeler, comme illustré dans l'encart 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Utilisation de la fonction `search` du crate de bibliothèque `minigrep` dans *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

<!--
We add a `use minigrep::search` line to bring the `search` function from
the library crate into the binary crate's scope. Then, in the `run` function,
rather than printing out the contents of the file, we call the `search`
function and pass the `config.query` value and `contents` as arguments. Then,
`run` will use a `for` loop to print each line returned from `search` that
matched the query. This is also a good time to remove the `println!` calls in
the `main` function that displayed the query and the file path so that our
program only prints the search results (if no errors occur).
-->

Nous ajoutons une ligne `use minigrep::search` pour importer la fonction `search` du crate de bibliothèque dans la portée du crate binaire. Ensuite, dans la fonction `run`, au lieu d'afficher le contenu du fichier, nous appelons la fonction `search` et passons la valeur `config.query` et `contents` comme arguments. Ensuite, `run` utilisera une boucle `for` pour afficher chaque ligne renvoyée par `search` qui correspondait à la requête. C'est aussi le bon moment pour supprimer les appels à `println!` dans la fonction `main` qui affichaient la requête et le chemin du fichier, afin que notre programme n'affiche que les résultats de la recherche (si aucune erreur ne survient).

<!--
Note that the search function will be collecting all the results into a vector
it returns before any printing happens. This implementation could be slow to
display results when searching large files, because results aren't printed as
they're found; we'll discuss a possible way to fix this using iterators in
Chapter 13.
-->

Notez que la fonction de recherche collectera tous les résultats dans un vecteur qu'elle renvoie avant que l'affichage n'ait lieu. Cette implémentation pourrait être lente à afficher les résultats lors de la recherche dans de gros fichiers, car les résultats ne sont pas affichés au fur et à mesure qu'ils sont trouvés ; nous discuterons d'une façon possible de corriger cela en utilisant les itérateurs au Chapitre 13.

<!--
Whew! That was a lot of work, but we've set ourselves up for success in the
future. Now it's much easier to handle errors, and we've made the code more
modular. Almost all of our work will be done in _src/lib.rs_ from here on out.
-->

Ouf ! C'était beaucoup de travail, mais nous nous sommes préparés pour le succès futur. Maintenant, il est beaucoup plus facile de gérer les erreurs, et nous avons rendu le code plus modulaire. Presque tout notre travail se fera dans _src/lib.rs_ à partir de maintenant.

<!--
Let's take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: We'll
write some tests!
-->

Profitons de cette nouvelle modularité en faisant quelque chose qui aurait été difficile avec l'ancien code mais qui est facile avec le nouveau : nous allons écrire des tests !

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
