# Result, Option, Match

```rust,editable
fn main() {
    
    let maybe_string: Option<String> = Some(String::from("Hello World"));

    println!("{:?}", maybe_string);

    let list = vec![None, Some(String::from("Hello World"))];

    list.iter().for_each(|maybe| {

        if maybe.is_some(){
            println!("some");
        }
        if maybe.is_none(){
            println!("none");
        }
    });

} 
```

```rust,editable
fn main() {
    let maybe_string: Option<String> = Some(String::from("Hello World"));

    let the_string = maybe_string.unwrap();
    println!("{the_string}");

    let maybe_string = None;
    let the_string = maybe_string.unwrap_or(String::from("Default String"));
    println!("{the_string}");
    
    let maybe_string = None;
    let the_string = maybe_string.unwrap_or_else(|| String::from("Default String"));
    println!("{the_string}");
    
    let maybe_string = None;
    // Type inference!
    let the_string: String = maybe_string.unwrap_or_default();
    println!("{the_string}");
    println!("{:?}", the_string);
}
```

```rust,editable
fn parse(thing: &str) -> u32 {
    return thing.parse().expect("Not a number");
}
fn try_parse(thing: &str) -> Result<u32, &str> {
    // 'turbofish': ::<>
    let val: Result<u32, std::num::ParseIntError> = thing.parse::<u32>();
    val.map_err(|_e| "Not a number")
}
fn main() {
    let x = parse("5");
    println!("x is {}", x);

    let res = try_parse("5");
    match res {
        Ok(x) => println!("x is {}", x),
        Err(e) => println!("Error: {}", e),
    }
}
```
