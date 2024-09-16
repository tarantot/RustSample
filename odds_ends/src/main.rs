// ODDS & ENDS

// crates.io - to get packages for rust

extern crate rand;
// use rand::Rng;
extern crate phrases;

use phrases::greetings::french; // importing a part of code

fn main () {
    // Consuming a crate
//    let mut rng = rand::thread_rng();
//    let b:bool = rng.gen();

    println!("English: {}, {}", 
    phrases::greetings::english::hello(),
    phrases::greetings::english::goodbye());

    println!("French: {}, {}", 
    french::hello(),
    french::goodbye());
}