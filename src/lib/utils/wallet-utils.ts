import { ethers } from 'ethers';

export async function prepareSignData(signer: any): Promise<[number, string]> {
  const timestamp = new Date().getTime() - 60 * 1000;
  const control_message = `APILLON_REQUEST_MSG: ${timestamp}`;
  let signature = '';
  let hashedMessage = '';
  try {
    // Creates a signature to be verified in Phala
    signature = await signer.signMessage(control_message);
  } catch (err) {
    console.error(err);
  }

  return [timestamp, signature.replace('0x', '')];
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

  // console.log('Connecting wallet ...');
  // console.log('Signer ', signer);
  // console.log('Provider ', provider);
  return [signer, provider];
}
