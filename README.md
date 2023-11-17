# NEAR Grant Protocol

This project was initiated during the NEARCon 2023 hackathon. [Submission](https://devpost.com/software/grant-596m4y).

Token used to distribute grants in the ecosystem and incentivize the grant recivers to reinvest withing the ecosystem.
The grant is first wrapped and packed into the `grant token` and then distributed to the grant recivers. If they want to unwrap the tokens they would only receive 80% of the original value. If they spend it with the verified service providers, they on the other hand can unwrap the 100% of the original value. The difference so lets say the 20% should go back to the grant provider.

## Why using MT token?

We need to distinguish tokens by granter:

- granter should be able to withdraw his tokens without being a subject of the penalty
- our long term goal is to use measure token velocity by granter
- each token has it's underlying value, and if we introduce more levels of penalty, we end up with tracking problems (people sending to each other grant tokens from different grants)
- token per granter allows us to easily measure performance of granter and grantee
- opportunity to test and work with the Multi Token template and had more hands on experience for MT standard we want to finalize.

## Example of the flow

Let's define the addresses we will use in the example:

```bash
# the grant token protocol we created and deployed, ready to use
CTR=grant-protocol.testnet
# a FT with self-mint function
FT=test-token.cheddar.testnet

# Some fake accounts
GRANTER=grant-provider.testnet
GRANTEE=grant-reciver.testnet
```

Some FT require prior registration. Our Multi Fungible Token also requires storage registration:

```bash
near call $FT storage_deposit '{}' --accountId $GRANTER --deposit 0.05
near call $FT storage_deposit '{}' --accountId $GRANTEE --deposit 0.05
near call $FT storage_deposit '{"account_id": "'$CTR'"}' --accountId $GRANTER --deposit 0.05

near call $CTR storage_deposit '{}' --accountId $GRANTER --deposit 0.05
near call $CTR storage_deposit '{}' --accountId $GRANTEE --deposit 0.05
```

Now the granter can create his grantToken by supplying FT to the protocol. It will wrap the FT into a MT.

```bash
near call $FT ft_transfer_call '{"receiver_id": "'$CTR'", "amount": "200000000000000000000000", "msg": ""}'  --accountId $GRANTER --depositYocto 1 --gas 300000000000000
```

Once the granter has his grantTokens, he can distribute them:

```bash
near call $CTR mt_transfer '{"receiver_id": "'$GRANTEE'", "token_id": "1", "amount": "10000000000000000000000"}' --accountId $GRANTER --depositYocto 1
```

The grant receiver then can immediately unwrap the token calling. This will cost him a 20% penalty:

```bash
near call $CTR unwrap '{"token_id": "1", amount: "10000000000000000000000"}' --accountId $GRANTEE
```

Otherwise, to keep 100% value, the grantee can spend it in the ecosystem, by sending it to any whitelisted service provider.

```bash
near call $CTR storage_deposit '{}' --accountId design-studio.testnet --deposit 0.05

near call $CTR mt_transfer '{"receiver_id": "design-studio.testnet", "token_id": "1", "amount": "10000000000000000000000"}' --accountId $GRANTEE --depositYocto 1
```

Whitelisted accounts can call `unwrap` and get 100% of the underlying value.

Service provider can
