<script setup lang="ts">
import { DropdownOption, NCheckbox, NDropdown, NSelect, NTooltip, useNotification } from 'naive-ui';
import { h, reactive, ref, watch } from 'vue'
import AddIcon from '../assets/AddIcon.vue';
import DoneIcon from '../assets/DoneIcon.vue';
import DeleteIcon from '../assets/DeleteIcon.vue';
import { ProfileManager } from '../helpers/ProfileManager';
import { I18n } from '../i18n/strings';
import SettingsIcon from '../assets/SettingsIcon.vue';
import { useFeatStore, useProfileStore } from '../store';

const i18n = I18n.getInstance();

const createNewOpen = ref(false)

const newProfileName = ref('')

const profileStore = useProfileStore();
const featStore = useFeatStore();

const selectedProfile = ref<string>(profileStore.profile);

watch(selectedProfile, (value) => {
  profileStore.setProfile(value);
});

const loadedProfileList:{label:string,value:string}[] = [];
const profileManager = await ProfileManager.getProfileManager();
profileManager.profileList.forEach((profile) => {
  console.log(profile);
  loadedProfileList.push({ label: profile, value: profile });
});

const profileOptions = reactive(loadedProfileList);

const menuKeys={
  feat:"feat",
  featLv:"featLv",
  featSkill:"featSkill",
}

const menuOptions:DropdownOption[] = [
  {
    label: i18n.getStringDef("menu_feat"),
    key: "feat",
    children:[
      {
        label: i18n.getStringDef("menu_feat_lv"),
        key: "featLv",
        icon(){
          return h(NCheckbox, {
            checked: featStore.useLvFeature,
          })
        }
      },
      {
        label: i18n.getStringDef("menu_feat_skill"),
        key: "featSkill",
        icon(){
          return h(NCheckbox, {
            checked: featStore.useSkillFeature,
          })
        }
      }
    ]
  }
]

function handleNewProfile() {
  console.log("handleNewProfile");
  profileManager.createProfile(newProfileName.value);
  createNewOpen.value = false;
  profileOptions.push({ label: newProfileName.value, value: newProfileName.value });
  selectedProfile.value = newProfileName.value;
  newProfileName.value = '';
  useNotification().success({
    title: 'Success',
    content: 'Profile created',
  });
}

function handleDeleteProfile() {
  console.log("handleDeleteProfile");
  profileManager.deleteProfile(profileStore.profile);
  const list = profileOptions.filter((option) => option.value != profileStore.profile);
  if (list.length == 0) {
    profileManager.createProfile("Default");
    list.push({ label: "Default", value: "Default" });
  }
  profileOptions.splice(0, profileOptions.length, ...list);
  selectedProfile.value = list[0].value;
}

function handleMenuSelect(key: string|number){
  switch(String(key)){
    case menuKeys.featLv:
      featStore.setUseLvFeature(!featStore.useLvFeature);
      break;
    case menuKeys.featSkill:
      featStore.setUseSkillFeature(!featStore.useSkillFeature);
      break;
  }
}
</script>

<template>

  <div id="header" class="row page-header noselect">
    <h2 class="title">{{ i18n.getStringDef("head") }}</h2>
    <NSelect class="profile-select" v-model:value="selectedProfile" :options="profileOptions"
      @update:value="(value: string) => $emit('updateProfile',value)" :default-value="selectedProfile" />
    <NTooltip trigger="hover" placement="bottom">
      <template #trigger>
        <NButton text class="icon-btn" @click="handleDeleteProfile">
          <DeleteIcon />
        </NButton>
      </template>
      <span>{{ i18n.getStringDef("tooltip_delete_profile") }}</span>
    </NTooltip>
    <NTooltip trigger="hover" placement="bottom">
      <template #trigger>
        <NButton text class="icon-btn" @click="createNewOpen = !createNewOpen">
          <AddIcon />
        </NButton>
      </template>
      <span>{{ i18n.getStringDef("tooltip_add_profile") }}</span>
    </NTooltip>
    <div class="row" :class="{ 'hidden': !createNewOpen }">
      <n-input v-model:value="newProfileName" placeholder="Profile Name" />
      <NButton text class="icon-btn" @click="handleNewProfile">
        <DoneIcon />
      </NButton>
    </div>
    <NDropdown trigger="hover" :options="menuOptions" @select="handleMenuSelect" placement="bottom-start">
        <NButton text class="icon-btn menu-icon">
          <SettingsIcon />
        </NButton>
    </NDropdown>
  </div>

</template>

<style scoped>

.page-header {
  height: 60px;
  background-color: var(--bar-background-color);
  position:sticky;
  top:0;
}
.title {
  margin: 1%;
}

.profile-select {
  float: left;
  width: 200px;
  padding-left: 5%;
}

.icon-btn {
  margin-left: 1%;
  font-size: 24px;
}

.hidden {
  visibility: hidden;
}

.menu-icon {
  float: right;
  margin-left: auto;
  margin-right: 1%;
}
</style>