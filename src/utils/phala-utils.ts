import { waitReady } from '@polkadot/wasm-crypto';
import { Keyring, WsProvider, ApiPromise } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
// @ts-ignore
import { create, signCertificate, types } from '@phala/sdk';
import rust_vault_snippets from '../../data/rust_vault_snippets.json';
import type { AddressOrPair } from '@polkadot/api/types';

// Creates a Contract instance and connects to the phala network (PoC5 atm)
const initPhalaContract = async function () {
  await waitReady();

  // Move to configuration
  const wsProvider = 'wss://poc5.phala.network/ws';
  const pruntimeURL = ' https://poc5.phala.network/tee-api-1';
  const contractId = '0xf33090f06376a206c9e2ba9150a5d55e0d114df7317f1359a0259a6c773c2b3a';

  const provider = new WsProvider(wsProvider);
  const api = await ApiPromise.create({ provider, types });
  const metadata = rust_vault_snippets;

  const keyring = new Keyring({ type: 'sr25519' });
  const account = keyring.addFromUri('//Alice');

  const certificate = await signCertificate({ api, pair: account });
  const decoratedApi = (await create({ api, baseURL: pruntimeURL, contractId })).api;

  return [certificate, new ContractPromise(decoratedApi, metadata, contractId)];
};

export const setCid = async function (
  injector: any,
  sender: AddressOrPair,
  nft_id: number,
  cid: String,
  callback: any
) {
  const [certificate, contract] = await initPhalaContract();
  const { gasRequired, storageDeposit } = await contract.query.setCid(certificate, {}, nft_id, cid);

  const options = {
    gasLimit: gasRequired.refTime,
    storageDepositLimit: storageDeposit.isCharge ? storageDeposit.asCharge : null,
  };

  toast('Submitting transaction to Phala', { type: 'warning' });

  console.log('NFT ID: ', nft_id);
  await contract.tx
    .setCid(options, nft_id, cid)
    .signAndSend(sender, { signer: injector.signer }, async (result: any) => {
      if (result.status.isInBlock) {
        console.log('TRANSACTION IS IN BLOCK ...');
      } else if (result.status.isFinalized) {
        console.log('TRANSACTION IS FINALIZED ...');
        callback('Done! File successfully encrypted and uploaded to decentralized storage', true);
      } else {
        console.info('TRANSACTION STATE: ', result.status);
      }
    });

  return false;
};

export const verifyNftOwnership = async function (
  nft_id: number,
  signature: string,
  message: string
) {
  toast('Verifying NFT ownership', { type: 'warning' });

  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.verifyNftOwnership(
    certificate,
    {},
    signature,
    message,
    nft_id
  );

  console.log('Response: ', response);

  return response.output.toJSON().ok.ok;
};

export const verifyContractOwnership = async function () {
  toast('Verifying NFT ownership', { type: 'warning' });

  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.getCid(certificate, {}, 2);
  console.log('CID: ', response.output.toJSON().ok.ok);

  return response.output.toJSON().ok.ok;
};

export const getTestData = async function () {
  toast('Getting TEST DATA', { type: 'warning' });

  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.testGetData(certificate, {});
  console.log('Test data: ', response.output.toJSON().ok.ok);

  return response.output.toJSON().ok.ok;
};

export const getOwnerCaller = async function () {
  toast('Test owner', { type: 'warning' });

  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.testCaller(certificate, {});
  console.log('Test owner ', response.output.toJSON().ok.ok);

  return response.output.toJSON().ok.ok;
};

export const contractSetOwner = async function () {
  toast('Test set owner', { type: 'warning' });

  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.setOwner(certificate, {});
  console.log('Response set owner: ', response.output.toJSON().ok.ok);

  return response.output.toJSON().ok.ok;
};

export const getCid = async function (nft_id: number) {
  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.testCaller(certificate, {}, 1);
  let cid = response.output.toJSON().ok.ok;

  return cid;
};

export const encryptContent = async function (content: String) {
  toast('Encrypting content', { type: 'warning' });
  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.encryptContent(certificate, {}, content);
  let encryptedContent = response.output.toJSON().ok.ok;

  return encryptedContent;
};

export const downloadAndDecryptContent = async function (
  signature: String,
  hashedMessage: String,
  nft_id: number
) {
  toast('Decrypting and downloading file', { type: 'warning' });

  console.log('NFT_ID', nft_id);
  const [certificate, contract] = await initPhalaContract();

  const response = await contract.query.downloadAndDecrypt(
    certificate,
    {},
    signature,
    hashedMessage,
    nft_id
  );

  console.log('RESPONSE ', response);

  return response;
};
