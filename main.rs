use std::net::{TcpListener, TcpStream};

pub fn start_network(port: &str) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Network started on port {}", port);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("New connection established");
    }
}
