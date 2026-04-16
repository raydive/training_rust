extern crate hello;
use hello::ThreadPool;
use std::fs::File;
use std::io::BufReader;
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
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Failed to read request line");
            return;
        }
    };

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_line == "GET /sleep HTTP/1.1" {
        // NOTE: The /sleep endpoint exists for educational purposes (demonstrating
        // thread pool behavior). In production, unprotected slow endpoints like this
        // are a denial-of-service risk and should be removed or rate-limited.
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = match File::open(filename).and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", filename, e);
            let body = "Internal Server Error";
            let response = format!(
                "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
            return;
        }
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}
