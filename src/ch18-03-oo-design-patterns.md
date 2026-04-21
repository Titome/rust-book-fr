<!--
## Implementing an Object-Oriented Design Pattern
-->

## Implémenter un patron de conception orienté objet

<!--
The _state pattern_ is an object-oriented design pattern. The crux of the
pattern is that we define a set of states a value can have internally. The
states are represented by a set of _state objects_, and the value's behavior
changes based on its state. We're going to work through an example of a blog
post struct that has a field to hold its state, which will be a state object
from the set "draft," "review," or "published."
-->

Le _patron état_ est un patron de conception orienté objet. Le cœur du patron est que nous définissons un ensemble d'états qu'une valeur peut avoir en interne. Les états sont représentés par un ensemble d'_objets état_, et le comportement de la valeur change en fonction de son état. Nous allons travailler sur un exemple de struct d'article de blog qui a un champ pour contenir son état, qui sera un objet état de l'ensemble « brouillon », « en revue » ou « publié ».

<!--
The state objects share functionality: In Rust, of course, we use structs and
traits rather than objects and inheritance. Each state object is responsible
for its own behavior and for governing when it should change into another
state. The value that holds a state object knows nothing about the different
behavior of the states or when to transition between states.
-->

Les objets état partagent des fonctionnalités : en Rust, bien sûr, nous utilisons des structs et des traits plutôt que des objets et l'héritage. Chaque objet état est responsable de son propre comportement et de déterminer quand il doit changer vers un autre état. La valeur qui contient un objet état ne sait rien du comportement différent des états ni de quand effectuer la transition entre les états.

<!--
The advantage of using the state pattern is that, when the business
requirements of the program change, we won't need to change the code of the
value holding the state or the code that uses the value. We'll only need to
update the code inside one of the state objects to change its rules or perhaps
add more state objects.
-->

L'avantage d'utiliser le patron état est que, quand les exigences métier du programme changent, nous n'aurons pas besoin de modifier le code de la valeur contenant l'état ni le code qui utilise la valeur. Nous n'aurons qu'à mettre à jour le code à l'intérieur d'un des objets état pour changer ses règles, ou peut-être ajouter d'autres objets état.

<!--
First, we're going to implement the state pattern in a more traditional
object-oriented way. Then, we'll use an approach that's a bit more natural in
Rust. Let's dig in to incrementally implement a blog post workflow using the
state pattern.
-->

D'abord, nous allons implémenter le patron état de manière plus traditionnelle orientée objet. Ensuite, nous utiliserons une approche un peu plus naturelle en Rust. Plongeons-y pour implémenter progressivement un flux de travail d'articles de blog en utilisant le patron état.

<!--
The final functionality will look like this:
-->

La fonctionnalité finale ressemblera à ceci :

<!--
1. A blog post starts as an empty draft.
1. When the draft is done, a review of the post is requested.
1. When the post is approved, it gets published.
1. Only published blog posts return content to print so that unapproved posts
   can't accidentally be published.
-->

1. Un article de blog commence comme un brouillon vide.
1. Quand le brouillon est terminé, une revue de l'article est demandée.
1. Quand l'article est approuvé, il est publié.
1. Seuls les articles de blog publiés retournent du contenu à afficher pour que les articles non approuvés ne puissent pas être publiés accidentellement.

<!--
Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we've requested a review, the post
should remain an unpublished draft.
-->

Toute autre modification tentée sur un article ne devrait avoir aucun effet. Par exemple, si nous essayons d'approuver un brouillon d'article de blog avant d'avoir demandé une revue, l'article devrait rester un brouillon non publié.

<!--
Old headings. Do not remove or links may break.
-->

<a id="a-traditional-object-oriented-attempt"></a>

<!--
### Attempting Traditional Object-Oriented Style
-->

### Tenter le style orienté objet traditionnel

<!--
There are infinite ways to structure code to solve the same problem, each with
different trade-offs. This section's implementation is more of a traditional
object-oriented style, which is possible to write in Rust, but doesn't take
advantage of some of Rust's strengths. Later, we'll demonstrate a different
solution that still uses the object-oriented design pattern but is structured
in a way that might look less familiar to programmers with object-oriented
experience. We'll compare the two solutions to experience the trade-offs of
designing Rust code differently than code in other languages.
-->

Il existe une infinité de façons de structurer du code pour résoudre le même problème, chacune avec des compromis différents. L'implémentation de cette section est plus dans un style orienté objet traditionnel, qu'il est possible d'écrire en Rust, mais qui ne tire pas parti de certaines des forces de Rust. Plus tard, nous démontrerons une solution différente qui utilise toujours le patron de conception orienté objet mais qui est structurée d'une manière qui pourrait sembler moins familière aux programmeurs ayant de l'expérience en programmation orientée objet. Nous comparerons les deux solutions pour expérimenter les compromis de la conception de code Rust différemment du code dans d'autres langages.

<!--
Listing 18-11 shows this workflow in code form: This is an example usage of the
API we'll implement in a library crate named `blog`. This won't compile yet
because we haven't implemented the `blog` crate.
-->

L'encart 18-11 montre ce flux de travail sous forme de code : c'est un exemple d'utilisation de l'API que nous implémenterons dans un crate de bibliothèque nommé `blog`. Cela ne compilera pas encore car nous n'avons pas implémenté le crate `blog`.

<Listing number="18-11" file-name="src/main.rs" caption="Code qui démontre le comportement souhaité pour notre crate `blog`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

<!--
We want to allow the user to create a new draft blog post with `Post::new`. We
want to allow text to be added to the blog post. If we try to get the post's
content immediately, before approval, we shouldn't get any text because the
post is still a draft. We've added `assert_eq!` in the code for demonstration
purposes. An excellent unit test for this would be to assert that a draft blog
post returns an empty string from the `content` method, but we're not going to
write tests for this example.
-->

Nous voulons permettre à l'utilisateur de créer un nouveau brouillon d'article de blog avec `Post::new`. Nous voulons permettre d'ajouter du texte à l'article de blog. Si nous essayons d'obtenir le contenu de l'article immédiatement, avant l'approbation, nous ne devrions obtenir aucun texte car l'article est encore un brouillon. Nous avons ajouté `assert_eq!` dans le code à des fins de démonstration. Un excellent test unitaire pour cela serait de vérifier qu'un brouillon d'article de blog retourne une chaîne vide depuis la méthode `content`, mais nous n'allons pas écrire de tests pour cet exemple.

<!--
Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. When the post
receives approval, it should get published, meaning the text of the post will
be returned when `content` is called.
-->

Ensuite, nous voulons permettre de demander une revue de l'article, et nous voulons que `content` retourne une chaîne vide en attendant la revue. Quand l'article reçoit l'approbation, il devrait être publié, ce qui signifie que le texte de l'article sera retourné quand `content` est appelé.

<!--
Notice that the only type we're interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in—draft, review, or published. Changing from one state to another will be
managed internally within the `Post` type. The states change in response to the
methods called by our library's users on the `Post` instance, but they don't
have to manage the state changes directly. Also, users can't make a mistake
with the states, such as publishing a post before it's reviewed.
-->

Remarquez que le seul type avec lequel nous interagissons depuis le crate est le type `Post`. Ce type utilisera le patron état et contiendra une valeur qui sera l'un des trois objets état représentant les différents états dans lesquels un article peut se trouver — brouillon, revue ou publié. Le passage d'un état à un autre sera géré en interne au sein du type `Post`. Les états changent en réponse aux méthodes appelées par les utilisateurs de notre bibliothèque sur l'instance `Post`, mais ils n'ont pas à gérer directement les changements d'état. De plus, les utilisateurs ne peuvent pas faire d'erreur avec les états, comme publier un article avant qu'il ne soit révisé.

<!--
Old headings. Do not remove or links may break.
-->

<a id="defining-post-and-creating-a-new-instance-in-the-draft-state"></a>

<!--
#### Defining `Post` and Creating a New Instance
-->

#### Définir `Post` et créer une nouvelle instance

<!--
Let's get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so we'll start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 18-12. We'll also make a private
`State` trait that will define the behavior that all state objects for a `Post`
must have.
-->

Commençons l'implémentation de la bibliothèque ! Nous savons que nous avons besoin d'une struct publique `Post` qui contient du contenu, donc nous commencerons par la définition de la struct et une fonction publique associée `new` pour créer une instance de `Post`, comme montré dans l'encart 18-12. Nous créerons aussi un trait privé `State` qui définira le comportement que tous les objets état d'un `Post` doivent avoir.

<!--
Then, `Post` will hold a trait object of `Box<dyn State>` inside an `Option<T>`
in a private field named `state` to hold the state object. You'll see why the
`Option<T>` is necessary in a bit.
-->

Ensuite, `Post` contiendra un objet trait de `Box<dyn State>` à l'intérieur d'un `Option<T>` dans un champ privé nommé `state` pour contenir l'objet état. Vous verrez bientôt pourquoi l'`Option<T>` est nécessaire.

<Listing number="18-12" file-name="src/lib.rs" caption="Définition d'une struct `Post` et d'une fonction `new` qui crée une nouvelle instance de `Post`, d'un trait `State` et d'une struct `Draft`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

<!--
The `State` trait defines the behavior shared by different post states. The
state objects are `Draft`, `PendingReview`, and `Published`, and they will all
implement the `State` trait. For now, the trait doesn't have any methods, and
we'll start by defining just the `Draft` state because that is the state we
want a post to start in.
-->

Le trait `State` définit le comportement partagé par les différents états d'article. Les objets état sont `Draft`, `PendingReview` et `Published`, et ils implémenteront tous le trait `State`. Pour l'instant, le trait n'a aucune méthode, et nous commencerons par définir uniquement l'état `Draft` car c'est l'état dans lequel nous voulons qu'un article démarre.

<!--
When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures that whenever we create a new instance of `Post`, it will start out as
a draft. Because the `state` field of `Post` is private, there is no way to
create a `Post` in any other state! In the `Post::new` function, we set the
`content` field to a new, empty `String`.
-->

Quand nous créons un nouveau `Post`, nous définissons son champ `state` à une valeur `Some` qui contient un `Box`. Ce `Box` pointe vers une nouvelle instance de la struct `Draft`. Cela garantit que chaque fois que nous créons une nouvelle instance de `Post`, elle commencera en tant que brouillon. Comme le champ `state` de `Post` est privé, il n'y a aucun moyen de créer un `Post` dans un autre état ! Dans la fonction `Post::new`, nous définissons le champ `content` à une nouvelle `String` vide.

<!--
#### Storing the Text of the Post Content
-->

#### Stocker le texte du contenu de l'article

<!--
We saw in Listing 18-11 that we want to be able to call a method named
`add_text` and pass it a `&str` that is then added as the text content of the
blog post. We implement this as a method, rather than exposing the `content`
field as `pub`, so that later we can implement a method that will control how
the `content` field's data is read. The `add_text` method is pretty
straightforward, so let's add the implementation in Listing 18-13 to the `impl
Post` block.
-->

Nous avons vu dans l'encart 18-11 que nous voulons pouvoir appeler une méthode nommée `add_text` et lui passer un `&str` qui sera ensuite ajouté comme contenu texte de l'article de blog. Nous implémentons cela comme une méthode, plutôt que d'exposer le champ `content` comme `pub`, pour que plus tard nous puissions implémenter une méthode qui contrôlera comment les données du champ `content` sont lues. La méthode `add_text` est assez simple, alors ajoutons l'implémentation dans l'encart 18-13 au bloc `impl Post`.

<Listing number="18-13" file-name="src/lib.rs" caption="Implémentation de la méthode `add_text` pour ajouter du texte au `content` d'un article">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

<!--
The `add_text` method takes a mutable reference to `self` because we're
changing the `Post` instance that we're calling `add_text` on. We then call
`push_str` on the `String` in `content` and pass the `text` argument to add to
the saved `content`. This behavior doesn't depend on the state the post is in,
so it's not part of the state pattern. The `add_text` method doesn't interact
with the `state` field at all, but it is part of the behavior we want to
support.
-->

La méthode `add_text` prend une référence mutable vers `self` car nous modifions l'instance `Post` sur laquelle nous appelons `add_text`. Nous appelons ensuite `push_str` sur la `String` dans `content` et passons l'argument `text` pour l'ajouter au `content` sauvegardé. Ce comportement ne dépend pas de l'état dans lequel se trouve l'article, donc il ne fait pas partie du patron état. La méthode `add_text` n'interagit pas du tout avec le champ `state`, mais elle fait partie du comportement que nous voulons supporter.

<!--
Old headings. Do not remove or links may break.
-->

<a id="ensuring-the-content-of-a-draft-post-is-empty"></a>

<!--
#### Ensuring That the Content of a Draft Post Is Empty
-->

#### S'assurer que le contenu d'un brouillon d'article est vide

<!--
Even after we've called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice because the post is
still in the draft state, as shown by the first `assert_eq!` in Listing 18-11.
For now, let's implement the `content` method with the simplest thing that will
fulfill this requirement: always returning an empty string slice. We'll change
this later once we implement the ability to change a post's state so that it
can be published. So far, posts can only be in the draft state, so the post
content should always be empty. Listing 18-14 shows this placeholder
implementation.
-->

Même après avoir appelé `add_text` et ajouté du contenu à notre article, nous voulons toujours que la méthode `content` retourne une tranche de chaîne vide car l'article est encore dans l'état brouillon, comme montré par le premier `assert_eq!` dans l'encart 18-11. Pour l'instant, implémentons la méthode `content` avec la chose la plus simple qui satisfera cette exigence : toujours retourner une tranche de chaîne vide. Nous changerons cela plus tard quand nous implémenterons la capacité de changer l'état d'un article pour qu'il puisse être publié. Jusqu'ici, les articles ne peuvent être que dans l'état brouillon, donc le contenu de l'article devrait toujours être vide. L'encart 18-14 montre cette implémentation provisoire.

<Listing number="18-14" file-name="src/lib.rs" caption="Ajouter une implémentation provisoire pour la méthode `content` de `Post` qui retourne toujours une tranche de chaîne vide">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

<!--
With this added `content` method, everything in Listing 18-11 through the first
`assert_eq!` works as intended.
-->

Avec cette méthode `content` ajoutée, tout dans l'encart 18-11 jusqu'au premier `assert_eq!` fonctionne comme prévu.

<!--
Old headings. Do not remove or links may break.
-->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>
<a id="requesting-a-review-changes-the-posts-state"></a>

<!--
#### Requesting a Review, Which Changes the Post's State
-->

#### Demander une revue, ce qui change l'état de l'article

<!--
Next, we need to add functionality to request a review of a post, which should
change its state from `Draft` to `PendingReview`. Listing 18-15 shows this code.
-->

Ensuite, nous devons ajouter la fonctionnalité pour demander une revue d'un article, ce qui devrait changer son état de `Draft` à `PendingReview`. L'encart 18-15 montre ce code.

<Listing number="18-15" file-name="src/lib.rs" caption="Implémentation des méthodes `request_review` sur `Post` et le trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

<!--
We give `Post` a public method named `request_review` that will take a mutable
reference to `self`. Then, we call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method consumes the
current state and returns a new state.
-->

Nous donnons à `Post` une méthode publique nommée `request_review` qui prendra une référence mutable vers `self`. Ensuite, nous appelons une méthode interne `request_review` sur l'état actuel de `Post`, et cette seconde méthode `request_review` consomme l'état actuel et retourne un nouvel état.

<!--
We add the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, invalidating the old state so that the state value of
the `Post` can transform into a new state.
-->

Nous ajoutons la méthode `request_review` au trait `State` ; tous les types qui implémentent le trait devront maintenant implémenter la méthode `request_review`. Notez que plutôt que d'avoir `self`, `&self` ou `&mut self` comme premier paramètre de la méthode, nous avons `self: Box<Self>`. Cette syntaxe signifie que la méthode n'est valide que lorsqu'elle est appelée sur un `Box` contenant le type. Cette syntaxe prend possession de `Box<Self>`, invalidant l'ancien état pour que la valeur d'état du `Post` puisse se transformer en un nouvel état.

<!--
To consume the old state, the `request_review` method needs to take ownership
of the state value. This is where the `Option` in the `state` field of `Post`
comes in: We call the `take` method to take the `Some` value out of the `state`
field and leave a `None` in its place because Rust doesn't let us have
unpopulated fields in structs. This lets us move the `state` value out of
`Post` rather than borrowing it. Then, we'll set the post's `state` value to
the result of this operation.
-->

Pour consommer l'ancien état, la méthode `request_review` doit prendre possession de la valeur d'état. C'est là que l'`Option` dans le champ `state` de `Post` entre en jeu : nous appelons la méthode `take` pour extraire la valeur `Some` du champ `state` et laisser un `None` à sa place car Rust ne nous laisse pas avoir des champs non remplis dans les structs. Cela nous permet de déplacer la valeur `state` hors de `Post` plutôt que de l'emprunter. Ensuite, nous définirons la valeur `state` de l'article au résultat de cette opération.

<!--
We need to set `state` to `None` temporarily rather than setting it directly
with code like `self.state = self.state.request_review();` to get ownership of
the `state` value. This ensures that `Post` can't use the old `state` value
after we've transformed it into a new state.
-->

Nous devons mettre `state` à `None` temporairement plutôt que de le définir directement avec du code comme `self.state = self.state.request_review();` pour obtenir la possession de la valeur `state`. Cela garantit que `Post` ne peut pas utiliser l'ancienne valeur `state` après que nous l'ayons transformée en un nouvel état.

<!--
The `request_review` method on `Draft` returns a new, boxed instance of a new
`PendingReview` struct, which represents the state when a post is waiting for a
review. The `PendingReview` struct also implements the `request_review` method
but doesn't do any transformations. Rather, it returns itself because when we
request a review on a post already in the `PendingReview` state, it should stay
in the `PendingReview` state.
-->

La méthode `request_review` sur `Draft` retourne une nouvelle instance boxée d'une nouvelle struct `PendingReview`, qui représente l'état quand un article attend une revue. La struct `PendingReview` implémente aussi la méthode `request_review` mais ne fait aucune transformation. Elle se retourne elle-même car quand nous demandons une revue sur un article déjà dans l'état `PendingReview`, il devrait rester dans l'état `PendingReview`.

<!--
Now we can start seeing the advantages of the state pattern: The
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.
-->

Nous pouvons maintenant commencer à voir les avantages du patron état : la méthode `request_review` sur `Post` est la même quel que soit sa valeur `state`. Chaque état est responsable de ses propres règles.

<!--
We'll leave the `content` method on `Post` as is, returning an empty string
slice. We can now have a `Post` in the `PendingReview` state as well as in the
`Draft` state, but we want the same behavior in the `PendingReview` state.
Listing 18-11 now works up to the second `assert_eq!` call!
-->

Nous laisserons la méthode `content` de `Post` telle quelle, retournant une tranche de chaîne vide. Nous pouvons maintenant avoir un `Post` dans l'état `PendingReview` aussi bien que dans l'état `Draft`, mais nous voulons le même comportement dans l'état `PendingReview`. L'encart 18-11 fonctionne maintenant jusqu'au deuxième appel `assert_eq!` !

<!--
Old headings. Do not remove or links may break.
-->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>
<a id="adding-approve-to-change-the-behavior-of-content"></a>

<!--
#### Adding `approve` to Change `content`'s Behavior
-->

#### Ajouter `approve` pour changer le comportement de `content`

<!--
The `approve` method will be similar to the `request_review` method: It will
set `state` to the value that the current state says it should have when that
state is approved, as shown in Listing 18-16.
-->

La méthode `approve` sera similaire à la méthode `request_review` : elle définira `state` à la valeur que l'état actuel dit qu'il devrait avoir quand cet état est approuvé, comme montré dans l'encart 18-16.

<Listing number="18-16" file-name="src/lib.rs" caption="Implémentation de la méthode `approve` sur `Post` et le trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

<!--
We add the `approve` method to the `State` trait and add a new struct that
implements `State`, the `Published` state.
-->

Nous ajoutons la méthode `approve` au trait `State` et ajoutons une nouvelle struct qui implémente `State`, l'état `Published`.

<!--
Similar to the way `request_review` on `PendingReview` works, if we call the
`approve` method on a `Draft`, it will have no effect because `approve` will
return `self`. When we call `approve` on `PendingReview`, it returns a new,
boxed instance of the `Published` struct. The `Published` struct implements the
`State` trait, and for both the `request_review` method and the `approve`
method, it returns itself because the post should stay in the `Published` state
in those cases.
-->

De la même manière que `request_review` fonctionne sur `PendingReview`, si nous appelons la méthode `approve` sur un `Draft`, cela n'aura aucun effet car `approve` retournera `self`. Quand nous appelons `approve` sur `PendingReview`, il retourne une nouvelle instance boxée de la struct `Published`. La struct `Published` implémente le trait `State`, et pour les méthodes `request_review` et `approve`, elle se retourne elle-même car l'article devrait rester dans l'état `Published` dans ces cas.

<!--
Now we need to update the `content` method on `Post`. We want the value
returned from `content` to depend on the current state of the `Post`, so we're
going to have the `Post` delegate to a `content` method defined on its `state`,
as shown in Listing 18-17.
-->

Maintenant, nous devons mettre à jour la méthode `content` de `Post`. Nous voulons que la valeur retournée par `content` dépende de l'état actuel du `Post`, donc nous allons faire en sorte que `Post` délègue à une méthode `content` définie sur son `state`, comme montré dans l'encart 18-17.

<Listing number="18-17" file-name="src/lib.rs" caption="Mise à jour de la méthode `content` de `Post` pour déléguer à une méthode `content` de `State`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

<!--
Because the goal is to keep all of these rules inside the structs that
implement `State`, we call a `content` method on the value in `state` and pass
the post instance (that is, `self`) as an argument. Then, we return the value
that's returned from using the `content` method on the `state` value.
-->

Comme l'objectif est de garder toutes ces règles à l'intérieur des structs qui implémentent `State`, nous appelons une méthode `content` sur la valeur dans `state` et passons l'instance de l'article (c'est-à-dire `self`) comme argument. Ensuite, nous retournons la valeur retournée par l'utilisation de la méthode `content` sur la valeur `state`.

<!--
We call the `as_ref` method on the `Option` because we want a reference to the
value inside the `Option` rather than ownership of the value. Because `state` is
an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn
State>>` is returned. If we didn't call `as_ref`, we would get an error because
we can't move `state` out of the borrowed `&self` of the function parameter.
-->

Nous appelons la méthode `as_ref` sur l'`Option` car nous voulons une référence vers la valeur à l'intérieur de l'`Option` plutôt que la possession de la valeur. Comme `state` est un `Option<Box<dyn State>>`, quand nous appelons `as_ref`, un `Option<&Box<dyn State>>` est retourné. Si nous n'appelions pas `as_ref`, nous obtiendrions une erreur car nous ne pouvons pas déplacer `state` hors du `&self` emprunté du paramètre de la fonction.

<!--
We then call the `unwrap` method, which we know will never panic because we
know the methods on `Post` ensure that `state` will always contain a `Some`
value when those methods are done. This is one of the cases we talked about in
the ["When You Have More Information Than the
Compiler"][more-info-than-rustc] ignore
--> section of Chapter 9 when we
know that a `None` value is never possible, even though the compiler isn't able
to understand that.
-->

Nous appelons ensuite la méthode `unwrap`, dont nous savons qu'elle ne paniquera jamais car nous savons que les méthodes de `Post` garantissent que `state` contiendra toujours une valeur `Some` quand ces méthodes sont terminées. C'est l'un des cas dont nous avons parlé dans la section [« Quand vous avez plus d'informations que le compilateur »][more-info-than-rustc]<!--
ignore
--> du chapitre 9, quand nous savons qu'une valeur `None` n'est jamais possible, même si le compilateur n'est pas en mesure de le comprendre.

<!--
At this point, when we call `content` on the `&Box<dyn State>`, deref coercion
will take effect on the `&` and the `Box` so that the `content` method will
ultimately be called on the type that implements the `State` trait. That means
we need to add `content` to the `State` trait definition, and that is where
we'll put the logic for what content to return depending on which state we
have, as shown in Listing 18-18.
-->

À ce stade, quand nous appelons `content` sur le `&Box<dyn State>`, la coercition de déréférencement prendra effet sur le `&` et le `Box` pour que la méthode `content` soit finalement appelée sur le type qui implémente le trait `State`. Cela signifie que nous devons ajouter `content` à la définition du trait `State`, et c'est là que nous mettrons la logique pour déterminer quel contenu retourner en fonction de l'état dans lequel nous sommes, comme montré dans l'encart 18-18.

<Listing number="18-18" file-name="src/lib.rs" caption="Ajouter la méthode `content` au trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

<!--
We add a default implementation for the `content` method that returns an empty
string slice. That means we don't need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`. While convenient, having the
`content` method on `State` determine the content of the `Post` is blurring
the lines between the responsibility of `State` and the responsibility of
`Post`.
-->

Nous ajoutons une implémentation par défaut pour la méthode `content` qui retourne une tranche de chaîne vide. Cela signifie que nous n'avons pas besoin d'implémenter `content` sur les structs `Draft` et `PendingReview`. La struct `Published` surchargera la méthode `content` et retournera la valeur dans `post.content`. Bien que pratique, avoir la méthode `content` sur `State` qui détermine le contenu du `Post` brouille les frontières entre la responsabilité de `State` et la responsabilité de `Post`.

<!--
Note that we need lifetime annotations on this method, as we discussed in
Chapter 10. We're taking a reference to a `post` as an argument and returning a
reference to part of that `post`, so the lifetime of the returned reference is
related to the lifetime of the `post` argument.
-->

Notez que nous avons besoin d'annotations de durée de vie sur cette méthode, comme nous l'avons discuté au chapitre 10. Nous prenons une référence vers un `post` comme argument et retournons une référence vers une partie de ce `post`, donc la durée de vie de la référence retournée est liée à la durée de vie de l'argument `post`.

<!--
And we're done—all of Listing 18-11 now works! We've implemented the state
pattern with the rules of the blog post workflow. The logic related to the
rules lives in the state objects rather than being scattered throughout `Post`.
-->

Et c'est terminé — tout l'encart 18-11 fonctionne maintenant ! Nous avons implémenté le patron état avec les règles du flux de travail des articles de blog. La logique liée aux règles réside dans les objets état plutôt que d'être dispersée dans `Post`.

<!--
> ### Why Not An Enum?
>
> You may have been wondering why we didn't use an enum with the different
> possible post states as variants. That's certainly a possible solution; try it
> and compare the end results to see which you prefer! One disadvantage of using
> an enum is that every place that checks the value of the enum will need a
> `match` expression or similar to handle every possible variant. This could get
> more repetitive than this trait object solution.
-->

> ### Pourquoi pas un enum ?
>
> Vous vous êtes peut-être demandé pourquoi nous n'avons pas utilisé un enum avec les différents états possibles d'un article comme variantes. C'est certainement une solution possible ; essayez-la et comparez les résultats finaux pour voir laquelle vous préférez ! Un inconvénient de l'utilisation d'un enum est que chaque endroit qui vérifie la valeur de l'enum aura besoin d'une expression `match` ou similaire pour gérer chaque variante possible. Cela pourrait devenir plus répétitif que cette solution avec des objets trait.

<!--
Old headings. Do not remove or links may break.
-->

<a id="trade-offs-of-the-state-pattern"></a>

<!--
#### Evaluating the State Pattern
-->

#### Évaluer le patron état

<!--
We've shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the various behaviors.
Because of the way we organized the code, we have to look in only one place to
know the different ways a published post can behave: the implementation of the
`State` trait on the `Published` struct.
-->

Nous avons montré que Rust est capable d'implémenter le patron état orienté objet pour encapsuler les différents types de comportement qu'un article devrait avoir dans chaque état. Les méthodes de `Post` ne savent rien des différents comportements. Grâce à la façon dont nous avons organisé le code, nous n'avons qu'un seul endroit à regarder pour connaître les différentes façons dont un article publié peut se comporter : l'implémentation du trait `State` sur la struct `Published`.

<!--
If we were to create an alternative implementation that didn't use the state
pattern, we might instead use `match` expressions in the methods on `Post` or
even in the `main` code that checks the state of the post and changes behavior
in those places. That would mean we would have to look in several places to
understand all the implications of a post being in the published state.
-->

Si nous devions créer une implémentation alternative qui n'utilise pas le patron état, nous pourrions plutôt utiliser des expressions `match` dans les méthodes de `Post` ou même dans le code `main` qui vérifie l'état de l'article et change le comportement à ces endroits. Cela signifierait que nous devrions regarder à plusieurs endroits pour comprendre toutes les implications d'un article dans l'état publié.

<!--
With the state pattern, the `Post` methods and the places we use `Post` don't
need `match` expressions, and to add a new state, we would only need to add a
new struct and implement the trait methods on that one struct in one location.
-->

Avec le patron état, les méthodes de `Post` et les endroits où nous utilisons `Post` n'ont pas besoin d'expressions `match`, et pour ajouter un nouvel état, nous n'aurions qu'à ajouter une nouvelle struct et implémenter les méthodes du trait sur cette struct à un seul endroit.

<!--
The implementation using the state pattern is easy to extend to add more
functionality. To see the simplicity of maintaining code that uses the state
pattern, try a few of these suggestions:
-->

L'implémentation utilisant le patron état est facile à étendre pour ajouter plus de fonctionnalités. Pour voir la simplicité de la maintenance du code qui utilise le patron état, essayez quelques-unes de ces suggestions :

<!--
- Add a `reject` method that changes the post's state from `PendingReview` back
  to `Draft`.
- Require two calls to `approve` before the state can be changed to `Published`.
- Allow users to add text content only when a post is in the `Draft` state.
  Hint: have the state object responsible for what might change about the
  content but not responsible for modifying the `Post`.
-->

- Ajouter une méthode `reject` qui change l'état de l'article de `PendingReview` à `Draft`.
- Exiger deux appels à `approve` avant que l'état puisse être changé en `Published`.
- Permettre aux utilisateurs d'ajouter du contenu texte uniquement quand un article est dans l'état `Draft`. Indice : faites en sorte que l'objet état soit responsable de ce qui pourrait changer dans le contenu mais pas de la modification du `Post`.

<!--
One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` didn't need to
change with the addition of a new state, but that would mean switching to
another design pattern.
-->

Un inconvénient du patron état est que, comme les états implémentent les transitions entre états, certains des états sont couplés les uns aux autres. Si nous ajoutons un autre état entre `PendingReview` et `Published`, comme `Scheduled`, nous devrions changer le code dans `PendingReview` pour effectuer la transition vers `Scheduled` à la place. Ce serait moins de travail si `PendingReview` n'avait pas besoin de changer avec l'ajout d'un nouvel état, mais cela signifierait passer à un autre patron de conception.

<!--
Another downside is that we've duplicated some logic. To eliminate some of the
duplication, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`.
However, this wouldn't work: When using `State` as a trait object, the trait
doesn't know what the concrete `self` will be exactly, so the return type isn't
known at compile time. (This is one of the dyn compatibility rules mentioned
earlier.)
-->

Un autre inconvénient est que nous avons dupliqué une partie de la logique. Pour éliminer une partie de la duplication, nous pourrions essayer de créer des implémentations par défaut pour les méthodes `request_review` et `approve` du trait `State` qui retournent `self`. Cependant, cela ne fonctionnerait pas : en utilisant `State` comme objet trait, le trait ne sait pas ce que sera exactement le `self` concret, donc le type de retour n'est pas connu au moment de la compilation. (C'est l'une des règles de compatibilité dyn mentionnées plus tôt.)

<!--
Other duplication includes the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods use `Option::take` with the
`state` field of `Post`, and if `state` is `Some`, they delegate to the wrapped
value's implementation of the same method and set the new value of the `state`
field to the result. If we had a lot of methods on `Post` that followed this
pattern, we might consider defining a macro to eliminate the repetition (see
the ["Macros"][macros] ignore
--> section in Chapter 20).
-->

D'autres duplications incluent les implémentations similaires des méthodes `request_review` et `approve` de `Post`. Les deux méthodes utilisent `Option::take` avec le champ `state` de `Post`, et si `state` est `Some`, elles délèguent à l'implémentation de la même méthode de la valeur encapsulée et définissent la nouvelle valeur du champ `state` au résultat. Si nous avions beaucoup de méthodes sur `Post` qui suivaient ce motif, nous pourrions envisager de définir une macro pour éliminer la répétition (voir la section [« Macros »][macros]<!--
ignore
--> au chapitre 20).

<!--
By implementing the state pattern exactly as it's defined for object-oriented
languages, we're not taking as full advantage of Rust's strengths as we could.
Let's look at some changes we can make to the `blog` crate that can make
invalid states and transitions into compile-time errors.
-->

En implémentant le patron état exactement comme il est défini pour les langages orientés objet, nous ne tirons pas pleinement parti des forces de Rust comme nous le pourrions. Voyons quelques changements que nous pouvons apporter au crate `blog` pour transformer les états et transitions invalides en erreurs de compilation.

<!--
### Encoding States and Behavior as Types
-->

### Encoder les états et le comportement comme des types

<!--
We'll show you how to rethink the state pattern to get a different set of
trade-offs. Rather than encapsulating the states and transitions completely so
that outside code has no knowledge of them, we'll encode the states into
different types. Consequently, Rust's type-checking system will prevent
attempts to use draft posts where only published posts are allowed by issuing a
compiler error.
-->

Nous allons vous montrer comment repenser le patron état pour obtenir un ensemble différent de compromis. Plutôt que d'encapsuler complètement les états et les transitions pour que le code extérieur n'en ait aucune connaissance, nous encoderons les états dans différents types. Par conséquent, le système de vérification de types de Rust empêchera les tentatives d'utiliser des brouillons d'articles là où seuls les articles publiés sont autorisés en émettant une erreur de compilation.

<!--
Let's consider the first part of `main` in Listing 18-11:
-->

Considérons la première partie de `main` dans l'encart 18-11 :

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

<!--
We still enable the creation of new posts in the draft state using `Post::new`
and the ability to add text to the post's content. But instead of having a
`content` method on a draft post that returns an empty string, we'll make it so
that draft posts don't have the `content` method at all. That way, if we try to
get a draft post's content, we'll get a compiler error telling us the method
doesn't exist. As a result, it will be impossible for us to accidentally
display draft post content in production because that code won't even compile.
Listing 18-19 shows the definition of a `Post` struct and a `DraftPost` struct,
as well as methods on each.
-->

Nous permettons toujours la création de nouveaux articles dans l'état brouillon en utilisant `Post::new` et la capacité d'ajouter du texte au contenu de l'article. Mais au lieu d'avoir une méthode `content` sur un brouillon d'article qui retourne une chaîne vide, nous ferons en sorte que les brouillons d'articles n'aient pas du tout la méthode `content`. De cette façon, si nous essayons d'obtenir le contenu d'un brouillon d'article, nous obtiendrons une erreur du compilateur nous disant que la méthode n'existe pas. En conséquence, il sera impossible pour nous d'afficher accidentellement le contenu d'un brouillon d'article en production car ce code ne compilera même pas. L'encart 18-19 montre la définition d'une struct `Post` et d'une struct `DraftPost`, ainsi que les méthodes sur chacune.

<Listing number="18-19" file-name="src/lib.rs" caption="Un `Post` avec une méthode `content` et un `DraftPost` sans méthode `content`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

<!--
Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field because
we're moving the encoding of the state to the types of the structs. The `Post`
struct will represent a published post, and it has a `content` method that
returns the `content`.
-->

Les structs `Post` et `DraftPost` ont toutes deux un champ privé `content` qui stocke le texte de l'article de blog. Les structs n'ont plus le champ `state` car nous déplaçons l'encodage de l'état vers les types des structs. La struct `Post` représentera un article publié, et elle a une méthode `content` qui retourne le `content`.

<!--
We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. Because `content` is private and
there aren't any functions that return `Post`, it's not possible to create an
instance of `Post` right now.
-->

Nous avons toujours une fonction `Post::new`, mais au lieu de retourner une instance de `Post`, elle retourne une instance de `DraftPost`. Comme `content` est privé et qu'il n'y a pas de fonctions qui retournent `Post`, il n'est pas possible de créer une instance de `Post` pour l'instant.

<!--
The `DraftPost` struct has an `add_text` method, so we can add text to
`content` as before, but note that `DraftPost` does not have a `content` method
defined! So now the program ensures that all posts start as draft posts, and
draft posts don't have their content available for display. Any attempt to get
around these constraints will result in a compiler error.
-->

La struct `DraftPost` a une méthode `add_text`, donc nous pouvons ajouter du texte à `content` comme avant, mais notez que `DraftPost` n'a pas de méthode `content` définie ! Donc maintenant le programme garantit que tous les articles commencent comme des brouillons, et les brouillons n'ont pas leur contenu disponible pour l'affichage. Toute tentative de contourner ces contraintes résultera en une erreur du compilateur.

<!--
Old headings. Do not remove or links may break.
-->

<a id="implementing-transitions-as-transformations-into-different-types"></a>

<!--
So, how do we get a published post? We want to enforce the rule that a draft
post has to be reviewed and approved before it can be published. A post in the
pending review state should still not display any content. Let's implement
these constraints by adding another struct, `PendingReviewPost`, defining the
`request_review` method on `DraftPost` to return a `PendingReviewPost` and
defining an `approve` method on `PendingReviewPost` to return a `Post`, as
shown in Listing 18-20.
-->

Alors, comment obtenir un article publié ? Nous voulons imposer la règle qu'un brouillon d'article doit être révisé et approuvé avant de pouvoir être publié. Un article en état de revue en attente ne devrait toujours pas afficher de contenu. Implémentons ces contraintes en ajoutant une autre struct, `PendingReviewPost`, en définissant la méthode `request_review` sur `DraftPost` pour retourner un `PendingReviewPost` et en définissant une méthode `approve` sur `PendingReviewPost` pour retourner un `Post`, comme montré dans l'encart 18-20.

<Listing number="18-20" file-name="src/lib.rs" caption="Un `PendingReviewPost` qui est créé en appelant `request_review` sur `DraftPost` et une méthode `approve` qui transforme un `PendingReviewPost` en un `Post` publié">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

<!--
The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won't have any lingering `DraftPost` instances after we've called
`request_review` on them, and so forth. The `PendingReviewPost` struct doesn't
have a `content` method defined on it, so attempting to read its content
results in a compiler error, as with `DraftPost`. Because the only way to get a
published `Post` instance that does have a `content` method defined is to call
the `approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we've now encoded the blog post workflow into the type system.
-->

Les méthodes `request_review` et `approve` prennent possession de `self`, consommant ainsi les instances `DraftPost` et `PendingReviewPost` et les transformant en un `PendingReviewPost` et un `Post` publié, respectivement. De cette façon, nous n'aurons pas d'instances `DraftPost` persistantes après avoir appelé `request_review` sur elles, et ainsi de suite. La struct `PendingReviewPost` n'a pas de méthode `content` définie, donc tenter de lire son contenu résulte en une erreur du compilateur, comme avec `DraftPost`. Comme le seul moyen d'obtenir une instance de `Post` publiée qui a une méthode `content` définie est d'appeler la méthode `approve` sur un `PendingReviewPost`, et le seul moyen d'obtenir un `PendingReviewPost` est d'appeler la méthode `request_review` sur un `DraftPost`, nous avons maintenant encodé le flux de travail des articles de blog dans le système de types.

<!--
But we also have to make some small changes to `main`. The `request_review` and
`approve` methods return new instances rather than modifying the struct they're
called on, so we need to add more `let post =` shadowing assignments to save
the returned instances. We also can't have the assertions about the draft and
pending review posts' contents be empty strings, nor do we need them: We can't
compile code that tries to use the content of posts in those states any longer.
The updated code in `main` is shown in Listing 18-21.
-->

Mais nous devons aussi apporter quelques petites modifications à `main`. Les méthodes `request_review` et `approve` retournent de nouvelles instances plutôt que de modifier la struct sur laquelle elles sont appelées, donc nous devons ajouter plus d'assignations par masquage `let post =` pour sauvegarder les instances retournées. Nous ne pouvons pas non plus avoir les assertions sur le contenu des brouillons et des articles en revue en attente comme des chaînes vides, et nous n'en avons pas besoin : nous ne pouvons plus compiler du code qui essaie d'utiliser le contenu des articles dans ces états. Le code mis à jour dans `main` est montré dans l'encart 18-21.

<Listing number="18-21" file-name="src/main.rs" caption="Modifications de `main` pour utiliser la nouvelle implémentation du flux de travail des articles de blog">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

<!--
The changes we needed to make to `main` to reassign `post` mean that this
implementation doesn't quite follow the object-oriented state pattern anymore:
The transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as display of the content of
an unpublished post, will be discovered before they make it to production.
-->

Les changements que nous avons dû apporter à `main` pour réassigner `post` signifient que cette implémentation ne suit plus tout à fait le patron état orienté objet : les transformations entre les états ne sont plus entièrement encapsulées dans l'implémentation de `Post`. Cependant, notre gain est que les états invalides sont maintenant impossibles grâce au système de types et à la vérification de types qui se fait au moment de la compilation ! Cela garantit que certains bugs, comme l'affichage du contenu d'un article non publié, seront découverts avant qu'ils n'arrivent en production.

<!--
Try the tasks suggested at the start of this section on the `blog` crate as it
is after Listing 18-21 to see what you think about the design of this version
of the code. Note that some of the tasks might be completed already in this
design.
-->

Essayez les tâches suggérées au début de cette section sur le crate `blog` tel qu'il est après l'encart 18-21 pour voir ce que vous pensez de la conception de cette version du code. Notez que certaines des tâches pourraient déjà être complétées dans cette conception.

<!--
We've seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns, such as encoding state into the type system,
are also available in Rust. These patterns have different trade-offs. Although
you might be very familiar with object-oriented patterns, rethinking the
problem to take advantage of Rust's features can provide benefits, such as
preventing some bugs at compile time. Object-oriented patterns won't always be
the best solution in Rust due to certain features, like ownership, that
object-oriented languages don't have.
-->

Nous avons vu que même si Rust est capable d'implémenter des patrons de conception orientés objet, d'autres patrons, comme encoder l'état dans le système de types, sont aussi disponibles en Rust. Ces patrons ont des compromis différents. Bien que vous puissiez être très familier avec les patrons orientés objet, repenser le problème pour tirer parti des fonctionnalités de Rust peut apporter des avantages, comme prévenir certains bugs au moment de la compilation. Les patrons orientés objet ne seront pas toujours la meilleure solution en Rust en raison de certaines fonctionnalités, comme la possession, que les langages orientés objet n'ont pas.

<!--
## Summary
-->

## Résumé

<!--
Regardless of whether you think Rust is an object-oriented language after
reading this chapter, you now know that you can use trait objects to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. You can use this
flexibility to implement object-oriented patterns that can help your code's
maintainability. Rust also has other features, like ownership, that
object-oriented languages don't have. An object-oriented pattern won't always
be the best way to take advantage of Rust's strengths, but it is an available
option.
-->

Que vous pensiez ou non que Rust est un langage orienté objet après avoir lu ce chapitre, vous savez maintenant que vous pouvez utiliser des objets trait pour obtenir certaines fonctionnalités orientées objet en Rust. La répartition dynamique peut donner à votre code une certaine flexibilité en échange d'un peu de performance à l'exécution. Vous pouvez utiliser cette flexibilité pour implémenter des patrons orientés objet qui peuvent aider la maintenabilité de votre code. Rust a aussi d'autres fonctionnalités, comme la possession, que les langages orientés objet n'ont pas. Un patron orienté objet ne sera pas toujours la meilleure façon de tirer parti des forces de Rust, mais c'est une option disponible.

<!--
Next, we'll look at patterns, which are another of Rust's features that enable
lots of flexibility. We've looked at them briefly throughout the book but
haven't seen their full capability yet. Let's go!
-->

Ensuite, nous examinerons les motifs, qui sont une autre fonctionnalité de Rust permettant beaucoup de flexibilité. Nous les avons examinés brièvement tout au long du livre mais n'avons pas encore vu toutes leurs capacités. Allons-y !

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch20-05-macros.html#macros
