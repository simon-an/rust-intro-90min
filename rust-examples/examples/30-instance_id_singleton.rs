use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref INSTANCE_ID: Uuid = Uuid::new_v4();
}

fn main() {
    println!("The INSTANCE_ID is {}.", *INSTANCE_ID);
    println!("The INSTANCE_ID is {}.", *INSTANCE_ID);
    println!("The INSTANCE_ID is {}.", *INSTANCE_ID);
    println!("The INSTANCE_ID is {}.", *INSTANCE_ID);
}
