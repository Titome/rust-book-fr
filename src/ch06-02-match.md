<!--
Old headings. Do not remove or links may break.
-->

<a id="the-match-control-flow-operator"></a>

<!--
## The `match` Control Flow Construct
-->

## La construction de flux de contrôle `match`

<!--
Rust has an extremely powerful control flow construct called `match` that
allows you to compare a value against a series of patterns and then execute
code based on which pattern matches. Patterns can be made up of literal values,
variable names, wildcards, and many other things; [Chapter
19][ch19-00-patterns] ignore
--> covers all the different kinds of patterns
and what they do. The power of `match` comes from the expressiveness of the
patterns and the fact that the compiler confirms that all possible cases are
handled.
-->

Rust possède une construction de flux de contrôle extrêmement puissante appelée
`match` qui vous permet de comparer une valeur à une série de motifs puis
d'exécuter du code en fonction du motif qui correspond. Les motifs peuvent être
composés de valeurs littérales, de noms de variables, de jokers, et de bien
d'autres choses ; le [chapitre 19][ch19-00-patterns]<!--
ignore
--> couvre tous
les différents types de motifs et ce qu'ils font. La puissance de `match` vient
de l'expressivité des motifs et du fait que le compilateur confirme que tous les
cas possibles sont gérés.

<!--
Think of a `match` expression as being like a coin-sorting machine: Coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value "fits,"
the value falls into the associated code block to be used during execution.
-->

Imaginez une expression `match` comme une machine à trier les pièces de
monnaie : les pièces glissent le long d'une piste percée de trous de
différentes tailles, et chaque pièce tombe dans le premier trou qu'elle
rencontre et dans lequel elle passe. De la même manière, les valeurs passent à
travers chaque motif d'un `match`, et au premier motif auquel la valeur
« correspond », la valeur tombe dans le bloc de code associé pour être utilisée
lors de l'exécution.

<!--
Speaking of coins, let's use them as an example using `match`! We can write a
function that takes an unknown US coin and, in a similar way as the counting
machine, determines which coin it is and returns its value in cents, as shown
in Listing 6-3.
-->

En parlant de pièces de monnaie, utilisons-les comme exemple avec `match` !
Nous pouvons écrire une fonction qui prend une pièce américaine inconnue et,
de manière similaire à la machine à compter, détermine quelle pièce c'est et
renvoie sa valeur en centimes, comme le montre l'encart 6-3.

<Listing number="6-3" caption="Une enum et une expression `match` qui a les variantes de l'enum comme motifs">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

<!--
Let's break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to a conditional expression used with `if`, but
there's a big difference: With `if`, the condition needs to evaluate to a
Boolean value, but here it can be any type. The type of `coin` in this example
is the `Coin` enum that we defined on the first line.
-->

Décomposons le `match` dans la fonction `value_in_cents`. D'abord, nous
écrivons le mot-clé `match` suivi d'une expression, qui dans ce cas est la
valeur `coin`. Cela semble très similaire à une expression conditionnelle
utilisée avec `if`, mais il y a une grande différence : avec `if`, la condition
doit s'évaluer à une valeur booléenne, mais ici elle peut être de n'importe
quel type. Le type de `coin` dans cet exemple est l'enum `Coin` que nous avons
définie à la première ligne.

<!--
Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.
-->

Ensuite viennent les bras du `match`. Un bras a deux parties : un motif et du
code. Le premier bras ici a un motif qui est la valeur `Coin::Penny` puis
l'opérateur `=>` qui sépare le motif et le code à exécuter. Le code dans ce
cas est simplement la valeur `1`. Chaque bras est séparé du suivant par une
virgule.

<!--
When the `match` expression executes, it compares the resultant value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn't match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: In Listing 6-3, our `match` has four arms.
-->

Lorsque l'expression `match` s'exécute, elle compare la valeur résultante au
motif de chaque bras, dans l'ordre. Si un motif correspond à la valeur, le code
associé à ce motif est exécuté. Si ce motif ne correspond pas à la valeur,
l'exécution continue au bras suivant, tout comme dans une machine à trier les
pièces. Nous pouvons avoir autant de bras que nécessaire : dans l'encart 6-3,
notre `match` a quatre bras.

<!--
The code associated with each arm is an expression, and the resultant value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.
-->

Le code associé à chaque bras est une expression, et la valeur résultante de
l'expression dans le bras correspondant est la valeur qui est renvoyée pour
l'ensemble de l'expression `match`.

<!--
We don't typically use curly brackets if the match arm code is short, as it is
in Listing 6-3 where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you must use curly brackets, and the comma
following the arm is then optional. For example, the following code prints
"Lucky penny!" every time the method is called with a `Coin::Penny`, but it
still returns the last value of the block, `1`:
-->

Nous n'utilisons généralement pas d'accolades si le code du bras de match est
court, comme c'est le cas dans l'encart 6-3 où chaque bras renvoie simplement
une valeur. Si vous voulez exécuter plusieurs lignes de code dans un bras de
match, vous devez utiliser des accolades, et la virgule après le bras est alors
optionnelle. Par exemple, le code suivant affiche « Lucky penny! » chaque fois
que la méthode est appelée avec un `Coin::Penny`, mais renvoie quand même la
dernière valeur du bloc, `1` :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!--
### Patterns That Bind to Values
-->

### Les motifs qui se lient à des valeurs

<!--
Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.
-->

Une autre fonctionnalité utile des bras de match est qu'ils peuvent se lier aux
parties des valeurs qui correspondent au motif. C'est ainsi que nous pouvons
extraire des valeurs des variantes d'enum.

<!--
As an example, let's change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value
stored inside it, which we've done in Listing 6-4.
-->

À titre d'exemple, modifions l'une de nos variantes d'enum pour qu'elle
contienne des données. De 1999 à 2008, les États-Unis ont frappé des quarters
(pièces de 25 centimes) avec des designs différents pour chacun des 50 États
sur une face. Aucune autre pièce n'a reçu de design d'État, donc seuls les
quarters ont cette valeur supplémentaire. Nous pouvons ajouter cette
information à notre `enum` en modifiant la variante `Quarter` pour qu'elle
inclue une valeur `UsState` stockée à l'intérieur, ce que nous avons fait dans
l'encart 6-4.

<Listing number="6-4" caption="Une enum `Coin` dans laquelle la variante `Quarter` contient aussi une valeur `UsState`">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

<!--
Let's imagine that a friend is trying to collect all 50 state quarters. While
we sort our loose change by coin type, we'll also call out the name of the
state associated with each quarter so that if it's one our friend doesn't have,
they can add it to their collection.
-->

Imaginons qu'un ami essaie de collectionner les quarters des 50 États. Pendant
que nous trions notre monnaie par type de pièce, nous allons aussi annoncer le
nom de l'État associé à chaque quarter afin que si c'en est un que notre ami
n'a pas, il puisse l'ajouter à sa collection.

<!--
In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter's state. Then, we can use `state` in the code for that arm, like so:
-->

Dans l'expression match pour ce code, nous ajoutons une variable appelée
`state` au motif qui correspond aux valeurs de la variante `Coin::Quarter`.
Quand un `Coin::Quarter` correspond, la variable `state` se liera à la valeur
de l'État de ce quarter. Ensuite, nous pouvons utiliser `state` dans le code
de ce bras, comme ceci :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

<!--
If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.
-->

Si nous appelions `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
serait `Coin::Quarter(UsState::Alaska)`. Quand nous comparons cette valeur avec
chacun des bras du match, aucun d'entre eux ne correspond jusqu'à ce que nous
atteignions `Coin::Quarter(state)`. À ce moment-là, la liaison pour `state`
sera la valeur `UsState::Alaska`. Nous pouvons ensuite utiliser cette liaison
dans l'expression `println!`, obtenant ainsi la valeur interne de l'État de la
variante `Coin` pour `Quarter`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="matching-with-optiont"></a>

<!--
### The `Option<T>` `match` Pattern
-->

### Le motif `match` avec `Option<T>`

<!--
In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match`, as
we did with the `Coin` enum! Instead of comparing coins, we'll compare the
variants of `Option<T>`, but the way the `match` expression works remains the
same.
-->

Dans la section précédente, nous voulions obtenir la valeur interne `T` du cas
`Some` en utilisant `Option<T>` ; nous pouvons aussi gérer `Option<T>` avec
`match`, comme nous l'avons fait avec l'enum `Coin` ! Au lieu de comparer des
pièces, nous allons comparer les variantes d'`Option<T>`, mais le
fonctionnement de l'expression `match` reste le même.

<!--
Let's say we want to write a function that takes an `Option<i32>` and, if
there's a value inside, adds 1 to that value. If there isn't a value inside,
the function should return the `None` value and not attempt to perform any
operations.
-->

Disons que nous voulons écrire une fonction qui prend un `Option<i32>` et, s'il
y a une valeur à l'intérieur, ajoute 1 à cette valeur. S'il n'y a pas de
valeur à l'intérieur, la fonction devrait renvoyer la valeur `None` et ne pas
tenter d'effectuer d'opération.

<!--
This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.
-->

Cette fonction est très facile à écrire, grâce à `match`, et ressemblera à
l'encart 6-5.

<Listing number="6-5" caption="Une fonction qui utilise une expression `match` sur un `Option<i32>`">


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

<!--
Let's examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm:
-->

Examinons plus en détail la première exécution de `plus_one`. Lorsque nous
appelons `plus_one(five)`, la variable `x` dans le corps de `plus_one` aura
la valeur `Some(5)`. Nous comparons ensuite cette valeur à chaque bras du
match :


```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
The `Some(5)` value doesn't match the pattern `None`, so we continue to the
next arm:
-->

La valeur `Some(5)` ne correspond pas au motif `None`, donc nous continuons au
bras suivant :


```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

<!--
Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i`
binds to the value contained in `Some`, so `i` takes the value `5`. The code in
the match arm is then executed, so we add 1 to the value of `i` and create a
new `Some` value with our total `6` inside.
-->

Est-ce que `Some(5)` correspond à `Some(i)` ? Oui ! Nous avons la même
variante. Le `i` se lie à la valeur contenue dans `Some`, donc `i` prend la
valeur `5`. Le code du bras de match est alors exécuté, nous ajoutons donc 1
à la valeur de `i` et créons une nouvelle valeur `Some` avec notre total `6`
à l'intérieur.

<!--
Now let's consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm:
-->

Considérons maintenant le second appel de `plus_one` dans l'encart 6-5, où `x`
est `None`. Nous entrons dans le `match` et comparons au premier bras :


```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
It matches! There's no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.
-->

Ça correspond ! Il n'y a pas de valeur à laquelle ajouter, donc le programme
s'arrête et renvoie la valeur `None` à droite du `=>`. Parce que le premier
bras a correspondu, aucun autre bras n'est comparé.

<!--
Combining `match` and enums is useful in many situations. You'll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in all languages. It's
consistently a user favorite.
-->

Combiner `match` et les enums est utile dans de nombreuses situations. Vous
verrez beaucoup ce motif dans le code Rust : faire un `match` sur une enum,
lier une variable aux données à l'intérieur, puis exécuter du code en fonction
de cela. C'est un peu déroutant au début, mais une fois que vous vous y serez
habitué, vous souhaiterez l'avoir dans tous les langages. C'est constamment un
favori des utilisateurs.

<!--
### Matches Are Exhaustive
-->

### Les correspondances sont exhaustives

<!--
There's one other aspect of `match` we need to discuss: The arms' patterns must
cover all possibilities. Consider this version of our `plus_one` function,
which has a bug and won't compile:
-->

Il y a un autre aspect de `match` que nous devons aborder : les motifs des bras
doivent couvrir toutes les possibilités. Considérez cette version de notre
fonction `plus_one`, qui contient un bogue et ne compilera pas :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

<!--
We didn't handle the `None` case, so this code will cause a bug. Luckily, it's
a bug Rust knows how to catch. If we try to compile this code, we'll get this
error:
-->

Nous n'avons pas géré le cas `None`, donc ce code causera un bogue.
Heureusement, c'est un bogue que Rust sait détecter. Si nous essayons de
compiler ce code, nous obtiendrons cette erreur :


```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

<!--
Rust knows that we didn't cover every possible case and even knows which
pattern we forgot! Matches in Rust are _exhaustive_: We must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.
-->

Rust sait que nous n'avons pas couvert tous les cas possibles et sait même quel
motif nous avons oublié ! Les correspondances en Rust sont _exhaustives_ : nous
devons épuiser toutes les possibilités pour que le code soit valide. En
particulier dans le cas d'`Option<T>`, quand Rust nous empêche d'oublier de
gérer explicitement le cas `None`, il nous protège de l'hypothèse que nous
avons une valeur quand nous pourrions avoir null, rendant ainsi impossible
l'erreur à un milliard de dollars évoquée précédemment.

<!--
### Catch-All Patterns and the `_` Placeholder
-->

### Les motifs attrape-tout et le caractère générique `_`

<!--
Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we're implementing a game
where, if you roll a 3 on a dice roll, your player doesn't move but instead
gets a fancy new hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here's
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:
-->

En utilisant des enums, nous pouvons aussi effectuer des actions spéciales pour
quelques valeurs particulières, mais pour toutes les autres valeurs, effectuer
une action par défaut. Imaginons que nous implémentons un jeu où, si vous
lancez un 3 avec un dé, votre joueur ne bouge pas mais obtient un nouveau
chapeau élégant. Si vous lancez un 7, votre joueur perd un chapeau élégant.
Pour toutes les autres valeurs, votre joueur avance de ce nombre de cases sur
le plateau de jeu. Voici un `match` qui implémente cette logique, avec le
résultat du lancer de dé codé en dur plutôt qu'une valeur aléatoire, et toute
autre logique représentée par des fonctions sans corps car leur implémentation
réelle dépasse le cadre de cet exemple :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

<!--
For the first two arms, the patterns are the literal values `3` and `7`. For
the last arm that covers every other possible value, the pattern is the
variable we've chosen to name `other`. The code that runs for the `other` arm
uses the variable by passing it to the `move_player` function.
-->

Pour les deux premiers bras, les motifs sont les valeurs littérales `3` et `7`.
Pour le dernier bras qui couvre toutes les autres valeurs possibles, le motif
est la variable que nous avons choisi de nommer `other`. Le code qui s'exécute
pour le bras `other` utilise la variable en la passant à la fonction
`move_player`.

<!--
This code compiles, even though we haven't listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. If we had put the catch-all arm earlier, the
other arms would never run, so Rust will warn us if we add arms after a
catch-all!
-->

Ce code compile, même si nous n'avons pas listé toutes les valeurs possibles
qu'un `u8` peut avoir, parce que le dernier motif correspondra à toutes les
valeurs non spécifiquement listées. Ce motif attrape-tout satisfait l'exigence
que `match` doit être exhaustif. Notez que nous devons placer le bras
attrape-tout en dernier car les motifs sont évalués dans l'ordre. Si nous
avions placé le bras attrape-tout plus tôt, les autres bras ne s'exécuteraient
jamais, donc Rust nous avertira si nous ajoutons des bras après un
attrape-tout !

<!--
Rust also has a pattern we can use when we want a catch-all but don't want to
_use_ the value in the catch-all pattern: `_` is a special pattern that matches
any value and does not bind to that value. This tells Rust we aren't going to
use the value, so Rust won't warn us about an unused variable.
-->

Rust a aussi un motif que nous pouvons utiliser lorsque nous voulons un
attrape-tout mais ne voulons pas _utiliser_ la valeur dans le motif
attrape-tout : `_` est un motif spécial qui correspond à n'importe quelle
valeur et ne se lie pas à cette valeur. Cela indique à Rust que nous n'allons
pas utiliser la valeur, donc Rust ne nous avertira pas d'une variable inutilisée.

<!--
Let's change the rules of the game: Now, if you roll anything other than a 3 or
a 7, you must roll again. We no longer need to use the catch-all value, so we
can change our code to use `_` instead of the variable named `other`:
-->

Changeons les règles du jeu : maintenant, si vous lancez autre chose qu'un 3
ou un 7, vous devez relancer. Nous n'avons plus besoin d'utiliser la valeur
attrape-tout, donc nous pouvons modifier notre code pour utiliser `_` au lieu
de la variable nommée `other` :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

<!--
This example also meets the exhaustiveness requirement because we're explicitly
ignoring all other values in the last arm; we haven't forgotten anything.
-->

Cet exemple satisfait aussi l'exigence d'exhaustivité car nous ignorons
explicitement toutes les autres valeurs dans le dernier bras ; nous n'avons
rien oublié.

<!--
Finally, we'll change the rules of the game one more time so that nothing else
happens on your turn if you roll anything other than a 3 or a 7. We can express
that by using the unit value (the empty tuple type we mentioned in ["The Tuple
Type"][tuples] ignore
--> section) as the code that goes with the `_` arm:
-->

Enfin, nous allons modifier les règles du jeu une dernière fois pour que rien
d'autre ne se passe pendant votre tour si vous lancez autre chose qu'un 3 ou
un 7. Nous pouvons exprimer cela en utilisant la valeur unitaire (le type tuple
vide que nous avons mentionné dans la section [« Le type tuple »][tuples]<!--
ignore
-->)
comme code associé au bras `_` :


```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

<!--
Here, we're telling Rust explicitly that we aren't going to use any other value
that doesn't match a pattern in an earlier arm, and we don't want to run any
code in this case.
-->

Ici, nous disons explicitement à Rust que nous n'allons pas utiliser d'autre
valeur qui ne correspond pas à un motif dans un bras précédent, et que nous ne
voulons exécuter aucun code dans ce cas.

<!--
There's more about patterns and matching that we'll cover in [Chapter
19][ch19-00-patterns] ignore
-->. For now, we're going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.
-->

Il y a encore beaucoup à dire sur les motifs et le filtrage par motif que nous
couvrirons au [chapitre 19][ch19-00-patterns]<!--
ignore
-->. Pour l'instant,
nous allons passer à la syntaxe `if let`, qui peut être utile dans les
situations où l'expression `match` est un peu verbeuse.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch19-00-patterns]: ch19-00-patterns.html
