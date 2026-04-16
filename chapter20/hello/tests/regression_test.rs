use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const SERVER_ADDR: &str = "127.0.0.1:7878";

/// Start the server as a child process.
/// The working directory is set to the crate root so that `hello.html` and
/// `404.html` can be found by the server.
fn start_server() -> Child {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Command::new("cargo")
        .args(["run", "--bin", "main"])
        .current_dir(manifest_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server")
}

/// Try to connect to the server, retrying until `timeout` elapses.
/// Returns the connected `TcpStream` on success so the caller can
/// immediately use it for the first request without wasting a connection
/// slot (the server only accepts 2 connections via `take(2)`).
fn connect_with_retry(timeout: Duration) -> TcpStream {
    let start = Instant::now();
    loop {
        match TcpStream::connect(SERVER_ADDR) {
            Ok(stream) => return stream,
            Err(e) => {
                if start.elapsed() >= timeout {
                    panic!("Failed to connect to server at {SERVER_ADDR} within {timeout:?}: {e}");
                }
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

/// Send an HTTP GET request on an existing TCP stream and return the full
/// response as a string.
fn send_request(mut stream: TcpStream, path: &str) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n");
    stream.write_all(request.as_bytes()).unwrap();

    let mut response = String::new();
    // The server closes the connection after writing the response,
    // so `read_to_string` will return once the stream is closed.
    let _ = stream.read_to_string(&mut response);
    response
}

/// Extract the first line (HTTP status line) from a response.
fn status_line(response: &str) -> &str {
    response.lines().next().unwrap_or("")
}

#[test]
fn test_root_returns_200_and_unknown_path_returns_404() {
    let mut server = start_server();

    // Connection 1: wait for the server to be ready, then request GET /
    let stream = connect_with_retry(Duration::from_secs(30));
    let response = send_request(stream, "/");

    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "Expected 200 OK for GET /, got: {}",
        status_line(&response),
    );
    assert!(
        response.contains("Hello!"),
        "Expected hello.html content in the 200 response body",
    );

    // Connection 2: request an unknown resource
    let stream = TcpStream::connect(SERVER_ADDR).expect("Failed to connect for the second request");
    let response = send_request(stream, "/nonexistent");

    assert!(
        response.starts_with("HTTP/1.1 404 NOT FOUND"),
        "Expected 404 NOT FOUND for GET /nonexistent, got: {}",
        status_line(&response),
    );
    assert!(
        response.contains("Sorry"),
        "Expected 404.html content in the 404 response body",
    );

    // The server accepts only 2 connections (`take(2)`), so it should
    // exit cleanly after we have consumed both slots.
    let status = server.wait().expect("Failed to wait for server process");
    assert!(
        status.success(),
        "Server exited with non-zero status: {status:?}",
    );
}
