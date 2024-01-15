# Exercice 003 -- Utiliser cargo

Sur le projet créé en Exercice 2, exécuter les commandes suivantes :

## Etape 1

Compiler en mode release (avec optimisation)

```sh
cargo build --release
```

Exécuter :

```sh
./target/release/cargo_hello
```

## Etape 2

Nettoyer les sorties de cargo :

```sh
cargo clean
```

Constater que le répertoire `target` a été supprimé

## Etape 3

Vérifier la compilation des sources sans produire de sortie :

```sh
cargo check
```

## Etape 4

Exécuter les tests unitaires

```sh
cargo test
```

## Etape 5

Générer la documentation du projet :

```sh
cargo doc
```
