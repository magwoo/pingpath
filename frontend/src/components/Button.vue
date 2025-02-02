<script setup lang="ts">
const BUTTON_VARIANTS = {
    "accent": "bg-accent text-white cursor-pointer",
    "neutral": "bg-neutral/10 cursor-pointer text-neutral/75",
    "transparent": "bg-transparent text-neutral/75"
}

const BUTTON_SIZES = {
    "small": "px-3 py-1",
    "medium": "px-8 py-2",
    "large": "px-8 py-2 text-xl",
}

type ButtonVariant = keyof typeof BUTTON_VARIANTS;
type ButtonSize = keyof typeof BUTTON_SIZES;

interface Props {
    readonly variant?: ButtonVariant,
    readonly size?: ButtonSize,
    readonly icon?: string,
}

const { variant = 'accent', size = 'small', icon } = defineProps<Props>();

</script>

<template>
    <button class="flex gap-2 items-center" :class="[BUTTON_VARIANTS[variant], BUTTON_SIZES[size]]">
        <img v-if="icon" class="size-5 shrink-0" :src="icon" >
        <slot></slot>
    </button>
</template>