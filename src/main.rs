#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn entry() -> &'static str {
    "Welcome"
}

fn main() {
    
    rocket::ignite()

    .mount("/", StaticFiles::from("./images"))
    // Main Routes
    .mount("/", routes![entry])

    // All Delete/Post requests for images.
    .mount("/api/v1/images", routes![routes::images::upload_image])

    // Start the server
    .launch();

}
