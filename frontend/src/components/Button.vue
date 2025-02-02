<script setup lang="ts">
interface Props {
    readonly variant?: 'accent' | 'neutral' | 'transparent',
    readonly size?: 'sm' | 'md' | 'lg',
    readonly icon?: string,
}

const { variant = 'accent', size = 'sm', icon } = defineProps<Props>();

const buttonClass = {
    "accent": "bg-accent text-white cursor-pointer",
    "neutral": "bg-neutral/10 cursor-pointer text-neutral/75",
    "transparent": "bg-transparent text-neutral/75"
}[variant];

const buttonSize = {
    "sm": "px-3 py-1",
    "md": "px-8 py-2",
    "lg": "px-8 py-2 text-xl",
}[size];
</script>

<template>
    <button class="flex gap-2 items-center" :class="[buttonClass, buttonSize]">
        <img v-if="icon" class="size-5 shrink-0" :src="icon" >
        <slot></slot>
    </button>
</template>