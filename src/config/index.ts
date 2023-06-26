import { BigNumber } from 'ethers';

export {};

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
