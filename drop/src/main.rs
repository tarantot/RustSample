// DROP

#![allow(unused_mut)]
#![allow(unused_variables)]

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game.", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead...hoho", self.name);
    }
}

fn main () {
    let goblin = Creature::new("Jeff");
    println!("game proceeds...");
    drop(goblin);
}