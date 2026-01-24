<script setup lang="ts">
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
import { Titlebar } from '@/components/Titlebar';
import { Sidebar } from '@/components/Sidebar';
import { useSystemStore } from '@/store';

import { computed } from 'vue';
import {
  darkTheme,
  enUS,
  lightTheme,
  NConfigProvider,
  NNotificationProvider,
  zhCN,
} from 'naive-ui';
import { RouterView } from 'vue-router';
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
    <NNotificationProvider>
      <main class="relative flex flex-row flex-1">
        <Sidebar />
        <RouterView v-slot="{ Component }">
          <div class="flex flex-col flex-1">
            <component :is="Component" />
          </div>
        </RouterView>
      </main>
    </NNotificationProvider>
  </NConfigProvider>
</template>
