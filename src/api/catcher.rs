use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("The requested resource {} is not available", req.uri())
}

#[catch(401)]
pub fn not_authorized(req: &Request) -> String {
    format!(
        "Not authorized to access the requested ressource {}",
        req.uri()
    )
}
