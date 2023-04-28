use std::{any::Any, thread::JoinHandle};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyErrors {
    #[error("Inhertied Error")]
    InhertiedIoError(#[from] std::io::Error),
    #[error("Child panicked with this message:\n{0}")]
    ChildPanic(String),
    #[error("Child panicked! Not sure why, here's the panic:\n{0:?}")]
    Panic(Box<dyn Any + Send>),
}

fn main() -> Result<(), MyErrors> {
    let child: JoinHandle<Result<(), MyErrors>> =
        std::thread::spawn(move || -> Result<(), MyErrors> {
            crash_burn().map_err(MyErrors::from)?;
            Ok(())
        });
    let result = chain_any(child.join());

    //Ok(result.unwrap_or_else(|e| println!("{}", e)))
    result
}

fn chain_any(
    x: std::result::Result<
        std::result::Result<(), MyErrors>,
        std::boxed::Box<dyn std::any::Any + std::marker::Send>,
    >,
) -> Result<(), MyErrors> {
    // println! {"chain_any:{:?}", x};
    match x {
        Ok(x) => x,

        // Child panicked, let's try to print out the reason why.
        Err(e) => {
            // Child panicked, let's try to print out the reason why.
            println! {"{:?}", e.type_id()};
            if let Some(s) = e.downcast_ref::<&str>() {
                Err(MyErrors::ChildPanic(s.to_string()))
            } else if let Some(s) = e.downcast_ref::<String>() {
                Err(MyErrors::ChildPanic(s.to_string()))
            } else {
                Err(MyErrors::Panic(e))
            }
        }
    }
}

fn crash_burn() -> Result<(), std::io::Error> {
    // panic!(4);
    //panic!("OH no a &str!");
    //panic!("OH no a {}", "String");
    Err(std::io::Error::from_raw_os_error(22))
    //Ok(())
}
