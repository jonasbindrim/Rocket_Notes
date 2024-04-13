use http_auth_basic::Credentials;
use rocket::{
    http::{hyper::header::AUTHORIZATION, Status},
    request::{FromRequest, Outcome, Request},
};

pub struct AdminUser();

#[derive(Debug)]
pub enum AdminAuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AdminAuthError;

    /// Checks whether the given request has the correct authentication information
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header_map = request.headers();
        let Some(auth_header) = header_map.get_one(AUTHORIZATION.as_str()) else {
            return Outcome::Error((Status::Unauthorized, AdminAuthError::Missing));
        };

        let Ok(credentials) = Credentials::from_header(auth_header.to_string()) else {
            return Outcome::Error((Status::Unauthorized, AdminAuthError::Missing));
        };

        if credentials.user_id == "username" && credentials.password == "password" {
            Outcome::Success(AdminUser())
        } else {
            Outcome::Error((Status::Unauthorized, AdminAuthError::Invalid))
        }
    }
}
