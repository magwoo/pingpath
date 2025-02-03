<script setup lang="ts">
import { useMap } from "@/composables/useMap";
import type { RefSymbol } from "@vue/reactivity";
import { ref, onMounted, useTemplateRef, watch } from "vue";
import MessagePopup from "./MessagePopup.vue";

const { convertToRatio } = useMap();

const allPoints = [
  convertToRatio(-42.187956, 146.496657),
  convertToRatio(33.958073, -114.34227),
  convertToRatio(35.338997, 25.137026),
  convertToRatio(47.496952, 19.063225),
];
</script>

<template>
  <section class="flex w-full justify-center">
    <div class="relative w-max">
      <img
        src="/map.png"
        class="pointer-events-none relative -z-50 w-full max-w-[800px] select-none"
      />
      <button
        v-for="(point, id) in allPoints"
        :key="id"
        @click="console.log(123)"
        class="group absolute -mt-5 -ml-5 size-10 transform cursor-pointer rounded-full"
        :style="[[`left: ${point.x! * 100}%`], [`top: ${point.y! * 100}%;`]]"
      >
        <div
          class="bg-accent relative mx-auto flex size-2 justify-center rounded-full"
        >
          <MessagePopup
            class="invisible absolute -top-28 transition-all group-hover:visible"
          />
        </div>
      </button>
    </div>
  </section>
</template>
