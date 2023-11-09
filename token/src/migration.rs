use crate::*;

use near_sdk::env;

// registry/v1.6.0
#[derive(BorshDeserialize, PanicOnDefault)]
pub struct OldState {
    tokens: core::MultiToken,
    metadata: LazyOption<MtContractMetadata>,
    pub ft_balances: LookupMap<(AccountId, AccountId), WrappedToken>,
    pub gtokens: LookupMap<u64, (AccountId, AccountId)>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    // #[allow(dead_code)]
    pub fn migrate(registry: AccountId) -> Self {
        let old_state: OldState = env::state_read().expect("failed");
        // new field in the smart contract :
        // + transfer_lock: LookupMap<AccountId, u64>,

        Self {
            tokens: old_state.tokens,
            metadata: old_state.metadata,
            ft_balances: old_state.ft_balances,
            gtokens: old_state.gtokens,
            registry,
        }
    }
}
