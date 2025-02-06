<script setup lang="ts">
import Warning from "../svg/Warning.vue";
import Check from "../svg/Check.vue";
import AssignmentLate from "../svg/AssignmentLate.vue";
import HStack from "../HStack.vue";

const NOTIFICATION_VARIANTS = {
  success: "bg-success",
  warn: "bg-warn",
  error: "bg-error",
};

interface Props {
  variant: "warn" | "success" | "error";
}

const { variant } = defineProps<Props>();
</script>

<template>
  <Transition enter-from-class="opacity-0">
    <HStack
      class="relative w-max max-w-xs items-center gap-2 px-4 py-2 font-medium transition-transform duration-300 select-none"
      :class="NOTIFICATION_VARIANTS[variant]"
    >
      <AssignmentLate v-if="variant == 'warn'" />
      <Check v-if="variant == 'success'" />
      <Warning v-if="variant == 'error'" />
      <span class="text-white"><slot></slot></span>
    </HStack>
  </Transition>
</template>
