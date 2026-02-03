<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { reactive } from 'vue';
import { type } from '@tauri-apps/plugin-os';
import { useSystemStore } from '@/store';

import { Profile } from '@/components/Profile';

const appWindow = getCurrentWindow();
const isMacos = type() === 'macos';

const system = useSystemStore();

const s = reactive({
  isFocused: true,
  isMaximized: false,
});

const buttons = [
  {
    icon: () => '/titlebar/min.svg',
    action: () => appWindow.minimize(),
    enabled: !isMacos,
    hoverBg: () => (system.isDark ? '#FFFFFF1A' : '#0000001A'),
    color: () =>
      s.isFocused
        ? system.isDark
          ? '#FFFFFFFF'
          : '#000000FF'
        : system.isDark
          ? '#FFFFFF66'
          : '#00000066',
  },
  {
    icon: () => (s.isMaximized ? '/titlebar/unmax.svg' : '/titlebar/max.svg'),
    action: () => appWindow.toggleMaximize(),
    enabled: !isMacos,
    hoverBg: () => (system.isDark ? '#FFFFFF1A' : '#0000001A'),
    color: () =>
      s.isFocused
        ? system.isDark
          ? '#FFFFFFFF'
          : '#000000FF'
        : system.isDark
          ? '#FFFFFF66'
          : '#00000066',
  },
  {
    icon: () => '/titlebar/close.svg',
    action: () => appWindow.close(),
    enabled: !isMacos,
    hoverBg: () => '#E81123FF',
    color: () =>
      s.isFocused
        ? system.isDark
          ? '#FFFFFFFF'
          : '#000000FF'
        : system.isDark
          ? '#FFFFFF66'
          : '#00000066',
  },
];

appWindow.onFocusChanged((e) => {
  s.isFocused = e.payload;
});

appWindow.onResized(async () => {
  s.isMaximized = await appWindow.isMaximized();
});
</script>

<template>
  <div class="w-full h-9 relative flex items-start justify-end *:shrink-0">
    <div v-if="isMacos" class="w-20.5"></div>
    <div v-else class="w-12 h-full flex items-center justify-center">
      <img class="w-4 h-4" src="/app-icon.png" />
    </div>
    <div class="h-full">
      <Profile />
    </div>
    <div
      data-tauri-drag-region
      class="flex-1 h-full"
      @dblclick="appWindow.toggleMaximize"
    ></div>
    <template v-for="(item, index) in buttons" :key="index">
      <button
        v-if="item.enabled"
        class="w-11.25 h-7.25 flex items-center justify-center transition-colors hover:bg-(--hover-bg)"
        :style="`--hover-bg:${item.hoverBg()}`"
        @click="item.action"
      >
        <div
          class="bg-(--bg)"
          :style="`mask-image:url(&quot;${item.icon()}&quot;); --bg:${item.color()}`"
        ></div>
      </button>
    </template>
  </div>
</template>

<style scoped>
@reference 'tailwindcss';

button > div {
  @apply w-3.5 h-3.5 transition-colors duration-150;
  mask-repeat: no-repeat;
  mask-position: center;
  mask-size: 10px 10px;
}
</style>
