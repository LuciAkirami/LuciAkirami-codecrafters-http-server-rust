// Uncomment this block to pass the first stage
#[allow(unused_imports)]
use std::io::{prelude::*, BufReader, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //let mut streamer = TcpStream::connect("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                //println!("{:?}");
                handle_connetions(_stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    //streamer.read(&mut [0; 128]);
}

fn handle_connetions(mut stream: TcpStream) {
    let buffer = BufReader::new(&mut stream);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|result| !result.is_empty())
        .collect();
    let path = http_request.get(0).unwrap();
    let fourth_value = path.char_indices().collect::<Vec<_>>()[4].1;

    if fourth_value == '/' {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }

    //println!("{indices:#?}");
    //println!("Request: {http_request:#?}");
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
