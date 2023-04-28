use std::thread::*;

#[macro_use]
extern crate error_chain;
error_chain! {}

fn main() -> Result<()> {
    let child: JoinHandle<Result<()>> = spawn(crash_burn);
    chain_any(child.join())
}

fn chain_any(
    x: std::result::Result<
        std::result::Result<(), Error>,
        std::boxed::Box<dyn std::any::Any + std::marker::Send>,
    >,
) -> Result<()> {
    match x {
        // Child did not panic, but might have thrown an error.
        Ok(x) => x.chain_err(|| "Child threw an error.")?,

        // Child panicked, let's try to print out the reason why.
        Err(e) => {
            println! {"{:?}", e.type_id()};
            if let Some(s) = e.downcast_ref::<&str>() {
                bail!(format!("Child panicked with this message:\n{}", s));
            } else if let Some(s) = e.downcast_ref::<String>() {
                bail!(format!("Child panicked with this message:\n{}", s));
            }
            bail!(format!(
                "Child panicked! Not sure why, here's the panic:\n{:?}",
                e
            ));
        }
    }
    Ok(())
}

fn crash_burn() -> Result<()> {
    // panic!(4);
    // panic!("OH no a &str!");
    panic!("OH no a {}", "String");
    //bail!("I'm an error.");
    //Ok(())
}
