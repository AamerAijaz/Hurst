use std::{net::TcpStream, io::BufWriter};

#[derive(Debug)]
pub struct Response<'a> {
    pub writer: BufWriter<&'a TcpStream>,
}

impl<'a> Response<'a> {
    pub fn new(stream: &'a TcpStream) -> Response<'a> {
        let writer = BufWriter::new(stream);
        Response {
            writer: writer
        }
    }
}