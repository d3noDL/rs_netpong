use std::{net::*, io::Write};

pub fn join(ip: &String, name: &str) {
    let mut stream = TcpStream::connect(ip).unwrap();
    stream.write_all(format!("{} connected!", name).as_bytes()).unwrap();
    println!("Connected to {}", stream.peer_addr().unwrap())
}