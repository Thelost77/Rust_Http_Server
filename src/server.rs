use std::net::TcpListener;
use std::io::Read;
use request::Request;
use std::convert::TryFrom;
use crate::http::request;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self{
            addr
        }
    }
    pub fn run(self) {
        println!("Listening on port {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recived a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request with error {}", e), 
                            }

                        },
                        Err(e) => println!("Error - {} occured during reading from stream", e)
                    }
                },
                Err(e) => println!("Error occured during accepting listener {}!", e),
            }
        }
    }
}