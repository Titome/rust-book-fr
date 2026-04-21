<!--
## Publishing a Crate to Crates.io
-->

## Publier un crate sur Crates.io

<!--
We've used packages from [crates.io](https://crates.io/) ignore
--> as
dependencies of our project, but you can also share your code with other people
by publishing your own packages. The crate registry at
[crates.io](https://crates.io/)<!--
ignore
--> distributes the source code of
your packages, so it primarily hosts code that is open source.
-->

Nous avons utilise des paquets de [crates.io](https://crates.io/)<!--
ignore
--> comme dependances de notre projet, mais vous pouvez egalement partager votre code avec d'autres personnes en publiant vos propres paquets. Le registre de crates sur [crates.io](https://crates.io/)<!--
ignore
--> distribue le code source de vos paquets, donc il heberge principalement du code open source.

<!--
Rust and Cargo have features that make your published package easier for people
to find and use. We'll talk about some of these features next and then explain
how to publish a package.
-->

Rust et Cargo ont des fonctionnalites qui rendent votre paquet publie plus facile a trouver et a utiliser pour les autres. Nous allons parler de certaines de ces fonctionnalites puis expliquer comment publier un paquet.

<!--
### Making Useful Documentation Comments
-->

### Rediger des commentaires de documentation utiles

<!--
Accurately documenting your packages will help other users know how and when to
use them, so it's worth investing the time to write documentation. In Chapter
3, we discussed how to comment Rust code using two slashes, `//`. Rust also has
a particular kind of comment for documentation, known conveniently as a
_documentation comment_, that will generate HTML documentation. The HTML
displays the contents of documentation comments for public API items intended
for programmers interested in knowing how to _use_ your crate as opposed to how
your crate is _implemented_.
-->

Documenter precisement vos paquets aidera les autres utilisateurs a savoir comment et quand les utiliser, donc cela vaut la peine d'investir du temps pour ecrire de la documentation. Dans le chapitre 3, nous avons vu comment commenter le code Rust en utilisant deux barres obliques, `//`. Rust possede egalement un type particulier de commentaire pour la documentation, connu de maniere pratique sous le nom de _commentaire de documentation_, qui generera une documentation HTML. Le HTML affiche le contenu des commentaires de documentation pour les elements de l'API publique, destine aux programmeurs interesses par savoir comment _utiliser_ votre crate par opposition a comment votre crate est _implemente_.

<!--
Documentation comments use three slashes, `///`, instead of two and support
Markdown notation for formatting the text. Place documentation comments just
before the item they're documenting. Listing 14-1 shows documentation comments
for an `add_one` function in a crate named `my_crate`.
-->

Les commentaires de documentation utilisent trois barres obliques, `///`, au lieu de deux et supportent la notation Markdown pour le formatage du texte. Placez les commentaires de documentation juste avant l'element qu'ils documentent. L'encart 14-1 montre des commentaires de documentation pour une fonction `add_one` dans un crate nomme `my_crate`.

<Listing number="14-1" file-name="src/lib.rs" caption="Un commentaire de documentation pour une fonction">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

<!--
Here, we give a description of what the `add_one` function does, start a
section with the heading `Examples`, and then provide code that demonstrates
how to use the `add_one` function. We can generate the HTML documentation from
this documentation comment by running `cargo doc`. This command runs the
`rustdoc` tool distributed with Rust and puts the generated HTML documentation
in the _target/doc_ directory.
-->

Ici, nous donnons une description de ce que fait la fonction `add_one`, commencons une section avec le titre `Examples`, puis fournissons du code qui montre comment utiliser la fonction `add_one`. Nous pouvons generer la documentation HTML a partir de ce commentaire de documentation en executant `cargo doc`. Cette commande execute l'outil `rustdoc` distribue avec Rust et place la documentation HTML generee dans le repertoire _target/doc_.

<!--
For convenience, running `cargo doc --open` will build the HTML for your
current crate's documentation (as well as the documentation for all of your
crate's dependencies) and open the result in a web browser. Navigate to the
`add_one` function and you'll see how the text in the documentation comments is
rendered, as shown in Figure 14-1.
-->

Par commodite, executer `cargo doc --open` construira le HTML de la documentation de votre crate actuel (ainsi que la documentation de toutes les dependances de votre crate) et ouvrira le resultat dans un navigateur web. Naviguez vers la fonction `add_one` et vous verrez comment le texte des commentaires de documentation est rendu, comme montre dans la figure 14-1.

<!--
<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />
-->

<img alt="Documentation HTML rendue pour la fonction `add_one` de `my_crate`" src="img/trpl14-01.png" class="center" />

<!--
<span class="caption">Figure 14-1: The HTML documentation for the `add_one`
function</span>
-->

<span class="caption">Figure 14-1 : La documentation HTML de la fonction `add_one`</span>

<!--
#### Commonly Used Sections
-->

#### Sections couramment utilisees

<!--
We used the `# Examples` Markdown heading in Listing 14-1 to create a section
in the HTML with the title "Examples." Here are some other sections that crate
authors commonly use in their documentation:
-->

Nous avons utilise le titre Markdown `# Examples` dans l'encart 14-1 pour creer une section dans le HTML avec le titre "Examples". Voici d'autres sections que les auteurs de crates utilisent couramment dans leur documentation :

<!--
- **Panics**: These are the scenarios in which the function being documented
  could panic. Callers of the function who don't want their programs to panic
  should make sure they don't call the function in these situations.
- **Errors**: If the function returns a `Result`, describing the kinds of
  errors that might occur and what conditions might cause those errors to be
  returned can be helpful to callers so that they can write code to handle the
  different kinds of errors in different ways.
- **Safety**: If the function is `unsafe` to call (we discuss unsafety in
  Chapter 20), there should be a section explaining why the function is unsafe
  and covering the invariants that the function expects callers to uphold.
-->

- **Panics** : ce sont les scenarios dans lesquels la fonction documentee pourrait paniquer. Les appelants de la fonction qui ne veulent pas que leurs programmes paniquent doivent s'assurer de ne pas appeler la fonction dans ces situations.
- **Errors** : si la fonction retourne un `Result`, decrire les types d'erreurs qui pourraient survenir et quelles conditions pourraient provoquer le retour de ces erreurs peut etre utile aux appelants afin qu'ils puissent ecrire du code pour gerer les differents types d'erreurs de differentes manieres.
- **Safety** : si la fonction est `unsafe` a appeler (nous discutons de l'unsafety dans le chapitre 20), il devrait y avoir une section expliquant pourquoi la fonction est unsafe et couvrant les invariants que la fonction attend des appelants qu'ils respectent.

<!--
Most documentation comments don't need all of these sections, but this is a
good checklist to remind you of the aspects of your code users will be
interested in knowing about.
-->

La plupart des commentaires de documentation n'ont pas besoin de toutes ces sections, mais c'est une bonne liste de verification pour vous rappeler les aspects de votre code que les utilisateurs seront interesses a connaitre.

<!--
#### Documentation Comments as Tests
-->

#### Les commentaires de documentation comme tests

<!--
Adding example code blocks in your documentation comments can help demonstrate
how to use your library and has an additional bonus: Running `cargo test` will
run the code examples in your documentation as tests! Nothing is better than
documentation with examples. But nothing is worse than examples that don't work
because the code has changed since the documentation was written. If we run
`cargo test` with the documentation for the `add_one` function from Listing
14-1, we will see a section in the test results that looks like this:
-->

Ajouter des blocs de code d'exemple dans vos commentaires de documentation peut aider a montrer comment utiliser votre bibliotheque et offre un avantage supplementaire : executer `cargo test` executera les exemples de code dans votre documentation comme des tests ! Rien n'est mieux que de la documentation avec des exemples. Mais rien n'est pire que des exemples qui ne fonctionnent pas parce que le code a change depuis que la documentation a ete ecrite. Si nous executons `cargo test` avec la documentation de la fonction `add_one` de l'encart 14-1, nous verrons une section dans les resultats de test qui ressemble a ceci :

<!--
manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

<!--
```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

<!--
Now, if we change either the function or the example so that the `assert_eq!`
in the example panics, and run `cargo test` again, we'll see that the doc tests
catch that the example and the code are out of sync with each other!
-->

Maintenant, si nous modifions soit la fonction soit l'exemple de sorte que le `assert_eq!` dans l'exemple panique, et executons `cargo test` a nouveau, nous verrons que les tests de documentation detectent que l'exemple et le code ne sont plus synchronises !

<!--
Old headings. Do not remove or links may break.
-->

<a id="commenting-contained-items"></a>

<!--
#### Contained Item Comments
-->

#### Les commentaires d'elements conteneurs

<!--
The style of doc comment `//!` adds documentation to the item that *contains*
the comments rather than to the items *following* the comments. We typically
use these doc comments inside the crate root file (_src/lib.rs_ by convention)
or inside a module to document the crate or the module as a whole.
-->

Le style de commentaire de documentation `//!` ajoute de la documentation a l'element qui *contient* les commentaires plutot qu'aux elements qui *suivent* les commentaires. Nous utilisons generalement ces commentaires de documentation a l'interieur du fichier racine du crate (_src/lib.rs_ par convention) ou a l'interieur d'un module pour documenter le crate ou le module dans son ensemble.

<!--
For example, to add documentation that describes the purpose of the `my_crate`
crate that contains the `add_one` function, we add documentation comments that
start with `//!` to the beginning of the _src/lib.rs_ file, as shown in Listing
14-2.
-->

Par exemple, pour ajouter de la documentation qui decrit l'objectif du crate `my_crate` qui contient la fonction `add_one`, nous ajoutons des commentaires de documentation qui commencent par `//!` au debut du fichier _src/lib.rs_, comme montre dans l'encart 14-2.

<Listing number="14-2" file-name="src/lib.rs" caption="La documentation du crate `my_crate` dans son ensemble">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

<!--
Notice there isn't any code after the last line that begins with `//!`. Because
we started the comments with `//!` instead of `///`, we're documenting the item
that contains this comment rather than an item that follows this comment. In
this case, that item is the _src/lib.rs_ file, which is the crate root. These
comments describe the entire crate.
-->

Remarquez qu'il n'y a aucun code apres la derniere ligne qui commence par `//!`. Parce que nous avons commence les commentaires avec `//!` au lieu de `///`, nous documentons l'element qui contient ce commentaire plutot qu'un element qui suit ce commentaire. Dans ce cas, cet element est le fichier _src/lib.rs_, qui est la racine du crate. Ces commentaires decrivent l'ensemble du crate.

<!--
When we run `cargo doc --open`, these comments will display on the front page
of the documentation for `my_crate` above the list of public items in the
crate, as shown in Figure 14-2.
-->

Lorsque nous executons `cargo doc --open`, ces commentaires s'afficheront sur la page d'accueil de la documentation de `my_crate` au-dessus de la liste des elements publics du crate, comme montre dans la figure 14-2.

<!--
Documentation comments within items are useful for describing crates and
modules especially. Use them to explain the overall purpose of the container to
help your users understand the crate's organization.
-->

Les commentaires de documentation a l'interieur des elements sont utiles pour decrire les crates et les modules en particulier. Utilisez-les pour expliquer l'objectif general du conteneur afin d'aider vos utilisateurs a comprendre l'organisation du crate.

<!--
<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />
-->

<img alt="Documentation HTML rendue avec un commentaire pour le crate dans son ensemble" src="img/trpl14-02.png" class="center" />

<!--
<span class="caption">Figure 14-2: The rendered documentation for `my_crate`,
including the comment describing the crate as a whole</span>
-->

<span class="caption">Figure 14-2 : La documentation rendue pour `my_crate`, incluant le commentaire decrivant le crate dans son ensemble</span>

<!--
Old headings. Do not remove or links may break.
-->

<a id="exporting-a-convenient-public-api-with-pub-use"></a>

<!--
### Exporting a Convenient Public API
-->

### Exporter une API publique pratique

<!--
The structure of your public API is a major consideration when publishing a
crate. People who use your crate are less familiar with the structure than you
are and might have difficulty finding the pieces they want to use if your crate
has a large module hierarchy.
-->

La structure de votre API publique est une consideration majeure lors de la publication d'un crate. Les personnes qui utilisent votre crate sont moins familieres avec la structure que vous et pourraient avoir des difficultes a trouver les elements qu'elles veulent utiliser si votre crate a une grande hierarchie de modules.

<!--
In Chapter 7, we covered how to make items public using the `pub` keyword, and
how to bring items into a scope with the `use` keyword. However, the structure
that makes sense to you while you're developing a crate might not be very
convenient for your users. You might want to organize your structs in a
hierarchy containing multiple levels, but then people who want to use a type
you've defined deep in the hierarchy might have trouble finding out that type
exists. They might also be annoyed at having to enter `use
my_crate::some_module::another_module::UsefulType;` rather than `use
my_crate::UsefulType;`.
-->

Dans le chapitre 7, nous avons vu comment rendre des elements publics en utilisant le mot-cle `pub`, et comment amener des elements dans une portee avec le mot-cle `use`. Cependant, la structure qui a du sens pour vous pendant que vous developpez un crate pourrait ne pas etre tres pratique pour vos utilisateurs. Vous pourriez vouloir organiser vos structs dans une hierarchie contenant plusieurs niveaux, mais alors les personnes qui veulent utiliser un type que vous avez defini en profondeur dans la hierarchie pourraient avoir du mal a decouvrir que ce type existe. Elles pourraient aussi etre agacees de devoir ecrire `use my_crate::some_module::another_module::UsefulType;` plutot que `use my_crate::UsefulType;`.

<!--
The good news is that if the structure _isn't_ convenient for others to use
from another library, you don't have to rearrange your internal organization:
Instead, you can re-export items to make a public structure that's different
from your private structure by using `pub use`. *Re-exporting* takes a public
item in one location and makes it public in another location, as if it were
defined in the other location instead.
-->

La bonne nouvelle est que si la structure _n'est pas_ pratique pour que d'autres l'utilisent depuis une autre bibliotheque, vous n'avez pas a reorganiser votre organisation interne : au lieu de cela, vous pouvez re-exporter des elements pour creer une structure publique differente de votre structure privee en utilisant `pub use`. La *re-exportation* prend un element public a un emplacement et le rend public a un autre emplacement, comme s'il etait defini a l'autre emplacement.

<!--
For example, say we made a library named `art` for modeling artistic concepts.
Within this library are two modules: a `kinds` module containing two enums
named `PrimaryColor` and `SecondaryColor` and a `utils` module containing a
function named `mix`, as shown in Listing 14-3.
-->

Par exemple, supposons que nous ayons cree une bibliotheque nommee `art` pour modeliser des concepts artistiques. Dans cette bibliotheque se trouvent deux modules : un module `kinds` contenant deux enums nommes `PrimaryColor` et `SecondaryColor` et un module `utils` contenant une fonction nommee `mix`, comme montre dans l'encart 14-3.

<Listing number="14-3" file-name="src/lib.rs" caption="Une bibliotheque `art` avec des elements organises en modules `kinds` et `utils`">


```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

<!--
Figure 14-3 shows what the front page of the documentation for this crate
generated by `cargo doc` would look like.
-->

La figure 14-3 montre a quoi ressemblerait la page d'accueil de la documentation de ce crate generee par `cargo doc`.

<!--
<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />
-->

<img alt="Documentation rendue pour le crate `art` qui liste les modules `kinds` et `utils`" src="img/trpl14-03.png" class="center" />

<!--
<span class="caption">Figure 14-3: The front page of the documentation for `art`
that lists the `kinds` and `utils` modules</span>
-->

<span class="caption">Figure 14-3 : La page d'accueil de la documentation pour `art` qui liste les modules `kinds` et `utils`</span>

<!--
Note that the `PrimaryColor` and `SecondaryColor` types aren't listed on the
front page, nor is the `mix` function. We have to click `kinds` and `utils` to
see them.
-->

Notez que les types `PrimaryColor` et `SecondaryColor` ne sont pas listes sur la page d'accueil, pas plus que la fonction `mix`. Nous devons cliquer sur `kinds` et `utils` pour les voir.

<!--
Another crate that depends on this library would need `use` statements that
bring the items from `art` into scope, specifying the module structure that's
currently defined. Listing 14-4 shows an example of a crate that uses the
`PrimaryColor` and `mix` items from the `art` crate.
-->

Un autre crate qui depend de cette bibliotheque aurait besoin d'instructions `use` qui amenent les elements d'`art` dans la portee, en specifiant la structure de modules actuellement definie. L'encart 14-4 montre un exemple de crate qui utilise les elements `PrimaryColor` et `mix` du crate `art`.

<Listing number="14-4" file-name="src/main.rs" caption="Un crate utilisant les elements du crate `art` avec sa structure interne exportee">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

<!--
The author of the code in Listing 14-4, which uses the `art` crate, had to
figure out that `PrimaryColor` is in the `kinds` module and `mix` is in the
`utils` module. The module structure of the `art` crate is more relevant to
developers working on the `art` crate than to those using it. The internal
structure doesn't contain any useful information for someone trying to
understand how to use the `art` crate, but rather causes confusion because
developers who use it have to figure out where to look, and must specify the
module names in the `use` statements.
-->

L'auteur du code de l'encart 14-4, qui utilise le crate `art`, a du comprendre que `PrimaryColor` est dans le module `kinds` et que `mix` est dans le module `utils`. La structure de modules du crate `art` est plus pertinente pour les developpeurs travaillant sur le crate `art` que pour ceux qui l'utilisent. La structure interne ne contient aucune information utile pour quelqu'un essayant de comprendre comment utiliser le crate `art`, mais cause plutot de la confusion car les developpeurs qui l'utilisent doivent comprendre ou chercher et doivent specifier les noms de modules dans les instructions `use`.

<!--
To remove the internal organization from the public API, we can modify the
`art` crate code in Listing 14-3 to add `pub use` statements to re-export the
items at the top level, as shown in Listing 14-5.
-->

Pour retirer l'organisation interne de l'API publique, nous pouvons modifier le code du crate `art` de l'encart 14-3 pour ajouter des instructions `pub use` afin de re-exporter les elements au niveau superieur, comme montre dans l'encart 14-5.

<Listing number="14-5" file-name="src/lib.rs" caption="Ajout d'instructions `pub use` pour re-exporter des elements">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

<!--
The API documentation that `cargo doc` generates for this crate will now list
and link re-exports on the front page, as shown in Figure 14-4, making the
`PrimaryColor` and `SecondaryColor` types and the `mix` function easier to find.
-->

La documentation de l'API que `cargo doc` genere pour ce crate listera et liera desormais les re-exportations sur la page d'accueil, comme montre dans la figure 14-4, rendant les types `PrimaryColor` et `SecondaryColor` et la fonction `mix` plus faciles a trouver.

<!--
<img alt="Rendered documentation for the `art` crate with the re-exports on the front page" src="img/trpl14-04.png" class="center" />
-->

<img alt="Documentation rendue pour le crate `art` avec les re-exportations sur la page d'accueil" src="img/trpl14-04.png" class="center" />

<!--
<span class="caption">Figure 14-4: The front page of the documentation for `art`
that lists the re-exports</span>
-->

<span class="caption">Figure 14-4 : La page d'accueil de la documentation pour `art` qui liste les re-exportations</span>

<!--
The `art` crate users can still see and use the internal structure from Listing
14-3 as demonstrated in Listing 14-4, or they can use the more convenient
structure in Listing 14-5, as shown in Listing 14-6.
-->

Les utilisateurs du crate `art` peuvent toujours voir et utiliser la structure interne de l'encart 14-3 comme demontre dans l'encart 14-4, ou ils peuvent utiliser la structure plus pratique de l'encart 14-5, comme montre dans l'encart 14-6.

<Listing number="14-6" file-name="src/main.rs" caption="Un programme utilisant les elements re-exportes du crate `art`">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

<!--
In cases where there are many nested modules, re-exporting the types at the top
level with `pub use` can make a significant difference in the experience of
people who use the crate. Another common use of `pub use` is to re-export
definitions of a dependency in the current crate to make that crate's
definitions part of your crate's public API.
-->

Dans les cas ou il y a de nombreux modules imbriques, re-exporter les types au niveau superieur avec `pub use` peut faire une difference significative dans l'experience des personnes qui utilisent le crate. Une autre utilisation courante de `pub use` est de re-exporter les definitions d'une dependance dans le crate actuel pour faire de ces definitions une partie de l'API publique de votre crate.

<!--
Creating a useful public API structure is more an art than a science, and you
can iterate to find the API that works best for your users. Choosing `pub use`
gives you flexibility in how you structure your crate internally and decouples
that internal structure from what you present to your users. Look at some of
the code of crates you've installed to see if their internal structure differs
from their public API.
-->

Creer une structure d'API publique utile est plus un art qu'une science, et vous pouvez iterer pour trouver l'API qui fonctionne le mieux pour vos utilisateurs. Choisir `pub use` vous donne de la flexibilite dans la facon dont vous structurez votre crate en interne et decouple cette structure interne de ce que vous presentez a vos utilisateurs. Examinez le code de certains crates que vous avez installes pour voir si leur structure interne differe de leur API publique.

<!--
### Setting Up a Crates.io Account
-->

### Configurer un compte Crates.io

<!--
Before you can publish any crates, you need to create an account on
[crates.io](https://crates.io/) ignore
--> and get an API token. To do so,
visit the home page at [crates.io](https://crates.io/)<!--
ignore
--> and log
in via a GitHub account. (The GitHub account is currently a requirement, but
the site might support other ways of creating an account in the future.) Once
you're logged in, visit your account settings at
[https://crates.io/me/](https://crates.io/me/)<!--
ignore
--> and retrieve your
API key. Then, run the `cargo login` command and paste your API key when prompted, like this:
-->

Avant de pouvoir publier des crates, vous devez creer un compte sur [crates.io](https://crates.io/)<!--
ignore
--> et obtenir un jeton d'API. Pour ce faire, visitez la page d'accueil de [crates.io](https://crates.io/)<!--
ignore
--> et connectez-vous via un compte GitHub. (Le compte GitHub est actuellement une exigence, mais le site pourrait supporter d'autres moyens de creer un compte a l'avenir.) Une fois connecte, visitez les parametres de votre compte a [https://crates.io/me/](https://crates.io/me/)<!--
ignore
--> et recuperez votre cle d'API. Ensuite, executez la commande `cargo login` et collez votre cle d'API lorsque cela vous est demande, comme ceci :

<!--
```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```
-->

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

<!--
This command will inform Cargo of your API token and store it locally in
_~/.cargo/credentials.toml_. Note that this token is a secret: Do not share
it with anyone else. If you do share it with anyone for any reason, you should
revoke it and generate a new token on [crates.io](https://crates.io/) ignore
-->.
-->

Cette commande informera Cargo de votre jeton d'API et le stockera localement dans _~/.cargo/credentials.toml_. Notez que ce jeton est un secret : ne le partagez avec personne. Si vous le partagez avec quelqu'un pour quelque raison que ce soit, vous devez le revoquer et generer un nouveau jeton sur [crates.io](https://crates.io/)<!--
ignore
-->.

<!--
### Adding Metadata to a New Crate
-->

### Ajouter des metadonnees a un nouveau crate

<!--
Let's say you have a crate you want to publish. Before publishing, you'll need
to add some metadata in the `[package]` section of the crate's _Cargo.toml_
file.
-->

Supposons que vous avez un crate que vous souhaitez publier. Avant de publier, vous devrez ajouter des metadonnees dans la section `[package]` du fichier _Cargo.toml_ du crate.

<!--
Your crate will need a unique name. While you're working on a crate locally,
you can name a crate whatever you'd like. However, crate names on
[crates.io](https://crates.io/) ignore
--> are allocated on a first-come,
first-served basis. Once a crate name is taken, no one else can publish a crate
with that name. Before attempting to publish a crate, search for the name you
want to use. If the name has been used, you will need to find another name and
edit the `name` field in the _Cargo.toml_ file under the `[package]` section to
use the new name for publishing, like so:
-->

Votre crate aura besoin d'un nom unique. Pendant que vous travaillez sur un crate localement, vous pouvez nommer le crate comme vous le souhaitez. Cependant, les noms de crates sur [crates.io](https://crates.io/)<!--
ignore
--> sont attribues selon le principe du premier arrive, premier servi. Une fois qu'un nom de crate est pris, personne d'autre ne peut publier un crate avec ce nom. Avant de tenter de publier un crate, recherchez le nom que vous souhaitez utiliser. Si le nom a deja ete utilise, vous devrez trouver un autre nom et modifier le champ `name` dans le fichier _Cargo.toml_ sous la section `[package]` pour utiliser le nouveau nom pour la publication, comme ceci :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[package]
name = "guessing_game"
```
-->

```toml
[package]
name = "guessing_game"
```

<!--
Even if you've chosen a unique name, when you run `cargo publish` to publish
the crate at this point, you'll get a warning and then an error:
-->

Meme si vous avez choisi un nom unique, lorsque vous executez `cargo publish` pour publier le crate a ce stade, vous obtiendrez un avertissement puis une erreur :

<!--
manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->

<!--
```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

<!--
This results in an error because you're missing some crucial information: A
description and license are required so that people will know what your crate
does and under what terms they can use it. In _Cargo.toml_, add a description
that's just a sentence or two, because it will appear with your crate in search
results. For the `license` field, you need to give a _license identifier
value_. The [Linux Foundation's Software Package Data Exchange (SPDX)][spdx]
lists the identifiers you can use for this value. For example, to specify that
you've licensed your crate using the MIT License, add the `MIT` identifier:
-->

Cela entraine une erreur car il vous manque des informations cruciales : une description et une licence sont requises pour que les gens sachent ce que fait votre crate et sous quelles conditions ils peuvent l'utiliser. Dans _Cargo.toml_, ajoutez une description d'une ou deux phrases, car elle apparaitra avec votre crate dans les resultats de recherche. Pour le champ `license`, vous devez donner une _valeur d'identifiant de licence_. Le [Software Package Data Exchange (SPDX) de la Linux Foundation][spdx] liste les identifiants que vous pouvez utiliser pour cette valeur. Par exemple, pour specifier que vous avez licencie votre crate sous la licence MIT, ajoutez l'identifiant `MIT` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[package]
name = "guessing_game"
license = "MIT"
```
-->

```toml
[package]
name = "guessing_game"
license = "MIT"
```

<!--
If you want to use a license that doesn't appear in the SPDX, you need to place
the text of that license in a file, include the file in your project, and then
use `license-file` to specify the name of that file instead of using the
`license` key.
-->

Si vous souhaitez utiliser une licence qui n'apparait pas dans le SPDX, vous devez placer le texte de cette licence dans un fichier, inclure le fichier dans votre projet, puis utiliser `license-file` pour specifier le nom de ce fichier au lieu d'utiliser la cle `license`.

<!--
Guidance on which license is appropriate for your project is beyond the scope
of this book. Many people in the Rust community license their projects in the
same way as Rust by using a dual license of `MIT OR Apache-2.0`. This practice
demonstrates that you can also specify multiple license identifiers separated
by `OR` to have multiple licenses for your project.
-->

Les conseils sur la licence appropriee pour votre projet depassent le cadre de ce livre. De nombreuses personnes dans la communaute Rust licencient leurs projets de la meme maniere que Rust en utilisant une double licence `MIT OR Apache-2.0`. Cette pratique demontre que vous pouvez egalement specifier plusieurs identifiants de licence separes par `OR` pour avoir plusieurs licences pour votre projet.

<!--
With a unique name, the version, your description, and a license added, the
_Cargo.toml_ file for a project that is ready to publish might look like this:
-->

Avec un nom unique, la version, votre description et une licence ajoutee, le fichier _Cargo.toml_ d'un projet pret a etre publie pourrait ressembler a ceci :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```
-->

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

<!--
[Cargo's documentation](https://doc.rust-lang.org/cargo/) describes other
metadata you can specify to ensure that others can discover and use your crate
more easily.
-->

[La documentation de Cargo](https://doc.rust-lang.org/cargo/) decrit d'autres metadonnees que vous pouvez specifier pour vous assurer que d'autres puissent decouvrir et utiliser votre crate plus facilement.

<!--
### Publishing to Crates.io
-->

### Publier sur Crates.io

<!--
Now that you've created an account, saved your API token, chosen a name for
your crate, and specified the required metadata, you're ready to publish!
Publishing a crate uploads a specific version to
[crates.io](https://crates.io/) ignore
--> for others to use.
-->

Maintenant que vous avez cree un compte, sauvegarde votre jeton d'API, choisi un nom pour votre crate et specifie les metadonnees requises, vous etes pret a publier ! Publier un crate televerse une version specifique sur [crates.io](https://crates.io/)<!--
ignore
--> pour que d'autres puissent l'utiliser.

<!--
Be careful, because a publish is _permanent_. The version can never be
overwritten, and the code cannot be deleted except in certain circumstances.
One major goal of Crates.io is to act as a permanent archive of code so that
builds of all projects that depend on crates from
[crates.io](https://crates.io/) ignore
--> will continue to work. Allowing
version deletions would make fulfilling that goal impossible. However, there is
no limit to the number of crate versions you can publish.
-->

Soyez prudent, car une publication est _permanente_. La version ne peut jamais etre ecrasee, et le code ne peut pas etre supprime sauf dans certaines circonstances. Un objectif majeur de Crates.io est d'agir comme une archive permanente de code afin que les compilations de tous les projets qui dependent de crates de [crates.io](https://crates.io/)<!--
ignore
--> continuent de fonctionner. Permettre la suppression de versions rendrait impossible la realisation de cet objectif. Cependant, il n'y a pas de limite au nombre de versions de crate que vous pouvez publier.

<!--
Run the `cargo publish` command again. It should succeed now:
-->

Executez a nouveau la commande `cargo publish`. Elle devrait reussir maintenant :

<!--
manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

<!--
```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```

<!--
Congratulations! You've now shared your code with the Rust community, and
anyone can easily add your crate as a dependency of their project.
-->

Felicitations ! Vous avez maintenant partage votre code avec la communaute Rust, et n'importe qui peut facilement ajouter votre crate comme dependance de son projet.

<!--
### Publishing a New Version of an Existing Crate
-->

### Publier une nouvelle version d'un crate existant

<!--
When you've made changes to your crate and are ready to release a new version,
you change the `version` value specified in your _Cargo.toml_ file and
republish. Use the [Semantic Versioning rules][semver] to decide what an
appropriate next version number is, based on the kinds of changes you've made.
Then, run `cargo publish` to upload the new version.
-->

Lorsque vous avez apporte des modifications a votre crate et que vous etes pret a publier une nouvelle version, vous modifiez la valeur `version` specifiee dans votre fichier _Cargo.toml_ et republiez. Utilisez les [regles de versionnage semantique][semver] pour decider quel est le numero de version suivant approprie, en fonction des types de modifications que vous avez apportees. Ensuite, executez `cargo publish` pour televerser la nouvelle version.

<!--
Old headings. Do not remove or links may break.
-->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>
<a id="deprecating-versions-from-cratesio-with-cargo-yank"></a>

<!--
### Deprecating Versions from Crates.io
-->

### Deprecier des versions de Crates.io

<!--
Although you can't remove previous versions of a crate, you can prevent any
future projects from adding them as a new dependency. This is useful when a
crate version is broken for one reason or another. In such situations, Cargo
supports yanking a crate version.
-->

Bien que vous ne puissiez pas supprimer les versions precedentes d'un crate, vous pouvez empecher tout futur projet de les ajouter comme nouvelle dependance. Cela est utile lorsqu'une version de crate est cassee pour une raison ou une autre. Dans de telles situations, Cargo supporte le retrait (yank) d'une version de crate.

<!--
_Yanking_ a version prevents new projects from depending on that version while
allowing all existing projects that depend on it to continue. Essentially, a
yank means that all projects with a _Cargo.lock_ will not break, and any future
_Cargo.lock_ files generated will not use the yanked version.
-->

_Retirer_ (yank) une version empeche les nouveaux projets de dependre de cette version tout en permettant a tous les projets existants qui en dependent de continuer. Essentiellement, un yank signifie que tous les projets avec un _Cargo.lock_ ne seront pas casses, et que tout futur fichier _Cargo.lock_ genere n'utilisera pas la version retiree.

<!--
To yank a version of a crate, in the directory of the crate that you've
previously published, run `cargo yank` and specify which version you want to
yank. For example, if we've published a crate named `guessing_game` version
1.0.1 and we want to yank it, then we'd run the following in the project
directory for `guessing_game`:
-->

Pour retirer une version d'un crate, dans le repertoire du crate que vous avez precedemment publie, executez `cargo yank` et specifiez quelle version vous souhaitez retirer. Par exemple, si nous avons publie un crate nomme `guessing_game` version 1.0.1 et que nous voulons le retirer, nous executerions la commande suivante dans le repertoire du projet `guessing_game` :

<!--
manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

<!--
```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

<!--
By adding `--undo` to the command, you can also undo a yank and allow projects
to start depending on a version again:
-->

En ajoutant `--undo` a la commande, vous pouvez egalement annuler un retrait et permettre aux projets de dependre a nouveau d'une version :

<!--
```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```
-->

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

<!--
A yank _does not_ delete any code. It cannot, for example, delete accidentally
uploaded secrets. If that happens, you must reset those secrets immediately.
-->

Un retrait _ne supprime pas_ de code. Il ne peut pas, par exemple, supprimer des secrets accidentellement televerses. Si cela arrive, vous devez reinitialiser ces secrets immediatement.

[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
