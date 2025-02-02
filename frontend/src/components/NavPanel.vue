<script setup>
import Link from "./Link.vue";
import Logo from "./svg/Logo.vue";
import Button from "./Button.vue";
import GitHub from "./svg/GitHub.vue";
</script>
<template>
  <section class="my-12 flex justify-between">
    <nav class="flex gap-10">
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
        <p>Pingpath</p>
      </Link>
      <Link href="/tarif">Тарифы</Link>
      <Link href="/profile">О нас</Link>
    </nav>
  </section>
</template>
