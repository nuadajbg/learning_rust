# Exercice 004 -- Ecriture de tests unitaires

Le projet fourni pour cette exercice est un projet de librairie.
Dans répertoire des sources `src`, le point d'entrée se trouve donc être `lib.rs`.

## Etape 1 -- tests simples

Ecrire deux tests unitaires pour la fonction `is_even` dans `src/lib.rs`.

* Utiliser l'annotation `#[test]` sur les fonctions de test.
* Utiliser les macros `assert_eq!(x, y)` et `assert!(xx)` pour l'écriture des assertions de test.

Exécuter les tests avec la commande `cargo test`.

## Etape 2 -- module de tests

Toujours dans `lib.rs`, définir un module spécifique de test annoté (annoté avec `#[cfg(test)]`).

Dans ce nouveau module, implanter une nouvelle fonction `is_odd` d'une manière similaire à `is_even`.

Dans le nouveau module, écrire deux tests unitaires pour la fonction `is_odd`.

Exécuter les tests avec la commande `cargo test`.

## Etape 3 -- tests d'intégration

A la racine du projet, créer un répertoire `tests` et un fichier `.rs` dans celui-ci. Par exemple `tests/more.rs`.

Dans ce nouveau fichier de tests, ajouter une référence vers la fonction `is_even` :

```rust
use exercice_unit_tests::is_even;
```

Ecrire de nouveaux tests pour cette fonction, ou copier ceux existants.

Exécuter les tests avec la commande `cargo test`.

## Etape 4 -- échec de test

Modifier un test pour le mettre en échec.

Exécuter les tests avec la commande `cargo test`.

(réparer le test ensuite)

## Etape 5 -- sortie console des tests

Modifier un tests pour afficher la valeur de retour de la fonction `is_even` ou `is_odd` avec :

```rust
println!("valeur de is_even: {}", value);
```

Exécuter les tests avec la commande `cargo test`.

Que constatez-vous ?

Exécuter les tests avec la commande `cargo test -- --nocapture`.

## Etape 6 -- sélection des tests à exécuter

Exécuter les tests dont le nom commence par `xxx` avec `cargo test xxx`.

## Etape 7 -- tester l'erreur avec `should_panic`

Dans le fichier `lib.rs`, écrire deux tests unitaires pour la fonction `must_be_more_than_ten`. L'un portant sur la valeur 10 et l'autre sur la valeur 11.

* Utiliser l'annotation `#[should_panic]` sur le test pour la valeur 10.

Exécuter les tests avec la commande `cargo test`, avec et sans l'annotation `#[should_panic]`.

## Etape 8 -- tester l'erreur avec `Result<T,E>`

Dans le fichier `lib.rs`, écrire deux tests unitaires pour la fonction `safe_integer_divide`.

* Un test avec a=4, b=2
* Un test avec a=4, b=0
* Ecrire des tests avec pour type de retour `Result<(), String>` :

```rust
#[test]
fn safe_integer_divide_ok() -> Result<(), String> { /* compléter ici */ }
```
