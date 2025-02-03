<script setup lang="ts">
import Button from "../Button.vue";
import HStack from "../HStack.vue";
import Text from "../Text.vue";
</script>

<template>
  <HStack class="items-center justify-between">
    <Text class="hover:underline" variant="accent" size="md">magwoo.ru</Text>
    <Text variant="neutral" size="md">17 мар. 15:34</Text>
    <HStack class="items-center gap-16">
      <HStack class="items-center gap-8">
        <HStack class="items-center gap-3">
          <Text variant="neutral" size="sm">мин.</Text>
          <span class="text-success text-xl">27ms</span>
        </HStack>
        <HStack class="items-center gap-3">
          <Text variant="neutral" size="sm">ср.</Text>
          <span class="text-warn text-xl">129ms</span>
        </HStack>
        <HStack class="items-center gap-3">
          <Text variant="neutral" size="sm">макс.</Text>
          <span class="text-error text-xl">490ms</span>
        </HStack>
      </HStack>
      <Button
        variant="transparent"
        size="lg"
        class="hover:text-accent cursor-pointer"
        >подробнее
      </Button>
    </HStack>
  </HStack>
</template>
