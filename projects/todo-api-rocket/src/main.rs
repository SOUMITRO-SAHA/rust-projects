#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/welcome")]
fn welcome() -> &'static str {
    return "Welcome to Rocket!";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, welcome])
}
