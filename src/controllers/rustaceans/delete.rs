use rocket::response::status;
use super::super::auth::BasicAuth;

#[delete("/rustaceans/<_id>")]
pub fn delete_rustaceans(_id: i32,_auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
