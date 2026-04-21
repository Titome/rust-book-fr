<!--
## References and Borrowing
-->

## Les references et l'emprunt

<!--
The issue with the tuple code in Listing 4-5 is that we have to return the
`String` to the calling function so that we can still use the `String` after
the call to `calculate_length`, because the `String` was moved into
`calculate_length`. Instead, we can provide a reference to the `String` value.
A reference is like a pointer in that it's an address we can follow to access
the data stored at that address; that data is owned by some other variable.
Unlike a pointer, a reference is guaranteed to point to a valid value of a
particular type for the life of that reference.
-->

Le probleme avec le code utilisant des tuples dans le listing 4-5 est que nous
devons renvoyer la `String` a la fonction appelante afin de pouvoir encore
utiliser la `String` apres l'appel a `calculate_length`, car la `String` a ete
deplacee dans `calculate_length`. A la place, nous pouvons fournir une reference
a la valeur `String`. Une reference est comme un pointeur en ce sens que c'est
une adresse que nous pouvons suivre pour acceder aux donnees stockees a cette
adresse ; ces donnees appartiennent a une autre variable. Contrairement a un
pointeur, une reference est garantie de pointer vers une valeur valide d'un type
particulier pendant toute la duree de vie de cette reference.

<!--
Here is how you would define and use a `calculate_length` function that has a
reference to an object as a parameter instead of taking ownership of the value:
-->

Voici comment vous definiriez et utiliseriez une fonction `calculate_length` qui
prend une reference a un objet comme parametre au lieu de prendre la possession
de la valeur :

<Listing file-name="src/main.rs">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

</Listing>

<!--
First, notice that all the tuple code in the variable declaration and the
function return value is gone. Second, note that we pass `&s1` into
`calculate_length` and, in its definition, we take `&String` rather than
`String`. These ampersands represent references, and they allow you to refer to
some value without taking ownership of it. Figure 4-6 depicts this concept.
-->

Premierement, remarquez que tout le code avec des tuples dans la declaration de
variable et la valeur de retour de la fonction a disparu. Deuxiemement, notez
que nous passons `&s1` a `calculate_length` et, dans sa definition, nous prenons
`&String` plutot que `String`. Ces esperluettes representent des references, et
elles vous permettent de faire reference a une valeur sans en prendre la
possession. La figure 4-6 illustre ce concept.

<!--
<img alt="Three tables: the table for s contains only a pointer to the table
for s1. The table for s1 contains the stack data for s1 and points to the
string data on the heap." src="img/trpl04-06.svg" class="center" />
-->

<img alt="Trois tableaux : le tableau pour s contient uniquement un pointeur
vers le tableau de s1. Le tableau de s1 contient les donnees de la pile pour s1
et pointe vers les donnees de chaine sur le tas." src="img/trpl04-06.svg" class="center" />

<!--
<span class="caption">Figure 4-6: A diagram of `&String` `s` pointing at
`String` `s1`</span>
-->

<span class="caption">Figure 4-6 : Un diagramme de `&String` `s` pointant vers
`String` `s1`</span>

<!--
> Note: The opposite of referencing by using `&` is _dereferencing_, which is
> accomplished with the dereference operator, `*`. We'll see some uses of the
> dereference operator in Chapter 8 and discuss details of dereferencing in
> Chapter 15.
-->

> Note : L'oppose du referencement avec `&` est le _dereferencement_, qui
> s'effectue avec l'operateur de dereferencement, `*`. Nous verrons quelques
> utilisations de l'operateur de dereferencement au chapitre 8 et discuterons
> des details du dereferencement au chapitre 15.

<!--
Let's take a closer look at the function call here:
-->

Examinons de plus pres l'appel de fonction ici :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

<!--
The `&s1` syntax lets us create a reference that _refers_ to the value of `s1`
but does not own it. Because the reference does not own it, the value it points
to will not be dropped when the reference stops being used.
-->

La syntaxe `&s1` nous permet de creer une reference qui _fait reference_ a la
valeur de `s1` mais ne la possede pas. Comme la reference ne la possede pas, la
valeur vers laquelle elle pointe ne sera pas supprimee quand la reference cesse
d'etre utilisee.

<!--
Likewise, the signature of the function uses `&` to indicate that the type of
the parameter `s` is a reference. Let's add some explanatory annotations:
-->

De meme, la signature de la fonction utilise `&` pour indiquer que le type du
parametre `s` est une reference. Ajoutons quelques annotations explicatives :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

<!--
The scope in which the variable `s` is valid is the same as any function
parameter's scope, but the value pointed to by the reference is not dropped
when `s` stops being used, because `s` doesn't have ownership. When functions
have references as parameters instead of the actual values, we won't need to
return the values in order to give back ownership, because we never had
ownership.
-->

La portee dans laquelle la variable `s` est valide est la meme que celle de tout
parametre de fonction, mais la valeur pointee par la reference n'est pas
supprimee quand `s` cesse d'etre utilisee, car `s` n'a pas la possession.
Quand les fonctions prennent des references comme parametres au lieu des valeurs
reelles, nous n'aurons pas besoin de renvoyer les valeurs pour rendre la
possession, car nous n'avons jamais eu la possession.

<!--
We call the action of creating a reference _borrowing_. As in real life, if a
person owns something, you can borrow it from them. When you're done, you have
to give it back. You don't own it.
-->

Nous appelons l'action de creer une reference l'_emprunt_ (borrowing). Comme
dans la vie reelle, si une personne possede quelque chose, vous pouvez le lui
emprunter. Quand vous avez termine, vous devez le rendre. Vous ne le possedez
pas.

<!--
So, what happens if we try to modify something we're borrowing? Try the code in
Listing 4-6. Spoiler alert: It doesn't work!
-->

Alors, que se passe-t-il si nous essayons de modifier quelque chose que nous
empruntons ? Essayez le code du listing 4-6. Attention spoiler : ca ne marche
pas !

<Listing number="4-6" file-name="src/main.rs" caption="Tentative de modification d'une valeur empruntee">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

</Listing>

<!--
Here's the error:
-->

Voici l'erreur :


```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

<!--
Just as variables are immutable by default, so are references. We're not
allowed to modify something we have a reference to.
-->

Tout comme les variables sont immuables par defaut, les references le sont
aussi. Nous ne sommes pas autorises a modifier quelque chose vers quoi nous avons
une reference.

<!--
### Mutable References
-->

### Les references mutables

<!--
We can fix the code from Listing 4-6 to allow us to modify a borrowed value
with just a few small tweaks that use, instead, a _mutable reference_:
-->

Nous pouvons corriger le code du listing 4-6 pour nous permettre de modifier une
valeur empruntee avec seulement quelques petites modifications qui utilisent, a
la place, une _reference mutable_ :

<Listing file-name="src/main.rs">


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

</Listing>

<!--
First, we change `s` to be `mut`. Then, we create a mutable reference with
`&mut s` where we call the `change` function and update the function signature
to accept a mutable reference with `some_string: &mut String`. This makes it
very clear that the `change` function will mutate the value it borrows.
-->

D'abord, nous changeons `s` pour qu'elle soit `mut`. Ensuite, nous creons une
reference mutable avec `&mut s` la ou nous appelons la fonction `change` et nous
mettons a jour la signature de la fonction pour accepter une reference mutable
avec `some_string: &mut String`. Cela rend tres clair que la fonction `change`
va modifier la valeur qu'elle emprunte.

<!--
Mutable references have one big restriction: If you have a mutable reference to
a value, you can have no other references to that value. This code that
attempts to create two mutable references to `s` will fail:
-->

Les references mutables ont une grande restriction : si vous avez une reference
mutable vers une valeur, vous ne pouvez avoir aucune autre reference vers cette
valeur. Ce code qui tente de creer deux references mutables vers `s` echouera :

<Listing file-name="src/main.rs">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

</Listing>

<!--
Here's the error:
-->

Voici l'erreur :


```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

<!--
This error says that this code is invalid because we cannot borrow `s` as
mutable more than once at a time. The first mutable borrow is in `r1` and must
last until it's used in the `println!`, but between the creation of that
mutable reference and its usage, we tried to create another mutable reference
in `r2` that borrows the same data as `r1`.
-->

Cette erreur indique que ce code est invalide car nous ne pouvons pas emprunter
`s` de maniere mutable plus d'une fois a la fois. Le premier emprunt mutable est
dans `r1` et doit durer jusqu'a son utilisation dans le `println!`, mais entre
la creation de cette reference mutable et son utilisation, nous avons essaye de
creer une autre reference mutable dans `r2` qui emprunte les memes donnees que
`r1`.

<!--
The restriction preventing multiple mutable references to the same data at the
same time allows for mutation but in a very controlled fashion. It's something
that new Rustaceans struggle with because most languages let you mutate
whenever you'd like. The benefit of having this restriction is that Rust can
prevent data races at compile time. A _data race_ is similar to a race
condition and happens when these three behaviors occur:
-->

La restriction empechant plusieurs references mutables vers les memes donnees en
meme temps permet la mutation mais de maniere tres controlee. C'est quelque chose
avec lequel les nouveaux Rustaceans ont du mal car la plupart des langages vous
permettent de modifier quand vous le souhaitez. L'avantage de cette restriction
est que Rust peut prevenir les courses de donnees (data races) a la compilation.
Une _course de donnees_ est similaire a une condition de course (race condition)
et se produit quand ces trois comportements surviennent :

<!--
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.
-->

- Deux pointeurs ou plus accedent aux memes donnees en meme temps.
- Au moins un des pointeurs est utilise pour ecrire dans les donnees.
- Aucun mecanisme n'est utilise pour synchroniser l'acces aux donnees.

<!--
Data races cause undefined behavior and can be difficult to diagnose and fix
when you're trying to track them down at runtime; Rust prevents this problem by
refusing to compile code with data races!
-->

Les courses de donnees causent un comportement indefini et peuvent etre
difficiles a diagnostiquer et corriger quand vous essayez de les traquer a
l'execution ; Rust empeche ce probleme en refusant de compiler du code avec des
courses de donnees !

<!--
As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not _simultaneous_ ones:
-->

Comme toujours, nous pouvons utiliser des accolades pour creer une nouvelle
portee, ce qui permet d'avoir plusieurs references mutables, simplement pas
_simultanement_ :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

<!--
Rust enforces a similar rule for combining mutable and immutable references.
This code results in an error:
-->

Rust applique une regle similaire pour combiner des references mutables et
immuables. Ce code produit une erreur :


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

<!--
Here's the error:
-->

Voici l'erreur :


```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

<!--
Whew! We _also_ cannot have a mutable reference while we have an immutable one
to the same value.
-->

Ouf ! Nous ne pouvons _pas non plus_ avoir une reference mutable tant que nous
avons une reference immuable vers la meme valeur.

<!--
Users of an immutable reference don't expect the value to suddenly change out
from under them! However, multiple immutable references are allowed because no
one who is just reading the data has the ability to affect anyone else's
reading of the data.
-->

Les utilisateurs d'une reference immuable ne s'attendent pas a ce que la valeur
change soudainement sous leurs pieds ! Cependant, plusieurs references immuables
sont autorisees car personne qui se contente de lire les donnees n'a la capacite
d'affecter la lecture des donnees par quelqu'un d'autre.

<!--
Note that a reference's scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references is in the `println!`,
before the mutable reference is introduced:
-->

Notez que la portee d'une reference commence la ou elle est introduite et
continue jusqu'a la derniere utilisation de cette reference. Par exemple, ce code
compilera car la derniere utilisation des references immuables se trouve dans le
`println!`, avant que la reference mutable ne soit introduite :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

<!--
The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don't overlap, so this code is allowed: The compiler can
tell that the reference is no longer being used at a point before the end of
the scope.
-->

Les portees des references immuables `r1` et `r2` se terminent apres le
`println!` ou elles sont utilisees pour la derniere fois, ce qui est avant que la
reference mutable `r3` ne soit creee. Ces portees ne se chevauchent pas, donc ce
code est autorise : le compilateur peut determiner que la reference n'est plus
utilisee a un moment avant la fin de la portee.

<!--
Even though borrowing errors may be frustrating at times, remember that it's
the Rust compiler pointing out a potential bug early (at compile time rather
than at runtime) and showing you exactly where the problem is. Then, you don't
have to track down why your data isn't what you thought it was.
-->

Meme si les erreurs d'emprunt peuvent etre frustrantes parfois, rappelez-vous
que c'est le compilateur Rust qui signale un bug potentiel tot (a la compilation
plutot qu'a l'execution) et vous montre exactement ou se trouve le probleme.
Ainsi, vous n'avez pas a chercher pourquoi vos donnees ne sont pas ce que vous
pensiez qu'elles etaient.

<!--
### Dangling References
-->

### Les references pendantes (dangling references)

<!--
In languages with pointers, it's easy to erroneously create a _dangling
pointer_—a pointer that references a location in memory that may have been
given to someone else—by freeing some memory while preserving a pointer to that
memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: If you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.
-->

Dans les langages avec des pointeurs, il est facile de creer par erreur un
_pointeur pendant_ (dangling pointer) -- un pointeur qui fait reference a un
emplacement en memoire qui a pu etre donne a quelqu'un d'autre -- en liberant de
la memoire tout en conservant un pointeur vers cette memoire. En Rust, en
revanche, le compilateur garantit que les references ne seront jamais des
references pendantes : si vous avez une reference vers des donnees, le
compilateur s'assurera que les donnees ne sortiront pas de la portee avant la
reference vers ces donnees.

<!--
Let's try to create a dangling reference to see how Rust prevents them with a
compile-time error:
-->

Essayons de creer une reference pendante pour voir comment Rust les empeche avec
une erreur de compilation :

<Listing file-name="src/main.rs">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

</Listing>

<!--
Here's the error:
-->

Voici l'erreur :


```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

<!--
This error message refers to a feature we haven't covered yet: lifetimes. We'll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:
-->

Ce message d'erreur fait reference a une fonctionnalite que nous n'avons pas
encore couverte : les durees de vie (lifetimes). Nous discuterons des durees de
vie en detail au chapitre 10. Mais, si vous ignorez les parties sur les durees
de vie, le message contient la cle de la raison pour laquelle ce code pose
probleme :

<!--
```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```
-->

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

<!--
Let's take a closer look at exactly what's happening at each stage of our
`dangle` code:
-->

Examinons de plus pres ce qui se passe exactement a chaque etape de notre code
`dangle` :

<Listing file-name="src/main.rs">


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

</Listing>

<!--
Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That's no good! Rust
won't let us do this.
-->

Comme `s` est creee a l'interieur de `dangle`, quand le code de `dangle` est
termine, `s` sera desallouee. Mais nous avons essaye de renvoyer une reference
vers elle. Cela signifie que cette reference pointerait vers une `String`
invalide. Ce n'est pas bon ! Rust ne nous laissera pas faire cela.

<!--
The solution here is to return the `String` directly:
-->

La solution ici est de renvoyer la `String` directement :


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

<!--
This works without any problems. Ownership is moved out, and nothing is
deallocated.
-->

Cela fonctionne sans aucun probleme. La possession est transferee, et rien n'est
desalloue.

<!--
### The Rules of References
-->

### Les regles des references

<!--
Let's recap what we've discussed about references:
-->

Recapitulons ce que nous avons discute au sujet des references :

<!--
- At any given time, you can have _either_ one mutable reference _or_ any
  number of immutable references.
- References must always be valid.
-->

- A tout moment, vous pouvez avoir _soit_ une reference mutable _soit_ un
  nombre quelconque de references immuables.
- Les references doivent toujours etre valides.

<!--
Next, we'll look at a different kind of reference: slices.
-->

Ensuite, nous examinerons un type different de reference : les slices.
