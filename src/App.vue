<script setup lang="ts">
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
import { Titlebar } from '@/components/Titlebar';
import { Main } from './components/Main';
import { useSystemStore } from '@/store';

import { computed } from 'vue';
import {
  darkTheme,
  enUS,
  lightTheme,
  NConfigProvider,
  NMessageProvider,
  zhCN,
} from 'naive-ui';
import { darkThemeOverrides, lightThemeOverrides } from '@/theme';

const system = useSystemStore();

const locale = computed(() => {
  switch (system.locale) {
    case 'zh-CN':
      return zhCN;
    case 'en-US':
      return enUS;
    default:
      return enUS;
  }
});

const theme = computed(() => (system.isDark ? darkTheme : lightTheme));
</script>

<template>
  <NConfigProvider
    :locale
    :theme
    :theme-overrides="system.isDark ? darkThemeOverrides : lightThemeOverrides"
    class="h-full flex flex-col"
  >
    <Titlebar />
    <NMessageProvider
      :max="6"
      :duration="0"
      container-class="top-9!"
      placement="top-right"
    >
      <Main />
    </NMessageProvider>
  </NConfigProvider>
</template>
