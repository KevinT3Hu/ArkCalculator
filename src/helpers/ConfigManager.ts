import { fs } from "@tauri-apps/api";
import { appConfigDir, join } from "@tauri-apps/api/path";

export class ConfigManager{

    private _initialized:boolean = false;

    private static _configManager:ConfigManager|undefined;

    private constructor(){}

    private _configFilePath:string|undefined;

    private configMap:Map<string,any> = new Map();

    async init():Promise<void>{
        const configDir = await appConfigDir();
        const configFilePath = await join(configDir, "config.json");
        this._configFilePath = configFilePath;
        if(!await fs.exists(configFilePath)){
            await fs.writeTextFile(configFilePath, "{}");
        }
        const json = await fs.readTextFile(configFilePath);
        const config = new Map(Object.entries(JSON.parse(json)));
        this.configMap = config;
    }

    static async getConfigManager():Promise<ConfigManager>{
        if(!this._configManager){
            this._configManager = new ConfigManager();
        }
        if(!this._configManager._initialized){
            await this._configManager.init();
            this._configManager._initialized = true;
        }
        return this._configManager;
    }

    private async updateConfigFile():Promise<void>{
        if(this._configFilePath==undefined){
            return;
        }
        const json = JSON.stringify(Object.fromEntries(this.configMap));
        await fs.writeTextFile(this._configFilePath, json);
    }

    setConfig(key:string, value:any){
        this.configMap.set(key, value);
        this.updateConfigFile();
    }

    private getConfig(key:string):any{
        return this.configMap.get(key);
    }

    getConfigOr(key:string, defaultValue:any):any{
        const value = this.getConfig(key);
        if(value==undefined){
            return defaultValue;
        }
        return value;
    }
}

export const DEFAULT_PROFILE="defProfile"