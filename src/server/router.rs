
use std::error::Error;
use crate::server::routes::*;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::db::db_handler::DbHandler;


pub async fn route(mut stream: TcpStream, db_pool: sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let home_header = b"GET / HTTP/1.1\r\n";
    let new_tab_header = b"GET /new HTTP/1.1\r\n";
    let tab_header = b"GET /list/";
    let styles_header = b"GET /styles.css HTTP/1.1\r\n";
    let new_tab_js_header = b"GET /new_tab.js HTTP/1.1\r\n";
    let list_js_header = b"GET /list.js HTTP/1.1\r\n";
    let tab_js_header = b"GET /tab.js HTTP/1.1\r\n";
    let post_new_tab_header = b"POST /new_tab HTTP/1.1\r\n";
    let tab_list_header = b"GET /tab_list HTTP/1.1\r\n";
    let tab_get_header = b"GET /tab/";

    if buffer.starts_with(home_header) {
        home_page(stream);
    } else if buffer.starts_with(new_tab_header) {
        new_tab_page(stream);
    } else if buffer.starts_with(styles_header) {
        styles_file(stream);
    } else if buffer.starts_with(new_tab_js_header) {
        new_tab_js_file(stream);
    } else if buffer.starts_with(post_new_tab_header) {
        new_tab(stream, std::str::from_utf8(&buffer).unwrap().to_string(), db_pool).await?;
    } else if buffer.starts_with(list_js_header) {
        list_js_file(stream);
    } else if buffer.starts_with(tab_list_header) {
        list_tabs(stream, db_pool).await?;
    } else if buffer.starts_with(tab_header) {
        tab_page(stream);
    } else if buffer.starts_with(tab_js_header) {
        tab_js_file(stream);
    } else if buffer.starts_with(tab_get_header) {
        tab_get(stream, db_pool, buffer).await?;
    } else {
        page_does_not_exist(stream);
    }
    Ok(())
}
