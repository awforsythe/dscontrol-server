#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize};

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
    rocket::build().mount("/", routes![index])
}
