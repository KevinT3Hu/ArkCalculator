import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { invoke } from "@tauri-apps/api";
import { I18n } from "./i18n/strings";
import { appWindow } from "@tauri-apps/api/window";
import { createPinia } from "pinia";
import { ConfigManager, Configs } from "./helpers/ConfigManager";
import { useFeatStore, useProfileStore } from "./store";

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
const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

invoke("init").then(async ()=>{
    const defProfile:string = await ConfigManager.getConfigOr(Configs.DEFAULT_PROFILE,"Default");
    const useLvFeat:boolean = await ConfigManager.getConfigOr(Configs.USE_LV_FEAT,true);
    const useSkillFeat:boolean = await ConfigManager.getConfigOr(Configs.USE_SKILL_FEAT,true);

    const profileStore = useProfileStore();
    profileStore.profile = defProfile;

    const featStore = useFeatStore();
    featStore.useLvFeature = useLvFeat;
    featStore.useSkillFeature = useSkillFeat;
})

app.mount("#app");