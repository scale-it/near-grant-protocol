use crate::U128;
use near_sdk::ext_contract;

#[ext_contract(ext_ft)]
pub trait ExtFungibleToken {
    fn ft_transfer(&mut self, receiver_id: near_sdk::AccountId, amount: U128, memo: Option<String>);
}
