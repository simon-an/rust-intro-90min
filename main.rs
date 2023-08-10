fn main() {
    let x = parse("5");
    println!("x is {}", x);

    let res = try_parse("5");
    match res {
        Ok(x) => println!("x is {}", x),
        Err(e) => println!("Error: {}", e),
    }
}

fn parse(thing: &str) -> u32 {
    return thing.parse().expect("Not a number");
}
fn try_parse(thing: &str) -> Result<u32, &str> {
    // 'turbofish': ::<>
    let val: Result<u32, std::num::ParseIntError> = thing.parse::<u32>();
    val.map_err(|_e| "Not a number")
}
