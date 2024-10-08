<script setup lang="ts">
import { greet } from "@/tauri";
import { ref } from "vue";
const name = ref("unknown");
async function setName() {
    name.value = await greet("Bob");
}
</script>

<template>
    <h2>Home</h2>
    <button class="w-fit" @click="setName">Set name</button>
    <div>{{ name }}</div>
</template>

<style scoped></style>
