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
    use utils::utils::{recover_acc_address, verify_nft_ownership};

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
        contract_id: String
    }

    impl ApillonContract {
        #[ink(constructor)]
        pub fn new(contract_id: String, owner_restriction: bool) -> Self {
            // Default constructor
            let salt = b"981781668367".to_vec();
            let private_key = derive_sr25519_key(&salt);
            let owner = Self::env().caller();
            let cid_map = Mapping::default();

            Self {
                private_key, salt, cid_map, owner, contract_id, owner_restriction
            }
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
        pub fn set_cid(&mut self, nft_id: u8, cid: String) {
            self.cid_map.insert(nft_id, &cid);
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: u8) -> CustomResult<String> {
            let cid = self.cid_map.get(nft_id).unwrap();
            Ok(format!("{}", cid))
        }

        #[ink(message)]
        pub fn get_caller(&self) -> CustomResult<String> {
            let caller = Self::env().caller();
            Ok(format!("{:?}", caller))
        }

        #[ink(message)]
        pub fn get_owner(&self) -> CustomResult<String> {
            Ok(format!("{:?}", self.owner))
        }

        #[ink(message)]
        pub fn test_ownership(&self) -> CustomResult<String> {
            let caller = Self::env().caller();
            if(caller != self.owner) {
                core::panic!("Unauthorized");
            }
            Ok(format!("{:?}", caller != self.owner))
        }
        
        #[ink(message)]
        pub fn verify_nft_ownership(&self, signature: String, message: String, nft_id: u8) -> CustomResult<bool> {
            let is_owner = verify_nft_ownership(signature, message, nft_id, &self.contract_id);
            Ok(is_owner)
        }

        #[ink(message)]
        pub fn download_and_decrypt(&self, signature: String, message: String, nft_id: u8) -> CustomResult<String> {
            let is_owner = verify_nft_ownership(signature, message, nft_id, &self.contract_id);

            if is_owner == true {
                let cid = self.cid_map.get(nft_id).unwrap();
                let response = http_get!(
                    format!("https://ipfs2.apillon.io/ipfs/{}", cid));

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
        use core::panic;

        use super::*;

        // TODO: TEST!!!!!!!!!!!!! This is a demo, but heck if this gets in production, tests must be written
        // BEFORE the contract is deployed!!!! If someone does not do it this way, I will personally terminate them

    }

}