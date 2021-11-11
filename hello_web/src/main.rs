use hello_web::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Tcp connection established");
    }
}

fn handle_connection(mut stream : TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";  // b->bytes
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}