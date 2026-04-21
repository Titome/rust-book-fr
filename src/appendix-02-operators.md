<!--
## Appendix B: Operators and Symbols
-->

## Annexe B : les opérateurs et les symboles

<!--
This appendix contains a glossary of Rust's syntax, including operators and
other symbols that appear by themselves or in the context of paths, generics,
trait bounds, macros, attributes, comments, tuples, and brackets.
-->

Cette annexe contient un glossaire de la syntaxe de Rust, qui comprend les
opérateurs et d'autres symboles qui apparaissent seuls ou dans le contexte de
chemins, de génériques, de limites de traits, de macros, d'attributs, de
commentaires, de tuples et de crochets.

<!--
### Operators
-->

### Les opérateurs

<!--
Table B-1 contains the operators in Rust, an example of how the operator would
appear in context, a short explanation, and whether that operator is
overloadable. If an operator is overloadable, the relevant trait to use to
overload that operator is listed.
-->

Le tableau B-1 contient les opérateurs de Rust, un exemple montrant comment
l'opérateur pourrait apparaître en contexte, une courte explication, et si cet
opérateur est surchargeable. Si un opérateur est surchargeable, le trait
correspondant à utiliser pour le surcharger est indiqué.

<!--
<span class="caption">Table B-1: Operators</span>
-->

<span class="caption">Tableau B-1 : les opérateurs</span>

<!--
| Operator | Example | Explanation | Overloadable? |
-->

| Opérateur                 | Exemple                                                 | Explication                                                                    | Surchargeable ? |
| ------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------ | --------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | Expansion de macro                                                             |                 |
| `!`                       | `!expr`                                                 | Complément binaire ou logique                                                  | `Not`           |
| `!=`                      | `expr != expr`                                          | Comparaison de non-égalité                                                     | `PartialEq`     |
| `%`                       | `expr % expr`                                           | Reste arithmétique                                                             | `Rem`           |
| `%=`                      | `var %= expr`                                           | Reste arithmétique et assignation                                              | `RemAssign`     |
| `&`                       | `&expr`, `&mut expr`                                    | Emprunt                                                                        |                 |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | Type de pointeur emprunté                                                      |                 |
| `&`                       | `expr & expr`                                           | ET binaire                                                                     | `BitAnd`        |
| `&=`                      | `var &= expr`                                           | ET binaire et assignation                                                      | `BitAndAssign`  |
| `&&`                      | `expr && expr`                                          | ET logique avec court-circuit                                                  |                 |
| `*`                       | `expr * expr`                                           | Multiplication arithmétique                                                    | `Mul`           |
| `*=`                      | `var *= expr`                                           | Multiplication arithmétique et assignation                                     | `MulAssign`     |
| `*`                       | `*expr`                                                 | Déréférencement                                                                | `Deref`         |
| `*`                       | `*const type`, `*mut type`                              | Pointeur brut                                                                  |                 |
| `+`                       | `trait + trait`, `'a + trait`                           | Contrainte de type composée                                                    |                 |
| `+`                       | `expr + expr`                                           | Addition arithmétique                                                          | `Add`           |
| `+=`                      | `var += expr`                                           | Addition arithmétique et assignation                                           | `AddAssign`     |
| `,`                       | `expr, expr`                                            | Séparateur d'arguments et d'éléments                                           |                 |
| `-`                       | `- expr`                                                | Négation arithmétique                                                          | `Neg`           |
| `-`                       | `expr - expr`                                           | Soustraction arithmétique                                                      | `Sub`           |
| `-=`                      | `var -= expr`                                           | Soustraction arithmétique et assignation                                       | `SubAssign`     |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Type de retour d'une fonction ou d'une fermeture                               |                 |
| `.`                       | `expr.ident`                                            | Accès à un champ                                                               |                 |
| `.`                       | `expr.ident(expr, ...)`                                 | Appel de méthode                                                               |                 |
| `.`                       | `expr.0`, `expr.1`, et ainsi de suite                   | Indexation de tuple                                                            |                 |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | Littéral d'intervalle exclusif à droite                                        | `PartialOrd`    |
| `..=`                     | `..=expr`, `expr..=expr`                                | Littéral d'intervalle inclusif à droite                                        | `PartialOrd`    |
| `..`                      | `..expr`                                                | Syntaxe de mise à jour de littéral de structure                                |                 |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | Motif "et le reste"                                                            |                 |
| `...`                     | `expr...expr`                                           | (Déprécié, utilisez `..=` à la place) Dans un motif : motif d'intervalle inclusif |             |
| `/`                       | `expr / expr`                                           | Division arithmétique                                                          | `Div`           |
| `/=`                      | `var /= expr`                                           | Division arithmétique et assignation                                           | `DivAssign`     |
| `:`                       | `pat: type`, `ident: type`                              | Contraintes                                                                    |                 |
| `:`                       | `ident: expr`                                           | Initialiseur de champ de structure                                             |                 |
| `:`                       | `'a: loop {...}`                                        | Étiquette de boucle                                                            |                 |
| `;`                       | `expr;`                                                 | Fin d'instruction et d'élément                                                 |                 |
| `;`                       | `[...; len]`                                            | Partie de la syntaxe de tableau à taille fixe                                  |                 |
| `<<`                      | `expr << expr`                                          | Décalage à gauche                                                              | `Shl`           |
| `<<=`                     | `var <<= expr`                                          | Décalage à gauche et assignation                                               | `ShlAssign`     |
| `<`                       | `expr < expr`                                           | Comparaison inférieur à                                                        | `PartialOrd`    |
| `<=`                      | `expr <= expr`                                          | Comparaison inférieur ou égal à                                                | `PartialOrd`    |
| `=`                       | `var = expr`, `ident = type`                            | Assignation/équivalence                                                        |                 |
| `==`                      | `expr == expr`                                          | Comparaison d'égalité                                                          | `PartialEq`     |
| `=>`                      | `pat => expr`                                           | Partie de la syntaxe d'une branche de match                                   |                 |
| `>`                       | `expr > expr`                                           | Comparaison supérieur à                                                        | `PartialOrd`    |
| `>=`                      | `expr >= expr`                                          | Comparaison supérieur ou égal à                                                | `PartialOrd`    |
| `>>`                      | `expr >> expr`                                          | Décalage à droite                                                              | `Shr`           |
| `>>=`                     | `var >>= expr`                                          | Décalage à droite et assignation                                               | `ShrAssign`     |
| `@`                       | `ident @ pat`                                           | Liaison de motif                                                               |                 |
| `^`                       | `expr ^ expr`                                           | OU exclusif binaire                                                            | `BitXor`        |
| `^=`                      | `var ^= expr`                                           | OU exclusif binaire et assignation                                             | `BitXorAssign`  |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | Alternatives de motif                                                          |                 |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | OU binaire                                                                     | `BitOr`         |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | OU binaire et assignation                                                      | `BitOrAssign`   |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | OU logique avec court-circuit                                                  |                 |
| `?`                       | `expr?`                                                 | Propagation d'erreur                                                           |                 |

<!--
### Non-operator Symbols
-->

### Les symboles non-opérateurs

<!--
The following tables contain all symbols that don't function as operators; that
is, they don't behave like a function or method call.
-->

Les tableaux suivants contiennent tous les symboles qui ne fonctionnent pas
comme des opérateurs ; c'est-à-dire qu'ils ne se comportent pas comme un appel
de fonction ou de méthode.

<!--
Table B-2 shows symbols that appear on their own and are valid in a variety of
locations.
-->

Le tableau B-2 montre les symboles qui apparaissent seuls et sont valides à
divers emplacements.

<!--
<span class="caption">Table B-2: Stand-alone Syntax</span>
-->

<span class="caption">Tableau B-2 : syntaxe autonome</span>

<!--
| Symbol | Explanation |
-->

| Symbole                                                                | Explication                                                                    |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `'ident`                                                               | Durée de vie nommée ou étiquette de boucle                                     |
| Chiffres immédiatement suivis de `u8`, `i32`, `f64`, `usize`, etc.     | Littéral numérique d'un type spécifique                                        |
| `"..."`                                                                | Littéral de chaîne de caractères                                               |
| `r"..."`, `r#"..."#`, `r##"..."##`, et ainsi de suite                   | Littéral de chaîne brute ; les caractères d'échappement ne sont pas traités    |
| `b"..."`                                                               | Littéral de chaîne d'octets ; construit un tableau d'octets plutôt qu'une chaîne |
| `br"..."`, `br#"..."#`, `br##"..."##`, et ainsi de suite                | Littéral de chaîne d'octets brute ; combinaison de chaîne brute et d'octets   |
| `'...'`                                                                | Littéral de caractère                                                          |
| `b'...'`                                                               | Littéral d'octet ASCII                                                         |
| <code>&vert;...&vert; expr</code>                                      | Fermeture                                                                      |
| `!`                                                                    | Type bas (*bottom*) toujours vide pour les fonctions divergentes               |
| `_`                                                                    | Motif "ignoré" ; utilisé aussi pour rendre les littéraux entiers lisibles      |

<!--
Table B-3 shows symbols that appear in the context of a path through the module
hierarchy to an item.
-->

Le tableau B-3 montre les symboles qui apparaissent dans le contexte d'un chemin
à travers la hiérarchie de modules vers un élément.

<!--
<span class="caption">Table B-3: Path-Related Syntax</span>
-->

<span class="caption">Tableau B-3 : syntaxe relative aux chemins</span>

<!--
| Symbol | Explanation |
-->

| Symbole                                 | Explication                                                                                                            |
| --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `ident::ident`                          | Chemin d'espace de noms                                                                                                |
| `::path`                                | Chemin relatif à la racine de la crate (c'est-à-dire un chemin explicitement absolu)                                   |
| `self::path`                            | Chemin relatif au module courant (c'est-à-dire un chemin explicitement relatif)                                        |
| `super::path`                           | Chemin relatif au module parent du module courant                                                                      |
| `type::ident`, `<type as trait>::ident` | Constantes, fonctions et types associés                                                                                |
| `<type>::...`                           | Élément associé pour un type qui ne peut pas être directement nommé (par exemple, `<&T>::...`, `<[T]>::...`, etc.)     |
| `trait::method(...)`                    | Désambiguïsation d'un appel de méthode en nommant le trait qui la définit                                              |
| `type::method(...)`                     | Désambiguïsation d'un appel de méthode en nommant le type pour lequel elle est définie                                 |
| `<type as trait>::method(...)`          | Désambiguïsation d'un appel de méthode en nommant le trait et le type                                                  |

<!--
Table B-4 shows symbols that appear in the context of using generic type
parameters.
-->

Le tableau B-4 montre les symboles qui apparaissent dans le contexte de
l'utilisation de paramètres de types génériques.

<!--
<span class="caption">Table B-4: Generics</span>
-->

<span class="caption">Tableau B-4 : les génériques</span>

<!--
| Symbol | Explanation |
-->

| Symbole                        | Explication                                                                                                                                                     |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | Renseigne les paramètres d'un type générique dans un type (par exemple, `Vec<u8>`)                                                                              |
| `path::<...>`, `method::<...>` | Renseigne les paramètres d'un type, d'une fonction ou d'une méthode générique dans une expression ; souvent appelé *turbofish* (par exemple, `"42".parse::<i32>()`) |
| `fn ident<...> ...`            | Définit une fonction générique                                                                                                                                  |
| `struct ident<...> ...`        | Définit une structure générique                                                                                                                                 |
| `enum ident<...> ...`          | Définit une énumération générique                                                                                                                               |
| `impl<...> ...`                | Définit une implémentation générique                                                                                                                            |
| `for<...> type`                | Limites de durée de vie de rang supérieur                                                                                                                       |
| `type<ident=type>`             | Un type générique où un ou plusieurs types associés ont des assignations spécifiques (par exemple, `Iterator<Item=T>`)                                          |

<!--
Table B-5 shows symbols that appear in the context of constraining generic type
parameters with trait bounds.
-->

Le tableau B-5 montre les symboles qui apparaissent dans le contexte de la
contrainte de paramètres de types génériques avec des limites de traits.

<!--
<span class="caption">Table B-5: Trait Bound Constraints</span>
-->

<span class="caption">Tableau B-5 : les contraintes de limites de traits</span>

<!--
| Symbol | Explanation |
-->

| Symbole                       | Explication                                                                                                                                             |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `T: U`                        | Le paramètre générique `T` est contraint aux types qui implémentent `U`                                                                                |
| `T: 'a`                       | Le type générique `T` doit vivre au moins aussi longtemps que la durée de vie `'a` (le type ne peut pas contenir transitivement de références avec des durées de vie plus courtes que `'a`) |
| `T: 'static`                  | Le type générique `T` ne contient pas d'autres références empruntées que des références `'static`                                                      |
| `'b: 'a`                      | La durée de vie générique `'b` doit vivre au moins aussi longtemps que la durée de vie `'a`                                                            |
| `T: ?Sized`                   | Autorise le paramètre de type générique à être un type à taille dynamique                                                                              |
| `'a + trait`, `trait + trait` | Contrainte de type composée                                                                                                                             |

<!--
Table B-6 shows symbols that appear in the context of calling or defining
macros and specifying attributes on an item.
-->

Le tableau B-6 montre les symboles qui apparaissent dans le contexte de l'appel
ou de la définition de macros et de la spécification d'attributs sur un élément.

<!--
<span class="caption">Table B-6: Macros and Attributes</span>
-->

<span class="caption">Tableau B-6 : les macros et les attributs</span>

<!--
| Symbol | Explanation |
-->

| Symbole                                     | Explication              |
| ------------------------------------------- | ------------------------ |
| `#[meta]`                                   | Attribut externe         |
| `#![meta]`                                  | Attribut interne         |
| `$ident`                                    | Substitution de macro    |
| `$ident:kind`                               | Métavariable de macro    |
| `$(...)...`                                 | Répétition de macro      |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Invocation de macro      |

<!--
Table B-7 shows symbols that create comments.
-->

Le tableau B-7 montre les symboles qui créent des commentaires.

<!--
<span class="caption">Table B-7: Comments</span>
-->

<span class="caption">Tableau B-7 : les commentaires</span>

<!--
| Symbol | Explanation |
-->

| Symbole    | Explication                              |
| ---------- | ---------------------------------------- |
| `//`       | Commentaire de ligne                     |
| `//!`      | Commentaire de documentation interne     |
| `///`      | Commentaire de documentation externe     |
| `/*...*/`  | Commentaire de bloc                      |
| `/*!...*/` | Commentaire de documentation interne de bloc |
| `/**...*/` | Commentaire de documentation externe de bloc |

<!--
Table B-8 shows the contexts in which parentheses are used.
-->

Le tableau B-8 montre les contextes dans lesquels les parenthèses sont
utilisées.

<!--
<span class="caption">Table B-8: Parentheses</span>
-->

<span class="caption">Tableau B-8 : les parenthèses</span>

<!--
| Symbol | Explanation |
-->

| Symbole                  | Explication                                                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `()`                     | Tuple vide (aussi appelé *unit*), aussi bien en littéral qu'en type                                                |
| `(expr)`                 | Expression entre parenthèses                                                                                       |
| `(expr,)`                | Expression de tuple à un seul élément                                                                              |
| `(type,)`                | Type de tuple à un seul élément                                                                                    |
| `(expr, ...)`            | Expression de tuple                                                                                                |
| `(type, ...)`            | Type de tuple                                                                                                      |
| `expr(expr, ...)`        | Appel de fonction ; utilisé aussi pour initialiser les `struct` tuples et les variantes d'`enum` tuples            |

<!--
Table B-9 shows the contexts in which curly brackets are used.
-->

Le tableau B-9 montre les contextes dans lesquels les accolades sont utilisées.

<!--
<span class="caption">Table B-9: Curly Brackets</span>
-->

<span class="caption">Tableau B-9 : les accolades</span>

<!--
| Context | Explanation |
-->

| Contexte     | Explication              |
| ------------ | ------------------------ |
| `{...}`      | Expression de bloc       |
| `Type {...}` | Littéral de structure    |

<!--
Table B-10 shows the contexts in which square brackets are used.
-->

Le tableau B-10 montre les contextes dans lesquels les crochets sont utilisés.

<!--
<span class="caption">Table B-10: Square Brackets</span>
-->

<span class="caption">Tableau B-10 : les crochets</span>

<!--
| Context | Explanation |
-->

| Contexte                                           | Explication                                                                                                                                      |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `[...]`                                            | Littéral de tableau                                                                                                                              |
| `[expr; len]`                                      | Littéral de tableau contenant `len` copies de `expr`                                                                                             |
| `[type; len]`                                      | Type de tableau contenant `len` instances de `type`                                                                                              |
| `expr[expr]`                                       | Indexation de collection ; surchargeable (`Index`, `IndexMut`)                                                                                   |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Indexation de collection simulant un découpage de collection, en utilisant `Range`, `RangeFrom`, `RangeTo` ou `RangeFull` comme "index"          |
