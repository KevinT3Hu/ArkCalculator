<script setup lang="ts">
import { config } from "process";
import { ref } from "vue";
import MainTable from "./components/MainTable.vue";
import PageHeader from "./components/PageHeader.vue";
import { ConfigManager,Configs } from "./helpers/ConfigManager";

const configManager = await ConfigManager.getConfigManager();

const selectedProfile = ref(configManager.getConfigOr(Configs.DEFAULT_PROFILE,"default"));

const useLvFeature = ref(configManager.getConfigOr(Configs.USE_LV_FEAT,true));
const useSkillFeature = ref(configManager.getConfigOr(Configs.USE_SKILL_FEAT,true));

function updateProfile(profile: string) {
  selectedProfile.value = profile;
  console.log("update profile:", profile);
}

function updateFeatures(){
  useLvFeature.value = configManager.getConfigOr(Configs.USE_LV_FEAT,true);
  useSkillFeature.value = configManager.getConfigOr(Configs.USE_SKILL_FEAT,true);
  console.log("update features:", useLvFeature.value, useSkillFeature.value);
}

</script>

<template>
    <div class="container column">
      <PageHeader @update-profile="updateProfile" @update-use-lv-feature="updateFeatures" @update-use-skill-feature="updateFeatures" :def-profile="selectedProfile"/>
      <MainTable :profile="selectedProfile" :use-lv-feature="useLvFeature" :use-skill-feature="useSkillFeature"/>
    </div>
</template>