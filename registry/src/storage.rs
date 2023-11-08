use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Whitelist,
    Blacklist,,
}

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum Status {
    Blacklisted,
    Whitelisted,
}