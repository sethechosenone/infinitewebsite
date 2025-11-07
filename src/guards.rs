use rocket::request::{self, FromRequest, Request};

pub struct UserAgent(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("user-agent") {
            Some(ua) => request::Outcome::Success(UserAgent(ua.to_string())),
            None => request::Outcome::Success(UserAgent(String::new())),
        }
    }
}

impl UserAgent {
    pub fn is_health_check(&self) -> bool {
        let ua_lower = self.0.to_lowercase();
        ua_lower.contains("amazon") 
        || ua_lower.contains("lightsail")
        || ua_lower.contains("health")
        || ua_lower.contains("check")
        || ua_lower.contains("monitor")
    }
}