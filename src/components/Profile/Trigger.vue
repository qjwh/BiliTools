<script setup lang="ts">
import i18n from '@/i18n';
import { useAuthStore } from '@/store';
import { NAvatar, NSkeleton, NText } from 'naive-ui';
import { computed } from 'vue';

const auth = useAuthStore();

const name = computed(() => {
  if (auth.isAnonymous) {
    return i18n.global.t('auth.stat.login');
  } else if (auth.profile && auth.isValid) {
    return auth.profile.name;
  } else {
    return i18n.global.t('auth.stat.loading');
  }
});
</script>

<template>
  <div class="flex h-full gap-2 items-center justify-center">
    <NAvatar round :size="24">
      <img v-if="auth.profile" :src="auth.profile.avatar" />
      <NSkeleton v-else circle />
    </NAvatar>
    <NText>{{ name }}</NText>
  </div>
</template>
