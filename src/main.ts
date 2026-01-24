import { createApp } from 'vue';
import App from './App.vue';

import store, { useSystemStore } from '@/store';
import router from '@/router';
import { commands } from '@backend';
import '@/style.css';
import { init, result } from '@/utils';

async function bootstrap() {
  const config = await commands.resolveConfig();

  const app = createApp(App).use(store).use(router);

  const system = useSystemStore();
  system.apply(config.locale, config.theme, config.window_effect);

  app.mount('#app');

  await init.init();
}

result.setHook();
bootstrap();
