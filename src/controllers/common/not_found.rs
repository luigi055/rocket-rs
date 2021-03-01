use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn send_not_found() -> JsonValue {
    json!("Not Found!")
}
