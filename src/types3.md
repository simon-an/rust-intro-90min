# Typesystem
1. ~~Trait~~
2. ~~Struct~~
3. Enum

```rust
# fn main() {
# pub struct Animal{
#     pub name: String,
#     legs: u32,
#     pub(crate) wings: u32
# }
# pub struct Dog{
#     pub name: String,
#     legs: u32
# }
pub enum AnimalType{
    Chicken(Animal),
    Dog(Animal),
    Other(String),
    BadTypesystemUse((String, String, u32, String, String)),
}
# }
```
