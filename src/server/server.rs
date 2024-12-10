
use std::net::TcpListener;
use std::error::Error;
use crate::server::thread_pool::ThreadPool;
use crate::server::router::route;
use std::sync::Arc;
use tokio::runtime::Runtime;


pub struct Server {
    db_pool: sqlx::PgPool,
    url: String
}

impl Server {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let db_pool = sqlx::postgres::PgPool::connect("postgres://user:password@localhost:5432/db").await?;

        // Run db migrations
        sqlx::migrate!("./migrations").run(&db_pool).await?;

        Ok(Server { db_pool, url: url.to_string() })
    }

    pub fn run(&self) -> () {
        let listener: TcpListener = TcpListener::bind(self.url.clone()).unwrap();
    
        let thread_pool = ThreadPool::new(10); // 10 here is the number of threads there are
        let runtime = Arc::new(Runtime::new().unwrap());

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let db_p =  self.db_pool.clone();
            
            thread_pool.execute({
                let runtime = Arc::clone(&runtime);
                move || {
                    runtime.block_on(async {
                        route(stream, db_p).await;
                    });
                }
            });
        }
    }
}
