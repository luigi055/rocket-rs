use rocket::response::status;

#[delete("/rustaceans/<_id>")]
pub fn delete_rustaceans(_id: i32) -> status::NoContent {
    status::NoContent
}
