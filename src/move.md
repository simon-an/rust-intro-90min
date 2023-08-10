# Ref, Owned and Move, Closures

```rust,editable
fn main() {
    
    let text = String::from("Hello World");
    let print = |text: String| { println!("{text}")};
    print(text);
    //println!("{text}"); Compile error

} 

```

```rust,editable
fn main() {
    
    let text = String::from("Hello World");
    let print = move || { println!("{text}")};
    print();
    // println!("{text}");

} 
```

```rust,editable
fn main() {
    
    let text = String::from("Hello World");
    let print = || { println!("{text}")};
    print();
    println!("{text}");

} 
```
```rust,editable
fn main() {
    
    let mut text = String::from("Hello World");
    let mut mutate = || { text = String::from("Hi"); };
    mutate();
    println!("{text}");

} 
```
```rust,editable
fn main() {
    
    let mut text = String::from("Hello World");
    let mut mutate = move || { text = String::from("Hi"); text };
    let text = mutate();
    println!("{text}");

} 
```

```rust,editable
fn main() {
    
    let text = String::from("Hello World");
    
    let print = |text: &String| {
        println!("{text}");
        let mut t = text.to_owned();
        t+="++";
        println!("{t}");
    };
    print(&text);

    println!("{text}");

} 

```