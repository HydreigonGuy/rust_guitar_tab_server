
use std::error::Error;
use crate::db::db_handler::DbHandler;
// use db::DbHandler;

pub struct Server {
    db_handler: DbHandler,
}


impl Server {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let db_handler = DbHandler::new("DB_URL").await?;

        Ok(Server { db_handler })
    }

    fn run() -> () {
    }
}
