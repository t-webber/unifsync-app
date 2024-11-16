<script setup lang="ts">
import { updateNote, deleteNote } from "@/tauri";
import { NoteProps } from "@/types";
import { reactive, watch } from "vue";

const props = defineProps<{ note: NoteProps }>();

const state = reactive({
    title: props.note.title,
    content: props.note.content,
    nb: props.nb,
});

watch(state, async () => {
    console.log(state.title);
    updateNote(props.note.id, state.title, state.content);
});

function onChange(key: any) {
    key.target.style.height = "0px";
    key.target.style.height = 25 + key.target.scrollHeight + "px";
}
</script>

<template>
    <div class="flex">
        <input v-model="state.title" class="w-full bg-transparent text-2xl" />
        <button :onclick="deleteNote(state.nb)">
            <!-- missing update and redirect -->
            <v-icon name="io-trash-bin" />
        </button>
    </div>
    <textarea
        v-model="state.content"
        class="w-full bg-transparent"
        :onkeydown="onChange"
        :onkeyup="onChange"
    ></textarea>
</template>
