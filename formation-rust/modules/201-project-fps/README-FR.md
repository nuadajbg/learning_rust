# Projet Farthest Point Search

Vous pouvez démarrer ce projet à n'importe quelle étape de cette feuille de route.
Un tag git de la forme `step{i}` où `i` est le numéro de l'étape est fourni pour facilement commencer de n'importe où.
Par example pour reprendre à partir de l'étape 5, exécutez :

```bash
git checkout step5
```

Pour avoir le projet dans sa forme finale, exécutez :

```git
git checkout step24
```

## Step 1 : Créer la structure `Vertex`

En Rust, une structure s'écrit de la manière suivante :

```rust
struct MyType {
    username: String
}
```

Créez une structure `Vertex` pour un sommet 3D avec des coordonnées `x`, `y`, `z` de type `f64`.

## Step 2 : Lire les arguments de la ligne de command

Lisez tous les arguments de la ligne de commande dans un vecteur.
Jetez un oeil à [std::env::args](https://doc.rust-lang.org/stable/std/env/fn.args.html).

## Step 3 : Chargement des données - début

Ecrire une fonction `load_input` qui prend en argument un nom de fichier (de type `&str`).
Cette fonction peut échouer avec des erreurs d'ES.
Jetez un oeil à [Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html).
Lorsque tout fonctionne bien, la fonction doit retourner un [vecteur](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) de `Vertex`.

## Step 4 : Chargement des données - ouverture du fichier

Dans la fonction `load_input`, ouvrir le fichier dont le nom est passé en argument.
Jetez un oeil à [File::open](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).
Propagez l'erreur produite par `File::open` en utilisant l'opérateur de propagation d'erreur : `?`.

## Step 5 : Chargement des données - itération sur les lignes

Une fois le fichier ouvert, itérez sur les lignes avec un boucle `for`.
Jetez un oeil à [BufReader](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html) et [BufRead::lines](https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.lines).

## Step 6 : Chargement des données - itération sur les mots

Dans la boucle sur toutes les lignes, itérez sur les mots d'une ligne, séparés par des espaces.
Jetez un oeil à [str::split_ascii_whitespace](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace).

## Step 7 : Chargement des données - parse x, y, z

Utilisez l'itérateur produit par [str::split_ascii_whitespace](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace) pour avoir accès à chaque nombre sur la ligne et ainsi parser chacun d'eux.
Pour parser un `&str`, vou pouvez faire :

```rust
"1.3".parse::<f64>();
```

A la fin, vous devez avoir trois coordonnées `x`, `y`, `z` de type `f64`.

## Step 8 : Chargement des données - empilement dans le vecteur

Utilisez les coordonnées `x`, `y`, `z` pour instancier un `Vertex` pour chaque ligne et l'empiler dans le vecteur de sommets à retourner.

## Step 9 : Chargement des données - collecter comme un `Result<Vec<_>, _>`

Jusqu'ici nous avons utilisé un style impératif avec une boucle `for` pour itérer sur les lignes et empiler dans un vecteur mutable de sommets.
Maintenant nous voulons utiliser un style fonctionnel.

Réécrivez la boucle `for` qui itère sur les lignes en utilisant directement l'itérateur sur les lignes et en mappant chaque ligne vers quelque chose que l'on pourra accumuler dans un vecteur en utilisant [Iterator::map](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map) et [Iterator::collect](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect).

Notez que l'on ne veut pas avoir pour résultat de `collect` un `Vec<Result<Vertex, _>>` mais bien un `Result<Vec<Vertex>, _>`. Jetez un oeil à [ces exemples](https://doc.rust-lang.org/stable/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E).

## Step 10 : Chargement des données - type d'erreur personnalisé

Jusqu'ici nous avons seulement tenu compte et propagé les erreur de type `std::io::Error`.
Les resultats avec d'autres types d'erreur ont simplement été traité avec `unwrap`, marquant une hypothèse que l'erreur ne peut pas se produire.
Dans cette étape nous voulons traiter correctement l'erreur de parsing; nous devons alors propager aussi bien `std::io::Error` que `ParseFloatError` qui est produit par `parse`.

* Créez une énumération `FpsError` qui va encapsuler dans ses variantes les différents types d'erreurs susceptibles d'être produit par `load_input`.
* Propagez le `ParseFloatError` avec l'opérateur `?` sur `parse`.
* Pour que cette propagation fonctionne, nous devons transformer les `std::io::Error` et `ParseFloatError` en `FpsError`. Utilisez des implémentations du [trait From](https://doc.rust-lang.org/stable/std/convert/trait.From.html).

## Step 11 : Chargement des données - ajouter un nouveau scénario d'erreur

Maintenant nous voulons traiter correctement le cas où une ligne de l'entrée serait mal formée avec trop peu de nombres.
Dans l'état présent, un nombre manquant sur une ligne est représenté par le fait que l'itérateur `words` retourne une [Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html) avec `next`; et nous appelons simplement `unwrap` dessus.

Pour traiter correctement ce problème, nous devons émettre un nouveau type d'erreur avec `FpsError`, représentant le fait que l'une des coordonnées est manquante sur la ligne.

Puis nous voulons propager cette nouvelle erreur lorsque le problème survient.
Jetez un oeil à [la méthode `ok_or`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.ok_or) sur `Option` pour cela.

## Step 12 : Chargement des données - implémentation du trait Error

Notre type `FpsError` est maintenant presque prêt.
Il serait intéressant qu'il se comporte que n'importe quel autre type d'erreur.
En Rust, les types d'erreur implémente généralement le trait [std::error::Error](https://doc.rust-lang.org/stable/std/error/trait.Error.html).
Implémentez ce trait sur `FpsError` ainsi que tout ce qui est nécessaire pour que cela fonctionne.

## Step 13 : Chargement des données - utilisez la fonction

Notre fonction `load_input` est maintenant complète !

* Utilisez la dans la fonction `main` pour charger les sommets en utilisant le nom de fichier passé en argument du programme.
* Propagez l'erreur produite par `load_input` à la fonction `main`.

## Step 14 : Calcul de la distance entre sommets

Avant d'implémenter réellement l'algorithme FPS, nous devons pouvoir calculer la distance entre 2 sommets.

Implementez une méthode `distance` sur `Vertex` pour calculer la distance euclidienne entre 2 sommets.

## Step 15 : FPS - début

Implémentez en fonction `fps` vide qui prend en argument une slice de sommets et un nombre de sommets à sélectionner et retourne un vecteur de sommets sélectionnés.
Pour le moment :

* Ecrire la signature de la fonction et lui faire retourner un vecteur vide.
* Appeler la fonction `fps` dans `main` pour obtenir un modèle réduit.

## Step 16 : FPS - sélection du premier sommet

Dans `fps`, sélectionner un premier sommet (nommé `p0`) dans les sommets d'entrée et l'empiler dans le vecteur de sortie des sommets sélectionnés.

## Step 17 : FPS - calcul initial des distances

Calculez un vecteur de distances qui contient la distance de chaque sommet vers le premier sélectionné `p0`.

* Utilisez la méthode `distance` sur `Vertex`
* Utilisez le style fonctionnel avec `map` et `collect`.

## Step 18 : FPS - calcul de l'index du plus éloigné

En utilisant seulement le vecteur de distances, calculez l'index du sommet le plus éloigné de `p0`.

Astuces :
* Vous pouvez itérer sur les distances avec leur index en utilisant [enumerate](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.enumerate)
* Jetez un oeil à la méthode [max_by](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.max_by) sur les itérateurs.

## Step 19 : FPS - sélectionner le second sommet

Maintenant que nous avons l'index du sommet le plus éloigné de `p0`, appelons ce nouveau sommet sélectionné `p1` et empilons le dans le vecteur de sommets sélectionnés.

## Step 20 : FPS - mise à jour des distances

Maintenant que nous avons 2 sommets sélectionnés `p0` et `p1`; nous devons mettre à jour le vecteur de distances pour qu'il contienne, pour chaque sommet, la distance la plus courte (min) vers soit `p0` soit `p1`.

Astuces:
* Vous pouvez itérer conjointement sur les sommets et les distances associées avec [zip](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.zip).
* Vous pouvez itérer avec des références mutables sur les distances avec [iter_mut](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.iter_mut).

## Step 21 : FPS - finalisation de la boucle FPS

Dans l'état actuel nous avons écrit le code sous la forme d'une boucle déroulée.
Tout ce qu'il nous reste à faire dans `fps` est de reconstituer la boucle pour répéter :

* le calcul de l'index du sommet le plus éloigné,
* la sélection de ce somment (et son empilement dans le vecteur de sommets sélectionnés),
* la mise à jour du vecteur de distances.

> Par construction, le vecteur de distances va toujours contenir, pour chaque sommet, la distance la plus courte vers un des sommets déjà sélectionné. Et nous recherchons toujours dans ce vecteur le sommet le plus éloigné, c'est à dire celui qui le plus loin de tous ceux déjà sélectionné, implémentant ainsi l'algorithme FPS.

## Step 22 : Ecriture du fichier de sortie

Ecrire une fonction  `write_output` pour ecrire dans un fichier, dans le même format que l'entrée, le vecteur de sommets sélectionnés.

## Step 23 : Affichage des modèles

Ecrire une fonction `plot` pour afficher les modèles d'entrée et réduit en utilisant `gnuplot`.

Jetez un oeil aux examples dans [la documentation de gnuplot](https://docs.rs/gnuplot/latest/gnuplot/).
Faites attention à bien utiliser [axes3d](https://docs.rs/gnuplot/latest/gnuplot/struct.Figure.html#method.axes3d).

## Step 24 : C'est terminé, bravo !
