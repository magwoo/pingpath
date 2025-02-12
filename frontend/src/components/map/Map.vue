<script setup lang="ts">
import { useMap } from "@/composables/useMap";
import MessagePopup from "./MessagePopup.vue";

const { convertToRatio } = useMap();

const allPoints = [
  convertToRatio(-42.184649, 146.572013),
  convertToRatio(33.958073, -114.34227),
  convertToRatio(52.368125, 4.889498),
  convertToRatio(-12.365658, 49.187515),
];

interface Point {
  x: number;
  y: number;
}

function getPointStyles(point: Point) {
  return {
    left: `${point.x * 100}%`,
    top: `${point.y * 100}%`,
    transform: "translate(-50%, -50%)",
  };
}
</script>
<template>
  <section class="flex w-full justify-center">
    <div class="relative w-max">
      <img
        src="/map.png"
        class="pointer-events-none relative w-full max-w-[800px] select-none"
      />
      <div
        v-for="(point, id) in allPoints"
        :key="id"
        class="group absolute cursor-pointer"
        :style="getPointStyles(point! as Point)"
      >
        <button class="relative flex flex-col items-center">
          <div class="absolute -inset-3 size-4 opacity-0" />
          <div class="bg-accent size-2 rounded-full" />
          <MessagePopup />
        </button>
      </div>
    </div>
  </section>
</template>
