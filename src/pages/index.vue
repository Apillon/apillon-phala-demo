<template>
  <div>
    <div ref="headerRef">
      <Header
        :wallet-connected="!!address"
        :wallet-loading="walletLoading"
        @wallet-connect="connectWallet()"
      />
    </div>
    <div class="absolute top-40 left-1/2 w-full max-w-7xl -translate-x-1/2">
      <div v-if="nfts && nfts.length" class="absolute left-8 top-0 w-40">
        <h4>Your NFTs:</h4>
        <NftCard
          :id="1"
          :src="nfts[0].image"
          :description="nfts[0].description"
          :name="nfts[0].name"
        />
      </div>
      <div
        v-else-if="nfts && nfts.length == 0 && encryptionState == EncryptionState.WALLET_CONNECTED"
        class="absolute left-8 top-0 w-40"
      >
        <h4>You don't have any NFTs</h4>
      </div>
    </div>
    <div class="overflow-auto" :style="contentMaxStyle">
      <div class="flex justify-center items-center" :style="contentMinStyle">
        <div class="relative pb-24">
          <div class="container min-w-[80vw] lg:min-w-[40rem]">
            <h1 class="text-center my-10">Schrödinger’s NFT</h1>
            <DropFile
              ref="dropFileRef"
              :state="encryptionState"
              @uploaded="onFileUploaded"
              @verify="verifyOwner"
            />
          </div>
          <div v-if="encryptionState === EncryptionState.DECRYPTED" class="flex gap-8 mt-8">
            <div class="w-1/2" id="connect-btn">
              <Btn
                type="secondary"
                :loading="loadingUpload"
                :disabled="!uploadedFile || !fileData"
                @click="uploadAndEncryptFile()"
              >
                Upload
              </Btn>
            </div>
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
              :href="`https://ipfs2.apillon.io/ipfs/${ipfsCid}`"
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
import { ethers, BigNumber } from 'ethers';

import type { AddressOrPair } from '@polkadot/api/types';
import * as fs from 'file-saver';
import { Buffer } from 'buffer';
import axios from 'axios';
import { ref } from 'vue';
import { SvgNames } from '@/components/general/SvgInclude.vue';
import { EncryptionState } from '@/config/types';

const walletLoading = ref<boolean>(false);
const loadingNfts = ref<boolean>(false);
const loadingUpload = ref<boolean>(false);
const loadingDownload = ref<boolean>(false);
const dropFileRef = ref<HTMLElement>();
const encryptionState = ref<EncryptionState>(EncryptionState.IDLE);

const uploadedFile = ref<File>();
const fileData = ref<string>('');
const nftRef = ref();
const ifpsLoading = ref<boolean>(false);
const ipfsCid = ref<String>('');
const setIntervalRef = ref();
const fileUuid = ref('');
const fileName = ref<string>('');
const signature = ref<string>('');
const hashedMessage = ref<string>('');

const ENV_CONFIG = ref(import.meta.env);
const nfts = ref<Nft[]>([]);
const collectionInfo = ref<CollectionInfo>();

let provider: any = null;
let contract: any = null;
let injector: any = null;
let address: any = null;
let signer: any = null;

const config = {
  CHAIN_ID: ENV_CONFIG.value.VITE_CHAIN_ID,
  NFT_ADDRESS: ENV_CONFIG.value.VITE_NFT_ADDRESS,
};

let apiKey = '4ba34873-fdd2-4b0f-a383-05303bd0e615';
let apiSecret = '*AHM5#u0uoev';
let bucketUuid = 'f4833a8c-f010-4bc5-aa48-4b5223bd2ff5';
let creds = apiKey + ':' + apiSecret;
let credsB64Encoded = Buffer.from(creds).toString('base64');

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

async function connectWallet() {
  walletLoading.value = true;
  [signer, provider] = await connectMetamaskWallet();
  [injector, address] = await connectPolkadotAccount();

  contract = new ethers.Contract(config.NFT_ADDRESS, contractAbi, provider);

  try {
    collectionInfo.value = await getCollectionInfo();
    console.log(collectionInfo.value);
    encryptionState.value = EncryptionState.WALLET_CONNECTED;
  } catch (e) {
    console.error(e);
    toast('Error' + e, { type: 'error' });

    walletLoading.value = false;
    return;
  }

  await loadAllNFTs();
  walletLoading.value = false;
}

async function verifyOwner() {
  if (signature.value && hashedMessage.value) {
    return;
  }
  encryptionState.value = EncryptionState.VERIFYING_OWNER;

  const signData = await prepareSignData(signer);
  signature.value = signData[0];
  hashedMessage.value = signData[1];

  console.log('Hashed value: ', hashedMessage.value);
  console.log('Signature: ', signature.value);

  let nft_id = nfts.value[0].id;
  const isAuthenticated = await verifyNftOwnership(nft_id, signature.value, hashedMessage.value);
  if (isAuthenticated) {
    encryptionState.value = EncryptionState.DECRYPTED;
  } else {
    encryptionState.value = EncryptionState.ERROR;
  }
}

async function setPhalaCid() {
  toast('Data synced. Setting CID in Phala', { type: 'warning' });

  let cid = ipfsCid.value.toString();
  let nft_id = nfts.value[0].id;

  if (nft_id != undefined) {
    await setCid(
      injector,
      address as AddressOrPair,
      nft_id,
      cid,
      (msg: string, finished: boolean) => {
        toast(msg, { type: 'warning' });
        if (finished) {
          loadingUpload.value = false;
        }
      }
    );
  }
}

async function loadAllNFTs() {
  loadingNfts.value = true;

  if (collectionInfo.value) {
    await fetchNFTs(collectionInfo.value.totalSupply);
  }
  loadingNfts.value = false;
}

async function getCollectionInfo(): Promise<CollectionInfo> {
  if (!provider || !contract) return {} as CollectionInfo;
  return {
    name: await contract.name(),
    symbol: await contract.symbol(),
    maxSupply: await contract.maxSupply(),
    totalSupply: await contract.totalSupply(),
    soulbound: await contract.isSoulbound(),
    revokable: await contract.isRevokable(),
    drop: await contract.isDrop(),
    dropStart: await contract.dropStart(),
    reserve: await contract.reserve(),
    price: await contract.price(),
    royaltiesFees: await contract.royaltiesFees(),
    royaltiesAddress: await contract.royaltiesAddress(),
  };
}

async function fetchNFTs(balance: BigNumber | null | undefined) {
  nfts.value = [];
  if (!contract || !balance || balance.toNumber() === 0) {
    return;
  }

  console.log('Address:  ');
  for (let i = 0; i < balance.toBigInt(); i++) {
    try {
      let nftId = (await contract.tokenOfOwnerByIndex(await signer.getAddress(), i)).toNumber();

      const url = await contract.tokenURI(nftId);
      const metadata = await fetch(url).then(response => {
        return response.json();
      });
      nfts.value.push({ id: nftId, ...metadata });
    } catch (e) {
      console.error(e);
      encryptionState.value = EncryptionState.IDLE;
    }
  }
}

async function uploadFiles(content: String) {
  ipfsCid.value = '';

  try {
    const uploadResponse = await axios({
      method: 'post',
      url: `https://api-dev.apillon.io/storage/${bucketUuid}/upload`,
      headers: {
        Authorization: `Basic ${credsB64Encoded}`,
        'Content-Type': 'application/json; charset=utf-8',
      },
      data: JSON.stringify({
        files: {
          fileName: uploadedFile?.value?.name,
          contentType: 'text/html',
        },
      }),
    });

    let putContenUrl = uploadResponse.data.data.files[0].url;
    let sessionUuid = uploadResponse.data.data.sessionUuid;
    fileUuid.value = uploadResponse.data.data.files[0].fileUuid;
    fileName.value = uploadResponse.data.data.files[0].fileName;

    await axios({
      method: 'put',
      url: putContenUrl,
      headers: {
        'Content-Type': 'txt',
      },
      data: content,
    });

    let endUploadUrl = `https://api-dev.apillon.io/storage/${bucketUuid}/upload/${sessionUuid}/end`;
    await axios({
      method: 'post',
      url: endUploadUrl,
      headers: {
        Authorization: `Basic ${credsB64Encoded}`,
        'Content-Type': 'applicaution/json',
      },
      data: { directSync: true },
    });

    toast('Uploading file to IPFS. This can take several seconds. ', { type: 'warning' });
    let fileSynced = verifyFileSyncedToIPFS();
    console.log('Is file synced to IPFS: ', fileSynced);
  } catch (e) {
    console.log(e);
    toast('Error: ' + e, { type: 'error' });
  }
}

function verifyFileSyncedToIPFS() {
  if (ipfsCid.value == '') {
    setIntervalRef.value = setInterval(
      function () {
        return checkFileStatus();
      }.bind(verifyFileSyncedToIPFS),
      2000
    );
  }
}

async function checkFileStatus() {
  let fileUrl = `https://api-dev.apillon.io/storage/${bucketUuid}/file/${fileUuid.value}/detail`;
  const response = await axios({
    method: 'get',
    url: fileUrl,
    headers: {
      Authorization: `Basic ${credsB64Encoded}`,
      'Content-Type': 'application/json',
    },
  });

  let status = response.data.data.fileStatus;
  if (status == 4) {
    let cid = response.data.data.file.CID;
    ipfsCid.value = cid;
    console.log('CID ', cid);
    clearInterval(setIntervalRef.value);
    setPhalaCid();

    return true;
  }

  return false;
}

async function uploadAndEncryptFile() {
  loadingUpload.value = true;
  try {
    const encrypted = await encryptContent(fileData.value);
    await uploadFiles(encrypted);
  } catch (error) {
    toast('Error: ' + error, { type: 'error' });
  }
}

async function phalaDownloadAndDecrypt() {
  loadingDownload.value = true;
  const decrypted = await downloadAndDecryptContent(
    signature.value,
    hashedMessage.value,
    nfts.value[0].id
  );

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

function onFileUploaded(file: File) {
  uploadedFile.value = file;
  parseUploadedFile(file);
}

function parseUploadedFile(file?: File) {
  if (!file) {
    return;
  }
  let reader = new FileReader();
  reader.onload = (ev: ProgressEvent<FileReader>) => {
    if (!!ev?.target?.result) {
      fileData.value = ev.target.result.toString();
    } else {
      console.warn('File is empty or is not valid!');
    }
  };
  reader.readAsText(file);
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
