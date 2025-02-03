<script setup lang="ts">
import HStack from "@/components/HStack.vue";
import Input from "@/components/Input.vue";
import Nav from "@/components/Nav.vue";
import Text from "@/components/Text.vue";
import VStack from "@/components/VStack.vue";
import Map from "@/components/map/Map.vue";
import Button from "@/components/Button.vue";
import HistoryItem from "@/components/history/HistoryItem.vue";
</script>

<template>
  <Nav></Nav>
  <VStack class="mt-32 gap-32">
    <VStack class="w-max">
      <Text variant="accent" size="2xl" pattern>Измерение пинга</Text>
      <Text class="text-right" variant="neutral" size="md">со всего мира</Text>
    </VStack>
    <VStack class="gap-4">
      <Text variant="accent" size="md">Домен или ссылка</Text>
      <HStack class="gap-6">
        <Input placeholder="https://example.com" />
        <Button variant="accent" size="md">Измерить</Button>
        <Button variant="transparent" size="md">Очистить</Button>
      </HStack>
    </VStack>
    <HStack class="flex gap-8">
      <Map class="max-w-[800px]" />
      <VStack class="w-full items-end gap-6">
        <HistoryItem>Москва</HistoryItem>
        <HistoryItem>Москва</HistoryItem>
        <HistoryItem>Москва</HistoryItem>
        <HistoryItem>Москва</HistoryItem>
        <HistoryItem>Москва</HistoryItem>
      </VStack>
    </HStack>
  </VStack>
</template>
