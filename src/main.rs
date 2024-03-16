use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const CLRF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection_selectively(stream);
    }
}
fn handle_connection_selectively(mut stream: TcpStream) {
    println!("stream: {:?}", stream);
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let content = fs::read_to_string(format!("src/html/{filename}")).unwrap();
    let length = content.len();

    let response = format!("{status_line}{CLRF}Content-Length: {length}{CLRF}{CLRF}{content}");

    stream.write_all(response.as_bytes()).unwrap();
}