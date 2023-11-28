use std::sync::{Arc, Mutex};

use rocket::{http::Status, serde::json::Json, State};

use crate::{data::note::NoteData, manager::note::NoteManager};

#[get("/read/<title>")]
pub fn read(
    title: &str,
    note_manager: &State<Arc<Mutex<NoteManager>>>,
) -> Result<Json<NoteData>, Status> {
    let state_clone = note_manager.inner().clone();
    let mut manager = state_clone.lock().unwrap();

    let note = manager.get_note(title);
    match note {
        Some(note) => Ok(Json(note.clone())),
        None => Err(Status::NotFound),
    }
}

#[get("/listAll")]
pub fn list_all(note_manager: &State<Arc<Mutex<NoteManager>>>) -> Json<Vec<NoteData>> {
    let state_clone = note_manager.inner().clone();
    let manager = state_clone.lock().unwrap();

    Json(manager.get_all_notes())
}

#[post("/create", format = "json", data = "<note_input>")]
pub fn create(note_input: Json<NoteData>, note_manager: &State<Arc<Mutex<NoteManager>>>) -> Status {
    let state_clone = note_manager.inner().clone();
    let mut manager = state_clone.lock().unwrap();
    let note_id = manager.add_note(&note_input.get_title(), &note_input.get_content());

    match note_id {
        Ok(()) => Status::Ok,
        Err(()) => Status::Conflict,
    }
}

#[delete("/delete/<title>")]
pub fn delete(title: &str, note_manager: &State<Arc<Mutex<NoteManager>>>) -> Status {
    let state_clone = note_manager.inner().clone();
    let mut manager = state_clone.lock().unwrap();

    match manager.delete_note(title) {
        Ok(()) => Status::Ok,
        Err(()) => Status::NotFound,
    }
}
