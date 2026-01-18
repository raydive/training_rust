use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Handle the connection
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let responce = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}