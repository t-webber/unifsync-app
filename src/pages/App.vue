<script setup lang="ts">
import { reactive, ref } from "vue";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";

const nav = reactive({
    open: true,
    loading: true,
    unfolded: true,
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
    .then(() => (nav.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <div class="flex h-screen">
        <header class="bg-muted space-y-4 h-full">
            <Button
                variant="ghost"
                @click="nav.open = !nav.open"
                class="w-full"
            >
                <v-icon name="hi-menu" />
            </Button>
            <nav class="space-y-4 h-full">
                <ul>
                    <li class="m-auto w-fit">
                        <RouterLink to="/" class="hover:underline">
                            <p v-if="nav.open">Home</p>
                            <v-icon v-else name="bi-house-door" />
                        </RouterLink>
                    </li>
                </ul>
                <Separator v-if="nav.open" class="w-full h-1 bg-background" />
                <div
                    :class="[
                        nav.open
                            ? 'flex justify-around'
                            : 'flex flex-col space-y-4',
                    ]"
                >
                    <button class="text-center" @click="initNotes">
                        <v-icon name="la-sync-alt-solid" />
                    </button>
                    <button class="text-center">
                        <v-icon name="io-settings-sharp" />
                    </button>
                </div>
                <Separator v-if="nav.open" class="w-full h-1 bg-background" />
                <div v-if="nav.open" class="space-y-4 p-4 pt-0">
                    <Button
                        variant="outline"
                        @click="nav.unfolded = !nav.unfolded"
                        class="p-2"
                    >
                        <div
                            v-if="nav.unfolded"
                            class="flex space-x-2 items-center"
                        >
                            <p>Collapse</p>
                            <v-icon name="bi-chevron-up" />
                        </div>
                        <div v-else class="flex space-x-2 items-center">
                            <p>Unfold</p>
                            <v-icon name="bi-chevron-down" />
                        </div>
                    </Button>
                    <div v-if="nav.loading">Loading</div>
                    <div v-else-if="nav.unfolded">
                        <ul class="space-y-4">
                            <li
                                v-for="note in notes"
                                class="flex flex-col items-center"
                            >
                                <RouterLink
                                    :to="'/note/' + note.id"
                                    class="hover:underline"
                                >
                                    {{ note.title }}
                                </RouterLink>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
        <main class="p-4">
            <h1 class="p-2">UnifSync</h1>
            <RouterView />
        </main>
    </div>
</template>
