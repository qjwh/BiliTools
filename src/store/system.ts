import { Effect, getCurrentWindow } from '@tauri-apps/api/window';
import { defineStore } from 'pinia';

import { Locale, Theme, WindowEffect } from '@backend';
import i18n from '@/i18n';

export const useSystemStore = defineStore('system', {
  state: (): {
    locale: Locale;
    theme: Theme;
    windowEffect: WindowEffect | null;
  } => ({
    locale: 'en-US',
    theme: 'light',
    windowEffect: null,
  }),
  getters: {
    isDark: (s) => s.theme === 'dark',
  },
  actions: {
    apply(locale: Locale, theme: Theme, windowEffect: WindowEffect | null) {
      this.locale = locale;
      this.theme = theme;
      this.windowEffect = windowEffect;
      document.documentElement.dataset['theme'] = theme;
    },
    async applyLocale(next: Locale) {
      if (this.locale === next) return;
      this.locale = next;
      document.documentElement.lang = next;
      i18n.global.locale.value = next;
    },
    async applyTheme(next: Theme) {
      if (this.theme === next) return;
      this.theme = next;
      document.documentElement.dataset['theme'] = next;
      await getCurrentWindow().setTheme(next);
    },
    async applyWindowEffect(next: WindowEffect | null) {
      if (this.windowEffect === next) return;
      this.windowEffect = next;
      await getCurrentWindow().setEffects({
        effects: next ? [next as Effect] : [],
      });
    },
  },
});
