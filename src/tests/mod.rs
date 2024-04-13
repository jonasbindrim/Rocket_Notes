use crate::launch;
use std::fs;

use rocket::local::blocking::Client;

pub mod auth;
pub mod count;
pub mod note;

/// Deletes the current db-file to run the tests against a clean environment
fn prepare_tests() -> Client {
    let _ = fs::remove_file("notes.db");
    Client::tracked(launch()).expect("Unable to create rocket client")
}
