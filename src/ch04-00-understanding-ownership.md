<!--
# Understanding Ownership
-->

# Comprendre la possession

<!--
Ownership is Rust's most unique feature and has deep implications for the rest
of the language. It enables Rust to make memory safety guarantees without
needing a garbage collector, so it's important to understand how ownership
works. In this chapter, we'll talk about ownership as well as several related
features: borrowing, slices, and how Rust lays data out in memory.
-->

La possession (ownership) est la fonctionnalite la plus distinctive de Rust et
a des implications profondes sur le reste du langage. Elle permet a Rust de
garantir la securite de la memoire sans avoir besoin d'un ramasse-miettes
(garbage collector), il est donc important de comprendre comment fonctionne la
possession. Dans ce chapitre, nous parlerons de la possession ainsi que de
plusieurs fonctionnalites associees : l'emprunt (borrowing), les slices, et la
facon dont Rust organise les donnees en memoire.
