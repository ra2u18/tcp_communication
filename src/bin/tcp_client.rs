use tokio::io;
use tokio::net::TcpStream;

use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct A {
    id: i8,
    key: i16
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind to a server socket
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    // stream.write_all(b"hello_world").await?;

    loop {
        let a = A { id: 42, key: 1336 };
        let bytes: &[u8] = &bincode::serialize(&a).unwrap()[..];

        stream.writable().await?;
        match stream.try_write(bytes) {
            Ok(n) => {
                println!("{:?}", &bytes[..n]);
                println!("write {} bytes", n);
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