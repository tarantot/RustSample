// MATCH STATEMENT

#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]

fn main()
{
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        254 => "Kenya",
        _ => "unknown"
    };

    println! ("The country with the code {} is {}.", country_code, country);

    let x = false;

    let s = match x { 
        true => "yes",
        false => "no"
        
        // !!! need to cover all the possible values for the given datatype
        // otherwise the program will fail to compile
    };
}