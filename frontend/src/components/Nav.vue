<script setup>
import Link from "./Link.vue";
import Logo from "./svg/Logo.vue";
import Button from "./Button.vue";
import GitHub from "./svg/GitHub.vue";
import HStack from "./HStack.vue";
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
    <Button>
      <template #before>
        <GitHub class="size-5"></GitHub>
      </template>
      Войти
    </Button>
  </nav>
</template>
