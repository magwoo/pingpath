<script setup lang="ts">
const LINK_VARIANTS = {
    "accent": "text-accent text-xl",
    "neutral": "text-neutral/75 text-xl"
}

interface Props {
    variant?: keyof typeof LINK_VARIANTS;
}

const { variant = "neutral" } = defineProps<Props>();
</script>

<template>
    <RouterLink to="/" class="flex gap-2" :class="LINK_VARIANTS[variant]">
        <slot name="before"></slot>
        <slot></slot>
        <slot name="after"></slot>
    </RouterLink>
</template>