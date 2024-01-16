fn main() {

    let mut my: (&str, i32, f64, bool, (&str, i32))= ("name",42,12.54,false, ("plop",4));

    my.3 = true;
    my.4.1 = 5;

    println!("Hello, world! {my:?}");
}
