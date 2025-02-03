<script setup lang="ts">
import HStack from "../HStack.vue";
import ConnectionBar from "../svg/ConnectionBar.vue";
import Text from "../Text.vue";
import VStack from "../VStack.vue";
</script>

<template>
  <HStack class="w-full items-center justify-between">
    <HStack class="items-center gap-6">
      <div class="bg-success size-3 rounded-full" />
      <HStack class="items-center gap-4">
        <img src="/flags/ru.png" class="max-h-5" />
        <span class="text-accent/75 text-xl font-medium">Москва</span>
      </HStack>
    </HStack>
    <ConnectionBar />
  </HStack>
</template>
