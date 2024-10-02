<script setup lang="ts">
import { reactive, ref } from "vue";

const store = reactive({
    activated: true,
    loading: true,
});
const name = ref("unknown");
async function setName() {
    name.value = await window.__TAURI__.core.invoke("greet", { name: "Bob" });
}

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
    .then(() => (store.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <div class="flex h-screen">
        <div class="p-4 bg-muted space-y-4 h-full">
            <button @click="store.activated = !store.activated" class="w-full">
                <v-icon name="hi-menu" />
            </button>
            <div v-if="store.activated && !store.loading">
                <ul>
                    <li v-for="note in notes">
                        <a>{{ note.title }}</a>
                        <p>{{ note.content }}</p>
                    </li>
                </ul>
            </div>
            <div v-else-if="store.activated">Loading</div>
        </div>
        <div>
            <div v-if="store.activated">Edit</div>
            <div v-else>Save</div>
            <button @click="setName">Set name</button>
            <div>{{ name }}</div>
        </div>
    </div>
</template>

<style scoped></style>
