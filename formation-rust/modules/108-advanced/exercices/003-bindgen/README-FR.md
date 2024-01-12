# Exercice 003 -- Appel de fonctions C depuis Rust

## Objectif

Cet exercice comprend les dossiers suivant :

* `libgeo`, petite librarie C pour les coordonnées géographiques.
* `libgeo-rust`, librairie Rust devant embarquer directement un version de la librairie géographique.
* `libgeo-wrapper`, librairie Rust servant de wrapper pour la librairie `libgeo`.

Le but de cet exercice est de réussir à encapsuler la librairie `libgeo`, de deux manières différentes, afin de pouvoir l'appeller depuis du code Rust.

## Etape 1 -- `libgeo-rust`

Dans cette étape, nous nous concentrons uniquement sur le projet `libgeo-rust`. La logique de ce dernier est qu'il s'agit d'une librairie Rust dont tout ou partie du code est écrit en C. Les sources de ce projet contiennent donc :

> * libgeo-rust
>    * Cargo.toml
>    * src
>        * libgeo.h
>        * libgeo.c
>        * lib.rs

### Etape 1.1

* Regarder les sources *.h et *.c de ce projet afin de comprendre la logique.
* Regarder les tests définis par `lib.rs` afin de comprendre l'interface qui sera attendue de cette librairie.

### Etape 1.2

* Préparer la configuration de ce projet pour compiler les sources C contenues.

Dans le fichier `Cargo.toml`, ajouter à la section `[package]` la spécification d'un script de build `build.rs`.

```toml
build = "build.rs"
```

Ajouter aussi les dépendances de build nécessaires : `cc` et `bindgen`.

```toml
[build-dependencies]
bindgen = "0.59"
cc = "1"
```

### Etape 1.3

Dans un fichier `build.rs` :

* Importer les dépendances `bindgen` et `cc`.
* Dans une fonction `main()`, utiliser `cc` pour compiler `libgeo.c`.
    * Se référer à [la doc de `cc`](https://docs.rs/cc/1.0.37/cc/).
* Dans cette même fonction, générer les bindings à partir de `libgeo.h` à l'aide de `bindgen`.
    * Se référer à [la doc de `bindgen`](https://docs.rs/bindgen/0.49.1/bindgen/struct.Builder.html#method.header).
* S'assurer que les bindings généré son ensuite sérialisés dans un fichier `bindings.rs` qui sera placé dans le répertoire de sortie de cargo, indiqué dans la variable d'environnement `OUT_DIR`.

### Etape 1.4

Compléter le fichier `lib.rs` en incluant les bindings avec :

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

Compiler et tester avec `cargo test`. Que se passe-t-il ?

### Etape 1.5

Résoudre le ou les problèmes identifiés à l'étape précédente en :

* incluant les bindings dans le context d'un module interne à la librairie.
* écrivant à la racine de la libraire un interface Rust ``propre'' sur ce module interne.

Compiler et tester avec `cargo test`.



## Etape 2 -- `libgeo-wrapper`

Dans cette second étapes, nous nous concentrons sur la projet `libgeo-wrapper` dont l'objectif est de fournir une interface Rust sur une librairie supposée pré-existante. Dans notre cas, cette librairie est dans le dossier `libgeo`.

### Etape 2.1

* Compiler la librairie `libgeo` en exécutant le script `build.sh` contenu dans ce dossier. Ce script produit une librairie statique libgeo.a et une librairie dynamique libgeo.so sous Linux. Pour les autres OS, adaptez les commandes afin de produire l'équivalent.

### Etape 2.2

* Préparer la configuration de ce projet pour générer les bindings nécessaires lors du build.

Dans le fichier `Cargo.toml`, ajouter à la section `[package]` la spécification d'un script de build `build.rs`.

```toml
build = "build.rs"
```

Ajouter aussi les dépendances de build nécessaires : `bindgen`.

```toml
[build-dependencies]
bindgen = "0.59"
```

### Etape 2.3

Dans un fichier `build.rs` :

* Importer la dépendance `bindgen`.
* Dans une fonction `main()`, générer les bindings à partir de `wrapper.h` à l'aide de `bindgen`.
    * Se référer à [la doc de `bindgen`](https://docs.rs/bindgen/0.49.1/bindgen/struct.Builder.html#method.header).
* S'assurer que les bindings généré son ensuite sérialisés dans un fichier `bindings.rs` qui sera placé dans le répertoire de sortie de cargo, indiqué dans la variable d'environnement `OUT_DIR`.
* Toujours dans la fonction `main`, ajouter les sorties nécessaires pour la configuration de `cargo` et `rustc` pour faire référence à la librairie C `libgeo` :

```rust
// spécifie un dossier dans lequel les librairies peuvent être recherchées
println!("cargo:rustc-link-search=native={}", location);
// ajoute une référence sur la librairie statique `libgeo`
println!("cargo:rustc-link-lib=static=geo");
```

> Dans ce bout de code, `location` doit faire référence au dossier `libgeo`.
> Utilisez la variable d'environnement `CARGO_MANIFEST_DIR` donnant le chemin vers le dossier contenant le fichier `Cargo.toml` du projet courant, c'est à dire `libgeo-wrapper` dans notre cas.

### Etape 2.4

* Réutiliser le résultat de l'étape 1.5 pour compléter de la même manière le fichier `lib.rs`
* Compiler et tester avec `cargo test`.
