
import { ethers } from "ethers";
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export async function prepareSignData(signer: ethers.JsonRpcSigner) {
    const control_message = "APILLON_REQUEST_MSG";
    let signature = '';
    let hashedMessage = '';
    try {
        // Creates a signature to be veirifed in Phala
        signature = await signer.signMessage(control_message);
        // Creates a hash of the message
        hashedMessage = ethers.hashMessage(control_message);
        
    } catch (err) {
        console.error(err);
    }

    return [signature.replace("0x", ""), hashedMessage.replace("0x", "")];
}

export async function connectMetamaskWallet() {
    // @ts-expect-error Window.ethereum not typed
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();

    return [signer, provider]
}

export async function connectPolkadotAccount() {
    await web3Enable('APILLON');
    const allAccounts = await web3Accounts();
    console.log("Found accounts ", allAccounts.length);

    const polkadotAcc = allAccounts.filter(
        account => account.meta.source == "polkadot-js"
    )[0];
    const address = polkadotAcc.address;

    console.log('Found wallet', polkadotAcc);
    console.log('Polkadot address ', address);

    const injector = await web3FromAddress(address);

    return [injector, address]
}