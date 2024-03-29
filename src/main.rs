use std::{
    env, fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration
};

use http_rs::ThreadPool;

const CLRF: &str = "\r\n";

fn main() {
    let ip_addr = match env::var("HTTP_RS_ADDR") {
        Ok(addr) => addr,
        Err(_) => "127.0.0.1".to_string()
    };
    let port = match env::var("HTTP_RS_PORT") {
        Ok(port) => port,
        Err(_) => "7878".to_string()
    };
    let url = format!("{ip_addr}:{port}");

    let listener = TcpListener::bind(&url).unwrap();

    println!("Server listening on {url}");

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(format!("src/html/{filename}")).unwrap();
    let length = content.len();

    let response = format!("{status_line}{CLRF}Content-Length: {length}{CLRF}{CLRF}{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
