use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("The requested resource {} is not available", req.uri())
}