<script setup lang="ts">
import { Profile } from '@/store/auth';

const props = defineProps<{
  profile: Profile;
}>();
</script>

<template>
  <div>
    <img :src="props.profile.avatar" />
  </div>
</template>
