import { Buffer } from 'buffer';
import { OnChainRegistry, signCertificate, PinkContractPromise, types } from '@phala/sdk';
import { waitReady } from '@polkadot/wasm-crypto';
import { Keyring, WsProvider, ApiPromise } from '@polkadot/api';
import type { AddressOrPair } from '@polkadot/api/types';
import rust_vault_snippets from '~/../data/rust_vault_snippets.json';

let account: any = null;
let phalaContract: any = null;
let phalaCertificate: any = null;

// Creates a Contract instance and connects to the phala network (PoC5 atm)
const initPhalaContract = async function () {
  if (!account || !phalaCertificate || !phalaContract) {
    await waitReady();

    const provider = new WsProvider(PHALA_RPC);
    const api = await ApiPromise.create({ provider, types });
    const phatRegistry = await OnChainRegistry.create(api);

    const keyring = new Keyring({ type: 'sr25519' });
    account = keyring.addFromUri('//Alice');

    phalaCertificate = await signCertificate({ api, pair: account });

    const contractKey = await phatRegistry.getContractKeyOrFail(CONTRACT_ADDRESS);
    phalaContract = new PinkContractPromise(
      api,
      phatRegistry,
      rust_vault_snippets,
      CONTRACT_ADDRESS,
      contractKey
    );
  }

  return [account, phalaCertificate, phalaContract];
};

export const setCid = async (
  injector: any,
  sender: AddressOrPair,
  nft_id: number,
  cid: String,
  callback: any
) => {
  const [account, certificate, contract] = await initPhalaContract();
  const { gasRequired, storageDeposit } = await contract.query.setCid(certificate, {}, nft_id, cid);

  console.error(storageDeposit);

  const options = {
    gasLimit: gasRequired.refTime,
    storageDepositLimit: storageDeposit.isCharge ? storageDeposit.asCharge : null,
  };

  toast('Submitting transaction to Phala', { type: 'warning' });

  console.log('NFT ID: ', nft_id);
  const response = await contract.tx
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

  console.log('setCid: ', response);

  return false;
};

export async function decryptContent(nftId: number, timestamp: number, signature: string) {
  const [account, certificate, contract] = await initPhalaContract();

  toast('Decrypting and downloading file', { type: 'info' });
  let response = null;
  try {
    response = await contract.query.downloadAndDecrypt(
      account.address,
      { cert: certificate },
      nftId,
      timestamp,
      signature
    );
  } catch (error) {
    console.log(error);
    return null;
  }
  const result = response.result.toJSON();
  console.log('result', result);

  if ('err' in result) {
    throw new Error(`Failed to execute contract: ${result}`);
  }

  const output = response.output.toJSON();
  console.log('output', output);
  if ('err' in output) {
    throw new Error(`Failed to decrypt content: ${output.err}`);
  } else if (!output.ok) {
    throw new Error(`Failed to decrypt content: unknown error`);
  } else if ('err' in output.ok) {
    throw new Error(`Failed to decrypt content: ${output.ok.err}`);
  }
  // console.log('result', outputOk)
  let contentParts = output.ok.ok.split(',');
  let content = new Buffer.from(contentParts[1], 'base64').toString();
  console.log(`content: "${content}"`);
  console.log('content type:', contentParts[0]);
  return content;
}
