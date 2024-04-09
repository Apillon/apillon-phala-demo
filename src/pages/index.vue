<template>
  <div>
    <div ref="headerRef">
      <Header />
    </div>
    <div
      class="relative max-w-[90vw] mx-auto lg:absolute lg:top-40 lg:left-1/2 lg:w-full lg:max-w-7xl lg:-translate-x-1/2"
    >
      <div
        v-if="nfts && nfts.length"
        class="relative lg:absolute lg:left-8 lg:top-0 lg:w-40 lg:z-10 lg:pb-24"
      >
        <h4>Select NFT:</h4>
        <Nfts v-model="selectedNft" :nfts="nfts" />
      </div>
    </div>

    <div class="overflow-auto" :style="contentMaxStyle">
      <div class="flex justify-center items-center" :style="contentMinStyle">
        <div class="relative pb-24">
          <div class="container min-w-[80vw] lg:min-w-[40rem]">
            <h1 class="text-center my-10">Schrödinger’s NFT</h1>
            <DropFile :state="encryptionState" @verify="decrypt()" />
          </div>
          <div v-if="encryptionState === EncryptionState.DECRYPTED" class="flex gap-8 mt-8">
            <div class="md:w-1/4"></div>
            <div class="w-1/2">
              <Btn type="primary" size="large" :loading="loadingDownload" @click="downloadFile()">
                Download
              </Btn>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAccount } from 'use-wagmi';
import { EncryptionState } from '~/lib/types/general.types';

const { address } = useAccount({
  onConnect: onWalletConnected,
  onDisconnect: onWalletDisconnected,
});

const { contract, initContract, getBalance, getTokenOfOwner, getTokenUri, prepareSignData } =
  useContract();

const loadingNFT = ref<boolean>(false);
const loadingNfts = ref<boolean>(false);
const loadingDownload = ref<boolean>(false);
const encryptionState = ref<number>(EncryptionState.IDLE);

const nftsLoaded = ref<boolean>(false);

const nfts = ref<Nft[]>([]);
const selectedNft = ref<number>(0);
const decryptedContent = ref<any>(null);

/** Heading height */
const headerRef = ref<HTMLElement>();
const contentMinStyle = computed(() => {
  return {
    minHeight: `calc(100vh - ${headerRef.value?.clientHeight || 0}px)`,
  };
});
const contentMaxStyle = computed(() => {
  return {
    maxHeight: `calc(100vh - ${headerRef.value?.clientHeight || 0}px)`,
  };
});

watch(
  () => selectedNft.value,
  _ => {
    encryptionState.value = EncryptionState.WALLET_CONNECTED;
  }
);

async function onWalletConnected() {
  await sleep(200);

  loadingNFT.value = true;
  encryptionState.value = EncryptionState.WALLET_CONNECTED;

  await initContract();
  await loadNfts();

  loadingNFT.value = false;
}

async function onWalletDisconnected() {
  encryptionState.value = EncryptionState.IDLE;
  nfts.value = [];
}

async function decrypt() {
  if (selectedNft.value === 0) {
    toast('You need NFT to decrypt file!', { type: 'warning' });
    return;
  }
  encryptionState.value = EncryptionState.VERIFYING_OWNER;

  const [timestamp, signature] = await prepareSignData();

  try {
    decryptedContent.value = await decryptContent(selectedNft.value, timestamp, signature);

    if (decryptedContent.value) {
      encryptionState.value = EncryptionState.DECRYPTED;
    } else {
      toast('This NFT does`t have assigned CID.', { type: 'error' });
      encryptionState.value = EncryptionState.ERROR;
    }
  } catch (e) {
    console.log(e);
    toast('This NFT does`t have assigned CID.', { type: 'error' });
    encryptionState.value = EncryptionState.ERROR;
  }
}

async function loadNfts() {
  loadingNfts.value = true;
  nfts.value = [];

  try {
    const balance = contract.value ? await getBalance() : null;

    if (!contract.value || !balance || balance.toString() === '0') {
      loadingNfts.value = false;
      return;
    }
    for (let i = 0; i < balance; i++) {
      const id = await getTokenOfOwner(i);
      const url = await getTokenUri(id);

      const metadata = await fetch(url).then(response => {
        return response.json();
      });
      nfts.value.push({ id: id, ...metadata });
      nftsLoaded.value = true;
    }

    /** Select first NFT */
    if (nfts.value.length) {
      selectedNft.value = nfts.value[0].id;
    }
  } catch (e) {
    console.log(e);
    toast('Loading NFTs failed! Please check NFT_ADDRESS and CHAIN_ID in config!', {
      type: 'warning',
    });
  } finally {
    loadingNfts.value = false;
  }
}

async function downloadFile() {
  loadingDownload.value = true;

  try {
    saveFile(decryptedContent.value);
  } catch (e) {
    console.log(e);
  } finally {
    loadingDownload.value = false;
  }
}
</script>

<style lang="postcss" module>
.input {
  @apply w-full h-12 py-3 px-5 text-sm bg-bg-light border-1 border-bg-lighter rounded-none transition-all duration-300 outline-none placeholder:text-body;

  &:focus {
    @apply border-white;
  }
  &:hover {
    @apply border-body;
  }
}
</style>
