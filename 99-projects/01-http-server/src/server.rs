use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        return Self {
            addr: addr.to_string(),
        };
    }

    // taking ownership
    pub fn run(self, mut handler: impl Handler) {
        println!("Addr: {}", self.addr);

        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connected {}", addr);

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(bytes) => {
                            // println!("Data: {:?}", String::from_utf8_lossy(&buffer[..bytes]));

                            let response = match Request::try_from(&buffer[..bytes]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
    }
}
