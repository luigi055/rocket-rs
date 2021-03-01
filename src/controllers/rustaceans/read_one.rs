use rocket_contrib::json::JsonValue;

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) -> JsonValue {
    json!([
        {
            "id": id,
            "name": "John Doe",
            "email": "johndoe@company.com"
        }
    ])
}
