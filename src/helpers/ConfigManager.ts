import { invoke } from "@tauri-apps/api";

export class ConfigManager{

    static async setConfig(key:string, value:any){
        await invoke("set_config", {key:key, value:value});
    }

    static async getConfigOr(key:string, defaultValue:any):Promise<any>{
        return invoke("get_config", {key:key, default:defaultValue});
    }
}

export class Configs{
    static DEFAULT_PROFILE:string = "defProfile";
    static USE_LV_FEAT:string = "useLvFeat";
    static USE_SKILL_FEAT:string = "useSkillFeat";
}