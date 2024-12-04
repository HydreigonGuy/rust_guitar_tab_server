
use std::error::Error;
use sqlx::Row;

struct Tab {
    pub title: String,
    pub tab: String
}

pub struct DbHandler {
    pool: sqlx::PgPool // originally a & so check if I need to change that
}


impl DbHandler {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = sqlx::postgres::PgPool::connect(url).await?;

        // Run migrations (commented while migrations don't exist)
        //sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(DbHandler { pool })
    }

//    fn create(&self, tab: &Tab) -> Result<(), Box<dyn Error>> {
//        let query = "INSERT INTO tab (title, tab) VALUES ($1, $2)";
//    
//        sqlx::query(query).bind(&tab.title).bind(&tab.tab).execute(self.pool).await?;
//        Ok(())
//    }
    
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
