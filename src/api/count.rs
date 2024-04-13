use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::{auth::admin::AdminUser, manager::count::CountManager};

/// Returns the current counter value
#[get("/")]
pub fn get_counter(count_manager: &State<Arc<Mutex<CountManager>>>) -> Json<u32> {
    let counter = count_manager.inner().clone();
    let counter = counter.lock().unwrap();

    Json(counter.value())
}

/// Raises the current counter value by 1
#[get("/increment")]
pub fn increment_counter(count_manager: &State<Arc<Mutex<CountManager>>>) {
    let counter = count_manager.inner().clone();
    let mut counter = counter.lock().unwrap();

    counter.increment();
}

/// Raises the current counter value by the given number
#[get("/increment/<number>")]
pub fn increment_counter_by(
    _admin: AdminUser,
    count_manager: &State<Arc<Mutex<CountManager>>>,
    number: u32,
) {
    let counter = count_manager.inner().clone();
    let mut counter = counter.lock().unwrap();

    counter.increment_by(number);
}
