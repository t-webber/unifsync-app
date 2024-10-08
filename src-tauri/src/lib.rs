use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Note {
    id: u32,
    title: String,
    content: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    let res = format!("Hello, {name}! You've been greeted from Rust!");
    println!("{res}");
    res
}

const NOTES_PATH: &str = "../data/notes.json";

#[tauri::command]
fn get_notes() -> Vec<Note> {
    match fs::read_to_string(NOTES_PATH).map_err(|err| err.to_string()) {
        Ok(content) => serde_json::from_str(&content)
            .unwrap_or_else(|_| panic!("Failed to convert to Vec<Note>")),
        Err(_) => {
            fs::write(NOTES_PATH, "").unwrap_or_else(|_| panic!("Failed to create file"));
            vec![]
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
