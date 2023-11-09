# near-grant-protocol

Token used to distribute grants in the ecosystem and incentivize the grant recivers to reinvest withing the ecosystem.
The grant is first wrapped and packed into the `grant token` and then distributed to the grant recivers. If they want to unwrap the tokens they would only recive 80% of the origianl value. If they spend it with the verified service providers, they on the other hand can unwrap the 100% of the original value. The difference so lets say the 20% should go back to the grant provider.

## Example of the flow

1. Register into FT contract

```bash
    CTR=grant-protocol.testnet
    FT=test-token.cheddar.testnet
    GRANTER=grant-provider.testnet
    GRANTEE=grant-reciver.testnet

    near call $FT storage_deposit '{"account_id": "'$GRANTER'"}' --accountId $GRANTER --deposit 0.05
    near call $FT storage_deposit '{"account_id": "'$GRANTEE'"}' --accountId $GRANTEE --deposit 0.05
```

2. The grant provider wraps the token

```bash
near call $FT ft_transfer_call '{"receiver_id": "'$CTR'", "amount": "200000000000000000000000", "msg": ""}'  --accountId $GRANTER --depositYocto 1 --gas 300000000000000

```

3. The grant provider send the `grant token` (the grant token can be send inbetween accounts using the same function)

```bash
near call $CTR mt_transfer '{"receiver_id": "'$GRANTEE'", "token_id": "1", "amount": "10000000000000000000000"}' --accountId $GRANTER --depositYocto 1

```

4. The grant reciver then can unwrap the token calling

```bash
near call $CTR unwrap '{"token_id": "1", amount: "10000000000000000000000"}' --accountId $GRANTEE
```
