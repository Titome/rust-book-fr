<!--
## Building a Single-Threaded Web Server
-->

## Construire un serveur web monothread

<!--
We'll start by getting a single-threaded web server working. Before we begin,
let's look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a brief overview will give you the information you need.
-->

Nous allons commencer par faire fonctionner un serveur web monothread. Avant de commencer, examinons un rapide aperçu des protocoles impliqués dans la construction de serveurs web. Les détails de ces protocoles dépassent le cadre de ce livre, mais un bref aperçu vous donnera les informations dont vous avez besoin.

<!--
The two main protocols involved in web servers are _Hypertext Transfer
Protocol_ _(HTTP)_ and _Transmission Control Protocol_ _(TCP)_. Both protocols
are _request-response_ protocols, meaning a _client_ initiates requests and a
_server_ listens to the requests and provides a response to the client. The
contents of those requests and responses are defined by the protocols.
-->

Les deux principaux protocoles impliqués dans les serveurs web sont le _Hypertext Transfer Protocol_ _(HTTP)_ et le _Transmission Control Protocol_ _(TCP)_. Ces deux protocoles sont des protocoles de type _requête-réponse_, ce qui signifie qu'un _client_ initie des requêtes et qu'un _serveur_ écoute les requêtes et fournit une réponse au client. Le contenu de ces requêtes et réponses est défini par les protocoles.

<!--
TCP is the lower-level protocol that describes the details of how information
gets from one server to another but doesn't specify what that information is.
HTTP builds on top of TCP by defining the contents of the requests and
responses. It's technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We'll work with the
raw bytes of TCP and HTTP requests and responses.
-->

TCP est le protocole de plus bas niveau qui décrit les détails de la façon dont l'information circule d'un serveur à un autre, mais ne spécifie pas ce qu'est cette information. HTTP s'appuie sur TCP en définissant le contenu des requêtes et des réponses. Il est techniquement possible d'utiliser HTTP avec d'autres protocoles, mais dans la grande majorité des cas, HTTP envoie ses données via TCP. Nous allons travailler avec les octets bruts des requêtes et réponses TCP et HTTP.

<!--
### Listening to the TCP Connection
-->

### Écouter la connexion TCP

<!--
Our web server needs to listen to a TCP connection, so that's the first part
we'll work on. The standard library offers a `std::net` module that lets us do
this. Let's make a new project in the usual fashion:
-->

Notre serveur web doit écouter une connexion TCP, c'est donc la première partie sur laquelle nous allons travailler. La bibliothèque standard offre un module `std::net` qui nous permet de faire cela. Créons un nouveau projet de la manière habituelle :

<!--
```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```
-->

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

<!--
Now enter the code in Listing 21-1 in _src/main.rs_ to start. This code will
listen at the local address `127.0.0.1:7878` for incoming TCP streams. When it
gets an incoming stream, it will print `Connection established!`.
-->

Maintenant, entrez le code de l'encart 21-1 dans _src/main.rs_ pour commencer. Ce code écoutera à l'adresse locale `127.0.0.1:7878` les flux TCP entrants. Quand il recevra un flux entrant, il affichera `Connection established!`.

<Listing number="21-1" file-name="src/main.rs" caption="Écouter les flux entrants et afficher un message quand nous recevons un flux">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

<!--
Using `TcpListener`, we can listen for TCP connections at the address
`127.0.0.1:7878`. In the address, the section before the colon is an IP address
representing your computer (this is the same on every computer and doesn't
represent the authors' computer specifically), and `7878` is the port. We've
chosen this port for two reasons: HTTP isn't normally accepted on this port, so
our server is unlikely to conflict with any other web server you might have
running on your machine, and 7878 is _rust_ typed on a telephone.
-->

En utilisant `TcpListener`, nous pouvons écouter les connexions TCP à l'adresse `127.0.0.1:7878`. Dans l'adresse, la partie avant les deux-points est une adresse IP représentant votre ordinateur (c'est la même sur tous les ordinateurs et ne représente pas spécifiquement l'ordinateur des auteurs), et `7878` est le port. Nous avons choisi ce port pour deux raisons : HTTP n'est normalement pas accepté sur ce port, donc notre serveur a peu de chances d'entrer en conflit avec un autre serveur web que vous pourriez avoir en fonctionnement sur votre machine, et 7878 correspond à _rust_ tapé sur un téléphone.

<!--
The `bind` function in this scenario works like the `new` function in that it
will return a new `TcpListener` instance. The function is called `bind`
because, in networking, connecting to a port to listen to is known as "binding
to a port."
-->

La fonction `bind` dans ce scénario fonctionne comme la fonction `new` en ce qu'elle retourne une nouvelle instance de `TcpListener`. La fonction s'appelle `bind` parce que, en réseau, se connecter à un port pour l'écouter est connu sous le nom de « liaison à un port » (binding).

<!--
The `bind` function returns a `Result<T, E>`, which indicates that it's
possible for binding to fail, for example, if we ran two instances of our
program and so had two programs listening to the same port. Because we're
writing a basic server just for learning purposes, we won't worry about
handling these kinds of errors; instead, we use `unwrap` to stop the program if
errors happen.
-->

La fonction `bind` retourne un `Result<T, E>`, ce qui indique qu'il est possible que la liaison échoue, par exemple si nous exécutions deux instances de notre programme et avions donc deux programmes écoutant sur le même port. Comme nous écrivons un serveur basique uniquement à des fins d'apprentissage, nous ne nous soucierons pas de gérer ce type d'erreurs ; à la place, nous utilisons `unwrap` pour arrêter le programme si des erreurs surviennent.

<!--
The `incoming` method on `TcpListener` returns an iterator that gives us a
sequence of streams (more specifically, streams of type `TcpStream`). A single
_stream_ represents an open connection between the client and the server.
_Connection_ is the name for the full request and response process in which a
client connects to the server, the server generates a response, and the server
closes the connection. As such, we will read from the `TcpStream` to see what
the client sent and then write our response to the stream to send data back to
the client. Overall, this `for` loop will process each connection in turn and
produce a series of streams for us to handle.
-->

La méthode `incoming` sur `TcpListener` retourne un itérateur qui nous donne une séquence de flux (plus précisément, des flux de type `TcpStream`). Un seul _flux_ (stream) représente une connexion ouverte entre le client et le serveur. _Connexion_ est le nom du processus complet de requête et réponse dans lequel un client se connecte au serveur, le serveur génère une réponse, et le serveur ferme la connexion. Ainsi, nous lirons depuis le `TcpStream` pour voir ce que le client a envoyé, puis nous écrirons notre réponse dans le flux pour renvoyer des données au client. Globalement, cette boucle `for` traitera chaque connexion à tour de rôle et produira une série de flux à gérer.

<!--
For now, our handling of the stream consists of calling `unwrap` to terminate
our program if the stream has any errors; if there aren't any errors, the
program prints a message. We'll add more functionality for the success case in
the next listing. The reason we might receive errors from the `incoming` method
when a client connects to the server is that we're not actually iterating over
connections. Instead, we're iterating over _connection attempts_. The
connection might not be successful for a number of reasons, many of them
operating system specific. For example, many operating systems have a limit to
the number of simultaneous open connections they can support; new connection
attempts beyond that number will produce an error until some of the open
connections are closed.
-->

Pour l'instant, notre gestion du flux consiste à appeler `unwrap` pour terminer notre programme si le flux a des erreurs ; s'il n'y a pas d'erreurs, le programme affiche un message. Nous ajouterons plus de fonctionnalités pour le cas de succès dans le prochain encart. La raison pour laquelle nous pourrions recevoir des erreurs de la méthode `incoming` quand un client se connecte au serveur est que nous n'itérons pas réellement sur des connexions. Au lieu de cela, nous itérons sur des _tentatives de connexion_. La connexion pourrait ne pas réussir pour plusieurs raisons, dont beaucoup sont spécifiques au système d'exploitation. Par exemple, de nombreux systèmes d'exploitation ont une limite au nombre de connexions ouvertes simultanées qu'ils peuvent prendre en charge ; les nouvelles tentatives de connexion au-delà de ce nombre produiront une erreur jusqu'à ce que certaines des connexions ouvertes soient fermées.

<!--
Let's try running this code! Invoke `cargo run` in the terminal and then load
_127.0.0.1:7878_ in a web browser. The browser should show an error message
like "Connection reset" because the server isn't currently sending back any
data. But when you look at your terminal, you should see several messages that
were printed when the browser connected to the server!
-->

Essayons d'exécuter ce code ! Lancez `cargo run` dans le terminal puis chargez _127.0.0.1:7878_ dans un navigateur web. Le navigateur devrait afficher un message d'erreur comme « Connection reset » car le serveur ne renvoie actuellement aucune donnée. Mais quand vous regardez votre terminal, vous devriez voir plusieurs messages qui ont été affichés quand le navigateur s'est connecté au serveur !

<!--
```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```
-->

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

<!--
Sometimes you'll see multiple messages printed for one browser request; the
reason might be that the browser is making a request for the page as well as a
request for other resources, like the _favicon.ico_ icon that appears in the
browser tab.
-->

Parfois, vous verrez plusieurs messages affichés pour une seule requête du navigateur ; la raison pourrait être que le navigateur fait une requête pour la page ainsi qu'une requête pour d'autres ressources, comme l'icône _favicon.ico_ qui apparaît dans l'onglet du navigateur.

<!--
It could also be that the browser is trying to connect to the server multiple
times because the server isn't responding with any data. When `stream` goes out
of scope and is dropped at the end of the loop, the connection is closed as
part of the `drop` implementation. Browsers sometimes deal with closed
connections by retrying, because the problem might be temporary.
-->

Il se pourrait aussi que le navigateur essaie de se connecter au serveur plusieurs fois parce que le serveur ne répond avec aucune donnée. Quand `stream` sort de la portée et est libéré à la fin de la boucle, la connexion est fermée dans le cadre de l'implémentation de `drop`. Les navigateurs gèrent parfois les connexions fermées en réessayant, car le problème pourrait être temporaire.

<!--
Browsers also sometimes open multiple connections to the server without sending
any requests so that if they *do* later send requests, those requests can
happen more quickly. When this occurs, our server will see each connection,
regardless of whether there are any requests over that connection. Many
versions of Chrome-based browsers do this, for example; you can disable that
optimization by using private browsing mode or using a different browser.
-->

Les navigateurs ouvrent aussi parfois plusieurs connexions au serveur sans envoyer de requêtes afin que s'ils envoient *effectivement* des requêtes plus tard, celles-ci puissent être traitées plus rapidement. Quand cela se produit, notre serveur verra chaque connexion, qu'il y ait ou non des requêtes sur cette connexion. De nombreuses versions de navigateurs basés sur Chrome font cela, par exemple ; vous pouvez désactiver cette optimisation en utilisant le mode de navigation privée ou en utilisant un autre navigateur.

<!--
The important factor is that we've successfully gotten a handle to a TCP
connection!
-->

Le point important est que nous avons réussi à obtenir un handle vers une connexion TCP !

<!--
Remember to stop the program by pressing <kbd>ctrl</kbd>-<kbd>C</kbd> when
you're done running a particular version of the code. Then, restart the program
by invoking the `cargo run` command after you've made each set of code changes
to make sure you're running the newest code.
-->

N'oubliez pas d'arrêter le programme en appuyant sur <kbd>ctrl</kbd>-<kbd>C</kbd> quand vous avez fini d'exécuter une version particulière du code. Ensuite, redémarrez le programme en invoquant la commande `cargo run` après avoir effectué chaque ensemble de modifications du code pour vous assurer que vous exécutez le code le plus récent.

<!--
### Reading the Request
-->

### Lire la requête

<!--
Let's implement the functionality to read the request from the browser! To
separate the concerns of first getting a connection and then taking some action
with the connection, we'll start a new function for processing connections. In
this new `handle_connection` function, we'll read data from the TCP stream and
print it so that we can see the data being sent from the browser. Change the
code to look like Listing 21-2.
-->

Implémentons la fonctionnalité pour lire la requête depuis le navigateur ! Pour séparer les préoccupations entre d'abord obtenir une connexion et ensuite effectuer une action avec la connexion, nous allons créer une nouvelle fonction pour traiter les connexions. Dans cette nouvelle fonction `handle_connection`, nous lirons les données du flux TCP et les afficherons pour que nous puissions voir les données envoyées par le navigateur. Modifiez le code pour qu'il ressemble à l'encart 21-2.

<Listing number="21-2" file-name="src/main.rs" caption="Lire depuis le `TcpStream` et afficher les données">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

<!--
We bring `std::io::BufReader` and `std::io::prelude` into scope to get access
to traits and types that let us read from and write to the stream. In the `for`
loop in the `main` function, instead of printing a message that says we made a
connection, we now call the new `handle_connection` function and pass the
`stream` to it.
-->

Nous importons `std::io::BufReader` et `std::io::prelude` dans la portée pour accéder aux traits et types qui nous permettent de lire et d'écrire dans le flux. Dans la boucle `for` de la fonction `main`, au lieu d'afficher un message disant que nous avons établi une connexion, nous appelons maintenant la nouvelle fonction `handle_connection` et lui passons le `stream`.

<!--
In the `handle_connection` function, we create a new `BufReader` instance that
wraps a reference to the `stream`. The `BufReader` adds buffering by managing
calls to the `std::io::Read` trait methods for us.
-->

Dans la fonction `handle_connection`, nous créons une nouvelle instance de `BufReader` qui enveloppe une référence au `stream`. Le `BufReader` ajoute une mise en tampon en gérant les appels aux méthodes du trait `std::io::Read` pour nous.

<!--
We create a variable named `http_request` to collect the lines of the request
the browser sends to our server. We indicate that we want to collect these
lines in a vector by adding the `Vec<_>` type annotation.
-->

Nous créons une variable nommée `http_request` pour collecter les lignes de la requête que le navigateur envoie à notre serveur. Nous indiquons que nous voulons collecter ces lignes dans un vecteur en ajoutant l'annotation de type `Vec<_>`.

<!--
`BufReader` implements the `std::io::BufRead` trait, which provides the `lines`
method. The `lines` method returns an iterator of `Result<String,
std::io::Error>` by splitting the stream of data whenever it sees a newline
byte. To get each `String`, we `map` and `unwrap` each `Result`. The `Result`
might be an error if the data isn't valid UTF-8 or if there was a problem
reading from the stream. Again, a production program should handle these errors
more gracefully, but we're choosing to stop the program in the error case for
simplicity.
-->

`BufReader` implémente le trait `std::io::BufRead`, qui fournit la méthode `lines`. La méthode `lines` retourne un itérateur de `Result<String, std::io::Error>` en découpant le flux de données chaque fois qu'elle rencontre un octet de retour à la ligne. Pour obtenir chaque `String`, nous appliquons `map` et `unwrap` sur chaque `Result`. Le `Result` pourrait être une erreur si les données ne sont pas du UTF-8 valide ou s'il y a eu un problème de lecture depuis le flux. Encore une fois, un programme en production devrait gérer ces erreurs plus élégamment, mais nous choisissons d'arrêter le programme en cas d'erreur par souci de simplicité.

<!--
The browser signals the end of an HTTP request by sending two newline
characters in a row, so to get one request from the stream, we take lines until
we get a line that is the empty string. Once we've collected the lines into the
vector, we're printing them out using pretty debug formatting so that we can
take a look at the instructions the web browser is sending to our server.
-->

Le navigateur signale la fin d'une requête HTTP en envoyant deux caractères de retour à la ligne consécutifs, donc pour obtenir une requête depuis le flux, nous prenons des lignes jusqu'à obtenir une ligne qui est la chaîne vide. Une fois que nous avons collecté les lignes dans le vecteur, nous les affichons en utilisant le formatage de débogage élégant pour que nous puissions examiner les instructions que le navigateur web envoie à notre serveur.

<!--
Let's try this code! Start the program and make a request in a web browser
again. Note that we'll still get an error page in the browser, but our
program's output in the terminal will now look similar to this:
-->

Essayons ce code ! Démarrez le programme et faites une requête dans un navigateur web à nouveau. Notez que nous obtiendrons toujours une page d'erreur dans le navigateur, mais la sortie de notre programme dans le terminal ressemblera maintenant à ceci :

<!--
manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

<!--
Depending on your browser, you might get slightly different output. Now that
we're printing the request data, we can see why we get multiple connections
from one browser request by looking at the path after `GET` in the first line
of the request. If the repeated connections are all requesting _/_, we know the
browser is trying to fetch _/_ repeatedly because it's not getting a response
from our program.
-->

Selon votre navigateur, vous pourriez obtenir une sortie légèrement différente. Maintenant que nous affichons les données de la requête, nous pouvons voir pourquoi nous obtenons plusieurs connexions à partir d'une seule requête du navigateur en regardant le chemin après `GET` dans la première ligne de la requête. Si les connexions répétées demandent toutes _/_, nous savons que le navigateur essaie de récupérer _/_ de manière répétée parce qu'il ne reçoit pas de réponse de notre programme.

<!--
Let's break down this request data to understand what the browser is asking of
our program.
-->

Décomposons ces données de requête pour comprendre ce que le navigateur demande à notre programme.

<!--
Old headings. Do not remove or links may break.
-->

<a id="a-closer-look-at-an-http-request"></a>
<a id="looking-closer-at-an-http-request"></a>

<!--
### Looking More Closely at an HTTP Request
-->

### Examiner de plus près une requête HTTP

<!--
HTTP is a text-based protocol, and a request takes this format:
-->

HTTP est un protocole basé sur le texte, et une requête prend ce format :

<!--
```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```
-->

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

<!--
The first line is the _request line_ that holds information about what the
client is requesting. The first part of the request line indicates the method
being used, such as `GET` or `POST`, which describes how the client is making
this request. Our client used a `GET` request, which means it is asking for
information.
-->

La première ligne est la _ligne de requête_ qui contient les informations sur ce que le client demande. La première partie de la ligne de requête indique la méthode utilisée, comme `GET` ou `POST`, qui décrit comment le client fait cette requête. Notre client a utilisé une requête `GET`, ce qui signifie qu'il demande des informations.

<!--
The next part of the request line is _/_, which indicates the _uniform resource
identifier_ _(URI)_ the client is requesting: A URI is almost, but not quite,
the same as a _uniform resource locator_ _(URL)_. The difference between URIs
and URLs isn't important for our purposes in this chapter, but the HTTP spec
uses the term _URI_, so we can just mentally substitute _URL_ for _URI_ here.
-->

La partie suivante de la ligne de requête est _/_, qui indique l'_identifiant de ressource uniforme_ _(URI)_ que le client demande : un URI est presque, mais pas tout à fait, la même chose qu'un _localisateur de ressource uniforme_ _(URL)_. La différence entre les URI et les URL n'est pas importante pour nos besoins dans ce chapitre, mais la spécification HTTP utilise le terme _URI_, donc nous pouvons simplement substituer mentalement _URL_ à _URI_ ici.

<!--
The last part is the HTTP version the client uses, and then the request line
ends in a CRLF sequence. (_CRLF_ stands for _carriage return_ and _line feed_,
which are terms from the typewriter days!) The CRLF sequence can also be
written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed. The
_CRLF sequence_ separates the request line from the rest of the request data.
Note that when the CRLF is printed, we see a new line start rather than `\r\n`.
-->

La dernière partie est la version HTTP que le client utilise, puis la ligne de requête se termine par une séquence CRLF. (_CRLF_ signifie _carriage return_ (retour chariot) et _line feed_ (saut de ligne), qui sont des termes issus de l'époque des machines à écrire !) La séquence CRLF peut aussi s'écrire `\r\n`, où `\r` est un retour chariot et `\n` est un saut de ligne. La _séquence CRLF_ sépare la ligne de requête du reste des données de la requête. Notez que quand le CRLF est affiché, nous voyons un nouveau retour à la ligne plutôt que `\r\n`.

<!--
Looking at the request line data we received from running our program so far,
we see that `GET` is the method, _/_ is the request URI, and `HTTP/1.1` is the
version.
-->

En regardant les données de la ligne de requête que nous avons reçues en exécutant notre programme jusqu'à présent, nous voyons que `GET` est la méthode, _/_ est l'URI de la requête, et `HTTP/1.1` est la version.

<!--
After the request line, the remaining lines starting from `Host:` onward are
headers. `GET` requests have no body.
-->

Après la ligne de requête, les lignes restantes à partir de `Host:` sont les en-têtes. Les requêtes `GET` n'ont pas de corps.

<!--
Try making a request from a different browser or asking for a different
address, such as _127.0.0.1:7878/test_, to see how the request data changes.
-->

Essayez de faire une requête depuis un autre navigateur ou de demander une adresse différente, comme _127.0.0.1:7878/test_, pour voir comment les données de la requête changent.

<!--
Now that we know what the browser is asking for, let's send back some data!
-->

Maintenant que nous savons ce que le navigateur demande, renvoyons-lui quelques données !

<!--
### Writing a Response
-->

### Écrire une réponse

<!--
We're going to implement sending data in response to a client request.
Responses have the following format:
-->

Nous allons implémenter l'envoi de données en réponse à une requête client. Les réponses ont le format suivant :

<!--
```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
-->

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

<!--
The first line is a _status line_ that contains the HTTP version used in the
response, a numeric status code that summarizes the result of the request, and
a reason phrase that provides a text description of the status code. After the
CRLF sequence are any headers, another CRLF sequence, and the body of the
response.
-->

La première ligne est une _ligne de statut_ qui contient la version HTTP utilisée dans la réponse, un code de statut numérique qui résume le résultat de la requête, et une phrase de raison qui fournit une description textuelle du code de statut. Après la séquence CRLF se trouvent les en-têtes, une autre séquence CRLF, et le corps de la réponse.

<!--
Here is an example response that uses HTTP version 1.1 and has a status code of
200, an OK reason phrase, no headers, and no body:
-->

Voici un exemple de réponse qui utilise la version HTTP 1.1 et a un code de statut 200, une phrase de raison OK, pas d'en-têtes et pas de corps :

<!--
```text
HTTP/1.1 200 OK\r\n\r\n
```
-->

```text
HTTP/1.1 200 OK\r\n\r\n
```

<!--
The status code 200 is the standard success response. The text is a tiny
successful HTTP response. Let's write this to the stream as our response to a
successful request! From the `handle_connection` function, remove the
`println!` that was printing the request data and replace it with the code in
Listing 21-3.
-->

Le code de statut 200 est la réponse de succès standard. Le texte est une toute petite réponse HTTP réussie. Écrivons cela dans le flux comme notre réponse à une requête réussie ! Depuis la fonction `handle_connection`, supprimez le `println!` qui affichait les données de la requête et remplacez-le par le code de l'encart 21-3.

<Listing number="21-3" file-name="src/main.rs" caption="Écrire une toute petite réponse HTTP de succès dans le flux">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

<!--
The first new line defines the `response` variable that holds the success
message's data. Then, we call `as_bytes` on our `response` to convert the
string data to bytes. The `write_all` method on `stream` takes a `&[u8]` and
sends those bytes directly down the connection. Because the `write_all`
operation could fail, we use `unwrap` on any error result as before. Again, in
a real application, you would add error handling here.
-->

La première nouvelle ligne définit la variable `response` qui contient les données du message de succès. Ensuite, nous appelons `as_bytes` sur notre `response` pour convertir les données de la chaîne en octets. La méthode `write_all` sur `stream` prend un `&[u8]` et envoie ces octets directement à travers la connexion. Comme l'opération `write_all` pourrait échouer, nous utilisons `unwrap` sur tout résultat d'erreur comme précédemment. Encore une fois, dans une application réelle, vous ajouteriez la gestion des erreurs ici.

<!--
With these changes, let's run our code and make a request. We're no longer
printing any data to the terminal, so we won't see any output other than the
output from Cargo. When you load _127.0.0.1:7878_ in a web browser, you should
get a blank page instead of an error. You've just handcoded receiving an HTTP
request and sending a response!
-->

Avec ces changements, exécutons notre code et faisons une requête. Nous n'affichons plus aucune donnée dans le terminal, donc nous ne verrons aucune sortie autre que celle de Cargo. Quand vous chargez _127.0.0.1:7878_ dans un navigateur web, vous devriez obtenir une page vierge au lieu d'une erreur. Vous venez de coder à la main la réception d'une requête HTTP et l'envoi d'une réponse !

<!--
### Returning Real HTML
-->

### Renvoyer du vrai HTML

<!--
Let's implement the functionality for returning more than a blank page. Create
the new file _hello.html_ in the root of your project directory, not in the
_src_ directory. You can input any HTML you want; Listing 21-4 shows one
possibility.
-->

Implémentons la fonctionnalité pour renvoyer plus qu'une page vierge. Créez le nouveau fichier _hello.html_ à la racine du répertoire de votre projet, pas dans le répertoire _src_. Vous pouvez saisir le HTML que vous voulez ; l'encart 21-4 montre une possibilité.

<Listing number="21-4" file-name="hello.html" caption="Un exemple de fichier HTML à renvoyer dans une réponse">


```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

</Listing>

<!--
This is a minimal HTML5 document with a heading and some text. To return this
from the server when a request is received, we'll modify `handle_connection` as
shown in Listing 21-5 to read the HTML file, add it to the response as a body,
and send it.
-->

C'est un document HTML5 minimal avec un titre et du texte. Pour le renvoyer depuis le serveur quand une requête est reçue, nous allons modifier `handle_connection` comme montré dans l'encart 21-5 pour lire le fichier HTML, l'ajouter à la réponse comme corps, et l'envoyer.

<Listing number="21-5" file-name="src/main.rs" caption="Envoyer le contenu de *hello.html* comme corps de la réponse">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

<!--
We've added `fs` to the `use` statement to bring the standard library's
filesystem module into scope. The code for reading the contents of a file to a
string should look familiar; we used it when we read the contents of a file for
our I/O project in Listing 12-4.
-->

Nous avons ajouté `fs` à l'instruction `use` pour importer le module de système de fichiers de la bibliothèque standard dans la portée. Le code pour lire le contenu d'un fichier dans une chaîne devrait vous sembler familier ; nous l'avons utilisé quand nous avons lu le contenu d'un fichier pour notre projet d'E/S dans l'encart 12-4.

<!--
Next, we use `format!` to add the file's contents as the body of the success
response. To ensure a valid HTTP response, we add the `Content-Length` header,
which is set to the size of our response body—in this case, the size of
`hello.html`.
-->

Ensuite, nous utilisons `format!` pour ajouter le contenu du fichier comme corps de la réponse de succès. Pour garantir une réponse HTTP valide, nous ajoutons l'en-tête `Content-Length`, qui est défini à la taille du corps de notre réponse -- dans ce cas, la taille de `hello.html`.

<!--
Run this code with `cargo run` and load _127.0.0.1:7878_ in your browser; you
should see your HTML rendered!
-->

Exécutez ce code avec `cargo run` et chargez _127.0.0.1:7878_ dans votre navigateur ; vous devriez voir votre HTML affiché !

<!--
Currently, we're ignoring the request data in `http_request` and just sending
back the contents of the HTML file unconditionally. That means if you try
requesting _127.0.0.1:7878/something-else_ in your browser, you'll still get
back this same HTML response. At the moment, our server is very limited and
does not do what most web servers do. We want to customize our responses
depending on the request and only send back the HTML file for a well-formed
request to _/_.
-->

Actuellement, nous ignorons les données de la requête dans `http_request` et renvoyons simplement le contenu du fichier HTML de manière inconditionnelle. Cela signifie que si vous essayez de demander _127.0.0.1:7878/something-else_ dans votre navigateur, vous obtiendrez toujours la même réponse HTML. Pour le moment, notre serveur est très limité et ne fait pas ce que font la plupart des serveurs web. Nous voulons personnaliser nos réponses en fonction de la requête et ne renvoyer le fichier HTML que pour une requête bien formée vers _/_.

<!--
### Validating the Request and Selectively Responding
-->

### Valider la requête et répondre de manière sélective

<!--
Right now, our web server will return the HTML in the file no matter what the
client requested. Let's add functionality to check that the browser is
requesting _/_ before returning the HTML file and to return an error if the
browser requests anything else. For this we need to modify `handle_connection`,
as shown in Listing 21-6. This new code checks the content of the request
received against what we know a request for _/_ looks like and adds `if` and
`else` blocks to treat requests differently.
-->

Pour l'instant, notre serveur web renverra le HTML du fichier peu importe ce que le client a demandé. Ajoutons une fonctionnalité pour vérifier que le navigateur demande _/_ avant de renvoyer le fichier HTML et pour renvoyer une erreur si le navigateur demande autre chose. Pour cela, nous devons modifier `handle_connection`, comme montré dans l'encart 21-6. Ce nouveau code vérifie le contenu de la requête reçue par rapport à ce que nous savons être une requête pour _/_ et ajoute des blocs `if` et `else` pour traiter les requêtes différemment.

<Listing number="21-6" file-name="src/main.rs" caption="Gérer les requêtes vers */* différemment des autres requêtes">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

<!--
We're only going to be looking at the first line of the HTTP request, so rather
than reading the entire request into a vector, we're calling `next` to get the
first item from the iterator. The first `unwrap` takes care of the `Option` and
stops the program if the iterator has no items. The second `unwrap` handles the
`Result` and has the same effect as the `unwrap` that was in the `map` added in
Listing 21-2.
-->

Nous n'allons examiner que la première ligne de la requête HTTP, donc plutôt que de lire la requête entière dans un vecteur, nous appelons `next` pour obtenir le premier élément de l'itérateur. Le premier `unwrap` gère l'`Option` et arrête le programme si l'itérateur n'a pas d'éléments. Le second `unwrap` gère le `Result` et a le même effet que le `unwrap` qui était dans le `map` ajouté dans l'encart 21-2.

<!--
Next, we check the `request_line` to see if it equals the request line of a GET
request to the _/_ path. If it does, the `if` block returns the contents of our
HTML file.
-->

Ensuite, nous vérifions la `request_line` pour voir si elle correspond à la ligne de requête d'une requête GET vers le chemin _/_. Si c'est le cas, le bloc `if` renvoie le contenu de notre fichier HTML.

<!--
If the `request_line` does _not_ equal the GET request to the _/_ path, it
means we've received some other request. We'll add code to the `else` block in
a moment to respond to all other requests.
-->

Si la `request_line` ne correspond _pas_ à la requête GET vers le chemin _/_, cela signifie que nous avons reçu une autre requête. Nous ajouterons du code au bloc `else` dans un instant pour répondre à toutes les autres requêtes.

<!--
Run this code now and request _127.0.0.1:7878_; you should get the HTML in
_hello.html_. If you make any other request, such as
_127.0.0.1:7878/something-else_, you'll get a connection error like those you
saw when running the code in Listing 21-1 and Listing 21-2.
-->

Exécutez ce code maintenant et demandez _127.0.0.1:7878_ ; vous devriez obtenir le HTML de _hello.html_. Si vous faites une autre requête, comme _127.0.0.1:7878/something-else_, vous obtiendrez une erreur de connexion comme celles que vous avez vues en exécutant le code des encarts 21-1 et 21-2.

<!--
Now let's add the code in Listing 21-7 to the `else` block to return a response
with the status code 404, which signals that the content for the request was
not found. We'll also return some HTML for a page to render in the browser
indicating the response to the end user.
-->

Maintenant, ajoutons le code de l'encart 21-7 au bloc `else` pour renvoyer une réponse avec le code de statut 404, qui signale que le contenu de la requête n'a pas été trouvé. Nous renverrons aussi du HTML pour une page à afficher dans le navigateur indiquant la réponse à l'utilisateur final.

<Listing number="21-7" file-name="src/main.rs" caption="Répondre avec le code de statut 404 et une page d'erreur si autre chose que */* a été demandé">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

<!--
Here, our response has a status line with status code 404 and the reason phrase
`NOT FOUND`. The body of the response will be the HTML in the file _404.html_.
You'll need to create a _404.html_ file next to _hello.html_ for the error
page; again, feel free to use any HTML you want, or use the example HTML in
Listing 21-8.
-->

Ici, notre réponse a une ligne de statut avec le code de statut 404 et la phrase de raison `NOT FOUND`. Le corps de la réponse sera le HTML du fichier _404.html_. Vous devrez créer un fichier _404.html_ à côté de _hello.html_ pour la page d'erreur ; encore une fois, n'hésitez pas à utiliser le HTML que vous voulez, ou utilisez l'exemple de HTML de l'encart 21-8.

<Listing number="21-8" file-name="404.html" caption="Exemple de contenu pour la page à renvoyer avec toute réponse 404">


```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

<!--
With these changes, run your server again. Requesting _127.0.0.1:7878_ should
return the contents of _hello.html_, and any other request, like
_127.0.0.1:7878/foo_, should return the error HTML from _404.html_.
-->

Avec ces changements, relancez votre serveur. Demander _127.0.0.1:7878_ devrait renvoyer le contenu de _hello.html_, et toute autre requête, comme _127.0.0.1:7878/foo_, devrait renvoyer le HTML d'erreur de _404.html_.

<!--
Old headings. Do not remove or links may break.
-->

<a id="a-touch-of-refactoring"></a>

<!--
### Refactoring
-->

### Refactorisation

<!--
At the moment, the `if` and `else` blocks have a lot of repetition: They're
both reading files and writing the contents of the files to the stream. The
only differences are the status line and the filename. Let's make the code more
concise by pulling out those differences into separate `if` and `else` lines
that will assign the values of the status line and the filename to variables;
we can then use those variables unconditionally in the code to read the file
and write the response. Listing 21-9 shows the resultant code after replacing
the large `if` and `else` blocks.
-->

Pour le moment, les blocs `if` et `else` ont beaucoup de répétitions : ils lisent tous les deux des fichiers et écrivent le contenu des fichiers dans le flux. Les seules différences sont la ligne de statut et le nom du fichier. Rendons le code plus concis en extrayant ces différences dans des lignes `if` et `else` séparées qui assigneront les valeurs de la ligne de statut et du nom de fichier à des variables ; nous pourrons ensuite utiliser ces variables de manière inconditionnelle dans le code pour lire le fichier et écrire la réponse. L'encart 21-9 montre le code résultant après le remplacement des gros blocs `if` et `else`.

<Listing number="21-9" file-name="src/main.rs" caption="Refactoriser les blocs `if` et `else` pour ne contenir que le code qui diffère entre les deux cas">


```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

<!--
Now the `if` and `else` blocks only return the appropriate values for the
status line and filename in a tuple; we then use destructuring to assign these
two values to `status_line` and `filename` using a pattern in the `let`
statement, as discussed in Chapter 19.
-->

Maintenant, les blocs `if` et `else` ne renvoient que les valeurs appropriées pour la ligne de statut et le nom de fichier dans un tuple ; nous utilisons ensuite la déstructuration pour assigner ces deux valeurs à `status_line` et `filename` en utilisant un motif dans l'instruction `let`, comme discuté au chapitre 19.

<!--
The previously duplicated code is now outside the `if` and `else` blocks and
uses the `status_line` and `filename` variables. This makes it easier to see
the difference between the two cases, and it means we have only one place to
update the code if we want to change how the file reading and response writing
work. The behavior of the code in Listing 21-9 will be the same as that in
Listing 21-7.
-->

Le code précédemment dupliqué est maintenant en dehors des blocs `if` et `else` et utilise les variables `status_line` et `filename`. Cela rend plus facile de voir la différence entre les deux cas, et cela signifie que nous n'avons qu'un seul endroit où mettre à jour le code si nous voulons changer la façon dont la lecture du fichier et l'écriture de la réponse fonctionnent. Le comportement du code de l'encart 21-9 sera le même que celui de l'encart 21-7.

<!--
Awesome! We now have a simple web server in approximately 40 lines of Rust code
that responds to one request with a page of content and responds to all other
requests with a 404 response.
-->

Formidable ! Nous avons maintenant un serveur web simple en environ 40 lignes de code Rust qui répond à une requête avec une page de contenu et répond à toutes les autres requêtes avec une réponse 404.

<!--
Currently, our server runs in a single thread, meaning it can only serve one
request at a time. Let's examine how that can be a problem by simulating some
slow requests. Then, we'll fix it so that our server can handle multiple
requests at once.
-->

Actuellement, notre serveur fonctionne dans un seul thread, ce qui signifie qu'il ne peut traiter qu'une seule requête à la fois. Examinons comment cela peut poser problème en simulant des requêtes lentes. Ensuite, nous le corrigerons pour que notre serveur puisse gérer plusieurs requêtes à la fois.
