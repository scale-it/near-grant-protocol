use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet};
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};

pub mod storage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// DAO
    pub authority: AccountId,
    pub whitelist: UnorderedSet<AccountId>,
    pub blacklist: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    /// Contract constructor.
    #[init]
    pub fn new(
        authority: AccountId,
        whitelist: Vec<AccountId>,
        banned: Vec<AccountId>,
    ) -> Self {
        let mut contract = Self {
            authority,
            whitelist: UnorderedSet::new(StorageKey::Whitelist)
            blacklist: UnorderedSet::new(StorageKey::Blacklist),
        };
        contract._whitelist_accounts(&whitelist);
        contract._blacklist_accounts(&blacklist);
        contract
    }

    //
    // Queries
    //

    pub fn is_whitelisted(&self, account: AccountId) -> bool {
        self.whitelist.contains(&account)
    }

    pub fn is_blacklisted(&self, account: AccountId) -> bool {
        self.blacklist.contains(&account)
    }

    /// Returns the account status, either `Whitelisted`, `Blacklisted`.
    /// Returns None if the account doesnt have a status.
    pub fn account_status(&self, account: AccountId) -> Option<AccountFlag> {
        if self.whitelist.contains(account) {
            return Some(AccountFlag::Whitelisted)
        }
        if self.blacklist.contains(account) {
            return Some(AccountFlag::Blacklist)
        }
        None
    }

    pub fn authority(self) -> AccountId {
        self.authority
    }

    //
    // Authority Transactions
    //

    pub fn change_authority(&mut self, new_authority: AccountId) {
        self.assert_authority();
        self.authority = new_authority;
    }

    #[payable]
    pub fn whitelist(
        &mut self,
        accounts: Vec<AccountId>,
    ) {
        let storage_start = env::storage_usage();
        self.assert_authority();
        for a in &accounts {
            if self.blacklist.contains(a) {
                self.blacklist.remove(a);
            }
            self.whitelist.insert(a);
        }

        self.assert_deposit(storage_start);
    }

    #[payable]
    pub fn blacklist(
        &mut self,
        accounts: Vec<AccountId>,
    ) {
        let storage_start = env::storage_usage();
        self.assert_authority();
        for a in &accounts {
            if self.whitelist.contains(a) {
                self.whitelist.remove(a);
            }
            self.blacklist.insert(a);
        }

        self.assert_deposit(storage_start);
    }


    pub fn assert_authority(&self) {
        require!(
            self.authority == env::predecessor_account_id(),
            "not an authority"
        )
    }

    fn assert_deposit(&self, storage_start: u128)  {
        const required_deposit = env::storage_usage - storage_start * env::storage_byte_cost();
        require!(required_deposit <= env::attached_deposit, "not enought deposit to cover the storage"); 
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{self, VMContextBuilder};
    use near_sdk::{testing_env, Balance, Gas, VMContext};

    use pretty_assertions::assert_eq;

    use super::*;

    fn alice() -> AccountId {
        AccountId::new_unchecked("alice.near".to_string())
    }

    fn bob() -> AccountId {
        AccountId::new_unchecked("bob.near".to_string())
    }

    fn carol() -> AccountId {
        AccountId::new_unchecked("carol.near".to_string())
    }

    fn dan() -> AccountId {
        AccountId::new_unchecked("dan.near".to_string())
    }

    fn authority() -> AccountId {
        AccountId::new_unchecked("authority.near".to_string())
    }

    fn max_gas() -> Gas {
        Gas::ONE_TERA.mul(300)
    }

    const MSECOND: u64 = 1_000_000; // milisecond in ns
    const START: u64 = 10;
    const MINT_DEPOSIT: Balance = 9 * MILI_NEAR;

    fn setup(predecessor: &AccountId, deposit: Balance) -> (VMContext, Contract) {
        let mut ctx = VMContextBuilder::new()
            .predecessor_account_id(admin())
            .block_timestamp(START * MSECOND) // multiplying by mili seconds for easier testing
            .is_view(false)
            .build();
        testing_env!(ctx.clone());
        let mut ctr = Contract::new(admin(), fractal_mainnet(), vec![1], admins_flagged());
        ctr.admin_add_sbt_issuer(issuer1());
        ctr.admin_add_sbt_issuer(issuer2());
        ctr.admin_add_sbt_issuer(issuer3());
        ctr.admin_set_authorized_flaggers([predecessor.clone()].to_vec());
        ctx.predecessor_account_id = predecessor.clone();
        testing_env!(ctx.clone());
        (ctx, ctr)
    }

    #[test]
    fn init_method() {
        let ctr = Contract::new(admin(), fractal_mainnet(), vec![1], vec![]);
        // make sure the iah_issuer has been set as an issuer
        assert_eq!(1, ctr.assert_issuer(&fractal_mainnet()));
    }
}
