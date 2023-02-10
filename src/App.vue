<script setup lang="ts">
import MainTable from "./components/MainTable.vue";
import PageHeader from "./components/PageHeader.vue";

</script>

<template>
    <div class="app-container column">
      <PageHeader />
      <MainTable />
    </div>
</template>

<style scoped>

.app-container {
  height: 100vh;
  width: 100vw;
}

</style>