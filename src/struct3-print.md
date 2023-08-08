# Typesystem
1. ~~Trait~~
2. Struct
3. Enum

```rust
#[derive(Debug)]
pub struct Animal{
    pub name: String,
    legs: u32,
    pub(crate) wings: u32
}
impl std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{name} has {legs} legs and {wings} wings", name=self.name, legs=self.legs, wings=self.wings)
    }
}
fn main() {
    let chicken = Animal {
        name: "Hubert".to_string(),
        legs: 3,
        wings: 1
    };
    println!("{chicken}");
}
```
