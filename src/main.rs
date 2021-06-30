mod controllers;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use controllers::{common, rustaceans};

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .mount(
            "/",
            routes![
                rustaceans::get_rustaceans,
                rustaceans::view_rustacean,
                rustaceans::create_rustacean,
                rustaceans::update_rustacean,
                rustaceans::delete_rustaceans
            ],
        )
        .register(catchers![common::send_not_found, common::send_unauthorized])
        .launch()
        .await;
}
