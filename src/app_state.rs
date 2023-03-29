use std::sync::Mutex;

use crate::models;

pub struct AppState {
    pub counter: Mutex<u32>,
    pub payments: models::PaymentsTable,
    connection: models::Connection,
}

impl AppState {
    pub fn new() -> AppState {
        let conn = models::Connection::new();

        AppState {
            counter: Mutex::new(0),
            payments: models::PaymentsTable::new(conn.get_pool()),
            connection: conn,
        }
    }
}
