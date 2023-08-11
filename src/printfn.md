# Print
```rust,editable
fn main() {
    
    let text = String::from("Hello World");
    println!("{text}");
    println!("{}", text);
    println!("{text}", text=text);

    let formatted = format!("{text} from Munich", text=text);
    println!("{formatted}"); 
} 

```