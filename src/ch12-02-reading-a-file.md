<!--
## Reading a File
-->

## Lire un fichier

<!--
Now we'll add functionality to read the file specified in the `file_path`
argument. First, we need a sample file to test it with: We'll use a file with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
_poem.txt_ at the root level of your project, and enter the poem "I'm Nobody!
Who are you?"
-->

Nous allons maintenant ajouter la fonctionnalité de lecture du fichier spécifié dans l'argument `file_path`. Tout d'abord, nous avons besoin d'un fichier d'exemple pour le tester : nous utiliserons un fichier contenant une petite quantité de texte sur plusieurs lignes avec quelques mots répétés. L'encart 12-3 contient un poème d'Emily Dickinson qui conviendra parfaitement ! Créez un fichier appelé _poem.txt_ à la racine de votre projet, et saisissez le poème "I'm Nobody! Who are you?"

<Listing number="12-3" file-name="poem.txt" caption="Un poème d'Emily Dickinson constitue un bon cas de test.">


```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

<!--
With the text in place, edit _src/main.rs_ and add code to read the file, as
shown in Listing 12-4.
-->

Avec le texte en place, modifiez _src/main.rs_ et ajoutez du code pour lire le fichier, comme illustré dans l'encart 12-4.

<Listing number="12-4" file-name="src/main.rs" caption="Lecture du contenu du fichier spécifié par le second argument">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

<!--
First, we bring in a relevant part of the standard library with a `use`
statement: We need `std::fs` to handle files.
-->

Tout d'abord, nous importons une partie pertinente de la bibliothèque standard avec une instruction `use` : nous avons besoin de `std::fs` pour manipuler les fichiers.

<!--
In `main`, the new statement `fs::read_to_string` takes the `file_path`, opens
that file, and returns a value of type `std::io::Result<String>` that contains
the file's contents.
-->

Dans `main`, la nouvelle instruction `fs::read_to_string` prend le `file_path`, ouvre ce fichier, et renvoie une valeur de type `std::io::Result<String>` qui contient le contenu du fichier.

<!--
After that, we again add a temporary `println!` statement that prints the value
of `contents` after the file is read so that we can check that the program is
working so far.
-->

Ensuite, nous ajoutons à nouveau une instruction `println!` temporaire qui affiche la valeur de `contents` après la lecture du fichier afin de vérifier que le programme fonctionne jusqu'ici.

<!--
Let's run this code with any string as the first command line argument (because
we haven't implemented the searching part yet) and the _poem.txt_ file as the
second argument:
-->

Exécutons ce code avec n'importe quelle chaîne comme premier argument de ligne de commande (car nous n'avons pas encore implémenté la partie recherche) et le fichier _poem.txt_ comme second argument :


```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

<!--
Great! The code read and then printed the contents of the file. But the code
has a few flaws. At the moment, the `main` function has multiple
responsibilities: Generally, functions are clearer and easier to maintain if
each function is responsible for only one idea. The other problem is that we're
not handling errors as well as we could. The program is still small, so these
flaws aren't a big problem, but as the program grows, it will be harder to fix
them cleanly. It's a good practice to begin refactoring early on when
developing a program because it's much easier to refactor smaller amounts of
code. We'll do that next.
-->

Parfait ! Le code a lu puis affiché le contenu du fichier. Mais le code présente quelques défauts. Pour le moment, la fonction `main` a plusieurs responsabilités : en général, les fonctions sont plus claires et plus faciles à maintenir si chaque fonction n'est responsable que d'une seule chose. L'autre problème est que nous ne gérons pas les erreurs aussi bien que nous le pourrions. Le programme est encore petit, donc ces défauts ne constituent pas un gros problème, mais à mesure que le programme grandit, il sera plus difficile de les corriger proprement. C'est une bonne pratique de commencer le refactoring tôt lors du développement d'un programme, car il est beaucoup plus facile de remanier de petites quantités de code. C'est ce que nous ferons ensuite.
