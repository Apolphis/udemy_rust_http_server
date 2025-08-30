use std::net::{TcpListener, TcpStream};
use std::io::{Write,Read};
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
    
}

pub struct Server{
        ip_address: String,
    }

    impl Server{
        pub fn new(ip_address: String) -> Self {
            Self {
                ip_address
            }
        }
    
        pub fn run(self, mut handler: impl Handler){
            println!("server is listening on {}", self.ip_address);

            let listener = TcpListener::bind(&self.ip_address).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer){
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                                let response = match Request::try_from(&buffer[..]){
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),
                                
                                };

                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response: {}", e);
                                }
                            },
                            Err(e) => {
                                println!("failed to make a connection: {}", e);
                            },
                        }
                    },

                    Err(e) => {
                        println!("error: {}", e);
                    },
                }
            }
        }
    }