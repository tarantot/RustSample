// CHARACTERS AND STRINGS

use std::thread;
use std::time;

fn strings () {
    // utf-8 characters
    let s:&'static str = "hello there!"; // &str = string slice
    s = "abc";
    // let h = s[0];
    
    for c in s.chars() { // getting all letters
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) { // getting the first letter
        println!("first letter is {}" first_char);
    }

    // heap String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    
    while a <= ('z' as u8) {
        letters.push(a as char);
        letter.push_str(",");
        a += 1;
    }

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String
    let z = letters + "abc";
    let z1 = letters + &letters;

    let mut abc = String::from("hello world");
    let mut abc1 = "hello world".to_string();

    abc.remove(0); // remove an element
    abc.push_str("!!!"); // add an element to the end
    println!("{}", abc.replace("ello", "goodbye")); // replacing elements
}

fn formatting () {
    let name = "James";
    let greeting = format!("hi, I'm {}, nice to meet you!", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    prinltn!("{}", hello_rust);

    let run = "run";
    let forrest = "Forrest";
    let rfr = format!("{0}, {1}, {0}!!!", run, forrest); // avoid repetition

    let info = format!("the name is {last}. {first} {last}.",
               first = "James",
               last = "Bond");
    println!(info);

    let mixed = format!("{1} {} {0} {} {data}",
                "alpha",
                "beta",
                // "gamma", // unused
                data = "delta");
    println!("{}", mixed);
}

fn main () {
    strings();
    formatting();
    number_guessing();
}