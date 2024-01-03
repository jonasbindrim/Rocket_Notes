use std::fs;

pub mod note;

/// Deletes the current db-file to run the tests against a clean environment
fn prepare_tests() {
    let _ = fs::remove_file("notes.db");
}
