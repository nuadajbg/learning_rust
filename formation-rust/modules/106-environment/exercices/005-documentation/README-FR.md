# Exercice 005 -- Ecriture de documentation

Le projet fourni pour cette exercice est un projet de librairie.
Dans répertoire des sources `src`, le point d'entrée se trouve donc être `lib.rs`.

## Etape 1 -- documentation du module

Ecrire une documentation succinte du module racine, au début du fichier `lib.rs`.

* Utiliser le préfixe de commentaire `//!`

Produire la documentation avec la commande `cargo doc`.

Retrouver cette documentation dans le répertoire `target/doc/exercice_005/index.html`.

## Etape 2 -- documentation de la fonction `safe_integer_divide`

Dans `lib.rs` se trouve la définition de la fonction `safe_integer_divide`.
Ecrire un début de documentation décrivant ce que fait cette fonction.

* Utiliser le préfixe de commentaire `///`

Produire la documentation avec la commande `cargo doc` et regarder ce qui a été produit.

## Etape 2 -- exemple d'utilisation

Compléter la documentation de la fonction `safe_integer_divide` avec un exemple d'usage.

* Créer une nouvelle section `Examples`.
* Utiliser une assertion (`assert!`, etc.)

Produire la documentation avec la commande `cargo doc` et regarder ce qui a été produit.

Exécuter le test correspondant avec `cargo test`.

## Etape 3 -- documentation de la sortie erreur de la fonction

Compléter la documentation de la fonction `safe_integer_divide` avec une description simple de la sortie de la fonction en cas d'erreur, ainsi qu'un exemple d'usage dans ce cas.

Produire la documentation avec la commande `cargo doc` et regarder ce qui a été produit.

Exécuter le test correspondant avec `cargo test`.

## Etape 4 -- liens hypertext dans la documentation

Dans la documentation du module, ajouter un lien vers la documentation de la fonction `safe_integer_divide`.

* En Markdown, un lien se note `[nom visible](chemin relatif)`
* Regarder dans `target/doc/exercice_005` pour trouver la cible du lien.

Produire la documentation avec la commande `cargo doc` et regarder ce qui a été produit.
