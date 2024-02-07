import { BigNumber } from 'ethers';

export enum EncryptionState {
  IDLE = 1,
  WALLET_CONNECTED = 2,
  VERIFYING_OWNER = 3,
  DECRYPTED = 4,
  ERROR = 5,
}

declare global {
  /**
   * Window
   */
  interface Window {
    ethereum: any;
  }

  interface Nft {
    id: number;
    name: string;
    description: string;
    image: string;
  }
}
