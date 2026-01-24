<script setup lang="ts">
import {
  NText,
  NQrCode,
  NForm,
  NFormItem,
  NInputGroup,
  NInput,
  NSelect,
  NInputOtp,
  NButton,
} from 'naive-ui';
import { IconLinkPlus } from '@tabler/icons-vue';
import { onBeforeUnmount, onMounted } from 'vue';

onMounted(() => {});

onBeforeUnmount(() => {});

const onlyAllowNumber = (value: string) => !value || /^\d+$/.test(value);
</script>

<template>
  <div class="flex px-16 py-12 gap-16 bg-[]">
    <div class="flex flex-col gap-4 items-center">
      <NText strong class="text-lg">扫描二维码登录</NText>
      <NQrCode
        class="w-fit! h-fit! p-2! rounded-lg! border border-[#e3e5e7]"
        :size="144"
      />
      <NText>扫码登录或扫码下载APP</NText>
    </div>
    <div class="flex flex-col gap-4 items-center">
      <NText strong class="text-lg">短信登录</NText>
      <NForm label-placement="left" class="h-40.5">
        <NFormItem label="手机号">
          <NInputGroup>
            <NInput
              class="min-w-57"
              :allow-input="onlyAllowNumber"
              placeholder="请输入手机号"
            />
            <NSelect placeholder="+86" />
          </NInputGroup>
        </NFormItem>
        <NFormItem label="验证码">
          <div class="flex gap-2">
            <NInputOtp :allow-input="onlyAllowNumber" />
            <NButton>获取验证码</NButton>
          </div>
        </NFormItem>
        <NFormItem>
          <NButton class="w-full!">
            <IconLinkPlus width="18" height="18" />
            <span class="ml-1.5">登录</span>
          </NButton>
        </NFormItem>
      </NForm>
      <!-- <NText>免责声明</NText> -->
    </div>
  </div>
</template>
