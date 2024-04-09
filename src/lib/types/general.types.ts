export enum EncryptionState {
  IDLE = 1,
  WALLET_CONNECTED = 2,
  VERIFYING_OWNER = 3,
  DECRYPTED = 4,
  ERROR = 5,
}

declare global {
  interface Nft {
    id: number;
    name: string;
    description: string;
    image: string;
  }
}
