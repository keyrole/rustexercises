use std::net::TcpStream;
use std::{net::TcpListener, thread};
use std::io::{self, Error, Read, Write};
use std::time;
use std::str;


fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];
    for _ in 0..1000{
        let bytes_num = stream.read(&mut buf)?;
        if bytes_num == 0 {
            return Ok(());
        }

        println!("c:{}", str::from_utf8(&buf[0..bytes_num]).unwrap());

        stream.write(&buf[0..bytes_num])?;
        thread::sleep(time::Duration::from_secs(1_u64));
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let thread = thread::spawn(move || {
            let stream = stream.expect("failed!");
            handle_client(stream).unwrap_or_else(|err| eprintln!("{:?}", err));
        });

        thread_vec.push(thread);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}