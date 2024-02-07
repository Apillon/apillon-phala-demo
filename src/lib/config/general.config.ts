const CONFIG = ref(import.meta.env);

export const PHALA_RPC = 'wss://api.phala.network/ws';
export const CONTRACT_ADDRESS = CONFIG.value.VITE_CONTRACT_ADDRESS;
export const NFT_ADDRESS = CONFIG.value.VITE_NFT_ADDRESS;
export const NFT_CHAIN_ID = CONFIG.value.VITE_NFT_CHAIN_ID;
