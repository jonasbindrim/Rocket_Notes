use std::sync::{Arc, Mutex};

use rocket::{
    http::Status,
    response::{content::RawJson, status},
    serde::{json::Json, Deserialize},
    State,
};

use crate::{data::note_data::NoteData, manager::note_manager::NoteManager};

#[get("/read/<id>")]
pub fn read_note(
    id: usize,
    note_manager: &State<Arc<Mutex<NoteManager>>>,
) -> Result<Json<NoteData>, Status> {
    let state_clone = note_manager.inner().clone();
    let mut manager = state_clone.lock().unwrap();

    let note = manager.get_note(id);
    match note {
        Some(note) => Ok(Json(note.clone())),
        None => Err(Status::NotFound),
    }
}

// #[get("/listAll")]
// fn list_note() {}

#[post("/create", format = "json", data = "<note_input>")]
pub fn create_note(
    note_input: Json<NoteInputData<'_>>,
    note_manager: &State<Arc<Mutex<NoteManager>>>,
) -> status::Custom<RawJson<String>> {
    let state_clone = note_manager.inner().clone();
    let mut manager = state_clone.lock().unwrap();
    let note_id = manager.add_note(note_input.title, note_input.content);

    let response = format!("{{ \"id\": \"{}\"}}", note_id);
    status::Custom(Status::Ok, RawJson(response))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NoteInputData<'r> {
    title: &'r str,
    content: &'r str,
}
