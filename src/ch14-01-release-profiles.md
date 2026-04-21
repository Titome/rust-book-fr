<!--
## Customizing Builds with Release Profiles
-->

## Personnaliser les compilations avec les profils de publication

<!--
In Rust, _release profiles_ are predefined, customizable profiles with
different configurations that allow a programmer to have more control over
various options for compiling code. Each profile is configured independently of
the others.
-->

En Rust, les _profils de publication_ (release profiles) sont des profils predefinis et personnalisables avec differentes configurations qui permettent a un programmeur d'avoir plus de controle sur les differentes options de compilation du code. Chaque profil est configure independamment des autres.

<!--
Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo
build`, and the `release` profile Cargo uses when you run `cargo build
--release`. The `dev` profile is defined with good defaults for development,
and the `release` profile has good defaults for release builds.
-->

Cargo a deux profils principaux : le profil `dev` que Cargo utilise lorsque vous executez `cargo build`, et le profil `release` que Cargo utilise lorsque vous executez `cargo build --release`. Le profil `dev` est defini avec de bons parametres par defaut pour le developpement, et le profil `release` a de bons parametres par defaut pour les compilations de publication.

<!--
These profile names might be familiar from the output of your builds:
-->

Ces noms de profils vous sont peut-etre familiers d'apres la sortie de vos compilations :

<!--
manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

<!--
```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

<!--
The `dev` and `release` are these different profiles used by the compiler.
-->

`dev` et `release` sont ces differents profils utilises par le compilateur.

<!--
Cargo has default settings for each of the profiles that apply when you haven't
explicitly added any `[profile.*]` sections in the project's _Cargo.toml_ file.
By adding `[profile.*]` sections for any profile you want to customize, you
override any subset of the default settings. For example, here are the default
values for the `opt-level` setting for the `dev` and `release` profiles:
-->

Cargo a des parametres par defaut pour chacun des profils qui s'appliquent lorsque vous n'avez explicitement ajoute aucune section `[profile.*]` dans le fichier _Cargo.toml_ du projet. En ajoutant des sections `[profile.*]` pour tout profil que vous souhaitez personnaliser, vous remplacez n'importe quel sous-ensemble des parametres par defaut. Par exemple, voici les valeurs par defaut pour le parametre `opt-level` des profils `dev` et `release` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
-->

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

<!--
The `opt-level` setting controls the number of optimizations Rust will apply to
your code, with a range of 0 to 3. Applying more optimizations extends
compiling time, so if you're in development and compiling your code often,
you'll want fewer optimizations to compile faster even if the resultant code
runs slower. The default `opt-level` for `dev` is therefore `0`. When you're
ready to release your code, it's best to spend more time compiling. You'll only
compile in release mode once, but you'll run the compiled program many times,
so release mode trades longer compile time for code that runs faster. That is
why the default `opt-level` for the `release` profile is `3`.
-->

Le parametre `opt-level` controle le nombre d'optimisations que Rust appliquera a votre code, avec une plage de 0 a 3. Appliquer plus d'optimisations allonge le temps de compilation, donc si vous etes en developpement et que vous compilez souvent votre code, vous voudrez moins d'optimisations pour compiler plus rapidement meme si le code resultant s'execute plus lentement. Le `opt-level` par defaut pour `dev` est donc `0`. Lorsque vous etes pret a publier votre code, il est preferable de passer plus de temps a compiler. Vous ne compilerez en mode release qu'une seule fois, mais vous executerez le programme compile de nombreuses fois, donc le mode release echange un temps de compilation plus long contre un code qui s'execute plus rapidement. C'est pourquoi le `opt-level` par defaut pour le profil `release` est `3`.

<!--
You can override a default setting by adding a different value for it in
_Cargo.toml_. For example, if we want to use optimization level 1 in the
development profile, we can add these two lines to our project's _Cargo.toml_
file:
-->

Vous pouvez remplacer un parametre par defaut en ajoutant une valeur differente dans _Cargo.toml_. Par exemple, si nous voulons utiliser le niveau d'optimisation 1 dans le profil de developpement, nous pouvons ajouter ces deux lignes au fichier _Cargo.toml_ de notre projet :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[profile.dev]
opt-level = 1
```
-->

```toml
[profile.dev]
opt-level = 1
```

<!--
This code overrides the default setting of `0`. Now when we run `cargo build`,
Cargo will use the defaults for the `dev` profile plus our customization to
`opt-level`. Because we set `opt-level` to `1`, Cargo will apply more
optimizations than the default, but not as many as in a release build.
-->

Ce code remplace le parametre par defaut de `0`. Maintenant, lorsque nous executons `cargo build`, Cargo utilisera les parametres par defaut du profil `dev` plus notre personnalisation de `opt-level`. Parce que nous avons defini `opt-level` a `1`, Cargo appliquera plus d'optimisations que le parametre par defaut, mais pas autant que dans une compilation de publication.

<!--
For the full list of configuration options and defaults for each profile, see
[Cargo's documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
-->

Pour la liste complete des options de configuration et des valeurs par defaut pour chaque profil, consultez [la documentation de Cargo](https://doc.rust-lang.org/cargo/reference/profiles.html).
