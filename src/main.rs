use api::note::{create, delete, list_all, read};
use database::note::{init_notesdb, NotesDb};
use rocket::{fairing::AdHoc, Build, Rocket};

#[macro_use]
extern crate rocket;

mod api;
mod data;
mod database;

#[cfg(test)]
mod tests;

#[launch]
fn launch() -> Rocket<Build> {
    rocket::build()
        .attach(NotesDb::fairing())
        .attach(AdHoc::on_ignite("Init NotesDb", init_notesdb))
        .mount("/note", routes![create, read, list_all, delete])
}
