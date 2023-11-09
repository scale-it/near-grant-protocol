use near_sdk::json_types::U128;
use near_sdk::{env, ext_contract, near_bindgen, AccountId, PromiseOrValue};

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
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token = env::predecessor_account_id();
        let gtoken = &(sender_id, token);
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
        // self.tokens
        //     .internal_mint(owner_id, supply, metadata, refund_id);

        // TODO: mint grant tokens
        // TODO: mint event

        PromiseOrValue::Value(0.into())
    }
}
