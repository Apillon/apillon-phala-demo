<template>
  <div>
    <div ref="headerRef">
      <Header />
    </div>
    <div class="absolute top-40 left-1/2 w-full max-w-7xl -translate-x-1/2">
      <div v-if="nfts && nfts.length" class="absolute left-8 top-0 w-40">
        <h4>Your nfts:</h4>
        <NftCard v-for="nft in nfts" :key="nft.id" :nft="nft" />
      </div>
    </div>

    <div class="overflow-auto" :style="contentMaxStyle">
      <div class="flex justify-center items-center" :style="contentMinStyle">
        <div class="relative pb-24">
          <div class="container min-w-[80vw] lg:min-w-[40rem]">
            <h1 class="text-center my-10">Schrödinger’s NFT</h1>
            <DropFile :state="encryptionState" @verify="verifyOwner" />
          </div>
          <div v-if="encryptionState === EncryptionState.DECRYPTED" class="flex gap-8 mt-8">
            <div class="md:w-1/4"></div>
            <div class="w-1/2">
              <Btn
                type="primary"
                size="large"
                :loading="loadingDownload"
                @click="phalaDownloadAndDecrypt()"
              >
                Download
              </Btn>
            </div>
          </div>
          <br />
          <div v-if="ipfsCid" class="text-center">
            IPFS file:
            <Btn
              type="link"
              class="inline-block"
              :href="`https://ipfs.apillon.io/ipfs/${ipfsCid}`"
              target="_blank"
            >
              {{ ipfsCid }}
            </Btn>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAccount } from 'use-wagmi';

import * as fs from 'file-saver';
import { EncryptionState } from '@/lib/types/general.types';

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

const ipfsCid = ref<String>('');
const fileName = ref<string>('');
const signature = ref<string>('');
const hashedMessage = ref<string>('');
const nftsLoaded = ref<boolean>(false);

const nfts = ref<Nft[]>([]);

let signer: any = null;

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

async function onWalletConnected() {
  sleep(100);

  loadingNFT.value = true;
  encryptionState.value = EncryptionState.WALLET_CONNECTED;

  await initContract();
  await loadNfts();

  loadingNFT.value = false;
}

async function onWalletDisconnected() {
  encryptionState.value = EncryptionState.IDLE;
}

async function verifyOwner() {
  encryptionState.value = EncryptionState.VERIFYING_OWNER;

  const [timestamp, signature] = await prepareSignData();

  let isAuthenticated = false;

  try {
    for (let i = 0; i < nfts.value.length; i++) {
      // const isOwner = await verifyNftOwnership(nft_id, signature.value, hashedMessage.value);
      console.log(nfts.value[i]);
      const file = await decryptContent(nfts.value[i].id, timestamp, signature);
      console.log(file);
      if (file) {
        isAuthenticated = true;
        break;
      }
    }
  } catch (error) {
    console.log(error);
  }

  if (isAuthenticated) {
    encryptionState.value = EncryptionState.DECRYPTED;
  } else {
    encryptionState.value = EncryptionState.ERROR;
  }
}

async function loadNfts() {
  loadingNfts.value = true;
  nfts.value = [];

  const balance = contract.value ? await getBalance() : null;
  console.log(balance);

  if (!contract.value || !balance || balance.toString() === '0') {
    loadingNfts.value = false;
    return;
  }

  try {
    for (let i = 0; i < balance; i++) {
      const id = await getTokenOfOwner(i);
      const url = await getTokenUri(id);
      console.log(i, id, url);

      const metadata = await fetch(url).then(response => {
        return response.json();
      });
      nfts.value.push({ id: id, ...metadata });
      nftsLoaded.value = true;
    }
  } catch (e) {
    console.log(e);
  }

  loadingNfts.value = false;
}

async function phalaDownloadAndDecrypt() {
  loadingDownload.value = true;

  const decrypted = await downloadAndDecryptContent(
    signature.value,
    hashedMessage.value,
    nfts.value[0].id
  );

  console.log('Decrypted file: ', decrypted);

  try {
    writeFile(decrypted.output.toJSON().ok.ok);
  } catch (e) {
  } finally {
    loadingDownload.value = false;
  }
}

function writeFile(data: any) {
  var blob = new Blob([data], { type: 'text/plain;charset=utf-8' });
  // Save as is apparently now natively supported
  fs.saveAs(blob, fileName?.value);
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
