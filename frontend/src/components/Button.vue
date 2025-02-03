<script setup lang="ts">
const BUTTON_VARIANTS = {
  accent: "bg-accent text-white",
  neutral: "bg-neutral/10 text-neutral/75",
  transparent: "bg-transparent text-neutral/75",
};

const BUTTON_SIZES = {
  sm: "px-3 py-1",
  md: "px-8 py-2",
  lg: "px-8 py-2 text-xl",
};

type ButtonVariant = keyof typeof BUTTON_VARIANTS;
type ButtonSize = keyof typeof BUTTON_SIZES;

interface Props {
  readonly variant?: ButtonVariant;
  readonly size?: ButtonSize;
}

const { variant = "accent", size = "sm" } = defineProps<Props>();
</script>

<template>
  <button
    class="flex cursor-pointer items-center gap-2 font-medium hover:brightness-95"
    :class="[BUTTON_VARIANTS[variant], BUTTON_SIZES[size]]"
  >
    <slot name="before"></slot>
    <slot></slot>
    <slot name="after"></slot>
  </button>
</template>
