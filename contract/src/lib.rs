#![cfg_attr(not(feature = "std"), no_std, no_main)]

extern crate alloc;

pub mod error;
pub mod utils;

pub use scale::{Decode, Encode};

// pink_extension is short for Phala Ink! extension
use pink_extension as pink;
use pink::PinkEnvironment;
use pink::chain_extension::signing::derive_sr25519_key;
use crate::alloc::string::ToString;

#[pink::contract(env = PinkEnvironment)]
mod phat_crypto {
    use super::*;
    use pink::http_get;
    use alloc::{vec::Vec, string::String, format};

    use crate::error::ApillonError;
    use utils::utils::is_nft_owner;
    use utils::utils::hash_message;

    use ink_storage::Mapping;
    use aes_gcm_siv::{
        Aes256GcmSiv,
        aead::{Nonce, KeyInit, Aead},
    };
    use cipher::{consts::{U12, U32}, generic_array::GenericArray};

    const SIGNATURE_VALID_TIME_IN_SECONDS: u64 = 5 * 60;

    pub type CustomResult<T> = Result<T, ApillonError>;

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
        ipfs_endpoint: String,
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
                private_key,
                salt,
                cid_map,
                owner,
                contract_id,
                owner_restriction,
                rpc_api,
                ipfs_endpoint,
            }
        }

        #[ink(message)]
        pub fn set_cid(&mut self, nft_id: u8, cid: String) -> CustomResult<String> {
            if !self.caller_is_contract_owner() {
                return Err(ApillonError::NoPermission);
            }
            self.cid_map.insert(nft_id, &cid);

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn set_cid_with_nft(&mut self, nft_id: u8, cid: String, signature: String, unix_timestamp: u64) -> CustomResult<String> {
            let hashed_message = match Self::check_timestamp_and_generate_message(unix_timestamp) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };
            if self.owner_restriction {
                return Err(ApillonError::NoPermission);
            }
            if !is_nft_owner(signature, hashed_message, nft_id, self.contract_id.to_string(), self.rpc_api.to_string()) {
                return Err(ApillonError::NotNftOwner);
            }
            self.cid_map.insert(nft_id, &cid);

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn get_cid(&self, nft_id: u8) -> CustomResult<String> {
            let cid = self.cid_map.get(nft_id);
            if cid.is_none() {
                return Err(ApillonError::CidMissingFordNftId);
            }
            Ok(format!("{}", cid.unwrap()))
        }

        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) -> CustomResult<String> {
            if !self.caller_is_contract_owner() {
                return Err(ApillonError::NoPermission);
            }
            self.owner = new_owner;

            Ok(String::from("Done"))
        }

        #[ink(message)]
        pub fn encrypt_content(&self, content: String) -> CustomResult<String> {
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);

            // Encrypt payload
            let cipher = Aes256GcmSiv::new(key.into());
            let encrypted_content: Vec<u8> = cipher.encrypt(nonce, content.as_bytes().as_ref()).unwrap();

            Ok(format!("{}", hex::encode(&encrypted_content)))
        }

        #[ink(message)]
        pub fn download_and_decrypt(&self, signature: String, unix_timestamp: u64, nft_id: u8) -> CustomResult<String> {
            let hashed_message = match Self::check_timestamp_and_generate_message(unix_timestamp) {
                Ok(value) => value,
                Err(value) => return Err(value),
            };
            if !is_nft_owner(signature, hashed_message, nft_id, self.contract_id.to_string(), self.rpc_api.to_string()) {
                return Err(ApillonError::NotNftOwner);
            }

            // retrieve content from IPFS by CID
            let cid = self.cid_map.get(nft_id);
            if cid.is_none() {
                return Err(ApillonError::CidMissingFordNftId);
            }
            let response = http_get!(format!("{}/{}", self.ipfs_endpoint.to_string(), cid.unwrap()));
            let response_body = match String::from_utf8(response.body) {
                Ok(value) => value,
                Err(e) => panic!("Failed to read response body: {}", e),
            };

            // decrypt payload
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&self.private_key[..32]);
            let cipher = Aes256GcmSiv::new(key.into());
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&self.salt);
            let response_body_decoded = hex::decode(response_body).unwrap();
            let content_decrypted: Vec<u8> = cipher.decrypt(nonce, response_body_decoded.as_ref()).unwrap();
            let content = format!("{}", String::from_utf8_lossy(&content_decrypted));

            Ok(content)
        }

        // HELPERS
        fn caller_is_contract_owner(&mut self) -> bool {
            let owner = String::from(format!("{:?}", &self.owner));
            let caller = String::from(format!("{:?}", Self::env().caller()));

            return owner == caller;
        }

        /*
        Check that signature/timestamp was generated before block timestamp and it should be at most 5 minutes old.
        If Timestamp is valid returned hashed message consisting of hardcoded message and timestamp.
         */
        fn check_timestamp_and_generate_message(unix_timestamp: u64) -> Result<[u8; 32], ApillonError> {
            let block_timestamp = Self::env().block_timestamp();
            if unix_timestamp < block_timestamp || block_timestamp.abs_diff(unix_timestamp) >= SIGNATURE_VALID_TIME_IN_SECONDS {
                return Err(ApillonError::BadTimestamp);
            }
            let timestamped_message = format!("APILLON_REQUEST_MSG: {}", unix_timestamp);
            let hashed_message = hash_message(&timestamped_message.to_string());
            Ok(hashed_message)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_CONTRACT_ADDRESS: &str = "51e044373c4ba5a3d6eef0f7f7502b3d2f60276f";
        const TEST_RPC_API: &str = "https://rpc.api.moonbeam.network/";
        const TEST_IPFS_ENDPOINT: &str = "https://ipfs.apillon.io/ipfs/";
        const TEST_NFT_ID: u8 = 1;
        const TEST_CID: &str = "QmZJTqJzHFt2kSDVWGWUXcgomDSBby1sTtiJcs3LXjXNnC";
        const TEST_DECRYPTED_CONTENT: &str = "test_string";
        const TEST_ENCRYPTED_CONTENT: &str = "53bfb3715cb5c28a6949d36d0e551a2434d10ad5415aaf783786d0";
        const TEST_MESSAGE_SIGNATURE: &str = "16519d5b537efb071f64d2728a1917e760a8eb97807795a8e4d144ebd6e460b810218071514795ee30b4a598beeb93cc80db5e6ac47ef695c8a8983a26f173991c";
        const TEST_SIGNATURE_TIMESTAMP: u64 = 1701392162;

        // TEST HELPERS
        fn test_accounts() -> ink::env::test::DefaultAccounts<PinkEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_block_timestamp(timestamp: u64) {
            ink::env::test::set_block_timestamp::<Environment>(timestamp);
        }

        fn set_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }

        fn get_contract(restrict_to_owner: bool) -> ApillonContract {
            // mock contract blockchain
            pink_extension_runtime::mock_ext::mock_all_ext();
            ApillonContract::new(
                TEST_CONTRACT_ADDRESS.to_string(),
                TEST_RPC_API.to_string(),
                TEST_IPFS_ENDPOINT.to_string(),
                restrict_to_owner,
            )
        }

        // TESTS
        // GET SET CID TESTS
        #[ink::test]
        fn new_creates_contract_correctly() {
            let contract = get_contract(true);

            assert_eq!(contract.contract_id, TEST_CONTRACT_ADDRESS);
            assert_eq!(contract.rpc_api, TEST_RPC_API);
            assert_eq!(contract.ipfs_endpoint, TEST_IPFS_ENDPOINT);
            assert_eq!(contract.owner_restriction, true);
            // assert_eq!(contract.cid_map, 0);
        }

        #[ink::test]
        fn contract_owner_can_set_and_get_cid() {
            let mut contract = get_contract(true);

            let result = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());

            assert_eq!(result.unwrap(), "Done");
            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID);
        }

        #[ink::test]
        fn get_cid_works_for_all_users() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);

            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID.to_string());
        }

        #[ink::test]
        fn get_cid_fails_if_cid_not_set_for_nft_id() {
            let contract = get_contract(true);

            assert_eq!(contract.get_cid(2), Err(ApillonError::CidMissingFordNftId));
        }

        #[ink::test]
        fn non_contract_owner_cant_set_cid() {
            let mut contract = get_contract(true);
            let accounts = test_accounts();
            _ = contract.set_owner(accounts.alice);
            set_caller(accounts.bob);

            assert_eq!(contract.set_cid(TEST_NFT_ID, TEST_CID.to_string()), Err(ApillonError::NoPermission));
        }

        #[ink::test]
        fn contract_owner_not_owning_nft_cant_set_cid_with_nft() {
            let mut contract = get_contract(false);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.set_cid_with_nft(
                2,
                TEST_CID.to_string(),
                TEST_MESSAGE_SIGNATURE.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
            );

            assert_eq!(result, Err(ApillonError::NotNftOwner));
            assert_eq!(contract.get_cid(2), Err(ApillonError::CidMissingFordNftId));
        }

        #[ink::test]
        fn nft_owner_can_set_cid_with_nft() {
            let mut contract = get_contract(false);
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.set_cid_with_nft(
                TEST_NFT_ID,
                TEST_CID.to_string(),
                TEST_MESSAGE_SIGNATURE.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
            );

            assert_eq!(result.unwrap(), "Done");
            assert_eq!(contract.get_cid(TEST_NFT_ID).unwrap(), TEST_CID);
        }

        // SET OWNER TESTS
        #[ink::test]
        fn contract_owner_can_set_new_contract_owner() {
            let mut contract = get_contract(true);

            assert_eq!(contract.set_owner(test_accounts().alice).unwrap(), "Done");
        }

        #[ink::test]
        fn non_contract_owner_cant_set_new_contract_owner() {
            let mut contract = get_contract(true);
            let accounts = test_accounts();
            set_caller(accounts.bob);

            assert_eq!(contract.set_owner(accounts.alice), Err(ApillonError::NoPermission));
        }

        // ENCRYPT CONTENT TESTS
        #[ink::test]
        fn anyone_can_encrypt_content() {
            let contract = get_contract(true);
            set_caller(test_accounts().bob);

            let result = contract.encrypt_content(TEST_DECRYPTED_CONTENT.to_string());

            assert_eq!(result.unwrap(), TEST_ENCRYPTED_CONTENT);
        }

        // DECRYPT CONTENT TESTS
        #[ink::test]
        fn user_can_decrypt_content_if_he_holds_whitelisted_nft() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.download_and_decrypt(
                TEST_MESSAGE_SIGNATURE.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
                TEST_NFT_ID,
            );

            assert_eq!(result.unwrap(), TEST_DECRYPTED_CONTENT);
        }

        #[ink::test]
        fn user_cant_decrypt_content_with_expired_signature_timestamp() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let expired_timestamp = TEST_SIGNATURE_TIMESTAMP + SIGNATURE_VALID_TIME_IN_SECONDS;
            let result = contract.download_and_decrypt(
                TEST_MESSAGE_SIGNATURE.to_string(),
                expired_timestamp,
                TEST_NFT_ID,
            );

            assert_eq!(result, Err(ApillonError::BadTimestamp));
        }

        #[ink::test]
        fn user_cant_decrypt_content_with_signature_timestamp_created_before_block_timestamp() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.download_and_decrypt(
                TEST_MESSAGE_SIGNATURE.to_string(),
                TEST_SIGNATURE_TIMESTAMP - 1,
                TEST_NFT_ID,
            );

            assert_eq!(result, Err(ApillonError::BadTimestamp));
        }

        #[ink::test]
        fn user_cant_decrypt_content_with_fake_signature_timestamp() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(TEST_NFT_ID, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);
            // signature of message="APILLON_REQUEST_MSG: 1701392163" with fake timestamp
            let fake_signature = "851caebcdb8af36a06fb9a2f011bd82f8285739198aa3c0f893c6bbc7a158fd7127e387e8c3737404618ae0515d22feaf3928c1d896163f4c441ce33d6e095f71b";

            let result = contract.download_and_decrypt(
                fake_signature.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
                TEST_NFT_ID,
            );

            assert_eq!(result, Err(ApillonError::NotNftOwner));
        }

        #[ink::test]
        fn user_cant_decrypt_content_if_he_holds_non_whitelisted_nft() {
            let mut contract = get_contract(true);
            _ = contract.set_cid(2, TEST_CID.to_string());
            set_caller(test_accounts().bob);
            set_block_timestamp(TEST_SIGNATURE_TIMESTAMP);

            let result = contract.download_and_decrypt(
                TEST_MESSAGE_SIGNATURE.to_string(),
                TEST_SIGNATURE_TIMESTAMP,
                TEST_NFT_ID,
            );

            assert_eq!(result, Err(ApillonError::CidMissingFordNftId));
        }
    }
}
