
use std::error::Error;

use crate::models::tab::Tab;

pub struct DbHandler {
    pool: sqlx::PgPool // originally a & so check if I need to change that
}


impl DbHandler {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(DbHandler { pool })
    }

    pub async fn create(&self, tab: Tab) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO tab (title, tab) VALUES ($1, $2)";
    
        println!("{}",
            tab.tab.iter().map(
                |string| {
                    let string_str= string.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
                    format!("[{}]", string_str)
                }
            ).collect::<Vec<String>>().join(",")
        );
        //sqlx::query(query).bind(tab.title).bind(tab.tab).execute(self.pool).await?;
        Ok(())
    }
    
//    fn getTabs() -> Result<Tab, Box<dyn Error>> {
//        let q = "SELECT title, tab FROM tab";
//        let query = sqlx::query(q);
//    
//        // now only gets one, we need to update it to get all
//        let row = query.fetch_one(conn).await?;
//        let tab = Tab{
//            title: row.get("title"),
//            tab: row.get("tab")
//        };
//        Ok(tab)
//    }
}
