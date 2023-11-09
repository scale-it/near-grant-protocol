use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/// FT info (it's MT ID, and wrapped balance)
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WrappedToken {
    pub id: u64, // token ID
    pub balance: u128,
}
