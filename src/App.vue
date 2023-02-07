<script setup lang="ts">
import { ref } from "vue";
import MainTable from "./components/MainTable.vue";
import PageHeader from "./components/PageHeader.vue";
import { ConfigManager,DEFAULT_PROFILE } from "./helpers/ConfigManager";

const configManager = await ConfigManager.getConfigManager();
const defProfile:string = configManager.getConfigOr(DEFAULT_PROFILE,"default");

const selectedProfile = ref(defProfile);

function updateProfile(profile: string) {
  selectedProfile.value = profile;
  configManager.setConfig(DEFAULT_PROFILE, profile);
  console.log("update profile:", profile);
}

</script>

<template>
    <div class="container column">
      <PageHeader @update-profile="updateProfile" :def-profile="defProfile"/>
      <MainTable :profile="selectedProfile"/>
    </div>
</template>