mod cache;
mod database;

use std::sync::Mutex;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub db: database::Database,
    pub ch: cache::Cache,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: Mutex::new(0),
            db: database::Database::default(),
            ch: cache::Cache::default(),
        }
    }
}
