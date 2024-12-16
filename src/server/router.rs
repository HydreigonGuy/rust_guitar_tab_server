
use std::error::Error;
use crate::server::routes::*;
use std::net::TcpStream;
use std::io::prelude::*;


pub async fn route(mut stream: TcpStream, db_pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();

    let method = String::from_utf8_lossy(&buffer[..]).split(" ").collect::<Vec<&str>>()[0].to_string();
    let path = String::from_utf8_lossy(&buffer[..]).split(" ").collect::<Vec<&str>>()[1].to_string();

    if method == "POST" {
        match path.as_str() {
            "new_tab" => new_tab(stream, std::str::from_utf8(&buffer).unwrap().to_string(), db_pool).await?,
            _ => page_does_not_exist(stream),
        }
    } else if method == "GET" {
        match path.split("/").collect::<Vec<&str>>()[1] {
            "" => home_page(stream),
            "new" => new_tab_page(stream),
            "tab_list" => list_tabs(stream, db_pool).await?,
            "list" => tab_page(stream),
            "tab" => tab_get(stream, db_pool, buffer).await?,
            "styles.css" => styles_file(stream),
            "new_tab.js" => new_tab_js_file(stream),
            "list.js" => list_js_file(stream),
            "tab.js" => tab_js_file(stream),
            _ => page_does_not_exist(stream),
        }
    }

    Ok(())
}
