declare global {
    interface Window {
        __TAURI__: {
            core: {
                invoke: (cmd: string, props?: object) => any;
            };
        };
    }
}

// Create App from vue files

import { createApp } from "vue";
import App from "./pages/App.vue";
import "./assets/index.css";
const app = createApp(App);

// Add icons

import { OhVueIcon, addIcons } from "oh-vue-icons";
import { HiMenu } from "oh-vue-icons/icons";
addIcons(HiMenu);
app.component("v-icon", OhVueIcon);

// Add router

import { createMemoryHistory, createRouter } from "vue-router";

import Home from "./pages/home.vue";
import Note from "./pages/note.vue";

const routes = [
    { path: "/", component: Home },
    { path: "/note/:id", component: Note },
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

app.use(router);

// Mount the app on html

app.mount("#app");
