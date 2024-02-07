<template>
  <div>
    <div ref="headerRef">
      <Header
        :wallet-connected="!!address || contract"
        :wallet-loading="walletLoading"
        @wallet-connect="connectWallet()"
      />
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
            <DropFile ref="dropFileRef" :state="encryptionState" @verify="verifyOwner" />
          </div>
          <div v-if="encryptionState === EncryptionState.DECRYPTED" class="flex gap-8 mt-8">
            <div class="w-1/2" id="connect-btn"></div>
            <div class="w-1/2" id="connect-btn">
              <Btn type="primary" :loading="loadingDownload" @click="phalaDownloadAndDecrypt()">
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
    <div class="footer absolute left-0 right-0 bottom-0 py-4 lg:py-8 flex justify-center bg-bg">
      <div class="flex gap-2 items-center">
        Powered by
        <img src="/images/phala.png" class="object-contain" alt="phala" width="82" height="16" />
        <img
          src="/images/moonbeam.svg"
          class="object-contain"
          alt="moonbeam"
          width="83"
          height="16"
        />
        <SvgInclude :name="SvgNames.Crust" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ethers } from 'ethers';

import * as fs from 'file-saver';
import { SvgNames } from '@/components/general/SvgInclude.vue';
import { EncryptionState } from '@/lib/types/general.types';

const walletLoading = ref<boolean>(false);
const loadingNfts = ref<boolean>(false);
const loadingDownload = ref<boolean>(false);
const dropFileRef = ref<HTMLElement>();
const encryptionState = ref<number>(EncryptionState.IDLE);

const ipfsCid = ref<String>('');
const fileName = ref<string>('');
const signature = ref<string>('');
const hashedMessage = ref<string>('');
const nftsLoaded = ref<boolean>(false);

const nfts = ref<Nft[]>([]);

let provider: any = null;
let contract: any = null;
let address: any = null;
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

onMounted(async () => {
  console.log('Phala demo init ...');

  [signer, provider] = await connectMetamaskWallet();
});

async function connectWallet() {
  walletLoading.value = true;
  [signer, provider] = await connectMetamaskWallet();

  contract = new ethers.Contract(NFT_ADDRESS, contractAbi, provider);

  await loadNfts(signer.getAddress());
  encryptionState.value = EncryptionState.WALLET_CONNECTED;

  walletLoading.value = false;
}

async function verifyOwner() {
  encryptionState.value = EncryptionState.VERIFYING_OWNER;

  const [timestamp, signature] = await prepareSignData(signer);

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

async function loadNfts(walletAddress?: string) {
  loadingNfts.value = true;
  nfts.value = [];

  try {
    const balance = await contract.balanceOf(walletAddress);

    for (let i = 0; i < balance.toNumber(); i++) {
      let nftId = walletAddress
        ? await contract.tokenOfOwnerByIndex(walletAddress, i)
        : await contract.tokenByIndex(walletAddress);

      const url = await contract.tokenURI(nftId.toNumber());
      const metadata = await fetch(url).then(response => {
        return response.json();
      });
      nfts.value.push({ id: nftId.toNumber(), ...metadata });
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
