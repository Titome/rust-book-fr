<!--
# Writing Automated Tests
-->

# Ecrire des tests automatisés

<!--
In his 1972 essay "The Humble Programmer," Edsger W. Dijkstra said that "program
testing can be a very effective way to show the presence of bugs, but it is
hopelessly inadequate for showing their absence." That doesn't mean we shouldn't
try to test as much as we can!
-->

Dans son essai de 1972 "The Humble Programmer", Edsger W. Dijkstra a dit que "les tests de programmes peuvent etre un moyen tres efficace de montrer la presence de bogues, mais ils sont desespérement insuffisants pour prouver leur absence." Cela ne signifie pas que nous ne devrions pas essayer de tester autant que possible !

<!--
_Correctness_ in our programs is the extent to which our code does what we
intend it to do. Rust is designed with a high degree of concern about the
correctness of programs, but correctness is complex and not easy to prove.
Rust's type system shoulders a huge part of this burden, but the type system
cannot catch everything. As such, Rust includes support for writing automated
software tests.
-->

La _correction_ de nos programmes correspond a la mesure dans laquelle notre code fait ce que nous voulons qu'il fasse. Rust est concu avec un souci eleve de la correction des programmes, mais la correction est complexe et difficile a prouver. Le systeme de types de Rust prend en charge une grande partie de ce fardeau, mais le systeme de types ne peut pas tout detecter. C'est pourquoi Rust inclut la possibilite d'ecrire des tests logiciels automatises.

<!--
Say we write a function `add_two` that adds 2 to whatever number is passed to
it. This function's signature accepts an integer as a parameter and returns an
integer as a result. When we implement and compile that function, Rust does all
the type checking and borrow checking that you've learned so far to ensure
that, for instance, we aren't passing a `String` value or an invalid reference
to this function. But Rust _can't_ check that this function will do precisely
what we intend, which is return the parameter plus 2 rather than, say, the
parameter plus 10 or the parameter minus 50! That's where tests come in.
-->

Imaginons que nous ecrivions une fonction `add_two` qui ajoute 2 a n'importe quel nombre qui lui est passe. La signature de cette fonction accepte un entier en parametre et retourne un entier en resultat. Quand nous implementons et compilons cette fonction, Rust effectue toutes les verifications de types et d'emprunts que vous avez apprises jusqu'ici pour s'assurer que, par exemple, nous ne passons pas une valeur `String` ou une reference invalide a cette fonction. Mais Rust _ne peut pas_ verifier que cette fonction fera precisement ce que nous voulons, c'est-a-dire retourner le parametre plus 2 plutot que, disons, le parametre plus 10 ou le parametre moins 50 ! C'est la que les tests entrent en jeu.

<!--
We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, the returned value is `5`. We can run these tests whenever
we make changes to our code to make sure any existing correct behavior has not
changed.
-->

Nous pouvons ecrire des tests qui verifient, par exemple, que lorsque nous passons `3` a la fonction `add_two`, la valeur retournee est `5`. Nous pouvons executer ces tests chaque fois que nous modifions notre code pour nous assurer que tout comportement correct existant n'a pas change.

<!--
Testing is a complex skill: Although we can't cover in one chapter every detail
about how to write good tests, in this chapter we will discuss the mechanics of
Rust's testing facilities. We'll talk about the annotations and macros
available to you when writing your tests, the default behavior and options
provided for running your tests, and how to organize tests into unit tests and
integration tests.
-->

Tester est une competence complexe : bien que nous ne puissions pas couvrir dans un seul chapitre chaque detail sur la maniere d'ecrire de bons tests, dans ce chapitre nous aborderons les mecanismes des outils de test de Rust. Nous parlerons des annotations et des macros disponibles lors de l'ecriture de vos tests, du comportement par defaut et des options fournies pour l'execution de vos tests, et de la facon d'organiser les tests en tests unitaires et tests d'integration.
