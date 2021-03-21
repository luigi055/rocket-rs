use super::super::auth::BasicAuth;
use rocket_contrib::json::JsonValue;

#[put("/rustaceans/<id>", format = "json")]
pub fn update_rustacean(id: i32, _auth: BasicAuth) -> JsonValue {
    json!([
        {
            "id": id,
            "name": "John Doe",
            "email": "johndoe@company.com"
        }
    ])
}
