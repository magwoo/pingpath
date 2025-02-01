<script setup lang="ts">
import Counter from "@/components/Counter.vue";
</script>

<template>
  <main
    class="flex h-screen flex-col items-center justify-center gap-8 pb-[20vh]"
  >
    <div class="flex items-start gap-3">
      <h1 class="text-4xl text-stone-600">
        Vue + Vite + Tailwind + Typescript
      </h1>
      <span class="text-lg text-stone-500">template</span>
    </div>
    <Counter />
  </main>
</template>
