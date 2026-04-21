<!--
Old headings. Do not remove or links may break.
-->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>
<a id="treating-smart-pointers-like-regular-references-with-deref"></a>

<!--
## Treating Smart Pointers Like Regular References
-->

## Traiter les pointeurs intelligents comme des références classiques

<!--
Implementing the `Deref` trait allows you to customize the behavior of the
_dereference operator_ `*` (not to be confused with the multiplication or glob
operator). By implementing `Deref` in such a way that a smart pointer can be
treated like a regular reference, you can write code that operates on
references and use that code with smart pointers too.
-->

Implémenter le trait `Deref` vous permet de personnaliser le comportement de l'_opérateur de déréférencement_ `*` (à ne pas confondre avec l'opérateur de multiplication ou le glob). En implémentant `Deref` de manière à ce qu'un pointeur intelligent puisse être traité comme une référence classique, vous pouvez écrire du code qui opère sur des références et utiliser ce code aussi avec des pointeurs intelligents.

<!--
Let's first look at how the dereference operator works with regular references.
Then, we'll try to define a custom type that behaves like `Box<T>` and see why
the dereference operator doesn't work like a reference on our newly defined
type. We'll explore how implementing the `Deref` trait makes it possible for
smart pointers to work in ways similar to references. Then, we'll look at
Rust's deref coercion feature and how it lets us work with either references or
smart pointers.
-->

Commençons par voir comment l'opérateur de déréférencement fonctionne avec des références classiques. Ensuite, nous essaierons de définir un type personnalisé qui se comporte comme `Box<T>` et nous verrons pourquoi l'opérateur de déréférencement ne fonctionne pas comme une référence sur notre type nouvellement défini. Nous explorerons comment l'implémentation du trait `Deref` permet aux pointeurs intelligents de fonctionner de manière similaire aux références. Puis, nous examinerons la fonctionnalité de coercition de déréférencement de Rust et comment elle nous permet de travailler avec des références ou des pointeurs intelligents.

<!--
Old headings. Do not remove or links may break.
-->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

<!--
### Following the Reference to the Value
-->

### Suivre la référence jusqu'à la valeur

<!--
A regular reference is a type of pointer, and one way to think of a pointer is
as an arrow to a value stored somewhere else. In Listing 15-6, we create a
reference to an `i32` value and then use the dereference operator to follow the
reference to the value.
-->

Une référence classique est un type de pointeur, et une façon de penser à un pointeur est comme une flèche vers une valeur stockée ailleurs. Dans l'encart 15-6, nous créons une référence vers une valeur `i32` puis utilisons l'opérateur de déréférencement pour suivre la référence jusqu'à la valeur.

<Listing number="15-6" file-name="src/main.rs" caption="Utiliser l'opérateur de déréférencement pour suivre une référence vers une valeur `i32`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

<!--
The variable `x` holds an `i32` value `5`. We set `y` equal to a reference to
`x`. We can assert that `x` is equal to `5`. However, if we want to make an
assertion about the value in `y`, we have to use `*y` to follow the reference
to the value it's pointing to (hence, _dereference_) so that the compiler can
compare the actual value. Once we dereference `y`, we have access to the
integer value `y` is pointing to that we can compare with `5`.
-->

La variable `x` contient une valeur `i32` de `5`. Nous définissons `y` égal à une référence vers `x`. Nous pouvons vérifier que `x` est égal à `5`. Cependant, si nous voulons faire une assertion sur la valeur dans `y`, nous devons utiliser `*y` pour suivre la référence jusqu'à la valeur vers laquelle elle pointe (d'où le _déréférencement_) afin que le compilateur puisse comparer la valeur réelle. Une fois que nous déréférençons `y`, nous avons accès à la valeur entière vers laquelle `y` pointe, que nous pouvons comparer avec `5`.

<!--
If we tried to write `assert_eq!(5, y);` instead, we would get this compilation
error:
-->

Si nous avions essayé d'écrire `assert_eq!(5, y);` à la place, nous aurions obtenu cette erreur de compilation :

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

<!--
Comparing a number and a reference to a number isn't allowed because they're
different types. We must use the dereference operator to follow the reference
to the value it's pointing to.
-->

Comparer un nombre et une référence vers un nombre n'est pas autorisé car ce sont des types différents. Nous devons utiliser l'opérateur de déréférencement pour suivre la référence jusqu'à la valeur vers laquelle elle pointe.

<!--
### Using `Box<T>` Like a Reference
-->

### Utiliser `Box<T>` comme une référence

<!--
We can rewrite the code in Listing 15-6 to use a `Box<T>` instead of a
reference; the dereference operator used on the `Box<T>` in Listing 15-7
functions in the same way as the dereference operator used on the reference in
Listing 15-6.
-->

Nous pouvons réécrire le code de l'encart 15-6 pour utiliser un `Box<T>` au lieu d'une référence ; l'opérateur de déréférencement utilisé sur le `Box<T>` dans l'encart 15-7 fonctionne de la même manière que l'opérateur de déréférencement utilisé sur la référence de l'encart 15-6.

<Listing number="15-7" file-name="src/main.rs" caption="Utiliser l'opérateur de déréférencement sur un `Box<i32>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

<!--
The main difference between Listing 15-7 and Listing 15-6 is that here we set
`y` to be an instance of a box pointing to a copied value of `x` rather than a
reference pointing to the value of `x`. In the last assertion, we can use the
dereference operator to follow the box's pointer in the same way that we did
when `y` was a reference. Next, we'll explore what is special about `Box<T>`
that enables us to use the dereference operator by defining our own box type.
-->

La principale différence entre l'encart 15-7 et l'encart 15-6 est qu'ici nous définissons `y` comme une instance d'une boîte pointant vers une copie de la valeur de `x` plutôt qu'une référence pointant vers la valeur de `x`. Dans la dernière assertion, nous pouvons utiliser l'opérateur de déréférencement pour suivre le pointeur de la boîte de la même manière que lorsque `y` était une référence. Ensuite, nous explorerons ce qui est spécial dans `Box<T>` et qui nous permet d'utiliser l'opérateur de déréférencement en définissant notre propre type de boîte.

<!--
### Defining Our Own Smart Pointer
-->

### Définir notre propre pointeur intelligent

<!--
Let's build a wrapper type similar to the `Box<T>` type provided by the
standard library to experience how smart pointer types behave differently from
references by default. Then, we'll look at how to add the ability to use the
dereference operator.
-->

Construisons un type enveloppeur similaire au type `Box<T>` fourni par la bibliothèque standard pour expérimenter comment les types de pointeurs intelligents se comportent différemment des références par défaut. Ensuite, nous verrons comment ajouter la possibilité d'utiliser l'opérateur de déréférencement.

<!--
> Note: There's one big difference between the `MyBox<T>` type we're about to
> build and the real `Box<T>`: Our version will not store its data on the heap.
> We are focusing this example on `Deref`, so where the data is actually stored
> is less important than the pointer-like behavior.
-->

> Remarque : il y a une grande différence entre le type `MyBox<T>` que nous allons
> construire et le vrai `Box<T>` : notre version ne stockera pas ses données sur le tas.
> Nous concentrons cet exemple sur `Deref`, donc l'endroit où les données sont réellement
> stockées est moins important que le comportement de type pointeur.

<!--
The `Box<T>` type is ultimately defined as a tuple struct with one element, so
Listing 15-8 defines a `MyBox<T>` type in the same way. We'll also define a
`new` function to match the `new` function defined on `Box<T>`.
-->

Le type `Box<T>` est fondamentalement défini comme une struct tuple avec un seul élément, donc l'encart 15-8 définit un type `MyBox<T>` de la même manière. Nous définirons également une fonction `new` correspondant à la fonction `new` définie sur `Box<T>`.

<Listing number="15-8" file-name="src/main.rs" caption="Définir un type `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

<!--
We define a struct named `MyBox` and declare a generic parameter `T` because we
want our type to hold values of any type. The `MyBox` type is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.
-->

Nous définissons une struct nommée `MyBox` et déclarons un paramètre générique `T` car nous voulons que notre type puisse contenir des valeurs de n'importe quel type. Le type `MyBox` est une struct tuple avec un élément de type `T`. La fonction `MyBox::new` prend un paramètre de type `T` et retourne une instance `MyBox` qui contient la valeur passée.

<!--
Let's try adding the `main` function in Listing 15-7 to Listing 15-8 and
changing it to use the `MyBox<T>` type we've defined instead of `Box<T>`. The
code in Listing 15-9 won't compile, because Rust doesn't know how to
dereference `MyBox`.
-->

Essayons d'ajouter la fonction `main` de l'encart 15-7 à l'encart 15-8 en la modifiant pour utiliser le type `MyBox<T>` que nous avons défini au lieu de `Box<T>`. Le code de l'encart 15-9 ne compilera pas, car Rust ne sait pas comment déréférencer `MyBox`.

<Listing number="15-9" file-name="src/main.rs" caption="Tentative d'utilisation de `MyBox<T>` de la même manière que les références et `Box<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

<!--
Here's the resultant compilation error:
-->

Voici l'erreur de compilation résultante :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

<!--
Our `MyBox<T>` type can't be dereferenced because we haven't implemented that
ability on our type. To enable dereferencing with the `*` operator, we
implement the `Deref` trait.
-->

Notre type `MyBox<T>` ne peut pas être déréférencé car nous n'avons pas implémenté cette capacité sur notre type. Pour activer le déréférencement avec l'opérateur `*`, nous implémentons le trait `Deref`.

<!--
Old headings. Do not remove or links may break.
-->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

<!--
### Implementing the `Deref` Trait
-->

### Implémenter le trait `Deref`

<!--
As discussed in ["Implementing a Trait on a Type"][impl-trait] ignore
--> in
Chapter 10, to implement a trait we need to provide implementations for the
trait's required methods. The `Deref` trait, provided by the standard library,
requires us to implement one method named `deref` that borrows `self` and
returns a reference to the inner data. Listing 15-10 contains an implementation
of `Deref` to add to the definition of `MyBox<T>`.
-->

Comme abordé dans ["Implémenter un trait sur un type"][impl-trait]<!--
ignore
--> au chapitre 10, pour implémenter un trait nous devons fournir des implémentations pour les méthodes requises du trait. Le trait `Deref`, fourni par la bibliothèque standard, nous demande d'implémenter une méthode nommée `deref` qui emprunte `self` et retourne une référence vers les données internes. L'encart 15-10 contient une implémentation de `Deref` à ajouter à la définition de `MyBox<T>`.

<Listing number="15-10" file-name="src/main.rs" caption="Implémenter `Deref` sur `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

<!--
The `type Target = T;` syntax defines an associated type for the `Deref` trait
to use. Associated types are a slightly different way of declaring a generic
parameter, but you don't need to worry about them for now; we'll cover them in
more detail in Chapter 20.
-->

La syntaxe `type Target = T;` définit un type associé que le trait `Deref` utilisera. Les types associés sont une manière légèrement différente de déclarer un paramètre générique, mais vous n'avez pas besoin de vous en préoccuper pour l'instant ; nous les couvrirons plus en détail au chapitre 20.

<!--
We fill in the body of the `deref` method with `&self.0` so that `deref`
returns a reference to the value we want to access with the `*` operator;
recall from ["Creating Different Types with Tuple Structs"][tuple-structs]
ignore
--> in Chapter 5 that `.0` accesses the first value in a tuple struct.
The `main` function in Listing 15-9 that calls `*` on the `MyBox<T>` value now
compiles, and the assertions pass!
-->

Nous remplissons le corps de la méthode `deref` avec `&self.0` pour que `deref` retourne une référence vers la valeur que nous voulons accéder avec l'opérateur `*` ; rappelez-vous de ["Créer des types différents avec les structs tuples"][tuple-structs]<!--
ignore
--> au chapitre 5 que `.0` accède à la première valeur d'une struct tuple. La fonction `main` de l'encart 15-9 qui appelle `*` sur la valeur `MyBox<T>` compile maintenant, et les assertions passent !

<!--
Without the `Deref` trait, the compiler can only dereference `&` references.
The `deref` method gives the compiler the ability to take a value of any type
that implements `Deref` and call the `deref` method to get a reference that
it knows how to dereference.
-->

Sans le trait `Deref`, le compilateur ne peut déréférencer que les références `&`. La méthode `deref` donne au compilateur la capacité de prendre une valeur de n'importe quel type qui implémente `Deref` et d'appeler la méthode `deref` pour obtenir une référence qu'il sait comment déréférencer.

<!--
When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this
code:

```rust,ignore
*(y.deref())
```
-->

Quand nous avons saisi `*y` dans l'encart 15-9, en coulisses Rust a en fait exécuté ce code :

```rust,ignore
*(y.deref())
```

<!--
Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so that we don't have to think about whether or not we need
to call the `deref` method. This Rust feature lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.
-->

Rust substitue l'opérateur `*` par un appel à la méthode `deref` suivi d'un déréférencement simple afin que nous n'ayons pas à nous demander si nous devons appeler la méthode `deref` ou non. Cette fonctionnalité de Rust nous permet d'écrire du code qui fonctionne de manière identique que nous ayons une référence classique ou un type qui implémente `Deref`.

<!--
The reason the `deref` method returns a reference to a value, and that the
plain dereference outside the parentheses in `*(y.deref())` is still necessary,
has to do with the ownership system. If the `deref` method returned the value
directly instead of a reference to the value, the value would be moved out of
`self`. We don't want to take ownership of the inner value inside `MyBox<T>` in
this case or in most cases where we use the dereference operator.
-->

La raison pour laquelle la méthode `deref` retourne une référence vers une valeur, et que le déréférencement simple en dehors des parenthèses dans `*(y.deref())` est toujours nécessaire, est liée au système de possession. Si la méthode `deref` retournait la valeur directement au lieu d'une référence vers la valeur, la valeur serait déplacée hors de `self`. Nous ne voulons pas prendre la possession de la valeur intérieure dans `MyBox<T>` dans ce cas ni dans la plupart des cas où nous utilisons l'opérateur de déréférencement.

<!--
Note that the `*` operator is replaced with a call to the `deref` method and
then a call to the `*` operator just once, each time we use a `*` in our code.
Because the substitution of the `*` operator does not recurse infinitely, we
end up with data of type `i32`, which matches the `5` in `assert_eq!` in
Listing 15-9.
-->

Notez que l'opérateur `*` est remplacé par un appel à la méthode `deref` puis un appel à l'opérateur `*` une seule fois, à chaque fois que nous utilisons un `*` dans notre code. Comme la substitution de l'opérateur `*` ne récurse pas indéfiniment, nous obtenons des données de type `i32`, qui correspondent au `5` dans `assert_eq!` de l'encart 15-9.

<!--
Old headings. Do not remove or links may break.
-->

<a id="implicit-deref-coercions-with-functions-and-methods"></a>
<a id="using-deref-coercions-in-functions-and-methods"></a>

<!--
### Using Deref Coercion in Functions and Methods
-->

### Utiliser la coercition de déréférencement dans les fonctions et les méthodes

<!--
_Deref coercion_ converts a reference to a type that implements the `Deref`
trait into a reference to another type. For example, deref coercion can convert
`&String` to `&str` because `String` implements the `Deref` trait such that it
returns `&str`. Deref coercion is a convenience Rust performs on arguments to
functions and methods, and it works only on types that implement the `Deref`
trait. It happens automatically when we pass a reference to a particular type's
value as an argument to a function or method that doesn't match the parameter
type in the function or method definition. A sequence of calls to the `deref`
method converts the type we provided into the type the parameter needs.
-->

La _coercition de déréférencement_ (deref coercion) convertit une référence vers un type qui implémente le trait `Deref` en une référence vers un autre type. Par exemple, la coercition de déréférencement peut convertir `&String` en `&str` car `String` implémente le trait `Deref` de manière à retourner `&str`. La coercition de déréférencement est une facilité que Rust effectue sur les arguments des fonctions et des méthodes, et elle ne fonctionne que sur les types qui implémentent le trait `Deref`. Elle se produit automatiquement lorsque nous passons une référence vers la valeur d'un type particulier comme argument à une fonction ou une méthode dont le type de paramètre ne correspond pas dans la définition de la fonction ou de la méthode. Une séquence d'appels à la méthode `deref` convertit le type que nous avons fourni en le type dont le paramètre a besoin.

<!--
Deref coercion was added to Rust so that programmers writing function and
method calls don't need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.
-->

La coercition de déréférencement a été ajoutée à Rust pour que les programmeurs écrivant des appels de fonctions et de méthodes n'aient pas besoin d'ajouter autant de références et de déréférencements explicites avec `&` et `*`. La fonctionnalité de coercition de déréférencement nous permet également d'écrire plus de code qui fonctionne aussi bien avec des références qu'avec des pointeurs intelligents.

<!--
To see deref coercion in action, let's use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter.
-->

Pour voir la coercition de déréférencement en action, utilisons le type `MyBox<T>` que nous avons défini dans l'encart 15-8 ainsi que l'implémentation de `Deref` que nous avons ajoutée dans l'encart 15-10. L'encart 15-11 montre la définition d'une fonction qui a un paramètre de type slice de chaîne de caractères.

<Listing number="15-11" file-name="src/main.rs" caption="Une fonction `hello` qui a le paramètre `name` de type `&str`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

<!--
We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");`, for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12.
-->

Nous pouvons appeler la fonction `hello` avec une slice de chaîne de caractères comme argument, comme `hello("Rust");` par exemple. La coercition de déréférencement rend possible l'appel de `hello` avec une référence vers une valeur de type `MyBox<String>`, comme montré dans l'encart 15-12.

<Listing number="15-12" file-name="src/main.rs" caption="Appeler `hello` avec une référence vers une valeur `MyBox<String>`, ce qui fonctionne grâce à la coercition de déréférencement">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

<!--
Here we're calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, and this is in the API documentation
for `Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function's definition.
-->

Ici, nous appelons la fonction `hello` avec l'argument `&m`, qui est une référence vers une valeur `MyBox<String>`. Comme nous avons implémenté le trait `Deref` sur `MyBox<T>` dans l'encart 15-10, Rust peut transformer `&MyBox<String>` en `&String` en appelant `deref`. La bibliothèque standard fournit une implémentation de `Deref` sur `String` qui retourne une slice de chaîne de caractères, et cela se trouve dans la documentation de l'API de `Deref`. Rust appelle `deref` à nouveau pour transformer le `&String` en `&str`, ce qui correspond à la définition de la fonction `hello`.

<!--
If Rust didn't implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`.
-->

Si Rust n'implémentait pas la coercition de déréférencement, nous devrions écrire le code de l'encart 15-13 au lieu du code de l'encart 15-12 pour appeler `hello` avec une valeur de type `&MyBox<String>`.

<Listing number="15-13" file-name="src/main.rs" caption="Le code que nous devrions écrire si Rust n'avait pas la coercition de déréférencement">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

<!--
The `(*m)` dereferences the `MyBox<String>` into a `String`. Then, the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. This code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.
-->

Le `(*m)` déréférence le `MyBox<String>` en un `String`. Ensuite, le `&` et le `[..]` prennent une slice de chaîne de caractères du `String` qui est égale à la chaîne entière pour correspondre à la signature de `hello`. Ce code sans coercition de déréférencement est plus difficile à lire, écrire et comprendre avec tous ces symboles impliqués. La coercition de déréférencement permet à Rust de gérer ces conversions pour nous automatiquement.

<!--
When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter's type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!
-->

Quand le trait `Deref` est défini pour les types concernés, Rust analysera les types et utilisera `Deref::deref` autant de fois que nécessaire pour obtenir une référence correspondant au type du paramètre. Le nombre de fois où `Deref::deref` doit être inséré est résolu à la compilation, il n'y a donc aucune pénalité à l'exécution pour tirer parti de la coercition de déréférencement !

<!--
Old headings. Do not remove or links may break.
-->

<a id="how-deref-coercion-interacts-with-mutability"></a>

<!--
### Handling Deref Coercion with Mutable References
-->

### Gérer la coercition de déréférencement avec les références mutables

<!--
Similar to how you use the `Deref` trait to override the `*` operator on
immutable references, you can use the `DerefMut` trait to override the `*`
operator on mutable references.
-->

De la même manière que vous utilisez le trait `Deref` pour redéfinir l'opérateur `*` sur les références immuables, vous pouvez utiliser le trait `DerefMut` pour redéfinir l'opérateur `*` sur les références mutables.

<!--
Rust does deref coercion when it finds types and trait implementations in three
cases:

1. From `&T` to `&U` when `T: Deref<Target=U>`
2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
3. From `&mut T` to `&U` when `T: Deref<Target=U>`
-->

Rust effectue la coercition de déréférencement lorsqu'il trouve des types et des implémentations de traits dans trois cas :

1. De `&T` vers `&U` quand `T: Deref<Target=U>`
2. De `&mut T` vers `&mut U` quand `T: DerefMut<Target=U>`
3. De `&mut T` vers `&U` quand `T: Deref<Target=U>`

<!--
The first two cases are the same except that the second implements mutability.
The first case states that if you have a `&T`, and `T` implements `Deref` to
some type `U`, you can get a `&U` transparently. The second case states that
the same deref coercion happens for mutable references.
-->

Les deux premiers cas sont identiques sauf que le deuxième implémente la mutabilité. Le premier cas indique que si vous avez un `&T`, et que `T` implémente `Deref` vers un certain type `U`, vous pouvez obtenir un `&U` de manière transparente. Le deuxième cas indique que la même coercition de déréférencement se produit pour les références mutables.

<!--
The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is _not_ possible: Immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn't compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that the
initial immutable reference is the only immutable reference to that data, but
the borrowing rules don't guarantee that. Therefore, Rust can't make the
assumption that converting an immutable reference to a mutable reference is
possible.
-->

Le troisième cas est plus délicat : Rust convertira aussi une référence mutable en une référence immuable. Mais l'inverse n'est _pas_ possible : les références immuables ne seront jamais converties en références mutables. En raison des règles d'emprunt, si vous avez une référence mutable, cette référence mutable doit être la seule référence vers ces données (sinon, le programme ne compilerait pas). Convertir une référence mutable en une référence immuable ne violera jamais les règles d'emprunt. Convertir une référence immuable en une référence mutable nécessiterait que la référence immuable initiale soit la seule référence immuable vers ces données, mais les règles d'emprunt ne le garantissent pas. Par conséquent, Rust ne peut pas supposer que convertir une référence immuable en une référence mutable est possible.

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
