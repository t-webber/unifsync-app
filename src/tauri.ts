import { NoteProps } from "./types";

const API = window.__TAURI__.core;
const invoke = API.invoke;

export async function getNotes(): Promise<NoteProps[]> {
    return invoke("get_notes");
}

export async function greet(name: string): Promise<string> {
    return invoke("greet", { name });
}

export async function updateNote(
    id: number,
    title: string,
    content: string
): Promise<void> {
    invoke("update_note", { id, title, content });
}

export async function createNote(): Promise<number> {
    return invoke("create_note");
}

export async function deleteNote(id: number): Promise<void> {
    invoke("delete_note", { id });
}
