
use sqlx::Row;
use std::error::Error;
use pwhash::bcrypt;
use rand::{distributions::Alphanumeric, Rng};

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

pub async fn check_login_auth(db_pool: sqlx::PgPool, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let q = format!("SELECT id, password FROM users WHERE username = '{}' and password = '{}'", username.to_string(), password.to_string());
    println!("{}", q);
    let rows = sqlx::query(&q).fetch_all(&db_pool).await?;
    let mut c = 0;

    for row in rows {
        return Ok(true);
    }
    return Ok(false);
}

pub async fn create_token_for_user(db_pool: sqlx::PgPool, user_id: u32) -> Result<String, Box<dyn Error>> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    Ok(s)
}
