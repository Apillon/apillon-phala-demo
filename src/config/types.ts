import { BigNumber } from 'ethers';

export enum EncryptionState {
  IDLE = 1,
  UPLOADED = 2,
  DECRYPTED = 5,
  ERROR = 6,
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
