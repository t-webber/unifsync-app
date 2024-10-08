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
        Ok(content) => serde_json::from_str(&content).expect("Failed to convert to Vec<Note>"),
        Err(_) => {
            fs::write(NOTES_PATH, "").expect("Failed to create file");
            vec![]
        }
    }
}

#[tauri::command]
fn update_note_content(id: usize, content: String) {
    let mut notes: Vec<Note> = get_notes();
    match notes.get_mut(id) {
        Some(note) => note.content = content,
        None => panic!("Not found"),
    }
    fs::write(
        NOTES_PATH,
        serde_json::to_string(&notes).expect("Serializing failed"),
    )
    .expect("Failed to write to file");
    println!("Successfully saved status")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_notes,
            update_note_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
