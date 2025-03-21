use std::{
    sync::{Arc, Mutex},
    thread,
};

use threads::error::ThreadError;

fn main() -> Result<(), ThreadError> {
    let value = Arc::new(Mutex::new(5));

    for _ in 1..3 {
        let value_safe = Arc::clone(&value);
        let handle = thread::spawn(move || {
            let mut data = value_safe.lock().unwrap();
            *data *= 2;
            println!("Data: {}", *data);
        });

        handle.join()?
    }

    println!("Data: {:?}", value);

    Ok(())
}
