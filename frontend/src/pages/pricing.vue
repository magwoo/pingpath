<script setup lang="ts">
import HStack from "@/components/HStack.vue";
import Nav from "@/components/Nav.vue";
import Text from "@/components/Text.vue";
import VStack from "@/components/VStack.vue";
</script>

<template>
  <Nav />
  <VStack class="mt-24 min-h-[101vh] gap-8">
    <Text variant="neutral" size="lg" class="font-bold opacity-50">
      Наши тарифы
    </Text>
    <HStack class="gap-16">
      <VStack class="gap-12">
        <Text variant="neutral" size="lg" class="opacity-75 select-none">
          &nbsp;
        </Text>
        <Text variant="neutral" size="md" class="opacity-75">
          Измерений в час
        </Text>
        <Text variant="neutral" size="md" class="opacity-75">
          Список ваших адресов
        </Text>
        <Text variant="neutral" size="md" class="opacity-75">
          Экспорт данных
        </Text>
      </VStack>
      <VStack class="w-64 gap-12">
        <Text variant="neutral" size="lg" class="opacity-75"> Пробный </Text>
        <Text variant="neutral" size="md" class="text-warn"> 50 </Text>
        <Text variant="neutral" size="md" class="text-error"> Нет </Text>
        <Text variant="neutral" size="md" class="text-error"> Нет </Text>
      </VStack>
      <VStack class="w-64 gap-12">
        <Text variant="neutral" size="lg" class="opacity-75"> Полный </Text>
        <Text variant="neutral" size="md" class="text-success"> 500 </Text>
        <Text variant="neutral" size="md" class="text-success"> Да </Text>
        <Text variant="neutral" size="md" class="text-success">csv, json</Text>
      </VStack>
    </HStack>
  </VStack>
</template>
