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
    use utils::utils::{recover_acc_address, verify_ownership_moonbase};

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
        owner: AccountId
    }

    impl ApillonContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            // Default constructor
            let salt = b"981781668367".to_vec();
            let private_key = derive_sr25519_key(&salt);
            let owner = Self::env().caller();
            let cid_map = Mapping::default();

            Self {
                private_key, salt, cid_map, owner,
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
            if self.env().caller() != self.owner {
                // TODO: This does not currently work
                core::panic!("caller {:?}, owner {:?}", self.env().caller(), self.owner)
            }
            self.cid_map.insert(nft_id, &cid);
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: u8) -> CustomResult<String> {
            let cid = self.cid_map.get(nft_id).unwrap();
            Ok(format!("{}", cid))
        }

        #[ink(message)]
        pub fn download_and_decrypt(&self, signature: String, message: String, nft_id: u8) -> CustomResult<String> {
            // Recover address: signature + message -> pubkey -> address 
            let address = recover_acc_address(signature, message);
            // Validate ownership for nft_id
            let owner_address = verify_ownership_moonbase(nft_id);

            if address == owner_address {
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
                Ok(format!("Invalid address {} vs {}", address, owner_address))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use core::panic;

        use super::*;

        // let response = http_get!(url);
        // let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key);
        // let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);
    
        // // Decrypt payload
        // let cipher = Aes256GcmSiv::new(key.into());
        // let decrypted_text = cipher.decrypt(&nonce, response.body.as_ref()).unwrap();
        // let result = format!("{}", String::from_utf8_lossy(&decrypted_text));

        // #[test]
        // fn validation() {
        //     // This is required in all pink tests
        //     pink_extension_runtime::mock_ext::mock_all_ext();

        //     use pink_web3::transports::{
        //         pink_http::PinkHttp,
        //         resolve_ready
        //     };

        //     // Signature of the caller
        //     let sig = "cfbc1f051882cb2551d5a5127bb03b39d97ade4b58a7f306a4f960e128913fc9172a3239f2065be3884b92885b617dbe0764ccbea57cf8100f24cc5801cd1be61b";

        //     // Hashed message of the caller
        //     let msg = "ccfbd2e0b3e43aa38ca81ef760175b91166405f59857ab98c2b5ceb51ba4e463";

        //     // ID's of issued nft tokens
        //     let token_id = 1;
        //     let token_id_control = 2;

        //     // Create a Transport using pink http API
        //     let phttp = PinkHttp::new("https://moonbase-alpha.public.blastapi.io");
        //     let eth = Eth::new(phttp);

        //     // This is the address of the contract that is issuing NTFs
        //     let addrs_hex = hex_literal::hex!("90f44d3b9d9fa626c1f3109ba55296b9edd7d3ce");
        //     let address: Address = Address::from_slice(&addrs_hex);
        //     // The contract object is created from the eth instance, address and the contract abi
        //     let contract = Contract::from_json(eth, address, include_bytes!("../abi/erc721_abi-new.json")).unwrap();

        //     let mut signature = [0u8; 65];
        //     hex::decode_to_slice(sig, &mut signature as &mut [u8; 65]).unwrap();

        //     let mut message = [0u8; 32];
        //     hex::decode_to_slice(msg, &mut message as &mut [u8; 32]).unwrap();
            
        //     // Perform ecdsa recovery and compute account address from the
        //     // recovered public key
        //     let mut output = [0; 33];
        //     let mut output_address = [0; 20];
        //     ecdsa_recover(&signature, &message, &mut output).unwrap();
        //     ecdsa_to_eth_address(&mut output, &mut output_address).unwrap();

        //     let address = String::from(hex::encode(output_address));
            
        //     // Get owner (public address) of the nft with ID tokenId and tokenIdControl
        //     let query = "ownerOf";
        //     let address_1: Address = resolve_ready(contract.query(
        //         &query, (U256::from(token_id), ), None, Options::default(), None)).unwrap();
            
        //     assert_eq!(format!("0x{}", address), format!("{:?}", address_1));

        //     // This address is valid, but it's not the caller's address
        //     let address_2: Address = resolve_ready(contract.query(
        //         &query, (U256::from(token_id_control), ), None, Options::default(), None)).unwrap();

        //     assert_ne!(format!("0x{}", address), format!("{:?}", address_2));

        // }

        // #[test]
        // fn address_recovery() {
        //     // R+S actually
        //     let sig = "83ee96fd0047083b3c302cd2de3a5c5eb87e56903486d10f410f94cb17137e0c69a94113361aeb2f958fc211709da2983824fb2247ba3a8773354df94d8b16921b";
        //     let msg = "5240a32aab803873141d1df3dbfce4213d56415caae2cdd234a8549836b6c96e";
        //     let expected_pub_key = "02b9e72dfd423bcf95b3801ac93f4392be5ff22143f9980eb78b3a860c4843bfd0";
        //     let expected_address = "71cb05ee1b1f506ff321da3dac38f25c0c9ce6e1";

        //     let mut signature = [0u8; 65];
        //     hex::decode_to_slice(sig, &mut signature as &mut [u8; 65]).unwrap();

        //     let mut message = [0u8; 32];
        //     hex::decode_to_slice(msg, &mut message as &mut [u8; 32]).unwrap();
            
        //     let mut output_pub_key = [0; 33];
        //     ecdsa_recover(&signature, &message, &mut output_pub_key).unwrap();
        //     assert_eq!(hex::encode(output_pub_key).to_string(), expected_pub_key);

        //     let mut output_address = [0; 20];
        //     ecdsa_to_eth_address(&mut output_pub_key, &mut output_address).unwrap();
        //     assert_eq!(hex::encode(output_address).to_string(), expected_address);
        // }

        // NOTE: Luka 10.5.2023 - Download and upload will be performed out of the contract
        //       this code should remain because it might be useful in the future - TODO: Archive
        // #[test]
        // fn test_download_files_ipfs() {
        //     pink_extension_runtime::mock_ext::mock_all_ext();
        //     let response = http_get!("https://ipfs2.apillon.io/ipfs/QmQLuYkRADePaWJtG6vxXaxurMgeuhcbEkRfC4xW6ceDFQ");
        //     assert_eq!(response.status_code, 200);
        // }

        // #[test]
        // fn test_upload_files_ipfs() {
        //     pink_extension_runtime::mock_ext::mock_all_ext();
        //     use crate::{
        //         create_file_payload,
        //         create_ipfs_sync,
        //         UploadedFile,
        //         StorageResponse
        //     };
            
        //     let content_type = String::from("text/html");
        //     let bucket_uuid: String = String::from("10268b28-684e-42a1-a037-5ce3663e7827");
        //     let file_name: String = String::from("IamTheFilerus.txt");
        //     let file_content: String = String::from("Goo goo g'joob, goo goo goo g'joob");

        //     // ** UPLOAD FILE TO APILLON STORAGE ** //
        //     let url_f_upload = format!("https://api-dev.apillon.io/storage/{}/upload", bucket_uuid);
        //     let url_get_content: String = format!("https://api-dev.apillon.io/storage/{}/content", bucket_uuid);
            
        //     let file = UploadedFile { fileName: String::from(file_name), contentType: content_type };
        //     let json_data = create_file_payload(file);

        //     let mut output_buffer = [0u8; 68];
        //     let message = "44f2b448-b89c-42df-afd2-c487d9a7b4a4:@AUAY0DHm86P";
        //     let encoded = b64encode(&message.as_bytes(), &mut output_buffer).ok().unwrap();
        //     let authorization = format!("Basic {}", String::from_utf8_lossy(encoded));
        //     let content_type = format!("application/json");

        //     let headers: Vec<(String, String)> = vec![
        //         ("Authorization".into(), authorization),
        //         ("Content-Type".into(), content_type)
        //     ];

        //     // assert_ne!(headers, headers);
        //     let response = http_get!(url_get_content, headers.clone());
        //     assert_eq!(response.status_code, 200);

        //     let response = http_post!(url_f_upload, json_data, headers.clone());

        //     let resp_body_str = match String::from_utf8(response.body) {
        //         Ok(r) => r,
        //         Err(e) => panic!("Mja, error, kaj ces {}", e),
        //     };

        //     assert_eq!(response.status_code, 201);
        //     let resp: StorageResponse = pink_json::from_str(&resp_body_str).unwrap();
        //     let file = &resp.data.files[0];
        //     let url_upload_s3: String = format!("{}", file.url);
        //     let content = file_content.as_bytes();
        //     let origin = String::from("https://app-dev.apillon.io/");

        //     let content_type = format!("text/plain");
        //     let headers_s3: Vec<(String, String)> = vec![
        //         ("Content-Type".into(), content_type),
        //         ("Referer".into(), origin.clone()),
        //         ("Origin".into(), origin.clone())
        //     ];

        //     let response = http_put!(url_upload_s3, *content, headers_s3);
        //     assert_eq!(response.status_code, 200);

        //     // ** TRIGGER UPLOAD TO IPFS (From Apillon storage) ** //
        //     let url_sync_ipfs = format!(
        //         "https://api-dev.apillon.io/storage/{}/upload/{}/end", 
        //         bucket_uuid, resp.data.sessionUuid);

        //     let ipfs_sync_json = create_ipfs_sync(true);

        //     let response = http_post!(url_sync_ipfs, ipfs_sync_json, headers.clone());
        //     assert_eq!(response.status_code, 200);

        // }

        // #[derive(Serialize, Deserialize)]
        // pub struct IPFSSync { directSync: bool }
        
        // pub fn create_ipfs_sync(direct_sync: bool) -> String {
        //     // Some data structure.
        //     let sync = IPFSSync { directSync: direct_sync };
        //     // Serialize it to a JSON string.
        //     let s = pink_json::to_string(&sync);
        //     s.unwrap()
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct RequestContent {
        //     data: String,
        // }
        
        // // This is a trait, which is used to serialize / deserialize data in the struct
        // #[derive(Serialize, Deserialize)]
        // pub struct UploadedFile {
        //     fileName: String,
        //     contentType: String
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct PendingFile {
        //     fileName: String,
        //     url: String,
        //     fileUuid: String
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct StorageResponseData {
        //     sessionUuid: String,
        //     files: Vec<PendingFile>,
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct StorageResponse {
        //     id: String,
        //     status: u8,
        //     data: StorageResponseData
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct IPFSSyncResponse {
        //     id: String,
        //     status: u8,
        //     data: bool
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct FileContent {
        //     id: String,
        // }
        
        // #[derive(Serialize, Deserialize)]
        // pub struct Files { files: Vec<UploadedFile> }
        
        // pub fn create_file_payload(file: UploadedFile) -> String {
        //     // Some data structure.
        //     let files = Files { files: vec![file]};
        
        //     // Serialize it to a JSON string.
        //     let v = pink_json::to_string(&files);
        
        //     v.unwrap()
        // }

    }

}