<!--
## Data Types
-->

## Les types de données

<!--
Every value in Rust is of a certain _data type_, which tells Rust what kind of
data is being specified so that it knows how to work with that data. We'll look
at two data type subsets: scalar and compound.
-->

Chaque valeur en Rust est d'un certain _type de données_, qui indique à Rust
quel genre de données est spécifié afin qu'il sache comment travailler avec ces
données. Nous examinerons deux sous-ensembles de types de données : les
scalaires et les composés.

<!--
Keep in mind that Rust is a _statically typed_ language, which means that it
must know the types of all variables at compile time. The compiler can usually
infer what type we want to use based on the value and how we use it. In cases
when many types are possible, such as when we converted a `String` to a numeric
type using `parse` in the ["Comparing the Guess to the Secret
Number"][comparing-the-guess-to-the-secret-number] ignore
--> section in
Chapter 2, we must add a type annotation, like this:
-->

Gardez à l'esprit que Rust est un langage à _typage statique_, ce qui signifie
qu'il doit connaître les types de toutes les variables à la compilation. Le
compilateur peut généralement inférer le type que nous voulons utiliser en se
basant sur la valeur et la façon dont nous l'utilisons. Dans les cas où
plusieurs types sont possibles, comme lorsque nous avons converti un `String` en
type numérique en utilisant `parse` dans la section [« Comparer la supposition
au nombre secret »][comparing-the-guess-to-the-secret-number]<!--
ignore
--> du
chapitre 2, nous devons ajouter une annotation de type, comme ceci :

<!--
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
-->

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

<!--
If we don't add the `: u32` type annotation shown in the preceding code, Rust
will display the following error, which means the compiler needs more
information from us to know which type we want to use:
-->

Si nous n'ajoutons pas l'annotation de type `: u32` montrée dans le code
précédent, Rust affichera l'erreur suivante, ce qui signifie que le compilateur
a besoin de plus d'informations de notre part pour savoir quel type nous voulons
utiliser :


```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

<!--
You'll see different type annotations for other data types.
-->

Vous verrez différentes annotations de type pour d'autres types de données.

<!--
### Scalar Types
-->

### Les types scalaires

<!--
A _scalar_ type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters. You may recognize
these from other programming languages. Let's jump into how they work in Rust.
-->

Un type _scalaire_ représente une valeur unique. Rust possède quatre types
scalaires principaux : les entiers, les nombres à virgule flottante, les
booléens et les caractères. Vous les reconnaîtrez peut-être d'autres langages
de programmation. Voyons comment ils fonctionnent en Rust.

<!--
#### Integer Types
-->

#### Les types entiers

<!--
An _integer_ is a number without a fractional component. We used one integer
type in Chapter 2, the `u32` type. This type declaration indicates that the
value it's associated with should be an unsigned integer (signed integer types
start with `i` instead of `u`) that takes up 32 bits of space. Table 3-1 shows
the built-in integer types in Rust. We can use any of these variants to declare
the type of an integer value.
-->

Un _entier_ est un nombre sans composante fractionnaire. Nous avons utilisé un
type entier au chapitre 2, le type `u32`. Cette déclaration de type indique que
la valeur qui lui est associée doit être un entier non signé (les types entiers
signés commencent par `i` au lieu de `u`) qui occupe 32 bits d'espace. Le
tableau 3-1 montre les types entiers intégrés à Rust. Nous pouvons utiliser
n'importe laquelle de ces variantes pour déclarer le type d'une valeur entière.

<!--
<span class="caption">Table 3-1: Integer Types in Rust</span>
-->

<span class="caption">Tableau 3-1 : Les types entiers en Rust</span>

<!--
| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| Architecture-dependent | `isize` | `usize`  |
-->

| Taille | Signé   | Non signé |
| ------ | ------- | --------- |
| 8 bits   | `i8`    | `u8`      |
| 16 bits  | `i16`   | `u16`    |
| 32 bits  | `i32`   | `u32`    |
| 64 bits  | `i64`   | `u64`    |
| 128 bits | `i128`  | `u128`  |
| Dépend de l'architecture | `isize` | `usize` |

<!--
Each variant can be either signed or unsigned and has an explicit size.
_Signed_ and _unsigned_ refer to whether it's possible for the number to be
negative—in other words, whether the number needs to have a sign with it
(signed) or whether it will only ever be positive and can therefore be
represented without a sign (unsigned). It's like writing numbers on paper: When
the sign matters, a number is shown with a plus sign or a minus sign; however,
when it's safe to assume the number is positive, it's shown with no sign.
Signed numbers are stored using [two's complement][twos-complement] ignore
--> representation.
-->

Chaque variante peut être signée ou non signée et possède une taille explicite.
_Signé_ et _non signé_ font référence à la possibilité que le nombre soit
négatif — en d'autres termes, si le nombre doit porter un signe (signé) ou s'il
sera toujours positif et peut donc être représenté sans signe (non signé). C'est
comme écrire des nombres sur du papier : quand le signe importe, un nombre est
affiché avec un signe plus ou un signe moins ; cependant, quand on peut
supposer sans risque que le nombre est positif, il est affiché sans signe. Les
nombres signés sont stockés en utilisant la représentation en [complément à
deux][twos-complement]<!--
ignore
-->.

<!--
Each signed variant can store numbers from −(2<sup>n − 1</sup>) to 2<sup>n −
1</sup> − 1 inclusive, where _n_ is the number of bits that variant uses. So, an
`i8` can store numbers from −(2<sup>7</sup>) to 2<sup>7</sup> − 1, which equals
−128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> − 1,
so a `u8` can store numbers from 0 to 2<sup>8</sup> − 1, which equals 0 to 255.
-->

Chaque variante signée peut stocker des nombres de −(2<sup>n − 1</sup>) à
2<sup>n − 1</sup> − 1 inclus, où _n_ est le nombre de bits que la variante
utilise. Ainsi, un `i8` peut stocker des nombres de −(2<sup>7</sup>) à
2<sup>7</sup> − 1, soit de −128 à 127. Les variantes non signées peuvent
stocker des nombres de 0 à 2<sup>n</sup> − 1, donc un `u8` peut stocker des
nombres de 0 à 2<sup>8</sup> − 1, soit de 0 à 255.

<!--
Additionally, the `isize` and `usize` types depend on the architecture of the
computer your program is running on: 64 bits if you're on a 64-bit architecture
and 32 bits if you're on a 32-bit architecture.
-->

De plus, les types `isize` et `usize` dépendent de l'architecture de
l'ordinateur sur lequel votre programme s'exécute : 64 bits si vous êtes sur une
architecture 64 bits et 32 bits si vous êtes sur une architecture 32 bits.

<!--
You can write integer literals in any of the forms shown in Table 3-2. Note
that number literals that can be multiple numeric types allow a type suffix,
such as `57u8`, to designate the type. Number literals can also use `_` as a
visual separator to make the number easier to read, such as `1_000`, which will
have the same value as if you had specified `1000`.
-->

Vous pouvez écrire des littéraux entiers sous n'importe laquelle des formes
indiquées dans le tableau 3-2. Notez que les littéraux numériques qui peuvent
être de plusieurs types numériques permettent un suffixe de type, tel que
`57u8`, pour désigner le type. Les littéraux numériques peuvent également
utiliser `_` comme séparateur visuel pour rendre le nombre plus facile à lire,
comme `1_000`, qui aura la même valeur que si vous aviez spécifié `1000`.

<!--
<span class="caption">Table 3-2: Integer Literals in Rust</span>
-->

<span class="caption">Tableau 3-2 : Les littéraux entiers en Rust</span>

<!--
| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |
-->

| Littéraux numériques | Exemple       |
| -------------------- | ------------- |
| Décimal              | `98_222`      |
| Hexadécimal          | `0xff`        |
| Octal                | `0o77`        |
| Binaire              | `0b1111_0000` |
| Octet (`u8` uniquement) | `b'A'`    |

<!--
So how do you know which type of integer to use? If you're unsure, Rust's
defaults are generally good places to start: Integer types default to `i32`.
The primary situation in which you'd use `isize` or `usize` is when indexing
some sort of collection.
-->

Alors comment savoir quel type d'entier utiliser ? Si vous n'êtes pas sûr, les
valeurs par défaut de Rust sont généralement un bon point de départ : les types
entiers ont pour valeur par défaut `i32`. La situation principale dans laquelle
vous utiliseriez `isize` ou `usize` est lors de l'indexation d'une collection.

<!--
> ##### Integer Overflow
>
> Let's say you have a variable of type `u8` that can hold values between 0 and
> 255. If you try to change the variable to a value outside that range, such as
> 256, _integer overflow_ will occur, which can result in one of two behaviors.
> When you're compiling in debug mode, Rust includes checks for integer overflow
> that cause your program to _panic_ at runtime if this behavior occurs. Rust
> uses the term _panicking_ when a program exits with an error; we'll discuss
> panics in more depth in the ["Unrecoverable Errors with
> `panic!`"][unrecoverable-errors-with-panic] ignore
--> section in Chapter
> 9.
>
> When you're compiling in release mode with the `--release` flag, Rust does
> _not_ include checks for integer overflow that cause panics. Instead, if
> overflow occurs, Rust performs _two's complement wrapping_. In short, values
> greater than the maximum value the type can hold "wrap around" to the minimum
> of the values the type can hold. In the case of a `u8`, the value 256 becomes
> 0, the value 257 becomes 1, and so on. The program won't panic, but the
> variable will have a value that probably isn't what you were expecting it to
> have. Relying on integer overflow's wrapping behavior is considered an error.
>
> To explicitly handle the possibility of overflow, you can use these families
> of methods provided by the standard library for primitive numeric types:
>
> - Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
> - Return the `None` value if there is overflow with the `checked_*` methods.
> - Return the value and a Boolean indicating whether there was overflow with
>   the `overflowing_*` methods.
> - Saturate at the value's minimum or maximum values with the `saturating_*`
>   methods.
-->

> ##### Le dépassement d'entier
>
> Supposons que vous ayez une variable de type `u8` qui peut contenir des
> valeurs entre 0 et 255. Si vous essayez de changer la variable pour une valeur
> en dehors de cette plage, comme 256, un _dépassement d'entier_ se produira,
> ce qui peut entraîner l'un des deux comportements suivants. Lorsque vous
> compilez en mode débogage, Rust inclut des vérifications de dépassement
> d'entier qui provoquent un _panic_ de votre programme à l'exécution si ce
> comportement se produit. Rust utilise le terme _panicking_ (paniquer) lorsqu'un
> programme se termine avec une erreur ; nous aborderons les panics plus en
> détail dans la section [« Les erreurs irrécupérables avec
> `panic!` »][unrecoverable-errors-with-panic]<!--
ignore
--> du chapitre 9.
>
> Lorsque vous compilez en mode release avec le drapeau `--release`, Rust
> n'inclut _pas_ les vérifications de dépassement d'entier qui provoquent des
> panics. Au lieu de cela, si un dépassement se produit, Rust effectue un
> _bouclage en complément à deux_. En bref, les valeurs supérieures à la valeur
> maximale que le type peut contenir « bouclent » vers la valeur minimale que le
> type peut contenir. Dans le cas d'un `u8`, la valeur 256 devient 0, la valeur
> 257 devient 1, et ainsi de suite. Le programme ne paniquera pas, mais la
> variable aura une valeur qui n'est probablement pas celle que vous attendiez.
> Se fier au comportement de bouclage du dépassement d'entier est considéré
> comme une erreur.
>
> Pour gérer explicitement la possibilité de dépassement, vous pouvez utiliser
> ces familles de méthodes fournies par la bibliothèque standard pour les types
> numériques primitifs :
>
> - Boucler dans tous les modes avec les méthodes `wrapping_*`, comme
>   `wrapping_add`.
> - Retourner la valeur `None` s'il y a dépassement avec les méthodes
>   `checked_*`.
> - Retourner la valeur et un booléen indiquant s'il y a eu dépassement avec
>   les méthodes `overflowing_*`.
> - Saturer aux valeurs minimale ou maximale avec les méthodes `saturating_*`.

<!--
#### Floating-Point Types
-->

#### Les types à virgule flottante

<!--
Rust also has two primitive types for _floating-point numbers_, which are
numbers with decimal points. Rust's floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because on modern CPUs, it's roughly the same speed as `f32` but is capable of
more precision. All floating-point types are signed.
-->

Rust possède également deux types primitifs pour les _nombres à virgule
flottante_, qui sont des nombres avec des décimales. Les types à virgule
flottante de Rust sont `f32` et `f64`, qui font respectivement 32 bits et 64
bits. Le type par défaut est `f64` car sur les processeurs modernes, il est à
peu près aussi rapide que `f32` mais offre une plus grande précision. Tous les
types à virgule flottante sont signés.

<!--
Here's an example that shows floating-point numbers in action:
-->

Voici un exemple qui montre les nombres à virgule flottante en action :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

<!--
Floating-point numbers are represented according to the IEEE-754 standard.
-->

Les nombres à virgule flottante sont représentés conformément à la norme
IEEE-754.

<!--
#### Numeric Operations
-->

#### Les opérations numériques

<!--
Rust supports the basic mathematical operations you'd expect for all the number
types: addition, subtraction, multiplication, division, and remainder. Integer
division truncates toward zero to the nearest integer. The following code shows
how you'd use each numeric operation in a `let` statement:
-->

Rust prend en charge les opérations mathématiques de base que vous attendez pour
tous les types numériques : addition, soustraction, multiplication, division et
reste. La division entière tronque vers zéro à l'entier le plus proche. Le code
suivant montre comment utiliser chaque opération numérique dans une instruction
`let` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

<!--
Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. [Appendix
B][appendix_b] ignore
--> contains a list of all operators that Rust
provides.
-->

Chaque expression dans ces instructions utilise un opérateur mathématique et
s'évalue en une seule valeur, qui est ensuite liée à une variable. [L'annexe
B][appendix_b]<!--
ignore
--> contient une liste de tous les opérateurs que Rust
fournit.

<!--
#### The Boolean Type
-->

#### Le type booléen

<!--
As in most other programming languages, a Boolean type in Rust has two possible
values: `true` and `false`. Booleans are one byte in size. The Boolean type in
Rust is specified using `bool`. For example:
-->

Comme dans la plupart des autres langages de programmation, un type booléen en
Rust a deux valeurs possibles : `true` et `false`. Les booléens font un octet.
Le type booléen en Rust est spécifié avec `bool`. Par exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

<!--
The main way to use Boolean values is through conditionals, such as an `if`
expression. We'll cover how `if` expressions work in Rust in the ["Control
Flow"][control-flow] ignore
--> section.
-->

La principale façon d'utiliser les valeurs booléennes est à travers les
conditions, comme une expression `if`. Nous verrons comment les expressions `if`
fonctionnent en Rust dans la section [« Flux de
contrôle »][control-flow]<!--
ignore
-->.

<!--
#### The Character Type
-->

#### Le type caractère

<!--
Rust's `char` type is the language's most primitive alphabetic type. Here are
some examples of declaring `char` values:
-->

Le type `char` de Rust est le type alphabétique le plus primitif du langage.
Voici quelques exemples de déclaration de valeurs `char` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

<!--
Note that we specify `char` literals with single quotation marks, as opposed to
string literals, which use double quotation marks. Rust's `char` type is 4
bytes in size and represents a Unicode scalar value, which means it can
represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and
Korean characters; emojis; and zero-width spaces are all valid `char` values in
Rust. Unicode scalar values range from `U+0000` to `U+D7FF` and `U+E000` to
`U+10FFFF` inclusive. However, a "character" isn't really a concept in Unicode,
so your human intuition for what a "character" is may not match up with what a
`char` is in Rust. We'll discuss this topic in detail in ["Storing UTF-8
Encoded Text with Strings"][strings] ignore
--> in Chapter 8.
-->

Notez que nous spécifions les littéraux `char` avec des guillemets simples,
contrairement aux littéraux de chaîne de caractères, qui utilisent des
guillemets doubles. Le type `char` de Rust fait 4 octets et représente une
valeur scalaire Unicode, ce qui signifie qu'il peut représenter bien plus que de
l'ASCII. Les lettres accentuées ; les caractères chinois, japonais et coréens ;
les emojis ; et les espaces de largeur nulle sont tous des valeurs `char`
valides en Rust. Les valeurs scalaires Unicode vont de `U+0000` à `U+D7FF` et
de `U+E000` à `U+10FFFF` inclus. Cependant, un « caractère » n'est pas
vraiment un concept en Unicode, donc votre intuition humaine de ce qu'est un
« caractère » peut ne pas correspondre à ce qu'est un `char` en Rust. Nous
aborderons ce sujet en détail dans [« Stocker du texte encodé en UTF-8 avec les
String »][strings]<!--
ignore
--> au chapitre 8.

<!--
### Compound Types
-->

### Les types composés

<!--
_Compound types_ can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.
-->

Les _types composés_ peuvent regrouper plusieurs valeurs en un seul type. Rust
possède deux types composés primitifs : les tuples et les tableaux.

<!--
#### The Tuple Type
-->

#### Le type tuple

<!--
A _tuple_ is a general way of grouping together a number of values with a
variety of types into one compound type. Tuples have a fixed length: Once
declared, they cannot grow or shrink in size.
-->

Un _tuple_ est un moyen général de regrouper un certain nombre de valeurs de
types variés en un seul type composé. Les tuples ont une longueur fixe : une
fois déclarés, ils ne peuvent ni grandir ni rétrécir.

<!--
We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don't have to be the same. We've added optional
type annotations in this example:
-->

Nous créons un tuple en écrivant une liste de valeurs séparées par des virgules
entre parenthèses. Chaque position dans le tuple a un type, et les types des
différentes valeurs du tuple n'ont pas besoin d'être identiques. Nous avons
ajouté des annotations de type optionnelles dans cet exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

<!--
The variable `tup` binds to the entire tuple because a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:
-->

La variable `tup` est liée au tuple entier car un tuple est considéré comme un
seul élément composé. Pour extraire les valeurs individuelles d'un tuple, nous
pouvons utiliser le filtrage par motif pour déstructurer une valeur de tuple,
comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

<!--
This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called _destructuring_ because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.
-->

Ce programme crée d'abord un tuple et le lie à la variable `tup`. Il utilise
ensuite un motif avec `let` pour prendre `tup` et le transformer en trois
variables distinctes, `x`, `y` et `z`. C'est ce qu'on appelle la
_déstructuration_ car elle décompose le tuple unique en trois parties. Enfin, le
programme affiche la valeur de `y`, qui est `6.4`.

<!--
We can also access a tuple element directly by using a period (`.`) followed by
the index of the value we want to access. For example:
-->

Nous pouvons également accéder directement à un élément du tuple en utilisant un
point (`.`) suivi de l'index de la valeur à laquelle nous voulons accéder. Par
exemple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

<!--
This program creates the tuple `x` and then accesses each element of the tuple
using their respective indices. As with most programming languages, the first
index in a tuple is 0.
-->

Ce programme crée le tuple `x` puis accède à chaque élément du tuple en
utilisant leurs indices respectifs. Comme dans la plupart des langages de
programmation, le premier index d'un tuple est 0.

<!--
The tuple without any values has a special name, _unit_. This value and its
corresponding type are both written `()` and represent an empty value or an
empty return type. Expressions implicitly return the unit value if they don't
return any other value.
-->

Le tuple sans aucune valeur porte un nom spécial, _unit_. Cette valeur et son
type correspondant s'écrivent tous deux `()` et représentent une valeur vide ou
un type de retour vide. Les expressions retournent implicitement la valeur unit
si elles ne retournent aucune autre valeur.

<!--
#### The Array Type
-->

#### Le type tableau

<!--
Another way to have a collection of multiple values is with an _array_. Unlike
a tuple, every element of an array must have the same type. Unlike arrays in
some other languages, arrays in Rust have a fixed length.
-->

Une autre façon d'avoir une collection de plusieurs valeurs est d'utiliser un
_tableau_ (array). Contrairement à un tuple, chaque élément d'un tableau doit
avoir le même type. Contrairement aux tableaux dans certains autres langages,
les tableaux en Rust ont une longueur fixe.

<!--
We write the values in an array as a comma-separated list inside square
brackets:
-->

Nous écrivons les valeurs d'un tableau sous forme de liste séparée par des
virgules entre crochets :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

<!--
Arrays are useful when you want your data allocated on the stack, the same as
the other types we have seen so far, rather than the heap (we will discuss the
stack and the heap more in [Chapter 4][stack-and-heap] ignore
-->) or when
you want to ensure that you always have a fixed number of elements. An array
isn't as flexible as the vector type, though. A vector is a similar collection
type provided by the standard library that _is_ allowed to grow or shrink in
size because its contents live on the heap. If you're unsure whether to use an
array or a vector, chances are you should use a vector. [Chapter
8][vectors]<!--
ignore
--> discusses vectors in more detail.
-->

Les tableaux sont utiles lorsque vous voulez que vos données soient allouées sur
la pile, de la même manière que les autres types que nous avons vus jusqu'ici,
plutôt que sur le tas (nous aborderons la pile et le tas plus en détail au
[chapitre 4][stack-and-heap]<!--
ignore
-->) ou lorsque vous voulez vous assurer
d'avoir toujours un nombre fixe d'éléments. Un tableau n'est cependant pas
aussi flexible que le type vecteur. Un vecteur est un type de collection
similaire fourni par la bibliothèque standard qui _peut_ grandir ou rétrécir
car son contenu vit sur le tas. Si vous n'êtes pas sûr de devoir utiliser un
tableau ou un vecteur, il y a de fortes chances que vous devriez utiliser un
vecteur. Le [chapitre 8][vectors]<!--
ignore
--> traite des vecteurs plus en
détail.

<!--
However, arrays are more useful when you know the number of elements will not
need to change. For example, if you were using the names of the month in a
program, you would probably use an array rather than a vector because you know
it will always contain 12 elements:
-->

Cependant, les tableaux sont plus utiles lorsque vous savez que le nombre
d'éléments n'aura pas besoin de changer. Par exemple, si vous utilisiez les noms
des mois dans un programme, vous utiliseriez probablement un tableau plutôt
qu'un vecteur car vous savez qu'il contiendra toujours 12 éléments :

<!--
```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```
-->

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

<!--
You write an array's type using square brackets with the type of each element,
a semicolon, and then the number of elements in the array, like so:
-->

Vous écrivez le type d'un tableau en utilisant des crochets avec le type de
chaque élément, un point-virgule, puis le nombre d'éléments dans le tableau,
comme ceci :

<!--
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
-->

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

<!--
Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the array contains five elements.
-->

Ici, `i32` est le type de chaque élément. Après le point-virgule, le nombre `5`
indique que le tableau contient cinq éléments.

<!--
You can also initialize an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets, as shown here:
-->

Vous pouvez également initialiser un tableau pour qu'il contienne la même valeur
pour chaque élément en spécifiant la valeur initiale, suivie d'un
point-virgule, puis la longueur du tableau entre crochets, comme montré ici :

<!--
```rust
let a = [3; 5];
```
-->

```rust
let a = [3; 5];
```

<!--
The array named `a` will contain `5` elements that will all be set to the value
`3` initially. This is the same as writing `let a = [3, 3, 3, 3, 3];` but in a
more concise way.
-->

Le tableau nommé `a` contiendra `5` éléments qui seront tous initialisés à la
valeur `3`. C'est la même chose que d'écrire `let a = [3, 3, 3, 3, 3];` mais de
manière plus concise.

<!--
Old headings. Do not remove or links may break.
-->
<a id="accessing-array-elements"></a>

<!--
#### Array Element Access
-->

#### Accès aux éléments d'un tableau

<!--
An array is a single chunk of memory of a known, fixed size that can be
allocated on the stack. You can access elements of an array using indexing,
like this:
-->

Un tableau est un bloc de mémoire unique d'une taille connue et fixe qui peut
être alloué sur la pile. Vous pouvez accéder aux éléments d'un tableau en
utilisant l'indexation, comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

<!--
In this example, the variable named `first` will get the value `1` because that
is the value at index `[0]` in the array. The variable named `second` will get
the value `2` from index `[1]` in the array.
-->

Dans cet exemple, la variable nommée `first` obtiendra la valeur `1` car c'est
la valeur à l'index `[0]` dans le tableau. La variable nommée `second` obtiendra
la valeur `2` à l'index `[1]` dans le tableau.

<!--
#### Invalid Array Element Access
-->

#### Accès invalide à un élément de tableau

<!--
Let's see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game in
Chapter 2, to get an array index from the user:
-->

Voyons ce qui se passe si vous essayez d'accéder à un élément d'un tableau qui
dépasse la fin du tableau. Supposons que vous exécutiez ce code, similaire au
jeu de devinettes du chapitre 2, pour obtenir un index de tableau de la part de
l'utilisateur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

<!--
This code compiles successfully. If you run this code using `cargo run` and
enter `0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding
value at that index in the array. If you instead enter a number past the end of
the array, such as `10`, you'll see output like this:
-->

Ce code compile avec succès. Si vous exécutez ce code avec `cargo run` et
saisissez `0`, `1`, `2`, `3` ou `4`, le programme affichera la valeur
correspondante à cet index dans le tableau. Si vous saisissez plutôt un nombre
dépassant la fin du tableau, comme `10`, vous verrez une sortie comme celle-ci :

<!--
manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

<!--
```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!--
The program resulted in a runtime error at the point of using an invalid
value in the indexing operation. The program exited with an error message and
didn't execute the final `println!` statement. When you attempt to access an
element using indexing, Rust will check that the index you've specified is less
than the array length. If the index is greater than or equal to the length,
Rust will panic. This check has to happen at runtime, especially in this case,
because the compiler can't possibly know what value a user will enter when they
run the code later.
-->

Le programme a provoqué une erreur à l'exécution au moment de l'utilisation
d'une valeur invalide dans l'opération d'indexation. Le programme s'est terminé
avec un message d'erreur et n'a pas exécuté la dernière instruction `println!`.
Lorsque vous tentez d'accéder à un élément par indexation, Rust vérifie que
l'index que vous avez spécifié est inférieur à la longueur du tableau. Si
l'index est supérieur ou égal à la longueur, Rust paniquera. Cette vérification
doit avoir lieu à l'exécution, surtout dans ce cas, car le compilateur ne peut
pas savoir quelle valeur un utilisateur saisira lorsqu'il exécutera le code
plus tard.

<!--
This is an example of Rust's memory safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust's error handling and how you can
write readable, safe code that neither panics nor allows invalid memory access.
-->

C'est un exemple des principes de sécurité mémoire de Rust en action. Dans de
nombreux langages bas niveau, ce type de vérification n'est pas effectué, et
lorsque vous fournissez un index incorrect, de la mémoire invalide peut être
accédée. Rust vous protège contre ce type d'erreur en quittant immédiatement au
lieu de permettre l'accès mémoire et de continuer. Le chapitre 9 aborde plus en
détail la gestion des erreurs de Rust et comment vous pouvez écrire du code
lisible et sûr qui ne panique pas et ne permet pas d'accès mémoire invalide.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
