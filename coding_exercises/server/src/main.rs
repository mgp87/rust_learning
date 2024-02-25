use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn main() {
    // Init server
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Server started on 127.0.0.1:8000");

    // Listen connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        // Handle connections
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        send_response(stream);
    }else{
        send_404(stream);
    }
}

fn build_response(content:String) -> String {
    // Carriage return (CR --> \r) and line feed (CRLF --> \n) characters are used to separate the headers from the body
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}

fn send_response(mut stream: TcpStream){
    let content = fs::read_to_string("index.html").unwrap();
    stream.write(build_response(content).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_404(mut stream: TcpStream){
    let content = fs::read_to_string("404.html").unwrap();
    stream.write(build_response(content).as_bytes()).unwrap();
    stream.flush().unwrap();
}