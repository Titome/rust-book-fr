<!--
## Unrecoverable Errors with `panic!`
-->

## Les erreurs irrécupérables avec `panic!`

<!--
Sometimes bad things happen in your code, and there's nothing you can do about
it. In these cases, Rust has the `panic!` macro. There are two ways to cause a
panic in practice: by taking an action that causes our code to panic (such as
accessing an array past the end) or by explicitly calling the `panic!` macro.
In both cases, we cause a panic in our program. By default, these panics will
print a failure message, unwind, clean up the stack, and quit. Via an
environment variable, you can also have Rust display the call stack when a
panic occurs to make it easier to track down the source of the panic.
-->

Parfois, de mauvaises choses se produisent dans votre code, et vous ne pouvez rien y faire. Dans ces cas-là, Rust dispose de la macro `panic!`. Il y a deux manières de provoquer un panic en pratique : en effectuant une action qui fait paniquer notre code (comme accéder à un tableau au-delà de sa fin) ou en appelant explicitement la macro `panic!`. Dans les deux cas, nous provoquons un panic dans notre programme. Par défaut, ces panics affichent un message d'erreur, déroulent la pile, nettoient la mémoire et quittent le programme. Via une variable d'environnement, vous pouvez également demander à Rust d'afficher la pile d'appels lorsqu'un panic se produit, afin de faciliter la recherche de son origine.

<!--
> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts _unwinding_, which means
> Rust walks back up the stack and cleans up the data from each function it
> encounters. However, walking back and cleaning up is a lot of work. Rust
> therefore allows you to choose the alternative of immediately _aborting_,
> which ends the program without cleaning up.
>
> Memory that the program was using will then need to be cleaned up by the
> operating system. If in your project you need to make the resultant binary as
> small as possible, you can switch from unwinding to aborting upon a panic by
> adding `panic = 'abort'` to the appropriate `[profile]` sections in your
> _Cargo.toml_ file. For example, if you want to abort on panic in release mode,
> add this:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```
-->

> ### Dérouler la pile ou abandonner en réponse à un panic
>
> Par défaut, lorsqu'un panic se produit, le programme commence à _dérouler la pile_ (unwinding), ce qui signifie que Rust remonte la pile et nettoie les données de chaque fonction qu'il rencontre. Cependant, remonter et nettoyer demande beaucoup de travail. Rust vous permet donc de choisir l'alternative d'_abandonner_ (aborting) immédiatement, ce qui met fin au programme sans nettoyage.
>
> La mémoire utilisée par le programme devra alors être nettoyée par le système d'exploitation. Si dans votre projet vous avez besoin de rendre le binaire résultant aussi petit que possible, vous pouvez passer du déroulement de pile à l'abandon en cas de panic en ajoutant `panic = 'abort'` aux sections `[profile]` appropriées dans votre fichier _Cargo.toml_. Par exemple, si vous voulez abandonner en cas de panic en mode release, ajoutez ceci :
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

<!--
Let's try calling `panic!` in a simple program:
-->

Essayons d'appeler `panic!` dans un programme simple :


<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

<!--
When you run the program, you'll see something like this:
-->

Lorsque vous exécutez le programme, vous verrez quelque chose comme ceci :


```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

<!--
The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: _src/main.rs:2:5_ indicates that it's the second line,
fifth character of our _src/main.rs_ file.
-->

L'appel à `panic!` provoque le message d'erreur contenu dans les deux dernières lignes. La première ligne affiche notre message de panic et l'endroit dans notre code source où le panic s'est produit : _src/main.rs:2:5_ indique qu'il s'agit de la deuxième ligne, cinquième caractère de notre fichier _src/main.rs_.

<!--
In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else's code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call.
-->

Dans ce cas, la ligne indiquée fait partie de notre code, et si nous allons à cette ligne, nous voyons l'appel à la macro `panic!`. Dans d'autres cas, l'appel à `panic!` pourrait se trouver dans du code que notre code appelle, et le nom de fichier et le numéro de ligne signalés par le message d'erreur correspondront au code de quelqu'un d'autre où la macro `panic!` est appelée, et non à la ligne de notre code qui a finalement conduit à l'appel de `panic!`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="using-a-panic-backtrace"></a>

<!--
We can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. To understand how to use
a `panic!` backtrace, let's look at another example and see what it's like when
a `panic!` call comes from a library because of a bug in our code instead of
from our code calling the macro directly. Listing 9-1 has some code that
attempts to access an index in a vector beyond the range of valid indexes.
-->

Nous pouvons utiliser la trace d'appels (backtrace) des fonctions d'où provient l'appel à `panic!` pour déterminer la partie de notre code qui cause le problème. Pour comprendre comment utiliser une trace d'appels de `panic!`, examinons un autre exemple et voyons ce qui se passe lorsqu'un appel à `panic!` provient d'une bibliothèque à cause d'un bogue dans notre code plutôt que de notre code appelant directement la macro. L'encart 9-1 contient du code qui tente d'accéder à un index dans un vecteur au-delà de la plage des index valides.


<Listing number="9-1" file-name="src/main.rs" caption="Tentative d'accès à un élément au-delà de la fin d'un vecteur, ce qui provoquera un appel à `panic!`">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

<!--
Here, we're attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but the vector has only three
elements. In this situation, Rust will panic. Using `[]` is supposed to return
an element, but if you pass an invalid index, there's no element that Rust
could return here that would be correct.
-->

Ici, nous tentons d'accéder au 100e élément de notre vecteur (qui se trouve à l'index 99 car l'indexation commence à zéro), mais le vecteur n'a que trois éléments. Dans cette situation, Rust va paniquer. L'utilisation de `[]` est censée renvoyer un élément, mais si vous passez un index invalide, il n'y a aucun élément que Rust pourrait renvoyer ici qui serait correct.

<!--
In C, attempting to read beyond the end of a data structure is undefined
behavior. You might get whatever is at the location in memory that would
correspond to that element in the data structure, even though the memory
doesn't belong to that structure. This is called a _buffer overread_ and can
lead to security vulnerabilities if an attacker is able to manipulate the index
in such a way as to read data they shouldn't be allowed to that is stored after
the data structure.
-->

En C, tenter de lire au-delà de la fin d'une structure de données est un comportement indéfini. Vous pourriez obtenir n'importe quelle valeur se trouvant à l'emplacement mémoire qui correspondrait à cet élément dans la structure de données, même si la mémoire n'appartient pas à cette structure. Cela s'appelle un _buffer overread_ (dépassement de lecture de tampon) et peut conduire à des vulnérabilités de sécurité si un attaquant est capable de manipuler l'index de manière à lire des données auxquelles il ne devrait pas avoir accès, stockées après la structure de données.

<!--
To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn't exist, Rust will stop execution and refuse to
continue. Let's try it and see:
-->

Pour protéger votre programme de ce type de vulnérabilité, si vous essayez de lire un élément à un index qui n'existe pas, Rust arrêtera l'exécution et refusera de continuer. Essayons pour voir :


```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

<!--
This error points at line 4 of our _main.rs_ where we attempt to access index
99 of the vector in `v`.
-->

Cette erreur pointe vers la ligne 4 de notre fichier _main.rs_ où nous tentons d'accéder à l'index 99 du vecteur `v`.

<!--
The `note:` line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. A
_backtrace_ is a list of all the functions that have been called to get to this
point. Backtraces in Rust work as they do in other languages: The key to
reading the backtrace is to start from the top and read until you see files you
wrote. That's the spot where the problem originated. The lines above that spot
are code that your code has called; the lines below are code that called your
code. These before-and-after lines might include core Rust code, standard
library code, or crates that you're using. Let's try to get a backtrace by
setting the `RUST_BACKTRACE` environment variable to any value except `0`.
Listing 9-2 shows output similar to what you'll see.
-->

La ligne `note:` nous indique que nous pouvons définir la variable d'environnement `RUST_BACKTRACE` pour obtenir une trace d'appels montrant exactement ce qui s'est passé pour provoquer l'erreur. Une _trace d'appels_ (backtrace) est une liste de toutes les fonctions qui ont été appelées pour arriver à ce point. Les traces d'appels en Rust fonctionnent comme dans les autres langages : la clé pour lire la trace d'appels est de commencer par le haut et de lire jusqu'à ce que vous voyiez des fichiers que vous avez écrits. C'est l'endroit où le problème est né. Les lignes au-dessus de cet endroit sont du code que votre code a appelé ; les lignes en dessous sont du code qui a appelé votre code. Ces lignes avant et après peuvent inclure du code du noyau de Rust, du code de la bibliothèque standard ou des crates que vous utilisez. Essayons d'obtenir une trace d'appels en définissant la variable d'environnement `RUST_BACKTRACE` à n'importe quelle valeur sauf `0`. L'encart 9-2 montre une sortie similaire à ce que vous verrez.

<!--
manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<!--
```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
-->

<Listing number="9-2" caption="La trace d'appels générée par un appel à `panic!` affichée lorsque la variable d'environnement `RUST_BACKTRACE` est définie">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

<!--
That's a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.
-->

Cela fait beaucoup de sortie ! La sortie exacte que vous verrez peut différer selon votre système d'exploitation et votre version de Rust. Pour obtenir des traces d'appels avec ces informations, les symboles de débogage doivent être activés. Les symboles de débogage sont activés par défaut lorsque vous utilisez `cargo build` ou `cargo run` sans le drapeau `--release`, comme c'est le cas ici.

<!--
In the output in Listing 9-2, line 6 of the backtrace points to the line in our
project that's causing the problem: line 4 of _src/main.rs_. If we don't want
our program to panic, we should start our investigation at the location pointed
to by the first line mentioning a file we wrote. In Listing 9-1, where we
deliberately wrote code that would panic, the way to fix the panic is to not
request an element beyond the range of the vector indexes. When your code
panics in the future, you'll need to figure out what action the code is taking
with what values to cause the panic and what the code should do instead.
-->

Dans la sortie de l'encart 9-2, la ligne 6 de la trace d'appels pointe vers la ligne de notre projet qui cause le problème : la ligne 4 de _src/main.rs_. Si nous ne voulons pas que notre programme panique, nous devrions commencer notre investigation à l'emplacement indiqué par la première ligne mentionnant un fichier que nous avons écrit. Dans l'encart 9-1, où nous avons délibérément écrit du code qui paniquerait, la façon de corriger le panic est de ne pas demander un élément au-delà de la plage des index du vecteur. Lorsque votre code paniquera à l'avenir, vous devrez déterminer quelle action le code effectue avec quelles valeurs pour provoquer le panic et ce que le code devrait faire à la place.

<!--
We'll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in the ["To `panic!` or Not to
`panic!`"][to-panic-or-not-to-panic] ignore
--> section later in this
chapter. Next, we'll look at how to recover from an error using `Result`.
-->

Nous reviendrons sur `panic!` et sur les cas où nous devrions ou ne devrions pas utiliser `panic!` pour gérer les conditions d'erreur dans la section [« Utiliser `panic!` ou ne pas utiliser `panic!` »][to-panic-or-not-to-panic]<!--
ignore
--> plus loin dans ce chapitre. Ensuite, nous verrons comment récupérer d'une erreur en utilisant `Result`.

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
