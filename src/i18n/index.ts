import { createI18n } from "vue-i18n";

import { zhCN } from "./locales/zh-CN";

// Type-define 'zh-CN' as the master schema for the resource
type MessageSchema = typeof zhCN

export default createI18n<[MessageSchema], 'zh-CN' | 'en-US'>({
  legacy: false,
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    // TODO
    'en-US': zhCN,
  }
})
