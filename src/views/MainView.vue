<template>
  <div class="container">
    <h1>File Upload</h1>
    <div class="container">
      <div id="borderBox" class="large-12 medium-12 small-12 cell">
        <label>Upload file
          <input
            ref="fileRef"
            type="file"
            name="file"
            id="fileInput"
            @change="onChange"
          />
        </label>
        <label>NFT ID</label>
        <div id="nftInput">
          <input
            v-model="nftRef"
            type="nft_id"
            name="nft_id"
            id="nftInput"
          />
        </div>
      </div>
      <div> 
        <label v-if="message != ''" class="absolute">{{ message }}</label>
        <!-- <label v-if="ipfsCid != ''" class="absolute" color="text-green">{{ ipfsCid }}</label> -->
      </div>
    </div>
    <div id="connect-btn">
      <button class="submit-btn" @click="uploadAndEncryptFile()">Encrypt and Upload</button>
    </div>
    <div id="connect-btn">
      <button class="submit-btn" @click="phalaDownloadAndDecrypt()">Download</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ethers } from "ethers";
import { 
    connectMetamaskWallet, 
    connectPolkadotAccount, 
    prepareSignData,
} from "../utils/wallet-utils"
import { setCid, encryptContent, downloadAndDecryptContent, getCid } from "../utils/phala-utils"
import type { AddressOrPair } from "@polkadot/api/types";
import * as fs from 'file-saver';
import { Buffer } from 'buffer';
import axios from 'axios'

const file = ref<File>();
const uploadedFile = ref('');
const fileRef = ref<HTMLInputElement>();
const nftRef = ref();
const ipfsCid = ref<String>('');
const message = ref<String>('');
const setIntervalRef = ref();
const fileUuid = ref('');
const fileName = ref<string>('');

let apiKey = "69f4c5c6-3f61-42b7-8119-ff888f4717af";
let apiSecret = "y99qC3fj&9HS";
let bucketUuid = "10268b28-684e-42a1-a037-5ce3663e7827";
let creds = apiKey + ":" + apiSecret;
let credsB64Encoded = Buffer.from(creds).toString('base64');

async function setPhalaCid() {
  console.log("Setting phala cid ...");
  let cid = ipfsCid.value.toString();
  let nft_id = nftRef?.value;

  const [ signer, provider ] = await connectMetamaskWallet();
  const [ injector, address ] = await connectPolkadotAccount();

  if(nft_id != undefined) {
    await setCid(injector, address as AddressOrPair, nft_id, cid, (msg: string) => {
      message.value = msg;
      ipfsCid.value = "";
    });
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
        "Content-Type": "application/json; charset=utf-8",
      },
      data: JSON.stringify({files: {
          fileName: file?.value?.name,
          contentType: 'text/html',
        }
      }),
    });

    let putContenUrl = uploadResponse.data.data.files[0].url;
    let sessionUuid = uploadResponse.data.data.sessionUuid;
    fileUuid.value = uploadResponse.data.data.files[0].fileUuid;
    fileName.value = uploadResponse.data.data.files[0].fileName;

    console.log("File name: ", fileName.value);
    
    const putContentResponse = await axios({
      method: 'put', 
      url: putContenUrl,
      headers: {
        "Content-Type": 'txt',
      },
      data: content,
    });

    let endUploadUrl = `https://api-dev.apillon.io/storage/${bucketUuid}/upload/${sessionUuid}/end`;
    const responseEnd = await axios({
      method: 'post', 
      url: endUploadUrl,
      headers: {
        Authorization: `Basic ${credsB64Encoded}`,
        "Content-Type": 'application/json',
      },
      data: {directSync: true},
    });

    message.value = "Uploading your file to IPFS..."
    let fileSynced = verifyFileSyncedToIPFS();
    console.log("Is file synced to IPFS: ", fileSynced);

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
      "Content-Type": 'application/json',
    }
  });

  let status = response.data.data.fileStatus;
  if (status == 4) {
    let cid = response.data.data.file.CID
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
  const [signature, hashedMessage] = await prepareSignData(signer as ethers.JsonRpcSigner);

  console.log("Siganture: ", signature, "hashed message: ", hashedMessage);

  const decrypted = await downloadAndDecryptContent(
      signature, hashedMessage, nft_id);

  let response = decrypted.output.toJSON().ok.ok;

  if (response.includes('Invalid')) {
    message.value = response.toString();
  } else {
    writeFile(decrypted.output.toJSON().ok.ok);
  }
}

function onChange() {
    const files = fileRef.value?.files
        ? Array.from(fileRef.value.files)
        : [];

  if (files && files.length > 0 && files[0]) {
    file.value = files[0];
    parseUploadedFile(file.value);
  }
}

function writeFile (data: any) {
  var blob = new Blob([data], { type: 'text/plain;charset=utf-8' });
  // Save as is apparently now natively supported
  fs.saveAs(blob, fileName?.value);
}


function parseUploadedFile(file: File) {
  let reader = new FileReader();
  reader.onload = (ev: ProgressEvent<FileReader>) => {
    if (!!ev?.target?.result) {
        uploadedFile.value = ev.target.result.toString();
    }
  };
  reader.readAsText(file);
}

</script>