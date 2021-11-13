#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize};

mod db;

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
    match db::conn() {
        Ok(_) => { println!("ok") },
        Err(e) => { println!("{}", e) },
    }
    rocket::build().mount("/", routes![index])
}
