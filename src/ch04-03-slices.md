<!--
## The Slice Type
-->

## Le type slice

<!--
_Slices_ let you reference a contiguous sequence of elements in a
[collection](ch08-00-common-collections.md) ignore
-->. A slice is a kind
of reference, so it does not have ownership.
-->

Les _slices_ vous permettent de faire reference a une sequence contigue
d'elements dans une [collection](ch08-00-common-collections.md)<!--
ignore
-->.
Une slice est un type de reference, elle n'a donc pas la possession.

<!--
Here's a small programming problem: Write a function that takes a string of
words separated by spaces and returns the first word it finds in that string.
If the function doesn't find a space in the string, the whole string must be
one word, so the entire string should be returned.
-->

Voici un petit probleme de programmation : ecrivez une fonction qui prend une
chaine de mots separes par des espaces et renvoie le premier mot qu'elle trouve
dans cette chaine. Si la fonction ne trouve pas d'espace dans la chaine, la
chaine entiere doit etre un seul mot, donc la chaine entiere doit etre renvoyee.

<!--
> Note: For the purposes of introducing slices, we are assuming ASCII only in
> this section; a more thorough discussion of UTF-8 handling is in the
> ["Storing UTF-8 Encoded Text with Strings"][strings] ignore
--> section
> of Chapter 8.
-->

> Note : Pour les besoins de l'introduction des slices, nous supposons
> uniquement de l'ASCII dans cette section ; une discussion plus approfondie du
> traitement de l'UTF-8 se trouve dans la section ["Stocker du texte encode en
> UTF-8 avec les Strings"][strings]<!--
ignore
--> du chapitre 8.

<!--
Let's work through how we'd write the signature of this function without using
slices, to understand the problem that slices will solve:
-->

Voyons comment nous ecririons la signature de cette fonction sans utiliser de
slices, pour comprendre le probleme que les slices vont resoudre :

<!--
```rust,ignore
fn first_word(s: &String) -> ?
```
-->

```rust,ignore
fn first_word(s: &String) -> ?
```

<!--
The `first_word` function has a parameter of type `&String`. We don't need
ownership, so this is fine. (In idiomatic Rust, functions do not take ownership
of their arguments unless they need to, and the reasons for that will become
clear as we keep going.) But what should we return? We don't really have a way
to talk about *part* of a string. However, we could return the index of the end
of the word, indicated by a space. Let's try that, as shown in Listing 4-7.
-->

La fonction `first_word` a un parametre de type `&String`. Nous n'avons pas
besoin de la possession, donc c'est bien. (En Rust idiomatique, les fonctions ne
prennent pas la possession de leurs arguments a moins qu'elles n'en aient besoin,
et les raisons deviendront claires au fur et a mesure.) Mais que devrions-nous
renvoyer ? Nous n'avons pas vraiment de moyen de parler d'une *partie* d'une
chaine. Cependant, nous pourrions renvoyer l'indice de la fin du mot, indique par
un espace. Essayons cela, comme le montre le listing 4-7.

<Listing number="4-7" file-name="src/main.rs" caption="La fonction `first_word` qui renvoie une valeur d'indice en octets dans le parametre `String`">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

</Listing>

<!--
Because we need to go through the `String` element by element and check whether
a value is a space, we'll convert our `String` to an array of bytes using the
`as_bytes` method.
-->

Comme nous devons parcourir la `String` element par element et verifier si une
valeur est un espace, nous convertirons notre `String` en un tableau d'octets en
utilisant la methode `as_bytes`.


```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

<!--
Next, we create an iterator over the array of bytes using the `iter` method:
-->

Ensuite, nous creons un iterateur sur le tableau d'octets en utilisant la
methode `iter` :


```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

<!--
We'll discuss iterators in more detail in [Chapter 13][ch13] ignore
-->.
For now, know that `iter` is a method that returns each element in a collection
and that `enumerate` wraps the result of `iter` and returns each element as
part of a tuple instead. The first element of the tuple returned from
`enumerate` is the index, and the second element is a reference to the element.
This is a bit more convenient than calculating the index ourselves.
-->

Nous discuterons des iterateurs plus en detail au [chapitre 13][ch13]<!--
ignore
-->. Pour l'instant, sachez que `iter` est une methode qui renvoie chaque
element d'une collection et que `enumerate` enveloppe le resultat de `iter` et
renvoie chaque element sous forme de tuple a la place. Le premier element du
tuple renvoye par `enumerate` est l'indice, et le second element est une
reference vers l'element. C'est un peu plus pratique que de calculer l'indice
nous-memes.

<!--
Because the `enumerate` method returns a tuple, we can use patterns to
destructure that tuple. We'll be discussing patterns more in [Chapter
6][ch6] ignore
-->. In the `for` loop, we specify a pattern that has `i`
for the index in the tuple and `&item` for the single byte in the tuple.
Because we get a reference to the element from `.iter().enumerate()`, we use
`&` in the pattern.
-->

Comme la methode `enumerate` renvoie un tuple, nous pouvons utiliser des motifs
(patterns) pour destructurer ce tuple. Nous discuterons des motifs plus en detail
au [chapitre 6][ch6]<!--
ignore
-->. Dans la boucle `for`, nous specif ions un
motif qui a `i` pour l'indice dans le tuple et `&item` pour l'octet unique dans
le tuple. Comme nous obtenons une reference vers l'element depuis
`.iter().enumerate()`, nous utilisons `&` dans le motif.

<!--
Inside the `for` loop, we search for the byte that represents the space by
using the byte literal syntax. If we find a space, we return the position.
Otherwise, we return the length of the string by using `s.len()`.
-->

A l'interieur de la boucle `for`, nous recherchons l'octet qui represente
l'espace en utilisant la syntaxe de litteral d'octet. Si nous trouvons un
espace, nous renvoyons la position. Sinon, nous renvoyons la longueur de la
chaine en utilisant `s.len()`.


```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

<!--
We now have a way to find out the index of the end of the first word in the
string, but there's a problem. We're returning a `usize` on its own, but it's
only a meaningful number in the context of the `&String`. In other words,
because it's a separate value from the `String`, there's no guarantee that it
will still be valid in the future. Consider the program in Listing 4-8 that
uses the `first_word` function from Listing 4-7.
-->

Nous avons maintenant un moyen de trouver l'indice de la fin du premier mot dans
la chaine, mais il y a un probleme. Nous renvoyons un `usize` seul, mais c'est
un nombre qui n'a de sens que dans le contexte de la `&String`. En d'autres
termes, comme c'est une valeur separee de la `String`, il n'y a aucune garantie
qu'elle sera encore valide a l'avenir. Considerez le programme du listing 4-8 qui
utilise la fonction `first_word` du listing 4-7.

<Listing number="4-8" file-name="src/main.rs" caption="Stockage du resultat de l'appel a la fonction `first_word` puis modification du contenu de la `String`">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

</Listing>

<!--
This program compiles without any errors and would also do so if we used `word`
after calling `s.clear()`. Because `word` isn't connected to the state of `s`
at all, `word` still contains the value `5`. We could use that value `5` with
the variable `s` to try to extract the first word out, but this would be a bug
because the contents of `s` have changed since we saved `5` in `word`.
-->

Ce programme compile sans aucune erreur et le ferait egalement si nous utilisions
`word` apres avoir appele `s.clear()`. Comme `word` n'est pas du tout connecte a
l'etat de `s`, `word` contient toujours la valeur `5`. Nous pourrions utiliser
cette valeur `5` avec la variable `s` pour essayer d'extraire le premier mot,
mais ce serait un bug car le contenu de `s` a change depuis que nous avons
enregistre `5` dans `word`.

<!--
Having to worry about the index in `word` getting out of sync with the data in
`s` is tedious and error-prone! Managing these indices is even more brittle if
we write a `second_word` function. Its signature would have to look like this:
-->

Devoir s'inquieter de la desynchronisation de l'indice dans `word` avec les
donnees dans `s` est fastidieux et source d'erreurs ! La gestion de ces indices
est encore plus fragile si nous ecrivons une fonction `second_word`. Sa signature
devrait ressembler a ceci :

<!--
```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```
-->

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

<!--
Now we're tracking a starting _and_ an ending index, and we have even more
values that were calculated from data in a particular state but aren't tied to
that state at all. We have three unrelated variables floating around that need
to be kept in sync.
-->

Maintenant nous suivons un indice de debut _et_ un indice de fin, et nous avons
encore plus de valeurs qui ont ete calculees a partir de donnees dans un etat
particulier mais qui ne sont pas du tout liees a cet etat. Nous avons trois
variables non liees qui flottent et qui doivent etre maintenues synchronisees.

<!--
Luckily, Rust has a solution to this problem: string slices.
-->

Heureusement, Rust a une solution a ce probleme : les slices de chaines de
caracteres.

<!--
### String Slices
-->

### Les slices de chaines de caracteres

<!--
A _string slice_ is a reference to a contiguous sequence of the elements of a
`String`, and it looks like this:
-->

Une _slice de chaine_ est une reference a une sequence contigue des elements
d'une `String`, et elle ressemble a ceci :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

<!--
Rather than a reference to the entire `String`, `hello` is a reference to a
portion of the `String`, specified in the extra `[0..5]` bit. We create slices
using a range within square brackets by specifying
`[starting_index..ending_index]`, where _`starting_index`_ is the first
position in the slice and _`ending_index`_ is one more than the last position
in the slice. Internally, the slice data structure stores the starting position
and the length of the slice, which corresponds to _`ending_index`_ minus
_`starting_index`_. So, in the case of `let world = &s[6..11];`, `world` would
be a slice that contains a pointer to the byte at index 6 of `s` with a length
value of `5`.
-->

Plutot qu'une reference a la `String` entiere, `hello` est une reference a une
portion de la `String`, specifiee par la partie supplementaire `[0..5]`. Nous
creons des slices en utilisant un intervalle entre crochets en specifiant
`[indice_de_debut..indice_de_fin]`, ou _`indice_de_debut`_ est la premiere
position dans la slice et _`indice_de_fin`_ est un de plus que la derniere
position dans la slice. En interne, la structure de donnees de la slice stocke la
position de debut et la longueur de la slice, qui correspond a _`indice_de_fin`_
moins _`indice_de_debut`_. Ainsi, dans le cas de `let world = &s[6..11];`,
`world` serait une slice qui contient un pointeur vers l'octet a l'indice 6 de
`s` avec une valeur de longueur de `5`.

<!--
Figure 4-7 shows this in a diagram.
-->

La figure 4-7 montre cela dans un diagramme.

<!--
<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table represents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-07.svg" class="center" style="width: 50%;" />
-->

<img alt="Trois tableaux : un tableau representant les donnees de la pile de s,
qui pointe vers l'octet a l'indice 0 dans un tableau des donnees de chaine
&quot;hello world&quot; sur le tas. Le troisieme tableau represente les donnees
de la pile de la slice world, qui a une valeur de longueur de 5 et pointe vers
l'octet 6 du tableau de donnees du tas."
src="img/trpl04-07.svg" class="center" style="width: 50%;" />

<!--
<span class="caption">Figure 4-7: A string slice referring to part of a
`String`</span>
-->

<span class="caption">Figure 4-7 : Une slice de chaine faisant reference a une
partie d'une `String`</span>

<!--
With Rust's `..` range syntax, if you want to start at index 0, you can drop
the value before the two periods. In other words, these are equal:
-->

Avec la syntaxe d'intervalle `..` de Rust, si vous voulez commencer a l'indice
0, vous pouvez omettre la valeur avant les deux points. En d'autres termes,
ceux-ci sont equivalents :

<!--
```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```
-->

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

<!--
By the same token, if your slice includes the last byte of the `String`, you
can drop the trailing number. That means these are equal:
-->

De la meme facon, si votre slice inclut le dernier octet de la `String`, vous
pouvez omettre le nombre final. Cela signifie que ceux-ci sont equivalents :

<!--
```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```
-->

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

<!--
You can also drop both values to take a slice of the entire string. So, these
are equal:
-->

Vous pouvez egalement omettre les deux valeurs pour prendre une slice de la
chaine entiere. Ainsi, ceux-ci sont equivalents :

<!--
```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```
-->

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

<!--
> Note: String slice range indices must occur at valid UTF-8 character
> boundaries. If you attempt to create a string slice in the middle of a
> multibyte character, your program will exit with an error.
-->

> Note : Les indices d'intervalle des slices de chaines doivent se situer a des
> limites de caracteres UTF-8 valides. Si vous tentez de creer une slice de
> chaine au milieu d'un caractere multi-octets, votre programme se terminera
> avec une erreur.

<!--
With all this information in mind, let's rewrite `first_word` to return a
slice. The type that signifies "string slice" is written as `&str`:
-->

Avec toutes ces informations en tete, reecrivons `first_word` pour renvoyer une
slice. Le type qui designe "slice de chaine" s'ecrit `&str` :

<Listing file-name="src/main.rs">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

</Listing>

<!--
We get the index for the end of the word the same way we did in Listing 4-7, by
looking for the first occurrence of a space. When we find a space, we return a
string slice using the start of the string and the index of the space as the
starting and ending indices.
-->

Nous obtenons l'indice de la fin du mot de la meme facon que dans le listing
4-7, en recherchant la premiere occurrence d'un espace. Quand nous trouvons un
espace, nous renvoyons une slice de chaine en utilisant le debut de la chaine et
l'indice de l'espace comme indices de debut et de fin.

<!--
Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.
-->

Maintenant, quand nous appelons `first_word`, nous recuperons une seule valeur
qui est liee aux donnees sous-jacentes. La valeur est composee d'une reference
au point de depart de la slice et du nombre d'elements dans la slice.

<!--
Returning a slice would also work for a `second_word` function:
-->

Renvoyer une slice fonctionnerait egalement pour une fonction `second_word` :

<!--
```rust,ignore
fn second_word(s: &String) -> &str {
```
-->

```rust,ignore
fn second_word(s: &String) -> &str {
```

<!--
We now have a straightforward API that's much harder to mess up because the
compiler will ensure that the references into the `String` remain valid.
Remember the bug in the program in Listing 4-8, when we got the index to the
end of the first word but then cleared the string so our index was invalid?
That code was logically incorrect but didn't show any immediate errors. The
problems would show up later if we kept trying to use the first word index with
an emptied string. Slices make this bug impossible and let us know much sooner
that we have a problem with our code. Using the slice version of `first_word`
will throw a compile-time error:
-->

Nous avons maintenant une API simple qui est beaucoup plus difficile a mal
utiliser car le compilateur s'assurera que les references dans la `String`
restent valides. Rappelez-vous le bug dans le programme du listing 4-8, quand
nous avons obtenu l'indice de la fin du premier mot mais avons ensuite vide la
chaine, rendant notre indice invalide ? Ce code etait logiquement incorrect mais
ne montrait aucune erreur immediate. Les problemes apparaitraient plus tard si
nous continuions a essayer d'utiliser l'indice du premier mot avec une chaine
videe. Les slices rendent ce bug impossible et nous informent bien plus tot que
nous avons un probleme avec notre code. Utiliser la version avec slice de
`first_word` produira une erreur de compilation :

<Listing file-name="src/main.rs">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

</Listing>

<!--
Here's the compiler error:
-->

Voici l'erreur du compilateur :


```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

<!--
Recall from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Because `clear` needs to
truncate the `String`, it needs to get a mutable reference. The `println!`
after the call to `clear` uses the reference in `word`, so the immutable
reference must still be active at that point. Rust disallows the mutable
reference in `clear` and the immutable reference in `word` from existing at the
same time, and compilation fails. Not only has Rust made our API easier to use,
but it has also eliminated an entire class of errors at compile time!
-->

Rappelez-vous des regles d'emprunt que si nous avons une reference immuable vers
quelque chose, nous ne pouvons pas egalement prendre une reference mutable.
Comme `clear` a besoin de tronquer la `String`, elle a besoin d'obtenir une
reference mutable. Le `println!` apres l'appel a `clear` utilise la reference
dans `word`, donc la reference immuable doit encore etre active a ce moment-la.
Rust interdit a la reference mutable dans `clear` et a la reference immuable
dans `word` d'exister en meme temps, et la compilation echoue. Non seulement
Rust a rendu notre API plus facile a utiliser, mais il a aussi elimine toute une
classe d'erreurs a la compilation !

<!--
Old headings. Do not remove or links may break.
-->

<a id="string-literals-are-slices"></a>

<!--
#### String Literals as Slices
-->

#### Les litteraux de chaines comme slices

<!--
Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:
-->

Rappelez-vous que nous avons parle des litteraux de chaines stockes a
l'interieur du binaire. Maintenant que nous connaissons les slices, nous pouvons
comprendre correctement les litteraux de chaines :

<!--
```rust
let s = "Hello, world!";
```
-->

```rust
let s = "Hello, world!";
```

<!--
The type of `s` here is `&str`: It's a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.
-->

Le type de `s` ici est `&str` : c'est une slice pointant vers ce point specifique
du binaire. C'est aussi pourquoi les litteraux de chaines sont immuables ; `&str`
est une reference immuable.

<!--
#### String Slices as Parameters
-->

#### Les slices de chaines comme parametres

<!--
Knowing that you can take slices of literals and `String` values leads us to
one more improvement on `first_word`, and that's its signature:
-->

Savoir que vous pouvez prendre des slices de litteraux et de valeurs `String`
nous amene a une amelioration supplementaire de `first_word`, et c'est sa
signature :

<!--
```rust,ignore
fn first_word(s: &String) -> &str {
```
-->

```rust,ignore
fn first_word(s: &String) -> &str {
```

<!--
A more experienced Rustacean would write the signature shown in Listing 4-9
instead because it allows us to use the same function on both `&String` values
and `&str` values.
-->

Un Rustacean plus experimente ecrirait plutot la signature montree dans le
listing 4-9 car elle nous permet d'utiliser la meme fonction sur les valeurs
`&String` et les valeurs `&str`.

<Listing number="4-9" caption="Amelioration de la fonction `first_word` en utilisant une slice de chaine pour le type du parametre `s`">


```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

</Listing>

<!--
If we have a string slice, we can pass that directly. If we have a `String`, we
can pass a slice of the `String` or a reference to the `String`. This
flexibility takes advantage of deref coercions, a feature we will cover in
the ["Using Deref Coercions in Functions and Methods"][deref-coercions]
ignore
--> section of Chapter 15.
-->

Si nous avons une slice de chaine, nous pouvons la passer directement. Si nous
avons une `String`, nous pouvons passer une slice de la `String` ou une
reference a la `String`. Cette flexibilite tire parti des conversions
automatiques de dereferencement (deref coercions), une fonctionnalite que nous
couvrirons dans la section ["Utiliser les conversions automatiques de
dereferencement dans les fonctions et les methodes"][deref-coercions]<!--
ignore
--> du chapitre 15.

<!--
Defining a function to take a string slice instead of a reference to a `String`
makes our API more general and useful without losing any functionality:
-->

Definir une fonction pour prendre une slice de chaine au lieu d'une reference a
une `String` rend notre API plus generale et utile sans perdre de
fonctionnalite :

<Listing file-name="src/main.rs">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

</Listing>

<!--
### Other Slices
-->

### Les autres slices

<!--
String slices, as you might imagine, are specific to strings. But there's a
more general slice type too. Consider this array:
-->

Les slices de chaines, comme vous pouvez l'imaginer, sont specifiques aux
chaines. Mais il existe aussi un type de slice plus general. Considerez ce
tableau :

<!--
```rust
let a = [1, 2, 3, 4, 5];
```
-->

```rust
let a = [1, 2, 3, 4, 5];
```

<!--
Just as we might want to refer to part of a string, we might want to refer to
part of an array. We'd do so like this:
-->

Tout comme nous pourrions vouloir faire reference a une partie d'une chaine, nous
pourrions vouloir faire reference a une partie d'un tableau. Nous le ferions
comme ceci :

<!--
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
-->

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

<!--
This slice has the type `&[i32]`. It works the same way as string slices do, by
storing a reference to the first element and a length. You'll use this kind of
slice for all sorts of other collections. We'll discuss these collections in
detail when we talk about vectors in Chapter 8.
-->

Cette slice a le type `&[i32]`. Elle fonctionne de la meme maniere que les
slices de chaines, en stockant une reference vers le premier element et une
longueur. Vous utiliserez ce type de slice pour toutes sortes d'autres
collections. Nous discuterons de ces collections en detail quand nous parlerons
des vecteurs au chapitre 8.

<!--
## Summary
-->

## Resume

<!--
The concepts of ownership, borrowing, and slices ensure memory safety in Rust
programs at compile time. The Rust language gives you control over your memory
usage in the same way as other systems programming languages. But having the
owner of data automatically clean up that data when the owner goes out of scope
means you don't have to write and debug extra code to get this control.
-->

Les concepts de possession, d'emprunt et de slices garantissent la securite de
la memoire dans les programmes Rust a la compilation. Le langage Rust vous donne
le controle sur votre utilisation de la memoire de la meme facon que les autres
langages de programmation systeme. Mais le fait que le proprietaire des donnees
nettoie automatiquement ces donnees quand le proprietaire sort de la portee
signifie que vous n'avez pas a ecrire et deboguer du code supplementaire pour
obtenir ce controle.

<!--
Ownership affects how lots of other parts of Rust work, so we'll talk about
these concepts further throughout the rest of the book. Let's move on to
Chapter 5 and look at grouping pieces of data together in a `struct`.
-->

La possession affecte le fonctionnement de nombreuses autres parties de Rust,
nous continuerons donc a parler de ces concepts tout au long du reste du livre.
Passons au chapitre 5 et voyons comment regrouper des donnees dans une `struct`.

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#using-deref-coercions-in-functions-and-methods
