use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::time;
use std::thread;

mod threadpool;
use threadpool::ThreadPool;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let a = b"GET / HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(a) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    let te = time::Duration::from_millis(10000);
    thread::sleep(te);
}

fn main() -> std::io::Result<()>{
    let listen = TcpListener::bind("127.0.0.1:8080")?;
    // let mut thread_vec:Vec<thread::JoinHandle<()>> = Vec::new();
    let thread_pool = ThreadPool::new(4);
    for stream in listen.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| handle_stream(stream));
    }

    Ok(())
}