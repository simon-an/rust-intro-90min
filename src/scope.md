# Scope

```rust,editable
fn main() {
    let mut number = 5;
    { // BLOCK
        let number = 8;
    }
    println!("{number}");

    let _new_number  = increment(5);
    
    let incrementor = |x: i32| -> i32 {
        x + 1
    };
    let new_number  = incrementor(5);

    println!("{new_number}");
} 

fn increment (x: i32) -> i32 {
    x + 1
}
```
