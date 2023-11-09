# near-grant-protocol

Token used to distribute grants in the ecosystem and incentivize the grant recivers to reinvest withing the ecosystem.
The grant is first wrapped and packed into the `grant token` and then distributed to the grant recivers. If they want to unwrap the tokens they would only recive 80% of the origianl value. If they spend it with the verified service providers, they on the other hand can unwrap the 100% of the original value. The difference so lets say the 20% should go back to the grant provider.

## Example of the flow

1. Register into FT contract

```bash
    near call test-token.near storage_deposit '{"account_id": "alice.near"}' --accountId alice.near
```

2. The grant provider wraps the token

```bash

near call test-token.near ft_transfer_call '{"receiver_id": "grant-token.near", "amount": "200000000000000000000000", "msg": ""}'  --accountId grant-provider.near --depositYocto 1 --gas 300000000000000

```

3. The grant provider send the `grant token` (the grant token can be send inbetween accounts using the same function)

```bash
near call grant-token.near mt_transfer '{"receiver_id": "grant-reciver.near", "token_id": "1", "amount": "10000000000000000000000"}' --accountId grant-provider --depositYocto 1

```

4. The grant reciver then can unwrap the token calling

```bash
near call grant-token.near unwrap '{"token_id": "1", amount: "10000000000000000000000"}' --accountId grant-reciver.near
```
