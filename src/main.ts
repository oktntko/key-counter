import { createApp } from 'vue';
import App from '~/App.vue';
import router from '~/lib/router';
import '~/main.css';

createApp(App).use(createPinia()).use(router).mount('#app');
