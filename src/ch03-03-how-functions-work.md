<!--
## Functions
-->

## Les fonctions

<!--
Functions are prevalent in Rust code. You've already seen one of the most
important functions in the language: the `main` function, which is the entry
point of many programs. You've also seen the `fn` keyword, which allows you to
declare new functions.
-->

Les fonctions sont omniprésentes dans le code Rust. Vous avez déjà vu l'une des
fonctions les plus importantes du langage : la fonction `main`, qui est le point
d'entrée de nombreux programmes. Vous avez également vu le mot-clé `fn`, qui
vous permet de déclarer de nouvelles fonctions.

<!--
Rust code uses _snake case_ as the conventional style for function and variable
names, in which all letters are lowercase and underscores separate words.
Here's a program that contains an example function definition:
-->

Le code Rust utilise le _snake case_ comme convention de style pour les noms de
fonctions et de variables, dans laquelle toutes les lettres sont en minuscules
et les mots sont séparés par des tirets bas (underscores). Voici un programme
qui contient un exemple de définition de fonction :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

<!--
We define a function in Rust by entering `fn` followed by a function name and a
set of parentheses. The curly brackets tell the compiler where the function
body begins and ends.
-->

Nous définissons une fonction en Rust en saisissant `fn` suivi d'un nom de
fonction et d'un jeu de parenthèses. Les accolades indiquent au compilateur où
le corps de la fonction commence et se termine.

<!--
We can call any function we've defined by entering its name followed by a set
of parentheses. Because `another_function` is defined in the program, it can be
called from inside the `main` function. Note that we defined `another_function`
_after_ the `main` function in the source code; we could have defined it before
as well. Rust doesn't care where you define your functions, only that they're
defined somewhere in a scope that can be seen by the caller.
-->

Nous pouvons appeler n'importe quelle fonction que nous avons définie en
saisissant son nom suivi d'un jeu de parenthèses. Comme `another_function` est
définie dans le programme, elle peut être appelée depuis la fonction `main`.
Notez que nous avons défini `another_function` _après_ la fonction `main` dans
le code source ; nous aurions tout aussi bien pu la définir avant. Rust ne se
soucie pas de l'endroit où vous définissez vos fonctions, seulement qu'elles
soient définies quelque part dans une portée visible par l'appelant.

<!--
Let's start a new binary project named _functions_ to explore functions
further. Place the `another_function` example in _src/main.rs_ and run it. You
should see the following output:
-->

Créons un nouveau projet binaire nommé _functions_ pour explorer davantage les
fonctions. Placez l'exemple `another_function` dans _src/main.rs_ et
exécutez-le. Vous devriez voir la sortie suivante :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

<!--
The lines execute in the order in which they appear in the `main` function.
First the "Hello, world!" message prints, and then `another_function` is called
and its message is printed.
-->

Les lignes s'exécutent dans l'ordre dans lequel elles apparaissent dans la
fonction `main`. D'abord le message "Hello, world!" s'affiche, puis
`another_function` est appelée et son message est affiché.

<!--
### Parameters
-->

### Les paramètres

<!--
We can define functions to have _parameters_, which are special variables that
are part of a function's signature. When a function has parameters, you can
provide it with concrete values for those parameters. Technically, the concrete
values are called _arguments_, but in casual conversation, people tend to use
the words _parameter_ and _argument_ interchangeably for either the variables
in a function's definition or the concrete values passed in when you call a
function.
-->

Nous pouvons définir des fonctions avec des _paramètres_, qui sont des variables
spéciales faisant partie de la signature d'une fonction. Lorsqu'une fonction a
des paramètres, vous pouvez lui fournir des valeurs concrètes pour ces
paramètres. Techniquement, les valeurs concrètes sont appelées _arguments_, mais
dans la conversation courante, les gens ont tendance à utiliser les mots
_paramètre_ et _argument_ de manière interchangeable pour désigner soit les
variables dans la définition d'une fonction, soit les valeurs concrètes passées
lors de l'appel d'une fonction.

<!--
In this version of `another_function` we add a parameter:
-->

Dans cette version de `another_function`, nous ajoutons un paramètre :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

<!--
Try running this program; you should get the following output:
-->

Essayez d'exécuter ce programme ; vous devriez obtenir la sortie suivante :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

<!--
The declaration of `another_function` has one parameter named `x`. The type of
`x` is specified as `i32`. When we pass `5` in to `another_function`, the
`println!` macro puts `5` where the pair of curly brackets containing `x` was
in the format string.
-->

La déclaration de `another_function` a un paramètre nommé `x`. Le type de `x`
est spécifié comme `i32`. Lorsque nous passons `5` à `another_function`, la
macro `println!` place `5` à l'endroit où se trouvait la paire d'accolades
contenant `x` dans la chaîne de format.

<!--
In function signatures, you _must_ declare the type of each parameter. This is
a deliberate decision in Rust's design: Requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code to figure out what type you mean. The compiler is also able to give
more-helpful error messages if it knows what types the function expects.
-->

Dans les signatures de fonctions, vous _devez_ déclarer le type de chaque
paramètre. C'est une décision délibérée dans la conception de Rust : exiger des
annotations de type dans les définitions de fonctions signifie que le compilateur
n'a presque jamais besoin que vous les utilisiez ailleurs dans le code pour
déterminer le type que vous souhaitez. Le compilateur est également en mesure de
fournir des messages d'erreur plus utiles s'il connaît les types attendus par la
fonction.

<!--
When defining multiple parameters, separate the parameter declarations with
commas, like this:
-->

Lorsque vous définissez plusieurs paramètres, séparez les déclarations de
paramètres par des virgules, comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

<!--
This example creates a function named `print_labeled_measurement` with two
parameters. The first parameter is named `value` and is an `i32`. The second is
named `unit_label` and is type `char`. The function then prints text containing
both the `value` and the `unit_label`.
-->

Cet exemple crée une fonction nommée `print_labeled_measurement` avec deux
paramètres. Le premier paramètre est nommé `value` et est de type `i32`. Le
second est nommé `unit_label` et est de type `char`. La fonction affiche ensuite
un texte contenant à la fois `value` et `unit_label`.

<!--
Let's try running this code. Replace the program currently in your _functions_
project's _src/main.rs_ file with the preceding example and run it using `cargo
run`:
-->

Essayons d'exécuter ce code. Remplacez le programme actuellement dans le fichier
_src/main.rs_ de votre projet _functions_ par l'exemple précédent et
exécutez-le avec `cargo run` :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

<!--
Because we called the function with `5` as the value for `value` and `'h'` as
the value for `unit_label`, the program output contains those values.
-->

Comme nous avons appelé la fonction avec `5` comme valeur pour `value` et `'h'`
comme valeur pour `unit_label`, la sortie du programme contient ces valeurs.

<!--
### Statements and Expressions
-->

### Les instructions et les expressions

<!--
Function bodies are made up of a series of statements optionally ending in an
expression. So far, the functions we've covered haven't included an ending
expression, but you have seen an expression as part of a statement. Because
Rust is an expression-based language, this is an important distinction to
understand. Other languages don't have the same distinctions, so let's look at
what statements and expressions are and how their differences affect the bodies
of functions.
-->

Les corps de fonctions sont constitués d'une série d'instructions se terminant
éventuellement par une expression. Jusqu'à présent, les fonctions que nous avons
couvertes n'incluaient pas d'expression finale, mais vous avez vu une expression
en tant que partie d'une instruction. Comme Rust est un langage basé sur les
expressions, c'est une distinction importante à comprendre. Les autres langages
ne font pas les mêmes distinctions, alors examinons ce que sont les instructions
et les expressions et comment leurs différences affectent le corps des fonctions.

<!--
- _Statements_ are instructions that perform some action and do not return
  a value.
- _Expressions_ evaluate to a resultant value.
-->

- Les _instructions_ (statements) sont des directives qui effectuent une action
  et ne retournent pas de valeur.
- Les _expressions_ s'évaluent pour produire une valeur résultante.

<!--
Let's look at some examples.
-->

Examinons quelques exemples.

<!--
We've actually already used statements and expressions. Creating a variable and
assigning a value to it with the `let` keyword is a statement. In Listing 3-1,
`let y = 6;` is a statement.
-->

Nous avons en fait déjà utilisé des instructions et des expressions. Créer une
variable et lui assigner une valeur avec le mot-clé `let` est une instruction.
Dans le listing 3-1, `let y = 6;` est une instruction.

<Listing number="3-1" file-name="src/main.rs" caption="Une déclaration de fonction `main` contenant une instruction">


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

<!--
Function definitions are also statements; the entire preceding example is a
statement in itself. (As we'll see shortly, calling a function is not a
statement, though.)
-->

Les définitions de fonctions sont également des instructions ; l'exemple
précédent dans son intégralité est une instruction en soi. (Comme nous le
verrons bientôt, appeler une fonction n'est cependant pas une instruction.)

<!--
Statements do not return values. Therefore, you can't assign a `let` statement
to another variable, as the following code tries to do; you'll get an error:
-->

Les instructions ne retournent pas de valeurs. Par conséquent, vous ne pouvez
pas assigner une instruction `let` à une autre variable, comme le code suivant
tente de le faire ; vous obtiendrez une erreur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

<!--
When you run this program, the error you'll get looks like this:
-->

Lorsque vous exécutez ce programme, l'erreur que vous obtiendrez ressemble à
ceci :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

<!--
The `let y = 6` statement does not return a value, so there isn't anything for
`x` to bind to. This is different from what happens in other languages, such as
C and Ruby, where the assignment returns the value of the assignment. In those
languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.
-->

L'instruction `let y = 6` ne retourne pas de valeur, il n'y a donc rien à quoi
lier `x`. Ceci est différent de ce qui se passe dans d'autres langages, comme C
et Ruby, où l'affectation retourne la valeur de l'affectation. Dans ces
langages, vous pouvez écrire `x = y = 6` et avoir à la fois `x` et `y` avec la
valeur `6` ; ce n'est pas le cas en Rust.

<!--
Expressions evaluate to a value and make up most of the rest of the code that
you'll write in Rust. Consider a math operation, such as `5 + 6`, which is an
expression that evaluates to the value `11`. Expressions can be part of
statements: In Listing 3-1, the `6` in the statement `let y = 6;` is an
expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. A new scope block created with
curly brackets is an expression, for example:
-->

Les expressions s'évaluent pour produire une valeur et constituent la majeure
partie du code que vous écrirez en Rust. Considérez une opération mathématique,
comme `5 + 6`, qui est une expression qui s'évalue à la valeur `11`. Les
expressions peuvent faire partie d'instructions : dans le listing 3-1, le `6`
dans l'instruction `let y = 6;` est une expression qui s'évalue à la valeur `6`.
Appeler une fonction est une expression. Appeler une macro est une expression.
Un nouveau bloc de portée créé avec des accolades est une expression, par
exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

<!--
This expression:
-->

Cette expression :

<!--
```rust,ignore
{
    let x = 3;
    x + 1
}
```
-->

```rust,ignore
{
    let x = 3;
    x + 1
}
```

<!--
is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement. Note the `x + 1` line without a semicolon at
the end, which is unlike most of the lines you've seen so far. Expressions do
not include ending semicolons. If you add a semicolon to the end of an
expression, you turn it into a statement, and it will then not return a value.
Keep this in mind as you explore function return values and expressions next.
-->

est un bloc qui, dans ce cas, s'évalue à `4`. Cette valeur est liée à `y` dans
le cadre de l'instruction `let`. Notez la ligne `x + 1` sans point-virgule à la
fin, ce qui est différent de la plupart des lignes que vous avez vues jusqu'ici.
Les expressions n'incluent pas de point-virgule final. Si vous ajoutez un
point-virgule à la fin d'une expression, vous la transformez en instruction, et
elle ne retournera alors pas de valeur. Gardez cela à l'esprit lorsque vous
explorerez les valeurs de retour des fonctions et les expressions dans la suite.

<!--
### Functions with Return Values
-->

### Les fonctions avec valeurs de retour

<!--
Functions can return values to the code that calls them. We don't name return
values, but we must declare their type after an arrow (`->`). In Rust, the
return value of the function is synonymous with the value of the final
expression in the block of the body of a function. You can return early from a
function by using the `return` keyword and specifying a value, but most
functions return the last expression implicitly. Here's an example of a
function that returns a value:
-->

Les fonctions peuvent retourner des valeurs au code qui les appelle. Nous ne
nommons pas les valeurs de retour, mais nous devons déclarer leur type après une
flèche (`->`). En Rust, la valeur de retour de la fonction est synonyme de la
valeur de la dernière expression dans le bloc du corps d'une fonction. Vous
pouvez retourner prématurément d'une fonction en utilisant le mot-clé `return`
et en spécifiant une valeur, mais la plupart des fonctions retournent
implicitement la dernière expression. Voici un exemple de fonction qui retourne
une valeur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

<!--
There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself. That's a perfectly valid function in
Rust. Note that the function's return type is specified too, as `-> i32`. Try
running this code; the output should look like this:
-->

Il n'y a pas d'appels de fonctions, de macros, ni même d'instructions `let` dans
la fonction `five` -- juste le nombre `5` tout seul. C'est une fonction
parfaitement valide en Rust. Notez que le type de retour de la fonction est
également spécifié, sous la forme `-> i32`. Essayez d'exécuter ce code ; la
sortie devrait ressembler à ceci :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

<!--
The `5` in `five` is the function's return value, which is why the return type
is `i32`. Let's examine this in more detail. There are two important bits:
First, the line `let x = five();` shows that we're using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:
-->

Le `5` dans `five` est la valeur de retour de la fonction, c'est pourquoi le
type de retour est `i32`. Examinons cela plus en détail. Il y a deux points
importants : premièrement, la ligne `let x = five();` montre que nous utilisons
la valeur de retour d'une fonction pour initialiser une variable. Comme la
fonction `five` retourne un `5`, cette ligne est équivalente à la suivante :

<!--
```rust
let x = 5;
```
-->

```rust
let x = 5;
```

<!--
Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it's an expression whose value we want to return.
-->

Deuxièmement, la fonction `five` n'a pas de paramètres et définit le type de la
valeur de retour, mais le corps de la fonction est un simple `5` sans
point-virgule car c'est une expression dont nous voulons retourner la valeur.

<!--
Let's look at another example:
-->

Examinons un autre exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

<!--
Running this code will print `The value of x is: 6`. But what happens if we
place a semicolon at the end of the line containing `x + 1`, changing it from
an expression to a statement?
-->

L'exécution de ce code affichera `The value of x is: 6`. Mais que se passe-t-il
si nous plaçons un point-virgule à la fin de la ligne contenant `x + 1`, la
transformant d'une expression en instruction ?

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

<!--
Compiling this code will produce an error, as follows:
-->

La compilation de ce code produira une erreur, comme suit :


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

<!--
The main error message, `mismatched types`, reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don't evaluate to a value, which is expressed by `()`,
the unit type. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: It suggests removing the semicolon, which
would fix the error.
-->

Le message d'erreur principal, `mismatched types`, révèle le problème
fondamental de ce code. La définition de la fonction `plus_one` indique qu'elle
retournera un `i32`, mais les instructions ne s'évaluent pas en une valeur, ce
qui est exprimé par `()`, le type unitaire. Par conséquent, rien n'est retourné,
ce qui contredit la définition de la fonction et provoque une erreur. Dans cette
sortie, Rust fournit un message pour aider à corriger ce problème : il suggère
de supprimer le point-virgule, ce qui corrigerait l'erreur.
