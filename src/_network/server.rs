use std::{net::*, io::Read};
use std::thread;
pub fn host(ip: &String, name: &str) {
    let listener = TcpListener::bind(ip).unwrap();
    println!("Started server on {} as {}", &ip, &name);
    thread::spawn(move ||{listen(&listener)});
}

fn listen(listener: &TcpListener) {
    println!("Listening for connections...");
    for stream in listener.incoming(){
        let mut tcp_stream = stream.unwrap();
        println!("Received connection from: {}", tcp_stream.peer_addr().unwrap());
        let mut buffer = [0; 1024];
        let result = tcp_stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..result]);
        println!("{}", message);
    }
}