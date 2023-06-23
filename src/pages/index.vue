<template>
  <div>
    <div ref="headerRef">
      <Header :wallet-connected="!!address" @wallet-connect="connectWallet()" />
    </div>
    <div class="overflow-auto" :style="contentMaxStyle">
      <div class="flex justify-center items-center" :style="contentMinStyle">
        <div class="relative pt-8 pb-28">
          <div class="container min-w-[80vw] lg:min-w-[40rem]">
            <h1 class="text-center mb-8">Schrödinger’s NFT</h1>
            <div v-if="nfts && nfts.length" style="width: 50%; display: inline-block">
              <NftCard
                :id="1"
                :src="nfts[0].image"
                :description="nfts[0].description"
                :name="nfts[0].name"
              />
            </div>
            <div id="borderBox" class="large-12 medium-12 small-12 cell">
              <DropFile class="mb-6" @uploaded="onFileUploaded" />
            </div>
            <div id="borderBox" class="large-12 medium-12 small-12 cell mb-8">
              <label>NFT ID</label>
              <div id="nftInput">
                <input
                  v-model="nftRef"
                  type="nft_id"
                  name="nft_id"
                  id="nftInput"
                  :class="$style.input"
                />
              </div>
            </div>
            <div>
              <label v-if="message != ''" class="absolute">{{ message }}</label>
              <!-- <label v-if="ipfsCid != ''" class="absolute" color="text-green">{{ ipfsCid }}</label> -->
            </div>
            <div class="flex gap-8">
              <div class="w-1/2" id="connect-btn">
                <Btn type="secondary" @click="uploadAndEncryptFile()">Encrypt and Upload</Btn>
              </div>
              <div class="w-1/2" id="connect-btn">
                <Btn type="primary" @click="phalaDownloadAndDecrypt()">Download</Btn>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="footer absolute left-0 right-0 bottom-0 py-4 lg:py-8 flex justify-center bg-bg">
      <div class="flex">
        Powered by
        <img
          src="/images/phala.png"
          class="ml-2 object-contain"
          alt="phala"
          width="82"
          height="16"
        />
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

const loading = ref<boolean>(false);
const loadingNfts = ref<boolean>(false);
const loadingMyNfts = ref<boolean>(false);
const file = ref<File>();
const uploadedFile = ref('');
const fileRef = ref<HTMLInputElement>();
const nftRef = ref();
const ipfsCid = ref<String>('');
const message = ref<String>('');
const setIntervalRef = ref();
const fileUuid = ref('');
const fileName = ref<string>('');

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

let apiKey = '69f4c5c6-3f61-42b7-8119-ff888f4717af';
let apiSecret = 'y99qC3fj&9HS';
let bucketUuid = '10268b28-684e-42a1-a037-5ce3663e7827';
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
  loading.value = true;
  [signer, provider] = await connectMetamaskWallet();
  [injector, address] = await connectPolkadotAccount();

  contract = new ethers.Contract(config.NFT_ADDRESS, contractAbi, provider);

  try {
    collectionInfo.value = await getCollectionInfo();
  } catch (e) {
    console.error(e);
    toast('Error' + e);

    loading.value = false;
    return;
  }

  await loadAllNFTs();
  loading.value = false;
}

async function setPhalaCid() {
  console.log('Setting phala cid ...');
  let cid = ipfsCid.value.toString();
  let nft_id = nftRef?.value;

  if (nft_id != undefined) {
    await setCid(injector, address as AddressOrPair, nft_id, cid, (msg: string) => {
      message.value = msg;
      ipfsCid.value = '';
    });
  }
}

async function loadAllNFTs() {
  loadingNfts.value = true;

  if (collectionInfo.value) {
    await fetchNFTs(collectionInfo.value.totalSupply);
  }
  loadingNfts.value = false;
}

async function loadMyNFTs() {
  loadingMyNfts.value = true;

  const balance = contract ? await contract.balanceOf(address.value) : null;

  await fetchNFTs(balance, address.value);
  loadingMyNfts.value = false;
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

async function fetchNFTs(balance: BigNumber | null | undefined, address = '') {
  nfts.value = [];
  if (!contract || !balance || balance.toNumber() === 0) {
    return;
  }

  for (let i = 0; i < balance.toBigInt(); i++) {
    try {
      const id = address
        ? await contract.tokenOfOwnerByIndex(address, i)
        : await contract.tokenByIndex(i);
      const url = await contract.tokenURI(id.toBigInt());
      const metadata = await fetch(url).then(response => {
        return response.json();
      });
      nfts.value.push({ id: i, ...metadata });
    } catch (e) {
      console.error(e);
    }
  }
}

async function uploadFiles(content: String) {
  ipfsCid.value = '';
  message.value = '';

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
          fileName: file?.value?.name,
          contentType: 'text/html',
        },
      }),
    });

    let putContenUrl = uploadResponse.data.data.files[0].url;
    let sessionUuid = uploadResponse.data.data.sessionUuid;
    fileUuid.value = uploadResponse.data.data.files[0].fileUuid;
    fileName.value = uploadResponse.data.data.files[0].fileName;

    console.log('File name: ', fileName.value);

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
        'Content-Type': 'application/json',
      },
      data: { directSync: true },
    });

    message.value = 'Uploading your file to IPFS...';
    let fileSynced = verifyFileSyncedToIPFS();
    console.log('Is file synced to IPFS: ', fileSynced);
  } catch (e) {
    console.log(e);
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
    message.value = '';
    clearInterval(setIntervalRef.value);
    console.log('File is synced successfully ...');
    setPhalaCid();
    return true;
  }

  console.log('Waiting for IPFS sync ...');
  return false;
}

async function uploadAndEncryptFile() {
  const encrypted = await encryptContent(uploadedFile.value);
  await uploadFiles(encrypted);
}

async function phalaDownloadAndDecrypt() {
  let nft_id = 1;
  const [signer, provider] = await connectMetamaskWallet();
  const [signature, hashedMessage] = await prepareSignData(signer);

  console.log('Siganture: ', signature, 'hashed message: ', hashedMessage);

  const decrypted = await downloadAndDecryptContent(signature, hashedMessage, nft_id);

  let response = decrypted.output.toJSON().ok.ok;

  if (response.includes('Invalid')) {
    message.value = response.toString();
  } else {
    writeFile(decrypted.output.toJSON().ok.ok);
  }
}

function writeFile(data: any) {
  var blob = new Blob([data], { type: 'text/plain;charset=utf-8' });
  // Save as is apparently now natively supported
  fs.saveAs(blob, fileName?.value);
}

function onFileUploaded(data: string) {
  uploadedFile.value = data;
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
