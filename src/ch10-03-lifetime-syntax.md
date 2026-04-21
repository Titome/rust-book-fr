<!--
## Validating References with Lifetimes
-->

## Valider les références avec les durées de vie

<!--
Lifetimes are another kind of generic that we've already been using. Rather
than ensuring that a type has the behavior we want, lifetimes ensure that
references are valid as long as we need them to be.
-->

Les durées de vie sont un autre type de générique que nous avons déjà utilisé.
Plutôt que de s'assurer qu'un type a le comportement que nous souhaitons, les
durées de vie s'assurent que les références sont valides aussi longtemps que
nous en avons besoin.

<!--
One detail we didn't discuss in the ["References and
Borrowing"][references-and-borrowing] ignore
--> section in Chapter 4 is
that every reference in Rust has a lifetime, which is the scope for which
that reference is valid. Most of the time, lifetimes are implicit and inferred,
just like most of the time, types are inferred. We are only required to
annotate types when multiple types are possible. In a similar way, we must
annotate lifetimes when the lifetimes of references could be related in a few
different ways. Rust requires us to annotate the relationships using generic
lifetime parameters to ensure that the actual references used at runtime will
definitely be valid.
-->

Un détail que nous n'avons pas abordé dans la section [« Les références et
l'emprunt »][references-and-borrowing]<!--
ignore
--> du chapitre 4 est que
chaque référence en Rust a une durée de vie, qui est la portée dans laquelle
cette référence est valide. La plupart du temps, les durées de vie sont
implicites et inférées, tout comme les types sont inférés la plupart du temps.
Nous ne sommes tenus d'annoter les types que lorsque plusieurs types sont
possibles. De la même manière, nous devons annoter les durées de vie lorsque
les durées de vie des références pourraient être liées de différentes
manières. Rust nous demande d'annoter les relations à l'aide de paramètres de
durée de vie génériques pour s'assurer que les références réellement utilisées
à l'exécution seront bien valides.

<!--
Annotating lifetimes is not even a concept most other programming languages
have, so this is going to feel unfamiliar. Although we won't cover lifetimes in
their entirety in this chapter, we'll discuss common ways you might encounter
lifetime syntax so that you can get comfortable with the concept.
-->

Annoter les durées de vie n'est même pas un concept que la plupart des autres
langages de programmation possèdent, donc cela va sembler inhabituel. Bien que
nous ne couvrirons pas les durées de vie dans leur intégralité dans ce
chapitre, nous aborderons les façons courantes dont vous pourriez rencontrer
la syntaxe des durées de vie afin que vous puissiez vous familiariser avec le
concept.

<!--
Old headings. Do not remove or links may break.
-->

<a id="preventing-dangling-references-with-lifetimes"></a>

<!--
### Dangling References
-->

### Les références pendantes

<!--
The main aim of lifetimes is to prevent dangling references, which, if they
were allowed to exist, would cause a program to reference data other than the
data it's intended to reference. Consider the program in Listing 10-16, which
has an outer scope and an inner scope.
-->

L'objectif principal des durées de vie est d'empêcher les références
pendantes (dangling references) qui, si elles étaient autorisées à exister,
amèneraient un programme à référencer des données autres que celles qu'il est
censé référencer. Considérez le programme de l'encart 10-16, qui a une portée
externe et une portée interne.

<Listing number="10-16" caption="Une tentative d'utilisation d'une référence dont la valeur est sortie de la portée">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

<!--
> Note: The examples in Listings 10-16, 10-17, and 10-23 declare variables
> without giving them an initial value, so the variable name exists in the outer
> scope. At first glance, this might appear to be in conflict with Rust having
> no null values. However, if we try to use a variable before giving it a value,
> we'll get a compile-time error, which shows that indeed Rust does not allow
> null values.
-->

> Remarque : les exemples des encarts 10-16, 10-17 et 10-23 déclarent des
> variables sans leur donner de valeur initiale, de sorte que le nom de la
> variable existe dans la portée externe. À première vue, cela pourrait
> sembler en conflit avec le fait que Rust n'a pas de valeurs nulles. Cependant,
> si nous essayons d'utiliser une variable avant de lui donner une valeur, nous
> obtiendrons une erreur de compilation, ce qui montre qu'effectivement Rust
> n'autorise pas les valeurs nulles.

<!--
The outer scope declares a variable named `r` with no initial value, and the
inner scope declares a variable named `x` with the initial value of `5`. Inside
the inner scope, we attempt to set the value of `r` as a reference to `x`.
Then, the inner scope ends, and we attempt to print the value in `r`. This code
won't compile, because the value that `r` is referring to has gone out of scope
before we try to use it. Here is the error message:
-->

La portée externe déclare une variable nommée `r` sans valeur initiale, et la
portée interne déclare une variable nommée `x` avec la valeur initiale `5`. À
l'intérieur de la portée interne, nous tentons de définir la valeur de `r`
comme une référence vers `x`. Ensuite, la portée interne se termine, et nous
tentons d'afficher la valeur dans `r`. Ce code ne compilera pas, car la valeur
à laquelle `r` fait référence est sortie de la portée avant que nous
n'essayions de l'utiliser. Voici le message d'erreur :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

<!--
The error message says that the variable `x` "does not live long enough." The
reason is that `x` will be out of scope when the inner scope ends on line 7.
But `r` is still valid for the outer scope; because its scope is larger, we say
that it "lives longer." If Rust allowed this code to work, `r` would be
referencing memory that was deallocated when `x` went out of scope, and
anything we tried to do with `r` wouldn't work correctly. So, how does Rust
determine that this code is invalid? It uses a borrow checker.
-->

Le message d'erreur dit que la variable `x` « ne vit pas assez longtemps ».
La raison est que `x` sera hors de portée lorsque la portée interne se
terminera à la ligne 7. Mais `r` est encore valide pour la portée externe ;
comme sa portée est plus grande, nous disons qu'elle « vit plus longtemps ».
Si Rust autorisait ce code à fonctionner, `r` référencerait de la mémoire qui
a été désallouée lorsque `x` est sorti de la portée, et tout ce que nous
essaierions de faire avec `r` ne fonctionnerait pas correctement. Alors,
comment Rust détermine-t-il que ce code est invalide ? Il utilise le
vérificateur d'emprunt (borrow checker).

<!--
### The Borrow Checker
-->

### Le vérificateur d'emprunt

<!--
The Rust compiler has a _borrow checker_ that compares scopes to determine
whether all borrows are valid. Listing 10-17 shows the same code as Listing
10-16 but with annotations showing the lifetimes of the variables.
-->

Le compilateur Rust possède un _vérificateur d'emprunt_ (borrow checker) qui
compare les portées pour déterminer si tous les emprunts sont valides. L'encart
10-17 montre le même code que l'encart 10-16 mais avec des annotations
montrant les durées de vie des variables.

<Listing number="10-17" caption="Annotations des durées de vie de `r` et `x`, nommées respectivement `'a` et `'b`">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

<!--
Here, we've annotated the lifetime of `r` with `'a` and the lifetime of `x`
with `'b`. As you can see, the inner `'b` block is much smaller than the outer
`'a` lifetime block. At compile time, Rust compares the size of the two
lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory
with a lifetime of `'b`. The program is rejected because `'b` is shorter than
`'a`: The subject of the reference doesn't live as long as the reference.
-->

Ici, nous avons annoté la durée de vie de `r` avec `'a` et la durée de vie de
`x` avec `'b`. Comme vous pouvez le voir, le bloc interne `'b` est beaucoup
plus petit que le bloc de durée de vie externe `'a`. Au moment de la
compilation, Rust compare la taille des deux durées de vie et constate que `r`
a une durée de vie de `'a` mais qu'il fait référence à de la mémoire avec une
durée de vie de `'b`. Le programme est rejeté car `'b` est plus court que
`'a` : le sujet de la référence ne vit pas aussi longtemps que la référence.

<!--
Listing 10-18 fixes the code so that it doesn't have a dangling reference and
it compiles without any errors.
-->

L'encart 10-18 corrige le code pour qu'il n'ait pas de référence pendante et
qu'il compile sans aucune erreur.

<Listing number="10-18" caption="Une référence valide car les données ont une durée de vie plus longue que la référence">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

<!--
Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x` because Rust knows that the reference in `r` will
always be valid while `x` is valid.
-->

Ici, `x` a la durée de vie `'b`, qui dans ce cas est plus grande que `'a`.
Cela signifie que `r` peut référencer `x` car Rust sait que la référence
dans `r` sera toujours valide tant que `x` est valide.

<!--
Now that you know where the lifetimes of references are and how Rust analyzes
lifetimes to ensure that references will always be valid, let's explore generic
lifetimes in function parameters and return values.
-->

Maintenant que vous savez où se trouvent les durées de vie des références et
comment Rust analyse les durées de vie pour s'assurer que les références
seront toujours valides, explorons les durées de vie génériques dans les
paramètres de fonction et les valeurs de retour.

<!--
### Generic Lifetimes in Functions
-->

### Les durées de vie génériques dans les fonctions

<!--
We'll write a function that returns the longer of two string slices. This
function will take two string slices and return a single string slice. After
we've implemented the `longest` function, the code in Listing 10-19 should
print `The longest string is abcd`.
-->

Nous allons écrire une fonction qui retourne la plus longue de deux slices de
chaînes de caractères. Cette fonction prendra deux slices de chaînes de
caractères et retournera une seule slice de chaîne de caractères. Après avoir
implémenté la fonction `longest`, le code de l'encart 10-19 devrait afficher
`The longest string is abcd`.

<Listing number="10-19" file-name="src/main.rs" caption="Une fonction `main` qui appelle la fonction `longest` pour trouver la plus longue de deux slices de chaînes de caractères">


```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

<!--
Note that we want the function to take string slices, which are references,
rather than strings, because we don't want the `longest` function to take
ownership of its parameters. Refer to ["String Slices as
Parameters"][string-slices-as-parameters] ignore
--> in Chapter 4 for more
discussion about why the parameters we use in Listing 10-19 are the ones we
want.
-->

Notez que nous voulons que la fonction prenne des slices de chaînes de
caractères, qui sont des références, plutôt que des chaînes de caractères,
car nous ne voulons pas que la fonction `longest` prenne la possession de ses
paramètres. Reportez-vous à [« Les slices de chaînes de caractères comme
paramètres »][string-slices-as-parameters]<!--
ignore
--> au chapitre 4 pour
plus de discussion sur les raisons pour lesquelles les paramètres que nous
utilisons dans l'encart 10-19 sont ceux que nous souhaitons.

<!--
If we try to implement the `longest` function as shown in Listing 10-20, it
won't compile.
-->

Si nous essayons d'implémenter la fonction `longest` comme montré dans l'encart
10-20, elle ne compilera pas.

<Listing number="10-20" file-name="src/main.rs" caption="Une implémentation de la fonction `longest` qui retourne la plus longue de deux slices de chaînes de caractères mais ne compile pas encore">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

<!--
Instead, we get the following error that talks about lifetimes:
-->

À la place, nous obtenons l'erreur suivante qui parle de durées de vie :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

<!--
The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can't tell whether the reference being returned refers to
`x` or `y`. Actually, we don't know either, because the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!
-->

Le texte d'aide révèle que le type de retour a besoin d'un paramètre de durée
de vie générique car Rust ne peut pas déterminer si la référence retournée fait
référence à `x` ou à `y`. En fait, nous ne le savons pas non plus, car le
bloc `if` dans le corps de cette fonction retourne une référence vers `x` et
le bloc `else` retourne une référence vers `y` !

<!--
When we're defining this function, we don't know the concrete values that will
be passed into this function, so we don't know whether the `if` case or the
`else` case will execute. We also don't know the concrete lifetimes of the
references that will be passed in, so we can't look at the scopes as we did in
Listings 10-17 and 10-18 to determine whether the reference we return will
always be valid. The borrow checker can't determine this either, because it
doesn't know how the lifetimes of `x` and `y` relate to the lifetime of the
return value. To fix this error, we'll add generic lifetime parameters that
define the relationship between the references so that the borrow checker can
perform its analysis.
-->

Lorsque nous définissons cette fonction, nous ne connaissons pas les valeurs
concrètes qui seront passées à cette fonction, donc nous ne savons pas si le
cas `if` ou le cas `else` s'exécutera. Nous ne connaissons pas non plus les
durées de vie concrètes des références qui seront passées, donc nous ne pouvons
pas examiner les portées comme nous l'avons fait dans les encarts 10-17 et
10-18 pour déterminer si la référence que nous retournons sera toujours valide.
Le vérificateur d'emprunt ne peut pas non plus le déterminer, car il ne sait
pas comment les durées de vie de `x` et `y` sont liées à la durée de vie de la
valeur de retour. Pour corriger cette erreur, nous ajouterons des paramètres de
durée de vie génériques qui définissent la relation entre les références afin
que le vérificateur d'emprunt puisse effectuer son analyse.

<!--
### Lifetime Annotation Syntax
-->

### La syntaxe des annotations de durée de vie

<!--
Lifetime annotations don't change how long any of the references live. Rather,
they describe the relationships of the lifetimes of multiple references to each
other without affecting the lifetimes. Just as functions can accept any type
when the signature specifies a generic type parameter, functions can accept
references with any lifetime by specifying a generic lifetime parameter.
-->

Les annotations de durée de vie ne changent pas la durée de vie d'une
référence. Elles décrivent plutôt les relations entre les durées de vie de
plusieurs références sans affecter les durées de vie elles-mêmes. Tout comme
les fonctions peuvent accepter n'importe quel type lorsque la signature
spécifie un paramètre de type générique, les fonctions peuvent accepter des
références avec n'importe quelle durée de vie en spécifiant un paramètre de
durée de vie générique.

<!--
Lifetime annotations have a slightly unusual syntax: The names of lifetime
parameters must start with an apostrophe (`'`) and are usually all lowercase
and very short, like generic types. Most people use the name `'a` for the first
lifetime annotation. We place lifetime parameter annotations after the `&` of a
reference, using a space to separate the annotation from the reference's type.
-->

Les annotations de durée de vie ont une syntaxe légèrement inhabituelle : les
noms des paramètres de durée de vie doivent commencer par une apostrophe (`'`)
et sont généralement tout en minuscules et très courts, comme les types
génériques. La plupart des gens utilisent le nom `'a` pour la première
annotation de durée de vie. Nous plaçons les annotations de paramètre de durée
de vie après le `&` d'une référence, en utilisant un espace pour séparer
l'annotation du type de la référence.

<!--
Here are some examples—a reference to an `i32` without a lifetime parameter, a
reference to an `i32` that has a lifetime parameter named `'a`, and a mutable
reference to an `i32` that also has the lifetime `'a`:
-->

Voici quelques exemples : une référence vers un `i32` sans paramètre de durée
de vie, une référence vers un `i32` qui a un paramètre de durée de vie nommé
`'a`, et une référence mutable vers un `i32` qui a aussi la durée de vie `'a` :

<!--
```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
-->

```rust,ignore
&i32        // une référence
&'a i32     // une référence avec une durée de vie explicite
&'a mut i32 // une référence mutable avec une durée de vie explicite
```

<!--
One lifetime annotation by itself doesn't have much meaning, because the
annotations are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other. Let's examine how the lifetime annotations
relate to each other in the context of the `longest` function.
-->

Une seule annotation de durée de vie n'a pas beaucoup de sens en elle-même, car
les annotations sont destinées à indiquer à Rust comment les paramètres de
durée de vie génériques de plusieurs références sont liés les uns aux autres.
Examinons comment les annotations de durée de vie sont liées les unes aux
autres dans le contexte de la fonction `longest`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="lifetime-annotations-in-function-signatures"></a>

<!--
### In Function Signatures
-->

### Dans les signatures de fonctions

<!--
To use lifetime annotations in function signatures, we need to declare the
generic lifetime parameters inside angle brackets between the function name and
the parameter list, just as we did with generic type parameters.
-->

Pour utiliser les annotations de durée de vie dans les signatures de fonctions,
nous devons déclarer les paramètres de durée de vie génériques entre chevrons
entre le nom de la fonction et la liste des paramètres, tout comme nous
l'avons fait avec les paramètres de type générique.

<!--
We want the signature to express the following constraint: The returned
reference will be valid as long as both of the parameters are valid. This is
the relationship between lifetimes of the parameters and the return value.
We'll name the lifetime `'a` and then add it to each reference, as shown in
Listing 10-21.
-->

Nous voulons que la signature exprime la contrainte suivante : la référence
retournée sera valide tant que les deux paramètres sont valides. C'est la
relation entre les durées de vie des paramètres et la valeur de retour. Nous
nommerons la durée de vie `'a` puis l'ajouterons à chaque référence, comme
montré dans l'encart 10-21.

<Listing number="10-21" file-name="src/main.rs" caption="La définition de la fonction `longest` spécifiant que toutes les références dans la signature doivent avoir la même durée de vie `'a`">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

<!--
This code should compile and produce the result we want when we use it with the
`main` function in Listing 10-19.
-->

Ce code devrait compiler et produire le résultat souhaité lorsque nous
l'utilisons avec la fonction `main` de l'encart 10-19.

<!--
The function signature now tells Rust that for some lifetime `'a`, the function
takes two parameters, both of which are string slices that live at least as
long as lifetime `'a`. The function signature also tells Rust that the string
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the
`longest` function is the same as the smaller of the lifetimes of the values
referred to by the function arguments. These relationships are what we want
Rust to use when analyzing this code.
-->

La signature de la fonction indique maintenant à Rust que pour une certaine
durée de vie `'a`, la fonction prend deux paramètres, qui sont tous les deux
des slices de chaînes de caractères qui vivent au moins aussi longtemps que la
durée de vie `'a`. La signature de la fonction indique aussi à Rust que la
slice de chaîne de caractères retournée par la fonction vivra au moins aussi
longtemps que la durée de vie `'a`. En pratique, cela signifie que la durée
de vie de la référence retournée par la fonction `longest` est la même que la
plus petite des durées de vie des valeurs référencées par les arguments de la
fonction. Ce sont ces relations que nous voulons que Rust utilise lors de
l'analyse de ce code.

<!--
Remember, when we specify the lifetime parameters in this function signature,
we're not changing the lifetimes of any values passed in or returned. Rather,
we're specifying that the borrow checker should reject any values that don't
adhere to these constraints. Note that the `longest` function doesn't need to
know exactly how long `x` and `y` will live, only that some scope can be
substituted for `'a` that will satisfy this signature.
-->

Rappelez-vous, lorsque nous spécifions les paramètres de durée de vie dans
cette signature de fonction, nous ne changeons pas les durées de vie des
valeurs passées ou retournées. Nous spécifions plutôt que le vérificateur
d'emprunt doit rejeter toute valeur qui ne respecte pas ces contraintes. Notez
que la fonction `longest` n'a pas besoin de savoir exactement combien de temps
`x` et `y` vivront, seulement qu'une certaine portée peut être substituée à
`'a` qui satisfera cette signature.

<!--
When annotating lifetimes in functions, the annotations go in the function
signature, not in the function body. The lifetime annotations become part of
the contract of the function, much like the types in the signature. Having
function signatures contain the lifetime contract means the analysis the Rust
compiler does can be simpler. If there's a problem with the way a function is
annotated or the way it is called, the compiler errors can point to the part of
our code and the constraints more precisely. If, instead, the Rust compiler
made more inferences about what we intended the relationships of the lifetimes
to be, the compiler might only be able to point to a use of our code many steps
away from the cause of the problem.
-->

Lorsque nous annotons les durées de vie dans les fonctions, les annotations
vont dans la signature de la fonction, pas dans le corps de la fonction. Les
annotations de durée de vie font partie du contrat de la fonction, tout comme
les types dans la signature. Avoir des signatures de fonctions contenant le
contrat de durée de vie signifie que l'analyse effectuée par le compilateur
Rust peut être plus simple. S'il y a un problème avec la façon dont une
fonction est annotée ou la façon dont elle est appelée, les erreurs du
compilateur peuvent pointer plus précisément vers la partie de notre code et
les contraintes. Si, au contraire, le compilateur Rust faisait plus
d'inférences sur ce que nous voulions que soient les relations entre les durées
de vie, le compilateur ne pourrait peut-être pointer que vers une utilisation
de notre code à plusieurs étapes de la cause du problème.

<!--
When we pass concrete references to `longest`, the concrete lifetime that is
substituted for `'a` is the part of the scope of `x` that overlaps with the
scope of `y`. In other words, the generic lifetime `'a` will get the concrete
lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because
we've annotated the returned reference with the same lifetime parameter `'a`,
the returned reference will also be valid for the length of the smaller of the
lifetimes of `x` and `y`.
-->

Lorsque nous passons des références concrètes à `longest`, la durée de vie
concrète qui est substituée à `'a` est la partie de la portée de `x` qui
chevauche la portée de `y`. En d'autres termes, la durée de vie générique `'a`
obtiendra la durée de vie concrète qui est égale à la plus petite des durées
de vie de `x` et `y`. Comme nous avons annoté la référence retournée avec le
même paramètre de durée de vie `'a`, la référence retournée sera aussi valide
pour la durée de la plus petite des durées de vie de `x` et `y`.

<!--
Let's look at how the lifetime annotations restrict the `longest` function by
passing in references that have different concrete lifetimes. Listing 10-22 is
a straightforward example.
-->

Voyons comment les annotations de durée de vie restreignent la fonction
`longest` en passant des références qui ont des durées de vie concrètes
différentes. L'encart 10-22 est un exemple simple.

<Listing number="10-22" file-name="src/main.rs" caption="Utilisation de la fonction `longest` avec des références vers des valeurs `String` qui ont des durées de vie concrètes différentes">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

<!--
In this example, `string1` is valid until the end of the outer scope, `string2`
is valid until the end of the inner scope, and `result` references something
that is valid until the end of the inner scope. Run this code and you'll see
that the borrow checker approves; it will compile and print `The longest string
is long string is long`.
-->

Dans cet exemple, `string1` est valide jusqu'à la fin de la portée externe,
`string2` est valide jusqu'à la fin de la portée interne, et `result`
référence quelque chose qui est valide jusqu'à la fin de la portée interne.
Exécutez ce code et vous verrez que le vérificateur d'emprunt approuve ; il
compilera et affichera `The longest string is long string is long`.

<!--
Next, let's try an example that shows that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We'll move the
declaration of the `result` variable outside the inner scope but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Then, we'll move the `println!` that uses `result` to outside the
inner scope, after the inner scope has ended. The code in Listing 10-23 will
not compile.
-->

Ensuite, essayons un exemple qui montre que la durée de vie de la référence
dans `result` doit être la plus petite durée de vie des deux arguments. Nous
déplacerons la déclaration de la variable `result` à l'extérieur de la portée
interne mais laisserons l'assignation de la valeur à la variable `result` à
l'intérieur de la portée avec `string2`. Ensuite, nous déplacerons le
`println!` qui utilise `result` à l'extérieur de la portée interne, après la
fin de la portée interne. Le code de l'encart 10-23 ne compilera pas.

<Listing number="10-23" file-name="src/main.rs" caption="Tentative d'utilisation de `result` après que `string2` est sorti de la portée">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

<!--
When we try to compile this code, we get this error:
-->

Lorsque nous essayons de compiler ce code, nous obtenons cette erreur :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

<!--
The error shows that for `result` to be valid for the `println!` statement,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values using the same lifetime parameter `'a`.
-->

L'erreur montre que pour que `result` soit valide pour l'instruction
`println!`, `string2` devrait être valide jusqu'à la fin de la portée externe.
Rust le sait car nous avons annoté les durées de vie des paramètres de
fonction et des valeurs de retour en utilisant le même paramètre de durée de
vie `'a`.

<!--
As humans, we can look at this code and see that `string1` is longer than
`string2`, and therefore, `result` will contain a reference to `string1`.
Because `string1` has not gone out of scope yet, a reference to `string1` will
still be valid for the `println!` statement. However, the compiler can't see
that the reference is valid in this case. We've told Rust that the lifetime of
the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the references passed in. Therefore, the borrow checker
disallows the code in Listing 10-23 as possibly having an invalid reference.
-->

En tant qu'humains, nous pouvons regarder ce code et voir que `string1` est
plus longue que `string2`, et donc, `result` contiendra une référence vers
`string1`. Comme `string1` n'est pas encore sortie de la portée, une référence
vers `string1` sera toujours valide pour l'instruction `println!`. Cependant,
le compilateur ne peut pas voir que la référence est valide dans ce cas. Nous
avons dit à Rust que la durée de vie de la référence retournée par la fonction
`longest` est la même que la plus petite des durées de vie des références
passées. Par conséquent, le vérificateur d'emprunt interdit le code de
l'encart 10-23 comme pouvant avoir une référence invalide.

<!--
Try designing more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether or not your experiments will pass the
borrow checker before you compile; then, check to see if you're right!
-->

Essayez de concevoir d'autres expériences qui font varier les valeurs et les
durées de vie des références passées à la fonction `longest` et la façon dont
la référence retournée est utilisée. Faites des hypothèses sur le fait que vos
expériences passeront ou non le vérificateur d'emprunt avant de compiler ;
puis, vérifiez si vous avez raison !

<!--
Old headings. Do not remove or links may break.
-->

<a id="thinking-in-terms-of-lifetimes"></a>

<!--
### Relationships
-->

### Les relations

<!--
The way in which you need to specify lifetime parameters depends on what your
function is doing. For example, if we changed the implementation of the
`longest` function to always return the first parameter rather than the longest
string slice, we wouldn't need to specify a lifetime on the `y` parameter. The
following code will compile:
-->

La façon dont vous devez spécifier les paramètres de durée de vie dépend de ce
que fait votre fonction. Par exemple, si nous changions l'implémentation de la
fonction `longest` pour toujours retourner le premier paramètre plutôt que la
plus longue slice de chaîne de caractères, nous n'aurions pas besoin de
spécifier une durée de vie sur le paramètre `y`. Le code suivant compilera :

<Listing file-name="src/main.rs">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

<!--
We've specified a lifetime parameter `'a` for the parameter `x` and the return
type, but not for the parameter `y`, because the lifetime of `y` does not have
any relationship with the lifetime of `x` or the return value.
-->

Nous avons spécifié un paramètre de durée de vie `'a` pour le paramètre `x`
et le type de retour, mais pas pour le paramètre `y`, car la durée de vie de
`y` n'a aucune relation avec la durée de vie de `x` ou la valeur de retour.

<!--
When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for one of the parameters. If
the reference returned does _not_ refer to one of the parameters, it must refer
to a value created within this function. However, this would be a dangling
reference because the value will go out of scope at the end of the function.
Consider this attempted implementation of the `longest` function that won't
compile:
-->

Lorsqu'on retourne une référence depuis une fonction, le paramètre de durée
de vie pour le type de retour doit correspondre au paramètre de durée de vie
de l'un des paramètres. Si la référence retournée ne fait _pas_ référence à
l'un des paramètres, elle doit faire référence à une valeur créée dans cette
fonction. Cependant, ce serait une référence pendante car la valeur sortira
de la portée à la fin de la fonction. Considérez cette tentative
d'implémentation de la fonction `longest` qui ne compilera pas :

<Listing file-name="src/main.rs">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

<!--
Here, even though we've specified a lifetime parameter `'a` for the return
type, this implementation will fail to compile because the return value
lifetime is not related to the lifetime of the parameters at all. Here is the
error message we get:
-->

Ici, même si nous avons spécifié un paramètre de durée de vie `'a` pour le
type de retour, cette implémentation ne compilera pas car la durée de vie de
la valeur de retour n'est pas du tout liée à la durée de vie des paramètres.
Voici le message d'erreur que nous obtenons :


```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

<!--
The problem is that `result` goes out of scope and gets cleaned up at the end
of the `longest` function. We're also trying to return a reference to `result`
from the function. There is no way we can specify lifetime parameters that
would change the dangling reference, and Rust won't let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so that the calling function is then responsible for
cleaning up the value.
-->

Le problème est que `result` sort de la portée et est nettoyé à la fin de la
fonction `longest`. Nous essayons aussi de retourner une référence vers
`result` depuis la fonction. Il n'y a aucun moyen de spécifier des paramètres
de durée de vie qui changeraient la référence pendante, et Rust ne nous
laissera pas créer une référence pendante. Dans ce cas, la meilleure solution
serait de retourner un type de données possédé plutôt qu'une référence afin
que la fonction appelante soit alors responsable du nettoyage de la valeur.

<!--
Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they're connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.
-->

En fin de compte, la syntaxe des durées de vie consiste à connecter les durées
de vie des différents paramètres et valeurs de retour des fonctions. Une fois
qu'elles sont connectées, Rust a suffisamment d'informations pour autoriser
les opérations sûres pour la mémoire et interdire les opérations qui
créeraient des pointeurs pendants ou violeraient autrement la sécurité de la
mémoire.

<!--
Old headings. Do not remove or links may break.
-->

<a id="lifetime-annotations-in-struct-definitions"></a>

<!--
### In Struct Definitions
-->

### Dans les définitions de structs

<!--
So far, the structs we've defined all hold owned types. We can define structs
to hold references, but in that case, we would need to add a lifetime
annotation on every reference in the struct's definition. Listing 10-24 has a
struct named `ImportantExcerpt` that holds a string slice.
-->

Jusqu'à présent, les structs que nous avons définies contiennent toutes des
types possédés. Nous pouvons définir des structs qui contiennent des
références, mais dans ce cas, nous devrions ajouter une annotation de durée de
vie sur chaque référence dans la définition de la struct. L'encart 10-24
possède une struct nommée `ImportantExcerpt` qui contient une slice de chaîne
de caractères.

<Listing number="10-24" file-name="src/main.rs" caption="Une struct qui contient une référence, nécessitant une annotation de durée de vie">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

<!--
This struct has the single field `part` that holds a string slice, which is a
reference. As with generic data types, we declare the name of the generic
lifetime parameter inside angle brackets after the name of the struct so that
we can use the lifetime parameter in the body of the struct definition. This
annotation means an instance of `ImportantExcerpt` can't outlive the reference
it holds in its `part` field.
-->

Cette struct a un seul champ `part` qui contient une slice de chaîne de
caractères, qui est une référence. Comme pour les types de données génériques,
nous déclarons le nom du paramètre de durée de vie générique entre chevrons
après le nom de la struct afin de pouvoir utiliser le paramètre de durée de
vie dans le corps de la définition de la struct. Cette annotation signifie
qu'une instance de `ImportantExcerpt` ne peut pas survivre à la référence
qu'elle contient dans son champ `part`.

<!--
The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`. The data in `novel` exists before the `ImportantExcerpt`
instance is created. In addition, `novel` doesn't go out of scope until after
the `ImportantExcerpt` goes out of scope, so the reference in the
`ImportantExcerpt` instance is valid.
-->

La fonction `main` ici crée une instance de la struct `ImportantExcerpt` qui
contient une référence vers la première phrase du `String` possédé par la
variable `novel`. Les données dans `novel` existent avant que l'instance de
`ImportantExcerpt` ne soit créée. De plus, `novel` ne sort pas de la portée
avant que `ImportantExcerpt` ne sorte de la portée, donc la référence dans
l'instance de `ImportantExcerpt` est valide.

<!--
### Lifetime Elision
-->

### L'élision des durées de vie

<!--
You've learned that every reference has a lifetime and that you need to specify
lifetime parameters for functions or structs that use references. However, we
had a function in Listing 4-9, shown again in Listing 10-25, that compiled
without lifetime annotations.
-->

Vous avez appris que chaque référence a une durée de vie et que vous devez
spécifier des paramètres de durée de vie pour les fonctions ou structs qui
utilisent des références. Cependant, nous avions une fonction dans l'encart
4-9, montrée à nouveau dans l'encart 10-25, qui compilait sans annotations de
durée de vie.

<Listing number="10-25" file-name="src/lib.rs" caption="Une fonction que nous avons définie dans l'encart 4-9 qui compilait sans annotations de durée de vie, même si le paramètre et le type de retour sont des références">


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

<!--
The reason this function compiles without lifetime annotations is historical:
In early versions (pre-1.0) of Rust, this code wouldn't have compiled, because
every reference needed an explicit lifetime. At that time, the function
signature would have been written like this:
-->

La raison pour laquelle cette fonction compile sans annotations de durée de vie
est historique : dans les premières versions (pré-1.0) de Rust, ce code
n'aurait pas compilé, car chaque référence nécessitait une durée de vie
explicite. À cette époque, la signature de la fonction aurait été écrite
comme ceci :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```
-->

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

<!--
After writing a lot of Rust code, the Rust team found that Rust programmers
were entering the same lifetime annotations over and over in particular
situations. These situations were predictable and followed a few deterministic
patterns. The developers programmed these patterns into the compiler's code so
that the borrow checker could infer the lifetimes in these situations and
wouldn't need explicit annotations.
-->

Après avoir écrit beaucoup de code Rust, l'équipe Rust a constaté que les
programmeurs Rust saisissaient les mêmes annotations de durée de vie encore
et encore dans des situations particulières. Ces situations étaient prévisibles
et suivaient quelques modèles déterministes. Les développeurs ont programmé
ces modèles dans le code du compilateur afin que le vérificateur d'emprunt
puisse inférer les durées de vie dans ces situations sans avoir besoin
d'annotations explicites.

<!--
This piece of Rust history is relevant because it's possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.
-->

Ce morceau d'histoire de Rust est pertinent car il est possible que d'autres
modèles déterministes émergent et soient ajoutés au compilateur. À l'avenir,
encore moins d'annotations de durée de vie pourraient être nécessaires.

<!--
The patterns programmed into Rust's analysis of references are called the
_lifetime elision rules_. These aren't rules for programmers to follow; they're
a set of particular cases that the compiler will consider, and if your code
fits these cases, you don't need to write the lifetimes explicitly.
-->

Les modèles programmés dans l'analyse des références par Rust sont appelés
les _règles d'élision des durées de vie_. Ce ne sont pas des règles que les
programmeurs doivent suivre ; ce sont un ensemble de cas particuliers que le
compilateur considérera, et si votre code correspond à ces cas, vous n'avez
pas besoin d'écrire les durées de vie explicitement.

<!--
The elision rules don't provide full inference. If there is still ambiguity
about what lifetimes the references have after Rust applies the rules, the
compiler won't guess what the lifetime of the remaining references should be.
Instead of guessing, the compiler will give you an error that you can resolve
by adding the lifetime annotations.
-->

Les règles d'élision ne fournissent pas une inférence complète. S'il reste
de l'ambiguïté sur les durées de vie des références après que Rust a appliqué
les règles, le compilateur ne devinera pas quelle devrait être la durée de vie
des références restantes. Au lieu de deviner, le compilateur vous donnera une
erreur que vous pouvez résoudre en ajoutant les annotations de durée de vie.

<!--
Lifetimes on function or method parameters are called _input lifetimes_, and
lifetimes on return values are called _output lifetimes_.
-->

Les durées de vie sur les paramètres de fonction ou de méthode sont appelées
_durées de vie d'entrée_, et les durées de vie sur les valeurs de retour sont
appelées _durées de vie de sortie_.

<!--
The compiler uses three rules to figure out the lifetimes of the references
when there aren't explicit annotations. The first rule applies to input
lifetimes, and the second and third rules apply to output lifetimes. If the
compiler gets to the end of the three rules and there are still references for
which it can't figure out lifetimes, the compiler will stop with an error.
These rules apply to `fn` definitions as well as `impl` blocks.
-->

Le compilateur utilise trois règles pour déterminer les durées de vie des
références lorsqu'il n'y a pas d'annotations explicites. La première règle
s'applique aux durées de vie d'entrée, et les deuxième et troisième règles
s'appliquent aux durées de vie de sortie. Si le compilateur arrive à la fin
des trois règles et qu'il reste des références pour lesquelles il ne peut pas
déterminer les durées de vie, le compilateur s'arrêtera avec une erreur. Ces
règles s'appliquent aux définitions `fn` ainsi qu'aux blocs `impl`.

<!--
The first rule is that the compiler assigns a lifetime parameter to each
parameter that's a reference. In other words, a function with one parameter
gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two
parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32,
y: &'b i32)`; and so on.
-->

La première règle est que le compilateur assigne un paramètre de durée de vie
à chaque paramètre qui est une référence. En d'autres termes, une fonction
avec un paramètre obtient un paramètre de durée de vie : `fn foo<'a>(x: &'a
i32)` ; une fonction avec deux paramètres obtient deux paramètres de durée de
vie séparés : `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` ; et ainsi de suite.

<!--
The second rule is that, if there is exactly one input lifetime parameter, that
lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32)
-> &'a i32`.
-->

La deuxième règle est que, s'il y a exactement un paramètre de durée de vie
d'entrée, cette durée de vie est assignée à tous les paramètres de durée de
vie de sortie : `fn foo<'a>(x: &'a i32) -> &'a i32`.

<!--
The third rule is that, if there are multiple input lifetime parameters, but
one of them is `&self` or `&mut self` because this is a method, the lifetime of
`self` is assigned to all output lifetime parameters. This third rule makes
methods much nicer to read and write because fewer symbols are necessary.
-->

La troisième règle est que, s'il y a plusieurs paramètres de durée de vie
d'entrée, mais que l'un d'eux est `&self` ou `&mut self` parce qu'il s'agit
d'une méthode, la durée de vie de `self` est assignée à tous les paramètres
de durée de vie de sortie. Cette troisième règle rend les méthodes beaucoup
plus agréables à lire et à écrire car moins de symboles sont nécessaires.

<!--
Let's pretend we're the compiler. We'll apply these rules to figure out the
lifetimes of the references in the signature of the `first_word` function in
Listing 10-25. The signature starts without any lifetimes associated with the
references:
-->

Faisons comme si nous étions le compilateur. Nous appliquerons ces règles pour
déterminer les durées de vie des références dans la signature de la fonction
`first_word` de l'encart 10-25. La signature commence sans aucune durée de vie
associée aux références :

<!--
```rust,ignore
fn first_word(s: &str) -> &str {
```
-->

```rust,ignore
fn first_word(s: &str) -> &str {
```

<!--
Then, the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We'll call it `'a` as usual, so now the signature is
this:
-->

Ensuite, le compilateur applique la première règle, qui spécifie que chaque
paramètre obtient sa propre durée de vie. Nous l'appellerons `'a` comme
d'habitude, donc maintenant la signature est celle-ci :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```
-->

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

<!--
The second rule applies because there is exactly one input lifetime. The second
rule specifies that the lifetime of the one input parameter gets assigned to
the output lifetime, so the signature is now this:
-->

La deuxième règle s'applique car il y a exactement une durée de vie d'entrée.
La deuxième règle spécifie que la durée de vie du seul paramètre d'entrée est
assignée à la durée de vie de sortie, donc la signature est maintenant
celle-ci :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```
-->

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

<!--
Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.
-->

Maintenant, toutes les références dans cette signature de fonction ont des
durées de vie, et le compilateur peut continuer son analyse sans avoir besoin
que le programmeur annote les durées de vie dans cette signature de fonction.

<!--
Let's look at another example, this time using the `longest` function that had
no lifetime parameters when we started working with it in Listing 10-20:
-->

Regardons un autre exemple, cette fois en utilisant la fonction `longest` qui
n'avait pas de paramètres de durée de vie lorsque nous avons commencé à
travailler avec dans l'encart 10-20 :

<!--
```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```
-->

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

<!--
Let's apply the first rule: Each parameter gets its own lifetime. This time we
have two parameters instead of one, so we have two lifetimes:
-->

Appliquons la première règle : chaque paramètre obtient sa propre durée de
vie. Cette fois nous avons deux paramètres au lieu d'un, donc nous avons deux
durées de vie :

<!--
```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
-->

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

<!--
You can see that the second rule doesn't apply, because there is more than one
input lifetime. The third rule doesn't apply either, because `longest` is a
function rather than a method, so none of the parameters are `self`. After
working through all three rules, we still haven't figured out what the return
type's lifetime is. This is why we got an error trying to compile the code in
Listing 10-20: The compiler worked through the lifetime elision rules but still
couldn't figure out all the lifetimes of the references in the signature.
-->

Vous pouvez voir que la deuxième règle ne s'applique pas, car il y a plus
d'une durée de vie d'entrée. La troisième règle ne s'applique pas non plus,
car `longest` est une fonction plutôt qu'une méthode, donc aucun des
paramètres n'est `self`. Après avoir appliqué les trois règles, nous n'avons
toujours pas déterminé quelle est la durée de vie du type de retour. C'est
pourquoi nous avons obtenu une erreur en essayant de compiler le code de
l'encart 10-20 : le compilateur a appliqué les règles d'élision des durées de
vie mais n'a toujours pas pu déterminer toutes les durées de vie des
références dans la signature.

<!--
Because the third rule really only applies in method signatures, we'll look at
lifetimes in that context next to see why the third rule means we don't have to
annotate lifetimes in method signatures very often.
-->

Comme la troisième règle ne s'applique vraiment que dans les signatures de
méthodes, nous examinerons les durées de vie dans ce contexte ensuite pour
voir pourquoi la troisième règle fait que nous n'avons pas à annoter les
durées de vie dans les signatures de méthodes très souvent.

<!--
Old headings. Do not remove or links may break.
-->

<a id="lifetime-annotations-in-method-definitions"></a>

<!--
### In Method Definitions
-->

### Dans les définitions de méthodes

<!--
When we implement methods on a struct with lifetimes, we use the same syntax as
that of generic type parameters, as shown in Listing 10-11. Where we declare
and use the lifetime parameters depends on whether they're related to the
struct fields or the method parameters and return values.
-->

Lorsque nous implémentons des méthodes sur une struct avec des durées de vie,
nous utilisons la même syntaxe que celle des paramètres de type générique,
comme montré dans l'encart 10-11. L'endroit où nous déclarons et utilisons les
paramètres de durée de vie dépend de s'ils sont liés aux champs de la struct
ou aux paramètres et valeurs de retour de la méthode.

<!--
Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct's name because those lifetimes are part
of the struct's type.
-->

Les noms de durée de vie pour les champs de la struct doivent toujours être
déclarés après le mot-clé `impl` puis utilisés après le nom de la struct car
ces durées de vie font partie du type de la struct.

<!--
In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct's fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren't necessary in method signatures. Let's look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-24.
-->

Dans les signatures de méthodes à l'intérieur du bloc `impl`, les références
peuvent être liées à la durée de vie des références dans les champs de la
struct, ou elles peuvent être indépendantes. De plus, les règles d'élision des
durées de vie font souvent en sorte que les annotations de durée de vie ne
soient pas nécessaires dans les signatures de méthodes. Examinons quelques
exemples en utilisant la struct nommée `ImportantExcerpt` que nous avons
définie dans l'encart 10-24.

<!--
First, we'll use a method named `level` whose only parameter is a reference to
`self` and whose return value is an `i32`, which is not a reference to anything:
-->

D'abord, nous utiliserons une méthode nommée `level` dont le seul paramètre
est une référence vers `self` et dont la valeur de retour est un `i32`, qui
n'est pas une référence vers quoi que ce soit :


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

<!--
The lifetime parameter declaration after `impl` and its use after the type name
are required, but because of the first elision rule, we're not required to
annotate the lifetime of the reference to `self`.
-->

La déclaration du paramètre de durée de vie après `impl` et son utilisation
après le nom du type sont requises, mais en raison de la première règle
d'élision, nous ne sommes pas tenus d'annoter la durée de vie de la référence
vers `self`.

<!--
Here is an example where the third lifetime elision rule applies:
-->

Voici un exemple où la troisième règle d'élision des durées de vie
s'applique :


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

<!--
There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.
-->

Il y a deux durées de vie d'entrée, donc Rust applique la première règle
d'élision des durées de vie et donne à `&self` et à `announcement` leurs
propres durées de vie. Ensuite, comme l'un des paramètres est `&self`, le type
de retour obtient la durée de vie de `&self`, et toutes les durées de vie ont
été prises en compte.

<!--
### The Static Lifetime
-->

### La durée de vie statique

<!--
One special lifetime we need to discuss is `'static`, which denotes that the
affected reference _can_ live for the entire duration of the program. All
string literals have the `'static` lifetime, which we can annotate as follows:
-->

Une durée de vie spéciale que nous devons aborder est `'static`, qui indique
que la référence concernée _peut_ vivre pendant toute la durée du programme.
Toutes les chaînes de caractères littérales ont la durée de vie `'static`, que
nous pouvons annoter comme suit :

<!--
```rust
let s: &'static str = "I have a static lifetime.";
```
-->

```rust
let s: &'static str = "I have a static lifetime.";
```

<!--
The text of this string is stored directly in the program's binary, which is
always available. Therefore, the lifetime of all string literals is `'static`.
-->

Le texte de cette chaîne de caractères est stocké directement dans le binaire
du programme, qui est toujours disponible. Par conséquent, la durée de vie de
toutes les chaînes de caractères littérales est `'static`.

<!--
You might see suggestions in error messages to use the `'static` lifetime. But
before specifying `'static` as the lifetime for a reference, think about
whether or not the reference you have actually lives the entire lifetime of
your program, and whether you want it to. Most of the time, an error message
suggesting the `'static` lifetime results from attempting to create a dangling
reference or a mismatch of the available lifetimes. In such cases, the solution
is to fix those problems, not to specify the `'static` lifetime.
-->

Vous pourriez voir des suggestions dans les messages d'erreur pour utiliser la
durée de vie `'static`. Mais avant de spécifier `'static` comme durée de vie
pour une référence, demandez-vous si la référence que vous avez vit réellement
pendant toute la durée de vie de votre programme, et si vous le souhaitez. La
plupart du temps, un message d'erreur suggérant la durée de vie `'static`
résulte d'une tentative de création d'une référence pendante ou d'une
incompatibilité des durées de vie disponibles. Dans de tels cas, la solution
est de corriger ces problèmes, pas de spécifier la durée de vie `'static`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>

<!--
## Generic Type Parameters, Trait Bounds, and Lifetimes
-->

## Paramètres de type générique, trait bounds et durées de vie

<!--
Let's briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!
-->

Examinons brièvement la syntaxe pour spécifier des paramètres de type
générique, des trait bounds et des durées de vie, le tout dans une seule
fonction !


```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

<!--
This is the `longest` function from Listing 10-21 that returns the longer of
two string slices. But now it has an extra parameter named `ann` of the generic
type `T`, which can be filled in by any type that implements the `Display`
trait as specified by the `where` clause. This extra parameter will be printed
using `{}`, which is why the `Display` trait bound is necessary. Because
lifetimes are a type of generic, the declarations of the lifetime parameter
`'a` and the generic type parameter `T` go in the same list inside the angle
brackets after the function name.
-->

C'est la fonction `longest` de l'encart 10-21 qui retourne la plus longue de
deux slices de chaînes de caractères. Mais maintenant elle a un paramètre
supplémentaire nommé `ann` du type générique `T`, qui peut être rempli par
n'importe quel type qui implémente le trait `Display` comme spécifié par la
clause `where`. Ce paramètre supplémentaire sera affiché en utilisant `{}`,
c'est pourquoi le trait bound `Display` est nécessaire. Comme les durées de
vie sont un type de générique, les déclarations du paramètre de durée de vie
`'a` et du paramètre de type générique `T` vont dans la même liste entre
chevrons après le nom de la fonction.

<!--
## Summary
-->

## Résumé

<!--
We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you're
ready to write code without repetition that works in many different situations.
Generic type parameters let you apply the code to different types. Traits and
trait bounds ensure that even though the types are generic, they'll have the
behavior the code needs. You learned how to use lifetime annotations to ensure
that this flexible code won't have any dangling references. And all of this
analysis happens at compile time, which doesn't affect runtime performance!
-->

Nous avons couvert beaucoup de choses dans ce chapitre ! Maintenant que vous
connaissez les paramètres de type générique, les traits et les trait bounds,
et les paramètres de durée de vie génériques, vous êtes prêt à écrire du
code sans répétition qui fonctionne dans de nombreuses situations différentes.
Les paramètres de type générique vous permettent d'appliquer le code à
différents types. Les traits et les trait bounds s'assurent que même si les
types sont génériques, ils auront le comportement dont le code a besoin. Vous
avez appris à utiliser les annotations de durée de vie pour vous assurer que
ce code flexible n'aura pas de références pendantes. Et toute cette analyse se
fait au moment de la compilation, ce qui n'affecte pas les performances à
l'exécution !

<!--
Believe it or not, there is much more to learn on the topics we discussed in
this chapter: Chapter 18 discusses trait objects, which are another way to use
traits. There are also more complex scenarios involving lifetime annotations
that you will only need in very advanced scenarios; for those, you should read
the [Rust Reference][reference]. But next, you'll learn how to write tests in
Rust so that you can make sure your code is working the way it should.
-->

Croyez-le ou non, il y a encore beaucoup à apprendre sur les sujets que nous
avons abordés dans ce chapitre : le chapitre 18 traite des objets trait, qui
sont une autre façon d'utiliser les traits. Il existe aussi des scénarios plus
complexes impliquant des annotations de durée de vie dont vous n'aurez besoin
que dans des cas très avancés ; pour ceux-là, vous devriez lire la
[Référence Rust][reference]. Mais ensuite, vous apprendrez à écrire des tests
en Rust pour vous assurer que votre code fonctionne comme il le devrait.

[references-and-borrowing]: ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]: ch04-03-slices.html#string-slices-as-parameters
[reference]: ../reference/trait-bounds.html
