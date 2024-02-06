
/*
    let matrice : [[u8; 2]; 2] = [[2,4],[1,3]];

  let array: [i32; 4] = [2, 4, 17, 19];
  let size = 4;
  let mut iteration = 0;

  loop {
    if iteration < size {
        println!("iteration est {iteration}");
        println!("{}",array[iteration]);
        iteration += 1
    } else {
        break
    }
  };

  iteration=0;

while iteration < size {
    println!("iteration est {iteration}");
    println!("{}",array[iteration]);
    iteration += 1

}

for value in array {
  println!("value for : {value}")
}


   // println!("Le tableau vaut {array:?}");
   // println!("{}",array[2]);
   // println!("La matrice vaut {matrice:?}");
   // println!("{}",matrice[1.1]);



struct Person {
  nom: &'static str,
  prenom: &'static str,
  age: u8,
  adresse: Adresse,
}

struct Adresse {
  ville : &'static str,
  code_postal: u32,
}

let adress = Adresse {
  ville : "Paris",
  code_postal : 75020,
};

let mut alice = Person {
  nom : "Dupont",
  prenom : "alice",
  age: 9,
  adresse : adress,
};

alice.age = alice.age + 1;

alice.adresse.ville="Brest";


println!("Age de alice: {}", alice.age);
println!("Ville de alice: {}", alice.adresse.ville);
*/

fn main() {
  let data1 = Data{
    a:3,
  };

  let a = hello(data1);
  println!("Hello {}", a.a);
}

struct Data {
  a :i32
}

fn hello (mut param1 : Data) -> Data{
  println!("Hello");
  param1.a=4;
  param1 
}