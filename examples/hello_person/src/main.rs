#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

#[rocket::launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello, hi])
}
