import './assets/css/main.css';
import 'vue3-toastify/dist/index.css';

import { createApp } from 'vue';
import { createHead } from '@vueuse/head';
import { createRouter, createWebHistory } from 'vue-router';
import App from './App.vue';
import routes from '~pages';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';

const app = createApp(App);

const router = createRouter({
  routes,
  history: createWebHistory(),
});
app.use(router);

app.use(createHead());

/** Messages: vue3-toastify */
app.use(Vue3Toastify, {
  autoClose: 3000,
  position: 'bottom-right',
} as ToastContainerOptions);

app.mount('#app');
