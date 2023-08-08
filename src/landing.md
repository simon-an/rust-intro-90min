# Hello World

```rust
fn main() {
    println!("hello rustaceans 🦀!");
}
```

# Goals:

- KISS - Keep it stupid simple
- Syntax, Semantics
- Hands-On

# Agenda

1. short intro
2. start coding
3. Afterwards: Keep coding.

# Simon's Recommendations

1. Use clone,ref,as_ref to mute the borrow checker
2. Avoid Box, Cell, RefCell, Arc, Mutex, etc
3. Avoid Generics in the beginning
4. Avoid writing your own macros
5. Aim for understanding async and multithreading, channels soon (very valuable) -> Next Session
6. Now you need Arc and maybe Mutex
7. Build a small real application (cli tool, website, Client App)
8. Forget about OOD. Rust is all about procedural programming with a strong typesystem. https://doc.rust-lang.org/book/ch17-00-oop.html

Rust supports a mixture of imperative procedural, concurrent actor, object-oriented and pure functional styles. It also supports generic programming and metaprogramming, in both static and dynamic styles.