<script setup lang="ts">
import { useAuthStore } from '@/store';
import { NPopover, NSpin } from 'naive-ui';

import Trigger from './Trigger.vue';
import Anonymous from './Anonymous.vue';
import Valid from './Valid.vue';

const auth = useAuthStore();
</script>

<template>
  <NPopover :show-arrow="false" placement="bottom-start">
    <template #trigger>
      <Trigger />
    </template>
    <NSpin v-if="auth.isChecking" />
    <Anonymous v-if="auth.isAnonymous" />
    <Valid v-if="auth.isValid && auth.profile" :profile="auth.profile" />
  </NPopover>
</template>
