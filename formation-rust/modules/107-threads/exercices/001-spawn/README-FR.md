# Exercice 001 -- Lancer plusieurs threads

Dans le cours nous avons vu comment lancer un thread :

```rust
use std::thread;
thread::spawn(|| {});
```

## Etape 1

Dans un premier temps, faites une boucle pour lancer 10 threads numérotés de 0 à 9.

* Le thread 0 doit attendre 0 milli-secondes avant d'écrire `Thread 0 has waited 0 ms`.
* Le thread 1 doit attendre 100 milli-secondes avant d'écrire `Thread 1 has waited 100 ms`.
* etc.

Utilisez la fonction [`thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html) pour attendre.

* Compilez et exécutez avec `cargo run`.
* Que constatez-vous ?

## Etape 2

Attendre la fin de tous les threads lancés avec la méthode [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join).

* Accumulez les handles des threads lancés dans un vecteur.
* Puis, consommez ces handles avec [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) pour attendre la fin des threads.
    * Utilisez [`into_iter()`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter) pour parcourir le vecteur en le consommant.
