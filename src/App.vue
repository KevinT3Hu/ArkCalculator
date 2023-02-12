<script setup lang="ts">
import { NNotificationProvider } from "naive-ui";
import MainTable from "./components/MainTable.vue";
import PageHeader from "./components/PageHeader.vue";

</script>

<template>
  <div class="app-container column">
    <NNotificationProvider>
      <PageHeader />
      <MainTable />
    </NNotificationProvider>
  </div>
</template>

<style scoped>
.app-container {
  height: 100vh;
  width: 100vw;
}
</style>