// SPAWNING AND JOINING THREADS

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

use std::thread;
use std::time;

fn main () {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("+");
            tread::sleep(time::Duration::from_millis(500));
        }
    })

    for _ in 1..10 {
        println!("-");
        tread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}