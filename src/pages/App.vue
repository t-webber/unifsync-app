<script setup lang="ts">
import { reactive, ref } from "vue";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import Note from "./note.vue";
import Home from "./home.vue";
import { NoteProps } from "@/types";

const nav: {
    open: boolean;
    loading: boolean;
    unfolded: boolean;
    noteNb: null | number;
} = reactive({
    open: true,
    loading: true,
    unfolded: true,
    noteNb: null,
});

const notes = ref<NoteProps[]>([]);
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
                        <Button variant="link" @click="nav.noteNb = null">
                            <p v-if="nav.open">Home</p>
                            <v-icon v-else name="bi-house-door" />
                        </Button>
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
                    <Button
                        variant="link"
                        class="text-center"
                        @click="initNotes"
                    >
                        <v-icon name="la-sync-alt-solid" />
                    </Button>
                    <Button variant="link" class="text-center">
                        <v-icon name="io-settings-sharp" />
                    </Button>
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
                                v-for="(note, nb) in notes"
                                class="flex flex-col items-center"
                            >
                                <Button
                                    variant="link"
                                    @click="nav.noteNb = nb"
                                    class="hover:underline"
                                >
                                    {{ note.title }}
                                </Button>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
        <main class="p-4">
            <h1 class="p-4">UnifSync</h1>
            <div v-if="nav.loading">
                <p>Loading</p>
                >
            </div>
            <div v-else-if="nav.noteNb === null">
                <router-view v-solt="{ Home }">
                    <div class="flex flex-col space-y-4 p-4">
                        <component :is="Home" />
                    </div>
                </router-view>
            </div>
            <div v-else>
                <router-view v-solt="{ Note }">
                    <div class="flex flex-col space-y-4 p-4">
                        <component
                            :is="Note"
                            :title="notes[nav.noteNb].title"
                            :content="notes[nav.noteNb].content"
                        />
                    </div>
                </router-view>
            </div>
        </main>
    </div>
</template>
