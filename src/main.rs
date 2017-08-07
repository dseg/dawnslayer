use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
//use std::str;

const RPC_PORT: i32 = 58332;

const NAME: &'static str = "DawnSlayer";
const VER: &'static str = "0.0.1";

fn ua() -> String {
    return format!("{}/{}", NAME, VER);
}

fn handle_client(mut stream: TcpStream) {
    thread::spawn(move || { stream.write(b"Hello World\r\n").unwrap(); });
}

fn handle_mainnet(mut stream: TcpStream) {}

fn main() {
    let rpc_listener = TcpListener::bind(format!("127.0.0.1:{}", RPC_PORT)).unwrap();
    println!("{} listening started, ready to accept connections.", ua());

    // Waiting for connections.
    for stream in rpc_listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // Connect to the full node on local host, standard port.
    let mainnet_listener = TcpListener::bind("127.0.0.1:8333").unwrap();
    for stream in mainnet_listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_mainnet(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
