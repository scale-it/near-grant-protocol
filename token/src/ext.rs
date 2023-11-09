use crate::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, AccountId};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum Status {
    Blacklisted,
    Whitelisted,
}

#[ext_contract(ext_ft)]
pub trait ExtFungibleToken {
    fn ft_transfer(&mut self, receiver_id: near_sdk::AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_reg)]
pub trait ExtRegistry {
    fn account_status(&self, account: AccountId) -> Option<Status>;
}
