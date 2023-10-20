// Uncomment this block to pass the first stage
//use clap::Parser;
#![allow(unused_imports)]
use std::io::{prelude::*, BufReader, Read, Write};
use std::net::TcpStream;
use std::net::{Shutdown, TcpListener};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, fs, fs::File};

// #[derive(Parser, Debug)]
// struct Args {
//     directory: String,
// }

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    //let dir_placeholder = env::args().skip(2).collect::<Vec<_>>();
    //let cloned_placeholder: Vec<String> = dir_placeholder.clone();

    // let dir: &'static str = match cloned_placeholder.get(0) {
    //     Some(value) => value,
    //     None => "",
    // };
    let dir = env::args().nth(2).unwrap_or("None".to_string());
    //let noone = "None".to_string();
    //let mut dir = cloned_placeholder.get(0).unwrap_or(&noone).clone();
    //let mut k = Arc::new(Mutex::new(dir));
    //dbg!(dir);

    // let dir_path = String::from("codecrafters-http-server-rust");

    // let new_path = dir_path.clone() + "/your_server.sh";
    // // let p = fs::read_dir(dir_path)
    // //     .unwrap()
    // //     .map(|dir| dir.unwrap())
    // //     .collect::<Vec<_>>();
    // // dbg!(p.last());

    // let path_exists = Path::new(&new_path).exists();
    // dbg!(path_exists);

    // for entry in fs::read_dir(&dir_path).unwrap() {
    //     let dir = entry.unwrap();
    //     let dir_list = dir.path();
    //     println!("{dir_list:?}");
    // }

    // let args = Args::parse();
    // dbg!(args.directory);
    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //let mut streamer = TcpStream::connect("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                //println!("{:?}");
                //let new_dir = Arc::clone(&k);
                let base_dir = dir.clone();
                thread::spawn(move || {
                    handle_connetions(_stream, &base_dir);
                });
                // let my_closure = || handle_connetions(_stream, &dir);
                // thread::scope(|scope| {
                //     scope.spawn(my_closure);
                // });

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    //streamer.read(&mut [0; 128]);
}

fn handle_connetions(mut stream: TcpStream, dir: &str) {
    //let mut other_stream = stream.try_clone().unwrap();
    let mut buffer = BufReader::new(&mut stream);
    //let extra_buffer = BufReader::new(&mut other_stream);
    // let http_request: Vec<_> = buffer
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|result| !result.is_empty())
    //     .collect();

    let mut http_request: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        buffer.read_line(&mut line).unwrap();
        if line == "\r\n" || line == "\n" {
            break; // End of headers
        }
        http_request.push(line);
    }

    let mut body: Vec<u8> = vec![101];

    if http_request.iter().any(|line| line.starts_with("POST")) {
        // Handle POST request

        // Parse the Content-Length header to determine the body size
        let content_length: usize = http_request
            .iter()
            .filter(|line| line.starts_with("Content-Length"))
            .flat_map(|line| line.split_whitespace().nth(1))
            .filter_map(|length| length.parse().ok())
            .next()
            .unwrap_or(0);

        // Read the request body
        body = Vec::with_capacity(content_length);
        buffer
            .take(content_length as u64)
            .read_to_end(&mut body)
            .unwrap();
        println!("Body: {body:?}");
    }

    let status_line: Vec<_> = http_request.get(0).unwrap().split(' ').collect();
    let uri = status_line[1];

    println!("{uri:?}");
    if uri == "/" {
        let success_response = "HTTP/1.1 200 OK\r\n\r\n";
        println!("success");
        stream.write(success_response.as_bytes()).unwrap();
        return;
    }
    // else {
    //     let failure_response = "HTTP/1.1 404 Not Found\r\n\r\n";
    //     println!("failure");
    //     stream.write(failure_response.as_bytes()).unwrap();
    // }

    let echo_uri = uri.split('/').collect::<Vec<_>>();
    println!("{echo_uri:?}");
    if echo_uri.len() >= 2 && echo_uri[1] == "echo" {
        let echo_string = &echo_uri[2..];
        let echo_string_joined = echo_string.to_owned().join("/");
        let echo_string_len = echo_string_joined.len();
        println!("{echo_string:?}, {echo_string_joined:?}, {echo_string_len:?}");
        println!("Request: {http_request:#?}");
        let echo_response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {echo_string_len}\r\n\
        \r\n\
        {echo_string_joined}"
        );
        stream.write(echo_response.as_bytes()).unwrap();
        return;
    }

    if uri == "/user-agent" {
        let user_agent_line = http_request.get(2).unwrap();
        let user_agent = user_agent_line.split(": ").collect::<Vec<_>>()[1];
        let user_agent_len = user_agent.len() - 2;
        let user_agent_response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {user_agent_len}\r\n\
        \r\n\
        {user_agent}"
        );
        println!("{user_agent:?}");
        println!("{user_agent_response:#?}");
        stream.write(user_agent_response.as_bytes()).unwrap();
        return;
    }
    dbg!(uri);
    let file_uri = uri.split('/').collect::<Vec<_>>();
    if file_uri.len() > 2 && file_uri[1] == "files" && status_line[0] == "GET" {
        let dir_path = dir.clone().to_string();

        let file_path = dir_path + "/" + file_uri[2];

        let file_exists = Path::new(&file_path).exists();
        println!("{file_path:?} {file_exists:?}");
        if file_exists {
            println!("{file_path:?}");
            let mut file = File::open(file_path).unwrap();

            println!("{file:?}");

            // Read the file and send it over the stream
            let mut buffer = [0; 1024];
            let mut _total_bytes_sent = 0;

            // Build the response with the Content-Type header
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                file.metadata().unwrap().len()
            );

            // Send the response header
            stream.write(response.as_bytes()).unwrap();

            // Send the file content
            loop {
                let bytes_read = file.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break; // End of file
                }

                let bytes_sent = stream.write(&buffer[0..bytes_read]).unwrap();
                _total_bytes_sent += bytes_sent;
            }
            println!("total bytes sent: {_total_bytes_sent}");
            return;
        }
    }

    if file_uri.len() > 2 && file_uri[1] == "files" && status_line[0] == "POST" {
        let dir_path = dir.clone().to_string();
        let file_path = dir_path + "/" + file_uri[2];

        let mut file = File::create(file_path).unwrap();

        //let mut buffer2 = [0; 1024];
        let mut _total_bytes_sent = 0;

        //let mut request = String::new();
        //other_stream.read_to_string(&mut request).unwrap();

        println!("{http_request:#?}");
        // loop {
        //     let bytes_read = stream.read(&mut buffer2).unwrap();
        //     if bytes_read == 0 {
        //         break; // End of file
        //     }

        //     let bytes_sent = file.write(&buffer2[0..bytes_read]).unwrap();
        //     _total_bytes_sent += bytes_sent;
        // }
        //let contents = &http_request[7];
        println!("Body: {body:?}");
        file.write(&body).unwrap();

        println!("total bytes received: {_total_bytes_sent}");

        let response = "HTTP/1.1 201\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
        return;
    }
    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
