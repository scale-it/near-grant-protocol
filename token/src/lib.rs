use crate::core::{MultiToken, MultiTokenCore, StorageKey};

use enumeration::MultiTokenEnumeration;
use metadata::MtContractMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
use token::{Token, TokenId};
use token_holders::MultiTokenHolders;
use types::WrappedToken;

pub mod approval;
pub mod core;
pub mod enumeration;
pub mod event_core;
pub mod events;
pub mod ext;
pub mod ft_receiver;
pub mod metadata;
pub mod migration;
pub mod storage_impl;
pub mod token;
pub mod token_holders;
pub mod types;
pub mod utils;

// penalty in bp
pub const PENALTY: u128 = 2_000;
pub const ONE_BP: u128 = 10_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: core::MultiToken,
    metadata: LazyOption<MtContractMetadata>,

    // TODO: add check how much GRANT tokens are locked to represent claims.
    /// ft_balances: (issuer, ft contract) => (mapped tokenID wrapped balance)
    pub ft_balances: LookupMap<(AccountId, AccountId), WrappedToken>,

    /// revers map from TokenId => (issuer, ft contract)
    pub gtokens: LookupMap<u64, (AccountId, AccountId)>,

    /// registry keeping track of the service providers status
    pub registry: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, registry: AccountId) -> Self {
        let metadata = MtContractMetadata {
            spec: metadata::MT_METADATA_SPEC.to_string(),
            name: "GrantToken".to_string(),
            symbol: "gToken".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        metadata.assert_valid();

        Self {
            tokens: MultiToken::new(owner_id),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            ft_balances: LookupMap::new(StorageKey::FTBalances),
            gtokens: LookupMap::new(StorageKey::GTokens),
            registry,
        }
    }
}

#[near_bindgen]
impl MultiTokenCore for Contract {
    #[payable]
    fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        approval: Option<(AccountId, u64)>,
        memo: Option<String>,
    ) {
        self.tokens
            .mt_transfer(receiver_id, token_id, amount, approval, memo)
    }

    #[payable]
    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        approvals: Option<Vec<Option<(AccountId, u64)>>>,
        memo: Option<String>,
    ) {
        self.tokens
            .mt_batch_transfer(receiver_id, token_ids, amounts, approvals, memo)
    }

    #[payable]
    fn mt_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: U128,
        approval: Option<(AccountId, u64)>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.tokens
            .mt_transfer_call(receiver_id, token_id, amount, approval, memo, msg)
    }

    #[payable]
    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        approvals: Option<Vec<Option<(AccountId, u64)>>>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<Vec<U128>> {
        self.tokens
            .mt_batch_transfer_call(receiver_id, token_ids, amounts, approvals, memo, msg)
    }

    fn mt_token(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.mt_token(token_id)
    }

    fn mt_token_list(&self, token_ids: Vec<TokenId>) -> Vec<Option<Token>> {
        self.tokens.mt_token_list(token_ids)
    }

    fn mt_balance_of(&self, account_id: AccountId, token_id: TokenId) -> U128 {
        self.tokens.mt_balance_of(account_id, token_id)
    }

    fn mt_batch_balance_of(&self, account_id: AccountId, token_ids: Vec<TokenId>) -> Vec<U128> {
        self.tokens.mt_batch_balance_of(account_id, token_ids)
    }

    fn mt_supply(&self, token_id: TokenId) -> Option<U128> {
        self.tokens.mt_supply(token_id)
    }

    fn mt_batch_supply(&self, token_ids: Vec<TokenId>) -> Vec<Option<U128>> {
        self.tokens.mt_batch_supply(token_ids)
    }
}

#[near_bindgen]
impl MultiTokenEnumeration for Contract {
    /// Get a list of all tokens (with pagination)
    ///
    /// # Arguments:
    /// * `from_index` - Index to start from, defaults to 0 if not provided
    /// * `limit` - The maximum number of tokens to return
    ///
    /// returns: List of [Token]s.
    ///
    fn mt_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.tokens.mt_tokens(from_index, limit)
    }

    /// Get list of all tokens by a given account
    ///
    /// # Arguments:
    /// * `account_id`: a valid NEAR account
    /// * `from_index` - Index to start from, defaults to 0 if not provided
    /// * `limit` - The maximum number of tokens to return
    ///
    /// returns: List of [Token]s owner by user
    ///
    fn mt_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        self.tokens
            .mt_tokens_for_owner(account_id, from_index, limit)
    }
}

#[near_bindgen]
impl MultiTokenHolders for Contract {
    fn mt_token_holders(
        &self,
        token_id: TokenId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AccountId> {
        self.tokens.mt_token_holders(token_id, from_index, limit)
    }
}
