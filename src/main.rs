#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;

#[get("/")]
fn entry() -> Redirect {
    return Redirect::moved("https://github.com/furry");
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
