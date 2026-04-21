<!--
## Storing UTF-8 Encoded Text with Strings
-->

## Stocker du texte encodé en UTF-8 avec les strings

<!--
We talked about strings in Chapter 4, but we'll look at them in more depth now.
New Rustaceans commonly get stuck on strings for a combination of three
reasons: Rust's propensity for exposing possible errors, strings being a more
complicated data structure than many programmers give them credit for, and
UTF-8. These factors combine in a way that can seem difficult when you're
coming from other programming languages.
-->

Nous avons parlé des strings au chapitre 4, mais nous allons maintenant les
examiner plus en profondeur. Les nouveaux Rustacés se retrouvent souvent
bloqués sur les strings pour une combinaison de trois raisons : la propension
de Rust à exposer les erreurs possibles, les strings étant une structure de
données plus compliquée que ce que de nombreux développeurs imaginent, et
l'UTF-8. Ces facteurs se combinent d'une manière qui peut sembler difficile
lorsqu'on vient d'autres langages de programmation.

<!--
We discuss strings in the context of collections because strings are
implemented as a collection of bytes, plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we'll
talk about the operations on `String` that every collection type has, such as
creating, updating, and reading. We'll also discuss the ways in which `String`
is different from the other collections, namely, how indexing into a `String` is
complicated by the differences between how people and computers interpret
`String` data.
-->

Nous abordons les strings dans le contexte des collections car les strings sont
implémentées comme une collection d'octets, avec en plus quelques méthodes pour
fournir des fonctionnalités utiles lorsque ces octets sont interprétés comme du
texte. Dans cette section, nous parlerons des opérations sur `String` que
possède chaque type de collection, comme la création, la mise à jour et la
lecture. Nous discuterons également des manières dont `String` diffère des
autres collections, à savoir comment l'indexation dans une `String` est
compliquée par les différences entre la façon dont les humains et les
ordinateurs interprètent les données d'une `String`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="what-is-a-string"></a>

<!--
### Defining Strings
-->

### Définir les strings

<!--
We'll first define what we mean by the term _string_. Rust has only one string
type in the core language, which is the string slice `str` that is usually seen
in its borrowed form, `&str`. In Chapter 4, we talked about string slices,
which are references to some UTF-8 encoded string data stored elsewhere. String
literals, for example, are stored in the program's binary and are therefore
string slices.
-->

Nous allons d'abord définir ce que nous entendons par le terme _string_. Rust
n'a qu'un seul type de string dans le langage de base, qui est la slice de
chaîne `str` que l'on voit généralement sous sa forme empruntée, `&str`. Au
chapitre 4, nous avons parlé des slices de chaîne, qui sont des références à
des données de chaîne encodées en UTF-8 stockées ailleurs. Les littéraux de
chaîne, par exemple, sont stockés dans le binaire du programme et sont donc
des slices de chaîne.

<!--
The `String` type, which is provided by Rust's standard library rather than
coded into the core language, is a growable, mutable, owned, UTF-8 encoded
string type. When Rustaceans refer to "strings" in Rust, they might be
referring to either the `String` or the string slice `&str` types, not just one
of those types. Although this section is largely about `String`, both types are
used heavily in Rust's standard library, and both `String` and string slices
are UTF-8 encoded.
-->

Le type `String`, qui est fourni par la bibliothèque standard de Rust plutôt
que codé dans le langage de base, est un type de chaîne extensible, mutable,
possédé et encodé en UTF-8. Quand les Rustacés font référence aux "strings" en
Rust, ils peuvent désigner soit le type `String` soit le type slice de chaîne
`&str`, et pas seulement l'un de ces types. Bien que cette section porte
principalement sur `String`, les deux types sont intensivement utilisés dans la
bibliothèque standard de Rust, et tant `String` que les slices de chaîne sont
encodés en UTF-8.

<!--
### Creating a New String
-->

### Créer une nouvelle String

<!--
Many of the same operations available with `Vec<T>` are available with `String`
as well because `String` is actually implemented as a wrapper around a vector
of bytes with some extra guarantees, restrictions, and capabilities. An example
of a function that works the same way with `Vec<T>` and `String` is the `new`
function to create an instance, shown in Listing 8-11.
-->

Beaucoup des mêmes opérations disponibles avec `Vec<T>` sont aussi disponibles
avec `String` car `String` est en fait implémenté comme un wrapper autour d'un
vector d'octets avec quelques garanties, restrictions et capacités
supplémentaires. Un exemple de fonction qui fonctionne de la même manière avec
`Vec<T>` et `String` est la fonction `new` pour créer une instance, montrée
dans l'encart 8-11.

<Listing number="8-11" caption="Création d'une nouvelle `String` vide">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

<!--
This line creates a new, empty string called `s`, into which we can then load
data. Often, we'll have some initial data with which we want to start the
string. For that, we use the `to_string` method, which is available on any type
that implements the `Display` trait, as string literals do. Listing 8-12 shows
two examples.
-->

Cette ligne crée une nouvelle chaîne vide appelée `s`, dans laquelle nous
pouvons ensuite charger des données. Souvent, nous aurons des données initiales
avec lesquelles nous voudrons démarrer la chaîne. Pour cela, nous utilisons la
méthode `to_string`, qui est disponible sur tout type implémentant le trait
`Display`, comme c'est le cas des littéraux de chaîne. L'encart 8-12 montre
deux exemples.

<Listing number="8-12" caption="Utilisation de la méthode `to_string` pour créer une `String` à partir d'un littéral de chaîne">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

<!--
This code creates a string containing `initial contents`.
-->

Ce code crée une chaîne contenant `initial contents`.

<!--
We can also use the function `String::from` to create a `String` from a string
literal. The code in Listing 8-13 is equivalent to the code in Listing 8-12
that uses `to_string`.
-->

Nous pouvons également utiliser la fonction `String::from` pour créer une
`String` à partir d'un littéral de chaîne. Le code de l'encart 8-13 est
équivalent au code de l'encart 8-12 qui utilise `to_string`.

<Listing number="8-13" caption="Utilisation de la fonction `String::from` pour créer une `String` à partir d'un littéral de chaîne">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

<!--
Because strings are used for so many things, we can use many different generic
APIs for strings, providing us with a lot of options. Some of them can seem
redundant, but they all have their place! In this case, `String::from` and
`to_string` do the same thing, so which one you choose is a matter of style and
readability.
-->

Parce que les strings sont utilisées pour tellement de choses, nous pouvons
utiliser de nombreuses API génériques différentes pour les strings, ce qui nous
offre beaucoup d'options. Certaines d'entre elles peuvent sembler redondantes,
mais elles ont toutes leur utilité ! Dans ce cas, `String::from` et `to_string`
font la même chose, donc le choix entre les deux est une question de style et
de lisibilité.

<!--
Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them, as shown in Listing 8-14.
-->

N'oubliez pas que les strings sont encodées en UTF-8, donc nous pouvons y
inclure toute donnée correctement encodée, comme montré dans l'encart 8-14.

<Listing number="8-14" caption="Stocker des salutations dans différentes langues dans des strings">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

<!--
All of these are valid `String` values.
-->

Toutes ces valeurs sont des `String` valides.

<!--
### Updating a String
-->

### Mettre à jour une String

<!--
A `String` can grow in size and its contents can change, just like the contents
of a `Vec<T>`, if you push more data into it. In addition, you can conveniently
use the `+` operator or the `format!` macro to concatenate `String` values.
-->

Une `String` peut grandir en taille et son contenu peut changer, tout comme le
contenu d'un `Vec<T>`, si vous y ajoutez plus de données. De plus, vous pouvez
utiliser l'opérateur `+` ou la macro `format!` pour concaténer des valeurs
`String`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="appending-to-a-string-with-push_str-and-push"></a>

<!--
#### Appending with `push_str` or `push`
-->

#### Ajouter du contenu avec `push_str` ou `push`

<!--
We can grow a `String` by using the `push_str` method to append a string slice,
as shown in Listing 8-15.
-->

Nous pouvons faire grandir une `String` en utilisant la méthode `push_str` pour
ajouter une slice de chaîne, comme montré dans l'encart 8-15.

<Listing number="8-15" caption="Ajout d'une slice de chaîne à une `String` en utilisant la méthode `push_str`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

<!--
After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don't necessarily want to take ownership of the
parameter. For example, in the code in Listing 8-16, we want to be able to use
`s2` after appending its contents to `s1`.
-->

Après ces deux lignes, `s` contiendra `foobar`. La méthode `push_str` prend une
slice de chaîne car nous ne voulons pas nécessairement prendre possession du
paramètre. Par exemple, dans le code de l'encart 8-16, nous voulons pouvoir
utiliser `s2` après avoir ajouté son contenu à `s1`.

<Listing number="8-16" caption="Utilisation d'une slice de chaîne après avoir ajouté son contenu à une `String`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

<!--
If the `push_str` method took ownership of `s2`, we wouldn't be able to print
its value on the last line. However, this code works as we'd expect!
-->

Si la méthode `push_str` prenait possession de `s2`, nous ne pourrions pas
afficher sa valeur à la dernière ligne. Cependant, ce code fonctionne comme
prévu !

<!--
The `push` method takes a single character as a parameter and adds it to the
`String`. Listing 8-17 adds the letter _l_ to a `String` using the `push`
method.
-->

La méthode `push` prend un seul caractère en paramètre et l'ajoute à la
`String`. L'encart 8-17 ajoute la lettre _l_ à une `String` en utilisant la
méthode `push`.

<Listing number="8-17" caption="Ajout d'un caractère à une valeur `String` en utilisant `push`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

<!--
As a result, `s` will contain `lol`.
-->

En conséquence, `s` contiendra `lol`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="concatenation-with-the--operator-or-the-format-macro"></a>

<!--
#### Concatenating with `+` or `format!`
-->

#### Concaténer avec `+` ou `format!`

<!--
Often, you'll want to combine two existing strings. One way to do so is to use
the `+` operator, as shown in Listing 8-18.
-->

Souvent, vous voudrez combiner deux chaînes existantes. Une façon de le faire
est d'utiliser l'opérateur `+`, comme montré dans l'encart 8-18.

<Listing number="8-18" caption="Utilisation de l'opérateur `+` pour combiner deux valeurs `String` en une nouvelle valeur `String`">


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

<!--
The string `s3` will contain `Hello, world!`. The reason `s1` is no longer
valid after the addition, and the reason we used a reference to `s2`, has to do
with the signature of the method that's called when we use the `+` operator.
The `+` operator uses the `add` method, whose signature looks something like
this:
-->

La chaîne `s3` contiendra `Hello, world!`. La raison pour laquelle `s1` n'est
plus valide après l'addition, et la raison pour laquelle nous avons utilisé une
référence à `s2`, est liée à la signature de la méthode qui est appelée
lorsque nous utilisons l'opérateur `+`. L'opérateur `+` utilise la méthode
`add`, dont la signature ressemble à ceci :

<!--
```rust,ignore
fn add(self, s: &str) -> String {
```
-->

```rust,ignore
fn add(self, s: &str) -> String {
```

<!--
In the standard library, you'll see `add` defined using generics and associated
types. Here, we've substituted in concrete types, which is what happens when we
call this method with `String` values. We'll discuss generics in Chapter 10.
This signature gives us the clues we need in order to understand the tricky
bits of the `+` operator.
-->

Dans la bibliothèque standard, vous verrez `add` définie à l'aide de la
généricité et de types associés. Ici, nous avons substitué des types concrets,
ce qui est ce qui se passe lorsque nous appelons cette méthode avec des valeurs
`String`. Nous discuterons de la généricité au chapitre 10. Cette signature
nous donne les indices nécessaires pour comprendre les subtilités de
l'opérateur `+`.

<!--
First, `s2` has an `&`, meaning that we're adding a reference of the second
string to the first string. This is because of the `s` parameter in the `add`
function: We can only add a string slice to a `String`; we can't add two
`String` values together. But wait—the type of `&s2` is `&String`, not `&str`,
as specified in the second parameter to `add`. So, why does Listing 8-18
compile?
-->

Premièrement, `s2` a un `&`, ce qui signifie que nous ajoutons une référence de
la deuxième chaîne à la première chaîne. C'est à cause du paramètre `s` dans
la fonction `add` : nous ne pouvons ajouter qu'une slice de chaîne à une
`String` ; nous ne pouvons pas additionner deux valeurs `String`. Mais
attendez -- le type de `&s2` est `&String`, et non `&str`, comme spécifié dans
le second paramètre de `add`. Alors, pourquoi l'encart 8-18 compile-t-il ?

<!--
The reason we're able to use `&s2` in the call to `add` is that the compiler
can coerce the `&String` argument into a `&str`. When we call the `add` method,
Rust uses a deref coercion, which here turns `&s2` into `&s2[..]`. We'll
discuss deref coercion in more depth in Chapter 15. Because `add` does not take
ownership of the `s` parameter, `s2` will still be a valid `String` after this
operation.
-->

La raison pour laquelle nous pouvons utiliser `&s2` dans l'appel à `add` est
que le compilateur peut contraindre l'argument `&String` en `&str`. Lorsque
nous appelons la méthode `add`, Rust utilise une coercition de déréférencement
(*deref coercion*), qui transforme ici `&s2` en `&s2[..]`. Nous discuterons de
la coercition de déréférencement plus en profondeur au chapitre 15. Comme `add`
ne prend pas possession du paramètre `s`, `s2` sera toujours une `String`
valide après cette opération.

<!--
Second, we can see in the signature that `add` takes ownership of `self`
because `self` does _not_ have an `&`. This means `s1` in Listing 8-18 will be
moved into the `add` call and will no longer be valid after that. So, although
`let s3 = s1 + &s2;` looks like it will copy both strings and create a new one,
this statement actually takes ownership of `s1`, appends a copy of the contents
of `s2`, and then returns ownership of the result. In other words, it looks
like it's making a lot of copies, but it isn't; the implementation is more
efficient than copying.
-->

Deuxièmement, nous pouvons voir dans la signature que `add` prend possession de
`self` car `self` n'a _pas_ de `&`. Cela signifie que `s1` dans l'encart 8-18
sera déplacé dans l'appel à `add` et ne sera plus valide après. Donc, bien que
`let s3 = s1 + &s2;` semble copier les deux chaînes et en créer une nouvelle,
cette instruction prend en fait possession de `s1`, ajoute une copie du contenu
de `s2`, puis renvoie la possession du résultat. En d'autres termes, cela
semble faire beaucoup de copies, mais ce n'est pas le cas ; l'implémentation
est plus efficace qu'une copie.

<!--
If we need to concatenate multiple strings, the behavior of the `+` operator
gets unwieldy:
-->

Si nous devons concaténer plusieurs chaînes, le comportement de l'opérateur `+`
devient lourd à manier :


```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

<!--
At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it's difficult to see what's going on. For combining strings in
more complicated ways, we can instead use the `format!` macro:
-->

À ce stade, `s` vaudra `tic-tac-toe`. Avec tous les caractères `+` et `"`, il
est difficile de voir ce qui se passe. Pour combiner des chaînes de manière
plus complexe, nous pouvons utiliser la macro `format!` à la place :


```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

<!--
This code also sets `s` to `tic-tac-toe`. The `format!` macro works like
`println!`, but instead of printing the output to the screen, it returns a
`String` with the contents. The version of the code using `format!` is much
easier to read, and the code generated by the `format!` macro uses references
so that this call doesn't take ownership of any of its parameters.
-->

Ce code assigne également `tic-tac-toe` à `s`. La macro `format!` fonctionne
comme `println!`, mais au lieu d'afficher la sortie à l'écran, elle renvoie une
`String` avec le contenu. La version du code utilisant `format!` est beaucoup
plus facile à lire, et le code généré par la macro `format!` utilise des
références pour que cet appel ne prenne possession d'aucun de ses paramètres.

<!--
### Indexing into Strings
-->

### Indexer dans les strings

<!--
In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if you try to access parts of a `String` using indexing syntax in Rust, you'll
get an error. Consider the invalid code in Listing 8-19.
-->

Dans de nombreux autres langages de programmation, accéder aux caractères
individuels d'une chaîne en les référençant par indice est une opération valide
et courante. Cependant, si vous essayez d'accéder à des parties d'une `String`
en utilisant la syntaxe d'indexation en Rust, vous obtiendrez une erreur.
Considérez le code invalide de l'encart 8-19.

<Listing number="8-19" caption="Tentative d'utilisation de la syntaxe d'indexation avec une `String`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

<!--
This code will result in the following error:
-->

Ce code produira l'erreur suivante :


```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

<!--
The error tells the story: Rust strings don't support indexing. But why not? To
answer that question, we need to discuss how Rust stores strings in memory.
-->

L'erreur dit tout : les strings en Rust ne supportent pas l'indexation. Mais
pourquoi ? Pour répondre à cette question, nous devons aborder la façon dont
Rust stocke les strings en mémoire.

<!--
#### Internal Representation
-->

#### Représentation interne

<!--
A `String` is a wrapper over a `Vec<u8>`. Let's look at some of our properly
encoded UTF-8 example strings from Listing 8-14. First, this one:
-->

Une `String` est un wrapper autour d'un `Vec<u8>`. Examinons certaines de nos
chaînes d'exemple correctement encodées en UTF-8 de l'encart 8-14. D'abord,
celle-ci :


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

<!--
In this case, `len` will be `4`, which means the vector storing the string
`"Hola"` is 4 bytes long. Each of these letters takes 1 byte when encoded in
UTF-8. The following line, however, may surprise you (note that this string
begins with the capital Cyrillic letter _Ze_, not the number 3):
-->

Dans ce cas, `len` vaudra `4`, ce qui signifie que le vector stockant la chaîne
`"Hola"` fait 4 octets de long. Chacune de ces lettres occupe 1 octet lorsqu'elle
est encodée en UTF-8. La ligne suivante, cependant, pourrait vous surprendre
(notez que cette chaîne commence par la lettre cyrillique majuscule _Ze_, et
non le chiffre 3) :


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

<!--
If you were asked how long the string is, you might say 12. In fact, Rust's
answer is 24: That's the number of bytes it takes to encode "Здравствуйте" in
UTF-8, because each Unicode scalar value in that string takes 2 bytes of
storage. Therefore, an index into the string's bytes will not always correlate
to a valid Unicode scalar value. To demonstrate, consider this invalid Rust
code:
-->

Si on vous demandait la longueur de la chaîne, vous pourriez dire 12. En fait,
la réponse de Rust est 24 : c'est le nombre d'octets nécessaires pour encoder
"Здравствуйте" en UTF-8, car chaque valeur scalaire Unicode dans cette chaîne
occupe 2 octets de stockage. Par conséquent, un indice dans les octets de la
chaîne ne correspondra pas toujours à une valeur scalaire Unicode valide. Pour
illustrer cela, considérez ce code Rust invalide :

<!--
```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```
-->

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

<!--
You already know that `answer` will not be `З`, the first letter. When encoded
in UTF-8, the first byte of `З` is `208` and the second is `151`, so it would
seem that `answer` should in fact be `208`, but `208` is not a valid character
on its own. Returning `208` is likely not what a user would want if they asked
for the first letter of this string; however, that's the only data that Rust
has at byte index 0. Users generally don't want the byte value returned, even
if the string contains only Latin letters: If `&"hi"[0]` were valid code that
returned the byte value, it would return `104`, not `h`.
-->

Vous savez déjà que `answer` ne sera pas `З`, la première lettre. Encodé en
UTF-8, le premier octet de `З` est `208` et le second est `151`, donc il
semblerait que `answer` devrait en fait être `208`, mais `208` n'est pas un
caractère valide en soi. Renvoyer `208` n'est probablement pas ce qu'un
utilisateur souhaiterait s'il demandait la première lettre de cette chaîne ;
cependant, c'est la seule donnée que Rust a à l'indice d'octet 0. Les
utilisateurs ne veulent généralement pas que la valeur de l'octet soit renvoyée,
même si la chaîne ne contient que des lettres latines : si `&"hi"[0]` était du
code valide renvoyant la valeur de l'octet, il renverrait `104`, et non `h`.

<!--
The answer, then, is that to avoid returning an unexpected value and causing
bugs that might not be discovered immediately, Rust doesn't compile this code
at all and prevents misunderstandings early in the development process.
-->

La réponse est donc que, pour éviter de renvoyer une valeur inattendue et de
causer des bogues qui pourraient ne pas être découverts immédiatement, Rust ne
compile tout simplement pas ce code et prévient les malentendus tôt dans le
processus de développement.

<!--
Old headings. Do not remove or links may break.
-->

<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>

<!--
#### Bytes, Scalar Values, and Grapheme Clusters
-->

#### Octets, valeurs scalaires et groupes de graphèmes

<!--
Another point about UTF-8 is that there are actually three relevant ways to
look at strings from Rust's perspective: as bytes, scalar values, and grapheme
clusters (the closest thing to what we would call _letters_).
-->

Un autre point concernant l'UTF-8 est qu'il y a en fait trois façons
pertinentes de regarder les strings du point de vue de Rust : sous forme
d'octets, de valeurs scalaires et de groupes de graphèmes (ce qui se rapproche
le plus de ce que nous appellerions des _lettres_).

<!--
If we look at the Hindi word "नमस्ते" written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:
-->

Si nous regardons le mot hindi "नमस्ते" écrit en alphabet devanagari, il est
stocké comme un vector de valeurs `u8` qui ressemble à ceci :

<!--
```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```
-->

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

<!--
That's 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust's `char` type is, those
bytes look like this:
-->

Cela fait 18 octets et c'est ainsi que les ordinateurs stockent finalement ces
données. Si nous les regardons comme des valeurs scalaires Unicode, ce que
représente le type `char` de Rust, ces octets ressemblent à ceci :

<!--
```text
['न', 'म', 'स', '्', 'त', 'े']
```
-->

```text
['न', 'म', 'स', '्', 'त', 'े']
```

<!--
There are six `char` values here, but the fourth and sixth are not letters:
They're diacritics that don't make sense on their own. Finally, if we look at
them as grapheme clusters, we'd get what a person would call the four letters
that make up the Hindi word:
-->

Il y a six valeurs `char` ici, mais la quatrième et la sixième ne sont pas des
lettres : ce sont des signes diacritiques qui n'ont pas de sens isolément.
Enfin, si nous les regardons comme des groupes de graphèmes, nous obtiendrions
ce qu'une personne appellerait les quatre lettres qui composent le mot hindi :

<!--
```text
["न", "म", "स्", "ते"]
```
-->

```text
["न", "म", "स्", "ते"]
```

<!--
Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.
-->

Rust fournit différentes façons d'interpréter les données brutes de chaîne que
les ordinateurs stockent afin que chaque programme puisse choisir
l'interprétation dont il a besoin, quelle que soit la langue humaine dans
laquelle se trouvent les données.

<!--
A final reason Rust doesn't allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn't possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.
-->

Une dernière raison pour laquelle Rust ne nous permet pas d'indexer dans une
`String` pour obtenir un caractère est que les opérations d'indexation sont
censées toujours s'exécuter en temps constant (O(1)). Mais il n'est pas
possible de garantir cette performance avec une `String`, car Rust devrait
parcourir le contenu depuis le début jusqu'à l'indice pour déterminer combien
de caractères valides il y a.

<!--
### Slicing Strings
-->

### Découper les strings en slices

<!--
Indexing into a string is often a bad idea because it's not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. If you really need to use
indices to create string slices, therefore, Rust asks you to be more specific.
-->

Indexer dans une chaîne est souvent une mauvaise idée car il n'est pas clair
quel devrait être le type de retour de l'opération d'indexation de la chaîne :
une valeur d'octet, un caractère, un groupe de graphèmes ou une slice de
chaîne. Si vous avez vraiment besoin d'utiliser des indices pour créer des
slices de chaîne, Rust vous demande donc d'être plus précis.

<!--
Rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:
-->

Au lieu d'indexer en utilisant `[]` avec un seul nombre, vous pouvez utiliser
`[]` avec un intervalle pour créer une slice de chaîne contenant des octets
particuliers :

<!--
```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```
-->

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

<!--
Here, `s` will be a `&str` that contains the first 4 bytes of the string.
Earlier, we mentioned that each of these characters was 2 bytes, which means
`s` will be `Зд`.
-->

Ici, `s` sera un `&str` contenant les 4 premiers octets de la chaîne. Plus
tôt, nous avons mentionné que chacun de ces caractères faisait 2 octets, ce qui
signifie que `s` sera `Зд`.

<!--
If we were to try to slice only part of a character's bytes with something like
`&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid
index were accessed in a vector:
-->

Si nous essayions de découper seulement une partie des octets d'un caractère
avec quelque chose comme `&hello[0..1]`, Rust paniquerait à l'exécution de la
même manière que si un indice invalide était accédé dans un vector :


```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

<!--
You should use caution when creating string slices with ranges, because doing
so can crash your program.
-->

Vous devriez faire preuve de prudence lorsque vous créez des slices de chaîne
avec des intervalles, car cela peut faire planter votre programme.

<!--
Old headings. Do not remove or links may break.
-->

<a id="methods-for-iterating-over-strings"></a>

<!--
### Iterating Over Strings
-->

### Itérer sur les strings

<!--
The best way to operate on pieces of strings is to be explicit about whether
you want characters or bytes. For individual Unicode scalar values, use the
`chars` method. Calling `chars` on "Зд" separates out and returns two values of
type `char`, and you can iterate over the result to access each element:
-->

La meilleure façon d'opérer sur des morceaux de strings est d'être explicite
sur le fait que vous vouliez des caractères ou des octets. Pour les valeurs
scalaires Unicode individuelles, utilisez la méthode `chars`. Appeler `chars`
sur "Зд" sépare et renvoie deux valeurs de type `char`, et vous pouvez itérer
sur le résultat pour accéder à chaque élément :

<!--
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```
-->

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

<!--
This code will print the following:
-->

Ce code affichera ce qui suit :

<!--
```text
З
д
```
-->

```text
З
д
```

<!--
Alternatively, the `bytes` method returns each raw byte, which might be
appropriate for your domain:
-->

Alternativement, la méthode `bytes` renvoie chaque octet brut, ce qui pourrait
être approprié pour votre domaine :

<!--
```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```
-->

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

<!--
This code will print the 4 bytes that make up this string:
-->

Ce code affichera les 4 octets qui composent cette chaîne :

<!--
```text
208
151
208
180
```
-->

```text
208
151
208
180
```

<!--
But be sure to remember that valid Unicode scalar values may be made up of more
than 1 byte.
-->

Mais n'oubliez pas que les valeurs scalaires Unicode valides peuvent être
composées de plus d'un octet.

<!--
Getting grapheme clusters from strings, as with the Devanagari script, is
complex, so this functionality is not provided by the standard library. Crates
are available on [crates.io](https://crates.io/) ignore
--> if this is the
functionality you need.
-->

Obtenir les groupes de graphèmes à partir de strings, comme avec l'alphabet
devanagari, est complexe, c'est pourquoi cette fonctionnalité n'est pas fournie
par la bibliothèque standard. Des crates sont disponibles sur
[crates.io](https://crates.io/)<!--
ignore
--> si c'est la fonctionnalité dont
vous avez besoin.

<!--
Old headings. Do not remove or links may break.
-->

<a id="strings-are-not-so-simple"></a>

<!--
### Handling the Complexities of Strings
-->

### Gérer la complexité des strings

<!--
To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which means programmers have to put more thought into
handling UTF-8 data up front. This trade-off exposes more of the complexity of
strings than is apparent in other programming languages, but it prevents you
from having to handle errors involving non-ASCII characters later in your
development life cycle.
-->

En résumé, les strings sont compliquées. Les différents langages de
programmation font des choix différents quant à la manière de présenter cette
complexité au développeur. Rust a choisi de faire du traitement correct des
données `String` le comportement par défaut pour tous les programmes Rust, ce
qui signifie que les développeurs doivent réfléchir davantage à la gestion
des données UTF-8 dès le départ. Ce compromis expose davantage la complexité
des strings que ce qui est apparent dans d'autres langages de programmation,
mais cela vous évite d'avoir à gérer des erreurs impliquant des caractères
non-ASCII plus tard dans votre cycle de développement.

<!--
The good news is that the standard library offers a lot of functionality built
off the `String` and `&str` types to help handle these complex situations
correctly. Be sure to check out the documentation for useful methods like
`contains` for searching in a string and `replace` for substituting parts of a
string with another string.
-->

La bonne nouvelle est que la bibliothèque standard offre beaucoup de
fonctionnalités construites sur les types `String` et `&str` pour aider à gérer
correctement ces situations complexes. N'hésitez pas à consulter la
documentation pour des méthodes utiles comme `contains` pour chercher dans une
chaîne et `replace` pour substituer des parties d'une chaîne par une autre
chaîne.

<!--
Let's switch to something a bit less complex: hash maps!
-->

Passons à quelque chose d'un peu moins complexe : les hash maps !
