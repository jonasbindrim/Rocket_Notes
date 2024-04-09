use crate::{
    data::note::NoteData,
    database::note::{NotesDb, NotesManager},
};
use rocket::{http::Status, serde::json::Json};

/// Returns the note identified by the notes title
#[get("/read/<title>")]
pub async fn read(database: NotesDb, title: &str) -> Result<Json<NoteData>, Status> {
    match NotesManager::get_note(database, title.to_string()).await {
        Some(note) => Ok(Json(note.clone())),
        None => Err(Status::NotFound),
    }
}

/// Returns all existing notes
#[get("/listAll")]
pub async fn list_all(database: NotesDb) -> Json<Vec<NoteData>> {
    Json(NotesManager::get_all_notes(database).await)
}

/// Creates a new note entry
#[post("/create", format = "json", data = "<note_input>")]
pub async fn create(database: NotesDb, note_input: Json<NoteData>) -> Status {
    match NotesManager::add_note(database, note_input.0).await {
        Ok(()) => Status::Created,
        Err(()) => Status::Conflict,
    }
}

/// Deletes the note with the given title
#[delete("/delete/<title>")]
pub async fn delete(database: NotesDb, title: &str) -> Status {
    match NotesManager::delete_note(database, title.to_string()).await {
        Ok(()) => Status::Ok,
        Err(()) => Status::NotFound,
    }
}
