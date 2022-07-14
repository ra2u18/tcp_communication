use tokio::io;
use tokio::net::{TcpListener, TcpStream};

use std::error::Error;
use serde::{Serialize, Deserialize};

use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct A {
    id: i8,
    key: i16,
    timestamp: u128
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    match listener.accept().await {
        Ok((socket, addr)) => {
            println!("new client: {:?}", addr);
            handle_client(socket).await?;
        },
        Err(e) => println!("couldn't get client {:?}", e)
    }

    Ok(())
}

async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data = vec![0 as u8; std::mem::size_of::<A>()];

    loop {
        stream.readable().await?;

        match stream.try_read(&mut data) {
            Ok(n) => {
                println!("{:?}", data);
                break;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    
    let decoded: A = bincode::deserialize(&data).unwrap();

    let start = SystemTime::now();
    let since_the_epoch_ns = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    println!("{:?}", since_the_epoch_ns);

    let time_elapsed = Duration::new(0, (since_the_epoch_ns - decoded.timestamp) as u32);

    println!("{:?}ns", time_elapsed.as_secs_f64());

    println!("{:?}", decoded);
    Ok(())
}

