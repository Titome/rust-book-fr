<!--
## Appendix G - How Rust is Made and "Nightly Rust"
-->

## Annexe G - Comment Rust est conçu et "Rust Nightly"

<!--
This appendix is about how Rust is made and how that affects you as a Rust
developer.
-->

Cette annexe explique comment Rust est conçu et comment cela vous affecte en tant que développeur Rust.

<!--
### Stability Without Stagnation
-->

### La stabilité sans la stagnation

<!--
As a language, Rust cares a _lot_ about the stability of your code. We want
Rust to be a rock-solid foundation you can build on, and if things were
constantly changing, that would be impossible. At the same time, if we can't
experiment with new features, we may not find out important flaws until after
their release, when we can no longer change things.
-->

En tant que langage, Rust se soucie _énormément_ de la stabilité de votre code. Nous voulons que Rust soit une fondation solide comme le roc sur laquelle vous pouvez construire, et si les choses changeaient constamment, ce serait impossible. En même temps, si nous ne pouvons pas expérimenter de nouvelles fonctionnalités, nous risquons de ne découvrir des défauts importants qu'après leur publication, lorsqu'il n'est plus possible de les modifier.

<!--
Our solution to this problem is what we call "stability without stagnation",
and our guiding principle is this: you should never have to fear upgrading to a
new version of stable Rust. Each upgrade should be painless, but should also
bring you new features, fewer bugs, and faster compile times.
-->

Notre solution à ce problème est ce que nous appelons "la stabilité sans la stagnation", et notre principe directeur est le suivant : vous ne devriez jamais avoir à craindre de passer à une nouvelle version de Rust stable. Chaque mise à jour devrait être indolore, mais devrait aussi vous apporter de nouvelles fonctionnalités, moins de bogues et des temps de compilation plus rapides.

<!--
### Choo, Choo! Release Channels and Riding the Trains
-->

### Tchou, tchou ! Les canaux de publication et le modèle du train

<!--
Rust development operates on a _train schedule_. That is, all development is
done in the main branch of the Rust repository. Releases follow a software
release train model, which has been used by Cisco IOS and other software
projects. There are three _release channels_ for Rust:

- Nightly
- Beta
- Stable
-->

Le développement de Rust fonctionne selon un _calendrier de train_. C'est-à-dire que tout le développement est effectué dans la branche principale du dépôt Rust. Les publications suivent un modèle de train de publication logiciel, qui a été utilisé par Cisco IOS et d'autres projets logiciels. Il existe trois _canaux de publication_ pour Rust :

- Nightly
- Beta
- Stable

<!--
Most Rust developers primarily use the stable channel, but those who want to
try out experimental new features may use nightly or beta.
-->

La plupart des développeurs Rust utilisent principalement le canal stable, mais ceux qui veulent essayer de nouvelles fonctionnalités expérimentales peuvent utiliser nightly ou beta.

<!--
Here's an example of how the development and release process works: let's
assume that the Rust team is working on the release of Rust 1.5. That release
happened in December of 2015, but it will provide us with realistic version
numbers. A new feature is added to Rust: a new commit lands on the main
branch. Each night, a new nightly version of Rust is produced. Every day is a
release day, and these releases are created by our release infrastructure
automatically. So as time passes, our releases look like this, once a night:
-->

Voici un exemple du fonctionnement du processus de développement et de publication : supposons que l'équipe Rust travaille sur la publication de Rust 1.5. Cette publication a eu lieu en décembre 2015, mais elle nous fournira des numéros de version réalistes. Une nouvelle fonctionnalité est ajoutée à Rust : un nouveau commit arrive sur la branche principale. Chaque nuit, une nouvelle version nightly de Rust est produite. Chaque jour est un jour de publication, et ces publications sont créées automatiquement par notre infrastructure de publication. Ainsi, au fil du temps, nos publications ressemblent à ceci, une fois par nuit :

<!--
```text
nightly: * - - * - - *
```
-->

```text
nightly: * - - * - - *
```

<!--
Every six weeks, it's time to prepare a new release! The `beta` branch of the
Rust repository branches off from the main branch used by nightly. Now,
there are two releases:
-->

Toutes les six semaines, il est temps de préparer une nouvelle publication ! La branche `beta` du dépôt Rust se sépare de la branche principale utilisée par nightly. Maintenant, il y a deux publications :

<!--
```text
nightly: * - - * - - *
                     |
beta:                *
```
-->

```text
nightly: * - - * - - *
                     |
beta:                *
```

<!--
Most Rust users do not use beta releases actively, but test against beta in
their CI system to help Rust discover possible regressions. In the meantime,
there's still a nightly release every night:
-->

La plupart des utilisateurs de Rust n'utilisent pas activement les versions beta, mais testent avec la version beta dans leur système d'intégration continue pour aider Rust à découvrir d'éventuelles régressions. Entre-temps, il y a toujours une publication nightly chaque nuit :

<!--
```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```
-->

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

<!--
Let's say a regression is found. Good thing we had some time to test the beta
release before the regression snuck into a stable release! The fix is applied
to the main branch, so that nightly is fixed, and then the fix is backported to
the `beta` branch, and a new release of beta is produced:
-->

Disons qu'une régression est trouvée. Heureusement que nous avons eu le temps de tester la version beta avant que la régression ne se glisse dans une version stable ! Le correctif est appliqué à la branche principale, de sorte que nightly est corrigé, puis le correctif est rétroporté sur la branche `beta`, et une nouvelle version beta est produite :

<!--
```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```
-->

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

<!--
Six weeks after the first beta was created, it's time for a stable release! The
`stable` branch is produced from the `beta` branch:
-->

Six semaines après la création de la première beta, il est temps pour une publication stable ! La branche `stable` est produite à partir de la branche `beta` :

<!--
```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```
-->

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

<!--
Hooray! Rust 1.5 is done! However, we've forgotten one thing: because the six
weeks have gone by, we also need a new beta of the _next_ version of Rust, 1.6.
So after `stable` branches off of `beta`, the next version of `beta` branches
off of `nightly` again:
-->

Hourra ! Rust 1.5 est terminé ! Cependant, nous avons oublié une chose : comme les six semaines se sont écoulées, nous avons aussi besoin d'une nouvelle beta de la _prochaine_ version de Rust, la 1.6. Donc après que `stable` se sépare de `beta`, la prochaine version de `beta` se sépare à nouveau de `nightly` :

<!--
```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```
-->

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

<!--
This is called the "train model" because every six weeks, a release "leaves the
station", but still has to take a journey through the beta channel before it
arrives as a stable release.
-->

C'est ce qu'on appelle le "modèle du train" car toutes les six semaines, une publication "quitte la gare", mais doit encore effectuer un voyage à travers le canal beta avant d'arriver en tant que publication stable.

<!--
Rust releases every six weeks, like clockwork. If you know the date of one Rust
release, you can know the date of the next one: it's six weeks later. A nice
aspect of having releases scheduled every six weeks is that the next train is
coming soon. If a feature happens to miss a particular release, there's no need
to worry: another one is happening in a short time! This helps reduce pressure
to sneak possibly unpolished features in close to the release deadline.
-->

Rust publie une nouvelle version toutes les six semaines, comme une horloge. Si vous connaissez la date d'une publication de Rust, vous pouvez connaître la date de la suivante : c'est six semaines plus tard. Un avantage agréable d'avoir des publications programmées toutes les six semaines est que le prochain train arrive bientôt. Si une fonctionnalité manque une publication particulière, il n'y a pas lieu de s'inquiéter : une autre arrivera dans peu de temps ! Cela aide à réduire la pression pour glisser des fonctionnalités potentiellement non finalisées juste avant la date limite de publication.

<!--
Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it's easy to upgrade to: if a beta release doesn't
work as expected, you can report it to the team and get it fixed before the
next stable release happens! Breakage in a beta release is relatively rare, but
`rustc` is still a piece of software, and bugs do exist.
-->

Grâce à ce processus, vous pouvez toujours essayer la prochaine version de Rust et vérifier par vous-même qu'il est facile de la mettre à jour : si une version beta ne fonctionne pas comme prévu, vous pouvez le signaler à l'équipe et obtenir un correctif avant la prochaine publication stable ! Les problèmes dans une version beta sont relativement rares, mais `rustc` reste un logiciel, et les bogues existent.

<!--
### Maintenance time
-->

### Durée de maintenance

<!--
The Rust project supports the most recent stable version. When a new stable
version is released, the old version reaches its end of life (EOL). This means
each version is supported for six weeks.
-->

Le projet Rust prend en charge la version stable la plus récente. Lorsqu'une nouvelle version stable est publiée, l'ancienne version atteint sa fin de vie (EOL). Cela signifie que chaque version est prise en charge pendant six semaines.

<!--
### Unstable Features
-->

### Les fonctionnalités instables

<!--
There's one more catch with this release model: unstable features. Rust uses a
technique called "feature flags" to determine what features are enabled in a
given release. If a new feature is under active development, it lands on the
main branch, and therefore, in nightly, but behind a _feature flag_. If you, as
a user, wish to try out the work-in-progress feature, you can, but you must be
using a nightly release of Rust and annotate your source code with the
appropriate flag to opt in.
-->

Il y a un autre aspect important de ce modèle de publication : les fonctionnalités instables. Rust utilise une technique appelée "feature flags" pour déterminer quelles fonctionnalités sont activées dans une publication donnée. Si une nouvelle fonctionnalité est en cours de développement actif, elle arrive sur la branche principale, et donc dans nightly, mais derrière un _feature flag_. Si vous, en tant qu'utilisateur, souhaitez essayer la fonctionnalité en cours de développement, vous le pouvez, mais vous devez utiliser une version nightly de Rust et annoter votre code source avec le flag approprié pour l'activer.

<!--
If you're using a beta or stable release of Rust, you can't use any feature
flags. This is the key that allows us to get practical use with new features
before we declare them stable forever. Those who wish to opt into the bleeding
edge can do so, and those who want a rock-solid experience can stick with
stable and know that their code won't break. Stability without stagnation.
-->

Si vous utilisez une version beta ou stable de Rust, vous ne pouvez utiliser aucun feature flag. C'est la clé qui nous permet d'obtenir une utilisation pratique des nouvelles fonctionnalités avant de les déclarer stables pour toujours. Ceux qui souhaitent opter pour les toutes dernières nouveautés peuvent le faire, et ceux qui veulent une expérience solide comme le roc peuvent rester sur stable en sachant que leur code ne cassera pas. La stabilité sans la stagnation.

<!--
This book only contains information about stable features, as in-progress
features are still changing, and surely they'll be different between when this
book was written and when they get enabled in stable builds. You can find
documentation for nightly-only features online.
-->

Ce livre ne contient que des informations sur les fonctionnalités stables, car les fonctionnalités en cours de développement changent encore, et elles seront certainement différentes entre le moment où ce livre a été écrit et celui où elles seront activées dans les versions stables. Vous pouvez trouver la documentation des fonctionnalités réservées à nightly en ligne.

<!--
### Rustup and the Role of Rust Nightly
-->

### Rustup et le rôle de Rust Nightly

<!--
Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis. By default, you'll have stable Rust installed. To
install nightly, for example:
-->

Rustup facilite le passage entre les différents canaux de publication de Rust, que ce soit globalement ou par projet. Par défaut, vous aurez Rust stable installé. Pour installer nightly, par exemple :

<!--
```console
$ rustup toolchain install nightly
```
-->

```console
$ rustup toolchain install nightly
```

<!--
You can see all of the _toolchains_ (releases of Rust and associated
components) you have installed with `rustup` as well. Here's an example on one
of your authors' Windows computer:
-->

Vous pouvez également voir toutes les _toolchains_ (versions de Rust et composants associés) que vous avez installées avec `rustup`. Voici un exemple sur l'ordinateur Windows de l'un des auteurs :

<!--
```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```
-->

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

<!--
As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project's directory to set the
nightly toolchain as the one `rustup` should use when you're in that directory:
-->

Comme vous pouvez le voir, la toolchain stable est celle par défaut. La plupart des utilisateurs de Rust utilisent stable la plupart du temps. Vous pourriez vouloir utiliser stable la plupart du temps, mais utiliser nightly sur un projet spécifique, parce qu'une fonctionnalité de pointe vous intéresse. Pour ce faire, vous pouvez utiliser `rustup override` dans le répertoire de ce projet pour définir la toolchain nightly comme celle que `rustup` doit utiliser lorsque vous êtes dans ce répertoire :

<!--
```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```
-->

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

<!--
Now, every time you call `rustc` or `cargo` inside of
_~/projects/needs-nightly_, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust. This comes in handy when you
have a lot of Rust projects!
-->

Désormais, chaque fois que vous appelez `rustc` ou `cargo` à l'intérieur de _~/projects/needs-nightly_, `rustup` s'assurera que vous utilisez Rust nightly, plutôt que votre Rust stable par défaut. C'est très pratique quand vous avez beaucoup de projets Rust !

<!--
### The RFC Process and Teams
-->

### Le processus RFC et les équipes

<!--
So how do you learn about these new features? Rust's development model follows
a _Request For Comments (RFC) process_. If you'd like an improvement in Rust,
you can write up a proposal, called an RFC.
-->

Alors comment se renseigner sur ces nouvelles fonctionnalités ? Le modèle de développement de Rust suit un _processus de Request For Comments (RFC)_. Si vous souhaitez une amélioration dans Rust, vous pouvez rédiger une proposition, appelée RFC.

<!--
Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many topic subteams. There's
a full list of the teams [on Rust's website](https://www.rust-lang.org/governance), which includes teams for
each area of the project: language design, compiler implementation,
infrastructure, documentation, and more. The appropriate team reads the
proposal and the comments, writes some comments of their own, and eventually,
there's consensus to accept or reject the feature.
-->

N'importe qui peut écrire des RFC pour améliorer Rust, et les propositions sont examinées et discutées par l'équipe Rust, qui est composée de nombreuses sous-équipes thématiques. Il y a une liste complète des équipes [sur le site web de Rust](https://www.rust-lang.org/governance), qui inclut des équipes pour chaque domaine du projet : conception du langage, implémentation du compilateur, infrastructure, documentation, et plus encore. L'équipe appropriée lit la proposition et les commentaires, rédige ses propres commentaires, et finalement, un consensus est atteint pour accepter ou rejeter la fonctionnalité.

<!--
If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be the
person who proposed the feature in the first place! When the implementation is
ready, it lands on the main branch behind a feature gate, as we discussed in
the ["Unstable Features"](#unstable-features) ignore
--> section.
-->

Si la fonctionnalité est acceptée, une issue est ouverte sur le dépôt Rust, et quelqu'un peut l'implémenter. La personne qui l'implémente peut très bien ne pas être celle qui a proposé la fonctionnalité en premier lieu ! Lorsque l'implémentation est prête, elle arrive sur la branche principale derrière un feature gate, comme nous l'avons discuté dans la section ["Les fonctionnalités instables"](#les-fonctionnalités-instables)<!--
ignore
-->.

<!--
After some time, once Rust developers who use nightly releases have been able
to try out the new feature, team members will discuss the feature, how it's
worked out on nightly, and decide if it should make it into stable Rust or not.
If the decision is to move forward, the feature gate is removed, and the
feature is now considered stable! It rides the trains into a new stable release
of Rust.
-->

Après un certain temps, une fois que les développeurs Rust qui utilisent les versions nightly ont pu essayer la nouvelle fonctionnalité, les membres de l'équipe discutent de la fonctionnalité, de son fonctionnement sur nightly, et décident si elle doit intégrer Rust stable ou non. Si la décision est d'aller de l'avant, le feature gate est supprimé, et la fonctionnalité est désormais considérée comme stable ! Elle prend le train vers une nouvelle publication stable de Rust.
