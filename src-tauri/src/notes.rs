use std::fs;

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

const NOTES_DIR: &str = "../data/";
const NOTES_PATH: &str = "../data/notes.json";

#[tauri::command]
pub fn get_notes() -> Vec<Note> {
    fs::read_to_string(NOTES_PATH).map_or_else(
        |_| {
            if let Err(err) =
                fs::create_dir_all(NOTES_DIR).and_then(|()| fs::write(NOTES_PATH, "[]"))
            {
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

fn write_notes(notes: &Vec<Note>) {
    match serde_json::to_string(&notes)
        .map_err(|er| er.to_string())
        .and_then(|stringified| fs::write(NOTES_PATH, stringified).map_err(|er| er.to_string()))
    {
        Ok(()) => (),
        Err(err) => eprintln!("Failed to re-write notes: {err}"),
    }
}

#[tauri::command]
pub fn update_note(id: u32, title: String, content: String) {
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
            note.title = title;
            note.content = content;
            write_notes(&notes);
        }
    }
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
