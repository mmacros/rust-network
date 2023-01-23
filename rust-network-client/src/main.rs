use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8080";


fn main() {
    // connection
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS){
        // connected
        println!("connected to echo server {}:{}",
                 stream.local_addr().unwrap().ip(),
                 stream.local_addr().unwrap().port(),
        );
        // write message
        let message = "Hello world";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent: {}", message);
        // read result
        let mut buffer = [0;1024];
        let _len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);
    } else {
        println!("Error connecting to echo server {}", ECHO_SERVER_ADDRESS);
    }

}
