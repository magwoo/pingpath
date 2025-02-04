<script setup lang="ts">
import Button from "../Button.vue";
import HStack from "../HStack.vue";
</script>

<template>
  <HStack class="items-center justify-between">
    <Button
      variant="transparent"
      size="sm-inline"
      class="text-accent/75! hover:text-accent!"
      >magwoo.ru</Button
    >
    <span class="text-neutral/50 font-medium">17 мар. 15:34</span>
    <HStack class="items-center gap-16">
      <HStack class="items-center gap-8">
        <HStack class="items-center gap-2">
          <span class="text-neutral/50 text-xs font-medium">мин.</span>
          <span class="text-success font-medium">27ms</span>
        </HStack>
        <HStack class="items-center gap-2">
          <span class="text-neutral/50 text-xs font-medium">ср.</span>
          <span class="text-warn font-medium">129ms</span>
        </HStack>
        <HStack class="items-center gap-2">
          <span class="text-neutral/50 text-xs font-medium">макс.</span>
          <span class="text-error font-medium">490ms</span>
        </HStack>
      </HStack>
      <Button
        variant="transparent"
        size="sm-inline"
        class="text-neutral/50! hover:text-neutral/75! cursor-pointer"
        >подробнее
      </Button>
    </HStack>
  </HStack>
</template>
