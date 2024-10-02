<script setup lang="ts">
import { reactive, ref } from "vue";
import { Separator } from "@/components/ui/separator";

const store = reactive({
    activated: true,
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
    .then(() => (store.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <div class="flex h-screen">
        <header class="p-4 bg-muted space-y-4 h-full">
            <button @click="store.activated = !store.activated" class="w-full">
                <v-icon name="hi-menu" />
            </button>
            <nav v-if="store.activated && !store.loading" class="space-y-4">
                <ul>
                    <li>
                        <RouterLink to="/">Home</RouterLink>
                    </li>
                </ul>
                <Separator class="w-full h-2 bg-background" />
                <ul>
                    <li v-for="note in notes">
                        <RouterLink :to="'/note/' + note.id">{{
                            note.title
                        }}</RouterLink>
                        <p>{{ note.content }}</p>
                    </li>
                </ul>
            </nav>
            <div v-else-if="store.activated">Loading</div>
        </header>
        <main class="p-4">
            <p><strong>Current route path:</strong> {{ $route.fullPath }}</p>
            <RouterView />
        </main>
    </div>
</template>
