# Exercice 003 -- Partage d'état entre thread

## Etape 1 -- Partage avec Arc et Mutex

* Créer 10 threads et attendre leur terminaison dans le thread principal.
* Chaque thread est numéroté de 0 à 9.
* Dans chacun de ces threads, incrémenter un compter commun à tous avec le numéro du thread
    * (Le total doit faire 45.)
    * Utiliser [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) et [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
* Dans le thread principal, après la terminaison des 10 threads, afficher le total.

## Etape 2 -- Partage avec AtomicXXX

* Même exercice en utilisant un structure de [`atomic`](https://doc.rust-lang.org/core/sync/atomic/index.html), par exemple `AtomicI32`.
