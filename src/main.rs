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

    let status_line: Vec<_> = http_request.get(0).unwrap().split(' ').collect();
    let uri = status_line[1];

    // println!("{uri:?}");
    // if uri == "/" {
    //     let success_response = "HTTP/1.1 200 OK\r\n\r\n";
    //     println!("success");
    //     stream.write(success_response.as_bytes()).unwrap();
    // } else {
    //     let failure_response = "HTTP/1.1 404 Not Found\r\n\r\n";
    //     println!("failure");
    //     stream.write(failure_response.as_bytes()).unwrap();
    // }

    let echo_uri = uri.split('/').collect::<Vec<_>>();
    println!("{echo_uri:?}");
    if echo_uri.len() == 3 {
        let echo_string = echo_uri[2];
        let echo_string_len = echo_string.len();
        println!("Request: {http_request:#?}");
        let echo_response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {echo_string_len}\r\n\
        \r\n\
        {echo_string}"
        );
        stream.write(echo_response.as_bytes()).unwrap();
        return;
    }

    let response = "HTTP/1.1 200 OK\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
