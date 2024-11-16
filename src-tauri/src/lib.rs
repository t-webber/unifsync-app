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
#![allow(clippy::blanket_clippy_restriction_lints, reason = "use restriction")]
#![allow(clippy::implicit_return, reason = "not necessary")]
#![allow(clippy::single_call_fn, reason = "usefull to segment code")]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::arithmetic_side_effects,
    reason = "I am lazy"
)]
#![allow(clippy::question_mark_used, reason = "very usefull")]
#![allow(clippy::mod_module_files, reason = "avoid fs complexity")]
#![allow(clippy::print_stderr, reason = "debugging")]
#![allow(
    clippy::allow_attributes_without_reason,
    clippy::pub_use,
    reason = "tauri requires those"
)]
#![allow(clippy::allow_attributes, reason = "new feature still bugged")]
#![allow(
    clippy::module_name_repetitions,
    reason = "conveniant to not `use` directly the functions"
)]

use notes::{create_note, delete_note, get_notes, update_note};

mod notes;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
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
            update_note,
            create_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
