use std::sync::{Arc, Mutex};

use api::{
    catcher::{not_authorized, not_found},
    count::{get_counter, increment_counter, increment_counter_by},
    note::{create, delete, list_all, read},
};
use database::note::{init_notesdb, NotesDb};
use manager::count::CountManager;
use rocket::{fairing::AdHoc, Build, Rocket};

#[macro_use]
extern crate rocket;

mod api;
mod auth;
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
        .mount(
            "/counter",
            routes![get_counter, increment_counter, increment_counter_by],
        )
        .mount("/note", routes![create, read, list_all, delete])
        .register("/", catchers![not_found, not_authorized])
}
