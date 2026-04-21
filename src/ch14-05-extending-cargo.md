<!--
## Extending Cargo with Custom Commands
-->

## Etendre Cargo avec des commandes personnalisees

<!--
Cargo is designed so that you can extend it with new subcommands without having
to modify it. If a binary in your `$PATH` is named `cargo-something`, you can
run it as if it were a Cargo subcommand by running `cargo something`. Custom
commands like this are also listed when you run `cargo --list`. Being able to
use `cargo install` to install extensions and then run them just like the
built-in Cargo tools is a super-convenient benefit of Cargo's design!
-->

Cargo est concu de maniere a pouvoir etre etendu avec de nouvelles sous-commandes sans avoir a le modifier. Si un binaire dans votre `$PATH` est nomme `cargo-something`, vous pouvez l'executer comme s'il s'agissait d'une sous-commande de Cargo en executant `cargo something`. Les commandes personnalisees comme celle-ci sont egalement listees lorsque vous executez `cargo --list`. Pouvoir utiliser `cargo install` pour installer des extensions puis les executer exactement comme les outils integres de Cargo est un avantage extremement pratique de la conception de Cargo !

<!--
## Summary
-->

## Resume

<!--
Sharing code with Cargo and [crates.io](https://crates.io/) ignore
--> is
part of what makes the Rust ecosystem useful for many different tasks. Rust's
standard library is small and stable, but crates are easy to share, use, and
improve on a timeline different from that of the language. Don't be shy about
sharing code that's useful to you on [crates.io](https://crates.io/)<!--
ignore
-->; it's likely that it will be useful to someone else as well!
-->

Partager du code avec Cargo et [crates.io](https://crates.io/)<!--
ignore
--> fait partie de ce qui rend l'ecosysteme Rust utile pour de nombreuses taches differentes. La bibliotheque standard de Rust est petite et stable, mais les crates sont faciles a partager, utiliser et ameliorer selon un calendrier different de celui du langage. N'hesitez pas a partager du code qui vous est utile sur [crates.io](https://crates.io/)<!--
ignore
--> ; il est probable qu'il sera egalement utile a quelqu'un d'autre !
