<script setup lang="ts">
import Nav from "@/components/Nav.vue";
import HStack from "@/components/HStack.vue";
import VStack from "@/components/VStack.vue";
import Text from "@/components/Text.vue";
import Button from "@/components/Button.vue";
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
});

const type = {
  Full: "Полный",
};
</script>

<template>
  <Nav></Nav>
  <VStack class="gap-8">
    <Text size="lg" variant="neutral" class="opacity-50">Профиль</Text>
    <HStack class="justify-between">
      <HStack class="gap-12">
        <img
          v-if="data"
          :src="data.iconUrl"
          class="size-64"
          alt="profile-preview"
        />
        <div v-else class="bg-neutral/25 size-64" />
        <VStack class="gap-6 text-right">
          <Text size="sm" variant="neutral">Имя пользователя:</Text>
          <Text size="sm" variant="neutral">Всего адресов:</Text>
          <Text size="sm" variant="neutral">Профиль GitHub:</Text>
          <Text size="sm" variant="neutral">Тариф:</Text>
        </VStack>
        <VStack class="gap-6 text-left">
          <Text size="sm" variant="neutral" class="opacity-75" v-if="data">{{
            data.username
          }}</Text>
          <div v-else class="bg-neutral/10 h-7 w-45" />
          <Text size="sm" variant="neutral" class="opacity-75" v-if="data">{{
            data.addressAmount
          }}</Text>
          <div v-else class="bg-neutral/10 h-7 w-40" />
          <Text
            size="sm"
            variant="neutral"
            class="hover:underline opacity-75"
            v-if="data"
          >
            <a :href="`https://github.com/${data.username}`">{{
              `github.com/${data.username}`
            }}</a></Text
          >
          <div v-else class="bg-neutral/10 h-7 w-35" />
          <Text size="sm" variant="neutral" class="opacity-75" v-if="data">{{
            type[data.type]
          }}</Text>
          <div v-else class="bg-neutral/10 h-7 w-25" />
        </VStack>
      </HStack>
      <HStack class="h-max items-center gap-4">
        <Button variant="error" size="sm-inline">Удалить аккаунт</Button>
        <div class="bg-neutral/25 h-4 w-0.5" />
        <Button variant="error" size="sm-inline">Выйти</Button>
      </HStack>
    </HStack>
  </VStack>
</template>
