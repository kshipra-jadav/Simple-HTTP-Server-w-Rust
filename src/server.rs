use std::net::TcpListener;
use std::io::Read;

pub struct Server{
    address : String
}

impl Server{
    pub fn new(address : String) -> Self{
        Self{
            address
        }
    }

    pub fn run(self){
        println!("Server running on : {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();


        loop {
            match listener.accept(){
    
                Ok((mut stream, addr)) => {
                    println!("Connected : address : {}", addr);
                    // println!("Stream : {:?}", stream);

                    // for stream

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => println!("Request Recieved : {}", String::from_utf8_lossy(&buffer)),
                        Err(e) => println!("There is some error in reading : {}", e)
                    }
                },
                Err(err) => println!("Error! {}", err)
    
            }

        }

        
        
    }
}