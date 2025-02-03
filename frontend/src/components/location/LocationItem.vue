<script setup lang="ts">
import HStack from "../HStack.vue";
import Text from "../Text.vue";
import VStack from "../VStack.vue";
</script>

<template>
  <HStack class="w-full items-center justify-between">
    <HStack class="items-center gap-6">
      <div class="bg-success size-3 shrink-0 rounded-full" />
      <HStack class="items-center gap-4">
        <img src="/flags/ru.png" class="h-6" />
        <span class="text-accent/75 text-xl font-medium">Москва</span>
      </HStack>
    </HStack>
    <HStack class="h-7 items-end gap-1">
      <div class="bg-neutral/25 h-[10%] w-2" />
      <div class="bg-neutral/25 h-[30%] w-2" />
      <div class="bg-neutral/25 h-[60%] w-2" />
      <div class="bg-neutral/25 h-[75%] w-2" />
      <div class="bg-neutral/25 h-[90%] w-2" />
    </HStack>
  </HStack>
</template>
