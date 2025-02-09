<script setup lang="ts">
import Link from "./Link.vue";
import Logo from "./svg/Logo.vue";
import Button from "./Button.vue";
import GitHub from "./svg/GitHub.vue";
import HStack from "./HStack.vue";
import { onMounted, ref } from "vue";

interface Data {
  username: string;
  imgUrl: string | null;
}

const data = ref<Data>();
const isLogin = ref(false);

onMounted(async () => {
  try {
    const response = await fetch("/api/profile");
    if (!response.ok) throw new Error("Response wasn`t ok :(");
    data.value = JSON.parse(await response.json());
    console.log(data.value);
    isLogin.value = true;
  } catch (e) {
    console.log(e);
    isLogin.value = false;
  }
});

const type = {
  Full: "Полный",
};
</script>
<template>
  <nav class="my-12 flex justify-between">
    <HStack class="gap-10">
      <Link class="group">
        <template #before="isActive" v-slot="isActive">
          <Logo
            :class="
              isActive.state
                ? 'stroke-accent'
                : 'stroke-neutral/75 group-hover:stroke-neutral'
            "
          ></Logo>
        </template>
        Pingpath
      </Link>
      <Link href="/tarif">Тарифы</Link>
      <Link href="/about">О нас</Link>
    </HStack>
    <Button v-if="!isLogin">
      <template #before>
        <GitHub class="size-5" />
      </template>
      Войти
    </Button>
    <div v-else>
      <HStack class="gap-3">
        <Link href="/profile">
          {{ data!.username }}
          <template #after>
            <div class="size-8 overflow-hidden rounded-full">
              <img v-if="data!.imgUrl" :src="data!.imgUrl" />
              <div v-else class="bg-neutral/20 h-full w-full" />
            </div>
          </template>
        </Link>
      </HStack>
    </div>
  </nav>
</template>
