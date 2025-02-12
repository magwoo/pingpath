<script setup lang="ts">
import VStack from "@/components/VStack.vue";
import Link from "@/components/Link.vue";
</script>

<template>
  <VStack
    class="h-screen w-full items-center justify-center pb-[15vh] text-center"
  >
    <h1 class="text-accent text-[200px]">404</h1>
    <VStack class="w-full -translate-y-10 items-center gap-16">
      <span class="text-accent text-3xl">Страница не найдена</span>
      <Link class="" to="/">Вернуться на главную</Link>
    </VStack>
  </VStack>
</template>
