pub fn analyze(events: Vec<String>) -> String {
    if events.iter().any(|e| e.contains("quiz_failed")) {
        "You seem to be struggling. Try revising the basics before moving ahead."
            .to_string()
    } else {
        "You're doing well. Keep learning consistently!"
            .to_string()
    }
}
