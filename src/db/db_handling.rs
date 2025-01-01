
use sqlx::Row;
use std::error::Error;
use rand::{distributions::Alphanumeric, Rng};
use bcrypt::verify;

pub async fn check_if_username_is_taken(db_pool: sqlx::PgPool, username: &str) -> Result<bool, Box<dyn Error>> {
    let q = format!("SELECT id FROM users WHERE username = '{}'", username.to_string());
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
    let q = format!("SELECT id, password FROM users WHERE username = '{}'", username.to_string());
    let rows = sqlx::query(&q).fetch_all(&db_pool).await?;
    let mut c = 0;

    for row in rows {
        if verify(password, row.get("password")).expect("Error verifying password") {
            return Ok(true);
        }
    }
    return Ok(false);
}

pub async fn create_token_for_user(db_pool: sqlx::PgPool, user_id: i32) -> Result<String, Box<dyn Error>> {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let query = format!("UPDATE users SET token = '{}' WHERE id = {}", token, user_id);
    sqlx::query(&query).execute(&db_pool).await?;
    Ok(token)
}

pub async fn get_user_id(db_pool: sqlx::PgPool, username: &str) -> Result<i32, Box<dyn Error>> {
    let q = format!("SELECT id FROM users WHERE username = '{}'", username.to_string());
    let row = sqlx::query(&q).fetch_one(&db_pool).await?;

    Ok(row.get("id"))
}

pub async fn get_user_id_from_token(db_pool: sqlx::PgPool, token: &str) -> Result<i32, Box<dyn Error>> {
    let q = format!("SELECT id FROM users WHERE token = '{}'", token.to_string());
    let row = sqlx::query(&q).fetch_one(&db_pool).await?;

    Ok(row.get("id"))
}
