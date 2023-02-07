import { createApp } from "vue";
import "./style.css";
import SuspensePage from "./SuspensePage.vue";
import { invoke } from "@tauri-apps/api";
import { I18n } from "./i18n/strings";
import { appWindow } from "@tauri-apps/api/window";

invoke("init").then(()=>
    console.log("Tauri init success")
).catch(()=>
    console.log("Tauri init failed")
);

const language = navigator.language;

console.log("Language: " + language);

const i18n = I18n.getInstance();
i18n.init(language,"en");

appWindow.setTitle(i18n.getStringDef("app_name"));

//prevent right click in release mode
if (process.env.NODE_ENV === "production") {
    document.addEventListener("contextmenu", (e) => {
        e.preventDefault();
    });
}
const app = createApp(SuspensePage);
app.mount("#app");