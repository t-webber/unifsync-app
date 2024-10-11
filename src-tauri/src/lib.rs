#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    reason = "enforce restriction linting"
)]
#![allow(clippy::implicit_return, reason = "not necessary")]
#![allow(clippy::single_call_fn, reason = "usefull to segment code")]
#![allow(clippy::missing_docs_in_private_items, reason = "I am lazy")]
#![allow(clippy::question_mark_used, reason = "very usefull")]
#![allow(clippy::mod_module_files, reason = "avoid fs complexity")]
#![allow(clippy::print_stderr, reason = "debugging")]
#![allow(clippy::allow_attributes_without_reason, reason = "tauri")]
#![allow(clippy::allow_attributes, reason = "new feature still bugged")]

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
    format!("Hello, {name}! You've been greeted from Rust!")
}

const NOTES_PATH: &str = "../data/notes.json";

#[tauri::command]
fn get_notes() -> Vec<Note> {
    fs::read_to_string(NOTES_PATH).map_or_else(
        |_| {
            if let Err(err) = fs::write(NOTES_PATH, "") {
                eprintln!("Failed to create file: {err}");
            }
            vec![]
        },
        |content| match serde_json::from_str(&content) {
            Ok(vec) => vec,
            Err(err) => {
                eprintln!("Failed to convert to Vec<Note>: {err}");
                vec![]
            }
        },
    )
}

#[tauri::command]
fn update_note_content(id: u32, content: String) {
    let mut notes: Vec<Note> = get_notes();
    let mut index = None;
    for (i, note) in notes.iter().enumerate() {
        if note.id == id {
            index = Some(i);
        }
    }
    match index.and_then(|i| notes.get_mut(i)) {
        None => eprintln!("Failed to find note with id {id}"),
        Some(note) => {
            note.content = content;
            match serde_json::to_string(&notes)
                .map_err(|e| e.to_string())
                .and_then(|stringified| {
                    fs::write(NOTES_PATH, stringified).map_err(|e| e.to_string())
                }) {
                Ok(_) => (),
                Err(err) => eprintln!("Failed to re-write notes: {err}"),
            }
        }
    }
}

#[allow(clippy::expect_used)]
#[allow(clippy::missing_inline_in_public_items, clippy::exit, reason = "tauri")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// # Panics
/// Function panics when context for running the app can't be loaded
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
