use rocket_contrib::json::JsonValue;

// you need to specify the content type when pass format
// to the endpoint
// curl 127.0.0.1:8000/rustaceans -X POST -H 'Content-type: application/json'
#[post("/rustaceans", format = "json")]
pub fn create_rustacean() -> JsonValue {
    json!([
        {
            "id": 3,
            "name": "John Doe",
            "email": "johndoe@company.com"
        }
    ])
}
