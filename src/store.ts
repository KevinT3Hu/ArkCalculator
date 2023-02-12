import { defineStore } from 'pinia';
import { ConfigManager, Configs } from './helpers/ConfigManager';

export const useProfileStore = defineStore({
    id: 'profile',
    state: () => ({
        profile: 'Default',
    }),
    actions:{
        async setProfile(profile:string){
            this.profile = profile;
            await ConfigManager.setConfig(Configs.DEFAULT_PROFILE, profile);
        }
    },
});

export const useFeatStore = defineStore({
    id: 'feat',
    state: () => ({
        useLvFeature: false,
        useSkillFeature: false,
    }),
    actions:{
        async setUseLvFeature(useLvFeature:boolean){
            this.useLvFeature = useLvFeature;
            await ConfigManager.setConfig(Configs.USE_LV_FEAT, useLvFeature);
        },

        async setUseSkillFeature(useSkillFeature:boolean){
            this.useSkillFeature = useSkillFeature;
            await ConfigManager.setConfig(Configs.USE_SKILL_FEAT, useSkillFeature);
        }
    },
});