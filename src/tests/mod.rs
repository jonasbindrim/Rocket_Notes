use std::fs;
use crate::launch;

use rocket::local::blocking::Client;

pub mod note;
pub mod count;

/// Deletes the current db-file to run the tests against a clean environment
fn prepare_tests() -> Client {
    let _ = fs::remove_file("notes.db");
    Client::tracked(launch()).expect("Unable to create rocket client")
}
