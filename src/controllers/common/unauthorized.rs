use rocket_contrib::json::JsonValue;

#[catch(401)]
pub fn send_unauthorized() -> JsonValue {
    json!("Invalid/Missing authorization")
}

