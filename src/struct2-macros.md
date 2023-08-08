# Typesystem
1. ~~Trait~~
2. Struct
3. Enum

```rust
#[derive(Default, Clone, Debug)]
pub struct Animal{
    pub name: String,
}
fn main() {
    let animal = Animal::default();
    let mut second_animal = animal.clone();
    second_animal.name = String::from("");
    println!("{:?}", animal);
    println!("{:?}", second_animal);
}
```
