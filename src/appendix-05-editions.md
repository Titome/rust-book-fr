<!--
## Appendix E: Editions
-->

## Annexe E : Les éditions

<!--
In Chapter 1, you saw that `cargo new` adds a bit of metadata to your
_Cargo.toml_ file about an edition. This appendix talks about what that means!
-->

Au chapitre 1, vous avez vu que `cargo new` ajoute un peu de métadonnées à votre fichier _Cargo.toml_ concernant une édition. Cette annexe explique ce que cela signifie !

<!--
The Rust language and compiler have a six-week release cycle, meaning users get
a constant stream of new features. Other programming languages release larger
changes less often; Rust releases smaller updates more frequently. After a
while, all of these tiny changes add up. But from release to release, it can be
difficult to look back and say, "Wow, between Rust 1.10 and Rust 1.31, Rust has
changed a lot!"
-->

Le langage Rust et son compilateur ont un cycle de publication de six semaines, ce qui signifie que les utilisateurs reçoivent un flux constant de nouvelles fonctionnalités. D'autres langages de programmation publient des changements plus importants moins souvent ; Rust publie des mises à jour plus petites plus fréquemment. Au bout d'un moment, tous ces petits changements s'accumulent. Mais d'une version à l'autre, il peut être difficile de prendre du recul et de dire : "Wow, entre Rust 1.10 et Rust 1.31, Rust a beaucoup changé !"

<!--
Every three years or so, the Rust team produces a new Rust _edition_. Each
edition brings together the features that have landed into a clear package with
fully updated documentation and tooling. New editions ship as part of the usual
six-week release process.
-->

Environ tous les trois ans, l'équipe Rust produit une nouvelle _édition_ de Rust. Chaque édition rassemble les fonctionnalités qui ont été intégrées dans un ensemble clair avec une documentation et des outils entièrement mis à jour. Les nouvelles éditions sont publiées dans le cadre du processus habituel de publication de six semaines.

<!--
Editions serve different purposes for different people:

- For active Rust users, a new edition brings together incremental changes into
  an easy-to-understand package.
- For non-users, a new edition signals that some major advancements have
  landed, which might make Rust worth another look.
- For those developing Rust, a new edition provides a rallying point for the
  project as a whole.
-->

Les éditions servent des objectifs différents selon les personnes :

- Pour les utilisateurs actifs de Rust, une nouvelle édition rassemble les changements incrémentaux dans un ensemble facile à comprendre.
- Pour les non-utilisateurs, une nouvelle édition signale que des avancées majeures ont été intégrées, ce qui pourrait justifier de reconsidérer Rust.
- Pour ceux qui développent Rust, une nouvelle édition fournit un point de ralliement pour le projet dans son ensemble.

<!--
At the time of this writing, four Rust editions are available: Rust 2015, Rust
2018, Rust 2021, and Rust 2024. This book is written using Rust 2024 edition
idioms.
-->

Au moment de la rédaction de ce livre, quatre éditions de Rust sont disponibles : Rust 2015, Rust 2018, Rust 2021 et Rust 2024. Ce livre est écrit en utilisant les conventions de l'édition Rust 2024.

<!--
The `edition` key in _Cargo.toml_ indicates which edition the compiler should
use for your code. If the key doesn't exist, Rust uses `2015` as the edition
value for backward compatibility reasons.
-->

La clé `edition` dans _Cargo.toml_ indique quelle édition le compilateur doit utiliser pour votre code. Si la clé n'existe pas, Rust utilise `2015` comme valeur d'édition pour des raisons de compatibilité ascendante.

<!--
Each project can opt in to an edition other than the default 2015 edition.
Editions can contain incompatible changes, such as including a new keyword that
conflicts with identifiers in code. However, unless you opt in to those
changes, your code will continue to compile even as you upgrade the Rust
compiler version you use.
-->

Chaque projet peut opter pour une édition autre que l'édition 2015 par défaut. Les éditions peuvent contenir des changements incompatibles, comme l'ajout d'un nouveau mot-clé qui entre en conflit avec des identifiants dans le code. Cependant, à moins que vous n'optiez pour ces changements, votre code continuera à compiler même si vous mettez à jour la version du compilateur Rust que vous utilisez.

<!--
All Rust compiler versions support any edition that existed prior to that
compiler's release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you're using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.
-->

Toutes les versions du compilateur Rust prennent en charge toute édition qui existait avant la publication de ce compilateur, et elles peuvent lier ensemble des crates de n'importe quelles éditions prises en charge. Les changements d'édition n'affectent que la façon dont le compilateur analyse initialement le code. Par conséquent, si vous utilisez Rust 2015 et que l'une de vos dépendances utilise Rust 2018, votre projet compilera et pourra utiliser cette dépendance. La situation inverse, où votre projet utilise Rust 2018 et qu'une dépendance utilise Rust 2015, fonctionne également.

<!--
To be clear: Most features will be available on all editions. Developers using
any Rust edition will continue to see improvements as new stable releases are
made. However, in some cases, mainly when new keywords are added, some new
features might only be available in later editions. You will need to switch
editions if you want to take advantage of such features.
-->

Pour être clair : la plupart des fonctionnalités seront disponibles sur toutes les éditions. Les développeurs utilisant n'importe quelle édition de Rust continueront à voir des améliorations au fil des nouvelles versions stables. Cependant, dans certains cas, principalement lorsque de nouveaux mots-clés sont ajoutés, certaines nouvelles fonctionnalités pourraient n'être disponibles que dans les éditions ultérieures. Vous devrez changer d'édition si vous souhaitez profiter de ces fonctionnalités.

<!--
For more details, see [_The Rust Edition Guide_][edition-guide]. This is a
complete book that enumerates the differences between editions and explains how
to automatically upgrade your code to a new edition via `cargo fix`.
-->

Pour plus de détails, consultez [_The Rust Edition Guide_][edition-guide]. Il s'agit d'un livre complet qui énumère les différences entre les éditions et explique comment mettre à jour automatiquement votre code vers une nouvelle édition via `cargo fix`.

[edition-guide]: https://doc.rust-lang.org/stable/edition-guide
