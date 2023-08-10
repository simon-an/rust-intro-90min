# Typesystem - Struct Impl

```rust
# pub struct Animal{
#     name: String
# }
# impl std::fmt::Display for Animal {
#     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
#         write!(f, "{name}", name=self.name)
#     }
# }
impl Animal {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}
fn main() {
    let chicken = Animal {
        name: "Hubert".to_string()
    };
    let mut chicken = Animal::new("Hubert".to_string());
    // chicken.name = String::from("Albert"); compile error
    chicken.set_name(String::from("Albert"));
    println!("{chicken}");
}
```
