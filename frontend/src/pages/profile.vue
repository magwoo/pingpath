<script setup lang="ts">
import Nav from "@/components/Nav.vue";
import HStack from "@/components/HStack.vue";
import VStack from "@/components/VStack.vue";
import Text from "@/components/Text.vue";
import { onMounted, ref } from "vue";

interface ProfileInfo {
  username: string;
  iconUrl: string;
  addressAmount: number;
  type: "Full";
}

const data = ref<ProfileInfo>();

onMounted(async () => {
  await fetch("/api/auth/dev", { method: "POST" });
  const response = await fetch("/api/profile");

  data.value = JSON.parse(await response.json());
  console.log(data.value);
});

const type = {
  Full: "Полный",
};
</script>

<template>
  <Nav></Nav>
  <VStack class="gap-8">
    <Text size="lg" variant="neutral/50">Профиль</Text>
    <HStack class="gap-12">
      <img v-if="data" :src="data.iconUrl" class="size-64" alt="" />
      <div v-else class="bg-neutral size-64"></div>
      <VStack class="gap-6 text-right">
        <Text size="md" variant="accent">Имя пользователя:</Text>
        <Text size="md" variant="accent">Всего адресов:</Text>
        <Text size="md" variant="accent">Профиль GitHub:</Text>
        <Text size="md" variant="accent">Тариф:</Text>
      </VStack>
      <VStack class="gap-6 text-left">
        <Text size="md" variant="neutral/75" v-if="data">{{
          data.username
        }}</Text>
        <Text size="md" variant="neutral/75" v-if="data">{{
          data.addressAmount
        }}</Text>
        <Text
          size="md"
          variant="neutral/75"
          class="hover:underline"
          v-if="data"
        >
          <a :href="`github/${data.username} `">{{
            `github/${data.username} `
          }}</a></Text
        >
        <Text size="md" variant="neutral/75" v-if="data">{{
          type[data.type]
        }}</Text>
      </VStack>
    </HStack>
  </VStack>
</template>
