
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn send_resp_from_file(mut stream: TcpStream, code: usize, filename: String) {
    let code_msg = match code {
        200 => "OK",
        404 => "NOT FOUND",
        500 => "SERVER ERROR",
        _ => "RESPONCE",
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        code,
        code_msg,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn home_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/home.html".to_string());
}

pub fn new_tab_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/new_tab.html".to_string());
}

pub fn page_does_not_exist(mut stream: TcpStream) {
    send_resp_from_file(stream, 404, "html/404.html".to_string());
}

pub fn styles_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "css/styles.css".to_string());
}
