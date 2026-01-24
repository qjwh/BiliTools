<script setup lang="ts">
import { IconSearch } from '@tabler/icons-vue';
import { NButton, NInput, NInputGroup } from 'naive-ui';
import { ref } from 'vue';

const input = ref<string>();

function search() {
  if (!input.value?.trim().length) return;

  console.log(input.value);
}
</script>

<template>
  <div class="h-12 flex items-center">
    <NInputGroup class="justify-center">
      <div class="w-lg">
        <NInput v-model:value="input" round clearable @keydown.enter="search" />
      </div>
      <NButton round @click="search">
        <IconSearch width="20" height="20" />
      </NButton>
    </NInputGroup>
  </div>
</template>
