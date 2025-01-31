<script setup lang="ts">
import { ref } from "vue";

const counter = ref<number>(0);
</script>

<template>
  <button
    @click="counter++"
    class="cursor-pointer rounded-md bg-blue-500 px-3 py-1 text-white"
  >
    click me! {{ counter }}
  </button>
</template>
