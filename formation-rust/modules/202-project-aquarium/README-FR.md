# Aquarium

Vous pouvez démarrer ce projet à n'importe quelle étape de cette feuille de route.
Un tag git de la forme `step{i}` où `i` est le numéro de l'étape est fourni pour facilement commencer de n'importe où.
Par exemple pour reprendre à partir de l'étape 5, exécutez :

```bash
git checkout step5
```

Pour avoir le projet dans sa forme finale, exécutez :

```git
git checkout step22
```

Ce projet est inspiré du [Javaquarium](https://zestedesavoir.com/forums/sujet/447/javaquarium/).

Dans ce projet nous voulons simuler un aquarium contenant des algues et des poissons. A chaque tour les actions des algues et poissons doivent être résolues. Le temps est simulé par le passage de tours.



----

## Chapitre 1 - Aquarium avec des algues et des poissons

L'aquarium doit pouvoir contenir un nombre quelconque d'algues et de poissons.
À ce stade les algues ne font rien et n'ont pas de caractéristiques particulières qui les distinguent les unes des autres.

Les poissons ont :
* un nom
* un sexe
* une race parmi:

| Poissons carnivores | Poissons herbivores |
|---------------------|---------------------|
| Mérou               | Sole                |
| Thon                | Bar                 |
| Poisson-Clown       | Carpe               |

Les algues ne font rien (n'ont pour l'instant pas d'action à faire à chaque tour).

A chaque tour, chaque poisson mange:
* Les poissons herbivores mangent une algue qui disparaît donc de l'aquarium.
* Les poissons carnivores mangent un poisson, n'importe lequel au hasard sauf ceux de son espèce, et celui-ci disparaît donc de l'aquarium.

La simulation s'arrête lorsque:
* il n'y a plus de poisson ou d'algues,
* ou il ne reste que des poissons carnivores de la même espèce, qui ne peuvent se manger entre eux.

----

### Étape 1 - Modélisation des poissons

* Créer une énumération `Sex` pour le sexe des poissons (`Male`, `Femelle`)
* Créer une énumération `Diet` pour le régime alimentaire des poissons (`Herbivorous`, `Carnivorous`)
* Créer une énumération `Specie` pour les races de poisson (`Grouper`, `Tuna`, `ClownFish`, `Sole`, `Bass`, `Carp`)
* Dériver sur toutes ces énumérations:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
```
* Ajouter une méthode `diet` à l'énumération `Specie` pour obtenir le régime alimentaire associé à une race.
* Créer une structure `Fish` pour un poisson avec les attributs `name`, `sex` et `specie`.

### Étape 2 - Modélisation de l'aquarium

* Créer une structure `Aquarium` avec:
  * un nombre d'algues (`kelp`)
  * une liste de poissons (`fishes`)

### Étape 3 - Création aléatoire d'un poisson

* Créer une fonction `random` pour générer aléatoirement un poisson.
  * Cette fonction doit prendre le générateur de nombres aléatoires en argument (`rand::rngs::ThreadRng`)
  * pour générer un nombre:
```rust
let mut rng = rand::thread_rng();
let value = rng.gen_range(0..6); // un nombre entre 0 et 6 exclus
```

* Ajouter une implémentation de `From` pour convertir un nombre entre 0 et 1 vers `Sex`,
* Ajouter une implémentation de `From` pour convertir un nombre entre 0 et 5 vers `Specie`.


### Étape 4 - Initialisation d'un aquarium

* Ajouter une fonction `random` pour créer un aquarium avec 10 algues et 10 poissons aléatoires.
  * Cette fonction n'a pas d'argument et renvoie un aquarium,
  * Elle a besoin de créer le générateur de nombres aléatoires avec:
```rust
let mut rng = rand::thread_rng();
```

### Étape 5 - Simuler l'aquarium

* Ajouter une méthode `tick` sur l'aquarium pour simuler un tour
  * Itérer sur les poissons et les faire manger.
  * Ajouter des `println!` pour signaler ce que mange un poisson.
* Sur `Aquarium`, ajouter une méthode `is_dead` pour déterminer si la simulation de l'aquarium doit s'arrêter car il n'y a plus de progrès. La simulation s'arrête lorsque:
  * il n'y a plus de poisson ou d'algues,
  * ou il ne reste que des poissons carnivores de la même espèce, qui ne peuvent se manger entre eux
* Dans la fonction main, créer un aquarium aléatoire et le simuler jusqu'à ce qu'il n'y ait plus d'activité.
  * Avant chaque tour, afficher avec un `println!` le numéro du tour avec le nombre d'algues et de poissons restant.


----

## Chapitre 2 - Santé des êtres vivants

On veut maintenant que les algues et les poissons puissent avoir un niveau de santé associé.
Le fait d’être mangé réduit la santé de l'algue ou du poisson.
À l'inverse, le fait de manger augmente la santé d'un poisson.
Voici les règles :

* Les algues et les poissons dont la santé tombe à 0 meurent et disparaissent.
* Les algues commencent avec une santé aléatoirement comprise entre 1 et 10.
* Les algues grandissent à chaque tour en gagnant 1 point de santé.
* A chaque tour, un poisson a de plus en plus faim et perd 1 point de santé.
* Un poisson a faim lorsqu'il a 5 ou moins points de santé.
* Lorsqu'un poisson herbivore mange, l'algue perd 2 points de santé, le poisson en gagne 3.
* Lorsqu'un poisson carnivore mange, la proie perd 4 points de santé, le prédateur en gagne 5.
* Un poisson carnivore ne se mange pas lui-même, ainsi que les autres membres de son espèce.

La simulation s'arrête lorsqu'il n'y a plus de poissons.

----

### Étape 6 - Modéliser les algues

* Ajouter une structure `Kelp` modélisant une algue, avec un attribut `life` pour sa santé.
* Ajouter une fonction `random` pour créer une algue avec un nombre aléatoire de points de santé compris entre 1 et 10
  * Cette fonction doit prendre le générateur de nombres aléatoires en argument (`rand::rngs::ThreadRng`)
* Modifier `Aquarium` stocker des instances de `Kelp` pour les algues.
* Modifier la fonction `random` de `Aquarium` pour générer aléatoirement 10 instances de `Kelp`.
* Adapter la méthode `is_dead` pour signaler que la condition d'arrêt est qu'il ne reste plus de poissons.
* Adapter éventuellement le code dans `main`.

### Étape 7 - Préparation de la méthode `tick`

La méthode `tick` doit maintenant agir à la fois sur les algues et les poissons.

* Externaliser le code actuel de `tick` dans une nouvelle méthode `tick_fishes`.

### Étape 8 - Simulation des algues

Ajouter une méthode `tick_kelps` pour simuler les algues.
* Boucler sur les algues
* Pour chacune les faire grandir en ajouter 1 à leur santé (attribut `life`)
Appeler `tick_kelps` dans `tick`.

### Étape 9 - Santé des poissons

* Ajouter un attribut `life` dans `Fish` pour représenter la santé de ceux-ci.
* Modifier la fonction `random` sur `Fish` pour utiliser un nombre aléatoire de points de santé compris entre 1 et 10.

### Étape 10 - Tous les être vivants

* Créer un trait `Lifeform` avec une seule méthode `is_alive` renvoyant un `bool`.
* Implémenter ce trait sur `Kelp` et `Fish` en utilisant leurs données de santé (`life`)

### Étape 11 - Nettoyage de l'aquarium

* Créer une fonction `cleanup_dead` dans `Aquarium`, générique pour tous les types implémentant `Lifeform` qui permet de supprimer d'un vecteur ceux qui sont morts.
* Cette fonction prend en argument uniquement un vecteur d'élément implémentant `Lifeform`.
* Appeler cette fonction pour nettoyer les vecteurs `kelps` et `fishes` de l'aquarium à la fin de la méthode `tick`.

### Étape 12 - Simulation des poissons

* Ajouter une méthode `tick_fish` pour ne laisser dans `tick_fishes` que la boucle sur les poissons et l'appel à cette nouvelle méthode.
* Commencer par simuler la faim du poisson dans `tick_fish` en décrémentant sa santé (`life`)
  * Si le poisson est mort, le signaler avec un `println`.
* Le poisson ne mange alors que s'il est toujours en vie et a une santé inférieure ou égale à 5.
* Externaliser le code pour trouver une algue et la manger dans une méthode `eat_kelp`.
  * Sélectionner aléatoirement l'algue à manger parmi celle encore vivante.
  * La méthode renvoie un `bool` indiquant si une algue a été trouvée.
  * La méthode décrémente la vie de l'algue mangée.
* Externaliser le code pour manger un poisson dans une méthode `eat_fish`.
  * Sélectionner aléatoirement une proie parmi les vivantes, différente du prédateur.
  * La méthode renvoie une `Option` de l'index de la proie.
  * La méthode décrémente la vie de la proie.


----

## Chapitre 3 - Reproduction et vieillissement

Simulons maintenant le fait que les êtres vivants vieillissent et se reproduisent.

* Les algues et les poissons doivent avoir un âge (en nombre de tours).
* A chaque tour leur âge avance de 1.
* Ils meurent lorsqu'ils atteignent l'âge de 20.

* Les algues ayant au moins 10 points de santé se reproduisent en se divisant en 2 algues.
  * La première se retrouve donc avec la moitié de ses points de santé et un âge inchangé.
  * La deuxième se trouve avec l'autre moitié des points de santé et un âge de 0.

* Les poissons se reproduisent généralement avec un autre poisson de la même espèce et de sexe opposé, choisi aléatoirement parmi tous ceux remplissant ces critères.
* Un poisson ne se reproduit que s'il n'a pas faim et a un âge d'au moins 2 tours.
* Un même poisson ne peut se reproduire qu'une fois par tour.
* Le poisson enfant apparaît dans l'aquarium comme un poisson de la même espèce avec un âge de 0.
  * ses autres caractéristiques choisies aléatoirement
* Les espèces de poisson ont des sexualités différentes résumées ci-dessous:

| Sexualité                  | Herbivore | Carnivore     |
|----------------------------|-----------|---------------|
| Hétérosexualité classique  | Carpe     | Thon          |
| Hermaphrodite avec l'âge   | Bar       | Mérou         |
| Hermaphrodite opportuniste | Sole      | Poisson-Clown |

* **Hétérosexualité classique**: Le poisson cherche un partenaire du sexe opposé.
* **Hermaphrodite avec l'âge**: Un poisson avec un âge entre 0 et 10 exclus est toujours mâle, et devient femelle à partir d'un âge de 10. Le poisson cherche un partenaire du sexe opposé.
* **Hermaphrodite opportuniste**: Le poisson cherche n'importe quel autre individu de la même espèce et change de sexe pour prendre le sexe opposé de celui de son partenaire.

----

### Étape 13 - Vieillissement des algues et poissons

* Ajout un attribut `âge` sur `Kelp` et `Fish`.
* Dans les fonctions `random` associée, générer aléatoirement cet attribut avec une valeur entre 0 et 20 exclus.
* Dans les implémentations de `is_alive`, prendre en compte qu un âge supérieur ou égal à 20 signifie un être vivant mort.
* Dans la fonction `tick_kelps`, incrémenter l'âge des algues et ne continuer que pour des algues avec un âge < 20.
* Dans la fonction `tick_fish`, commencer par incrémenter l'âge du poisson et ne continuer que pour un poisson avec un âge < 20.
  * Ajouter un `println!` pour signaler que le poisson est mort de vieillesse.

### Étape 14 - Modélisation de la sexualité des poissons

* Ajouter une énumération `Sexuality` avec les variantes `Heterosexuality`, `HermaphroditeWithAge`, `OpportunisticallyHermaphrodite`.
* Dériver `Debug, Clone, Copy, PartialEq, Eq` sur cette énumération.
* Ajouter une méthode `sexuality` sur `Specie` pour retrouver la sexualité associée à chaque espèce.
* Ajouter une méthode `opposite` sur `Sex` pour produire le sexe opposé.

### Étape 15 - Sélection aléatoire du sexe

* Ajouter une méthode `random_sex` sur `Specie` pour choisir aléatoirement le sexe d'un poisson, en fonction de l'espèce et de l'age.
* Les espèces avec une sexualité `HermaphroditeWithAge` doivent être mâles avec un âge < 10, femelle sinon.
* Pour les autres espèces, le sexe est toujours choisi aléatoirement.
  * Cette fonction doit prendre le générateur de nombres aléatoires en argument (`rand::rngs::ThreadRng`)
* Utiliser cette fonction dans la fonction `random` de `Fish`.

### Étape 16 - Création aléatoire d'un poisson enfant

* Ajouter une fonction `random_offspring` sur `Fish` pour créer aléatoirement un poisson naissant d'une espèce donné.
* L'âge est toujours de 0.
* La santé est toujours de 5.
* Cette fonction ne prend que l'espèce en argument.

### Étape 17 - Reproduction d'un poisson - stub

* Créer une méthode `reproduce_fish` sur `Aquarium` pour la reproduction d'un poisson.
* La laisser vide pour le moment.
* Dans la fonction `tick_fish`, après l'incrément de l'âge, si un poisson n'a pas faim (`life` > 5) et qu'il est suffisamment âgé (`age` >= 2), alors il se reproduit en appelant `reproduce_fish`.

### Etape 18 - Reproduction d'un poisson

* Implémenter le reste de la fonction `reproduce_fish` en fonction de sexualité du poisson
* Afficher avec un `println!` la reproduction avec les deux parents et l'enfant.

### Etape 19 - Reproduction une seule fois par tour

Dans l'état actuel un même poisson peut se produire plusieurs fois avec plusieurs partenaires différents pendant un même tour.
Pour éviter cela:

* Ajouter un attribut booléen `reproduced` sur `Fish`.
* Le prendre en compte dans la sélection des partenaires pour éviter ceux s'étant déjà reproduit.
* De même pour le poisson courant.
* A la fin d'un tour, ne pas oublier de réinitialiser cet attribut.


----

## Chapitre 4 - Échange de données

Nous souhaitons maintenant supporter l'échange de données entre utilisateurs de notre simulateur.
Notre simulateur en ligne de commande doit pouvoir:
* Créer un état initial aléatoire
* Simuler à partir d'un état sauvegardé dans un ficher jusqu'à l'arrêt, ou pendant un nombre de pas donné
* Sauvegarder l'état d'arrivée

----

### Etape 20 - `Serialize`, `Deserialize`

Dériver `Serialize` et `Deserialize` sur `Aquarium`, `Fish`, `Kelp`, `Sex`, `Specie`.

### Étape 21 - CLI

Utiliser [clap](https://crates.io/crates/clap) pour définir une interface CLI suivante :

* `aquarium seed <XX> <YY> <filename>`
  * créer aquarium avec XX algues et YY poissons et le sauvegarder dans le fichier
* `aquarium run <filename> [--steps COUNT] [--output FILE]`
  * simuler l'aquarium à partir de l'état `filename`, soit jusqu'à la fin, soit avec un nombre de pas fourni.
  * de manière optionnelle, sauvegarder l'état de sorti dans le fichier `FILE`.
* `aquarium play [--kelps XX] [--fishes YY]`
  * Créer un aquarium aléatoire et le simule jusqu'à la fin avec en arguments optionnels le nombre d'algues et de poissons
