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

    pub fn verify_ownership_moonbase(nft_id: u8) -> String  {
        let phttp = PinkHttp::new("https://moonbase-alpha.public.blastapi.io");
        let eth = Eth::new(phttp);
        
        let addr = String::from("90f44d3b9d9fa626c1f3109ba55296b9edd7d3ce").parse().unwrap();
        let contract = Contract::from_json(eth, addr, include_bytes!("../abi/erc721_abi.json")).unwrap();

        let query = "ownerOf";
        let addrs: Address = resolve_ready(contract.query(
            &query, (U256::from(nft_id), ), None, Options::default(), None)).unwrap();
        let addrs_moonbase: String =  hex::encode(addrs.0);
        addrs_moonbase   
    }
}
