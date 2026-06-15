use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let _server = thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            let mut buf = [0u8; 4];
            let _ = stream.read_exact(&mut buf);
            let _ = stream.write_all(b"PONG");
        }
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(b"PING").unwrap();
    let mut response = [0u8; 4];
    stream.read_exact(&mut response).unwrap();
    assert_eq!(&response, b"PONG");

    println!("Sync protocol test: PASS");
}
