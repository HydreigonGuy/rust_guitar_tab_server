
mod server;
mod models;

use crate::server::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:8000").await.unwrap();
    
    server.run();
}
