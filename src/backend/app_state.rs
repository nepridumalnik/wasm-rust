mod connection;

use std::sync::Mutex;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub db: connection::Database,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: Mutex::new(0),
            db: connection::Database::default(),
        }
    }
}
