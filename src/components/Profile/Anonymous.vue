<script setup lang="ts">
import {
  NText,
  NQrCode,
} from 'naive-ui';
import { onBeforeUnmount, onMounted } from 'vue';

onMounted(() => {});

onBeforeUnmount(() => {});

</script>

<template>
  <div class="flex px-16 py-12 gap-16 bg-[]">
    <div class="flex flex-col gap-4 items-center">
      <NText strong class="text-lg">{{ $t('auth.scan.hint') }}</NText>
      <NQrCode
        class="w-fit! h-fit! p-2! rounded-lg! border border-[#e3e5e7]"
        :size="144"
      />
      <NText class="whitespace-pre-wrap text-center">{{ $t('auth.scan.disclaimer') }}</NText>
    </div>
  </div>
</template>
