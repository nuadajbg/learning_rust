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


# VARIABLE

let <nom> = <valeur>




# FONCTION

déclaration : fn
fn <name_fonction> (<paramètres>) {}

# MACRO

    println!("Hello, world!");

println est en faite une macro qui permet écriture sur la sortie standart.
Identifiable avec le"!" 



