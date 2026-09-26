use std::{io::Write, net::TcpListener};

fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!("ERROR: Unable to bind to port: {}", err);
        panic!("Unable to bind to port");
    });

    let body = "Hello World";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("ERROR: Unable to accept new connection: {}", error);
                continue;
            }
        };

        println!("Accepted new connection");

        _ = match stream.write_all(response.as_bytes()) {
            Ok(n) => n,
            Err(error) => {
                eprintln!("ERROR: Unable to write data to connection: {}", error);
                continue;
            }
        };

        println!("Written data to connection");
    }
}
