use std::process::Command;
use tauri::{AppHandle, Manager, Emitter};
use tauri::path::BaseDirectory;

pub fn run_agent(app: AppHandle, prompt: String) {
    tauri::async_runtime::spawn_blocking(move || {

        let exe_path = match app.path().resolve(
            "bin/llama-cli.exe",
            BaseDirectory::Resource,
        ) {
            Ok(p) => p,
            Err(_) => {
                let _ = app.emit("agent_result", "❌ llama-cli not found");
                return;
            }
        };

        let model_path = match app.path().resolve(
            "agent/models/tinyllama-1.1b-chat.Q4_K_M.gguf",
            BaseDirectory::Resource,
        ) {
            Ok(p) => p,
            Err(_) => {
                let _ = app.emit("agent_result", "❌ model not found");
                return;
            }
        };

        let output = Command::new(exe_path)
            .args([
                "-m", model_path.to_str().unwrap(),

                "--prompt",
                &format!(
                    "You are an offline educational AI assistant.\n\
                     Answer clearly and simply.\n\
                     Question: {}",
                    prompt
                ),

                "--n-predict", "128",
                "--ctx-size", "1024",
                "--temp", "0.7",
                "--top-p", "0.9",

                // ✅ SAFE FLAGS (SUPPORTED)
                "--no-display-prompt"
            ])
            .output();

        let response = match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                let combined = if stdout.trim().is_empty() {
                    stderr
                } else {
                    stdout
                };

                combined
                    .lines()
                    .filter(|l|
                        !l.starts_with("build") &&
                        !l.starts_with("model") &&
                        !l.starts_with("modalities") &&
                        !l.contains("t/s") &&
                        !l.starts_with(">") &&
                        !l.contains("available commands") &&
                        !l.contains("Loading model")
                    )
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string()
            }
            Err(e) => format!("❌ AI engine failed: {}", e),
        };

        let final_text = if response.is_empty() {
            "⚠️ Model returned empty output".to_string()
        } else {
            response
        };

        let _ = app.emit("agent_result", final_text);
    });
}
