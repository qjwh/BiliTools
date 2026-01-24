<script setup lang="ts">
import { useAuthStore } from '@/store';
import { NAvatar, NSkeleton, NText } from 'naive-ui';
import { computed } from 'vue';

const auth = useAuthStore();

const name = computed(() => {
  if (auth.isAnonymous) {
    return '登录';
  } else if (auth.profile && auth.isValid) {
    return auth.profile.name;
  } else {
    return '加载中';
  }
});
</script>

<template>
  <div class="flex h-full ml-2 gap-2 items-center justify-center">
    <NAvatar round :size="24">
      <img v-if="auth.profile" :src="auth.profile.avatar" />
      <NSkeleton v-else circle />
    </NAvatar>
    <NText>{{ name }}</NText>
  </div>
</template>
