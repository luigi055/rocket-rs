use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

// example basic token got from wikipedia
// Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==
// https://en.wikipedia.org/wiki/Basic_access_authentication
// example
// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
  
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    // Basic 8113h9dueqfnefno
    fn from_base64_encoded(bases64_string: &str) -> Option<BasicAuth> {
    let decoded = base64::decode(bases64_string).ok()?;
    let decoded_str = String::from_utf8(decoded).ok()?;
    let split = decoded_str.split(":").collect::<Vec<_>>();
    println!("CALLED!");

    // validate username+password pair is present
    if split.len() != 2 {
        return None;
    }

    let (username, password) = (split[0].to_string(), split[1].to_string());

    if username != "foo" {
        return None;
    }

    if password != "bar"  {
        return None;
    }

    Some(BasicAuth {
        username,
        password
    })
}
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
