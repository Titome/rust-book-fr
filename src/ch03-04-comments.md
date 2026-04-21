<!--
## Comments
-->

## Les commentaires

<!--
All programmers strive to make their code easy to understand, but sometimes
extra explanation is warranted. In these cases, programmers leave _comments_ in
their source code that the compiler will ignore but that people reading the
source code may find useful.
-->

Tous les programmeurs s'efforcent de rendre leur code facile à comprendre, mais
parfois des explications supplémentaires sont nécessaires. Dans ces cas, les
programmeurs laissent des _commentaires_ dans leur code source que le compilateur
ignorera mais que les personnes lisant le code source pourront trouver utiles.

<!--
Here's a simple comment:
-->

Voici un commentaire simple :

<!--
```rust
// hello, world
```
-->

```rust
// hello, world
```

<!--
In Rust, the idiomatic comment style starts a comment with two slashes, and the
comment continues until the end of the line. For comments that extend beyond a
single line, you'll need to include `//` on each line, like this:
-->

En Rust, le style de commentaire idiomatique commence un commentaire avec deux
barres obliques, et le commentaire continue jusqu'à la fin de la ligne. Pour les
commentaires qui s'étendent au-delà d'une seule ligne, vous devrez inclure `//`
sur chaque ligne, comme ceci :

<!--
```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```
-->

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

<!--
Comments can also be placed at the end of lines containing code:
-->

Les commentaires peuvent également être placés à la fin des lignes contenant du
code :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

<!--
But you'll more often see them used in this format, with the comment on a
separate line above the code it's annotating:
-->

Mais vous les verrez plus souvent utilisés dans ce format, avec le commentaire
sur une ligne séparée au-dessus du code qu'il annote :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

<!--
Rust also has another kind of comment, documentation comments, which we'll
discuss in the ["Publishing a Crate to Crates.io"][publishing] ignore
-->
section of Chapter 14.
-->

Rust possède également un autre type de commentaire, les commentaires de
documentation, que nous aborderons dans la section ["Publishing a Crate to
Crates.io"][publishing]<!--
ignore
--> du chapitre 14.

[publishing]: ch14-02-publishing-to-crates-io.html
