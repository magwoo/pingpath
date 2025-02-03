<script setup lang="ts">
import { useMap } from "@/composables/useMap";
import type { RefSymbol } from "@vue/reactivity";
import { ref, onMounted, useTemplateRef, watch } from "vue";

const { convertToRatio } = useMap();

const allPoints = [convertToRatio(-42.187956, 146.496657)];
</script>

<template>
  <section class="flex w-full justify-center">
    <div class="relative w-max">
      <img
        src="/map.png"
        class="pointer-events-none w-full max-w-[800px] select-none"
      />
      <button
        v-for="(point, id) in allPoints"
        :key="id"
        @click="console.log(123)"
        class="bg-accent absolute size-[6px] cursor-pointer rounded-full"
        :style="[[`left: ${point.x! * 100}%`], [`top: ${point.y! * 100}%;`]]"
      />
    </div>
  </section>
</template>
