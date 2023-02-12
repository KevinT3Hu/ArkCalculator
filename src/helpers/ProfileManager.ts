import { invoke } from "@tauri-apps/api";
import { OperatorTarget } from "../types";

export class ProfileManager{

    static async getProfileList():Promise<string[]>{
        return invoke("get_profile_list");
    }

    static async createProfile(name:string):Promise<void>{
        return invoke("create_profile", {profileName:name});
    }

    static async deleteProfile(name:string):Promise<void>{
        return invoke("delete_profile", {profileName:name});
    }

    static async loadProfile(name:string):Promise<OperatorTarget[]>{
        return invoke("load_profile", {profileName:name});
    }

    static async saveProfile(name:string, profile:OperatorTarget[]):Promise<void>{
        return invoke("save_profile", {profileName:name, profile:profile});
    }
}