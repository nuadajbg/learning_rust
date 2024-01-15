# Exercice 002 -- Macro procédurale

## Objectif

Cet exercice consiste à écrire une macro procédurale pour dériver automatiquement un trait sur une structure.
Le trait qui nous intéresse est le suivant :

```rust
/// Trait for structures that can be printed on the console
pub trait Printable {
    /// Prints a description of this structure on the console
    fn print(&self);
}
```

Le but est de dériver automatiquement ce trait sur le type suivant :

```rust
struct MyStruct {
    a: String,
    b: f32,
}
```

Puis d'appeller la méthode `print()` sur une instance de cette structure, par exemple :

```rust
let instance = MyStruct {
    a: "something".to_string(),
    b: 42.0,
};
instance.print();
```

La sortie attendue est :

```
a=something
b=42
```

## Etape 1 -- Structure

L'exercice est composé de plusieurs projets :

* `mymacro` définissant le trait `Printable`.
* `mymacro_derive` définissant la macro à implanter dans cet exercice.
* `tester` utilisant les deux librairies précédentes et réalisant le test montré plus haut.

A la racine de cet exercice se trouve aussi un fichier `Cargo.toml` qui spécifie un workspace contenant ces trois projets de tel sorte qu'il est possible d'utiliser les commandes `cargo` directement à ce niveau.

Parcourez ces différents projets et inspectez le code déjà écrit afin de vous familiariser avec celui-ci.

Exécuter la commande `cargo build` à la racine de cet exercice.

La compilation doit se dérouler normalement jusqu'au dernier projet qui doit échouer avec l'erreur :

```
error[E0599]: no method named `print` found for type `MyStruct` in the current scope
```

## Etape 2 -- Make it compile!

* Modifier la fonction `impl_printable` dans `mymacro_derive` afin que le projet complet compile.
    * Remplissez le corps de la macro `quote!` afin de produire l'implémentation du trait `Printable` (laisser l'implantation de `print` vide pour l'instant). [Documentation de `quote`](https://docs.rs/quote/0.6.3/quote/)
    * Regarder la documentation de `DeriveInput` afin de trouver comment récupérer le nom de de la structure pour laquelle le trait doit être implanté. [Documentation de `DeriveInput`](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html)
* Compiler puis exécuter le programme avec :

```sh
./target/debug/exercice_002_tester
```

* La sortie doit être vide.

## Etape 3 -- Implanter le reste de la macro

Finir l'implantation de `impl_printable` et tester.

* Appuyez-vous sur la [documentation de `DeriveInput`](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html) afin de trouver les champs (fields) de la structure pour laquelle le trait doit être implanté.
* Vous aurez certainement besoin de
    * [`proc_macro2::TokenStream`](https://docs.rs/proc-macro2/0.4.27/proc_macro2/struct.TokenStream.html)
    * [`quote::TokenStreamExt`](https://docs.rs/quote/0.6.3/quote/trait.TokenStreamExt.html)
        * Méthodes `append` et `append_all`.
