<script setup lang="ts">
const model = defineModel();
</script>
<template>
  <input
    type="text"
    v-model="model"
    class="bg-neutral/10 text-neutral placeholder-neutral/50 w-full px-4 py-2 outline-none font-medium"
  />
</template>
