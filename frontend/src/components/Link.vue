<script setup lang="ts">
import { ref } from "vue";

interface Props {
  href?: string;
}

const { href = "/" } = defineProps<Props>();

const isActive = ref(window.location.pathname == href);

window.onhashchange = () => {
  isActive.value = window.location.pathname == href;
};
</script>

<template>
  <RouterLink
    :to="href"
    class="flex items-center gap-2 text-xl"
    :class="isActive ? 'text-accent' : 'text-neutral/75 hover:text-neutral'"
  >
    <slot name="before" :state="isActive"></slot>
    <slot></slot>
    <slot name="after" :state="isActive"></slot>
  </RouterLink>
</template>
