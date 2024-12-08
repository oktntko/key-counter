import { createApp } from 'vue';
import App from '~/App.vue';
import router from '~/lib/router';
import '~/main.css';
import { Tauri } from '~/middleware/tauri';

window.addEventListener('DOMContentLoaded', async () => {
  console.log('DOMContentLoaded');
  await Tauri.start_listen();
});

createApp(App).use(createPinia()).use(router).mount('#app');
