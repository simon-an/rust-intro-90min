# main() and exit codes

```rust,editable
fn main() {
    let number = 5;
    print!("{}", number);
}

```
Fail with default exit code
```rust,editable,ignore
fn main() {
    panic!("Fail with exit code 101");
}
```

Other Exit Code

```rust,editable,ignore
fn main() {
    std::process::exit(66);
}
```