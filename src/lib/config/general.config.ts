import { Buffer } from 'buffer';

const CONFIG = ref(import.meta.env);

export const NFT_ADDRESS = '0xfA793D247D906de7f5e27aD96bd0EEF86fBb084F';

export const PHALA_RPC = 'wss://api.phala.network/ws';
// export const CONTRACT_ADDRESS = '0x4a603261f21c415522e7e72044329f84530ba8f6ec15290488b9a0588b1f7eaa';
// export const CONTRACT_ADDRESS = '0x9597b016f12b0c72777a144cc8f56c3f32e56db56a1c2794a02710097fffd7d2';
export const CONTRACT_ADDRESS =
  '0x6300e4e5507c5ba2fa6e77fb44d416c97da4d952e9ef3bcbb52eab1a16b49f61';

const apiKey = '1a9202b6-6f2a-45c4-90a1-bcff3fe6a351';
const apiSecret = '8rZn4tLx5CBu';
export const AUTHORIZATION = Buffer.from(apiKey + ':' + apiSecret).toString('base64');
export const BUCKET_UUID = CONFIG.value.VITE_BUCKET_UUID;
