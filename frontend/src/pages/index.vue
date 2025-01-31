<script setup lang="ts">
import Counter from "@/components/Counter.vue";
</script>

<template>
  <main
    class="flex flex-col justify-center items-center gap-8 h-screen pb-[20vh]"
  >
    <div class="flex items-start gap-3">
      <h1 class="text-4xl text-stone-600">
        Vue + Vite + Tailwind + Typescript
      </h1>
      <span class="text-stone-500 text-lg">template</span>
    </div>
    <Counter />
  </main>
</template>
