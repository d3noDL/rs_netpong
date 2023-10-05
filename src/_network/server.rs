use std::{net::*, io::Read};


pub fn host(ip: &String) {
    let listener = TcpListener::bind(ip).unwrap();
    let listener_true = listener.try_clone().unwrap();
    listen(&listener_true);
}

fn listen(listener: &TcpListener) {
    for stream in listener.incoming(){
        let mut tcp_stream = stream.unwrap();
        println!("Received connection from: {}", tcp_stream.peer_addr().unwrap());
        let mut buffer = [0; 1024];
        let result = tcp_stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..result]);
        println!("{}", message);
    }
}