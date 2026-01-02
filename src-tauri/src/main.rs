#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agent;

#[tauri::command]
fn ask_agent(app: tauri::AppHandle, input: String) {
    agent::core::run_agent(app, input);
}

#[tauri::command] fn track_event(event: String) { agent::tracker::track(event); } 
#[tauri::command] fn get_feedback() -> String { let events = agent::tracker::get_events(); agent::feedback::analyze(events) }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ask_agent,track_event,get_feedback
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
