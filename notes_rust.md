# <span style="color:red"> INSTALLATION


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


-> + installation de rust-analyser pour VScode (dans la marketplace)


# <span style="color:red"> CARGO INIT

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

## <span style="color:blue"> cargo.toml
-> MD associées à un projet Rust

avec le nom du package (ici ep1_akanoa)

edition pourquoi ?
-> rust toujours en évolution 
-> permet de gérer les évolutions du langage
-> rétrio compatibilité complète

## <span style="color:blue"> .gitingnore

classique d'intégration d'éléments que nous ne souhaitons pas verisonner

## <span style="color:blue"> src

main.rs : contient la fonction main = point d'entrée de l'application

si on créer un projet binary -> c'est la premier fonction exécutée

# <span style="color:red"> CARGO

## <span style="color:blue"> CARGO BUILD

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

## <span style="color:blue"> CARGO RUN

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

## <span style="color:blue"> CARGO CLEAN

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


# <span style="color:red"> COMPILATEUR

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

Rust est un langage "fortement typé" -> ça veut dire qu'il ne peut pas y avoir de changement de type en cours de route. ex en JS ou un String peut être additionné à un chiffre.

F12 dans un navigateur -> console:

>>"abc" + 3
->"abc3" 

## <span style="color:blue"> Types

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

## <span style="color:blue"> Dépassement de taille de nombres

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


## <span style="color:blue"> Caster

Syntaxe:

```
    let e = 5;
    e as u8;

    let f = 6 as u8; // marche aussi
    let f = 6_u8; // marche aussi
    let f = 6u8; // marche aussi

```

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




# <span style="color:red"> TYPES

let <nom> : <type> = <valeur>

pas d'introspection en rust donc pas de fonction pour savoir le type d'une variable.
un bordel possible, fait par le compilateur lorsqu'il nous dit qu'il ne peut pas additionner tel type avec tel type, avec un "type id" puis va dans une table ...
-> en fait si ça existe type_name

## <span style="color:blue"> Nombres (primitif)
i signé
u non signé

8, 16, 32, 64, 128

- u8 : non signé 8 bits : 0 -> 255
- i8 : signé 8 bits : -128 -> 127
- i16 : signé 16 bits : -256 -> 255

on peut aussi signer un nombre

```
let mut a = 23;
a += 1_u32;
```

à la première ligne le a est inféré et peut-être un i32
la seconde ligne impose que le 1 soit un u32 donc a un u32.
le compilateur sait à ce moment là que a est contraint à être un u32.

on peut s'amuser aussi à caster le a ensuite

```
let mut a = 23;
a += 1_u32 as i128;
```

a est alors un i128

## <span style="color:blue"> Booléens (primitif)

type bool true, false

## <span style="color:blue"> Flottants (primitif)

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

## <span style="color:blue"> Chaînes de caractères (primitif)

Chaîne de caractère a un type un peu bizarre "&str"
plus complexe que ça mais on verra ça plus tard

```
    let name : &str = "plop";
```

Autre type "String"

```
    let lastname = String::from("pouet");
```
-> intéressant parce que c'est une manière de gérer les chaines de caractère sans que rust nous embête sur les "référence" - rust nous permettra de faire sans être trop à cheval sur ce que l'on est en train de faire.


## <span style="color:blue"> Immuabilité / Muabilité des variables

```
    let a = 1;
    a +=1 ;
```

```
error[E0384]: cannot assign twice to immutable variable `a`
  --> src/main.rs:22:5
   |
21 |     let a = 1;
   |         -
   |         |
   |         first assignment to `a`
   |         help: consider making this binding mutable: `mut a`
22 |     a +=1 ;
   |     ^^^^^ cannot assign twice to immutable variable
```

-> par défaut on ne peut pas réaffecté une deuxième fois une valeur à a

```
  let a (-> a là) = 1;
  let a (-> et ce a là ne sont pas les mêmes)= a (-> ça c'est le premier a) + 1 ;
```

on peut le montrer dans typant différemment les 2 a et en castant.

```
    let a : u8 = 1;
    let a : i8 = (a + 1) as i8 ;
    Hello, world! 2
```

le premier a existe toujours en mémoire mais on ne sait plus y accéder.
comme on est dans la stack -> c'est nettoyé automatiquement lorsque l'on sort du contexte d'éxécution. (voir présentation au sunny tech - https://www.youtube.com/watch?v=qJi1YCUy3nY)


```
    let mut a = 1;
    a+=1;

    Hello, world! 2
```

on dit ici explicitement que la variable est muable.

intérêt de l'immuabilité par défaut : faire en sorte que les variable soient au max immuables pour éviter par exemple que plusieurs personnes modifient la même variable et est des valeurs différentes. En JS par exemple c'est l'inverse, il faut dire que c'est ummuable.

## <span style="color:blue"> Tuple (composite)

Type composite (avant ce sont des types primitifs sauf le &str).
Compose plusieurs primitifs.

il y a en a plusieurs.
on se concentre sur les Tuples.

```
    let my_tuple = ("plop", 45, "pouet");
```

```
7 |     println!("Hello, world! {my_tuple}");
  |                             ^^^^^^^^^^ `(&str, {integer}, &str)` cannot be formatted with the default formatter
```

-> le type de notre tuple est &str, {integer}, &str.

-> lorsque l'on est en train de faire le formattage de la chaîne, on peut faire un formattage de débug. c'est fait avec un ":?"

```
    println!("Hello, world! {my_tuple:?}");

```


Il nous livre la représentation de débug du tuple
```
Hello, world! ("plop", 45, "pouet")
```

-> le type de notre tuple est &str, {integer}, &str.

en rust on peut typer un tuple comme si c'était une variable.

```
    let my_tuple : (&str, i8, &str) = ("plop", 45, "pouet");
    Hello, world! ("plop", 45, "pouet")
```

On peut aussi mettre un tuple dans un tuple (une boite dans une boite)
et ensuite modifier les éléments avec de l'indexation.

```
    let my_tuple : (&str, i8, &str, (&str,f32)) = ("plop", 45, "pouet", ("yop", 12.432));
    Hello, world! ("plop", 45, "pouet", ("yop", 12.432))
```

On peut comme pour toutes les variable retirer la signature pour laisse le compilateur inférer (c'est à dire deviner).

Sauf que l'infèrement en reste :
- pour les entiers c'est du i32
- pour les flottant c'est du f64

et donc si on veut choisir il faut signer.


```
    let my_tuple = ("plop", 45, "pouet", ("yop", 12.432));
    Hello, world! ("plop", 45, "pouet", ("yop", 12.432))
```

On peut aussi en choisir d'en typer qu'une partie avec un "_"

```
    let my_tuple : (&str, i8, &str, (&str,_)) = ("plop", 45, "pouet", ("yop", 12.432));
    Hello, world! ("plop", 45, "pouet", ("yop", 12.432))
```

Modification du tuple:

```
    let mut my = ("name",42,12.54,false, ("plop",4));

    my.3 = true;
    my.4.1 = 5;

    Hello, world! ("name", 42, 12.54, true, ("plop", 5))
```

# <span style="color:red"> CONDITIONS

## <span style="color:blue"> if / else / else if
1:43:35 ep1

if <condition> { 
    <code>;
    }else if <condition> {
        <code>;
    } else {
        <code>;
    }


la condition doit être un booléen ou le résultat d'une opération qui sort un booléen

if () parenthèse pas nécessaires

```
    let x: i32 = 42;
    let y: bool = true;

    if y {
        println!("on est dans le if");
    }

    on est dans le if
```

ou

```
    let x: i32 = 42;
    let y: bool = false;

    if y == false {
        println!("on est dans le if");
    }

    on est dans le if
```

La première qui est vrai prends le pas sur les suivantes

```
    if x > 42 {
        println!("x strictement supérieur à 42");
    } else if x < 42 {
        println!("x strictement inférieur à 42")
    } else {
        println!("x égale à 0");
    }

x égale à 0
```

Le bloc de conditions peut-être affecté à une variable (potetiellement n'existe qu'en RUST)
Lorsque l'on manipule un bloc en rust -> on manipule en fait un statement 
-> et lorsque l'on a un statement qui ne se termine pas par un point virgule
-> le statement va être le retour du bloc qui est analysé

```
    if x > 42 {
        println!("x strictement supérieur à 42");
    } else if x < 42 {
        println!("x strictement inférieur à 42")
    } else {
        println!("x égale à 0");
    }

    let a :i32 = if y == true {
        6
    } else {
        42
    };

a vaut 42
```

L'intéret c'est aussi que l'on peut y mettre plein de logique dedans
ici on définit la valeur de a à partir de x.

```
    let a :i32 = if x == 41 && y == false {
        6
    } else if x == 42 {
        let b = 2 * x;
        b
    } else {
        666
    };

a vaut 84

```


dans les conditions on a aussi les AND / OR.

OR : <> || <>
AND : <> && <> 

```
    let a :i32 = if y == true || y == false {
        6
    } else if x == 42 {
        42
    } else {
        666
    };

a vaut 6


    let a :i32 = if x == 42 && y == false {
        6
    } else if x == 42 {
        42
    } else {
        666
    };

a vaut 6
```

Attention ici il vaut que les retours soit homogènes.
Typiquement ça n'est pas possible :

```
    let a :i32 = if x == 41 && y == false {
        "plop"
    } else if x == 42 {
        let b = 2 * x;
        b
    } else {
        666
    };
```


# <span style="color:red"> TABLEAU

Syntaxe : let tab [i32; 4] = [2, 4, 17, 19];


```
    let array = [2, 4, 17, 19];

    println!("Le tableau vaut {array:?}");
    println!("{}",array[2]); 

Le tableau vaut [2, 4, 17, 19]
17
```



On peut itérer dans le tableau avec une boucle.

# <span style="color:red"> BOUCLE

## <span style="color:blue"> Loop

syntaxe
```
loop {}
```
break pour en sortir

```
    loop {
        loop{
            break;
        }
        break;
    }
```

```
    loop {
        println!("counter est {counter}");

        if counter == 0 { 
            println!("BREAK");
            break; 
        } else {
                counter -= 1;
            } 
    }

counter est 3
counter est 2
counter est 1
counter est 0
BREAK
```

on peut s'amuser avec des sortie (équivalent aux GOTO)

```
    'sortie: loop {
        loop{
            break 'sortie; // ce break ne sort pas de la boucle imbriquée mais de la boucle principale
        }
    }
```

On peut aussi set une valeur de sortie - ici 2.

```
    let a = 'sortie: loop {
        loop{
            break 'sortie 2
        }
    };

    println!("a = {a}");

a = 2
```


## <span style="color:blue"> While

```
  while counter != 0 {
    println!("counter est {counter}");
    counter -= 1;
  };


counter est 3
counter est 2
counter est 1
```


# <span style="color:red"> STRUCTURE



# <span style="color:red"> PRINTLN

```
    println!("Hello, world!");
    println!("a vaut {a}");
    println!("Le tableau vaut {array:?}");
    println!("{}",array[2]);
```

# <span style="color:red"> FONCTION

déclaration : fn
fn <name_fonction> (<paramètres>) {}

# <span style="color:red"> MACRO

    println!("Hello, world!");

println est en faite une macro qui permet écriture sur la sortie standart.
Identifiable avec le"!" 



# <span style="color:red"> MEMOIRE

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


