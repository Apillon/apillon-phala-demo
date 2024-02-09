<template>
  <div class="flex gap-2 items-center">
    <span v-if="address" class="mr-4">{{ shortHash(address) }}</span>
    <Btn
      v-if="isConnected"
      v-bind="$attrs"
      type="secondary"
      class="min-w-[12rem] bg-bg-dark !text-blue"
      :loading="loading || isLoading"
      @click="disconnect()"
    >
      Disconnect
    </Btn>
    <Btn v-else-if="isConnected" v-bind="$attrs" :loading="loading || isLoading" @click="login()">
      Login
    </Btn>
    <Btn
      v-else
      v-bind="$attrs"
      type="blue"
      :loading="loading || isLoading"
      @click="showModalWallet()"
    >
      Connect wallet
    </Btn>
  </div>

  <modal :show="modalWalletVisible" class="max-w-xl mx-auto">
    <Wallet />
  </modal>
</template>

<script lang="ts" setup>
import { shortHash } from '~/lib/utils/strings';
import { useAccount, useConnect, useDisconnect, useWalletClient } from 'use-wagmi';

const { connect, connectors, isLoading } = useConnect();
const { data: walletClient, refetch } = useWalletClient();
const { address, isConnected } = useAccount({ onConnect: loginDelay });
const { disconnect } = useDisconnect();

const loading = ref<boolean>(false);
const modalWalletVisible = ref<boolean>(false);
const showModalWallet = () => {
  modalWalletVisible.value = false;
  setTimeout(() => (modalWalletVisible.value = true), 1);
};

async function login() {
  loading.value = true;
  try {
    if (!isConnected.value) {
      await connect({ connector: connectors.value[0] });
      modalWalletVisible.value = false;
    } else {
      await refetch();

      if (!walletClient.value) {
        await connect({ connector: connectors.value[0] });

        if (!walletClient.value) {
          // error('Could not connect with wallet');
          loading.value = false;
          return;
        }
      }
      modalWalletVisible.value = false;
    }
  } catch (e) {
    console.log(e);
  }
  loading.value = false;
}

function loginDelay() {
  setTimeout(() => login(), 100);
}
</script>
