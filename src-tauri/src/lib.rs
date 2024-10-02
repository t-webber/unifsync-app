use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Note<T> {
    id: u32,
    title: T,
    content: T,
}

const NOTES: [Note<&str>; 4] = [
    Note {
        id: 1,
        title: "A first blog",
        content: "Lorem ipsum dolor sit amet",
    },
    Note {
        id: 2,
        title: "A second blog",
        content: "Lorem ipsum dolor sit amet",
    },
    Note {
        id: 3,
        title: "A third blog",
        content: "Lorem ipsum dolor sit amet",
    },
    Note {
        id: 4,
        title: "A forth blog",
        content: "Lorem ipsum dolor sit amet",
    },
];

#[tauri::command]
fn greet(name: &str) -> String {
    let res = format!("Hello, {name}! You've been greeted from Rust!");
    println!("{res}");
    res
}
#[tauri::command]
fn get_notes() -> [Note<&'static str>; 4] {
    NOTES
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
