use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn track(event: String) {
    EVENTS.lock().unwrap().push(event);
}

pub fn get_events() -> Vec<String> {
    EVENTS.lock().unwrap().clone()
}
