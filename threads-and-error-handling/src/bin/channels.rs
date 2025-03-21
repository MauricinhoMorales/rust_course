use std::sync::mpsc::{self, SendError};
use std::{thread, time::Duration};

use threads::error::ThreadError;

fn main() -> Result<(), ThreadError> {
    let (tx, rx) = mpsc::channel::<String>();
    let name = "Mauricio";
    println!("User: {name}");

    let _ = thread::spawn(move || -> Result<(), SendError<String>> {
        let message = format!("{name} is the user");
        thread::sleep(Duration::from_secs(2));

        tx.send(message)?;
        Ok(())
    });

    let message = rx.recv()?;

    println!("The Message is: {message}");
    Ok(())
}
