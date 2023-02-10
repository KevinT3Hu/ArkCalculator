import { defineStore } from 'pinia';
import { ConfigManager, Configs } from './helpers/ConfigManager';

const configManager = await ConfigManager.getConfigManager();

export const useProfileStore = defineStore({
    id: 'profile',
    state: () => ({
        profile: configManager.getConfigOr(Configs.DEFAULT_PROFILE,"Default"),
    }),
    actions:{
        setProfile(profile:string){
            this.profile = profile;
            configManager.setConfig(Configs.DEFAULT_PROFILE, profile);
        }
    },
});

export const useFeatStore = defineStore({
    id: 'feat',
    state: () => ({
        useLvFeature: configManager.getConfigOr(Configs.USE_LV_FEAT,true),
        useSkillFeature: configManager.getConfigOr(Configs.USE_SKILL_FEAT,true),
    }),
    actions:{
        setUseLvFeature(useLvFeature:boolean){
            this.useLvFeature = useLvFeature;
            configManager.setConfig(Configs.USE_LV_FEAT, useLvFeature);
        },

        setUseSkillFeature(useSkillFeature:boolean){
            this.useSkillFeature = useSkillFeature;
            configManager.setConfig(Configs.USE_SKILL_FEAT, useSkillFeature);
        }
    },
});