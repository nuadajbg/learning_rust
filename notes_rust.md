# INSTALLATION

C'est un curl qui va télécharger un script puis on éxécute le script.

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

A l’installation on a:

Le compilateur
```
bash-3.2$ rustc --version
rustc 1.74.0 (79e9716c9 2023-11-13)
```

Cargo is the Rust package manager. Un couteau suisse.
```
bash-3.2$ cargo --version
cargo 1.74.0 (ecb9851af 2023-10-18)
```

A tool for formatting Rust code according to style guidelines
```
bash-3.2$ rustfmt --version
rustfmt 1.6.0-stable (79e9716c 2023-11-13)
```

A collection of lints to catch common mistakes and improve your Rust code. Aide à mieux écrire le code
```
bash-3.2$ cargo clippy
```

# CARGO INIT

initialisation projet:

```
cago init <name>
```
```
jb@d3c07604-2 learning_rust % cargo init ep1_akanoa
     Created binary (application) package
jb@d3c07604-2 learning_rust % ls -ltra ep1_akanoa
total 8
drwxr-xr-x  9 jb  staff  288 14 jan 16:35 ..
-rw-r--r--  1 jb  staff  179 14 jan 16:35 Cargo.toml
drwxr-xr-x  4 jb  staff  128 14 jan 16:35 .
drwxr-xr-x  3 jb  staff   96 14 jan 16:35 src
```

## cargo.toml
-> MD associées à un projet Rust

avec le nom du package (ici ep1_akanoa)

edition pourquoi ?
-> rust toujours en évolution 
-> permet de gérer les évolutions du langage
-> rétrio compatibilité complète

## .gitingnore

classique d'intégration d'éléments que nous ne souhaitons pas verisonner

## src

main.rs : contient la fonction main = point d'entrée de l'application

si on créer un projet binary -> c'est la premier fonction exécutée

# CARGO BUILD

Compile

```
jb@d3c07604-2 ep1_akanoa % cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

unoptimized + debuginfo -> lorsque que l'on compile du code en rust : on a 2 modes:
- débug du projet
- mode production (plus léger et rapide)

```
jb@d3c07604-2 ep1_akanoa % cargo build --release
   Compiling ep1_akanoa v0.1.0 (/Users/jb/github/learning_rust/ep1_akanoa)
    Finished release [optimized] target(s) in 0.24s
```

-> tout ça on trouve dans le repertoire target

```
jb@d3c07604-2 target % ls -ltr debug 
total 968
drwxr-xr-x   2 jb  staff      64 14 jan 16:51 examples
drwxr-xr-x   2 jb  staff      64 14 jan 16:51 build
drwxr-xr-x   3 jb  staff      96 14 jan 16:51 incremental
-rwxr-xr-x   1 jb  staff  490256 14 jan 16:51 ep1_akanoa
drwxr-xr-x  10 jb  staff     320 14 jan 16:51 deps
-rw-r--r--   1 jb  staff     121 14 jan 16:51 ep1_akanoa.d
```

```
jb@d3c07604-2 target % ls -ltr release 
total 968
drwxr-xr-x  2 jb  staff      64 14 jan 16:55 incremental
drwxr-xr-x  2 jb  staff      64 14 jan 16:55 examples
drwxr-xr-x  2 jb  staff      64 14 jan 16:55 build
-rwxr-xr-x  1 jb  staff  487536 14 jan 16:55 ep1_akanoa
drwxr-xr-x  4 jb  staff     128 14 jan 16:55 deps
-rw-r--r--  1 jb  staff     123 14 jan 16:55 ep1_akanoa.d
```

# CARGO RUN

Compile et exécute

```
jb@d3c07604-2 ep1_akanoa % cargo run
   Compiling ep1_akanoa v0.1.0 (/Users/jb/github/learning_rust/ep1_akanoa)
    Finished dev [unoptimized + debuginfo] target(s) in 7.91s
     Running `target/debug/ep1_akanoa`
Hello, world!
```

```
jb@d3c07604-2 ep1_akanoa % ls -ltra
total 16
-rw-r--r--  1 jb  staff  179 14 jan 16:35 Cargo.toml
drwxr-xr-x  3 jb  staff   96 14 jan 16:35 src
drwxr-xr-x  8 jb  staff  256 14 jan 16:37 ..
-rw-r--r--  1 jb  staff  154 14 jan 16:51 Cargo.lock
drwxr-xr-x  6 jb  staff  192 14 jan 16:51 .
drwxr-xr-x@ 5 jb  staff  160 14 jan 16:51 target
```

```
jb@d3c07604-2 target % cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `release/ep1_akanoa`
Hello, world!
```

# CARGO CLEAN

Nettoyage du projet

```
jb@d3c07604-2 target % cargo clean
jb@d3c07604-2 ep1_akanoa % ls -ltr
total 16
-rw-r--r--  1 jb  staff  179 14 jan 16:35 Cargo.toml
drwxr-xr-x  3 jb  staff   96 14 jan 16:35 src
-rw-r--r--  1 jb  staff  154 14 jan 16:51 Cargo.lock
```

-> supprime de répertoire "target"

# COMPILATEUR

But : présenter les erreurs de la façon la plus précise

exemple
```
let a = 42;
let b1 = true;
let b2 = false;
let c = a + c;
```

```
jb@d3c07604-2 ep1_akanoa % cargo build
   Compiling ep1_akanoa v0.1.0 (/Users/jb/github/learning_rust/ep1_akanoa)
error[E0425]: cannot find value `c` in this scope
 --> src/main.rs:7:17
  |
7 |     let c = a + c;
  |                 ^ help: a local variable with a similar name exists: `a`

For more information about this error, try `rustc --explain E0425`.
error: could not compile `ep1_akanoa` (bin "ep1_akanoa") due to previous error
```

```
let a = 42;
let b1 = true;
let b2 = false;
let c = a + b1;
```

## Types

```
jb@d3c07604-2 ep1_akanoa % cargo build
   Compiling ep1_akanoa v0.1.0 (/Users/jb/github/learning_rust/ep1_akanoa)
error[E0277]: cannot add `bool` to `{integer}`
 --> src/main.rs:7:15
  |
7 |     let c = a + b1;
  |               ^ no implementation for `{integer} + bool`
  |
  = help: the trait `Add<bool>` is not implemented for `{integer}`
  = help: the following other types implement trait `Add<Rhs>`:
            <isize as Add>
            <isize as Add<&isize>>
            <i8 as Add>
            <i8 as Add<&i8>>
            <i16 as Add>
            <i16 as Add<&i16>>
            <i32 as Add>
            <i32 as Add<&i32>>
          and 48 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `ep1_akanoa` (bin "ep1_akanoa") due to previous error
```

```
7 |     let c = a + b1;
  |               ^ no implementation for `{integer} + bool`
```

integer est entre accolade parce que ce n'est pas un vrai type mais une collection de type.
En rust lorsque l'on ne définie pas le type que l'on est en train de manipuler, il nous donne un type par défault.

lorsque l'on écrit :

```
    let a = 42;
```

Le compilateur fait en réalité 

```
    let a : i32 = 42;
```
i32 étant le type numérique.

dans ce cas le compilateur change l'erreur en 

```
7 |     let c = a + b1;
  |               ^ no implementation for `i32 + bool`
```

et même avec 2 nombres mais pas du même type -> ça ne fonctionne pas parce que Rust est fortement typé.

```
    let a : i32 = 42;
    let b1 :u8 = 9;
```

```
7 |     let c = a + b1;
  |               ^ no implementation for `i32 + u8`
```

## Dépassement de taille de nombres

Rust protège contre ça
(histoire de gandhi sur civilization qui est le plus pacifique et qui quand ça diminue passe à un moment à -1 qui boucle et passe à 255 et de fait devient le plus agressif de la partie)

```
    let non_signe_8bit : u8 = 256; // 8 bits : 0 -> 255
```

```
error: literal out of range for `u8`
 --> src/main.rs:9:31
  |
9 |     let non_signe_8bit : u8 = 256; // 8 bits : 0 -> 255
  |                               ^^^
  |
  = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default
```

On est prévenu lorsque l'on essaie de sortir de la zone de représentation

```
let m :i8 = -128;
let t = m - 1;

println!("Hello, world! {t}");
```

```
15 | let t = m - 1;
   |         ^^^^^ attempt to compute `i8::MIN - 1_i8`, which would overflow
```


## Caster

```
    let signe_8bit : i8 = 42; // 8 bits : 0 -> 255
    let signe_16bit : i16 = -145; // 8 bits : 0 -> 255

let t = signe_8bit + signe_16bit;
```

```
12 | let t = signe_8bit + signe_16bit;
   |                    ^ no implementation for `i8 + i16`
```

Réponse on caste.

```
let t = signe_8bit as i16 + signe_16bit;
println!("Hello, world! {t}");
```

```
Hello, world! -103
```

-> mais possible que parce 8bit rentre dans 16bit.
sinon

```
    let signe_8bit : i8 = -42; // 8 bits : 0 -> 255
    let signe_16bit : i16 = -145; // 8 bits : 0 -> 255

let t = signe_8bit + signe_16bit as i8;
```

```
Hello, world! 69
```

Alors attention parce que si on le débraie, on le débraie et ce que l'on obtient est faux.




# TYPES

let <nom> = <valeur>

## Nombres
i signé
u non signé

8, 16, 32, 64, 128

- u8 : non signé 8 bits : 0 -> 255
- i8 : signé 8 bits : -128 -> 127
- i16 : signé 16 bits : -256 -> 255

## Booléens

true, false

## Flottants

nombre à virgule - décimaux -> sont signés par défault
2 types de flottants
- float : f32 (flottant sur 32 bits) ex: -12.56242342
- doubles : f64 (précision est double) ex: -12.56242342512525432543542

Attention à la représentation qui peut perdre de la représentation.
Faire des opération sur les flottants n'est pas une bonne idée.
Par exemple pour de la trigo c'est pas une bonne idée.

```
fn main() {
    let t :f64 = -12.1234523534252345324523452165476787;
    println!("Hello, world! {t}");
}
```
```
jb@d3c07604-2 ep1_akanoa % cargo run
   Compiling ep1_akanoa v0.1.0 (/Users/jb/github/learning_rust/ep1_akanoa)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/ep1_akanoa`
Hello, world! -12.123452353425234
```


# FONCTION

déclaration : fn
fn <name_fonction> (<paramètres>) {}

# MACRO

    println!("Hello, world!");

println est en faite une macro qui permet écriture sur la sortie standart.
Identifiable avec le"!" 



# MEMOIRE

https://www.youtube.com/watch?app=desktop&v=qJi1YCUy3nY

Stack
Heap

Borrow checker
  - avantage combiné
  - pas de free manuel
  - pas de garbage collecteur

importance du contexte d'éxécution pour la suppression des données

On peut :
 - move -> move dans un autre contexte de la Stack et change le pointeur vers la Heap (mais la première variable de la stack existe encore)
 - copy/clone -> copie les variables dans la Stacks et la Heap (on double toute la consommation mémoire Stack et Heap)

une variable


