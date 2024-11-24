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
    clippy::allow_attributes_without_reason,
    clippy::pub_use,
    reason = "tauri requires those"
)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::missing_docs_in_private_items, clippy::arithmetic_side_effects)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::print_stderr)]
#![allow(clippy::allow_attributes)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::ref_patterns)]

mod env;
mod errors;
extern crate logs;
mod notes;

use notes::{create_note, delete_note, get_notes, init_notes, update_note};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[allow(clippy::expect_used)]
#[allow(clippy::missing_inline_in_public_items, clippy::exit)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// # Panics
/// Function panics when context for running the app can't be loaded
pub fn run() {
    init_notes();
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
