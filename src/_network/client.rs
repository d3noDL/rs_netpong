use std::{net::*, io::Write};

pub fn join(ip: &String) {
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write_all(b"Client connected!").unwrap();
    println!("Connected to {}", stream.peer_addr().unwrap())
}