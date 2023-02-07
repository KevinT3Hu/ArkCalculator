import { fs } from "@tauri-apps/api";
import { appConfigDir,join } from "@tauri-apps/api/path"
import { OperatorTarget } from "../types";

export class ProfileManager{

    private _initialized:boolean = false;

    private static _profileManager:ProfileManager|undefined;

    private _profileList:string[] = [];

    private profileDirectory:string|undefined;

    get profileList():string[]{
        return this._profileList;
    }

    private constructor(){}

    async init():Promise<void>{
        this.profileDirectory = await join(await appConfigDir(), "profiles");
        console.log(this.profileDirectory);
        await fs.createDir(this.profileDirectory, { recursive: true });
        this._profileList = await this.getProfileList();
        if(this._profileList.length==0){
            await this.createProfile("default");
        }
    }

    static async getProfileManager():Promise<ProfileManager>{
        if(!this._profileManager){
            this._profileManager = new ProfileManager();
        }
        if(!this._profileManager._initialized){
            await this._profileManager.init();
            this._profileManager._initialized = true;
        }
        return this._profileManager;
    }

    private async getProfileList():Promise<string[]>{
        if(this.profileDirectory==undefined){
            return [];
        }
        const profileList = await fs.readDir(this.profileDirectory);
        var profileNameList:string[] = [];
        profileList.forEach(profile => {
            const name = profile.name;
            if(name==undefined){
                return;
            }
            if(name.endsWith(".json")){
                profileNameList.push(name.replace(".json", ""));
            }
        });
        console.log(profileNameList);
        return profileNameList;
    }

    async createProfile(name:string):Promise<void>{
        if(this.profileDirectory==undefined){
            return;
        }
        await fs.writeTextFile(await join(this.profileDirectory,`${name}.json`), "[]");
        this._profileList.push(name);
    }

    async deleteProfile(name:string):Promise<void>{
        if(this.profileDirectory==undefined){
            return;
        }
        await fs.removeFile(await join(this.profileDirectory, `${name}.json`));
        this._profileList = this._profileList.filter(profileName => profileName!=name);
    }

    async loadProfile(name:string):Promise<OperatorTarget[]>{
        if(this.profileDirectory==undefined){
            return [];
        }
        const profile = await fs.readTextFile(await join(this.profileDirectory, `${name}.json`));
        return JSON.parse(profile);
    }

    async saveProfile(name:string, profile:OperatorTarget[]):Promise<void>{
        console.log(`saving profile ${name}`);
        if(this.profileDirectory==undefined){
            return;
        }
        await fs.writeTextFile(await join(this.profileDirectory, `${name}.json`), JSON.stringify(profile));
    }
}