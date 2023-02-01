#[macro_use] extern crate rocket;
use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket.mount("/", routes![index])
}