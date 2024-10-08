<script setup lang="ts">
const props = defineProps({
    id: Number,
    title: String,
    content: String,
});

import { updateNoteContent } from "@/tauri";

function onchange(key: any) {
    updateNoteContent(props.id!, key.target.value);
    console.log("Hello world", key.target.value);
    key.target.style.height = "1px";
    key.target.style.height = 25 + key.target.scrollHeight + "px";
}
</script>

<template>
    <h2>{{ title }}</h2>
    <textarea
        :value="content"
        class="w-full bg-transparent"
        :onkeydown="onchange"
        :onkeyup="onchange"
    />
</template>
