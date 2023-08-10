# Typesystem - Trait + Impl

```rust,editable
# fn main() {

struct Eagle;

trait Bird{
    fn fly(&self);
}

impl Bird for Eagle {
    fn fly(&self){
        print!("I can fly!");
    }
}

Eagle.fly();

# }
```

<!-- > :bulb: **Tip:** Remember to appreciate the little things in life. -->