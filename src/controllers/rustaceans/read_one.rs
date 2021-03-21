use rocket_contrib::json::JsonValue;
use super::super::auth::BasicAuth;

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32,_auth: BasicAuth) -> JsonValue {
    json!([
        {
            "id": id,
            "name": "John Doe",
            "email": "johndoe@company.com"
        }
    ])
}
