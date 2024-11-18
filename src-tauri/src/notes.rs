use std::{fs, path::Path};

use crate::errors::Eprintln;
#[cfg(feature = "logs")]
use logs_lib::LOGS_PATH;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    content: String,
}

impl Note {
    fn new(id: u32) -> Self {
        Self {
            id,
            title: format!("Untitled - {id}"),
            content: String::new(),
        }
    }
}

const DATA_DIR: &str = "../data/";
const NOTES_PATH: &str = "../data/notes.json";

#[tauri::command]
pub fn get_notes() -> Vec<Note> {
    fs::read_to_string(NOTES_PATH).map_or_else(
        |_| {
            fs::write(NOTES_PATH, "[]").eprint("Failed to create file");
            vec![]
        },
        |content| {
            serde_json::from_str(&content).eprint_or("Failed to convert data to Vec<Note>", vec![])
        },
    )
}

fn write_notes(notes: &Vec<Note>) {
    serde_json::to_string(&notes)
        .map_err(|er| er.to_string())
        .and_then(|stringified| fs::write(NOTES_PATH, stringified).map_err(|er| er.to_string()))
        .eprint("Failed to re-write notes");
}

#[tauri::command]
pub fn update_note(id: u32, title: String, content: String) {
    let mut notes: Vec<Note> = get_notes();
    let mut note = None;
    for n in &mut notes {
        if n.id == id {
            note = Some(n);
            break;
        }
    }
    note.eprint(&format!("Failted to find note with id {id}"));
    if let Some(n) = note {
        n.title = title;
        n.content = content;
        write_notes(&notes);
    };
}

#[tauri::command]
pub fn create_note() -> u32 {
    let mut notes = get_notes();
    let mut ids = notes.iter().map(|note| note.id);
    let mut new_id = 0;
    while ids.any(|id| id == new_id) {
        new_id += 1;
    }
    notes.push(Note::new(new_id));
    write_notes(&notes);
    new_id
}

#[tauri::command]
pub fn delete_note(id: u32) {
    write_notes(
        &get_notes()
            .into_iter()
            .filter(|note| note.id != id)
            .collect(),
    );
}

#[allow(clippy::create_dir)]
pub fn init_notes() {
    if !Path::new(DATA_DIR).exists() {
        fs::create_dir(DATA_DIR).eprint("Failed to create data folder");
    }
    if !Path::new(NOTES_PATH).exists() {
        fs::write(NOTES_PATH, "").eprint("Failed to create data file");
    }
    #[cfg(feature = "logs")]
    if !Path::new(LOGS_PATH).exists() {
        fs::write(LOGS_PATH, "").eprint("Failed to create logs file");
    }
}
