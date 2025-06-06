use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server{
        ip_address: String,
    }

    impl Server{
        pub fn new(ip_address: String) -> Self {
            Self {
                ip_address
            }
        }
    
        pub fn run(self){
            println!("server is listening on {}", self.ip_address);

            let listener = TcpListener::bind(&self.ip_address).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer){
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                                match Request::try_from(&buffer[..]){
                                    Ok(request) => {},
                                    Err(e) => println!("failed to parse a request:{}", e),
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