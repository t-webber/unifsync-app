<script setup lang="ts">
import Note from "./note.vue";
import Home from "./home.vue";
import Settings from "./settings.vue";

import { reactive, ref } from "vue";

import { NoteProps } from "@/types";
import { createNote, getNotes } from "@/tauri";

import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";

type Pages = "home" | "edit" | "settings";

const state: {
    open: boolean;
    loading: boolean;
    unfolded: boolean;
    page: Pages;
    noteNb: null | number;
} = reactive({
    open: true,
    loading: true,
    unfolded: true,
    page: "home",
    noteNb: null,
});

const notes = ref<NoteProps[]>([]);
async function initNotes() {
    notes.value = await getNotes();
}

function nav(href: Pages) {
    if (href !== "edit") {
        state.noteNb = null;
    }
    state.page = href;
}

function newNote() {
    createNote().then((id) => {
        state.noteNb = id;
        state.page = "edit";
    });
}

initNotes()
    .then(() => (state.loading = false))
    .catch((err) => console.error("[initNotes] ", err));
</script>

<template>
    <div class="flex h-screen">
        <header class="bg-muted space-y-4 pt-4 h-full" :key="state">
            <h1 class="pt-4 px-4 w-full text-2xl text-center" v-if="state.open">
                UnifSync
            </h1>
            <h1 class="pt-4 w-full text-2xl text-center" v-else>US</h1>
            <Button
                variant="ghost"
                @click="state.open = !state.open"
                class="w-full"
            >
                <v-icon name="hi-menu" />
            </Button>
            <nav class="space-y-4 h-full">
                <ul>
                    <li class="m-auto w-fit">
                        <Button variant="link" @click="nav('home')">
                            <p v-if="state.open">Home</p>
                            <v-icon v-else name="bi-house-door" />
                        </Button>
                    </li>
                </ul>
                <Separator v-if="state.open" class="w-full h-1 bg-background" />
                <div
                    :class="[
                        state.open
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
                    <Button
                        @click="state.page = 'settings'"
                        variant="link"
                        class="text-center"
                    >
                        <v-icon name="io-settings-sharp" />
                    </Button>
                </div>
                <div v-if="notes.length > 0" class="space-y-4">
                    <Separator
                        v-if="state.open"
                        class="w-full h-1 bg-background"
                    />
                    <div v-if="state.open" class="space-y-4 p-4 pt-0">
                        <Button
                            variant="outline"
                            @click="state.unfolded = !state.unfolded"
                            class="p-2"
                        >
                            <div
                                v-if="state.unfolded"
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
                        <div v-if="state.loading">Loading</div>
                        <div v-else-if="state.unfolded">
                            <ul class="space-y-4">
                                <li
                                    v-for="(note, nb) in notes"
                                    class="flex flex-col items-center"
                                >
                                    <Button
                                        variant="link"
                                        @click="
                                            state.noteNb = nb;
                                            state.page = 'edit';
                                        "
                                        class="hover:underline"
                                    >
                                        {{ note.title }}
                                    </Button>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <Separator v-if="state.open" class="w-full h-1 bg-background" />
                <Button @click="newNote()" variant="link" class="w-full">
                    <div v-if="state.open">New note</div>
                    <div v-else><v-icon name="hi-plus-sm" /></div>
                </Button>
            </nav>
        </header>
        <main class="p-4 w-full" :key="state">
            <div v-if="state.loading">
                <p>Loading</p>
            </div>
            <div v-else-if="state.page == 'settings'">
                <Settings />
            </div>
            <div v-else-if="state.noteNb === null">
                <Home />
            </div>
            <div v-else class="h-full">
                <Note
                    :note="notes[state.noteNb]"
                    :nb="state.noteNb"
                    :key="state.noteNb"
                />
                <!-- add key to force refresh when noteNb is changed -->
            </div>
        </main>
    </div>
</template>
