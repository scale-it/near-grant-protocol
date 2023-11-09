use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseOrValue, ONE_YOCTO};

use crate::ext::{ext_ft, ext_reg, Status};
use crate::metadata::TokenMetadata;
use crate::types::WrappedToken;
use crate::*;

#[ext_contract(ext_ft_receiver)]
pub trait FungibleTokenReceiver {
    /// Called by fungible token contract after `ft_transfer_call` was initiated by
    /// `sender_id` of the given `amount` with the transfer message given in `msg` field.
    /// The `amount` of tokens were already transferred to this contract account and ready to be used.
    ///
    /// The method must return the amount of tokens that are *not* used/accepted by this contract from the transferred
    /// amount. Examples:
    /// - The transferred amount was `500`, the contract completely takes it and must return `0`.
    /// - The transferred amount was `500`, but this transfer call only needs `450` for the action passed in the `msg`
    ///   field, then the method must return `50`.
    /// - The transferred amount was `500`, but the action in `msg` field has expired and the transfer must be
    ///   cancelled. The method must return `500` or panic.
    ///
    /// Arguments:
    /// - `sender_id` - the account ID that initiated the transfer.
    /// - `amount` - the amount of tokens that were transferred to this account in a decimal string representation.
    /// - `msg` - a string message that was passed with this transfer call.
    ///
    /// Returns the amount of unused tokens that should be returned to sender, in a decimal string representation.
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        #[allow(unused_variables)] msg: String,
    ) -> PromiseOrValue<U128> {
        let token = env::predecessor_account_id();
        let gtoken = &(sender_id.clone(), token);
        let b = match self.ft_balances.get(gtoken) {
            Some(b) => b,
            None => {
                let id = self.tokens.next_token_id;
                self.tokens.next_token_id += 1;
                WrappedToken { id, balance: 0 }
            }
        };
        self.ft_balances.insert(
            gtoken,
            &WrappedToken {
                id: b.id,
                balance: b.balance + amount.0,
            },
        );

        let metadata = mk_metadata(format!("{:?}", gtoken));
        self.tokens
            .internal_mint2(sender_id, b.id, amount.0, Some(metadata), None);

        // TODO: mint grant tokens event

        PromiseOrValue::Value(0.into())
    }
}

#[near_bindgen]
impl Contract {
    pub fn unwrap(&mut self, token_id: TokenId, amount: u128) {
        // check if caller has tokens
        let caller = env::predecessor_account_id();
        self.tokens.internal_withdraw(&token_id, &caller, amount);

        let mt = token_id.parse::<u64>().expect("incorrect token id");
        let gtoken = self.gtokens.get(&mt).expect("grant token does not exist");

        let old_ft_balance = self
            .ft_balances
            .get(&gtoken)
            .expect("wrapped token does not exist");
        self.ft_balances.insert(
            &gtoken,
            &WrappedToken {
                id: old_ft_balance.id,
                balance: old_ft_balance.balance - amount,
            },
        );

        // step2: check if there is a penalty
        ext_reg::ext(self.registry.clone())
            .with_attached_deposit(ONE_YOCTO)
            .account_status(caller.clone())
            .then(
                Self::ext(env::current_account_id())
                    .on_status_verified(amount, gtoken.1, token_id, caller),
            );
    }

    #[private]
    pub fn on_status_verified(
        &mut self,
        #[callback_unwrap] status: Option<ext::Status>,
        amount: u128,
        ft_contract: AccountId,
        mt_id: TokenId,
        caller: AccountId,
    ) {
        let amount_to_send;
        let mut penalty = 0;
        match status {
            Some(Status::Whitelisted) => {
                amount_to_send = amount;
            }
            Some(Status::Blacklisted) => {
                amount_to_send = 0;
                // TODO rollback the withdraw
            }
            None => {
                penalty = amount * 2 / 10; // 20% of the origianal value for the non-verified service providers
                amount_to_send = amount - penalty;
            }
        }
        if penalty != 0 {
            let mt_owner = self.tokens.owner_by_id.get(&mt_id).unwrap();
            self.tokens.internal_deposit(&mt_id, &mt_owner, penalty);
        }

        ext_ft::ext(ft_contract)
            .with_attached_deposit(ONE_YOCTO)
            .ft_transfer(
                caller,
                near_sdk::json_types::U128::from(amount_to_send),
                None,
            );
    }
}

fn mk_metadata(title: String) -> TokenMetadata {
    TokenMetadata {
        title: Some(title),
        description: None,
        media: None,
        media_hash: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    }
}
