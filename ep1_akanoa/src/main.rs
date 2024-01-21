fn main() {

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

   // println!("Le tableau vaut {array:?}");
   // println!("{}",array[2]);
   // println!("La matrice vaut {matrice:?}");
   // println!("{}",matrice[1.1]);
}