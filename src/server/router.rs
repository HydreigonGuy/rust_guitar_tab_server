
use std::error::Error;
use crate::server::routes::*;
use std::net::TcpStream;
use std::io::prelude::*;


pub async fn route(mut stream: TcpStream, db_pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let mut req = Vec::new();
    let mut request: String;

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                req.extend_from_slice(&buffer[..n]);
                request = String::from_utf8(req.clone()).unwrap();
                if req.ends_with(b"\r\n\r\n") {
                    break;
                }
                if request.contains("\r\n\r\n") {
                    // this is for POST  requests to retrieve the whole body
                    let contents = request.split("\r\n\r\n").collect::<Vec<&str>>()[1];
                    let tmp = request.split("Content-Length: ").collect::<Vec<&str>>()[1].to_string();
                    let len = tmp.split("\r\n").collect::<Vec<&str>>()[0];
                    if len.to_string().parse::<usize>().unwrap() <= contents.len() {
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                return Ok(());
            }
        }
    }

    let request = String::from_utf8(req).unwrap();

    println!("{}", request);

    let method = request.split(" ").collect::<Vec<&str>>()[0].to_string();
    let path = request.split(" ").collect::<Vec<&str>>()[1].to_string();

    if method == "POST" {
        match path.split("/").collect::<Vec<&str>>()[1] {
            "new_tab" => new_tab(stream, request, db_pool).await?,
            "login" => login(stream, db_pool, request.split("\r\n\r\n").collect::<Vec<&str>>()[1]).await?,
            "register" => register(stream, db_pool, request.split("\r\n\r\n").collect::<Vec<&str>>()[1]).await?,
            _ => page_does_not_exist(stream),
        }
    } else if method == "GET" {
        match path.split("/").collect::<Vec<&str>>()[1] {
            "" => home_page(stream),
            "new" => new_tab_page(stream),
            "tab_list" => list_tabs(stream, db_pool).await?,
            "list" => tab_page(stream),
            "tab" => {
                let id = path.split("/").collect::<Vec<&str>>()[2];
                tab_get(stream, db_pool, id).await?;
            }
            "login" => login_page(stream),
            "register" => register_page(stream),
            "styles.css" => styles_file(stream),
            "new_tab.js" => new_tab_js_file(stream),
            "list.js" => list_js_file(stream),
            "tab.js" => tab_js_file(stream),
            "register.js" => register_js_file(stream),
            "login.js" => login_js_file(stream),
            _ => page_does_not_exist(stream),
        }
    }

    Ok(())
}
