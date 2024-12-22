
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use sqlx::Row;
use std::error::Error;
use pwhash::bcrypt;

use crate::models::tab::Tab;


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

fn send_success(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn home_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/home.html".to_string());
}

pub fn new_tab_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/new_tab.html".to_string());
}

pub fn tab_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/tab.html".to_string());
}

pub fn login_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/login.html".to_string());
}

pub fn register_page(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "html/register.html".to_string());
}

pub fn page_does_not_exist(mut stream: TcpStream) {
    send_resp_from_file(stream, 404, "html/404.html".to_string());
}

pub fn styles_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "css/styles.css".to_string());
}

pub fn new_tab_js_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "js/new_tab.js".to_string());
}

pub fn list_js_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "js/list.js".to_string());
}

pub fn tab_js_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "js/tab.js".to_string());
}

pub fn register_js_file(mut stream: TcpStream) {
    send_resp_from_file(stream, 200, "js/register.js".to_string());
}

pub async fn new_tab(mut stream: TcpStream, request: String, db_pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let req_split = request.split_once("\r\n\r\n");

    match req_split {
        Some((_, body)) =>  {
            let json_body = body.trim_matches(char::from(0));
            let json_result: Result<Tab, _> = serde_json::from_str(json_body);

            match json_result {
                Ok(new_tab) => {
                    let query = format!(
                        "INSERT INTO tab (title, tab) VALUES ('{}', ARRAY[{}])",
                        new_tab.title, new_tab.tab.iter().map(
                            |string| {
                                let string_str= string.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
                                format!("[{}]", string_str)
                            }
                        ).collect::<Vec<String>>().join(",")
                    );
    
                    sqlx::query(&query).execute(&db_pool).await?;
                    send_success(stream);
                }
                Err(e) => {
                    eprintln!("JSON parse error in new tab creation: {}", e);
                    let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nInvalid JSON\r\n\r\n";
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        },
        _ => {
            println!("New Tab request error: {}", request);
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid body\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    };
    Ok(())
}

fn decyfer_tab_from_db(s: String) -> Vec<Vec<u32>> {
    let ints: Vec<u8> = s.chars().map(|i| i as u8).collect();
    let mut sorted = Vec::<u8>::new();

    for i in (35..ints.len()).step_by(8) {
        sorted.push(ints[i]);
    }
    let len = sorted.len() / 6;
    let mut ret = Vec::<Vec<u32>>::new();
    for i in 0..6 {
        let mut row = Vec::<u32>::new();
        for j in 0..len {
            row.push(sorted[(i * len) + j] as u32)
        }
        ret.push(row);
    }
    ret
}

pub async fn list_tabs(mut stream: TcpStream, db_pool: sqlx::PgPool) -> Result<(),  Box<dyn Error>> {
    let q = "SELECT id, title, tab FROM tab";
    let rows = sqlx::query(q).fetch_all(&db_pool).await?;
    let mut tabs = Vec::<String>::new();

    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");

        let tab: String = row.get_unchecked("tab");
        let tab: Vec<Vec<u32>> = decyfer_tab_from_db(tab);

        tabs.push(format!("{{\"id\":{},\"title\":\"{}\",\"tab\":{:?}}}", id, title, tab));
    }

    let contents = format!("{{\"res\":[{}]}}", tabs.join(","));

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}

pub async fn tab_get(mut stream: TcpStream, db_pool: sqlx::PgPool, id: &str) -> Result<(), Box<dyn Error>> {
    let q = format!("SELECT title, tab FROM tab WHERE id = {}", id.to_string());
    println!("{}", q);
    let row = sqlx::query(&q).fetch_one(&db_pool).await?;

    let title: String = row.get("title");

    let tab: String = row.get_unchecked("tab");
    let tab: Vec<Vec<u32>> = decyfer_tab_from_db(tab);

    let contents = format!("{{\"title\":\"{}\",\"tab\":{:?}}}", title, tab);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}

pub async fn login(mut stream: TcpStream, db_pool: sqlx::PgPool, body: &str) -> Result<(), Box<dyn Error>> {
    println!("Given data: {}", body);
    Ok(())
}

pub async fn register(mut stream: TcpStream, db_pool: sqlx::PgPool, body: &str) -> Result<(), Box<dyn Error>> {
    let username = body.to_string().split("username=").collect::<Vec<&str>>()[1].to_string();
    let username = username.split("&").collect::<Vec<&str>>()[0];
    let password = body.to_string().split("password=").collect::<Vec<&str>>()[1].to_string();
    let password = password.split("&").collect::<Vec<&str>>()[0];
    let password = bcrypt::hash(password).unwrap(); // hash password

    // add protection against SQL injections here!!!

    // add something to make sure username is not already taken

    let q = format!("INSERT INTO users (username, password) VALUES ('{}', '{}')", username, password);
    sqlx::query(&q).execute(&db_pool).await?;
    send_success(stream);
    Ok(())
}
