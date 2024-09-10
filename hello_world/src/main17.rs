// STANDART COLLECTIONS

use std::collections::HashMap;
use std::collections::HashSet;

fn vectors () {
    let mut a = Vec::new();
    a.push(1); // adding an element
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    // usize vs isize
    let idx:usize = 0;
    
    println!("a[0] = {}", a[idx]);

    match a.get(2) { // option type
        Some(x) => println!("a[2] = {}", x),
        None => println!("error, no such element")
    }    

    for x in &a { println!("{}", x); }

    let last_element = a.pop(); // removes the last value and puts it into variable
    println!("the last elem is {:?}, a = {:?}", last_element, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn hashmap () {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3); // declaration: key, value
    shapes.insert(String::from("square"), 4);

    // println!("a square has {} sides!", shapes["square"].into());

    // println!("a square has {} sides!", shapes["circle"].into()); // would return an error

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.insert("square".into(), 5); // reassign a new value by the key
    println!("{:?}", shapes); // print everything

    shapes.entry("circle".into()).or_insert(1); { // append a new value
        let actual = shapes.entry("circle".into()).or_insert(2); // no changes because the value already there
        *actual = 0;
    }
}

fn hashset () {
    let mut greeks = HashSet::new(); // only unique elements
    greeks.insert("alpha");
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("alpha"); // inserting another element
    println!("{:?}", greeks); // nothing would change here

    greeks.insert("gamma"); // inserting new value

    if !greeks.contains("kappa") { // if not contains
        println!("we don't have it here.");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta!");
    }
    println!("{:?}", greeks);

    let _1_5:HashSet<_> = (1..=5).collect();
    let _6_10:HashSet<_> = (6..=10).collect();
    let _1_10:HashSet<_> = (1..=10).collect();
    let _2_8:HashSet<_> = (2..=8).collect();

    // subset
    println!("Is {:?} a subset of {:?}? {}", _2_8, _1_10, _2_8.is_subset(_1_10));

    // disjoint - no common elements
    println!("Is {:?} a subset of {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(_6_10));

    // union, intersection
    println!("items in either {:?} and {:?} are {:?}?", _2_8, _6_10, _2_8.union(&_6_10));

    // difference = union - intersection
    
}

fn iterators () {
    let mut vec = vec![3, 2, 1];

    for x in vec.iter() {
        println!("we got {}", *x);
    }

    for x in vec.iter_mut() {
        *x += 2;
    }

    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}", vec2);
}

fn main () {
    vectors();
    hashmap();
    hashset();
    iterators ();
}