import { createApp } from "vue";
import App from "./App.vue";
import "./main.css";
import { OhVueIcon, addIcons } from "oh-vue-icons";
import { HiMenu } from "oh-vue-icons/icons";

declare global {
    interface Window {
        __TAURI__: {
            core: {
                invoke: (cmd: string, props?: object) => any;
            };
        };
    }
}

addIcons(HiMenu);

const app = createApp(App);
app.component("v-icon", OhVueIcon);
app.mount("#app");
