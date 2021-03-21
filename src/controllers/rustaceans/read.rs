use super::super::auth::BasicAuth;

use rocket_contrib::json::JsonValue;

#[get("/rustaceans")]
pub fn get_rustaceans(_auth: BasicAuth) -> JsonValue {
    json!([
        {
            "id":1,
            "name": "John Doe"
        },
        {
            "id": 2,
            "name": "John Doe"
        },
        {
            "id": 3,
            "name": "John Doe"
        },
    ])
}
