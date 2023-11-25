use std::sync::{Arc, Mutex};

use api::note_api::{create_note, read_note};
use manager::note_manager::NoteManager;

#[macro_use]
extern crate rocket;

mod api;
mod data;
mod manager;

#[launch]
fn launch() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(NoteManager::new())))
        .mount("/note", routes![create_note, read_note])
}
