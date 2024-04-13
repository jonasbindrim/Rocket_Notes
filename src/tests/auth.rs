use http_auth_basic::Credentials;
use rocket::http::{Header, Status};

use crate::tests::prepare_tests;

/// Test case: Authenticate successfully
#[test]
fn successfull_auth() {
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

/// Test case: Authenticate successfully
#[test]
fn failed_auth() {
    let client = prepare_tests();

    let auth_credentials = Credentials {
        user_id: String::from("username"),
        password: String::from("usernane"),
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

    assert_eq!(response.status(), Status::Unauthorized);
}
