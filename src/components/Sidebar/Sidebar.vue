<script setup lang="ts">
import { computed } from 'vue';
import { MENU } from './menu';
import { useRouter } from 'vue-router';

const router = useRouter();

const activeKey = computed(
  () => MENU.find((item) => item.path === router.currentRoute.value.path)?.key,
);
</script>

<template>
  <aside class="w-12 h-full flex flex-col">
    <template v-for="item in MENU" :key="item.key">
      <div
        class="item w-12 h-12 flex items-center justify-center hover:cursor-pointer"
        :class="{
          active: activeKey === item.key,
          'mt-auto mb-2.5': item.gapped,
        }"
        @click="router.push(item.path)"
      >
        <span class="absolute left-0 w-0.75 h-6 rounded-full"></span>
        <component :is="item.icon" size="24" />
      </div>
    </template>
  </aside>
</template>

<style scoped>
@reference 'tailwindcss';

.item {
  & > svg,
  & > span {
    @apply transition-[opacity,background-color,color] duration-150;
  }
  & > svg {
    @apply text-(--text-color) opacity-40;
  }
  & > span {
    @apply bg-(--brand-color) opacity-0;
  }
  &:hover > svg {
    @apply opacity-100;
  }
  &.active > :is(svg, span) {
    @apply text-(--brand-color) opacity-100;
  }
}
</style>
