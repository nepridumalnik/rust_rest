use std::sync::Mutex;

use crate::models;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub users: models::UsersTable,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: Mutex::new(0),
            users: models::UsersTable::new(),
        }
    }
}
