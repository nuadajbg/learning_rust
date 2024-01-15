# Aquarium

You can start the project at any point in the roadmap.
A git tag of the form `step{i}` where `i` is the step number is provided to easily start at a further step.
For example, to start at step 5, execute :

```bash
git checkout step5
```

To see the fully completed project, execute :

```git
git checkout step22
```

This project is inspired from the [Javaquarium](https://zestedesavoir.com/forums/sujet/447/javaquarium/).

In this project we want to simulate an aquarium with kelps and fishes.
At each turn, the actions of kelps and fishes will be resolved.
Time is simulated by passing turns.



----

## Chapter 1 - Aquarium with kelps and fishes

The aquarium must be able to contain an arbitrary number of kelps and fishes.
At this time, the kelps do nothing and have no defining characteristics that distinguish one from the other.

Fishes have:
- a name,
- a sex,
- a specie among:

| Carnovorous fishes  | Herbivorous fishes  |
|---------------------|---------------------|
| Grouper             | Sole                |
| Tuna                | Bass                |
| Clown-Fish          | Carp                |

Kelps do nothing (that have no actions to do at each turn, yet.)

At each turn, each fish eats:
* Herbivorous fishes eat a single kelp that disappear from the aquarium,
* Carnivorous fishes eat a single fish, any one at random except those from the same specie. The eaten one disappear from the aquarium.

The simulation ends when:
* there is no more fish or kelp
* or, there are only carnivorous fishes from the same specie, that cannot eat each other.

----

### Step 1 - Modeling fishes

* Create a `Sex` enumeration for the sex of fishes (`Male`, `Femelle`)
* Create a `Diet` enumeration for the eating habits of fishes (`Herbivorous`, `Carnivorous`)
* Create a `Specie` enumeration for the species (`Grouper`, `Tuna`, `ClownFish`, `Sole`, `Bass`, `Carp`)
* On those enumerations, derive
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
```
* Add a `diet` method on `Specie` to obtain the diet for each specie.
* Create a `Fish` structure with the attributes `name`, `sex` et `specie`.

### Step 2 - Modeling the aquarium

* Create an `Aquarium` struture with:
  * the number of kelps,
  * the list of fishes

### Step 3 - Creation of random fish

* Create a `random` function to generate a random `Fish`
  * takes the generator as an argument (`rand::rngs::ThreadRng`)
  * to generate a number:
```rust
let mut rng = rand::thread_rng();
let value = rng.gen_range(0..6); // a number between 0 and 6 excluded
```

* Add a `From` implementation on `Sex` to convert from a number between 0 and 1.
* Add a `From` implementation on `Specie` to convert from a number between 0 and 5.

### Step 4 - Initialise an aquarium

* Add a `random` function on the `Aquarium` to create an aquarium with 10 kelps and 10 random fishes.
  * take no argument and return back the aquarium,
  * create the random number generator with:
```rust
let mut rng = rand::thread_rng();
```

### Step 5 - Simulate the aquarium

* Add a `tick` method on the aquarium to simulate a single turn.
  * Iterate on all fishes and make them eat.
  * Add some `println!` to signal fishes are eating.
* On the `Aquarium`, add an `is_dead` method to determine whether the simulation should stop when there can be no more progress. Simulation must end when:
  * there is no more fish or kelp,
  * or, there are only carnivorous fishes from the same specie, that cannot eat each other.
* In the `main` function, create a random aquarium and simulate it until there is no more progress.
  * At the beginning of each turn, print with `println!` the turn number and a summary of the aquarium (number of kelps and fishes).



---

## Chapter 2 - Health of lifeforms

Now we want to asociate a health indicator to each kelp and fish.
Being eaten reduce the health of kelps and fishes.
Conversely, eating increase the health of fishes.
Here are the rules:

* Kelps and fishes whose health are reduced to 0 die and disappear from the aquarium.
* Kelps begin with a random health between 1 and 10.
* Kelps grow each turn and gain 1 health.
* Each turn, fishes become more hungry and have their health reduced by 1.
* A fish is sufficiently hungry to eat when its health is 5 or below.
* When a herbivorous fish eat, the kelp loses 2 health, the fish gains 3.
* When a carnivorous fish eat, the prey loses 4 health, the fish gains 5.
* A carnivorous fish does not eat itself nor other members of its own specie.

Simulation ends when there are no more fish.

----

### Step 6 - Modeling kelps

* Add a `Kelp` structure with a `life` attribute.
* Add a `random` method to generate a new kelp with a random health between 1 and 10.
  * takes the generator as an argument (`rand::rngs::ThreadRng`)
* Modify the `Aquarium` to store instances of `Kelp`.
* Modify the `random` function on `Aquarium` to generate 10 random kelps.
* Update the `is_dead` method to signal the end when there are no more fish.
* Adapt the `main` function if necessary.

### Step 7 - Preparing the `tick` method

The `tick` method now must act on kelps and fishes.

* Extracts the current code in a new `tick_fishes` method.

### Step 8 - Simulating kelps

Add a `tick_kelps` method,
  * iterate on all kelps,
  * make them grow by adding 1 to their `life` attribute.
Call `tick_kelps` from `tick`.

### Step 9 - Health for fishes

* Add a `life` attribute on `Fish`.
* Modify the `random` function on `Fish` to use a random number for `life` between 1 and 10.

### Step 10 - All lifeforms

* Create a `Lifeform` trait with a single method `is_alive` that return a `bool`.
* Implement it on both `Kelp` and `Fish` by using their respective `life` attribute.

### Step 11 - Cleaning up the aquarium

* Create a `cleanup_dead` function on the `Aquarium`, generic over all types that implement `Lifeform` so that it can remove from a vector all the dead items.
* This function takes as an argument of vector of elements that implement `Lifeform`.
* Use this function to clean up both `kelps` and `fishes` vector of the `Aquarium` at the end of the `tick` method.

### Step 12 - Simulating fishes

* Add a `tick_fish` method to leave only the loop in the `tick_fishes` method and a call to `tick_fish`.
* Begin by simulating the hunger of fishes by decrementing its health by 1 in `tick_fish`.
  * If the fish dies, signal it with a `println`.
* A fish eats only when it is still alive and is hungry with health at 5 or below.
* Put the code to eat some kelp in a `eat_kelp` method:
  * randomly select a kelp to eat among those still alive.
  * return a `bool` indicating whether a kelp was indeed eaten.
  * decrement the health of the eated kelp.
* Put the code to eat a fish in a `eat_fish` method:
  * randomly select a prey among the alive ones, with a specie different from the predator.
  * return an `Option` with the index of the prey.
  * decrement the health of the prey.



---

## Chapter 3 - Reproduction and ageing

Now we simulate the fact that all lifeforms age, eventually die and reproduce.

* The kelps nad fishes must have an age (in number of turns).
* At each turn the age increase by 1.
* They die when they reach the old age of 20.

* Kelps with at least 10 health reproduce by dividing into two kelps:
  * The first one keep the same age and has its health divided by 2,
  * The second one gets the other half of the health and starts with an age of 0.

* Fishes reproduce generally with another fish of the same specie with the opposite sex, randomly selected among the ones that fulfill the following criteria:
* A fish only reproduce if it is not hungry and is old enough (age is at least 2).
* A single fish can only reproduce once per turn.
* The child appear in the aquaurim as a fish of the parents' specie with an age of 0.
  * other characteristics are random
* The sexuality of fishes depend on the specie as follow:

| Sexuality                   | Herbivorous | Carnivorous   |
|-----------------------------|-------------|---------------|
| Classic heterosexual        | Carp        | Tuna          |
| Hermaphrodite with age      | Bass        | Grouper       |
| Opportunistic hermaphrodite | Sole        | Clown-Fish    |

* **Classic heterosexual**: The fish seeks a partner of the opposite sex.
* **Hermaphrodite with age**: Fishes with an age between 0 and 10 excluded are always male, and become female after age 10. A fish seeks a partner of the opposite sex.
* **Opportunistic hermaphrodite**: A fish seeks any partner of the same specie and change its sex to become the opposite of the selected partner.

----

### Step 13 - Ageing of kelps and fishes

* Adds an `age` attribute to `Kelp` and `Fish`.
* In the respective `random` functions, randomy generate this new attribute with a value between 0 and 20 excluded.
* In implementations of `is_alive`, take into account that an age of 20 or more means death.
* In the method `tick_kelps`, increment the age of kelps and only continue with kelp of 19 and younger.
* Same with fishes in the method `tick_kelps`.
  * Add a `println!` to signal when a fish dies of old age.

### Step 14 - Modeling fish sexuality

* Add a `Sexuality` enumeration with variants `Heterosexuality`, `HermaphroditeWithAge`, `OpportunisticallyHermaphrodite`.
* Derive `Debug, Clone, Copy, PartialEq, Eq` on it.
* Add a method `sexuality` on `Specie` to get the sexuality associated to a specie.
* Add a method `opposite` on `Sex`.

### Step 15 - Random sex selection

* Add a method `random_sex` on `Specie` to randomly choose a sex for a fish specie, depending on the specie and the fish's age.
* Species with `HermaphroditeWithAge` as sexuality must always be male with a age strictly less than 10, female otherwise.
* For other species, the sex is always random~:
  * takes the generator as an argument (`rand::rngs::ThreadRng`)
  * use this function in the `random` of `Fish`.

### Step 16 - Generation of random fish children

* Add a `random_offspring` function on `Fish` to create a random fish a given specie.
* Age is always 0.
* Health is always 5.
* Take a single argument, the specie.

### Step 17 - Fish reproduction - stub

* Create a `reproduce_fish` on `Aquarium` for the reproduction of a fish.
* Let it be empty for now.
* In `tick_fish`, after incrementing the age, if the fish is not dead, is not hungry (health > 5) and is old enough (age >= 2), then it reproduces by calling `reproduce_fish`.

### Step 18 - Fish reproduction

* Implement the rest of `reproduce_fish`, taking into account the sexuality of the fish.
* Display with a `println!` the reproduction of both parents and the resulting child.

### Step 19 - Reproduction a single time per turn

In the current state, a single fish can reproduce multiple time per turn with different partners. To avoid this:

* Add a boolean attribute `reproduced` on `Fish`.
* Take into account whether the fish already reproduced when selecting a partner.
* Same for the reproducing fish.
* At the end of a turn, do not forget to reset the attribute.



----

## Chapter 4 - Data exchange

We now want to support the exchange of data between users of our simulator.
The command-line interface of the simulator must be able to
* create a random initial state
* simulate from state saved in a file until the end, or for a given number of steps
* save the final state

----

### Step 20 - `Serialize`, `Deserialize`

Derive `Serialize` and `Deserialize` on `Aquarium`, `Fish`, `Kelp`, `Sex`, `Specie`.

### Step 21 - CLI

Use [clap](https://crates.io/crates/clap) to define a CLI :

* `aquarium seed <XX> <YY> <filename>`
  * generate a random aquarium with XX kelps and YY fishes and save that into a file.
* `aquarium run <filename> [--steps COUNT] [--output FILE]`
  * simulate an aquarium from a state in file, either until the or, or if specified, for a number of steps, whichever is sooner.
  * optionally, export the final state into another file.
* `aquarium play [--kelps XX] [--fishes YY]`
  * Create a random aquarium and simulate it until the end with the optional specification of the number of kelps and fishes.
