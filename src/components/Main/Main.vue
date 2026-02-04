<script setup lang="ts">
import { RouterView } from 'vue-router';
import { Sidebar } from '../Sidebar';
import { injectMessage } from '@/utils/message';
import { useMessage } from 'naive-ui';

injectMessage(useMessage())
</script>

<template>
  <main class="relative flex flex-row flex-1">
    <Sidebar />
    <RouterView v-slot="{ Component }">
      <div class="flex flex-col flex-1">
        <component :is="Component" />
      </div>
    </RouterView>
  </main>
</template>
