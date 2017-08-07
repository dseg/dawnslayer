use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
     thread::spawn(move || {
        stream.write(b"Hello World\r\n").unwrap();
     });
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:54321").unwrap();
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { 
                /* connection failed */ 
            }
        }
    }
}
