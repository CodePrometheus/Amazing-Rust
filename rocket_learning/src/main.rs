#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn say_hello(name: &str) -> String {
    format!("hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, say_hello])
}