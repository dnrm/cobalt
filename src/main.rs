#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Helloooooooooooooo"
}

#[post("/post")]
fn post() -> &'static str {
    "Sample POST route"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![post])
}
