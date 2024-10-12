import "./assets/index.css";
import "./types";

// Create App from vue files

import { createApp } from "vue";
import App from "./pages/App.vue";
const app = createApp(App);

// Add icons

import { OhVueIcon, addIcons } from "oh-vue-icons";
import {
    HiMenu,
    LaSyncAltSolid,
    IoSettingsSharp,
    BiChevronDown,
    BiChevronUp,
    HiPlusSm,
    BiHouseDoor,
} from "oh-vue-icons/icons";
addIcons(
    HiMenu,
    LaSyncAltSolid,
    IoSettingsSharp,
    BiChevronDown,
    HiPlusSm,
    BiChevronUp,
    BiHouseDoor
);
app.component("v-icon", OhVueIcon);

// Mount the app on html

app.mount("#app");
