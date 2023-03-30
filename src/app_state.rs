use std::sync::Mutex;

use crate::models;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub users: models::UsersTable,
    connection: models::Connection,
}

impl AppState {
    pub fn new() -> AppState {
        let conn = models::Connection::new();

        AppState {
            counter: Mutex::new(0),
            users: models::UsersTable::new(conn.get_pool()),
            connection: conn,
        }
    }
}
