<!--
# Error Handling
-->

# La gestion des erreurs

<!--
Errors are a fact of life in software, so Rust has a number of features for
handling situations in which something goes wrong. In many cases, Rust requires
you to acknowledge the possibility of an error and take some action before your
code will compile. This requirement makes your program more robust by ensuring
that you'll discover errors and handle them appropriately before deploying your
code to production!
-->

Les erreurs font partie de la réalité du développement logiciel, c'est pourquoi Rust dispose de nombreuses fonctionnalités pour gérer les situations où quelque chose se passe mal. Dans de nombreux cas, Rust vous oblige à reconnaître la possibilité d'une erreur et à prendre des mesures avant que votre code ne puisse compiler. Cette exigence rend votre programme plus robuste en garantissant que vous découvrirez les erreurs et les traiterez correctement avant de déployer votre code en production !

<!--
Rust groups errors into two major categories: recoverable and unrecoverable
errors. For a _recoverable error_, such as a _file not found_ error, we most
likely just want to report the problem to the user and retry the operation.
_Unrecoverable errors_ are always symptoms of bugs, such as trying to access a
location beyond the end of an array, and so we want to immediately stop the
program.
-->

Rust classe les erreurs en deux grandes catégories : les erreurs récupérables et les erreurs irrécupérables. Pour une _erreur récupérable_, comme une erreur de type _fichier introuvable_, nous voulons très probablement simplement signaler le problème à l'utilisateur et réessayer l'opération. Les _erreurs irrécupérables_ sont toujours des symptômes de bogues, comme tenter d'accéder à un emplacement au-delà de la fin d'un tableau, et dans ce cas nous voulons arrêter immédiatement le programme.

<!--
Most languages don't distinguish between these two kinds of errors and handle
both in the same way, using mechanisms such as exceptions. Rust doesn't have
exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
unrecoverable error. This chapter covers calling `panic!` first and then talks
about returning `Result<T, E>` values. Additionally, we'll explore
considerations when deciding whether to try to recover from an error or to stop
execution.
-->

La plupart des langages ne font pas la distinction entre ces deux types d'erreurs et les gèrent de la même manière, en utilisant des mécanismes tels que les exceptions. Rust ne possède pas d'exceptions. À la place, il dispose du type `Result<T, E>` pour les erreurs récupérables et de la macro `panic!` qui arrête l'exécution lorsque le programme rencontre une erreur irrécupérable. Ce chapitre traite d'abord de l'appel à `panic!`, puis aborde le renvoi de valeurs `Result<T, E>`. De plus, nous examinerons les considérations à prendre en compte pour décider s'il faut tenter de récupérer d'une erreur ou arrêter l'exécution.
