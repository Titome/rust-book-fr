<!--
## To `panic!` or Not to `panic!`
-->

## Utiliser `panic!` ou ne pas utiliser `panic!`

<!--
So, how do you decide when you should call `panic!` and when you should return
`Result`? When code panics, there's no way to recover. You could call `panic!`
for any error situation, whether there's a possible way to recover or not, but
then you're making the decision that a situation is unrecoverable on behalf of
the calling code. When you choose to return a `Result` value, you give the
calling code options. The calling code could choose to attempt to recover in a
way that's appropriate for its situation, or it could decide that an `Err`
value in this case is unrecoverable, so it can call `panic!` and turn your
recoverable error into an unrecoverable one. Therefore, returning `Result` is a
good default choice when you're defining a function that might fail.
-->

Alors, comment décider quand vous devriez appeler `panic!` et quand vous devriez renvoyer `Result` ? Lorsque le code panique, il n'y a aucun moyen de récupérer. Vous pourriez appeler `panic!` pour n'importe quelle situation d'erreur, qu'il y ait un moyen possible de récupérer ou non, mais dans ce cas vous prenez la décision qu'une situation est irrécupérable au nom du code appelant. Lorsque vous choisissez de renvoyer une valeur `Result`, vous donnez des options au code appelant. Le code appelant pourrait choisir de tenter de récupérer d'une manière appropriée à sa situation, ou il pourrait décider qu'une valeur `Err` dans ce cas est irrécupérable, et ainsi appeler `panic!` pour transformer votre erreur récupérable en erreur irrécupérable. Par conséquent, renvoyer `Result` est un bon choix par défaut lorsque vous définissez une fonction qui pourrait échouer.

<!--
In situations such as examples, prototype code, and tests, it's more
appropriate to write code that panics instead of returning a `Result`. Let's
explore why, then discuss situations in which the compiler can't tell that
failure is impossible, but you as a human can. The chapter will conclude with
some general guidelines on how to decide whether to panic in library code.
-->

Dans des situations telles que les exemples, le code de prototypage et les tests, il est plus approprié d'écrire du code qui panique plutôt que de renvoyer un `Result`. Explorons pourquoi, puis discutons des situations dans lesquelles le compilateur ne peut pas déterminer que l'échec est impossible, mais vous en tant qu'humain le pouvez. Le chapitre se conclura par quelques directives générales sur la façon de décider s'il faut paniquer dans du code de bibliothèque.

<!--
### Examples, Prototype Code, and Tests
-->

### Exemples, code de prototypage et tests

<!--
When you're writing an example to illustrate some concept, also including
robust error-handling code can make the example less clear. In examples, it's
understood that a call to a method like `unwrap` that could panic is meant as a
placeholder for the way you'd want your application to handle errors, which can
differ based on what the rest of your code is doing.
-->

Lorsque vous écrivez un exemple pour illustrer un concept, inclure également du code robuste de gestion des erreurs peut rendre l'exemple moins clair. Dans les exemples, il est entendu qu'un appel à une méthode comme `unwrap` qui pourrait paniquer est destiné à servir de substitut pour la manière dont vous voudriez que votre application gère les erreurs, ce qui peut différer selon ce que fait le reste de votre code.

<!--
Similarly, the `unwrap` and `expect` methods are very handy when you're
prototyping and you're not yet ready to decide how to handle errors. They leave
clear markers in your code for when you're ready to make your program more
robust.
-->

De la même manière, les méthodes `unwrap` et `expect` sont très pratiques lorsque vous faites du prototypage et que vous n'êtes pas encore prêt à décider comment gérer les erreurs. Elles laissent des marqueurs clairs dans votre code pour le moment où vous serez prêt à rendre votre programme plus robuste.

<!--
If a method call fails in a test, you'd want the whole test to fail, even if
that method isn't the functionality under test. Because `panic!` is how a test
is marked as a failure, calling `unwrap` or `expect` is exactly what should
happen.
-->

Si un appel de méthode échoue dans un test, vous voudriez que le test entier échoue, même si cette méthode n'est pas la fonctionnalité testée. Comme `panic!` est la façon dont un test est marqué comme échoué, appeler `unwrap` ou `expect` est exactement ce qui devrait se passer.

<!--
Old headings. Do not remove or links may break.
-->

<a id="cases-in-which-you-have-more-information-than-the-compiler"></a>

<!--
### When You Have More Information Than the Compiler
-->

### Quand vous en savez plus que le compilateur

<!--
It would also be appropriate to call `expect` when you have some other logic
that ensures that the `Result` will have an `Ok` value, but the logic isn't
something the compiler understands. You'll still have a `Result` value that you
need to handle: Whatever operation you're calling still has the possibility of
failing in general, even though it's logically impossible in your particular
situation. If you can ensure by manually inspecting the code that you'll never
have an `Err` variant, it's perfectly acceptable to call `expect` and document
the reason you think you'll never have an `Err` variant in the argument text.
Here's an example:
-->

Il serait également approprié d'appeler `expect` lorsque vous avez une autre logique qui garantit que le `Result` aura une valeur `Ok`, mais que cette logique n'est pas quelque chose que le compilateur comprend. Vous aurez toujours une valeur `Result` que vous devez gérer : quelle que soit l'opération que vous appelez, elle a toujours la possibilité d'échouer en général, même si c'est logiquement impossible dans votre situation particulière. Si vous pouvez garantir en inspectant manuellement le code que vous n'aurez jamais de variante `Err`, il est tout à fait acceptable d'appeler `expect` et de documenter la raison pour laquelle vous pensez ne jamais avoir de variante `Err` dans le texte de l'argument. Voici un exemple :


```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

<!--
We're creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `127.0.0.1` is a valid IP address, so it's acceptable to use `expect`
here. However, having a hardcoded, valid string doesn't change the return type
of the `parse` method: We still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is a possibility
because the compiler isn't smart enough to see that this string is always a
valid IP address. If the IP address string came from a user rather than being
hardcoded into the program and therefore _did_ have a possibility of failure,
we'd definitely want to handle the `Result` in a more robust way instead.
Mentioning the assumption that this IP address is hardcoded will prompt us to
change `expect` to better error-handling code if, in the future, we need to get
the IP address from some other source instead.
-->

Nous créons une instance d'`IpAddr` en analysant une chaîne de caractères codée en dur. Nous pouvons voir que `127.0.0.1` est une adresse IP valide, il est donc acceptable d'utiliser `expect` ici. Cependant, avoir une chaîne valide codée en dur ne change pas le type de retour de la méthode `parse` : nous obtenons toujours une valeur `Result`, et le compilateur nous obligera toujours à gérer le `Result` comme si la variante `Err` était une possibilité, car le compilateur n'est pas assez intelligent pour voir que cette chaîne est toujours une adresse IP valide. Si la chaîne de l'adresse IP provenait d'un utilisateur plutôt que d'être codée en dur dans le programme et avait donc _effectivement_ une possibilité d'échec, nous voudrions certainement gérer le `Result` de manière plus robuste. Mentionner l'hypothèse que cette adresse IP est codée en dur nous incitera à remplacer `expect` par un meilleur code de gestion des erreurs si, à l'avenir, nous devons obtenir l'adresse IP depuis une autre source.

<!--
### Guidelines for Error Handling
-->

### Directives pour la gestion des erreurs

<!--
It's advisable to have your code panic when it's possible that your code could
end up in a bad state. In this context, a _bad state_ is when some assumption,
guarantee, contract, or invariant has been broken, such as when invalid values,
contradictory values, or missing values are passed to your code—plus one or
more of the following:
-->

Il est conseillé de faire paniquer votre code lorsqu'il est possible que votre code se retrouve dans un état incohérent. Dans ce contexte, un _état incohérent_ (bad state) survient lorsqu'une hypothèse, une garantie, un contrat ou un invariant a été violé, par exemple lorsque des valeurs invalides, contradictoires ou manquantes sont passées à votre code -- plus une ou plusieurs des conditions suivantes :

<!--
- The bad state is something that is unexpected, as opposed to something that
  will likely happen occasionally, like a user entering data in the wrong
  format.
- Your code after this point needs to rely on not being in this bad state,
  rather than checking for the problem at every step.
- There's not a good way to encode this information in the types you use. We'll
  work through an example of what we mean in ["Encoding States and Behavior as
  Types"][encoding] ignore
--> in Chapter 18.
-->

- L'état incohérent est quelque chose d'inattendu, par opposition à quelque chose qui se produira probablement de temps en temps, comme un utilisateur saisissant des données dans le mauvais format.
- Votre code après ce point doit pouvoir compter sur le fait de ne pas être dans cet état incohérent, plutôt que de vérifier le problème à chaque étape.
- Il n'y a pas de bonne façon d'encoder cette information dans les types que vous utilisez. Nous travaillerons sur un exemple de ce que nous voulons dire dans [« Encoder les états et les comportements comme des types »][encoding]<!--
ignore
--> au chapitre 18.

<!--
If someone calls your code and passes in values that don't make sense, it's
best to return an error if you can so that the user of the library can decide
what they want to do in that case. However, in cases where continuing could be
insecure or harmful, the best choice might be to call `panic!` and alert the
person using your library to the bug in their code so that they can fix it
during development. Similarly, `panic!` is often appropriate if you're calling
external code that is out of your control and returns an invalid state that you
have no way of fixing.
-->

Si quelqu'un appelle votre code et passe des valeurs qui n'ont pas de sens, il est préférable de renvoyer une erreur si vous le pouvez afin que l'utilisateur de la bibliothèque puisse décider ce qu'il veut faire dans ce cas. Cependant, dans les cas où continuer pourrait être dangereux ou nuisible, le meilleur choix pourrait être d'appeler `panic!` et d'alerter la personne utilisant votre bibliothèque du bogue dans son code afin qu'elle puisse le corriger pendant le développement. De même, `panic!` est souvent approprié si vous appelez du code externe qui échappe à votre contrôle et qui renvoie un état invalide que vous n'avez aucun moyen de corriger.

<!--
However, when failure is expected, it's more appropriate to return a `Result`
than to make a `panic!` call. Examples include a parser being given malformed
data or an HTTP request returning a status that indicates you have hit a rate
limit. In these cases, returning a `Result` indicates that failure is an
expected possibility that the calling code must decide how to handle.
-->

Cependant, lorsque l'échec est attendu, il est plus approprié de renvoyer un `Result` que de faire un appel à `panic!`. Les exemples incluent un analyseur recevant des données malformées ou une requête HTTP renvoyant un statut indiquant que vous avez atteint une limite de débit. Dans ces cas, renvoyer un `Result` indique que l'échec est une possibilité attendue que le code appelant doit décider comment gérer.

<!--
When your code performs an operation that could put a user at risk if it's
called using invalid values, your code should verify the values are valid first
and panic if the values aren't valid. This is mostly for safety reasons:
Attempting to operate on invalid data can expose your code to vulnerabilities.
This is the main reason the standard library will call `panic!` if you attempt
an out-of-bounds memory access: Trying to access memory that doesn't belong to
the current data structure is a common security problem. Functions often have
_contracts_: Their behavior is only guaranteed if the inputs meet particular
requirements. Panicking when the contract is violated makes sense because a
contract violation always indicates a caller-side bug, and it's not a kind of
error you want the calling code to have to explicitly handle. In fact, there's
no reasonable way for calling code to recover; the calling _programmers_ need
to fix the code. Contracts for a function, especially when a violation will
cause a panic, should be explained in the API documentation for the function.
-->

Lorsque votre code effectue une opération qui pourrait mettre un utilisateur en danger si elle est appelée avec des valeurs invalides, votre code devrait vérifier que les valeurs sont valides en premier et paniquer si les valeurs ne sont pas valides. C'est principalement pour des raisons de sécurité : tenter d'opérer sur des données invalides peut exposer votre code à des vulnérabilités. C'est la raison principale pour laquelle la bibliothèque standard appellera `panic!` si vous tentez un accès mémoire hors limites : essayer d'accéder à de la mémoire qui n'appartient pas à la structure de données actuelle est un problème de sécurité courant. Les fonctions ont souvent des _contrats_ : leur comportement n'est garanti que si les entrées satisfont des exigences particulières. Paniquer lorsque le contrat est violé est logique car une violation de contrat indique toujours un bogue du côté de l'appelant, et ce n'est pas un type d'erreur que vous voulez que le code appelant doive explicitement gérer. En fait, il n'y a pas de moyen raisonnable pour le code appelant de récupérer ; ce sont les _programmeurs_ appelants qui doivent corriger le code. Les contrats d'une fonction, en particulier lorsqu'une violation provoquera un panic, devraient être expliqués dans la documentation de l'API de la fonction.

<!--
However, having lots of error checks in all of your functions would be verbose
and annoying. Fortunately, you can use Rust's type system (and thus the type
checking done by the compiler) to do many of the checks for you. If your
function has a particular type as a parameter, you can proceed with your code's
logic knowing that the compiler has already ensured that you have a valid
value. For example, if you have a type rather than an `Option`, your program
expects to have _something_ rather than _nothing_. Your code then doesn't have
to handle two cases for the `Some` and `None` variants: It will only have one
case for definitely having a value. Code trying to pass nothing to your
function won't even compile, so your function doesn't have to check for that
case at runtime. Another example is using an unsigned integer type such as
`u32`, which ensures that the parameter is never negative.
-->

Cependant, avoir de nombreuses vérifications d'erreurs dans toutes vos fonctions serait verbeux et agaçant. Heureusement, vous pouvez utiliser le système de types de Rust (et donc la vérification de types effectuée par le compilateur) pour faire beaucoup de ces vérifications à votre place. Si votre fonction a un type particulier comme paramètre, vous pouvez poursuivre la logique de votre code en sachant que le compilateur a déjà garanti que vous avez une valeur valide. Par exemple, si vous avez un type plutôt qu'une `Option`, votre programme s'attend à avoir _quelque chose_ plutôt que _rien_. Votre code n'a alors pas besoin de gérer deux cas pour les variantes `Some` et `None` : il n'aura qu'un seul cas pour avoir définitivement une valeur. Du code essayant de passer rien à votre fonction ne compilera même pas, donc votre fonction n'a pas besoin de vérifier ce cas à l'exécution. Un autre exemple est l'utilisation d'un type d'entier non signé comme `u32`, qui garantit que le paramètre n'est jamais négatif.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-custom-types-for-validation"></a>

<!--
### Custom Types for Validation
-->

### Types personnalisés pour la validation

<!--
Let's take the idea of using Rust's type system to ensure that we have a valid
value one step further and look at creating a custom type for validation.
Recall the guessing game in Chapter 2 in which our code asked the user to guess
a number between 1 and 100. We never validated that the user's guess was
between those numbers before checking it against our secret number; we only
validated that the guess was positive. In this case, the consequences were not
very dire: Our output of "Too high" or "Too low" would still be correct. But it
would be a useful enhancement to guide the user toward valid guesses and have
different behavior when the user guesses a number that's out of range versus
when the user types, for example, letters instead.
-->

Poussons l'idée d'utiliser le système de types de Rust pour garantir que nous avons une valeur valide un cran plus loin et examinons la création d'un type personnalisé pour la validation. Rappelez-vous le jeu de devinettes du chapitre 2 dans lequel notre code demandait à l'utilisateur de deviner un nombre entre 1 et 100. Nous n'avons jamais validé que la supposition de l'utilisateur se trouvait entre ces nombres avant de la comparer avec notre nombre secret ; nous avons seulement validé que la supposition était positive. Dans ce cas, les conséquences n'étaient pas très graves : notre affichage de « Trop grand » ou « Trop petit » serait toujours correct. Mais ce serait une amélioration utile de guider l'utilisateur vers des suppositions valides et d'avoir un comportement différent lorsque l'utilisateur devine un nombre hors de la plage par rapport à quand l'utilisateur tape, par exemple, des lettres à la place.

<!--
One way to do this would be to parse the guess as an `i32` instead of only a
`u32` to allow potentially negative numbers, and then add a check for the
number being in range, like so:
-->

Une façon de faire cela serait d'analyser la supposition comme un `i32` au lieu d'un `u32` uniquement pour permettre des nombres potentiellement négatifs, puis d'ajouter une vérification que le nombre est dans la plage, comme ceci :


<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

<!--
The `if` expression checks whether our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.
-->

L'expression `if` vérifie si notre valeur est hors de la plage, informe l'utilisateur du problème et appelle `continue` pour démarrer la prochaine itération de la boucle et demander une autre supposition. Après l'expression `if`, nous pouvons poursuivre avec les comparaisons entre `guess` et le nombre secret en sachant que `guess` est entre 1 et 100.

<!--
However, this is not an ideal solution: If it were absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).
-->

Cependant, ce n'est pas une solution idéale : s'il était absolument critique que le programme n'opère que sur des valeurs entre 1 et 100, et qu'il avait de nombreuses fonctions avec cette exigence, avoir une vérification comme celle-ci dans chaque fonction serait fastidieux (et pourrait impacter les performances).

<!--
Instead, we can make a new type in a dedicated module and put the validations
in a function to create an instance of the type rather than repeating the
validations everywhere. That way, it's safe for functions to use the new type
in their signatures and confidently use the values they receive. Listing 9-13
shows one way to define a `Guess` type that will only create an instance of
`Guess` if the `new` function receives a value between 1 and 100.
-->

À la place, nous pouvons créer un nouveau type dans un module dédié et placer les validations dans une fonction qui crée une instance du type plutôt que de répéter les validations partout. De cette façon, les fonctions peuvent utiliser le nouveau type dans leurs signatures en toute sécurité et utiliser avec confiance les valeurs qu'elles reçoivent. L'encart 9-13 montre une façon de définir un type `Guess` qui ne créera une instance de `Guess` que si la fonction `new` reçoit une valeur entre 1 et 100.


<Listing number="9-13" caption="Un type `Guess` qui ne continuera qu'avec des valeurs entre 1 et 100" file-name="src/guessing_game.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/guessing_game.rs}}
```

</Listing>

<!--
Note that this code in *src/guessing_game.rs* depends on adding a module
declaration `mod guessing_game;` in *src/lib.rs* that we haven't shown here.
Within this new module's file, we define a struct named `Guess` that has a
field named `value` that holds an `i32`. This is where the number will be
stored.
-->

Notez que ce code dans *src/guessing_game.rs* dépend de l'ajout d'une déclaration de module `mod guessing_game;` dans *src/lib.rs* que nous n'avons pas montrée ici. Dans le fichier de ce nouveau module, nous définissons une structure nommée `Guess` qui a un champ nommé `value` contenant un `i32`. C'est là que le nombre sera stocké.

<!--
Then, we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `i32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it's between 1 and 100.
If `value` doesn't pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we'll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.
-->

Ensuite, nous implémentons une fonction associée nommée `new` sur `Guess` qui crée des instances de valeurs `Guess`. La fonction `new` est définie avec un paramètre nommé `value` de type `i32` et renvoie un `Guess`. Le code dans le corps de la fonction `new` teste `value` pour s'assurer qu'il est entre 1 et 100. Si `value` ne passe pas ce test, nous faisons un appel à `panic!`, qui alertera le programmeur écrivant le code appelant qu'il a un bogue qu'il doit corriger, car créer un `Guess` avec une `value` en dehors de cette plage violerait le contrat sur lequel `Guess::new` s'appuie. Les conditions dans lesquelles `Guess::new` pourrait paniquer devraient être discutées dans sa documentation d'API publique ; nous couvrirons les conventions de documentation indiquant la possibilité d'un `panic!` dans la documentation d'API que vous créerez au chapitre 14. Si `value` passe le test, nous créons un nouveau `Guess` avec son champ `value` défini sur le paramètre `value` et renvoyons le `Guess`.

<!--
Next, we implement a method named `value` that borrows `self`, doesn't have any
other parameters, and returns an `i32`. This kind of method is sometimes called
a _getter_ because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It's important that the `value` field be private so that
code using the `Guess` struct is not allowed to set `value` directly: Code
outside the `guessing_game` module _must_ use the `Guess::new` function to
create an instance of `Guess`, thereby ensuring that there's no way for a
`Guess` to have a `value` that hasn't been checked by the conditions in the
`Guess::new` function.
-->

Ensuite, nous implémentons une méthode nommée `value` qui emprunte `self`, n'a aucun autre paramètre et renvoie un `i32`. Ce type de méthode est parfois appelé un _getter_ car son but est d'obtenir des données de ses champs et de les renvoyer. Cette méthode publique est nécessaire car le champ `value` de la structure `Guess` est privé. Il est important que le champ `value` soit privé afin que le code utilisant la structure `Guess` ne soit pas autorisé à définir `value` directement : le code en dehors du module `guessing_game` _doit_ utiliser la fonction `Guess::new` pour créer une instance de `Guess`, garantissant ainsi qu'il n'y a aucun moyen pour un `Guess` d'avoir une `value` qui n'a pas été vérifiée par les conditions de la fonction `Guess::new`.

<!--
A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than an
`i32` and wouldn't need to do any additional checks in its body.
-->

Une fonction qui a un paramètre ou ne renvoie que des nombres entre 1 et 100 pourrait alors déclarer dans sa signature qu'elle prend ou renvoie un `Guess` plutôt qu'un `i32` et n'aurait pas besoin de faire de vérifications supplémentaires dans son corps.

<!--
## Summary
-->

## Résumé

<!--
Rust's error-handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can't handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust's type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.
-->

Les fonctionnalités de gestion des erreurs de Rust sont conçues pour vous aider à écrire du code plus robuste. La macro `panic!` signale que votre programme est dans un état qu'il ne peut pas gérer et vous permet de dire au processus de s'arrêter au lieu d'essayer de continuer avec des valeurs invalides ou incorrectes. L'enum `Result` utilise le système de types de Rust pour indiquer que des opérations pourraient échouer d'une manière dont votre code pourrait récupérer. Vous pouvez utiliser `Result` pour indiquer au code qui appelle votre code qu'il doit également gérer le succès ou l'échec potentiel. Utiliser `panic!` et `Result` dans les situations appropriées rendra votre code plus fiable face aux problèmes inévitables.

<!--
Now that you've seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we'll talk about how generics work and how you
can use them in your code.
-->

Maintenant que vous avez vu des manières utiles dont la bibliothèque standard utilise les génériques avec les enums `Option` et `Result`, nous parlerons de la façon dont les génériques fonctionnent et comment vous pouvez les utiliser dans votre code.

[encoding]: ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
