use http_auth_basic::{self, Credentials};
use rocket::http::{Header, Status};

use super::prepare_tests;

/// Test case: Initial read of the count value
#[test]
fn get_value() {
    let client = prepare_tests();

    let response = client.get(uri!("/counter/")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "0");
}

/// Test case: Increment the count value
#[test]
fn increment_value() {
    let client = prepare_tests();

    let response = client.get(uri!("/counter/increment")).dispatch();

    assert_eq!(response.status(), Status::Ok);
}

/// Test case: Read counter after increment
#[test]
fn read_after_increment() {
    let client = prepare_tests();

    // Initial read
    let response = client.get(uri!("/counter/")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "0");

    // Increment
    let response = client.get(uri!("/counter/increment")).dispatch();

    assert_eq!(response.status(), Status::Ok);

    // Read after increment
    let response = client.get(uri!("/counter/")).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "1");
}

/// Test case: Increment counter by 5
#[test]
fn increment_by_value() {
    let client = prepare_tests();

    let auth_credentials = Credentials {
        user_id: String::from("username"),
        password: String::from("password"),
    };
    let auth_header = Header::new(
        "Authorization",
        format!("Basic {}", auth_credentials.encode()),
    );

    // Initial read
    let response = client
        .get(uri!("/counter/increment/5"))
        .header(auth_header)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

/// Test case: Read counter after increment by 5
#[test]
fn read_after_increment_by_value() {
    let client = prepare_tests();

    let auth_credentials = Credentials {
        user_id: String::from("username"),
        password: String::from("password"),
    };
    let auth_header = Header::new(
        "Authorization",
        format!("Basic {}", auth_credentials.encode()),
    );

    // Initial read
    let response = client
        .get(uri!("/counter/"))
        .header(auth_header.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "0");

    // Increment
    let response = client
        .get(uri!("/counter/increment/5"))
        .header(auth_header.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    // Read after increment
    let response = client
        .get(uri!("/counter/"))
        .header(auth_header.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "5");
}
