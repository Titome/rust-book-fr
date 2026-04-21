<!--
## Appendix D: Useful Development Tools
-->
## Annexe D : outils de developpement utiles

<!--
In this appendix, we talk about some useful development tools that the Rust
project provides. We'll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.
-->
Dans cette annexe, nous presentons quelques outils de developpement utiles
fournis par le projet Rust. Nous verrons le formatage automatique, des moyens
rapides d'appliquer des corrections d'avertissements, un linter, et
l'integration avec les IDE.

<!--
### Automatic Formatting with `rustfmt`
-->
### Le formatage automatique avec `rustfmt`

<!--
The `rustfmt` tool reformats your code according to the community code style.
Many collaborative projects use `rustfmt` to prevent arguments about which
style to use when writing Rust: Everyone formats their code using the tool.
-->
L'outil `rustfmt` reformate votre code selon le style de code de la
communaute. De nombreux projets collaboratifs utilisent `rustfmt` pour eviter
les debats sur le style a utiliser lors de l'ecriture de Rust : tout le monde
formate son code en utilisant cet outil.

<!--
Rust installations include `rustfmt` by default, so you should already have the
programs `rustfmt` and `cargo-fmt` on your system. These two commands are
analogous to `rustc` and `cargo` in that `rustfmt` allows finer grained control
and `cargo-fmt` understands conventions of a project that uses Cargo. To format
any Cargo project, enter the following:
-->
Les installations de Rust incluent `rustfmt` par defaut, vous devriez donc
deja avoir les programmes `rustfmt` et `cargo-fmt` sur votre systeme. Ces
deux commandes sont analogues a `rustc` et `cargo` dans le sens ou `rustfmt`
offre un controle plus fin et `cargo-fmt` comprend les conventions d'un projet
qui utilise Cargo. Pour formater n'importe quel projet Cargo, saisissez la
commande suivante :

<!--
```console
$ cargo fmt
```
-->
```console
$ cargo fmt
```

<!--
Running this command reformats all the Rust code in the current crate. This
should only change the code style, not the code semantics. For more information
on `rustfmt`, see [its documentation][rustfmt].
-->
L'execution de cette commande reformate tout le code Rust dans le crate
actuel. Cela ne devrait modifier que le style du code, pas sa semantique. Pour
plus d'informations sur `rustfmt`, consultez [sa documentation][rustfmt].

<!--
### Fix Your Code with `rustfix`
-->
### Corriger votre code avec `rustfix`

<!--
The `rustfix` tool is included with Rust installations and can automatically
fix compiler warnings that have a clear way to correct the problem that's
likely what you want. You've probably seen compiler warnings before. For
example, consider this code:
-->
L'outil `rustfix` est inclus avec les installations de Rust et peut
automatiquement corriger les avertissements du compilateur qui ont une facon
claire de resoudre le probleme, ce qui est probablement ce que vous souhaitez.
Vous avez probablement deja vu des avertissements du compilateur. Par exemple,
considerez ce code :

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```
-->
```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

<!--
Here, we're defining the variable `x` as mutable, but we never actually mutate
it. Rust warns us about that:
-->
Ici, nous definissons la variable `x` comme mutable, mais nous ne la modifions
jamais en realite. Rust nous avertit a ce sujet :

<!--
```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
--> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
``` -->
```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

<!--
The warning suggests that we remove the `mut` keyword. We can automatically
apply that suggestion using the `rustfix` tool by running the command `cargo
fix`:
-->
L'avertissement suggere de supprimer le mot-cle `mut`. Nous pouvons
appliquer automatiquement cette suggestion en utilisant l'outil `rustfix` en
executant la commande `cargo fix` :

<!--
```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```
-->
```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

<!--
When we look at _src/main.rs_ again, we'll see that `cargo fix` has changed the
code:
-->
Lorsque nous regardons a nouveau _src/main.rs_, nous constatons que `cargo fix`
a modifie le code :

<!--
<span class="filename">Filename: src/main.rs</span>
-->
<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```
-->
```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

<!--
The variable `x` is now immutable, and the warning no longer appears.
-->
La variable `x` est maintenant immuable, et l'avertissement n'apparait plus.

<!--
You can also use the `cargo fix` command to transition your code between
different Rust editions. Editions are covered in [Appendix E][editions]
ignore
-->. -->
Vous pouvez egalement utiliser la commande `cargo fix` pour faire migrer votre
code entre differentes editions de Rust. Les editions sont traitees dans
l'[annexe E][editions]<!--
ignore
-->.

<!--
### More Lints with Clippy
-->
### Plus de lints avec Clippy

<!--
The Clippy tool is a collection of lints to analyze your code so that you can
catch common mistakes and improve your Rust code. Clippy is included with
standard Rust installations.
-->
L'outil Clippy est une collection de lints qui analysent votre code afin que
vous puissiez detecter les erreurs courantes et ameliorer votre code Rust.
Clippy est inclus avec les installations standard de Rust.

<!--
To run Clippy's lints on any Cargo project, enter the following:
-->
Pour executer les lints de Clippy sur n'importe quel projet Cargo, saisissez
la commande suivante :

<!--
```console
$ cargo clippy
```
-->
```console
$ cargo clippy
```

<!--
For example, say you write a program that uses an approximation of a
mathematical constant, such as pi, as this program does:
-->
Par exemple, imaginons que vous ecrivez un programme qui utilise une
approximation d'une constante mathematique, comme pi, comme le fait ce
programme :

<Listing file-name="src/main.rs">

<!--
```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```
-->
```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

<!--
Running `cargo clippy` on this project results in this error:
-->
L'execution de `cargo clippy` sur ce projet produit cette erreur :

<!--
```text
error: approximate value of `f{32, 64}::consts::PI` found
--> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
``` -->
```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

<!--
This error lets you know that Rust already has a more precise `PI` constant
defined, and that your program would be more correct if you used the constant
instead. You would then change your code to use the `PI` constant.
-->
Cette erreur vous indique que Rust dispose deja d'une constante `PI` plus
precise, et que votre programme serait plus correct si vous utilisiez cette
constante a la place. Vous modifieriez alors votre code pour utiliser la
constante `PI`.

<!--
The following code doesn't result in any errors or warnings from Clippy:
-->
Le code suivant ne produit aucune erreur ni avertissement de la part de Clippy :

<Listing file-name="src/main.rs">

<!--
```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```
-->
```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

<!--
For more information on Clippy, see [its documentation][clippy].
-->
Pour plus d'informations sur Clippy, consultez [sa documentation][clippy].

<!--
### IDE Integration Using `rust-analyzer`
-->
### Integration avec les IDE en utilisant `rust-analyzer`

<!--
To help with IDE integration, the Rust community recommends using
[`rust-analyzer`][rust-analyzer] ignore
-->. This tool is a set of
compiler-centric utilities that speak [Language Server Protocol][lsp]<!--
ignore
-->, which is a specification for IDEs and programming languages to
communicate with each other. Different clients can use `rust-analyzer`, such as
[the Rust analyzer plug-in for Visual Studio Code][vscode]. -->
Pour faciliter l'integration avec les IDE, la communaute Rust recommande
d'utiliser [`rust-analyzer`][rust-analyzer]<!--
ignore
-->. Cet outil est un
ensemble d'utilitaires centres sur le compilateur qui utilisent le
[Language Server Protocol][lsp]<!--
ignore
-->, une specification permettant
aux IDE et aux langages de programmation de communiquer entre eux. Differents
clients peuvent utiliser `rust-analyzer`, comme [le plugin Rust analyzer pour
Visual Studio Code][vscode].

<!--
Visit the `rust-analyzer` project's [home page][rust-analyzer] ignore
-->
for installation instructions, then install the language server support in your
particular IDE. Your IDE will gain capabilities such as autocompletion, jump to
definition, and inline errors. -->
Visitez la [page d'accueil][rust-analyzer]<!--
ignore
--> du projet
`rust-analyzer` pour les instructions d'installation, puis installez le support
du serveur de langage dans votre IDE. Votre IDE disposera alors de
fonctionnalites telles que l'autocompletion, la navigation vers la definition,
et l'affichage des erreurs en ligne.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
