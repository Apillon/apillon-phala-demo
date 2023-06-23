import { BigNumber } from 'ethers';

export interface Nft {
  name: string;
  description: string;
  image: string;
}
  
export interface CollectionInfo {
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