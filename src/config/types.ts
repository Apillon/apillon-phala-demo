import { BigNumber } from 'ethers';

export enum EncryptionState {
  IDLE = 1,
  WALLET_CONNECTED = 2,
  VERIFYING_OWNER = 3,
  DECRYPTED = 4,
  ERROR = 5,
}

declare global {
  interface Nft {
    name: string;
    description: string;
    image: string;
  }

  interface CollectionInfo {
    drop: Boolean;
    dropStart: BigNumber;
    maxSupply: BigNumber;
    name: String;
    price: BigNumber;
    reserve: BigNumber;
    revokable: Boolean;
    royaltiesFees: BigNumber;
    royaltiesAddress: String;
    soulbound: Boolean;
    symbol: String;
    totalSupply: BigNumber;
  }
}
