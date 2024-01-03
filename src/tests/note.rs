use crate::data::note::NoteData;
use crate::launch;
use crate::tests::prepare_tests;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

/// Test case: Create a single note by using the create-endpoint
#[test]
fn create_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    let request_body: &str = "{ \"title\": \"Test\", \"content\": \"Test\" }";
    let response = client
        .post(uri!("/note/create"))
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch();

    assert_eq!(response.status(), Status::Created);
}

/// Test case: Create a note with the same title twice
#[test]
fn create_duplicate_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    // Create valid note
    let request_body: &str = "{ \"title\": \"Test\", \"content\": \"Test\" }";
    let first_response = client
        .post(uri!("/note/create"))
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch();

    assert_eq!(first_response.status(), Status::Created);

    // Create invalid note
    let second_response = client
        .post(uri!("/note/create"))
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch();

    assert_eq!(second_response.status(), Status::Conflict);
}

/// Test case: Create a lot of notes
#[test]
fn create_multiple_notes() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    for note_counter in 0..100 {
        let request_body =
            format!("{{ \"title\": \"Test_{note_counter}\", \"content\": \"Test\"}}");
        let response = client
            .post(uri!("/note/create"))
            .header(ContentType::JSON)
            .body(request_body)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
    }
}

// Test case: Delete a non existing note
#[test]
fn delete_not_existing_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");
    let response = client.post(uri!("/note/create/Test")).dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

/// Test case: Delete an existing note
#[test]
fn delete_existing_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    // Create note to delete
    let request_body: &str = "{ \"title\": \"Test\", \"content\": \"Test\" }";
    let create_response = client
        .post(uri!("/note/create"))
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch();

    assert_eq!(create_response.status(), Status::Created);

    // Delete existing note
    let delete_response = client.post(uri!("/note/create/Test")).dispatch();
    assert_eq!(delete_response.status(), Status::NotFound);
}

/// Test case: Read a non existing note
#[test]
fn read_not_existing_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    let response = client.get(uri!("/note/read/Test")).dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

/// Test case: Read an existing note
#[test]
fn read_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    // Create note
    let request_body: &str = "{ \"title\": \"Test\", \"content\": \"Test\" }";
    let create_response = client
        .post(uri!("/note/create"))
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch();

    assert_eq!(create_response.status(), Status::Created);

    // Read note
    let read_response = client.get(uri!("/note/read/Test")).dispatch();

    assert_eq!(read_response.status(), Status::Ok);

    let response_content: NoteData = read_response.into_json().unwrap();
    assert_eq!(response_content.get_title(), "Test");
    assert_eq!(response_content.get_content(), "Test");
}

/// Test case: Read correct existing note
#[test]
fn read_correct_note() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    for note_counter in 0..2 {
        let request_body =
            format!("{{ \"title\": \"Test_{note_counter}\", \"content\": \"Test\"}}");
        let response = client
            .post(uri!("/note/create"))
            .header(ContentType::JSON)
            .body(request_body)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
    }

    // Read note
    let read_response = client.get(uri!("/note/read/Test_1")).dispatch();

    assert_eq!(read_response.status(), Status::Ok);

    let response_content: NoteData = read_response.into_json().unwrap();
    assert_eq!(response_content.get_title(), "Test_1");
    assert_eq!(response_content.get_content(), "Test");
}

/// Test case: List all notes when no node exists
#[test]
fn listall_notes_empty() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    let response = client.get(uri!("/note/listAll")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    let response_content: Vec<NoteData> = response.into_json().unwrap();
    assert_eq!(response_content.len(), 0);
}

/// Test case: List all notes when nodes exists
#[test]
fn listall_notes() {
    prepare_tests();
    let client = Client::tracked(launch()).expect("Unable to create rocket client");

    // Create nodes
    for note_counter in 0..100 {
        let request_body =
            format!("{{ \"title\": \"Test_{note_counter}\", \"content\": \"Test\"}}");
        let response = client
            .post(uri!("/note/create"))
            .header(ContentType::JSON)
            .body(request_body)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
    }

    // Get all notes and check the amount
    let response = client.get(uri!("/note/listAll")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    let response_content: Vec<NoteData> = response.into_json().unwrap();
    assert_eq!(response_content.len(), 100);
}
