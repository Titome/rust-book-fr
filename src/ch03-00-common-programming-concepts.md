<!--
# Common Programming Concepts
-->

# Les concepts courants de programmation

<!--
This chapter covers concepts that appear in almost every programming language
and how they work in Rust. Many programming languages have much in common at
their core. None of the concepts presented in this chapter are unique to Rust,
but we'll discuss them in the context of Rust and explain the conventions
around using them.
-->

Ce chapitre couvre des concepts que l'on retrouve dans presque tous les langages
de programmation, et la manière dont ils fonctionnent en Rust. De nombreux
langages de programmation partagent un socle commun. Aucun des concepts
présentés dans ce chapitre n'est propre à Rust, mais nous les aborderons dans
le contexte de Rust et expliquerons les conventions qui entourent leur
utilisation.

<!--
Specifically, you'll learn about variables, basic types, functions, comments,
and control flow. These foundations will be in every Rust program, and learning
them early will give you a strong core to start from.
-->

Plus précisément, vous découvrirez les variables, les types de base, les
fonctions, les commentaires et le flux de contrôle. Ces fondamentaux se
retrouvent dans chaque programme Rust, et les apprendre tôt vous donnera une
base solide pour démarrer.

<!--
> #### Keywords
>
> The Rust language has a set of _keywords_ that are reserved for use by the
> language only, much as in other languages. Keep in mind that you cannot use
> these words as names of variables or functions. Most of the keywords have
> special meanings, and you'll be using them to do various tasks in your Rust
> programs; a few have no current functionality associated with them but have
> been reserved for functionality that might be added to Rust in the future. You
> can find the list of the keywords in [Appendix A][appendix_a] ignore
-->.
-->

> #### Mots-clés
>
> Le langage Rust possède un ensemble de _mots-clés_ qui sont réservés à
> l'usage exclusif du langage, comme dans d'autres langages. Gardez à l'esprit
> que vous ne pouvez pas utiliser ces mots comme noms de variables ou de
> fonctions. La plupart des mots-clés ont des significations particulières, et
> vous les utiliserez pour accomplir diverses tâches dans vos programmes Rust ;
> quelques-uns n'ont pas de fonctionnalité associée pour le moment mais ont été
> réservés pour des fonctionnalités qui pourraient être ajoutées à Rust dans le
> futur. Vous pouvez trouver la liste des mots-clés dans
> [l'annexe A][appendix_a]<!--
ignore
-->.

[appendix_a]: appendix-01-keywords.md
