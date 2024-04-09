use rocket::http::Status;

use super::prepare_tests;

/// Test case: Initial read of the count value
#[test]
fn get_value() {
    let client = prepare_tests();
    
    let response = client
        .get(uri!("/counter/"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "0");
}

/// Test case: Increment the count value
#[test]
fn increment_value() {
    let client = prepare_tests();
    
    let response = client
        .get(uri!("/counter/increment"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

/// Test case: Read counter after increment
#[test]
fn read_after_increment() {
    let client = prepare_tests();
    
    // Initial read
    let response = client
        .get(uri!("/counter/"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "0");

    // Increment
    let response = client
        .get(uri!("/counter/increment"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    // Read after increment
    let response = client
        .get(uri!("/counter/"))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "1");
}