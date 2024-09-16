// CONDITIONS

fn if_statement() {
    let temp = 25;

    if temp > 30 {
        println!("really hot outside");
    }
    else if temp < 10 {
        println!("cold day!");
    }
    else {
        println!("temperature is OK");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"}; // setting a variable
    println!("today is {}", day);

    println!("is it {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"it\'s OK"});
}

fn main() {
    if_statement();
}