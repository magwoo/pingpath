<script setup lang="ts">
import Text from "../Text.vue";
import VStack from "../VStack.vue";
</script>

<template>
  <div
    class="absolute -top-3 left-1/2 hidden w-max -translate-x-1/2 -translate-y-full justify-center group-hover:flex"
  >
    <VStack class="bg-accent z-10 max-w-[200px] gap-1 p-2 text-left">
      <Text class="text-white" size="md">Нидерланды,<br />Амстердам</Text>
      <span class="text-success text-xs">задержка: 12мс</span>
      <div
        class="bg-accent absolute bottom-0 left-1/2 -z-10 h-3 w-3 -translate-x-1/2 translate-y-1 rotate-45"
      />
    </VStack>
  </div>
</template>
