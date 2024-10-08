import { NoteProps } from "./types";

const API = window.__TAURI__.core;
const invoke = API.invoke;

export async function getNotes(): Promise<NoteProps[]> {
    return invoke("get_notes");
}

export async function greet(name: string): Promise<string> {
    return invoke("greet", { name });
}

export async function updateNoteContent(
    id: number,
    content: string
): Promise<void> {
    return invoke("update_note_content", { id, content });
}
