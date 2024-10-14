use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // for line in buf_reader.lines() {
    //     println!("{line:#?}")
    // }

    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";

    let html_file = fs::read_to_string("hello.html").unwrap();
    let length = html_file.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_file}");

    stream.write(response.as_bytes()).unwrap();
}
