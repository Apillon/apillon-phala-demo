use crate::alloc::string::ToString;
use alloc::vec::Vec;
use alloc::string::String;

use ink_env::{ecdsa_recover, ecdsa_to_eth_address};

use pink_web3::{
    types::{U256, Address},
    contract::{Options, Contract},
    transports::{
        pink_http::PinkHttp,
        resolve_ready
    },
    api::{Eth, Namespace}
};


pub mod utils {
    use super::*;
    use ink_env::hash;

    pub fn hash_message(message: &str) -> [u8; 32] {
        let message_prefix = "\x19Ethereum Signed Message:\n";
        let prefix_bytes = to_utf8_bytes(message_prefix);
        let length_bytes = to_utf8_bytes(&message.len().to_string());

        let mut concatenated = Vec::new();
        concatenated.extend_from_slice(&prefix_bytes);
        concatenated.extend_from_slice(&length_bytes);
        concatenated.extend_from_slice(to_utf8_bytes(message).as_slice());

        let hash = hash_keccak_256(concatenated.as_slice());
        // let test = hex::encode(hash)
        hash
    }

    fn to_utf8_bytes(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    fn hash_keccak_256(input: &[u8]) -> [u8; 32] {
        let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
        ink_env::hash_bytes::<hash::Keccak256>(input, &mut output);
        output
    }

    pub fn recover_acc_address(signature: String, message: [u8; 32]) -> String {
        let signature: [u8; 65] = hex::decode(signature)
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();

        let message_hash = message.as_slice().try_into().unwrap();

        let mut pub_key = [0u8; 33];
        let mut address = [0u8; 20];
        
        ecdsa_recover(&signature, &message_hash, &mut pub_key).unwrap();
        ecdsa_to_eth_address(&mut pub_key, &mut address).unwrap();

        hex::encode(address)
    }

    pub fn get_nft_owner_address(nft_id: u8, contract_id: String, rpc_api: String) -> String  {
        let default: Address =  Address::zero();
        let phttp = PinkHttp::new(rpc_api);
        let eth = Eth::new(phttp);
        let addr = String::from(contract_id).parse().unwrap();

        let contract = Contract::from_json(eth, addr, include_bytes!("../abis/moonbase_nft_abi.json")).unwrap();

        let query = "ownerOf";
        let address: Address = resolve_ready(contract.query(&query, (U256::from(nft_id), ), None, Options::default(), None)).unwrap_or(default);
        hex::encode(address.0)
    }

    /*
    Checks if message signer is owner of NFT with specified id.
     */
    pub fn is_nft_owner(signature: String, message: [u8; 32], nft_id: u8, contract_id: String, rpc_api: String) -> bool{
        let signer_address = recover_acc_address(signature, message);
        let nft_owner_address = get_nft_owner_address(nft_id, contract_id, rpc_api);

        signer_address == nft_owner_address
    }
}
