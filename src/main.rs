use std::sync::{Arc, Mutex};

use api::{count::{get_counter, increment_counter}, note::{create, delete, list_all, read}};
use database::note::{init_notesdb, NotesDb};
use manager::count::CountManager;
use rocket::{fairing::AdHoc, Build, Rocket};

#[macro_use]
extern crate rocket;

mod api;
mod data;
mod database;
mod manager;

#[cfg(test)]
mod tests;

#[launch]
fn launch() -> Rocket<Build> {
    rocket::build()
        .attach(NotesDb::fairing())
        .attach(AdHoc::on_ignite("Init NotesDb", init_notesdb))
        .manage(Arc::new(Mutex::new(CountManager::new())))
        .mount("/counter", routes![get_counter, increment_counter])
        .mount("/note", routes![create, read, list_all, delete])
}
