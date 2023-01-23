use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpStream};

const ECHO_SERVER_ADDRESS: &str = "localhost:8080";

#[tokio::main]
async fn main() {
    // connection
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await{
        // connected
        println!("connected to echo server {}:{}",
                 stream.local_addr().unwrap().ip(),
                 stream.local_addr().unwrap().port(),
        );
        // write message
        let message = "Hello world";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);
        // read result
        let mut buffer = [0;1024];
        let _len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);
    } else {
        println!("Error connecting to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
