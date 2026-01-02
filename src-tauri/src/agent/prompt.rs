pub fn build_prompt(context: &str, user_input: &str) -> String {
    format!(
        "You are an offline educational AI assistant.
        Current section: {}
        User input: {}
        Give clear, short, helpful guidance.",
        context,
        user_input
    )
}
