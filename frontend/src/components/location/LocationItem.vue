<script setup lang="ts">
import HStack from "../HStack.vue";
import Text from "../Text.vue";
</script>

<template>
  <HStack class="w-full items-center justify-between">
    <HStack class="items-center gap-6">
      <div class="bg-success size-3 shrink-0 rounded-full" />
      <HStack class="items-center gap-4">
        <img src="/flags/ru.png" class="h-6" />
        <Text variant="accent" class="opacity-75">Москва</Text>
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
