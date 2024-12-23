
use std::error::Error;

pub async fn check_if_username_is_taken(db_pool: sqlx::PgPool, username: &str) -> Result<bool, Box<dyn Error>> {
    let q = format!("SELECT id FROM users WHERE username = '{}'", username.to_string());
    println!("{}", q);
    let rows = sqlx::query(&q).fetch_all(&db_pool).await?;
    let mut c = 0;

    for row in rows {
        c += 1;
    }
    if c == 0 {
        return Ok(false);
    }
    return Ok(true);
}
