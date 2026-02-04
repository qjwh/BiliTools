import { createApp } from 'vue';
import App from './App.vue';
import '@/style.css';

import { bootstrap } from '@/utils/bootstrap';
import i18n from '@/i18n';
import store from '@/store';
import router from '@/router';

const app = createApp(App).use(i18n).use(store).use(router);

bootstrap({
  mount: () => app.mount('#app')
})
