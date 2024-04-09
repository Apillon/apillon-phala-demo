import './assets/css/main.css';
import 'vue3-toastify/dist/index.css';

import { createApp } from 'vue';
import { createHead } from '@vueuse/head';
import { createRouter, createWebHistory } from 'vue-router';
import App from './App.vue';
import routes from '~pages';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';

import { UseWagmiPlugin, createConfig } from 'use-wagmi';
import { MetaMaskConnector } from 'use-wagmi/connectors/metaMask';
import { CoinbaseWalletConnector } from 'use-wagmi/connectors/coinbaseWallet';
import { createPublicClient, http } from 'viem';

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

/**
 * Wagmi config
 */
const usedChain = chains.find(item => item.id === Number(BigInt(NFT_CHAIN_ID))) || chains[0];
const config = createConfig({
  autoConnect: true,
  connectors: [
    new MetaMaskConnector({
      chains,
      options: {
        UNSTABLE_shimOnConnectSelectAccount: true,
      },
    }),
    markRaw(
      new CoinbaseWalletConnector({
        chains,
        options: {
          appName: 'Whitelist claim',
        },
      })
    ),
  ],
  publicClient: createPublicClient({
    chain: usedChain,
    transport: http(),
  }),
});
app.use(UseWagmiPlugin, config);

app.mount('#app');
