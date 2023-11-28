use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
