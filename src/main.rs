// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
     for stream in listener.incoming() {
        //let stream = stream.unwrap();
        thread::spawn(|| {handle_connection(stream);});
        
     }
}

fn handle_connection(stream: Result<TcpStream, std::io::Error>) {

    match stream {
        Ok(_stream) => {

            let mut n = 1;
            let mut buffer = [0;1024];
            let mut _streamer = _stream;
            while n!=0 {

               
               n = _streamer.read(&mut buffer).unwrap();
               if *buffer.get(0).unwrap() != b'*' {

                let response = "+PONG\r\n";
               
                _streamer.write_all(response.as_bytes()).unwrap();
               }
               else {
                    let s = String::from_utf8(buffer.to_vec()).unwrap();
                    if &s[4..14] == "$4\r\nECHO\r\n" {

                        let len : usize =s[15..16].parse().unwrap();
                        let mut message = String::from("+");
                        message.push_str(&String::from_utf8(buffer[18..(18+len)].to_vec()).unwrap());
                        message.push_str("\r\n");
                        _streamer.write_all(message.as_bytes()).unwrap();
                    }
               }

            }
            
            
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

