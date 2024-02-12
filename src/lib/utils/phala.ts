import { Buffer } from 'buffer';
import { saveAs } from 'file-saver';
import { OnChainRegistry, signCertificate, PinkContractPromise, types } from '@phala/sdk';
import { Keyring, WsProvider, ApiPromise } from '@polkadot/api';
import rust_vault_snippets from '~/../data/rust_vault_snippets.json';

let account: any = null;
let phalaContract: any = null;
let phalaCertificate: any = null;

// Creates a Contract instance and connects to the phala network (PoC5 atm)
const initPhalaContract = async function () {
  if (!account || !phalaCertificate || !phalaContract) {
    const provider = new WsProvider(PHALA_RPC);

    // @ts-ignore
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

  const output = response.output.toJSON();

  if ('err' in output) {
    throw new Error(`Failed to decrypt content: ${output.err}`);
  } else if (!output.ok) {
    throw new Error(`Failed to decrypt content: unknown error`);
  } else if ('err' in output.ok) {
    throw new Error(`Failed to decrypt content: ${output.ok.err}`);
  }
  return output.ok.ok;
}

export function saveFile(content: string) {
  let [type, contentData] = content.split(',');
  let data = Buffer.from(contentData, 'base64').toString();

  const fileExt = detectType(type);

  var blob = new Blob([data]);
  saveAs(blob, `decrypted_file.${fileExt}`);
}

function detectType(type: any) {
  let fileType = '';

  try {
    fileType = type.split(';')[0].split('/')[1].split('+')[0];
    if (fileType) {
      return fileType;
    }
  } catch (error) {
    fileType = 'txt';
  }

  const extensions = {
    xls: 'application/vnd.ms-excel',
    ppt: 'application/vnd.ms-powerpoint',
    doc: 'application/msword',
    xml: 'text/xml',
    mpeg: 'audio/mpeg',
    mpg: 'audio/mpeg',
    svg: 'image/svg+xml',
    png: 'image/png',
    jpg: 'image/jpeg',
    csv: 'text/csv',
    txt: 'text/plain',
  };
  Object.entries(extensions).forEach(([ext, name]) => {
    if (type.includes(name)) {
      fileType = ext;
      return;
    }
  });
  return fileType;
}
