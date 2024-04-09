use std::sync::{Arc, Mutex};

use rocket::{serde::json::Json, State};

use crate::manager::count::CountManager;

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