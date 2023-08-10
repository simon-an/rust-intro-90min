# Mutable

```rust,editable
fn main() {
    let mut number = 5;
    let reference: &mut u32 = &mut number;
    *reference = 17;
    println!("{number}");
} 

```
