use std::{any::Any, thread, time::Duration};

fn main() -> Result<(), Box<dyn Any + Send + 'static>> {
    let name = "Mauricio";
    println!("User: {name}");

    let handle = thread::spawn(move || {
        println!("{name} is the user");
        thread::sleep(Duration::from_secs(2));
    });

    handle.join()?;

    println!("{name} the program is finished");
    Ok(())
}
