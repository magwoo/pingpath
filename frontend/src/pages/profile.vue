<script setup lang="ts">
import Nav from "@/components/Nav.vue";
import HStack from "@/components/HStack.vue";
import VStack from "@/components/VStack.vue";
import Text from "@/components/Text.vue";
import { onMounted, ref } from "vue";

const data = ref();

onMounted(async () => {
  await fetch("/api/auth/dev", { method: "POST" });
  const response = await fetch("/api/profile");

  data.value = JSON.parse(await response.json());
  console.log(data.value);
});
</script>

<template>
  <Nav></Nav>
  <VStack class="gap-8">
    <Text size="lg" variant="neutral">Профиль</Text>
    <HStack>
      <img v-if="data" :src="data.iconUrl" class="size-64" alt="" />
      <div v-else class="bg-neutral size-64"></div>
    </HStack>
  </VStack>
</template>
