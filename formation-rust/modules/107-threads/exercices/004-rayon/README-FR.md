# Exercice 004 -- Utilisation de `rayon`

## Etape 1 -- Tâches concurrentes

Utiliser la librairie [`rayon`](https://docs.rs/rayon/1.0.3/rayon/) pour exécuter de manière concurrente 100 tâches définies ainsi :

```rust
thread::sleep(Duration::from_millis(100));
println!("Finished item {}", i); // i is the task number
```

> bonus : combien de threads on été utilisés par rayon ?

## Etape 2 -- Avec un résultat

* Constuire un vecteur des nombres de 0 à 99.
* Utiliser [`rayon`](https://docs.rs/rayon/1.0.3/rayon/) pour calculer en parallèle le carré de chacun de ces nombres.
* Récupérer le résultat dans un vecteur, dans le même ordre.
