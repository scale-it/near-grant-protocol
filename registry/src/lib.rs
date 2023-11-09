use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};
use storage::{Status, StorageKey};

pub mod storage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// DAO
    pub authority: AccountId,
    pub service_providers: UnorderedMap<AccountId, Status>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(authority: AccountId) -> Self {
        Self {
            authority,
            service_providers: UnorderedMap::new(StorageKey::ServiceProviders),
        }
    }

    //
    // Queries
    //

    pub fn is_whitelisted(&self, account: AccountId) -> bool {
        match self.service_providers.get(&account) {
            Some(Status::Whitelisted) => true,
            _ => false,
        }
    }

    pub fn is_blacklisted(&self, account: AccountId) -> bool {
        match self.service_providers.get(&account) {
            Some(Status::Blacklisted) => true,
            _ => false,
        }
    }

    /// Returns the account status, either `Whitelisted`, `Blacklisted`.
    /// Returns None if the account doesnt have a status.
    pub fn account_status(&self, account: AccountId) -> Option<Status> {
        self.service_providers.get(&account)
    }

    pub fn authority(self) -> AccountId {
        self.authority
    }

    pub fn get_whitelist(&self) -> Vec<AccountId> {
        let mut whitelist = Vec::new();
        for (acc, status) in self.service_providers.iter() {
            if status == Status::Whitelisted {
                whitelist.push(acc);
            }
        }
        whitelist
    }

    pub fn get_blacklist(&self) -> Vec<AccountId> {
        let mut blacklist = Vec::new();
        for (acc, status) in self.service_providers.iter() {
            if status == Status::Blacklisted {
                blacklist.push(acc);
            }
        }
        blacklist
    }
    //
    // Authority Transactions
    //

    pub fn change_authority(&mut self, new_authority: AccountId) {
        self.assert_authority();
        self.authority = new_authority;
    }

    #[payable]
    pub fn whitelist(&mut self, accounts: Vec<AccountId>) {
        let storage_start = env::storage_usage();
        self.assert_authority();
        for a in &accounts {
            self.service_providers.insert(a, &Status::Whitelisted);
        }

        self.assert_deposit(storage_start);
    }

    #[payable]
    pub fn blacklist(&mut self, accounts: Vec<AccountId>) {
        let storage_start = env::storage_usage();
        self.assert_authority();
        for a in &accounts {
            self.service_providers.insert(a, &Status::Blacklisted);
        }

        self.assert_deposit(storage_start);
    }

    pub fn unlist(&mut self, accounts: Vec<AccountId>) {
        self.assert_authority();
        for a in &accounts {
            self.service_providers.remove(a);
        }
    }

    //
    // Private
    //

    fn assert_authority(&self) {
        require!(
            self.authority == env::predecessor_account_id(),
            "not an authority"
        )
    }

    fn assert_deposit(&self, storage_start: u64) {
        let required_deposit =
            (env::storage_usage() - storage_start) as u128 * env::storage_byte_cost();
        require!(
            required_deposit <= env::attached_deposit(),
            "not enought deposit to cover the storage"
        );
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, Balance, VMContext, ONE_NEAR};

    use crate::storage::Status;
    use crate::Contract;

    const START: u64 = 10;
    const MILI_NEAR: Balance = ONE_NEAR / 1_000;

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

    fn setup(predecessor: &AccountId) -> (VMContext, Contract) {
        let mut ctx = VMContextBuilder::new()
            .predecessor_account_id(authority())
            .block_timestamp(START)
            .is_view(false)
            .build();
        testing_env!(ctx.clone());
        let ctr = Contract::new(authority());
        ctx.predecessor_account_id = predecessor.clone();
        testing_env!(ctx.clone());
        (ctx, ctr)
    }

    #[test]
    fn assert_authority() {
        let (_, ctr) = setup(&authority());
        ctr.assert_authority()
    }

    #[test]
    #[should_panic(expected = "not an authority")]
    fn assert_non_authority() {
        let (_, ctr) = setup(&alice());
        ctr.assert_authority()
    }

    #[test]
    #[should_panic(expected = "not an authority")]
    fn whitelist_not_authority() {
        let (_, mut ctr) = setup(&alice());
        ctr.whitelist(vec![bob(), carol()]);
    }

    #[test]
    #[should_panic(expected = "not an authority")]
    fn blacklist_not_authority() {
        let (_, mut ctr) = setup(&alice());
        ctr.blacklist(vec![bob(), carol()]);
    }

    #[test]
    #[should_panic(expected = "not enought deposit to cover the storage")]
    fn whitelist_wrong_deposit() {
        let (_, mut ctr) = setup(&authority());
        ctr.whitelist(vec![bob(), carol()]);
    }

    #[test]
    fn whitelist_authority() {
        let (mut ctx, mut ctr) = setup(&authority());
        ctx.attached_deposit = MILI_NEAR * 10;
        testing_env!(ctx);
        ctr.whitelist(vec![bob(), carol()]);
        assert!(ctr.is_whitelisted(bob()));
        assert!(ctr.is_whitelisted(carol()));
    }

    #[test]
    #[should_panic(expected = "not enought deposit to cover the storage")]
    fn blacklist_wrong_deposit() {
        let (_, mut ctr) = setup(&authority());
        ctr.blacklist(vec![bob(), carol()]);
    }

    #[test]
    fn blacklist_authority() {
        let (mut ctx, mut ctr) = setup(&authority());
        ctx.attached_deposit = MILI_NEAR * 10;
        testing_env!(ctx);
        ctr.blacklist(vec![bob(), carol()]);
        assert!(ctr.is_blacklisted(bob()));
        assert!(ctr.is_blacklisted(carol()));
    }

    #[test]
    fn account_status() {
        let (mut ctx, mut ctr) = setup(&authority());
        ctx.attached_deposit = MILI_NEAR * 10;
        testing_env!(ctx);
        ctr.whitelist(vec![alice(), bob()]);
        ctr.blacklist(vec![carol()]);
        assert!(ctr.is_whitelisted(alice()));
        assert!(ctr.is_whitelisted(bob()));
        assert!(ctr.is_blacklisted(carol()));
        assert_eq!(ctr.account_status(alice()), Some(Status::Whitelisted));
        assert_eq!(ctr.account_status(bob()), Some(Status::Whitelisted));
        assert_eq!(ctr.account_status(carol()), Some(Status::Blacklisted));
        assert_eq!(ctr.account_status(dan()), None);
    }

    #[test]
    fn query_authority() {
        let (_, ctr) = setup(&authority());
        assert_eq!(ctr.authority(), authority());
    }

    #[test]
    fn change_authority() {
        let (_, mut ctr) = setup(&authority());
        ctr.change_authority(alice());
        assert_eq!(ctr.authority(), alice());
    }

    #[test]
    #[should_panic(expected = "not an authority")]
    fn change_authority_non_authority() {
        let (_, mut ctr) = setup(&alice());
        ctr.change_authority(alice());
    }

    #[test]
    #[should_panic(expected = "not an authority")]
    fn unlist_not_authority() {
        let (_, mut ctr) = setup(&alice());
        ctr.unlist(vec![bob(), carol()]);
    }

    #[test]
    fn unlist_authority() {
        let (_, mut ctr) = setup(&authority());
        ctr.unlist(vec![bob(), carol()]);
    }

    #[test]
    fn change_status_multiple_times() {
        let (mut ctx, mut ctr) = setup(&authority());
        assert_eq!(ctr.account_status(alice()), None);

        ctx.attached_deposit = MILI_NEAR * 10;
        testing_env!(ctx);

        // move alice to whitelist
        ctr.whitelist(vec![alice()]);
        assert_eq!(ctr.account_status(alice()), Some(Status::Whitelisted));

        // move alice to blacklist
        ctr.blacklist(vec![alice()]);
        assert_eq!(ctr.account_status(alice()), Some(Status::Blacklisted));

        // unlist alice
        ctr.unlist(vec![alice()]);
        assert_eq!(ctr.account_status(alice()), None);

        // move alice to blacklist
        ctr.blacklist(vec![alice()]);
        assert_eq!(ctr.account_status(alice()), Some(Status::Blacklisted));

        // move alice to blacklist
        ctr.blacklist(vec![alice()]);
        assert_eq!(ctr.account_status(alice()), Some(Status::Blacklisted));
    }

    #[test]
    fn get_whitelist() {
        let (mut ctx, mut ctr) = setup(&authority());
        ctx.attached_deposit = MILI_NEAR * 10;
        testing_env!(ctx);

        ctr.whitelist(vec![alice(), bob(), dan()]);
        assert_eq!(ctr.get_whitelist(), vec![alice(), bob(), dan()]);
        assert_eq!(ctr.get_blacklist(), vec![]);

        ctr.blacklist(vec![alice(), bob()]);
        assert_eq!(ctr.get_whitelist(), vec![dan()]);
        assert_eq!(ctr.get_blacklist(), vec![alice(), bob()]);

        ctr.unlist(vec![alice()]);
        assert_eq!(ctr.get_whitelist(), vec![dan()]);
        assert_eq!(ctr.get_blacklist(), vec![bob()]);
    }
}
