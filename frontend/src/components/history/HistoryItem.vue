<script setup lang="ts">
import Button from "../Button.vue";
import HStack from "../HStack.vue";
</script>

<template>
  <HStack class="items-center justify-between">
    <button
      class="text-accent cursor-pointer text-xl font-medium hover:underline"
    >
      magwoo.ru
    </button>
    <span class="text-neutral/50 text-xl font-medium">17 мар. 15:34</span>
    <HStack class="items-center gap-16">
      <HStack class="items-center gap-8">
        <HStack class="items-center gap-3">
          <span class="text-neutral/50 font-medium">мин.</span>
          <span class="text-success text-xl font-medium">27ms</span>
        </HStack>
        <HStack class="items-center gap-3">
          <span class="text-neutral/50 font-medium">ср.</span>
          <span class="text-warn text-xl font-medium">129ms</span>
        </HStack>
        <HStack class="items-center gap-3">
          <span class="text-neutral/50 font-medium">макс.</span>
          <span class="text-error text-xl font-medium">490ms</span>
        </HStack>
      </HStack>
      <Button
        variant="transparent"
        size="lg"
        class="hover:text-accent text-neutral/50 cursor-pointer"
        >подробнее
      </Button>
    </HStack>
  </HStack>
</template>
