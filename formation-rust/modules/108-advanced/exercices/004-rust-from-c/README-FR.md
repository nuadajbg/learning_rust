# Exercice 004 -- Appel de fonctions Rust depuis C

## Objectif

Cet exercice comprend les dossiers suivant :

* `libgeo`, petite librarie Rust pour les coordonnées géographiques.
* `cuser`, petit programme C voulant utiliser cette librairie.

## Etape 1

Compiler et tester `libgeo` avec `cargo test`.

## Etape 2

* Configurer le projet `libgeo` pour produire une librairie système dynamique dans le manifest `Cargo.toml` :

```toml
[lib]
crate-type=["rlib", "dylib"]
#                   ^^^^^^^ will produce .so in linux
#           ^^^^^^ binary Rust library
```

## Etape 3

* Ajouter des annotations sur la structure `GeoCoordinate` et la fonction `offset` afin de les exporter correctement pour un programme C :
    * Ajouter `#[repr(C)]` sur `GeoCoordinate`.
    * Ajouter `#[no_mangle]` or `#[export_name = "offset"]` sur `offset`.

Compiler avec `cargo build`.

## Etape 4

* Compiler le projet `cuser` avec le script `build.sh`.
* Exécuter le programme avec le script `execute.sh`.
