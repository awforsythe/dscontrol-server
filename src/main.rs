#[macro_use] extern crate rocket;

use std::env;
use rocket::serde::json::Json;
use serde::{Serialize};

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

#[derive(Serialize)]
struct TestStruct {
    name: String,
    value: i32,
}

#[get("/")]
fn index() -> Json<TestStruct> {
    let data = TestStruct {
        name: "test data".to_owned(),
        value: 42,
    };
    Json(data)
}

#[launch]
fn rocket() -> _ {
    println!("{}", url());
    rocket::build().mount("/", routes![index])
}
