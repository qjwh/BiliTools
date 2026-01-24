<script setup lang="ts">
import { useSystemStore } from '@/store';
import { NButton } from 'naive-ui';

const system = useSystemStore();

</script>

<template>
  <NButton
    @click="system.applyTheme(system.theme === 'dark' ? 'light' : 'dark')"
  >{{ system.theme }}</NButton>
</template>
