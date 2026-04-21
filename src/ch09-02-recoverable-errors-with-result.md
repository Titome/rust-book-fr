<!--
## Recoverable Errors with `Result`
-->

## Les erreurs récupérables avec `Result`

<!--
Most errors aren't serious enough to require the program to stop entirely.
Sometimes when a function fails, it's for a reason that you can easily interpret
and respond to. For example, if you try to open a file and that operation fails
because the file doesn't exist, you might want to create the file instead of
terminating the process.
-->

La plupart des erreurs ne sont pas suffisamment graves pour nécessiter l'arrêt complet du programme. Parfois, lorsqu'une fonction échoue, c'est pour une raison que vous pouvez facilement interpréter et traiter. Par exemple, si vous essayez d'ouvrir un fichier et que cette opération échoue parce que le fichier n'existe pas, vous voudrez peut-être créer le fichier plutôt que de terminer le processus.

<!--
Recall from ["Handling Potential Failure with `Result`"][handle_failure]
ignore
--> in Chapter 2 that the `Result` enum is defined as having two
variants, `Ok` and `Err`, as follows:
-->

Rappelez-vous, dans [« Gérer les échecs potentiels avec `Result` »][handle_failure]<!--
ignore
--> au chapitre 2, que l'enum `Result` est définie comme ayant deux variantes, `Ok` et `Err`, comme suit :

<!--
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
-->

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `T` and `E` are generic type parameters: We'll discuss generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions defined on it in
many different situations where the success value and error value we want to
return may differ.
-->

`T` et `E` sont des paramètres de type générique : nous aborderons les génériques plus en détail au chapitre 10. Ce que vous devez savoir pour l'instant, c'est que `T` représente le type de la valeur qui sera renvoyée en cas de succès dans la variante `Ok`, et `E` représente le type de l'erreur qui sera renvoyée en cas d'échec dans la variante `Err`. Comme `Result` possède ces paramètres de type générique, nous pouvons utiliser le type `Result` et les fonctions définies dessus dans de nombreuses situations différentes où la valeur de succès et la valeur d'erreur que nous voulons renvoyer peuvent différer.

<!--
Let's call a function that returns a `Result` value because the function could
fail. In Listing 9-3, we try to open a file.
-->

Appelons une fonction qui renvoie une valeur `Result` parce que la fonction pourrait échouer. Dans l'encart 9-3, nous essayons d'ouvrir un fichier.


<Listing number="9-3" file-name="src/main.rs" caption="Ouverture d'un fichier">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

<!--
The return type of `File::open` is a `Result<T, E>`. The generic parameter `T`
has been filled in by the implementation of `File::open` with the type of the
success value, `std::fs::File`, which is a file handle. The type of `E` used in
the error value is `std::io::Error`. This return type means the call to
`File::open` might succeed and return a file handle that we can read from or
write to. The function call also might fail: For example, the file might not
exist, or we might not have permission to access the file. The `File::open`
function needs to have a way to tell us whether it succeeded or failed and at
the same time give us either the file handle or error information. This
information is exactly what the `Result` enum conveys.
-->

Le type de retour de `File::open` est un `Result<T, E>`. Le paramètre générique `T` a été rempli par l'implémentation de `File::open` avec le type de la valeur de succès, `std::fs::File`, qui est un descripteur de fichier. Le type de `E` utilisé dans la valeur d'erreur est `std::io::Error`. Ce type de retour signifie que l'appel à `File::open` pourrait réussir et renvoyer un descripteur de fichier à partir duquel nous pouvons lire ou écrire. L'appel de fonction pourrait également échouer : par exemple, le fichier pourrait ne pas exister, ou nous pourrions ne pas avoir la permission d'accéder au fichier. La fonction `File::open` a besoin d'un moyen de nous dire si elle a réussi ou échoué et en même temps de nous fournir soit le descripteur de fichier, soit les informations d'erreur. C'est exactement ce que l'enum `Result` transmet.

<!--
In the case where `File::open` succeeds, the value in the variable
`greeting_file_result` will be an instance of `Ok` that contains a file handle.
In the case where it fails, the value in `greeting_file_result` will be an
instance of `Err` that contains more information about the kind of error that
occurred.
-->

Dans le cas où `File::open` réussit, la valeur dans la variable `greeting_file_result` sera une instance de `Ok` contenant un descripteur de fichier. Dans le cas où elle échoue, la valeur dans `greeting_file_result` sera une instance de `Err` contenant plus d'informations sur le type d'erreur qui s'est produite.

<!--
We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.
-->

Nous devons compléter le code de l'encart 9-3 pour effectuer différentes actions selon la valeur renvoyée par `File::open`. L'encart 9-4 montre une façon de gérer le `Result` en utilisant un outil de base, l'expression `match` que nous avons abordée au chapitre 6.


<Listing number="9-4" file-name="src/main.rs" caption="Utilisation d'une expression `match` pour gérer les variantes de `Result` qui pourraient être renvoyées">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

<!--
Note that, like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don't need to specify `Result::`
before the `Ok` and `Err` variants in the `match` arms.
-->

Notez que, comme l'enum `Option`, l'enum `Result` et ses variantes ont été importées dans la portée par le prelude, donc nous n'avons pas besoin de spécifier `Result::` avant les variantes `Ok` et `Err` dans les branches du `match`.

<!--
When the result is `Ok`, this code will return the inner `file` value out of
the `Ok` variant, and we then assign that file handle value to the variable
`greeting_file`. After the `match`, we can use the file handle for reading or
writing.
-->

Lorsque le résultat est `Ok`, ce code renverra la valeur interne `file` de la variante `Ok`, et nous assignons ensuite cette valeur de descripteur de fichier à la variable `greeting_file`. Après le `match`, nous pouvons utiliser le descripteur de fichier pour lire ou écrire.

<!--
The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we've chosen to call the `panic!` macro. If
there's no file named _hello.txt_ in our current directory and we run this
code, we'll see the following output from the `panic!` macro:
-->

L'autre branche du `match` gère le cas où nous obtenons une valeur `Err` de `File::open`. Dans cet exemple, nous avons choisi d'appeler la macro `panic!`. S'il n'y a pas de fichier nommé _hello.txt_ dans notre répertoire actuel et que nous exécutons ce code, nous verrons la sortie suivante de la macro `panic!` :


```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

<!--
As usual, this output tells us exactly what has gone wrong.
-->

Comme d'habitude, cette sortie nous indique exactement ce qui s'est mal passé.

<!--
### Matching on Different Errors
-->

### Réagir selon les différentes erreurs

<!--
The code in Listing 9-4 will `panic!` no matter why `File::open` failed.
However, we want to take different actions for different failure reasons. If
`File::open` failed because the file doesn't exist, we want to create the file
and return the handle to the new file. If `File::open` failed for any other
reason—for example, because we didn't have permission to open the file—we still
want the code to `panic!` in the same way it did in Listing 9-4. For this, we
add an inner `match` expression, shown in Listing 9-5.
-->

Le code de l'encart 9-4 va appeler `panic!` quelle que soit la raison de l'échec de `File::open`. Cependant, nous voulons effectuer différentes actions selon les différentes raisons de l'échec. Si `File::open` a échoué parce que le fichier n'existe pas, nous voulons créer le fichier et renvoyer le descripteur du nouveau fichier. Si `File::open` a échoué pour toute autre raison -- par exemple, parce que nous n'avions pas la permission d'ouvrir le fichier -- nous voulons toujours que le code appelle `panic!` de la même manière que dans l'encart 9-4. Pour cela, nous ajoutons une expression `match` interne, montrée dans l'encart 9-5.

<!--
 ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol
-->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

-->

<Listing number="9-5" file-name="src/main.rs" caption="Gérer différents types d'erreurs de différentes manières">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

<!--
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method, `kind`, that we can call to get an `io::ErrorKind` value. The
enum `io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we're trying to open doesn't exist yet. So, we match on
`greeting_file_result`, but we also have an inner match on `error.kind()`.
-->

Le type de la valeur que `File::open` renvoie à l'intérieur de la variante `Err` est `io::Error`, qui est une structure fournie par la bibliothèque standard. Cette structure possède une méthode, `kind`, que nous pouvons appeler pour obtenir une valeur `io::ErrorKind`. L'enum `io::ErrorKind` est fournie par la bibliothèque standard et possède des variantes représentant les différents types d'erreurs qui peuvent résulter d'une opération d'`io`. La variante que nous voulons utiliser est `ErrorKind::NotFound`, qui indique que le fichier que nous essayons d'ouvrir n'existe pas encore. Donc, nous faisons un match sur `greeting_file_result`, mais nous avons aussi un match interne sur `error.kind()`.

<!--
The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can't be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.
-->

La condition que nous voulons vérifier dans le match interne est si la valeur renvoyée par `error.kind()` est la variante `NotFound` de l'enum `ErrorKind`. Si c'est le cas, nous essayons de créer le fichier avec `File::create`. Cependant, comme `File::create` pourrait également échouer, nous avons besoin d'une deuxième branche dans l'expression `match` interne. Lorsque le fichier ne peut pas être créé, un message d'erreur différent est affiché. La deuxième branche du `match` externe reste la même, de sorte que le programme panique pour toute erreur autre que l'erreur de fichier manquant.

<!--
> #### Alternatives to Using `match` with `Result<T, E>`
>
> That's a lot of `match`! The `match` expression is very useful but also very
> much a primitive. In Chapter 13, you'll learn about closures, which are used
> with many of the methods defined on `Result<T, E>`. These methods can be more
> concise than using `match` when handling `Result<T, E>` values in your code.
>
> For example, here's another way to write the same logic as shown in Listing
> 9-5, this time using closures and the `unwrap_or_else` method:
>
>  CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127
-->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> Although this code has the same behavior as Listing 9-5, it doesn't contain
> any `match` expressions and is cleaner to read. Come back to this example
> after you've read Chapter 13 and look up the `unwrap_or_else` method in the
> standard library documentation. Many more of these methods can clean up huge,
> nested `match` expressions when you're dealing with errors.
-->

> #### Alternatives à l'utilisation de `match` avec `Result<T, E>`
>
> Cela fait beaucoup de `match` ! L'expression `match` est très utile mais aussi très primitive. Au chapitre 13, vous apprendrez les fermetures (closures), qui sont utilisées avec beaucoup de méthodes définies sur `Result<T, E>`. Ces méthodes peuvent être plus concises que l'utilisation de `match` lors de la gestion des valeurs `Result<T, E>` dans votre code.
>
> Par exemple, voici une autre façon d'écrire la même logique que celle montrée dans l'encart 9-5, cette fois en utilisant des fermetures et la méthode `unwrap_or_else` :
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> Bien que ce code ait le même comportement que l'encart 9-5, il ne contient aucune expression `match` et est plus agréable à lire. Revenez à cet exemple après avoir lu le chapitre 13 et consultez la méthode `unwrap_or_else` dans la documentation de la bibliothèque standard. Beaucoup d'autres méthodes de ce type peuvent simplifier d'énormes expressions `match` imbriquées lorsque vous gérez des erreurs.

<!--
Old headings. Do not remove or links may break.
-->

<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>

<!--
#### Shortcuts for Panic on Error
-->

#### Raccourcis pour paniquer en cas d'erreur

<!--
Using `match` works well enough, but it can be a bit verbose and doesn't always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various, more specific tasks. The `unwrap` method is a
shortcut method implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
-->

L'utilisation de `match` fonctionne assez bien, mais elle peut être un peu verbeuse et ne communique pas toujours bien l'intention. Le type `Result<T, E>` possède de nombreuses méthodes utilitaires définies dessus pour effectuer diverses tâches plus spécifiques. La méthode `unwrap` est une méthode raccourci implémentée exactement comme l'expression `match` que nous avons écrite dans l'encart 9-4. Si la valeur `Result` est la variante `Ok`, `unwrap` renverra la valeur à l'intérieur du `Ok`. Si le `Result` est la variante `Err`, `unwrap` appellera la macro `panic!` pour nous. Voici un exemple de `unwrap` en action :


<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

<!--
If we run this code without a _hello.txt_ file, we'll see an error message from
the `panic!` call that the `unwrap` method makes:
-->

Si nous exécutons ce code sans fichier _hello.txt_, nous verrons un message d'erreur provenant de l'appel à `panic!` que la méthode `unwrap` effectue :

<!--
manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

<!--
```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

<!--
Similarly, the `expect` method lets us also choose the `panic!` error message.
Using `expect` instead of `unwrap` and providing good error messages can convey
your intent and make tracking down the source of a panic easier. The syntax of
`expect` looks like this:
-->

De la même manière, la méthode `expect` nous permet également de choisir le message d'erreur du `panic!`. Utiliser `expect` au lieu de `unwrap` et fournir de bons messages d'erreur peut transmettre votre intention et faciliter la recherche de l'origine d'un panic. La syntaxe de `expect` ressemble à ceci :


<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

<!--
We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here's what it looks like:
-->

Nous utilisons `expect` de la même manière que `unwrap` : pour renvoyer le descripteur de fichier ou appeler la macro `panic!`. Le message d'erreur utilisé par `expect` dans son appel à `panic!` sera le paramètre que nous passons à `expect`, plutôt que le message par défaut de `panic!` qu'utilise `unwrap`. Voici à quoi cela ressemble :

<!--
manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

<!--
```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

<!--
In production-quality code, most Rustaceans choose `expect` rather than
`unwrap` and give more context about why the operation is expected to always
succeed. That way, if your assumptions are ever proven wrong, you have more
information to use in debugging.
-->

Dans du code de qualité production, la plupart des Rustacés choisissent `expect` plutôt que `unwrap` et donnent plus de contexte sur la raison pour laquelle l'opération est censée toujours réussir. De cette façon, si vos hypothèses s'avèrent un jour incorrectes, vous avez plus d'informations à utiliser pour le débogage.

<!--
### Propagating Errors
-->

### Propager les erreurs

<!--
When a function's implementation calls something that might fail, instead of
handling the error within the function itself, you can return the error to the
calling code so that it can decide what to do. This is known as _propagating_
the error and gives more control to the calling code, where there might be more
information or logic that dictates how the error should be handled than what
you have available in the context of your code.
-->

Lorsque l'implémentation d'une fonction appelle quelque chose qui pourrait échouer, au lieu de gérer l'erreur au sein de la fonction elle-même, vous pouvez renvoyer l'erreur au code appelant afin qu'il puisse décider quoi faire. C'est ce qu'on appelle _propager_ l'erreur, et cela donne plus de contrôle au code appelant, là où il pourrait y avoir plus d'informations ou de logique dictant comment l'erreur devrait être gérée que ce dont vous disposez dans le contexte de votre code.

<!--
For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn't exist or can't be read, this function will return those errors
to the code that called the function.
-->

Par exemple, l'encart 9-6 montre une fonction qui lit un nom d'utilisateur à partir d'un fichier. Si le fichier n'existe pas ou ne peut pas être lu, cette fonction renverra ces erreurs au code qui a appelé la fonction.

<!--
 Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes.
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

-->

<Listing number="9-6" file-name="src/main.rs" caption="Une fonction qui renvoie les erreurs au code appelant en utilisant `match`">

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

<!--
This function can be written in a much shorter way, but we're going to start by
doing a lot of it manually in order to explore error handling; at the end,
we'll show the shorter way. Let's look at the return type of the function
first: `Result<String, io::Error>`. This means the function is returning a
value of the type `Result<T, E>`, where the generic parameter `T` has been
filled in with the concrete type `String` and the generic type `E` has been
filled in with the concrete type `io::Error`.
-->

Cette fonction peut être écrite de manière beaucoup plus concise, mais nous allons commencer par faire beaucoup de choses manuellement afin d'explorer la gestion des erreurs ; à la fin, nous montrerons la manière plus courte. Examinons d'abord le type de retour de la fonction : `Result<String, io::Error>`. Cela signifie que la fonction renvoie une valeur du type `Result<T, E>`, où le paramètre générique `T` a été rempli avec le type concret `String` et le type générique `E` a été rempli avec le type concret `io::Error`.

<!--
If this function succeeds without any problems, the code that calls this
function will receive an `Ok` value that holds a `String`—the `username` that
this function read from the file. If this function encounters any problems, the
calling code will receive an `Err` value that holds an instance of `io::Error`
that contains more information about what the problems were. We chose
`io::Error` as the return type of this function because that happens to be the
type of the error value returned from both of the operations we're calling in
this function's body that might fail: the `File::open` function and the
`read_to_string` method.
-->

Si cette fonction réussit sans aucun problème, le code qui appelle cette fonction recevra une valeur `Ok` contenant une `String` -- le `username` que cette fonction a lu dans le fichier. Si cette fonction rencontre un problème, le code appelant recevra une valeur `Err` contenant une instance de `io::Error` qui contient plus d'informations sur la nature des problèmes. Nous avons choisi `io::Error` comme type de retour de cette fonction parce que c'est justement le type de la valeur d'erreur renvoyée par les deux opérations que nous appelons dans le corps de cette fonction et qui pourraient échouer : la fonction `File::open` et la méthode `read_to_string`.

<!--
The body of the function starts by calling the `File::open` function. Then, we
handle the `Result` value with a `match` similar to the `match` in Listing 9-4.
If `File::open` succeeds, the file handle in the pattern variable `file`
becomes the value in the mutable variable `username_file` and the function
continues. In the `Err` case, instead of calling `panic!`, we use the `return`
keyword to return early out of the function entirely and pass the error value
from `File::open`, now in the pattern variable `e`, back to the calling code as
this function's error value.
-->

Le corps de la fonction commence par appeler la fonction `File::open`. Ensuite, nous gérons la valeur `Result` avec un `match` similaire au `match` de l'encart 9-4. Si `File::open` réussit, le descripteur de fichier dans la variable de motif `file` devient la valeur dans la variable mutable `username_file` et la fonction continue. Dans le cas `Err`, au lieu d'appeler `panic!`, nous utilisons le mot-clé `return` pour sortir prématurément de la fonction et transmettre la valeur d'erreur de `File::open`, maintenant dans la variable de motif `e`, au code appelant comme valeur d'erreur de cette fonction.

<!--
So, if we have a file handle in `username_file`, the function then creates a
new `String` in variable `username` and calls the `read_to_string` method on
the file handle in `username_file` to read the contents of the file into
`username`. The `read_to_string` method also returns a `Result` because it
might fail, even though `File::open` succeeded. So, we need another `match` to
handle that `Result`: If `read_to_string` succeeds, then our function has
succeeded, and we return the username from the file that's now in `username`
wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the
same way that we returned the error value in the `match` that handled the
return value of `File::open`. However, we don't need to explicitly say
`return`, because this is the last expression in the function.
-->

Donc, si nous avons un descripteur de fichier dans `username_file`, la fonction crée ensuite une nouvelle `String` dans la variable `username` et appelle la méthode `read_to_string` sur le descripteur de fichier dans `username_file` pour lire le contenu du fichier dans `username`. La méthode `read_to_string` renvoie également un `Result` car elle pourrait échouer, même si `File::open` a réussi. Nous avons donc besoin d'un autre `match` pour gérer ce `Result` : si `read_to_string` réussit, alors notre fonction a réussi, et nous renvoyons le nom d'utilisateur du fichier qui se trouve maintenant dans `username`, enveloppé dans un `Ok`. Si `read_to_string` échoue, nous renvoyons la valeur d'erreur de la même manière que nous avons renvoyé la valeur d'erreur dans le `match` qui gérait la valeur de retour de `File::open`. Cependant, nous n'avons pas besoin de dire explicitement `return`, car c'est la dernière expression de la fonction.

<!--
The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. It's
up to the calling code to decide what to do with those values. If the calling
code gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don't have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.
-->

Le code qui appelle ce code devra ensuite gérer soit une valeur `Ok` contenant un nom d'utilisateur, soit une valeur `Err` contenant un `io::Error`. C'est au code appelant de décider quoi faire avec ces valeurs. Si le code appelant obtient une valeur `Err`, il pourrait appeler `panic!` et faire planter le programme, utiliser un nom d'utilisateur par défaut, ou chercher le nom d'utilisateur ailleurs que dans un fichier, par exemple. Nous n'avons pas assez d'informations sur ce que le code appelant essaie réellement de faire, donc nous propageons toutes les informations de succès ou d'erreur vers le haut pour qu'il les gère de manière appropriée.

<!--
This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.
-->

Ce patron de propagation des erreurs est tellement courant en Rust que Rust fournit l'opérateur point d'interrogation `?` pour faciliter les choses.

<!--
Old headings. Do not remove or links may break.
-->

<a id="a-shortcut-for-propagating-errors-the--operator"></a>

<!--
#### The `?` Operator Shortcut
-->

#### Le raccourci de l'opérateur `?`

<!--
Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as in Listing 9-6, but this implementation uses the `?`
operator.
-->

L'encart 9-7 montre une implémentation de `read_username_from_file` qui a la même fonctionnalité que dans l'encart 9-6, mais cette implémentation utilise l'opérateur `?`.

<!--
 Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes.
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

-->

<Listing number="9-7" file-name="src/main.rs" caption="Une fonction qui renvoie les erreurs au code appelant en utilisant l'opérateur `?`">

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

<!--
The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions that we defined to handle the `Result` values in
Listing 9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok`
will get returned from this expression, and the program will continue. If the
value is an `Err`, the `Err` will be returned from the whole function as if we
had used the `return` keyword so that the error value gets propagated to the
calling code.
-->

Le `?` placé après une valeur `Result` est défini pour fonctionner presque de la même manière que les expressions `match` que nous avons définies pour gérer les valeurs `Result` dans l'encart 9-6. Si la valeur du `Result` est un `Ok`, la valeur à l'intérieur du `Ok` sera renvoyée par cette expression et le programme continuera. Si la valeur est un `Err`, le `Err` sera renvoyé par la fonction entière comme si nous avions utilisé le mot-clé `return`, de sorte que la valeur d'erreur est propagée au code appelant.

<!--
There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: Error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert values from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent
all the ways a function might fail, even if parts might fail for many different
reasons.
-->

Il y a une différence entre ce que fait l'expression `match` de l'encart 9-6 et ce que fait l'opérateur `?` : les valeurs d'erreur sur lesquelles l'opérateur `?` est appelé passent par la fonction `from`, définie dans le trait `From` de la bibliothèque standard, qui est utilisée pour convertir des valeurs d'un type en un autre. Lorsque l'opérateur `?` appelle la fonction `from`, le type d'erreur reçu est converti dans le type d'erreur défini dans le type de retour de la fonction courante. C'est utile lorsqu'une fonction renvoie un seul type d'erreur pour représenter toutes les façons dont une fonction peut échouer, même si certaines parties peuvent échouer pour de nombreuses raisons différentes.

<!--
For example, we could change the `read_username_from_file` function in Listing
9-7 to return a custom error type named `OurError` that we define. If we also
define `impl From<io::Error> for OurError` to construct an instance of
`OurError` from an `io::Error`, then the `?` operator calls in the body of
`read_username_from_file` will call `from` and convert the error types without
needing to add any more code to the function.
-->

Par exemple, nous pourrions modifier la fonction `read_username_from_file` de l'encart 9-7 pour renvoyer un type d'erreur personnalisé nommé `OurError` que nous définissons. Si nous définissons également `impl From<io::Error> for OurError` pour construire une instance de `OurError` à partir d'un `io::Error`, alors les appels à l'opérateur `?` dans le corps de `read_username_from_file` appelleront `from` et convertiront les types d'erreur sans avoir besoin d'ajouter du code supplémentaire à la fonction.

<!--
In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `username_file`. If an error
occurs, the `?` operator will return early out of the whole function and give
any `Err` value to the calling code. The same thing applies to the `?` at the
end of the `read_to_string` call.
-->

Dans le contexte de l'encart 9-7, le `?` à la fin de l'appel à `File::open` renverra la valeur à l'intérieur d'un `Ok` dans la variable `username_file`. Si une erreur se produit, l'opérateur `?` sortira prématurément de la fonction entière et transmettra toute valeur `Err` au code appelant. La même chose s'applique au `?` à la fin de l'appel à `read_to_string`.

<!--
The `?` operator eliminates a lot of boilerplate and makes this function's
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.
-->

L'opérateur `?` élimine beaucoup de code répétitif et rend l'implémentation de cette fonction plus simple. Nous pourrions même raccourcir davantage ce code en enchaînant les appels de méthode immédiatement après le `?`, comme montré dans l'encart 9-8.

<!--
 Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes.
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

-->

<Listing number="9-8" file-name="src/main.rs" caption="Enchaînement d'appels de méthode après l'opérateur `?`">

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

<!--
We've moved the creation of the new `String` in `username` to the beginning of
the function; that part hasn't changed. Instead of creating a variable
`username_file`, we've chained the call to `read_to_string` directly onto the
result of `File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing `username`
when both `File::open` and `read_to_string` succeed rather than returning
errors. The functionality is again the same as in Listing 9-6 and Listing 9-7;
this is just a different, more ergonomic way to write it.
-->

Nous avons déplacé la création de la nouvelle `String` dans `username` au début de la fonction ; cette partie n'a pas changé. Au lieu de créer une variable `username_file`, nous avons enchaîné l'appel à `read_to_string` directement sur le résultat de `File::open("hello.txt")?`. Nous avons toujours un `?` à la fin de l'appel à `read_to_string`, et nous renvoyons toujours une valeur `Ok` contenant `username` lorsque `File::open` et `read_to_string` réussissent tous les deux, plutôt que de renvoyer des erreurs. La fonctionnalité est à nouveau la même que dans les encarts 9-6 et 9-7 ; c'est simplement une manière différente et plus ergonomique de l'écrire.

<!--
Listing 9-9 shows a way to make this even shorter using `fs::read_to_string`.
-->

L'encart 9-9 montre un moyen de rendre cela encore plus court en utilisant `fs::read_to_string`.

<!--
 Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes.
-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

-->

<Listing number="9-9" file-name="src/main.rs" caption="Utilisation de `fs::read_to_string` au lieu d'ouvrir puis de lire le fichier">

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

<!--
Reading a file into a string is a fairly common operation, so the standard
library provides the convenient `fs::read_to_string` function that opens the
file, creates a new `String`, reads the contents of the file, puts the contents
into that `String`, and returns it. Of course, using `fs::read_to_string`
doesn't give us the opportunity to explain all the error handling, so we did it
the longer way first.
-->

Lire un fichier dans une chaîne de caractères est une opération assez courante, c'est pourquoi la bibliothèque standard fournit la fonction pratique `fs::read_to_string` qui ouvre le fichier, crée une nouvelle `String`, lit le contenu du fichier, place le contenu dans cette `String` et la renvoie. Bien sûr, utiliser `fs::read_to_string` ne nous donne pas l'occasion d'expliquer toute la gestion des erreurs, c'est pourquoi nous l'avons fait de la manière longue d'abord.

<!--
Old headings. Do not remove or links may break.
-->

<a id="where-the--operator-can-be-used"></a>

<!--
#### Where to Use the `?` Operator
-->

#### Où utiliser l'opérateur `?`

<!--
The `?` operator can only be used in functions whose return type is compatible
with the value the `?` is used on. This is because the `?` operator is defined
to perform an early return of a value out of the function, in the same manner
as the `match` expression we defined in Listing 9-6. In Listing 9-6, the
`match` was using a `Result` value, and the early return arm returned an
`Err(e)` value. The return type of the function has to be a `Result` so that
it's compatible with this `return`.
-->

L'opérateur `?` ne peut être utilisé que dans les fonctions dont le type de retour est compatible avec la valeur sur laquelle le `?` est utilisé. C'est parce que l'opérateur `?` est défini pour effectuer un retour anticipé d'une valeur hors de la fonction, de la même manière que l'expression `match` que nous avons définie dans l'encart 9-6. Dans l'encart 9-6, le `match` utilisait une valeur `Result`, et la branche de retour anticipé renvoyait une valeur `Err(e)`. Le type de retour de la fonction doit être un `Result` pour être compatible avec ce `return`.

<!--
In Listing 9-10, let's look at the error we'll get if we use the `?` operator
in a `main` function with a return type that is incompatible with the type of
the value we use `?` on.
-->

Dans l'encart 9-10, examinons l'erreur que nous obtiendrons si nous utilisons l'opérateur `?` dans une fonction `main` avec un type de retour incompatible avec le type de la valeur sur laquelle nous utilisons `?`.


<Listing number="9-10" file-name="src/main.rs" caption="Tenter d'utiliser `?` dans la fonction `main` qui renvoie `()` ne compilera pas.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

<!--
This code opens a file, which might fail. The `?` operator follows the `Result`
value returned by `File::open`, but this `main` function has the return type of
`()`, not `Result`. When we compile this code, we get the following error
message:
-->

Ce code ouvre un fichier, ce qui pourrait échouer. L'opérateur `?` suit la valeur `Result` renvoyée par `File::open`, mais cette fonction `main` a le type de retour `()`, pas `Result`. Lorsque nous compilons ce code, nous obtenons le message d'erreur suivant :


```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

<!--
This error points out that we're only allowed to use the `?` operator in a
function that returns `Result`, `Option`, or another type that implements
`FromResidual`.
-->

Cette erreur indique que nous ne sommes autorisés à utiliser l'opérateur `?` que dans une fonction qui renvoie `Result`, `Option`, ou un autre type qui implémente `FromResidual`.

<!--
To fix the error, you have two choices. One choice is to change the return type
of your function to be compatible with the value you're using the `?` operator
on as long as you have no restrictions preventing that. The other choice is to
use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>`
in whatever way is appropriate.
-->

Pour corriger l'erreur, vous avez deux choix. Un choix consiste à modifier le type de retour de votre fonction pour le rendre compatible avec la valeur sur laquelle vous utilisez l'opérateur `?`, tant qu'il n'y a pas de restrictions qui l'empêchent. L'autre choix consiste à utiliser un `match` ou l'une des méthodes de `Result<T, E>` pour gérer le `Result<T, E>` de la manière appropriée.

<!--
The error message also mentioned that `?` can be used with `Option<T>` values
as well. As with using `?` on `Result`, you can only use `?` on `Option` in a
function that returns an `Option`. The behavior of the `?` operator when called
on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`:
If the value is `None`, the `None` will be returned early from the function at
that point. If the value is `Some`, the value inside the `Some` is the
resultant value of the expression, and the function continues. Listing 9-11 has
an example of a function that finds the last character of the first line in the
given text.
-->

Le message d'erreur mentionnait également que `?` peut être utilisé avec des valeurs `Option<T>`. Comme avec l'utilisation de `?` sur `Result`, vous ne pouvez utiliser `?` sur `Option` que dans une fonction qui renvoie une `Option`. Le comportement de l'opérateur `?` lorsqu'il est appelé sur un `Option<T>` est similaire à son comportement lorsqu'il est appelé sur un `Result<T, E>` : si la valeur est `None`, le `None` sera renvoyé prématurément par la fonction à ce moment-là. Si la valeur est `Some`, la valeur à l'intérieur du `Some` est la valeur résultante de l'expression, et la fonction continue. L'encart 9-11 contient un exemple de fonction qui trouve le dernier caractère de la première ligne dans le texte donné.


<Listing number="9-11" caption="Utilisation de l'opérateur `?` sur une valeur `Option<T>`">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

<!--
This function returns `Option<char>` because it's possible that there is a
character there, but it's also possible that there isn't. This code takes the
`text` string slice argument and calls the `lines` method on it, which returns
an iterator over the lines in the string. Because this function wants to
examine the first line, it calls `next` on the iterator to get the first value
from the iterator. If `text` is the empty string, this call to `next` will
return `None`, in which case we use `?` to stop and return `None` from
`last_char_of_first_line`. If `text` is not the empty string, `next` will
return a `Some` value containing a string slice of the first line in `text`.
-->

Cette fonction renvoie `Option<char>` parce qu'il est possible qu'il y ait un caractère, mais il est aussi possible qu'il n'y en ait pas. Ce code prend l'argument de tranche de chaîne `text` et appelle la méthode `lines` dessus, qui renvoie un itérateur sur les lignes de la chaîne. Comme cette fonction veut examiner la première ligne, elle appelle `next` sur l'itérateur pour obtenir la première valeur de l'itérateur. Si `text` est la chaîne vide, cet appel à `next` renverra `None`, auquel cas nous utilisons `?` pour nous arrêter et renvoyer `None` depuis `last_char_of_first_line`. Si `text` n'est pas la chaîne vide, `next` renverra une valeur `Some` contenant une tranche de chaîne de la première ligne de `text`.

<!--
The `?` extracts the string slice, and we can call `chars` on that string slice
to get an iterator of its characters. We're interested in the last character in
this first line, so we call `last` to return the last item in the iterator.
This is an `Option` because it's possible that the first line is the empty
string; for example, if `text` starts with a blank line but has characters on
other lines, as in `"\nhi"`. However, if there is a last character on the first
line, it will be returned in the `Some` variant. The `?` operator in the middle
gives us a concise way to express this logic, allowing us to implement the
function in one line. If we couldn't use the `?` operator on `Option`, we'd
have to implement this logic using more method calls or a `match` expression.
-->

Le `?` extrait la tranche de chaîne, et nous pouvons appeler `chars` sur cette tranche de chaîne pour obtenir un itérateur sur ses caractères. Nous nous intéressons au dernier caractère de cette première ligne, donc nous appelons `last` pour renvoyer le dernier élément de l'itérateur. C'est une `Option` parce qu'il est possible que la première ligne soit la chaîne vide ; par exemple, si `text` commence par une ligne vide mais contient des caractères sur d'autres lignes, comme dans `"\nhi"`. Cependant, s'il y a un dernier caractère sur la première ligne, il sera renvoyé dans la variante `Some`. L'opérateur `?` au milieu nous donne une manière concise d'exprimer cette logique, nous permettant d'implémenter la fonction en une seule ligne. Si nous ne pouvions pas utiliser l'opérateur `?` sur `Option`, nous devrions implémenter cette logique en utilisant plus d'appels de méthode ou une expression `match`.

<!--
Note that you can use the `?` operator on a `Result` in a function that returns
`Result`, and you can use the `?` operator on an `Option` in a function that
returns `Option`, but you can't mix and match. The `?` operator won't
automatically convert a `Result` to an `Option` or vice versa; in those cases,
you can use methods like the `ok` method on `Result` or the `ok_or` method on
`Option` to do the conversion explicitly.
-->

Notez que vous pouvez utiliser l'opérateur `?` sur un `Result` dans une fonction qui renvoie `Result`, et vous pouvez utiliser l'opérateur `?` sur une `Option` dans une fonction qui renvoie `Option`, mais vous ne pouvez pas mélanger les deux. L'opérateur `?` ne convertira pas automatiquement un `Result` en `Option` ou vice versa ; dans ces cas, vous pouvez utiliser des méthodes comme la méthode `ok` sur `Result` ou la méthode `ok_or` sur `Option` pour effectuer la conversion explicitement.

<!--
So far, all the `main` functions we've used return `()`. The `main` function is
special because it's the entry point and exit point of an executable program,
and there are restrictions on what its return type can be for the program to
behave as expected.
-->

Jusqu'à présent, toutes les fonctions `main` que nous avons utilisées renvoient `()`. La fonction `main` est spéciale car elle est le point d'entrée et de sortie d'un programme exécutable, et il y a des restrictions sur ce que son type de retour peut être pour que le programme se comporte comme prévu.

<!--
Luckily, `main` can also return a `Result<(), E>`. Listing 9-12 has the code
from Listing 9-10, but we've changed the return type of `main` to be
`Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end. This
code will now compile.
-->

Heureusement, `main` peut également renvoyer un `Result<(), E>`. L'encart 9-12 reprend le code de l'encart 9-10, mais nous avons changé le type de retour de `main` en `Result<(), Box<dyn Error>>` et ajouté une valeur de retour `Ok(())` à la fin. Ce code compilera maintenant.


<Listing number="9-12" file-name="src/main.rs" caption="Modifier `main` pour renvoyer `Result<(), E>` permet l'utilisation de l'opérateur `?` sur les valeurs `Result`.">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

<!--
The `Box<dyn Error>` type is a trait object, which we'll talk about in ["Using
Trait Objects to Abstract over Shared Behavior"][trait-objects] ignore
-->
in Chapter 18. For now, you can read `Box<dyn Error>` to mean "any kind of
error." Using `?` on a `Result` value in a `main` function with the error type
`Box<dyn Error>` is allowed because it allows any `Err` value to be returned
early. Even though the body of this `main` function will only ever return
errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature
will continue to be correct even if more code that returns other errors is
added to the body of `main`.
-->

Le type `Box<dyn Error>` est un objet trait, dont nous parlerons dans [« Utiliser des objets trait pour abstraire les comportements partagés »][trait-objects]<!--
ignore
--> au chapitre 18. Pour l'instant, vous pouvez lire `Box<dyn Error>` comme signifiant « n'importe quel type d'erreur ». Utiliser `?` sur une valeur `Result` dans une fonction `main` avec le type d'erreur `Box<dyn Error>` est autorisé car cela permet à n'importe quelle valeur `Err` d'être renvoyée prématurément. Même si le corps de cette fonction `main` ne renverra jamais que des erreurs de type `std::io::Error`, en spécifiant `Box<dyn Error>`, cette signature restera correcte même si du code supplémentaire renvoyant d'autres erreurs est ajouté au corps de `main`.

<!--
When a `main` function returns a `Result<(), E>`, the executable will exit with
a value of `0` if `main` returns `Ok(())` and will exit with a nonzero value if
`main` returns an `Err` value. Executables written in C return integers when
they exit: Programs that exit successfully return the integer `0`, and programs
that error return some integer other than `0`. Rust also returns integers from
executables to be compatible with this convention.
-->

Lorsqu'une fonction `main` renvoie un `Result<(), E>`, l'exécutable se terminera avec la valeur `0` si `main` renvoie `Ok(())` et se terminera avec une valeur non nulle si `main` renvoie une valeur `Err`. Les exécutables écrits en C renvoient des entiers lorsqu'ils se terminent : les programmes qui se terminent avec succès renvoient l'entier `0`, et les programmes qui échouent renvoient un entier autre que `0`. Rust renvoie également des entiers depuis les exécutables pour être compatible avec cette convention.

<!--
The `main` function may return any types that implement [the
`std::process::Termination` trait][termination] ignore
-->, which contains
a function `report` that returns an `ExitCode`. Consult the standard library
documentation for more information on implementing the `Termination` trait for
your own types.
-->

La fonction `main` peut renvoyer n'importe quel type qui implémente [le trait `std::process::Termination`][termination]<!--
ignore
-->, qui contient une fonction `report` renvoyant un `ExitCode`. Consultez la documentation de la bibliothèque standard pour plus d'informations sur l'implémentation du trait `Termination` pour vos propres types.

<!--
Now that we've discussed the details of calling `panic!` or returning `Result`,
let's return to the topic of how to decide which is appropriate to use in which
cases.
-->

Maintenant que nous avons abordé les détails de l'appel à `panic!` ou du renvoi de `Result`, revenons au sujet de la façon de décider lequel est approprié dans quels cas.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[termination]: ../std/process/trait.Termination.html
