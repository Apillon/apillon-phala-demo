<script lang="ts" setup>
import { useConnect } from 'use-wagmi';
import WalletSVG from '~/assets/img/wallet.svg';
import MetaMaskSVG from '~/assets/icons/metaMask.svg';
import CoinbaseWalletSVG from '~/assets/icons/coinbaseWallet.svg';

const { connect, connectors, pendingConnector } = useConnect();

const walletIcons = {
  metaMask: MetaMaskSVG,
  coinbaseWallet: CoinbaseWalletSVG,
};
</script>

<template>
  <div class="max-w-md w-full md:px-6 my-12 mx-auto">
    <img :src="WalletSVG" class="mx-auto" width="241" height="240" alt="wallet" />

    <div class="my-8 text-center">
      <h3 class="mb-6">Choose wallet</h3>
      <p>
        To join this NFT airdrop, you need to connect your EVM compatible wallet. This step is
        crucial for securely receiving and managing the airdropped NFTs.
      </p>
    </div>

    <div class="flex flex-col gap-4">
      <Btn
        v-for="(connector, key) in connectors"
        :key="key"
        type="secondary"
        :loading="connector.id === pendingConnector?.id"
        :disabled="!connector.ready"
        @click="connect({ connector })"
      >
        <span class="inline-flex gap-2 items-center">
          <img :src="walletIcons[connector.id]" class="text-xl" filled />
          <span>{{ connector.name }}</span>
        </span>
      </Btn>
    </div>
  </div>
</template>
