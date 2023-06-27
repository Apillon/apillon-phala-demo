use alloc::string::String;

use ink_env::{ecdsa_recover, ecdsa_to_eth_address};

use pink_web3::{
    types::{U256, Address},
    contract::{Options, Contract},
    transports::{
        pink_http::PinkHttp,
        resolve_ready
    },
    contract::tokens::Detokenize,
    api::{Eth, Namespace}
};


pub mod utils {

    use super::*;

    pub fn recover_acc_address(signature: String, message: String) -> String {
        let signature: [u8; 65] = hex::decode(signature)
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();

        let message_hash: [u8; 32] = hex::decode(message)
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();

        let mut pub_key = [0u8; 33];
        let mut address = [0u8; 20];
        ecdsa_recover(&signature, &message_hash, &mut pub_key).unwrap();
        ecdsa_to_eth_address(&mut pub_key, &mut address).unwrap();

        hex::encode(address)
    }

    pub fn map_nft_to_address(nft_id: u8) -> String  {
        let default: Address =  Address::zero();
        let phttp = PinkHttp::new("https://rpc.api.moonbeam.network/");
        let eth = Eth::new(phttp);
        
        let addr = String::from("D412049ee57a2f83263248E2715e930E6E0780Cb").parse().unwrap();
        let contract = Contract::from_json(eth, addr, include_bytes!("../abis/moonbase_nft_abi.json")).unwrap();

        let query = "ownerOf";
        let addrs: Address = resolve_ready(contract.query(&query, (U256::from(nft_id), ), None, Options::default(), None)).unwrap_or(default);
        let addrs_moonbase: String =  hex::encode(addrs.0);
        addrs_moonbase
    }

    pub fn verify_nft_ownership(signature: String, message: String, nft_id: u8) -> bool{
        // Recover address: signature + message -> pubkey -> address 
        let address = recover_acc_address(signature, message);
        // Validate ownership for nft_id
        let owner_address = map_nft_to_address(nft_id);

        if address == owner_address {
            return true
        }

        return false
    }
}
