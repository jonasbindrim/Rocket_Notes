use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct NoteData {
    title: String,
    content: String,
}

impl NoteData {
    pub fn new(title: &str, content: &str) -> Self {
        NoteData {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}
