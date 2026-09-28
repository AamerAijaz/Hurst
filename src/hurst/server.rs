use std::{io::{Error, Write}, net::TcpListener};

use crate::hurst::{request::Request, response::Response};

#[derive(Debug)]
pub struct Server {
    address: String,
    port: String,
    listener: TcpListener
}

impl Server {
    pub fn new(address: &str, port: &str) -> Result<Self, Error> {
        let socket_addr = format!("{}:{}", address, port);
        let listener = TcpListener::bind(socket_addr)?;
        Ok(
            Self {
                address: String::from(address),
                port: String::from(port),
                listener: listener
            }
        )
    }

    pub fn serve(&self) {
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(stream) => {
                    println!("Accepted new connection");
                    stream
                },
                Err(error) => {
                    eprintln!("ERROR: Unable to accept new connection: {}", error);
                    continue;
                }
            };

            let request = match Request::new(&stream) {
                Ok(request) => request,
                Err(error) => {
                    eprintln!("ERROR: Unable to create request: {}", error);
                    continue;
                }
            };

            let mut response = Response::new(&stream);

            let body = "Not Found";
            let response_content = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );

            match response.writer.write_all(response_content.as_bytes()) {
                Ok(_) => {
                    println!("Written response");
                },
                Err(error) => {
                    eprintln!("ERROR: Unable to write data to connection: {}", error);
                    continue;
                }
            };
        }
    }
}
