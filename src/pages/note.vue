<script setup lang="ts">
defineProps({
    id: Number,
    title: String,
    content: String,
});

import { reactive, ref } from "vue";

const state = reactive({
    loading: true,
});

interface Note {
    id: number;
    title: string;
    content: string;
}

const notes = ref<Note[]>([]);
async function initNotes() {
    notes.value = await window.__TAURI__.core.invoke("get_notes");
}

initNotes()
    .then(() => (state.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <h2>{{ title }}</h2>
    <p>
        {{ content }}
    </p>
</template>
