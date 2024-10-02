<script setup lang="ts">
import { reactive } from "vue";

const store = reactive({
    activated: true,
});
</script>

<template>
    <div class="flex h-screen">
        <div class="p-4 bg-muted space-y-4 h-full">
            <button @click="store.activated = !store.activated" class="w-full">
                <v-icon name="hi-menu" />
            </button>
            <div v-if="store.activated">
                <ul>
                    <li><a>Hello World4</a></li>
                    <li><a>Hello World4</a></li>
                    <li><a>Hello World4</a></li>
                    <li><a>Hello World4</a></li>
                </ul>
            </div>
        </div>
        <div>
            <div>
                <div v-if="store.activated">Edit</div>
                <div v-else>Save</div>
            </div>
        </div>
    </div>
</template>

<style scoped></style>
