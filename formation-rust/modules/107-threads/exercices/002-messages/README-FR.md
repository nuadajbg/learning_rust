# Exercice 002 -- Passage de message entre threads

## Etape 1 -- Passage d'un seul message entre 2 threads

* Utiliser `thread::spawn` pour créer un thread à partir du thread principal.
* Utiliser [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) pour créer un couple d'émetteur/récepteur de message pour échanger entre le thread principal et le thread créé.
* Le thread créé doit recevoir un unique message (de type `String`) et l'afficher.
* Le thread principal doit envoyer le message `"Hello!"` et attendre la fin du thread créé avec `join`.

## Etape 2 -- Passage de plusieurs messages

Modifer la solution précédente pour :

* Envoyer 4 messages depuis le thread principal
    * Messages de la forme `"Hello X!"` où X est le numéro du message.
    * Attendre 1000 ms entre l'envoie de 2 message.
* Le thread écoutant doit utiliser une boucle pour recevoir un nombre infini de messages.

> Le programme ne se termine pas. Déterminez pourquoi.

## Etape 3 -- Terminaison

Modifier la solution précédente pour :

* Assurer la terminaison du programme
* Le thread écoutant doit toujours pouvoir recevoir un nombre indéterminé de messages.

> bonus : une fois une solution trouvée, trouvez en d'autres et les implanter.

## Etape 4 -- Plusieurs émetteurs

Modifier la solution pour :

* Créer un nouveau thread qui va aussi envoyer des messages, en plus du thread principal, au thread écoutant
    * Distinguer les messages envoyés par ce nouveau thread émetteur et le thread principal.
    * Regarder la doc de [`Sender`](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) pour trouver une solution.
