<!--
# Advanced Features
-->

# Les fonctionnalités avancées

<!--
By now, you've learned the most commonly used parts of the Rust programming
language. Before we do one more project, in Chapter 21, we'll look at a few
aspects of the language you might run into every once in a while but may not
use every day. You can use this chapter as a reference for when you encounter
any unknowns. The features covered here are useful in very specific situations.
Although you might not reach for them often, we want to make sure you have a
grasp of all the features Rust has to offer.
-->

À présent, vous avez appris les parties les plus couramment utilisées du langage de programmation Rust. Avant de réaliser un dernier projet au chapitre 21, nous allons examiner quelques aspects du langage que vous pourriez rencontrer de temps en temps mais que vous n'utiliserez peut-être pas tous les jours. Vous pouvez utiliser ce chapitre comme référence lorsque vous rencontrez des inconnues. Les fonctionnalités couvertes ici sont utiles dans des situations très spécifiques. Bien que vous ne les utilisiez peut-être pas souvent, nous voulons nous assurer que vous maîtrisez toutes les fonctionnalités que Rust a à offrir.

<!--
In this chapter, we'll cover:
-->

Dans ce chapitre, nous couvrirons :

<!--
- Unsafe Rust: How to opt out of some of Rust's guarantees and take
  responsibility for manually upholding those guarantees
- Advanced traits: Associated types, default type parameters, fully qualified
  syntax, supertraits, and the newtype pattern in relation to traits
- Advanced types: More about the newtype pattern, type aliases, the never type,
  and dynamically sized types
- Advanced functions and closures: Function pointers and returning closures
- Macros: Ways to define code that defines more code at compile time
-->

- Le Rust unsafe : comment désactiver certaines garanties de Rust et prendre la responsabilité de les maintenir manuellement
- Les traits avancés : les types associés, les paramètres de type par défaut, la syntaxe pleinement qualifiée, les supertraits et le patron newtype en relation avec les traits
- Les types avancés : plus sur le patron newtype, les alias de types, le type never et les types à taille dynamique
- Les fonctions et fermetures avancées : les pointeurs de fonction et le retour de fermetures
- Les macros : des moyens de définir du code qui définit davantage de code au moment de la compilation

<!--
It's a panoply of Rust features with something for everyone! Let's dive in!
-->

C'est tout un arsenal de fonctionnalités Rust avec quelque chose pour chacun ! Allons-y !
