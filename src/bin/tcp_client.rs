use tokio::io;
use tokio::net::TcpStream;

use serde::{Serialize, Deserialize};
use std::error::Error;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct A {
    id: i8,
    key: i16,
    timestamp: u128
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind to a server socket
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    let start = SystemTime::now();
    let since_the_epoch_ns = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    loop {
        println!("{:?}", since_the_epoch_ns);
        let a = A { id: 42, key: 1336, timestamp: since_the_epoch_ns };
        let bytes: &[u8] = &bincode::serialize(&a).unwrap()[..];

        stream.writable().await?;
        match stream.try_write(bytes) {
            Ok(n) => {
                println!("{:?}", &bytes[..n]);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }

        println!("{:?}", bytes);
    }

    Ok(())
}