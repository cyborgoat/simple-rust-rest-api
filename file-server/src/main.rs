#[macro_use]
extern crate rocket;
use rocket::tokio::time::{sleep, Duration};
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/file/<filename>")]
async fn file(filename: &str) -> String {
    format!("You asked for {}", filename)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![delay])
        .mount("/api", routes![file])
}
