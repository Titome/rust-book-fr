<!--
## Reference Cycles Can Leak Memory
-->

## Les cycles de références peuvent provoquer des fuites de mémoire

<!--
Rust's memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a _memory leak_).
Preventing memory leaks entirely is not one of Rust's guarantees, meaning
memory leaks are memory safe in Rust. We can see that Rust allows memory leaks
by using `Rc<T>` and `RefCell<T>`: It's possible to create references where
items refer to each other in a cycle. This creates memory leaks because the
reference count of each item in the cycle will never reach 0, and the values
will never be dropped.
-->

Les garanties de sécurité de la mémoire de Rust rendent difficile, mais pas impossible, la création accidentelle de mémoire qui n'est jamais nettoyée (connue sous le nom de _fuite de mémoire_). Prévenir entièrement les fuites de mémoire ne fait pas partie des garanties de Rust, ce qui signifie que les fuites de mémoire sont considérées comme sûres en mémoire en Rust. Nous pouvons constater que Rust permet les fuites de mémoire en utilisant `Rc<T>` et `RefCell<T>` : il est possible de créer des références où les éléments se réfèrent les uns aux autres dans un cycle. Cela crée des fuites de mémoire car le compteur de références de chaque élément dans le cycle n'atteindra jamais 0, et les valeurs ne seront jamais libérées.

<!--
### Creating a Reference Cycle
-->

### Créer un cycle de références

<!--
Let's look at how a reference cycle might happen and how to prevent it,
starting with the definition of the `List` enum and a `tail` method in Listing
15-25.
-->

Voyons comment un cycle de références peut se produire et comment le prévenir, en commençant par la définition de l'enum `List` et une méthode `tail` dans l'encart 15-25.

<Listing number="15-25" file-name="src/main.rs" caption="Une définition de liste cons qui contient un `RefCell<T>` pour pouvoir modifier ce à quoi une variante `Cons` fait référence">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs:here}}
```

</Listing>

<!--
We're using another variation of the `List` definition from Listing 15-5. The
second element in the `Cons` variant is now `RefCell<Rc<List>>`, meaning that
instead of having the ability to modify the `i32` value as we did in Listing
15-24, we want to modify the `List` value a `Cons` variant is pointing to.
We're also adding a `tail` method to make it convenient for us to access the
second item if we have a `Cons` variant.
-->

Nous utilisons une autre variation de la définition de `List` de l'encart 15-5. Le deuxième élément de la variante `Cons` est maintenant `RefCell<Rc<List>>`, ce qui signifie qu'au lieu de pouvoir modifier la valeur `i32` comme nous l'avons fait dans l'encart 15-24, nous voulons modifier la valeur `List` vers laquelle une variante `Cons` pointe. Nous ajoutons aussi une méthode `tail` pour faciliter l'accès au deuxième élément si nous avons une variante `Cons`.

<!--
In Listing 15-26, we're adding a `main` function that uses the definitions in
Listing 15-25. This code creates a list in `a` and a list in `b` that points to
the list in `a`. Then, it modifies the list in `a` to point to `b`, creating a
reference cycle. There are `println!` statements along the way to show what the
reference counts are at various points in this process.
-->

Dans l'encart 15-26, nous ajoutons une fonction `main` qui utilise les définitions de l'encart 15-25. Ce code crée une liste dans `a` et une liste dans `b` qui pointe vers la liste dans `a`. Ensuite, il modifie la liste dans `a` pour pointer vers `b`, créant un cycle de références. Il y a des instructions `println!` tout au long du processus pour montrer quels sont les compteurs de références à différents moments.

<Listing number="15-26" file-name="src/main.rs" caption="Créer un cycle de références de deux valeurs `List` pointant l'une vers l'autre">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

<!--
We create an `Rc<List>` instance holding a `List` value in the variable `a`
with an initial list of `5, Nil`. We then create an `Rc<List>` instance holding
another `List` value in the variable `b` that contains the value `10` and
points to the list in `a`.
-->

Nous créons une instance `Rc<List>` contenant une valeur `List` dans la variable `a` avec une liste initiale de `5, Nil`. Nous créons ensuite une instance `Rc<List>` contenant une autre valeur `List` dans la variable `b` qui contient la valeur `10` et pointe vers la liste dans `a`.

<!--
We modify `a` so that it points to `b` instead of `Nil`, creating a cycle. We
do that by using the `tail` method to get a reference to the
`RefCell<Rc<List>>` in `a`, which we put in the variable `link`. Then, we use
the `borrow_mut` method on the `RefCell<Rc<List>>` to change the value inside
from an `Rc<List>` that holds a `Nil` value to the `Rc<List>` in `b`.
-->

Nous modifions `a` pour qu'il pointe vers `b` au lieu de `Nil`, créant un cycle. Nous le faisons en utilisant la méthode `tail` pour obtenir une référence vers le `RefCell<Rc<List>>` dans `a`, que nous mettons dans la variable `link`. Ensuite, nous utilisons la méthode `borrow_mut` sur le `RefCell<Rc<List>>` pour changer la valeur à l'intérieur d'un `Rc<List>` contenant une valeur `Nil` vers le `Rc<List>` dans `b`.

<!--
When we run this code, keeping the last `println!` commented out for the
moment, we'll get this output:
-->

Quand nous exécutons ce code, en gardant le dernier `println!` commenté pour le moment, nous obtenons cette sortie :

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

<!--
The reference count of the `Rc<List>` instances in both `a` and `b` is 2 after
we change the list in `a` to point to `b`. At the end of `main`, Rust drops the
variable `b`, which decreases the reference count of the `b` `Rc<List>`
instance from 2 to 1. The memory that `Rc<List>` has on the heap won't be
dropped at this point because its reference count is 1, not 0. Then, Rust drops
`a`, which decreases the reference count of the `a` `Rc<List>` instance from 2
to 1 as well. This instance's memory can't be dropped either, because the other
`Rc<List>` instance still refers to it. The memory allocated to the list will
remain uncollected forever. To visualize this reference cycle, we've created
the diagram in Figure 15-4.
-->

Le compteur de références des instances `Rc<List>` dans `a` et `b` est de 2 après que nous avons modifié la liste dans `a` pour pointer vers `b`. À la fin de `main`, Rust libère la variable `b`, ce qui diminue le compteur de références de l'instance `Rc<List>` de `b` de 2 à 1. La mémoire que `Rc<List>` occupe sur le tas ne sera pas libérée à ce stade car son compteur de références est 1, pas 0. Ensuite, Rust libère `a`, ce qui diminue aussi le compteur de références de l'instance `Rc<List>` de `a` de 2 à 1. La mémoire de cette instance ne peut pas non plus être libérée, car l'autre instance `Rc<List>` y fait encore référence. La mémoire allouée à la liste restera non récupérée pour toujours. Pour visualiser ce cycle de références, nous avons créé le diagramme de la figure 15-4.

<img alt="A rectangle labeled 'a' that points to a rectangle containing the integer 5. A rectangle labeled 'b' that points to a rectangle containing the integer 10. The rectangle containing 5 points to the rectangle containing 10, and the rectangle containing 10 points back to the rectangle containing 5, creating a cycle." src="img/trpl15-04.svg" class="center" />

<!--
<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>
-->

<span class="caption">Figure 15-4 : Un cycle de références des listes `a` et `b` pointant l'une vers l'autre</span>

<!--
If you uncomment the last `println!` and run the program, Rust will try to
print this cycle with `a` pointing to `b` pointing to `a` and so forth until it
overflows the stack.
-->

Si vous décommentez le dernier `println!` et exécutez le programme, Rust essaiera d'afficher ce cycle avec `a` pointant vers `b` pointant vers `a` et ainsi de suite jusqu'à ce qu'il déborde la pile.

<!--
Compared to a real-world program, the consequences of creating a reference
cycle in this example aren't very dire: Right after we create the reference
cycle, the program ends. However, if a more complex program allocated lots of
memory in a cycle and held onto it for a long time, the program would use more
memory than it needed and might overwhelm the system, causing it to run out of
available memory.
-->

Comparé à un programme réel, les conséquences de la création d'un cycle de références dans cet exemple ne sont pas très graves : juste après avoir créé le cycle de références, le programme se termine. Cependant, si un programme plus complexe allouait beaucoup de mémoire dans un cycle et la conservait longtemps, le programme utiliserait plus de mémoire que nécessaire et pourrait submerger le système, le faisant manquer de mémoire disponible.

<!--
Creating reference cycles is not easily done, but it's not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don't create cycles; you can't rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.
-->

Créer des cycles de références ne se fait pas facilement, mais ce n'est pas impossible non plus. Si vous avez des valeurs `RefCell<T>` qui contiennent des valeurs `Rc<T>` ou des combinaisons imbriquées similaires de types avec mutabilité intérieure et comptage de références, vous devez vous assurer de ne pas créer de cycles ; vous ne pouvez pas compter sur Rust pour les détecter. Créer un cycle de références serait un bogue logique dans votre programme que vous devriez minimiser en utilisant des tests automatisés, des revues de code et d'autres pratiques de développement logiciel.

<!--
Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don't.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped. In Listing 15-25, we always want `Cons`
variants to own their list, so reorganizing the data structure isn't possible.
Let's look at an example using graphs made up of parent nodes and child nodes
to see when non-ownership relationships are an appropriate way to prevent
reference cycles.
-->

Une autre solution pour éviter les cycles de références est de réorganiser vos structures de données de sorte que certaines références expriment la possession et d'autres non. Ainsi, vous pouvez avoir des cycles composés de certaines relations de possession et de certaines relations sans possession, et seules les relations de possession affectent si une valeur peut être libérée ou non. Dans l'encart 15-25, nous voulons toujours que les variantes `Cons` possèdent leur liste, donc réorganiser la structure de données n'est pas possible. Examinons un exemple utilisant des graphes composés de noeuds parents et de noeuds enfants pour voir quand les relations sans possession sont un moyen approprié de prévenir les cycles de références.

<!--
Old headings. Do not remove or links may break.
-->

<a id="preventing-reference-cycles-turning-an-rct-into-a-weakt"></a>

<!--
### Preventing Reference Cycles Using `Weak<T>`
-->

### Prévenir les cycles de références avec `Weak<T>`

<!--
So far, we've demonstrated that calling `Rc::clone` increases the
`strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned
up if its `strong_count` is 0. You can also create a weak reference to the
value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a
reference to the `Rc<T>`. *Strong references* are how you can share ownership
of an `Rc<T>` instance. *Weak references* don't express an ownership
relationship, and their count doesn't affect when an `Rc<T>` instance is
cleaned up. They won't cause a reference cycle, because any cycle involving
some weak references will be broken once the strong reference count of values
involved is 0.
-->

Jusqu'ici, nous avons démontré que l'appel à `Rc::clone` augmente le `strong_count` d'une instance `Rc<T>`, et qu'une instance `Rc<T>` n'est nettoyée que si son `strong_count` est à 0. Vous pouvez aussi créer une référence faible vers la valeur dans une instance `Rc<T>` en appelant `Rc::downgrade` et en passant une référence vers le `Rc<T>`. Les *références fortes* sont la façon dont vous pouvez partager la possession d'une instance `Rc<T>`. Les *références faibles* n'expriment pas une relation de possession, et leur compteur n'affecte pas le moment où une instance `Rc<T>` est nettoyée. Elles ne causeront pas de cycle de références, car tout cycle impliquant des références faibles sera brisé une fois que le compteur de références fortes des valeurs impliquées est à 0.

<!--
When you call `Rc::downgrade`, you get a smart pointer of type `Weak<T>`.
Instead of increasing the `strong_count` in the `Rc<T>` instance by 1, calling
`Rc::downgrade` increases the `weak_count` by 1. The `Rc<T>` type uses
`weak_count` to keep track of how many `Weak<T>` references exist, similar to
`strong_count`. The difference is the `weak_count` doesn't need to be 0 for the
`Rc<T>` instance to be cleaned up.
-->

Quand vous appelez `Rc::downgrade`, vous obtenez un pointeur intelligent de type `Weak<T>`. Au lieu d'augmenter le `strong_count` dans l'instance `Rc<T>` de 1, l'appel à `Rc::downgrade` augmente le `weak_count` de 1. Le type `Rc<T>` utilise `weak_count` pour suivre combien de références `Weak<T>` existent, de manière similaire à `strong_count`. La différence est que le `weak_count` n'a pas besoin d'être à 0 pour que l'instance `Rc<T>` soit nettoyée.

<!--
Because the value that `Weak<T>` references might have been dropped, to do
anything with the value that a `Weak<T>` is pointing to you must make sure the
value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
instance, which will return an `Option<Rc<T>>`. You'll get a result of `Some`
if the `Rc<T>` value has not been dropped yet and a result of `None` if the
`Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`,
Rust will ensure that the `Some` case and the `None` case are handled, and
there won't be an invalid pointer.
-->

Comme la valeur que `Weak<T>` référence pourrait avoir été libérée, pour faire quoi que ce soit avec la valeur vers laquelle un `Weak<T>` pointe, vous devez vous assurer que la valeur existe encore. Faites-le en appelant la méthode `upgrade` sur une instance `Weak<T>`, qui retournera un `Option<Rc<T>>`. Vous obtiendrez un résultat `Some` si la valeur `Rc<T>` n'a pas encore été libérée et un résultat `None` si la valeur `Rc<T>` a été libérée. Comme `upgrade` retourne un `Option<Rc<T>>`, Rust s'assurera que les cas `Some` et `None` sont gérés, et il n'y aura pas de pointeur invalide.

<!--
As an example, rather than using a list whose items know only about the next
item, we'll create a tree whose items know about their child items _and_ their
parent items.
-->

Comme exemple, plutôt que d'utiliser une liste dont les éléments ne connaissent que l'élément suivant, nous allons créer un arbre dont les éléments connaissent leurs éléments enfants _et_ leurs éléments parents.

<!--
Old headings. Do not remove or links may break.
-->

<a id="creating-a-tree-data-structure-a-node-with-child-nodes"></a>

<!--
#### Creating a Tree Data Structure
-->

#### Créer une structure de données en arbre

<!--
To start, we'll build a tree with nodes that know about their child nodes.
We'll create a struct named `Node` that holds its own `i32` value as well as
references to its child `Node` values:
-->

Pour commencer, nous allons construire un arbre avec des noeuds qui connaissent leurs noeuds enfants. Nous allons créer une struct nommée `Node` qui contient sa propre valeur `i32` ainsi que des références vers les valeurs de ses `Node` enfants :

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

<!--
We want a `Node` to own its children, and we want to share that ownership with
variables so that we can access each `Node` in the tree directly. To do this,
we define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to
modify which nodes are children of another node, so we have a `RefCell<T>` in
`children` around the `Vec<Rc<Node>>`.
-->

Nous voulons qu'un `Node` possède ses enfants, et nous voulons partager cette possession avec des variables afin de pouvoir accéder directement à chaque `Node` dans l'arbre. Pour ce faire, nous définissons les éléments du `Vec<T>` comme des valeurs de type `Rc<Node>`. Nous voulons aussi modifier quels noeuds sont les enfants d'un autre noeud, donc nous avons un `RefCell<T>` dans `children` autour du `Vec<Rc<Node>>`.

<!--
Next, we'll use our struct definition and create one `Node` instance named
`leaf` with the value `3` and no children, and another instance named `branch`
with the value `5` and `leaf` as one of its children, as shown in Listing 15-27.
-->

Ensuite, nous utiliserons notre définition de struct pour créer une instance `Node` nommée `leaf` avec la valeur `3` et sans enfants, et une autre instance nommée `branch` avec la valeur `5` et `leaf` comme l'un de ses enfants, comme montré dans l'encart 15-27.

<Listing number="15-27" file-name="src/main.rs" caption="Créer un noeud `leaf` sans enfants et un noeud `branch` avec `leaf` comme l'un de ses enfants">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

<!--
We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the
`Node` in `leaf` now has two owners: `leaf` and `branch`. We can get from
`branch` to `leaf` through `branch.children`, but there's no way to get from
`leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and
doesn't know they're related. We want `leaf` to know that `branch` is its
parent. We'll do that next.
-->

Nous clonons le `Rc<Node>` dans `leaf` et le stockons dans `branch`, ce qui signifie que le `Node` dans `leaf` a maintenant deux propriétaires : `leaf` et `branch`. Nous pouvons aller de `branch` à `leaf` via `branch.children`, mais il n'y a aucun moyen d'aller de `leaf` à `branch`. La raison est que `leaf` n'a pas de référence vers `branch` et ne sait pas qu'ils sont liés. Nous voulons que `leaf` sache que `branch` est son parent. Nous ferons cela ensuite.

<!--
#### Adding a Reference from a Child to Its Parent
-->

#### Ajouter une référence d'un enfant vers son parent

<!--
To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can't contain an `Rc<T>`, because that would
create a reference cycle with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be 0.
-->

Pour que le noeud enfant connaisse son parent, nous devons ajouter un champ `parent` à notre définition de la struct `Node`. La difficulté réside dans le choix du type de `parent`. Nous savons qu'il ne peut pas contenir un `Rc<T>`, car cela créerait un cycle de références avec `leaf.parent` pointant vers `branch` et `branch.children` pointant vers `leaf`, ce qui ferait que leurs valeurs `strong_count` ne seraient jamais à 0.

<!--
Thinking about the relationships another way, a parent node should own its
children: If a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: If we drop a child node, the
parent should still exist. This is a case for weak references!
-->

En réfléchissant aux relations d'une autre manière, un noeud parent devrait posséder ses enfants : si un noeud parent est libéré, ses noeuds enfants devraient l'être aussi. Cependant, un enfant ne devrait pas posséder son parent : si nous libérons un noeud enfant, le parent devrait toujours exister. C'est un cas d'utilisation pour les références faibles !

<!--
So, instead of `Rc<T>`, we'll make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks
like this:
-->

Donc, au lieu de `Rc<T>`, nous allons faire en sorte que le type de `parent` utilise `Weak<T>`, spécifiquement un `RefCell<Weak<Node>>`. Maintenant, notre définition de la struct `Node` ressemble à ceci :

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

<!--
A node will be able to refer to its parent node but doesn't own its parent. In
Listing 15-28, we update `main` to use this new definition so that the `leaf`
node will have a way to refer to its parent, `branch`.
-->

Un noeud pourra faire référence à son noeud parent mais ne possède pas son parent. Dans l'encart 15-28, nous mettons à jour `main` pour utiliser cette nouvelle définition afin que le noeud `leaf` ait un moyen de faire référence à son parent, `branch`.

<Listing number="15-28" file-name="src/main.rs" caption="Un noeud `leaf` avec une référence faible vers son noeud parent, `branch`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

<!--
Creating the `leaf` node looks similar to Listing 15-27 with the exception of
the `parent` field: `leaf` starts out without a parent, so we create a new,
empty `Weak<Node>` reference instance.
-->

La création du noeud `leaf` ressemble à l'encart 15-27 à l'exception du champ `parent` : `leaf` commence sans parent, donc nous créons une nouvelle instance de référence `Weak<Node>` vide.

<!--
At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!` statement:
-->

À ce stade, quand nous essayons d'obtenir une référence vers le parent de `leaf` en utilisant la méthode `upgrade`, nous obtenons une valeur `None`. Nous voyons cela dans la sortie de la première instruction `println!` :

```text
leaf parent = None
```

<!--
When we create the `branch` node, it will also have a new `Weak<Node>`
reference in the `parent` field because `branch` doesn't have a parent node. We
still have `leaf` as one of the children of `branch`. Once we have the `Node`
instance in `branch`, we can modify `leaf` to give it a `Weak<Node>` reference
to its parent. We use the `borrow_mut` method on the `RefCell<Weak<Node>>` in
the `parent` field of `leaf`, and then we use the `Rc::downgrade` function to
create a `Weak<Node>` reference to `branch` from the `Rc<Node>` in `branch`.
-->

Quand nous créons le noeud `branch`, il aura aussi une nouvelle référence `Weak<Node>` dans le champ `parent` car `branch` n'a pas de noeud parent. Nous avons toujours `leaf` comme l'un des enfants de `branch`. Une fois que nous avons l'instance `Node` dans `branch`, nous pouvons modifier `leaf` pour lui donner une référence `Weak<Node>` vers son parent. Nous utilisons la méthode `borrow_mut` sur le `RefCell<Weak<Node>>` dans le champ `parent` de `leaf`, puis nous utilisons la fonction `Rc::downgrade` pour créer une référence `Weak<Node>` vers `branch` à partir du `Rc<Node>` dans `branch`.

<!--
When we print the parent of `leaf` again, this time we'll get a `Some` variant
holding `branch`: Now `leaf` can access its parent! When we print `leaf`, we
also avoid the cycle that eventually ended in a stack overflow like we had in
Listing 15-26; the `Weak<Node>` references are printed as `(Weak)`:
-->

Quand nous affichons le parent de `leaf` à nouveau, cette fois nous obtenons une variante `Some` contenant `branch` : maintenant `leaf` peut accéder à son parent ! Quand nous affichons `leaf`, nous évitons aussi le cycle qui finissait par un débordement de pile comme dans l'encart 15-26 ; les références `Weak<Node>` sont affichées comme `(Weak)` :

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

<!--
The lack of infinite output indicates that this code didn't create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.
-->

L'absence de sortie infinie indique que ce code n'a pas créé de cycle de références. Nous pouvons aussi le constater en regardant les valeurs que nous obtenons en appelant `Rc::strong_count` et `Rc::weak_count`.

<!--
#### Visualizing Changes to `strong_count` and `weak_count`
-->

#### Visualiser les changements de `strong_count` et `weak_count`

<!--
Let's look at how the `strong_count` and `weak_count` values of the `Rc<Node>`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. By doing so, we can see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-29.
-->

Voyons comment les valeurs `strong_count` et `weak_count` des instances `Rc<Node>` changent en créant une nouvelle portée intérieure et en déplaçant la création de `branch` dans cette portée. Ce faisant, nous pouvons voir ce qui se passe quand `branch` est créé puis libéré quand il sort de la portée. Les modifications sont montrées dans l'encart 15-29.

<Listing number="15-29" file-name="src/main.rs" caption="Créer `branch` dans une portée intérieure et examiner les compteurs de références fortes et faibles">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

<!--
After `leaf` is created, its `Rc<Node>` has a strong count of 1 and a weak
count of 0. In the inner scope, we create `branch` and associate it with
`leaf`, at which point when we print the counts, the `Rc<Node>` in `branch`
will have a strong count of 1 and a weak count of 1 (for `leaf.parent` pointing
to `branch` with a `Weak<Node>`). When we print the counts in `leaf`, we'll see
it will have a strong count of 2 because `branch` now has a clone of the
`Rc<Node>` of `leaf` stored in `branch.children` but will still have a weak
count of 0.
-->

Après la création de `leaf`, son `Rc<Node>` a un compteur fort de 1 et un compteur faible de 0. Dans la portée intérieure, nous créons `branch` et l'associons à `leaf`, à ce moment-là, quand nous affichons les compteurs, le `Rc<Node>` dans `branch` aura un compteur fort de 1 et un compteur faible de 1 (pour `leaf.parent` pointant vers `branch` avec un `Weak<Node>`). Quand nous affichons les compteurs dans `leaf`, nous verrons qu'il aura un compteur fort de 2 car `branch` a maintenant un clone du `Rc<Node>` de `leaf` stocké dans `branch.children` mais aura toujours un compteur faible de 0.

<!--
When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc<Node>` decreases to 0, so its `Node` is dropped. The weak count of 1
from `leaf.parent` has no bearing on whether or not `Node` is dropped, so we
don't get any memory leaks!
-->

Quand la portée intérieure se termine, `branch` sort de la portée et le compteur fort du `Rc<Node>` diminue à 0, donc son `Node` est libéré. Le compteur faible de 1 provenant de `leaf.parent` n'a aucune incidence sur le fait que `Node` soit libéré ou non, donc nous n'avons aucune fuite de mémoire !

<!--
If we try to access the parent of `leaf` after the end of the scope, we'll get
`None` again. At the end of the program, the `Rc<Node>` in `leaf` has a strong
count of 1 and a weak count of 0 because the variable `leaf` is now the only
reference to the `Rc<Node>` again.
-->

Si nous essayons d'accéder au parent de `leaf` après la fin de la portée, nous obtiendrons à nouveau `None`. À la fin du programme, le `Rc<Node>` dans `leaf` a un compteur fort de 1 et un compteur faible de 0 car la variable `leaf` est à nouveau la seule référence vers le `Rc<Node>`.

<!--
All of the logic that manages the counts and value dropping is built into
`Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. By
specifying that the relationship from a child to its parent should be a
`Weak<T>` reference in the definition of `Node`, you're able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.
-->

Toute la logique qui gère les compteurs et la libération des valeurs est intégrée dans `Rc<T>` et `Weak<T>` et dans leurs implémentations du trait `Drop`. En spécifiant que la relation d'un enfant vers son parent devrait être une référence `Weak<T>` dans la définition de `Node`, vous pouvez avoir des noeuds parents pointant vers des noeuds enfants et vice versa sans créer de cycle de références ni de fuites de mémoire.

<!--
## Summary
-->

## Résumé

<!--
This chapter covered how to use smart pointers to make different guarantees and
trade-offs from those Rust makes by default with regular references. The
`Box<T>` type has a known size and points to data allocated on the heap. The
`Rc<T>` type keeps track of the number of references to data on the heap so
that the data can have multiple owners. The `RefCell<T>` type with its interior
mutability gives us a type that we can use when we need an immutable type but
need to change an inner value of that type; it also enforces the borrowing
rules at runtime instead of at compile time.
-->

Ce chapitre a couvert comment utiliser les pointeurs intelligents pour obtenir des garanties et des compromis différents de ceux que Rust offre par défaut avec les références classiques. Le type `Box<T>` a une taille connue et pointe vers des données allouées sur le tas. Le type `Rc<T>` suit le nombre de références vers des données sur le tas afin que les données puissent avoir plusieurs propriétaires. Le type `RefCell<T>` avec sa mutabilité intérieure nous donne un type que nous pouvons utiliser quand nous avons besoin d'un type immuable mais devons changer une valeur intérieure de ce type ; il applique aussi les règles d'emprunt à l'exécution plutôt qu'à la compilation.

<!--
Also discussed were the `Deref` and `Drop` traits, which enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks and how to prevent them using `Weak<T>`.
-->

Nous avons aussi discuté des traits `Deref` et `Drop`, qui permettent une grande partie de la fonctionnalité des pointeurs intelligents. Nous avons exploré les cycles de références qui peuvent provoquer des fuites de mémoire et comment les prévenir en utilisant `Weak<T>`.

<!--
If this chapter has piqued your interest and you want to implement your own
smart pointers, check out ["The Rustonomicon"][nomicon] for more useful
information.
-->

Si ce chapitre a piqué votre curiosité et que vous voulez implémenter vos propres pointeurs intelligents, consultez ["The Rustonomicon"][nomicon] pour plus d'informations utiles.

<!--
Next, we'll talk about concurrency in Rust. You'll even learn about a few new
smart pointers.
-->

Ensuite, nous parlerons de la concurrence en Rust. Vous apprendrez même quelques nouveaux pointeurs intelligents.

[nomicon]: ../nomicon/index.html
