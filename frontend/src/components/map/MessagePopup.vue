<script setup lang="ts">
import { ref } from "vue";
import Text from "../Text.vue";
</script>

<template>
  <div class="bg-accent flex w-max flex-col gap-1 p-2 text-left">
    <Text class="text-white" size="md"> Нидерланды, <br />Амстердам </Text>
    <span class="text-success text-xs">задержка: 12мс</span>
    <div
      class="bg-accent absolute right-0 -bottom-2 left-0 -z-10 mx-auto size-5 rotate-45"
    />
  </div>
</template>
