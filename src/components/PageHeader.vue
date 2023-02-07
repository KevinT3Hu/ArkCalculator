<script setup lang="ts">
import { useNotification } from 'naive-ui';
import { reactive, ref } from 'vue'
import AddIcon from '../assets/AddIcon.vue';
import DoneIcon from '../assets/DoneIcon.vue';
import DeleteIcon from '../assets/DeleteIcon.vue';
import { ProfileManager } from '../helpers/ProfileManager';
import { I18n } from '../i18n/strings';

const props = defineProps<{
  defProfile: string
}>()

const i18n = I18n.getInstance();

const selected = ref(props.defProfile)

const createNewOpen = ref(false)

const newProfileName = ref('')

const emits = defineEmits<{
  (event: 'updateProfile', profile: string): void
}>()

const loadedProfileList:{label:string,value:string}[] = [];
const profileManager = await ProfileManager.getProfileManager();
profileManager.profileList.forEach((profile) => {
  console.log(profile);
  loadedProfileList.push({ label: profile, value: profile });
});

const profileOptions = reactive(loadedProfileList);

function setSelectedValue(value: string) {
  selected.value = value;
  emits('updateProfile', value)
}

function handleNewProfile() {
  console.log("handleNewProfile");
  profileManager.createProfile(newProfileName.value);
  createNewOpen.value = false;
  profileOptions.push({ label: newProfileName.value, value: newProfileName.value });
  setSelectedValue(newProfileName.value);
  newProfileName.value = '';
  useNotification().success({
    title: 'Success',
    content: 'Profile created',
  });
}

function handleDeleteProfile() {
  console.log("handleDeleteProfile");
  profileManager.deleteProfile(selected.value);
  const list = profileOptions.filter((option) => option.value != selected.value);
  profileOptions.splice(0, profileOptions.length, ...list);
  setSelectedValue(profileOptions[0].value);
  useNotification().success({
    title: 'Success',
    content: 'Profile deleted',
  });
}
</script>

<template>

  <div id="header" class="row page-header">
    <h2 class="title">{{ i18n.getStringDef("head") }}</h2>
    <n-select class="profile-select" v-model:value="selected" :options="profileOptions"
      @update:value="(value: string) => $emit('updateProfile',value)" :default-value="props.defProfile" />
    <n-tooltip trigger="hover" placement="bottom">
      <template #trigger>
        <n-button text class="icon-btn" @click="handleDeleteProfile">
          <DeleteIcon />
        </n-button>
      </template>
      <span>{{ i18n.getStringDef("tooltip_delete_profile") }}</span>
    </n-tooltip>
    <n-tooltip trigger="hover" placement="bottom">
      <template #trigger>
        <n-button text class="icon-btn" @click="createNewOpen = !createNewOpen">
          <AddIcon />
        </n-button>
      </template>
      <span>{{ i18n.getStringDef("tooltip_add_profile") }}</span>
    </n-tooltip>
    <div class="row" :class="{ 'hidden': !createNewOpen }">
      <n-input v-model:value="newProfileName" placeholder="Profile Name" />
      <n-button text class="icon-btn" @click="handleNewProfile">
        <DoneIcon />
      </n-button>
    </div>
  </div>

</template>

<style scoped>

.page-header {
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
</style>