#![cfg_attr(not(feature = "std"), no_std, no_main)]

extern crate alloc;

pub mod error;
pub mod utils;

pub use scale::{Decode, Encode};

// pink_extension is short for Phala Ink! extension
use pink_extension as pink;
use pink::PinkEnvironment;
use pink::chain_extension::signing::derive_sr25519_key;
#[pink::contract(env=PinkEnvironment)]
mod phat_crypto {
    use super::*;
    use pink::http_get;
    use alloc::{vec::Vec, string::String, format};

    use crate::error::ApillonError;
    use utils::utils::verify_nft_ownership;

    use ink_storage::Mapping;
    use aes_gcm_siv::{
        Aes256GcmSiv,
        aead::{Nonce, KeyInit, Aead}
    };
    use cipher::{consts::{U12, U32}, generic_array::GenericArray};

    pub type CustomResult<T> = core::result::Result<T, ApillonError>;

    type NftId = u8;
    type Cid = String;

    #[ink(storage)]
    pub struct ApillonContract {
        private_key: Vec<u8>,
        salt: Vec<u8>,
        cid_map: Mapping<NftId, Cid>,
        owner: AccountId,
        owner_restriction: bool,
        contract_id: String,
        rpc_api: String,
        ipfs_endpoint: String
    }

    impl ApillonContract {
        #[ink(constructor)]
        pub fn new(contract_id: String, rpc_api: String, ipfs_endpoint: String, owner_restriction: bool) -> Self {
            // Default constructor
            let salt = b"981781668367".to_vec();
            let private_key = derive_sr25519_key(&salt);
            let owner = Self::env().caller();
            let cid_map = Mapping::default();

            Self {
                private_key, salt, cid_map, owner, contract_id, owner_restriction, rpc_api, ipfs_endpoint
            }
        }

        #[ink(message)]
        pub fn set_cid(&mut self, nft_id: u8, cid: String) -> CustomResult<String> {
            if self.owner_restriction && self.owner != Self::env().caller() {
                Ok(format!("INVALID"))
            } else {
                self.cid_map.insert(nft_id, &cid);
                Ok(String::from("Ok"))
            }
        }

        #[ink(message)]
        pub fn test_get_data(&self) -> CustomResult<String> {
            let data = String::from(
                format!("owner {:?}, owner_restriction {:?}, contract_id {:?}, rpc_api {:?}, ipfs_endpoint {:?}", 
                    self.owner, self.owner_restriction, self.contract_id, self.rpc_api, self.ipfs_endpoint));
            Ok(data)
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: u8) -> CustomResult<String> {
            let cid = self.cid_map.get(nft_id).unwrap();
            Ok(format!("{}", cid))
        }

        #[ink(message)]
        pub fn set_owner(&mut self) -> CustomResult<String> {
            if self.owner != Self::env().caller() {
                Ok(format!("INVALID"))
            } else {
                self.owner = Self::env().caller();
                Ok(format!("New owner set {:?}", self.owner))
            }
        }

        #[ink(message)]
        pub fn verify_nft_ownership(&self, signature: String, message: String, nft_id: u8) -> CustomResult<bool> {
            let contract_id = String::from(&self.contract_id);
            let rpc_api = String::from(&self.rpc_api);

            let is_owner = verify_nft_ownership(signature, message, nft_id, contract_id, rpc_api);
            Ok(is_owner)
        }

        #[ink(message)]
        pub fn encrypt_content(&self, file_content: String) -> CustomResult<String> {
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);

            // Encrypt payload
            let cipher = Aes256GcmSiv::new(key.into());
            let encrypted: Vec<u8> = cipher.encrypt(nonce, file_content.as_bytes().as_ref()).unwrap();

            Ok(format!("{}", hex::encode(&encrypted)))
        }

        #[ink(message)]
        pub fn download_and_decrypt(&self, signature: String, message: String, nft_id: u8) -> CustomResult<String> {
            
            let contract_id = String::from(&self.contract_id);
            let rpc_api = String::from(&self.rpc_api);
            let ipfs_endpoint = String::from(&self.ipfs_endpoint);

            let is_owner = verify_nft_ownership(
                signature, message, nft_id, contract_id, rpc_api);

            if is_owner == true {
                let cid = self.cid_map.get(nft_id).unwrap();
                let response = http_get!(
                    format!("{}/{}", ipfs_endpoint, cid));

                let resp_body_str = match String::from_utf8(response.body) {
                    Ok(value) => value,
                    Err(e) => panic!("Mja, error, kaj ces {}", e),
                };

                // Decrypt payload
                let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
                let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);
            
                // Encrypt payload
                let decoded = hex::decode(resp_body_str).unwrap();
                let cipher = Aes256GcmSiv::new(key.into());
                let decrypted: Vec<u8> = cipher.decrypt(nonce, decoded.as_ref()).unwrap();

                let result = format!("{}", String::from_utf8_lossy(&decrypted));

                Ok(result)
            } else {
                Ok(format!("INVALID"))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        // use core::panic;
        // use ink_lang as ink;

        // use super::*;

        // #[ink_e2e::test]
        // async fn test_cid_set(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     Ok(())
        // }
    }

}