<script setup lang="ts">
import { useMap } from "@/composables/useMap";
import MessagePopup from "./MessagePopup.vue";

const { convertToRatio } = useMap();

const allPoints = [
  convertToRatio(-42.187956, 146.496657),
  convertToRatio(33.958073, -114.34227),
  convertToRatio(43.981497, -59.324327),
  convertToRatio(59.793035, -43.941145),
];

interface Point {
  x: number;
  y: number;
}

function getPointStyles(point: Point) {
  return {
    left: `${point.x * 100}%`,
    top: `${point.y * 100}%`,
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
      <button
        v-for="(point, id) in allPoints"
        :key="id"
        class="group absolute -mt-5 -ml-5 size-8 cursor-pointer rounded-full"
        :style="getPointStyles(point! as Point)"
      >
        <div class="bg-accent relative mx-auto size-2 rounded-full" />
        <MessagePopup />
      </button>
    </div>
  </section>
</template>
