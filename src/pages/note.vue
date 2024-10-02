<script setup lang="ts">
import { reactive, ref } from "vue";

const state = reactive({
    loading: true,
});

const notes = ref<Note[]>([]);
async function initNotes() {
    notes.value = await window.__TAURI__.core.invoke("get_notes");
}

initNotes()
    .then(() => (state.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <div>
        <!-- The current route is accessible as $route in the template -->
        User {{ $route.params.id }}
    </div>
</template>
