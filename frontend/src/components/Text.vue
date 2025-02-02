<script setup lang="ts">
const TEXT_SIZES = {
    "xs": "text-md",
    "sm": "text-xl",
    "md": "text-3xl",
    "xl": "text-6xl font-bold",
};

const TEXT_VARIANTS = {
    "accent": "text-accent",
    "neutral": "text-neutral",
};

interface Props {
    size?: keyof typeof TEXT_SIZES;
    variant?: keyof typeof TEXT_VARIANTS;
    pattern?: boolean;
}

const { size="md", variant="accent" } = defineProps<Props>();
</script>

<template>
    <span class="relative" :class="[TEXT_SIZES[size], TEXT_VARIANTS[variant]]">
        <slot></slot>
        <div v-if="pattern" class="absolute top-0 bottom-0 -left-4 pointer-events-none">
            <img src="/patterns/text-pattern.svg" >
        </div>
    </span>
</template>