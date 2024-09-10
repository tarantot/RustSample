// LOOPS

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

fn while_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;
        if x == 64 { continue; } // skip execution with this value

        println!("x = {}", x);
    }

    let mut y = 1;
    loop // "while true"; never stops unless to break out
    {
        y *= 2;
        println!("y = {}", y);
        
        if y == 1<<10 { break; } // out of the loop
    }
}


fn for_loop ()
{
    for x in 1..11 // values from 1 to 10
    {
        if x == 3 { continue; }
        if x == 8 { break; }
        
        println!("x = {}", x);
    }

    for (pos, element) in (30..41).enumerate()
    {
        println!("index: {}, element: {}", pos, element);
    }
}

fn main () {
    for_loop();
    while_loop();
}