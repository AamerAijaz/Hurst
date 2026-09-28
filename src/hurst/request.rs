use std::{collections::HashMap, io::{BufRead, BufReader, Error, ErrorKind}, net::TcpStream};

#[derive(Debug)]
pub enum Method {
    Post,
    Get
}

pub type Header = HashMap<String, String>;

#[derive(Debug)]
pub struct Request<'a> {
    reader: BufReader<&'a TcpStream>,
    method: Method,
    path: String,
    header: Header,
}

impl<'a> Request<'a> {
    fn read_request_line(reader: &mut BufReader<&'a TcpStream>) -> Result<(String, String), Error> {
        let mut request_line: String = String::new();

        let n = reader.read_line(&mut request_line)?;
        let mut request_parts = request_line
            .as_str()[0..n]
            .split(" ");

        let method = request_parts.nth(0);
        if method.is_none() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid request line"));
        }

        let path = request_parts.nth(0);
        if path.is_none() {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid request line"));
        }

        return Ok((String::from(method.unwrap()), String::from(path.unwrap())));
    }

    
    fn read_request_header(reader: &mut BufReader<&'a TcpStream>) -> Result<Header, Error> {
        let mut header: Header = HashMap::new();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    eprintln!("ERROR: Unable to read request header: {}", error);
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid request line"));
                }
            };

            // Signals the end of request headers.
            if line == "" {
                break;
            }

            let (key, value) = match line.split_once(":") {
                Some((key, value)) => (key, value),
                None => {
                    eprintln!("ERROR: Invalid request header");
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid request header"));
                }
            };

            let _ = header.insert(key.trim().to_string(), value.trim().to_string());
        }

        return Ok(header);
    }
    
    pub fn new(stream: &'a TcpStream) -> Result<Request<'a>, Error>{
        let mut reader = BufReader::new(stream);
        
        let (method, path) = Request::read_request_line(&mut reader)?;

        let mut req_method = Method::Get;
        if method == "POST" {
            req_method = Method::Post;
        }

        let header = Request::read_request_header(&mut reader)?;

        return Ok(Request {
            reader: reader,
            method: req_method,
            path: path,
            header: header,
        });
    }
}
