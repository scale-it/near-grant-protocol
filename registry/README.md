# Registry

Registry to keep track of accounts status for the `grant token`. We divide the accounts into 3 different categories:

- `Whitelisted` accounts that are verified and able to reedem 100% of the original value of the `grant token`.
- `Blacklisted` these accounts are not able to redeem the `grant token`.
- `Non-listed` accounts that have not been verified and are able to redeem only 80% of the origial vlaue of the `grant token`.

## Queries

- `is_whitelisted(account: AccountId) -> bool`
- `is_blacklisted(account: AccountId) -> bool`
- `account_status(account: AccountId) -> Status`
- `authority() -> AccountId`
- `get_whitelist() -> Vec<AccountId>`
- `get_blacklist() -> Vec<AccountId>`

## Authority Transactions

- `change_authority(new_authority: AccountId)`
- `whitelist(accounts: Vec<AccountId>)`
- `blacklist(accounts: Vec<AccountId>)`
- `unlist(accounts: Vec<AccountId)`
