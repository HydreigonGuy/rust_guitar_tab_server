
mod db;
mod server;

use crate::server::server::Server;

fn main() {
    let server = Server::new();
    println!("Hello, world!");
}
