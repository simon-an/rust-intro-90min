use chrono::prelude::*;
fn main() {
    println!("{}", Utc::now().format("%Y-%m-%d-%H:%M:%S").to_string());
}
