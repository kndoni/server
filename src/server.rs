use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

         loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found \r\n\n")
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            
                        }
                        Err(e) => println!("failed to read from coonection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
            // tuple let (stream, addr) = res.unwrap();
        }
    }
}
