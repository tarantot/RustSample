// COMBINATION LOCK

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

// use rand::Rng;
use std::io::stdin; // input

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    // "1234" but to enter "125"
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED!");
                entry.clear(); // ""
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println! ("UNLOCKED!");
            }
        }
    }
}