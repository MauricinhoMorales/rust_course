use std::net::SocketAddr;

use error::AsyncError;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

pub mod error;

#[tokio::main]
async fn main() -> Result<(), AsyncError> {
    let listener = TcpListener::bind("localhost:8080").await?;
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);
    let exit = false;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let result: Result<(), AsyncError> = async {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();
                loop {
                    tokio::select! {
                        bytes_read = reader.read_line(&mut line) => {
                            if bytes_read? == 0 {
                                break;
                            }
                            tx.send((line.clone(), addr))?;
                            line.clear();
                        }
                        result = rx.recv() => {
                            let (message, address) = result?;
                            if address != addr {
                                writer.write_all(message.as_bytes()).await?
                            }
                        }
                    }
                }
                Ok(())
            }
            .await;

            if let Err(e) = result {
                eprint!("Error Handling connection: {}", e);
            }
        });
        if exit {
            break;
        }
    }
    Ok(())
}
