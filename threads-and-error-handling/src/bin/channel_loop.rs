use std::sync::mpsc::{self, SendError};
use std::{thread, time::Duration};

use threads::error::ThreadError;

fn main() -> Result<(), ThreadError> {
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();
    let name = "Mauricio";
    println!("User: {name}");

    let _thread_1 = thread::spawn(move || -> Result<(), SendError<String>> {
        for count in 0..3 {
            let message = format!("{count} is the value for Thread1");
            thread::sleep(Duration::from_secs(2));
            tx.send(message)?;
        }
        Ok(())
    });

    let _thread_2 = thread::spawn(move || -> Result<(), SendError<String>> {
        for count in 0..3 {
            let message = format!("{count} is the value for Thread2");
            thread::sleep(Duration::from_secs(2));
            tx2.send(message)?;
        }
        Ok(())
    });

    for message in rx {
        println!("The Message is: {message}");
    }
    Ok(())
}
