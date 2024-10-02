<script setup lang="ts">
import { ref } from "vue";
const name = ref("unknown");
async function setName() {
    name.value = await window.__TAURI__.core.invoke("greet", { name: "Bob" });
}
</script>

<template>
    <div>Home</div>
    <button @click="setName">Set name</button>
    <div>{{ name }}</div>
</template>

<style scoped></style>
