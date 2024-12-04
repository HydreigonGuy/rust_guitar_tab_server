
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::error::Error;
use crate::db::db_handler::DbHandler;
use crate::server::routes::*;
use crate::server::thread_pool::ThreadPool;

pub struct Server {
    db_handler: DbHandler,
    url: String
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let home_header = b"GET / HTTP/1.1\r\n";
    let new_tab_header = b"GET /new HTTP/1.1\r\n";
    let styles_header = b"GET /styles.css HTTP/1.1\r\n";

    if buffer.starts_with(home_header) {
        home_page(stream);
    } else if buffer.starts_with(new_tab_header) {
        new_tab_page(stream);
    } else if buffer.starts_with(styles_header) {
        styles_file(stream);
    } else {
        page_does_not_exist(stream);
    }
}

impl Server {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let db_handler = DbHandler::new("postgres://user:password@localhost:5432/db").await?;

        Ok(Server { db_handler, url: url.to_string() })
    }

    pub fn run(&self) -> () {
        let listener: TcpListener = TcpListener::bind(self.url.clone()).unwrap();
    
        let thread_pool = ThreadPool::new(10); // 10 here is the number of threads there are

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            
            thread_pool.execute(|| {handle_connection(stream);});
        }
    }
}
