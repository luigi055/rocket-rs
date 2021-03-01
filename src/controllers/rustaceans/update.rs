use rocket_contrib::json::JsonValue;

#[put("/rustaceans/<id>", format = "json")]
pub fn update_rustacean(id: i32) -> JsonValue {
    json!([
        {
            "id": id,
            "name": "John Doe",
            "email": "johndoe@company.com"
        }
    ])
}
