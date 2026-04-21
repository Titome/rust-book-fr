<!--
Old headings. Do not remove or links may break.
-->

<a id="comparing-performance-loops-vs-iterators"></a>

<!--
## Performance in Loops vs. Iterators
-->

## Performances des boucles et des iterateurs

<!--
To determine whether to use loops or iterators, you need to know which
implementation is faster: the version of the `search` function with an explicit
`for` loop or the version with iterators.
-->

Pour determiner s'il faut utiliser des boucles ou des iterateurs, vous devez savoir quelle implementation est la plus rapide : la version de la fonction `search` avec une boucle `for` explicite ou la version avec des iterateurs.

<!--
We ran a benchmark by loading the entire contents of _The Adventures of
Sherlock Holmes_ by Sir Arthur Conan Doyle into a `String` and looking for the
word _the_ in the contents. Here are the results of the benchmark on the
version of `search` using the `for` loop and the version using iterators:
-->

Nous avons effectue un benchmark en chargeant l'integralite du contenu des _Aventures de Sherlock Holmes_ de Sir Arthur Conan Doyle dans une `String` et en recherchant le mot _the_ dans le contenu. Voici les resultats du benchmark sur la version de `search` utilisant la boucle `for` et la version utilisant les iterateurs :

<!--
```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```
-->

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

<!--
The two implementations have similar performance! We won't explain the
benchmark code here because the point is not to prove that the two versions
are equivalent but to get a general sense of how these two implementations
compare performance-wise.
-->

Les deux implementations ont des performances similaires ! Nous n'expliquerons pas le code du benchmark ici car l'objectif n'est pas de prouver que les deux versions sont equivalentes, mais d'avoir une idee generale de la comparaison de performances entre ces deux implementations.

<!--
For a more comprehensive benchmark, you should check using various texts of
various sizes as the `contents`, different words and words of different lengths
as the `query`, and all kinds of other variations. The point is this:
Iterators, although a high-level abstraction, get compiled down to roughly the
same code as if you'd written the lower-level code yourself. Iterators are one
of Rust's _zero-cost abstractions_, by which we mean that using the abstraction
imposes no additional runtime overhead. This is analogous to how Bjarne
Stroustrup, the original designer and implementor of C++, defines
zero-overhead in his 2012 ETAPS keynote presentation "Foundations of C++":
-->

Pour un benchmark plus complet, vous devriez tester avec differents textes de differentes tailles comme `contents`, differents mots et mots de differentes longueurs comme `query`, et toutes sortes d'autres variations. Le point essentiel est le suivant : les iterateurs, bien qu'etant une abstraction de haut niveau, sont compiles en un code a peu pres identique a celui que vous auriez ecrit vous-meme a plus bas niveau. Les iterateurs sont l'une des _abstractions a cout zero_ de Rust, ce qui signifie que l'utilisation de l'abstraction n'impose aucun surcout a l'execution. Cela est analogue a la maniere dont Bjarne Stroustrup, le concepteur et implementeur original du C++, definit le zero-overhead dans sa presentation ETAPS de 2012 intitulee "Foundations of C++" :

<!--
> In general, C++ implementations obey the zero-overhead principle: What you
> don't use, you don't pay for. And further: What you do use, you couldn't hand
> code any better.
-->

> En general, les implementations C++ obeissent au principe du zero-overhead : ce que vous n'utilisez pas, vous ne le payez pas. Et de plus : ce que vous utilisez, vous ne pourriez pas le coder mieux a la main.

<!--
In many cases, Rust code using iterators compiles to the same assembly you'd
write by hand. Optimizations such as loop unrolling and eliminating bounds
checking on array access apply and make the resultant code extremely efficient.
Now that you know this, you can use iterators and closures without fear! They
make code seem like it's higher level but don't impose a runtime performance
penalty for doing so.
-->

Dans de nombreux cas, le code Rust utilisant des iterateurs est compile en un assembleur identique a celui que vous ecririez a la main. Des optimisations telles que le deroulement de boucles et l'elimination des verifications de bornes lors de l'acces aux tableaux s'appliquent et rendent le code resultant extremement efficace. Maintenant que vous savez cela, vous pouvez utiliser les iterateurs et les fermetures sans crainte ! Ils donnent au code une apparence de haut niveau sans imposer de penalite de performance a l'execution.

<!--
## Summary
-->

## Resume

<!--
Closures and iterators are Rust features inspired by functional programming
language ideas. They contribute to Rust's capability to clearly express
high-level ideas at low-level performance. The implementations of closures and
iterators are such that runtime performance is not affected. This is part of
Rust's goal to strive to provide zero-cost abstractions.
-->

Les fermetures et les iterateurs sont des fonctionnalites de Rust inspirees des idees de la programmation fonctionnelle. Ils contribuent a la capacite de Rust a exprimer clairement des idees de haut niveau avec des performances de bas niveau. Les implementations des fermetures et des iterateurs sont telles que les performances a l'execution ne sont pas affectees. Cela fait partie de l'objectif de Rust de fournir des abstractions a cout zero.

<!--
Now that we've improved the expressiveness of our I/O project, let's look at
some more features of `cargo` that will help us share the project with the
world.
-->

Maintenant que nous avons ameliore l'expressivite de notre projet d'E/S, examinons d'autres fonctionnalites de `cargo` qui nous aideront a partager le projet avec le monde.
