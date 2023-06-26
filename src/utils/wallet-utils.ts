import { ethers } from 'ethers';
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export async function prepareSignData(signer: any) {
  const control_message = 'APILLON_REQUEST_MSG';
  let signature = '';
  let hashedMessage = '';
  try {
    // Creates a signature to be veirifed in Phala
    signature = await signer.signMessage(control_message);
    // Creates a hash of the message
    hashedMessage = ethers.utils.hashMessage(control_message);
  } catch (err) {
    console.error(err);
  }

  return [signature.replace('0x', ''), hashedMessage.replace('0x', '')];
}

export async function connectMetamaskWallet() {
  // A Web3Provider wraps a standard Web3 provider, which is
  // what MetaMask injects as window.ethereum into each page
  const provider = new ethers.providers.Web3Provider(window.ethereum);

  // MetaMask requires requesting permission to connect users accounts
  await provider.send('eth_requestAccounts', []);

  // The MetaMask plugin also allows signing transactions to
  // send ether and pay to change state within the blockchain.
  // For this, you need the account signer...
  const signer = provider.getSigner();

  console.log('Connecting wallet ...');
  console.log('Signer ', signer);
  console.log('Provider ', provider);
  return [signer, provider];
}

export async function connectPolkadotAccount() {
  try {
    await web3Enable('APILLON');
    const allAccounts = await web3Accounts();
    const polkadotAcc = allAccounts.filter(account => account.meta.source == 'polkadot-js')[0];
    const address = polkadotAcc.address;
    const injector = await web3FromAddress(address);
    return [injector, address];
  } catch (e) {
    console.warn(e);
    return [];
  }
}
