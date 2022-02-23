#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::JsonValue;
use rocket_study::lib::CORS;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod tests;

#[get("/")]
fn index() -> JsonValue {
    json!({ "hi": "hello-word" })
}

fn main() {
    rocket::ignite()
        .attach(CORS)
        .mount("/", routes![index])
        .launch();
}
