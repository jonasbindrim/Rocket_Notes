use std::sync::{Arc, Mutex};

use api::note::{create, delete, list_all, read};
use manager::note::NoteManager;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod api;
mod data;
mod manager;

#[launch]
fn launch() -> Rocket<Build> {
    rocket::build()
        .manage(Arc::new(Mutex::new(NoteManager::new())))
        .mount("/note", routes![create, read, list_all, delete])
}
