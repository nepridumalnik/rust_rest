use std::sync::Mutex;

pub struct AppState {
    pub counter: Mutex<u32>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            counter: Mutex::new(0),
        }
    }
}
