extern crate hello;
use hello::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

/*
$ cargo run                                                                                                     [git][main] -?
   Compiling hello v0.1.0 (/Users/arata_n/Documents/dev/training_rust/chapter20/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/main`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
Shutting down worker 0
Worker 1 got a job; executing.
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Worker 1 was told to terminate.
Worker 0 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
 */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // Handle the connection
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
