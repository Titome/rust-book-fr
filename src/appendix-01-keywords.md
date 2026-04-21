<!--
## Appendix A: Keywords
-->

## Annexe A : les mots-clés

<!--
The following lists contain keywords that are reserved for current or future
use by the Rust language. As such, they cannot be used as identifiers (except
as raw identifiers, as we discuss in the ["Raw
Identifiers"][raw-identifiers] section). _Identifiers_ are names
of functions, variables, parameters, struct fields, modules, crates, constants,
macros, static values, attributes, types, traits, or lifetimes.
-->

Les listes suivantes contiennent des mots-clés réservés qui sont actuellement
utilisés dans le langage Rust ou qui pourraient l'être à l'avenir. De ce fait,
ils ne peuvent pas être utilisés comme identificateurs (sauf comme
identificateurs bruts, ce que nous allons voir dans la section
"[les identificateurs bruts][raw-identifiers]<!--
ignore
-->"). Les
_identificateurs_ sont les noms de fonctions, de variables, de paramètres, de
champs de structures, de modules, de crates, de constantes, de macros, de
valeurs statiques, d'attributs, de types, de traits ou de durées de vie.

[raw-identifiers]: #les-identificateurs-bruts

<!--
### Keywords Currently in Use
-->

### Les mots-clés actuellement utilisés

<!--
The following is a list of keywords currently in use, with their functionality
described.
-->

Les mots-clés suivants ont actuellement la fonction décrite.

<!--
- **`as`**: Perform primitive casting, disambiguate the specific trait
  containing an item, or rename items in `use` statements.
- **`async`**: Return a `Future` instead of blocking the current thread.
- **`await`**: Suspend execution until the result of a `Future` is ready.
- **`break`**: Exit a loop immediately.
- **`const`**: Define constant items or constant raw pointers.
- **`continue`**: Continue to the next loop iteration.
- **`crate`**: In a module path, refers to the crate root.
- **`dyn`**: Dynamic dispatch to a trait object.
- **`else`**: Fallback for `if` and `if let` control flow constructs.
- **`enum`**: Define an enumeration.
- **`extern`**: Link an external function or variable.
- **`false`**: Boolean false literal.
- **`fn`**: Define a function or the function pointer type.
- **`for`**: Loop over items from an iterator, implement a trait, or specify a
  higher ranked lifetime.
- **`if`**: Branch based on the result of a conditional expression.
- **`impl`**: Implement inherent or trait functionality.
- **`in`**: Part of `for` loop syntax.
- **`let`**: Bind a variable.
- **`loop`**: Loop unconditionally.
- **`match`**: Match a value to patterns.
- **`mod`**: Define a module.
- **`move`**: Make a closure take ownership of all its captures.
- **`mut`**: Denote mutability in references, raw pointers, or pattern bindings.
- **`pub`**: Denote public visibility in struct fields, `impl` blocks, or
  modules.
- **`ref`**: Bind by reference.
- **`return`**: Return from function.
- **`Self`**: A type alias for the type we are defining or implementing.
- **`self`**: Method subject or current module.
- **`static`**: Global variable or lifetime lasting the entire program
  execution.
- **`struct`**: Define a structure.
- **`super`**: Parent module of the current module.
- **`trait`**: Define a trait.
- **`true`**: Boolean true literal.
- **`type`**: Define a type alias or associated type.
- **`union`**: Define a [union][union]; is a keyword only when
  used in a union declaration.
- **`unsafe`**: Denote unsafe code, functions, traits, or implementations.
- **`use`**: Bring symbols into scope.
- **`where`**: Denote clauses that constrain a type.
- **`while`**: Loop conditionally based on the result of an expression.
-->

- **`as`** : effectue une transformation de type primitive, précise le trait
  qui contient un élément ou renomme des éléments dans les instructions `use`.
- **`async`** : retourne un `Future` plutôt que de bloquer la tâche en cours.
- **`await`** : met en pause l'exécution jusqu'à ce que le résultat d'un
  `Future` soit disponible.
- **`break`** : sort immédiatement d'une boucle.
- **`const`** : définit des éléments constants ou des pointeurs bruts constants.
- **`continue`** : passe directement à la prochaine itération de la boucle.
- **`crate`** : dans un chemin de module, fait référence à la racine de la
  crate.
- **`dyn`** : utilisation dynamique d'un objet trait.
- **`else`** : une branche de repli pour les structures de contrôle de flux
  `if` et `if let`.
- **`enum`** : définit une énumération.
- **`extern`** : crée un lien vers une fonction ou une variable externe.
- **`false`** : le littéral booléen qui vaut faux.
- **`fn`** : définit une fonction ou le type pointeur de fonction.
- **`for`** : crée une boucle sur les éléments d'un itérateur, implémente un
  trait ou renseigne une durée de vie de niveau supérieur.
- **`if`** : une branche liée au résultat d'une expression conditionnelle.
- **`impl`** : implémente des fonctionnalités propres à un élément ou à un
  trait.
- **`in`** : fait partie de la syntaxe de la boucle `for`.
- **`let`** : lie une variable.
- **`loop`** : fait une boucle sans condition (théoriquement infinie).
- **`match`** : compare une valeur à des motifs.
- **`mod`** : définit un module.
- **`move`** : fait en sorte qu'une fermeture prenne possession de tout ce
  qu'elle capture.
- **`mut`** : autorise la mutabilité sur des références, des pointeurs bruts ou
  des éléments issus de motifs.
- **`pub`** : autorise la visibilité publique sur des champs de structures, des
  blocs `impl` ou des modules.
- **`ref`** : lie une valeur par référence.
- **`return`** : retourne une valeur depuis une fonction.
- **`Self`** : un alias de type pour le type que nous définissons ou
  implémentons.
- **`self`** : désigne le sujet d'une méthode ou le module courant.
- **`static`** : une variable globale ou une durée de vie qui persiste tout au
  long de l'exécution du programme.
- **`struct`** : définit une structure.
- **`super`** : le module parent du module courant.
- **`trait`** : définit un trait.
- **`true`** : le littéral booléen qui vaut vrai.
- **`type`** : définit un alias de type ou un type associé.
- **`union`** : définit une [union][union]<!--
ignore
--> ; n'est un mot-clé que
  lorsqu'il est utilisé dans la déclaration d'une union.
- **`unsafe`** : autorise du code, des fonctions, des traits ou des
  implémentations non sécurisés.
- **`use`** : importe des éléments dans la portée.
- **`where`** : indique des conditions pour contraindre un type.
- **`while`** : crée une boucle en fonction du résultat d'une expression.

[union]: ../reference/items/unions.html

<!--
### Keywords Reserved for Future Use
-->

### Les mots-clés réservés pour une utilisation future

<!--
The following keywords do not yet have any functionality but are reserved by
Rust for potential future use:
-->

Les mots-clés suivants n'offrent actuellement aucune fonctionnalité mais sont
réservés par Rust pour une potentielle utilisation future :

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

<!--
### Raw Identifiers
-->

### Les identificateurs bruts

<!--
_Raw identifiers_ are the syntax that lets you use keywords where they wouldn't
normally be allowed. You use a raw identifier by prefixing a keyword with `r#`.
-->

Les _identificateurs bruts_ sont une syntaxe qui vous permet d'utiliser des
mots-clés là où ils ne devraient pas pouvoir l'être. Vous pouvez utiliser un
identificateur brut en faisant précéder un mot-clé par `r#`.

<!--
For example, `match` is a keyword. If you try to compile the following function
that uses `match` as its name:
-->

Par exemple, `match` est un mot-clé. Si vous essayez de compiler la fonction
suivante qui utilise `match` comme nom :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

<!--
you'll get this error:
-->

… vous allez obtenir l'erreur suivante :

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

<!--
The error shows that you can't use the keyword `match` as the function
identifier. To use `match` as a function name, you need to use the raw
identifier syntax, like this:
-->

L'erreur montre que vous ne pouvez pas utiliser le mot-clé `match` comme
identificateur de la fonction. Pour utiliser `match` comme nom de fonction, vous
devez utiliser la syntaxe d'identificateur brut, comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

<!--
This code will compile without any errors. Note the `r#` prefix on the function
name in its definition as well as where the function is called in `main`.
-->

Ce code va se compiler sans erreur. Remarquez le préfixe `r#` sur le nom de la
fonction dans sa définition mais aussi lorsque cette fonction est appelée dans
`main`.

<!--
Raw identifiers allow you to use any word you choose as an identifier, even if
that word happens to be a reserved keyword. This gives us more freedom to choose
identifier names, as well as lets us integrate with programs written in a
language where these words aren't keywords. In addition, raw identifiers allow
you to use libraries written in a different Rust edition than your crate uses.
For example, `try` isn't a keyword in the 2015 edition but is in the 2018, 2021,
and 2024 editions. If you depend on a library that is written using the 2015
edition and has a `try` function, you'll need to use the raw identifier syntax,
`r#try` in this case, to call that function from your code on later editions.
See [Appendix E][appendix-e] for more information on editions.
-->

Les identificateurs bruts vous permettent d'utiliser n'importe quel mot de votre
choix comme identificateur, même si ce mot est un mot-clé réservé. Cela nous
donne plus de liberté pour choisir les noms des identificateurs, et nous permet
aussi de nous intégrer avec des programmes écrits dans un langage où ces mots ne
sont pas des mots-clés. De plus, les identificateurs bruts vous permettent
d'utiliser des bibliothèques écrites dans des éditions de Rust différentes de
celle qu'utilise votre crate. Par exemple, `try` n'est pas un mot-clé dans
l'édition 2015, mais il l'est dans les éditions 2018, 2021 et 2024. Si vous
dépendez d'une bibliothèque qui a été écrite avec l'édition 2015 et qui possède
une fonction `try`, vous allez avoir besoin d'utiliser la syntaxe
d'identificateur brut `r#try` dans ce cas, pour faire appel à cette fonction
à partir de votre code dans les éditions ultérieures. Voir
[l'annexe E][appendix-e]<!--
ignore
--> pour en savoir plus sur les éditions.

[appendix-e]: appendix-05-editions.html
