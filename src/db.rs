use std::env;
use postgres::{Client, NoTls, Error};

fn var(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}

fn url() -> String {
    let db_user = var("DB_USER", "postgres");
    let db_pass = var("DB_PASS", "");
    format!("postgresql://{}@{}:{}/{}",
        if db_pass.is_empty() { db_user.to_owned() } else { format!("{}:{}", db_user, db_pass) },
        var("DB_HOST", "localhost"),
        var("DB_PORT", "5432"),
        var("DB_NAME", &db_user),
    )
}

pub fn conn() -> Result<Client, Error> {
    let client = Client::connect(&url(), NoTls)?;
    Ok(client)
}
