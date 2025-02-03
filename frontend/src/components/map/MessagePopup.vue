<script setup lang="ts">
import { ref } from "vue";
import Text from "../Text.vue";
import VStack from "../VStack.vue";
</script>

<template>
  <VStack
    class="bg-accent invisible absolute -top-24 -left-14.5 z-10 gap-1 p-2 text-left group-hover:visible"
  >
    <Text class="text-white" size="md"> Нидерланды, <br />Амстердам </Text>
    <span class="text-success text-xs">задержка: 12мс</span>
    <div
      class="bg-accent absolute right-0 -bottom-2 left-0 mx-auto size-5 rotate-45 -z-10"
    />
  </VStack>
</template>
