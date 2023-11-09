# near-grant-protocol
### Hackathon idea

# GRANT, a Circular Economy Token for the Near Ecosystem

***Date: Oct 24, 2023***

***Author: KazanderDad***

***Also available as a [Google Docs](https://docs.google.com/document/d/15tJIha_76frcSC1zgPJlAFzVcysSzVJ2e05M226nMaA/edit?usp=sharing) and [on HackMD](https://hackmd.io/@Kazander/Hk1Q0b5ma)***

Originally written for NearCon Hackathon 2023


# 1. Overview

GRANT is a semi-fungible token that’s used for giving grants to projects. It promotes spending the GRANT within the Near Ecosystem rather than offramping or swapping it. GRANT could also be well suited for UBI or Citizen Salary to the humans on Near, with it's "soft lock-in" that prevents price dumping.


## 1.1 Introduction

The main idea is to both prevent grant-and-dash and to encourage the creation of a healthy circular economy within Near by distributing GRANTs and requiring all grant recipients to spend the GRANT token within the Near ecosystem before it is “unlocked” and can be offramped. 

The way we achieve this is by building the following features:

- Creating the GRANT token as a wrapped USD-denominated StableCoin (and possible also by wrapping Near and other stables)
- Creating a blacklist of accounts that are prohibited from using GRANT, which would include: all known exchanges, dexes, CEX accounts, off-ramps, liquidity providers, stakers, and node operators
- Creating a whitelist of accounts that can unwrap GRANT tokens back to USDC, which would include known service providers in the NDC ecosystem, such as dev shops, marketing agencies, lawyers, creative agencies, and other service providers which have been sanctioned and approved by the NDC community. We could seed or sync this list with the approved providers from the Near Horizon program. 
- Funding a whistleblower bounty for the community to report new suspected swap/bridge/off-ramp accounts to add to the blacklist. The funding for this can come directly from NDC or from fees charged from grantees or from whitelist applicants. We would also add a stake requirement for whitelisted accounts, with the stake up for grabs (slashing) for any whistleblower who can prove that a whitelisted entity misbehaves. 

![](https://lh7-us.googleusercontent.com/czMlQyNde-T7LO9E_VbZTkyVEHQT0uwcgjaeAIMsejZyzZBAN6-YyXVgnrUg_WZnw-JbDAusbvUI9yynlkIls81Ge345KTl_kgDj2Nv-Ujz7JTN6Im55y1muJeBmzLT4psvQ7562z1B4UPw-ImOG4Bs)


## 1.2 Use Cases

Any grant-giving or bounty-giving scenario would benefit from issuing their grants or bounties as GRANT tokens rather than regular USDC or Near. It is also a great token for user acquisition scenarios, such as issuing Universal Basic Income or paying a Citizen Salary to Nearians who have qualified for it.


## 1.3 Prior Art

This program is similar in nature to the **Near Horizon** program in two distinct ways:

- The Near Horizon Credit system was intended to be used this way, except it was centrally managed in a spreadsheet, whereas our GRANT token is decentralized and managed on-chain
- Their whitelist of service providers is similar or the same as ours. Our NDC whitelist can most likely be seeded by the current Horizon whitelist, or we can collaborate and simply make it one and the same going forward too. 

The lockup of GRANT tokens is also similar in function to how **KeyPom** allows projects to issue Near to new users with new accounts, where recipients have to spend these Near on a limited set of allowed use cases before their account is fully unlocked. However, GRANTS are different in that almost all use cases are allowed, with the exception of a certain set of blacklisted accounts. GRANTs can also be used both with new and previously existing accounts. In these ways GRANTs are more liquid and more widely useful / less restrictive than KeyPom-issued Near.


## 1.4 Key Concepts

Introduction of four new concepts

- **FractionID**: A regular fungible token is mainly characterized by an array of  HolderAccounts, each of which has a token balance and where the sum of all Holder Account balances equal the total minted amount. The GRANT Token is characterized instead by two arrays, adding a FractionID array. The sum of all FractionID balances will also always equal the sum of all HolderAccounts and the total minted amount. Each FractionID has not only a balance but also a CirculationEventCounter and a WhitelistID identifier, which is the last WhitelistID that incremented the counter for that Fraction. 
- **Wrapping and Minting**: anyone can wrap the underlying USDC token (or Near?) and mint a corresponding GRANT token. (Or we could use other techniques than wrapping, such as backing)
- **Whitelist**: list of service providers and their accounts for which the Unwrapping function is available 
- **Blacklist**: list of accounts for which transfers are not eligible. GRANT tokens cannot be transferred into these accounts. If GRANT tokens were in the account at the point of blacklisting it, then these tokens are frozen. 


## 1.5 Benefits of Using GRANT

The benefits of this GRANT token on the ecosystem are

- Grantees are encouraged to spend their grants within the ecosystem rather than outside. (They are prevented from swapping, offramping and staking their grant)

- NDC gets an added purpose: to vet and qualify service providers that are acting in the interest of Near

- Grantors get significantly enhanced reporting on and knowledges about where their grants went and how their grants were deployed and went to benefit the whole ecosystem 

- Qualified Service providers gets

  - Additional boost in revenues, since all grantee funds will (eventually) make it to one of the whitelisted service providers
  - Increased status and visibility in the community as  whitelisted provider

- The community benefits from

  - Significantly higher velocity in our circular economy
  - Increased demand for services
  - Clarity in what promoted behaviors are (the establishing of whitelisting guidelines)
  - Easy to find whitelisted service providers with a “stamp of approval”
  - Reduced sell pressure on the Near token (if GRANT is backed by Near)
  - Unlocking of new use cases that’s previously been impossible to achieve on Near (such as UBI or Citizen Salary)

- Community members get the opportunity to work within the program and make money, either by

  - Submitting whistleblower reports 
  - Working as an Arbitrator
  - Working as a program administrator

With the GRANT token we could unlock scenarios that were previously thought hard to implement due to grifters grabbing grants and benefits through a minimum amount of effort, and then disappearing without contributing to the Near Ecosystem. Examples include

- NDC grants to new grassroots projects with new founders who would have previously not been approved because they weren’t “trusted enough”. The inherent design of GRANTS makes it very suitable for previously untested founders. 
- Citizen Salary or UBI. One of the main arguments against creating a universal basic income program is often that “people would just take the money and leave”. With GRANTs we would know that the money would end up being spent on useful services within the ecosystem
- Etc.

![](https://lh7-us.googleusercontent.com/mOBFsiZSsKrr_Vgh3wUu5vvDfpZYEewZwrFmI3wAA6vNfSd7o2CeGuf1MY-g64oTRQCUwkYcZkRSpIfwlfNI5aq4zV0BUloIQM7cOB4I5RvygilUyfhV0zcYTH0iKBz0M9j-hGha0Wjm89fhY6Jp2Xs)


## 1.6 Wrapping + Minting GRANT

There is no initial mint of GRANT. All minting will stem from Grantors deciding to fund a grants program with underlying tokens such as USDC, and then to wrap such underlying tokens to mint GRANT.

Running a grants program with GRANT tokens is open to everyone in the ecosystem. We can make  functionality open and available to anyone to fund grants, wrap Near tokens or USDC tokens into GRANT tokens, and run a grant program and issuing grants. 

We would expect NDC to be the main Grantor, but use of GRANT is not limited to just NDC. 

Benefits to the Grantors include support with treasury management, much better reporting on how their grants were used, and other benefits to the ecosystem as listed above. 


## 1.7 FractionIDs

FractionID is an array in the SmartContract. Each Fraction has the following variables at minimum:

- hex: **FractionID**, a simple UID or counter to identify the Fraction
- uint: **TokenID**, defining which token the Fraction corresponds to
- uint: **Balance**, where sum of all FractionID balances will also always equal the sum of all HolderAccount balances, and also equal the total minted amount
- uint: **CircularEvents** counter
- account: **WhitelistID** identifier, which is the last WhitelistID that incremented the counter for that Fraction

One FractionID can at certain points be split into two separate FractionIDs This will happen during Transfer events where the transfer amount is less than the total available amount of the sender.  In such a case the FractionID is split into an old part (which retains the existing ID), and a new part (which gets a new and unique FractionID). 

- The balance of the new part is set to whatever fraction is required to be transferred to the recipient. The new part also inherits the prior attributes such as the CirculationEventCounter which is then possibly also incremented by one (if the recipient is whitelisted and different from the last recipient).
- The old part retains the attributes except the balance which is decremented by the same amount that is given to the new part. 

Other characteristics

- FractionIDs can never be merged, removed, inactivated or otherwise obsoleted

- The Balance of a FractionID can only be incremented once, which is when it’s created 

  - either by minting it or 
  - as a result of splitting one FractionID into an original and a newly created fraction, which would only happen during certain Transfers where the transfer amount was not an even function of available fractions

- The Balance of a FractionID can only be decremented as a result of 

  - an Unwrap function or 
  - as a result of a split which reduces the Balance of the original fraction, which would only happen during certain Transfers where the transfer amount was not an even function of available fractions


## 1.8 Transferring GRANT Tokens

GRANT should be designed to comply with the fungible token standard, such that sending tokens is as simple as using the transfer / send functionality of any wallet. 

Exception: when a blacklisted account os either the sender or the recipient then the contract does not allow this transaction. The best implementation of this would be to disable the send button for blacklisted accounts. A worse implementation would be to have the contract panic upon executing the transfer transaction. 

It’s important that each transfer event updates both 

- the balances of the Account Holders such that the total remains constant, and 
- the balances of the FractionID such that the total remains constant


## 1.9 Blacklisting

We would “blacklist” any known accounts belonging to protocols engaged in swapping and/or offramping. This is not to say that swapping and offramping isn’t allowed in the ecosystem. It is only to say that swapping and offramping are not allowed within the GRANTs program. 

We would then make a bounty available for the community to find, gather evidence against and report any pop-up swap sites. 

Management of the blacklist should be decentralized as far as practically possible. We can accomplish this by building community decision-making tools to allow democratic election or appointment of “blacklist arbitrators” which would be a new role, and then have these arbitrators apply open and transparent criteria when making their decisions on which accounts to blacklist. We would look for consensus or absolute majority amongst arbitrators before blacklisting an account.


## 1.10 Whitelisting and Unwrapping

Whitelisting of vendors and service providers is voluntary and subject to an application that must be reviewed and approved. Whitelisting is done by a new role called “whitelist application processors,” funded by the program. 

Whitelisted service providers get access to the main feature of this project: They are the only ones that can unwrap the GRANT tokens and extract the underlying USDC or Near 

Whitelisted businesses therefore will enjoy increased business. They are marketed and promoted on the project homepage. Projects with funds available (i.e. all grantees) will want to spend their funds with whitelisted providers over non-whitelisted providers. The reason for this is that whitelisted providers will accept GRANT tokens at par value or better, while non-whitelisted providers might be more likely to only accept GRANT tokens at some price below par.  

Management of the whitelist should be decentralized as far as practically possible. We can accomplish this by building community decision-making tools to allow democratic election or appointment of whitelist processors, and then have these processors apply open and transparent criteria when making their decisions on which businesses and which accounts to whitelist. We would look for consensus or absolute majority amongst processors before whitelisting an account.


## 1.11 A case against liquidity

Liquidity in a token is often seen as a good thing. We want to provide an ecosystem with deep liquidity to make Near attractive to investors. Its important for a DeFi protocol or an NFT marketplace that there be deep liquidity available on a blockchain before they decide to invest in building on it. 

However, liquidity is also detrimental. Whenever a newer, shinier protocol becomes available, then having a highly liquid protocol with more responsible yield levels is a disadvantage. Investors can sell fast and move away to a chain where yield is temporarily higher. We saw this when Terra/Luna held their yield artificially high for a long time. It becomes a race to the bottom, and it’s detrimental to the more responsible chains and protocols. 

The real health of an economy is measured in the amount of trade that goes on, not only as measured by it’s top three protocols but by the diversity of protocols and how well they synergistically interplay. 

So how do we define our ecosystem that we wish to measure the health of? 

- We could define it as all global economies, both fiat and crypto, in which case on/off-ramps become important and good, and the globe becomes a better place if we all got along. Freedom to move money fast anywhere should be celebrated, even if it’s selling all Near to USD. 
- We could define all blockchains as the economy we wish to make healthy, in which case cross-chain bridges are important and all chains compete together against fiat. The freedom to sell all Near for Bitcoin should be celebrated. 
- Or we could define Near as the ecosystem we wish to measure the health of. Then keeping the Near tokens within the chain becomes of strategic importance. This is the scenario that we are proposing to defend. Creation of marketplaces for various services within the Near ecosystem will be of vital importance to give users plenty of options where to spend their money. And likewise, encouraging them to seek out and use such marketplaces is what we can do by requiring that tokens are spent within the ecosystem before they can be unlocked and swapped/offramped. 

The case against staking and locking away tokens is harder to make. At first glance, it seems that a staked token is a less liquid token, which would be good for price action. But our argument is that if locking tokens away was the objective then leaving them in a Community Treasury and staking the treasury would simply be the best option. When we give grants to projects this is because we wish to help them to buy the services they most need, not for them to stake the tokens. We want to encourage a circular economy, and the best way of doing this is to first give grants to projects that need to buy service, and then encourage them to actually buy those services. Rather than staking and locking the tokens away. A circular economy is good for all ecosystem participants. High velocity of money is good. Only dying or stagnating protocols target high TVL and thereby reduced velocity of money. 


# 2. Design Options

## 2.1 Wrapped vs backed

The simplest option would be to just create GRANT as a new token that is not a wrapped token. This could be guaranteed by the NDC and backed by one of theree options

- The whole of the Community Treasury, or
- A dedicated and separate account that is always funded with an amount of stables that is equal to the amount of outstanding tokens, or
- A dedicated and separate account that is always funded with an amount of Near that is worth more than the amount of outstanding tokens. This account would have to be topped up whenever the price of Near decreases. 

However, it would be better if we built it as a decentralized and trustless wrapped token instead. This design is more aligned with the principles of web3 to decentralize, building self sovereign and trustless solutions. 

Making GRANT a wrapped token would also make it more resistant to the whims of politically assigned leaders who might want to defund the program. With a wrapped token there is no option to defund the program by removing liquidity. The only thing elected politicians can do is then to stop adding to the program. 

Wrapped

- Benefits: 

  - Trustless, unstoppable, decentralized
  - Value is known

- Drawbacks: 

  - Stranded funds are lost forever. 
  - More complicated to build.

Backed

- Benefits: 

  - Stranded funds can be recovered. 
  - Simpler to build. 
  - Value is known (assuming good design or reserves)

- Drawbacks: 

  - Centralized, not trustless, not unstoppable. 

Bonding curve

- Benefits: 

  - Stranded funds don’t subtract from the value. 
  - Simpler to build. 
  - Trustless, unstoppable, decentralized

- Drawbacks: 

  - Value is fluid, future value is unknown. 

_Decision: Make GRANT a wrapped token._ 


## 2.2 Underlying: USDC or Near?

We might need to choose between backing the GRANT token with Near or with a StableCoin:

- Wrap Near. 

  - Benefit: reduced sell pressure on Near. 
  - Drawback: grantees prefer stables. 

- Wrapped StableCoin such as USDC

  - Benefit: grantees prefer stables.
  - Drawback: increased sell pressure on Near. 

A third option could be to create both options, and let the grantor /  grantee choose their stipend. 

- Some grantees might prefer a more stable token, which might also mean a grant that’s somewhat smaller
- The Grantor / NDC will likely prefer to wrap Near, seeing how it would have a better impact from their perspective by significantly delaying the sell pressure, and they might be inclined to increase the grant somewhat for grantees who accept this option

Consideration: We would very much like to allow for multiple tokens as described in the third option. However, there is a significant risk of creating too much complexity in one smart contract, especially considering how each FractionID would have to be attributed to one particular TokenID. Adding both these entirely new and groundbreaking complexities to the same SmartContract might be too much…?

**_Decision: TBD. Allow for multiple different underlying assets, differentiated by a TokenID, if possible (which is what this document assumes, for now)._** 


## 2.3 Delay mechanism (optional)

In addition to requiring the token to be circulated before unwrapping it, we could also add a lockup effect. One way to achieve this could be to combine the unwrap event with a token lock function, such that upon unwrapping then the unwrapped USDC is locked up for a period of five years or so. This would further incentivize the holders to want to keep circulating the token rather than offelramoing / unlocking it. 

(Another way of introducing the delay mechanism would be to lock the underlying token before wrapping it. This is a much more complicated solution though, as it would have to be built as a semi-fungible token, where each token “batch” would have a different unlock date. A recipient would need to value each batch differently, introducing an unwarranted complexity into the transactions.)

If the wrapped token is Near then a delay mechanism would also decrease the sell pressure. 

The drawback of both of these delay mechanisms are a that it introduces further resistance towards the token, making recipients value it less in comparison to the liquid underlying token, meaning that our token might be traded at a value somewhere below the underlying token. 

**_Recommendation / Design Decision:_** Don’t implement any further lockup / delay implemented in version 1 of the GRANT token. 


## 2.5 Token Design

### CirculationEventCounter options

Require each token to circulate either once or several times before it’s eligible to redeem against StableCoin?


#### Option A: Circulate once

Simple to build. No need to count CirculationEvents. As soon as the token ends up with a whitelisted provider it’s eligible to be unwrapped.

Benefits: less resistance to unwrapping means the token might be more accepted at par value by anyone and everyone in the ecosystem, not just whitelisted providers. Good protection against grant-and/dash. Easier to build. 

Drawbacks: less resistance to unwrapping means the lockup effect might not last very long. As soon as grantees spend their money once then the token is unwrapped. No real creation of a circular economy. 


#### Option B: Circulate N Number of Times: 

Before a token can be redeemed/unwrapped then it has to be circulated for example five times.

How to implement: each time the token changes hand / is transferred to a new whitelisted account then it is counted as a Circulation Event. 

Benefits

- Better lockup effect
- Significantly enhanced stimulation of a real circular economy
- Somewhat better protection against grant and dash
- More business for the ecosystem-aligned service providers
- Reduced sell pressure on Near (if the underlying is Near)

Drawbacks

- Harder to build. Requires the token to be a semi-fungible token, where each transfer is counted uniquely for each token. If the token is split to be used for a partial payment, then only the fraction that’s sent is incremented while the remaining fraction is not incremented in terms of Circulation Events. 
- Only transactions where the recipient is a whitelisted account that’s different from the previous circulation recipient can be counted as a Circulation Event. The reason for this is that we need to prevent “artificial” transfers created simply to trick the system. 
- Decreases the liquidity and increases the user complexity, both of which makes the token less desirable as a means of trade and might thereby decrease the value of the token relative to the underlying liquid token. Particularly so for tokens that have been circulated few times. 

Note: This is a fundamental design decision. It will not be simple or even possible to start without circulation requirements and then plan to add this feature later. If it is ever to be an option then we should build this feature from day one.  

**_Recommendation / Design Decision: Design the GRANT token for an initial circulation threshold of three circulations before enabling unlock._** 


### Semi-fungible token design

_Note: I’m assuming that option B would require the use of a semi-fungible token?? I’m not even sure this would be possible. But let’s try._

The circulation count formula would require each token batch to “have a memory” of these data points:

- BatchID / FractionID
- Date and batch of minting (optional)
- Account of the minting Grantor (optional)
- Account of the first recipient = Grantee (optional)
- CirculationEventCounter (required)
- The last prior whitelisted holder of the token, which also generated the last CirculationEventCounter increment (required)


### FractionID design

The biggest invention in this token is the introduction of a fraction ID. We need this to enable the memory function for the circulation counter. 

The simplest way to implement a fraction ID would be to replace the account array of the standard fungible token with a fraction ID array of tuples, where each tuple includes the account holder, the circulation counter, and the balance. 

However, we need backwards compatibility with the fungible token standard and we need to be able to show the token within all wallets without having to ask them all to support a new standard. There are two main ways of achieving this backward compatibility.


#### Option 1: Replace the balance array

Replace the account holder array with a fraction ID array as described above and only maintain balances in this new array. Then replace the token balance functions: Where the old token balance function would simply query the account array for one account (self), it would now instead iterate over all the fraction IDs held by self and sum up the total.

Strengths: this is the cleanest way of implementing it, with the least amount of risk and mismatch of balances. Since there is only one source of truth for account balance, i.e. the fraction ID array, this would be the best way of maintaining account balances. Lower lower gas fees for transactions, and higher storage cost. 

Drawbacks: Each account balance query would be more computationally heavy and provide a slower response. Not sure how complex exactly it would be: 

- Can do a select query? 
- Or do we have to iterate across all FractionIDs each time? 
- Or, if wallets rely on indexers for their balance queries, then it shouldn’t be a large problem at all…


#### Option 2: Two separate balance arrays

Implement the new fraction ID array in parallel to the account holder array, and keep all balances twice. In other words, maintain the balance of each account holder in two separate arrays, both of which are always incremented or decremented in parallel.

Strengths: this provides the best backwards compatibility with the fungible token standard, and also responds faster to a token balance query. It is less computationally heavy for a token balance query.

Drawbacks include higher gas fees for transactions and higher storage costs

(This option is how the token functions have been pseudo-coded in section 4 below.)

**_Recommendation / Design Decision:_** tbd 


### FractionID Priority?

If a wallet holds several different FractionIDs of a token, which ones do we send out first? And if a FractionID must split into one partial Fraction to send and a remainder to retain, then which one of these gets to keep the old FractionID and which gets a new FractionID? These choices should NOT be asked of the user, they should be as automated as possible. We want to make these tokens as similar to fungible tokens as possible to the user. 

- **Batch splitting:** If a grantee has one whole Fraction worth $100 from their grant and wants to make a first purchase of $20 from a whitelisted provider, then the grant batch must be split into one Fraction of $20 which will increase circulation count by 1, and a remainder Fraction of $80 without an increase in circulation count

  - The Fraction that **remains** ($80) should retain the **original** FractionID
  - The Fraction that is **transferred** ($20) should be given a **new** unique FractionID

- **Batch grouping**: If a service provider sold services of $20 to three different grantees then the service provider will afterwards have three batches of $20 each for a total of $60. If they wish to make a $55 purchase from a different provider, then the wallet logic should take two batches of $20 and split the last batch into one part of $15 and keep another part of $5 unspent remainder. We will design a system of priority that’s designed to get Fractions progressed toward an unwrappable state as fast as possible, and also minimizes the risk of creating remaining dust of microscopic fractions, which would increase storage cost:  

  - First of all, if the sender is Whitelisted and if there are Factions that are unwrappable (CirculationEventCounter >= Threshold), then these Fractions shoul be placed last in the queue (i.e. for whitelisted senders only, prioritize not-yet unwrappable fractions)

  - Thereafter, 

    - If the recipient is whitelisted, the Fractions with the smallest balance should be picked first for transfer, regardless of the current Circulation count (to ensure we move all the dust towards unwrappable state first). 
    - Else ifthe recipient is not whitelisted then instead pick the largest fractions first (to minimize gas fees)

  - If ever the choice is between two Fractions with identical balance, then the oldest Fraction (with the lowest FractionID) should be picked first for transfer.


## 2.5 How the blacklist would work

The basic case is that a blacklisted account should not be able to receive the token. 

A more complicated question is what to do with the tokens already in the account at the point in time the account is blacklisted. This would be the case for accounts that weren’t seeded into the blacklist but rather was a result of a whistleblower report. We have to decide what’s the best way to handle these. Options are:

- Burn the tokens and release the underlying 

  - back to the grantor
  - to the central fund for whistleblowers and workers

- Move the tokens

  - to the grantor 
  - to the central fund

- Lock the tokens in the blacklisted account


### What’s blacklisted: the account or the tokens?

Are we painting the blacklisted tokens, or just locking up the account? Should a blacklisted account still be able to receive and send whitelisted tokens after the blacklisting event? The simple solution would be to just blacklist the full amount of a blacklisted account by simply disabling the send function for such an account.  

If the answer instead is that it’s the tokens that are painted black then we need to maintain two balances, one for regular tokens and another for black painted tokens for each user. The “send” function would be blocked for black tokens but not for regular tokens. 

**_Decision: The simplest and most intuitive answer here is to blacklist the account and freeze the tokens that might be in it. We’re NOT going to paint the individual tokens._** 


### Blacklisting: permanent or revocable?

The blacklisting also has an impact on how the underlying token should be treated. If a blacklisting event is permanent then the underlying token should be made available to the Pool administrator (grantor), or used to fund the whistleblower bounty pool. If on the other hand blacklisting is revokable then the underlying tokens should remain locked up. 

**_Decision: TBD_**


## 2.6 How the Whitelist would work

Any account that’s added to the whitelist gets two new features unblocked: 

1. the circulation counter is now going to be incremented for GRANT tokens that they receive, and 
2. they now have access to unwrap their GRANT tokens as long as the CirculationCount is equal to or larger than the Threshold value. 

Whitelisting should be available to service providers who are “Near native”, having served the Near ecosystem for some time with services to help new projects. Service providers wishing to whitelist should comit to the following

- Act in the best interest of the Near ecosystem
- Not offer any services related to swapping, bridging, off-ramping, staking or similar
- Only accept GRANT tokens as payment for services rendered in the category of service that the provider was whitelisted for (eg legal service for a legal provider)
- Accept the GRANT tokens as payment at face value (or above, but never below), where face value = the value of the underlying wrapped token
- Provide some form of discount to grantees buying their services and paying with GRANT tokens
- Not artificially circulate GRANT tokens. Only use them to pay for legitimate services and as allowed by this program

**_Decision: Specific criteria TBD_**


## 2.7 Incentive Model

There are some incentives we would need to fund. These are discussed individually visually elsewhere in this paper, but a long listing would include. 

- Whistleblower incentives 

  - For reporting general accounts engaging in unwanted activities, such as swaps/DEX/CEX/etc 
  - For reporting a whitelisted account that offers forbidden services or has engaged in forbidden behaviours

- Paying blacklist arbitrators for their work

- Paying whitelist application processors for their work

- Marketing the program to attract more Grants Grantors and fundraising

- Paying general support and admin staff for their work 

Money for this can come from a variety of sources

- Bonds and fees extracted from whitelist applicants
- Fees extracted from grantees
- Fees extracted from grantors
- Fundraising / grants from NDC or other sources such as Hackathos
- Fees extracted from each transaction, similar to a gas fee

**_Decision/Recommendation:_** 

- **_Charge an initial 2% minting fee from Grantors to pay the arbitrators and processors, and to fund a bounty to pay to whistleblowers_** 
- **_Charge a $100 whitelist application fee which is forfeited for unsuccessful applicants and paid to the admins, and which is added to a blacklist stake for succesful applicants._**
- **_Charge a 1% unwrapping fee which is used to pay the project admins to keep the program running and evolving_**


## 2.8 Role Granularity

We have envisioned thee main central roles:

- Blacklist Arbitrator
- Whitelist Application Processor
- Program Admin / Grants Grantor

However it is possible to simplify and just create one super admin role, which could be given to a DAO. Or to further split the roles into more granular parts. 

Blacklist Arbitrators might have certain skill sets or want to focus on certain categories of blacklisted activities

- CEXes
- DEXes
- Off-Ramps
- Bridges
- Stakers
- Etc

Whitelist Application Processor could be broken down into various categories of businesses, assuming a user might be more adept at processing one category over others:

- Legal services
- Marketing services
- Dev services
- Etc

Grants Grantor responsibilities could be broken into sub tasks following the required process steps:

- Funding the pool(s)
- Wrapping pools into tokens
- Approving grantees
- Approving grant milestone deliveries
- Sending grants to grantee approved / milestone completed
- Reporting of funding ratios & TVL
- User support desk
- Etc

It is also possible to split the Grants Grantor role down by type of grant

- Central grant given by Grassroots DAOs such as CDAO, MDAO
- Grants given by quadratic funds such as the Community Fund or the Potlock Fund
- Universal Basic Income for community members

**_Decision/Recommendation: TBD_**


## 2.8 Token Expiry

We want to create this token both to enable better grant giving, and also to kickstart a circular economy within Near. We could imagine  building an “expiry” function into the token to further encourage the circular economy and increase the velocity of the token. If we did, then for example, if the token sits untouched for some length of time, for example more than a year, then it would “expire” or be deleted.

This gives rise to a number of follow on design questions

- Which transactions would reset the expiration counter?

  - Transactions to a whitelisted account obviously would reset the counter
  - Transactions to any account should probably reset it. If an owner of GRANT is happy to hold on to it as a reserve for a planned future payment, but worried that they might disappear, then they should be able to hit the reset button without spending it. They way they would do this is to send the tokens between two of their own accounts. 

- How fast would it expire

  - 3 months?
  - 6 months?
  - 12 months?

- How to implement the expiry without requiring a new transaction

  - Do we need two different statuses, one for a tentatively expired but not yet unwrapped and another for expired and unwrapped.

- Should a user be able to transfer a tentatively expired token, and thereby reset the counter / unexpire the token?

  - Yes, probably the best scenario would be that if a Grantor didn’t yet unwrap a tentatively expired token, then the user can still transfer it and thus un-expirte the token

- Who would the wrapped underlying go back to?

  - Probably back to the Grantor

- Can we unwrap the underlying liquid token without a user transaction?

  - There should be a balance of tentatively expired tokens which are available for unwrapping by the Grantor, who can then decide to create a transaction to unwrap all such token, or decide to not do this.

- How would expiry impact token balance?

- How would expiry impact future transfers?

- How would the existence of expiry impact the logic? 

  - Both transfer and unwrap transactions would have to always prioritize the oldest FractionID. Preventing inadvertent expiry is more important than preventing a tail of small amounts.

- Etc

**_Decision/Recommendation: TBD. (Not in favor of expiration)_**


# 3. Required Widgets & Functions

Widgets should be organized by role and task at hand

- Wallets for simple sending & simple balances

- Wrapper/Unwrapper for what wallets can’t do:

  - Show balance by circulation count or unwrappable / wrappable
  - Show account status (whitelisted / blacklisted / normal)
  - Allow wrapping
  - Allow unwrapping (if whitelisted)
  - Multisend (optional)

- Admin widget 

  - Show account status (whitelisted / blacklisted / normal)
  - Allow whitelisting / un-whitelisting
  - Allow blacklisting / in-blacklisting 

- Super admin widget not needed, can be CLI


## 3.1 Blacklist Whistleblower Widget

Blacklisting is initially a semi-centralized activity but relies on the community to report suspected instances for centralized review. 

Creating a whistleblower report is available to anyone in the community. It could optionally require a stake of 10 Near, which would be slashed if the report is later deemed spam. 

The tentative UI of the whistleblower widget could include the following components:

- Title Bar:

  - Bold text displaying: "GRANT Whistleblower Report".

- Report Body:

  - Label: "Report Description"
  - Text area for users to input the main content of their report. Placeholder: "Describe the suspicious activity…"

- Reported Accounts Section:

  - Label: "Reported Account(s)"
  - Text field with placeholder: "Enter account name"
  - After the first account is entered, a button labeled "Add another account?" appears below the text field. When clicked, it spawns another identical text field for the next account.
  - Beside each account field, display the current status of the reported account and a clickable link labeled "View Previous Reports", which when clicked, lists prior whistleblower reports against that account.

- Bond Amount Section:

  - Label: "Bond Amount"
  - Text displaying: "Required stake of 10 Near (refunded if report is valid, slashed if deemed spam)."
  - Checkbox for users to agree to the bond terms. Label next to checkbox: "I agree to stake 10 Near."

- Submission Section:

  - Button labeled "Submit Report". Upon clicking, a confirmation pop-up appears with the message: "Your report has been submitted and will be reviewed by arbitrators. The reported accounts remain functional until a decision is made."

- Privacy Note:

  - Text displaying: "Note: Your report will be encrypted to preserve your privacy. Only our Blacklist Arbitrators can decrypt the report. Your identity will not be disclosed."

As a support to the whistleblower, we should include the current status of the reported account, and a count of any existing prior whistleblower reports against the reported account(s)

Upon submission the reported accounts remain functional in their current status. However, the accounts are also added to the inbox of the Arbitrators who now have to read the report and vote on it. 

The whistleblower reports could easily be off chain initially, if the implementation of a bond amount can be delayed. It may even be beneficial to keep them off chain for the long run, seeing how privacy preservation of the whistleblowers themselves is a requirement. 

The blacklist whistleblower widget shares many features and characteristics with the whitelist application widget. 


## 3.2 Whitelist Application Widget

Whitelisting gives a service provider the option of unwrapping the tokens. Having this option will make them more willing to accept the token as payment, since it is now a liquid token for them. It will also give them access to new customers with funds available. 

Applying to whitelisting is available to anyone in the community. It should require a stake of 100 Near, which is 

- slashed if the application is later deemed spam
- refunded for unsuccessful but non-spam applications
- transformed into two permanent program bonds for successful applicants, one specific and one general

The reason to split the permanent bond into a specific and a general part is that with a lot of general whistleblower reports the general pot might go empty. But we still need a specific stake related to each of the whitelisted service providers, as that’s where we are the most sensitive to breach and misuse.

- One part of the permanent bond acts as an incentive for the community to report non-compliance of the specific service provider and therefore also as an incentive for service provider to remain compliant. 
- The other part becomes a pool prize available for anyone who submits a whistleblower reports any other account (general, not specific to the service provider that funded it)

The features of the Whitelist Application widget include

- Title Bar:

  - Bold text displaying: "GRANT Whitelist Application".

- Business Name Section

  - Label: "Business Name"
  - Short text area for users to input the name of their business

- Application Body:

  - Label: "Business Description"
  - Text area for users to input the main content of their application. Placeholder: "Describe your business, the services you offer and the nature of discounts you will offer to Grantees…"

- Business Accounts Section:

  - Label: "Business Account(s)"
  - Text field with placeholder: "Enter account name"
  - After the first account is entered, a button labeled "Add another account?" appears below the text field. When clicked, it spawns another identical text field for the next account.
  - Beside each account field, display the current status of the account

- Bond Amount Section:

  - Label: "Bond Amount"
  - Text displaying: "Required stake of 100 Near (90% refunded if application is dismissed or not approved, staked if application is approved, then slashed if blacklisted)."
  - Checkbox for users to agree to the bond terms. Label next to checkbox: "I agree to stake 100 Near."

- Terms and Conditions Section

  - Text displaying: Applicant businesses must agree to our Terms and Conditions. Link to the T\&Cs document. Upon clicking the link a popup window appears with the full text of the Terms & Conditions.
  - Checkbox for users to agree to accept GRANT. Label next to checkbox: "I agree to accept GRANT tokens at face value as full payment for services provided."
  - Checkbox for users to agree to provide discounts. Label next to checkbox: "I agree to provide at least 10% discounts to Grantees."
  - Checkbox for users to agree to only accept GRANT for their core services. Label next to checkbox: "I agree to only accept GRANT for services which are within the core expertise of our Business."
  - Checkbox for users to agree to not provide prohibited services. Label next to checkbox: "I agree to not provide services related to swapping, off-ramping, bridging, nor staking."

- Submission Section:

  - Button labeled "Submit Application". Upon clicking, a confirmation pop-up appears with the message: "Your application has been submitted and will be reviewed by our processors."

- Privacy Note:

  - Text displaying: "Note: Your report will be stored on-chain and publicly viewable. Please don’t include any information you don’t want made public."

As a support to the applicant, we should then show the current status of the application, and a listing of any existing prior applications and whistleblower reports related to the business’ account(s)

Upon submission the accounts remain in their prior status. However, the accounts are also added to the inbox of the Whitelist Processors who now have to read the applications and vote on it. If a supermajority of Processors approve the application then all the business Accounts are added to the Whitelist and are enabled to access the Unwrap Token functions.


## 3.3 Election Widgets

Blacklisting & whitelisting is semi-decentralized. The creation of new roles of “blacklist arbitrator” and “whitelist processor” which are open for community members to apply to. Anyone from the community can sign up to be an arbitrator or processor candidate. They have to first commit to upholding the program standards. Election of arbitrators/processors is done at the same time as NDC elections. Candidates with the most votes win and are given the arbitrator/processor role. The role os revoked and re-elected during the next election. 

The standard NDC widgets are used to elect Arbitrators. 

Alternative solutions. Instead of creating a new role and a new House of Arbitrators, is would be possible to 

- Assign the role of arbitrators to the elected members of the Transparency Commission
- Let the HoM suggest Arbitrators/Processors, with a veto option from the CoA, and giving the TC option to remove Arbitrators/Processors who misbehave


## 3.4 Arbitrator/Processor Account Mgmt Widget

This widget is enabled only for people with either the Blacklist Arbitrator role or the Whitelist Processor role. 

Main view: List accounts with new reports from the whistleblower function or applications from the whitelisting application widget. Depending on role, the user will see either reports or applications, or both. 

The default view lists account with new whistleblower reports and new whitelist applications at the top. A report or applicationt is new if the Processor/Arbitrator hasn’t seen the report before, or hasn’t yet voted on the account. 

Available functionality:

- Blacklisted / Whitelisted / Normal

- Ability to read / inspect any whistleblower report(s) and whitelisting application(s) related to the account. 

- The widget should track which reports amd applications the user has read before. 

  - If it’s a new report for a new suspected account then the read receipt is the vote to blacklist / un-blacklist/ or justify not taking an action on the account. 
  - If it’s a new application for a new whitelisting of an account then the read receipt is the vote to whitelist or un-whitelist or justify not whitelisting the account. 
  - If it’s a new report for a previously voted on account then the receipt is the updated vote or an dismissal of the report/application

- Ability to vote and at the same time provide a justification comment for their vote

- Ability to see how other Arbitrators have voted on the account. 

A tentative design could look like this:

- Title Bar:

  - Bold text displaying: "GRANT Account Management".

- Role Identification:

  - Text displaying: "Your Role: Blacklist Arbitrator / Whitelist Processor / SuperAdmin", indicating the user's role.

- Main View:

  - List displaying accounts with whistleblower reports and/or whitelist applications, with default set to show newest at the top.

  - Filter Button which shows filter options when clicked: 

    - Show approved, rejected, pending
    - Show grouped by business or ungrouped accounts

  - Each account entry in the list has the following:

    - Account name.
    - Account status indicator: "Blacklisted", "Whitelisted", or "Normal".
    - Icons indicating a whistleblower report or whitelist application (distinct icons for each).
    - A badge or highlight indicating if the report/application is new and unread.

- Account Inspection:

  - Upon clicking an account entry from the main view:

    - A side panel expands to show details of the report / application
    - Sections displaying the whistleblower report(s) and whitelisting application(s) related to the clicked account.
    - Each report/application section has a header indicating its type ("Whistleblower Report" or "Whitelist Application") and a status indicator (e.g., "New", "Read", "Voted").
    - Clickable links/buttons to view the full content of each report/application.
    - An icon or mark indicating whether the user has read the report/application before.

- Voting Mechanism:

  - Below each report/application detail in the side panel:

    - Radio buttons or dropdown options for voting: 

      - "Blacklist", "Un-Whitelist", "Reject Report" (for whistleblower reports) 
      - "Un-Blacklist", "Reject Appeal" (for blacklisting appeals)
      - "Whitelist", "Reject Application" (for whitelist applications)

    - A textarea for users to provide a justification comment for their vote.

    - A button labeled "Submit Vote".

- Other Arbitrator Votes:

  - Below the voting mechanism:
  - A section displaying how other Arbitrators have voted on the account. This can be shown as a list or a chart.
  - Each entry shows the Arbitrator's name, their vote, and their justification comment.

- Footer:

  - Text displaying: "Ensure to provide justifications for your votes. Your decisions will impact account statuses."


## 3.5 Issuance and TVL Manager / Wrapping Widget

Depending on design, user access will work differently. If the token is backed by a pool of assets then this widget is only for users with the role Grant Grantor. If the token is a wrapped token then anyone in the community can access this widget. 

The main functionality is to mint new wrapped tokens and send them to grantees. 


### Minting wrapped tokens

If the token is a wrapped token then this widget allows a central Issuer the right to minting new tokens only if and when the asset pool is worth more than the number of outstanding tokens.  


### Minting backed token

If the token is backed by a pool of assets then this widget allows holders of the underlying tokens to convert them into wrapped tokens. 

The issuance and TVL manager widget would be very much more useful for the users of it if it also included a reporting tool for viewing TVL stats


## 3.6 Reporting Widget

This is an indexer and reporting function. The user should be able to produce a report that outlines useful statistics such as breakdown of

- Total TVL locked into wrapped tokens
- Total minted / wrapped
- Total redeemed / unwrapped


### Breakdown by status

- Total with grantees
- Total with blacklisted
- Total with whitelisted
- Total with rest of community


### Breakdown by grant program

- Issued by CDAO
- Issued by MDAO
- Issued by Potlock
- Issued as UBI
- Etc


## 3.7 Unwrapper Widget

Redeeming and unwrapping is the same thing: converting the GRANT Token back into Near or back into stables. 

Unwrapping should be enabled only for whitelisted accounts. These are accounts that satisfy the criteria of a redeemer account and thereby qualify to redeem tokens.

Anyone can use the widget to view a breakdown of their balance by circulation counter. But only whitelisted accounts can execute the unwrap. 

The Unwrap function should take as input an Amount quantity which is equal to or smaller than the quantity available in the account. It should 

- Reduce the token count for the user by the Amount
- Reduce the total outstanding tokens by the same Amount
- Release the same Amount of underlying tokens (e.g. USDC), reducing the total count of locked/wrapped underlying tokens
- Charge a redemption fee if applicable, by sending the fee in USDC to the program administrator
- Transfer the remaining unlocked USDC balance to the whitelisted user
- Panic if all of the above cannot be simultaneously executed successfully
- Emit an Unwrap Event

A tentative design of the Unwrapper Widget might look like this:

- Title Bar:

  - Bold text displaying: "Unwrap You GRANT".

- Role & Eligibility Verification:

  - Text displaying: "Unwrapping is enabled only for whitelisted accounts."
  - A checkmark icon or text indicating the status: "Your account is Whitelisted" or "Your account is Not Whitelisted."

- Token Information:

  - Label: "Available GRANT Tokens"
  - Text displaying the number of GRANT Tokens available in the user's account.
  - Label: "Of which are eligible to Unwrap"
  - Text displaying the number of GRANT Tokens available for unwrapping.

- Unwrap Function:

  - Label: "Enter Amount to Unwrap"
  - Input field for users to specify the amount of GRANT Tokens they wish to convert. Placeholder: "Enter amount..."
  - Button labeled “Max”, which when selected will enter the full amount into the input field above
  - Beneath the input, a dynamic text displays the equivalent value in USD that will be received after unwrapping.

- Redemption Fee (if applicable):

  - Label: "Redemption Fee"
  - Text displaying the fee amount in USDC that will be charged for the unwrapping process, with a note: "Fee will be sent to the program administrator."

- Total Amount to be Transferred:

  - Label: "Total USDC to be Transferred"
  - Text displaying the total USDC amount the user will receive post-redemption, after subtracting the fee.

- Unwrap Button:

  - A button labeled "Unwrap". Upon clicking, the system will:

    - Deduct the specified amount of GRANT Tokens from the user's account.
    - Reduce the total outstanding GRANT Tokens.
    - Release the equivalent amount of underlying tokens (USDC).
    - Charge the redemption fee if applicable.
    - Transfer the remaining USDC balance to the whitelisted user.

- Footer:

  - Text displaying: "Only GRANT Tokens that have been circulated 3 times or more are available for unwrapping”


# 4. Smart Contract Design

![](https://lh7-us.googleusercontent.com/CJkrmC_Vvp34vjHULSgvHcf8zXrI0wbDZzDjxiSa1NNF2TB1zJT9RwJMcUodue8k2cMlW7OR53Ip8X29hFWWAIwOQnpQfTfUMEsSmEcqyXnxwJlwRnju7SC103lqlehc3gy65gaoxK6q7eGp68i3CPQ)


## 4.1 Token Standard & Wallet integration

The token must be visible within any Near Wallet. This will ensure that the token can be used as any other token. It should be very easy for users to see their current balance, send tokens to others as payment for goods and services, and track their transaction history. 


## 4.2 Assumptions

We allow for two different tokens: a Near-backed version and a USDC-backed version. We might even allow for other stables to be added in the future

We build a “semi-fungible” token with “memory” of how many Circulation Events it has been through, and which whitelisted accounts it has been through, each of which triggered an increment of the CirculationEventsCounter. 

In the following section we assume

- That we will maintain two separate and distinct arrays of balances (by token holder and by FractionID). 
- That there is an option for different underlying tokens, with an ID to identify which
- There is no memory of which minting batch or from which minter the token originated from (other than emitting a standard event). Though this info would be possible to store in the FractionID we opt against it to save storage space and gas fees. 
- As discussed above, this may not be the best way to build it and could change during detailed design. 


## 4.3 Core Functions

### Core Function: Transfer token

- Input arguments

  - Sender (self)
  - Token (Near- or USDC-backed)
  - Amount
  - Target

- Logic

  - Check requirements

    - Sender is not blacklisted
    - Target is not blacklisted 
    - Sender has balance available

  - Set Transferred variable to zero

  - Loop through the available FractionID of Sender starting from the FractionID with the smallest balance

    - If \[Transferred + Balance of FractionID <= Amount] then transfer the FractionID 

      - Update owner of the FractionID from Sender to Target

      - Increment Transferred by Balance of FractionID

      - Loop to the next FractionID

      - If Target is Whitelisted and not equal to the memory of the last CirculationEvent trigger, then

        - Increment CirculationEvenCounter by 1
        - Set memory to Target 

    - Else

      - Decrease the Balance of the FractionID by \[Amount - Transferred]

      - Increase Transferred by \[Amount - Transferred]

      - Create new FractionID

        - Inherit the property of the old FractionID

        - Set balance to \[Amount - Transferred]

        - Set owner to Target

        - If Target is Whitelisted and not equal to the inherited memory of the last CirculationEvent trigger, then

          - Increment CirculationEvenCounter by 1
          - Set memory to Target 

      - Reduce the balance of the old FractionID by \[Amount - Transferred]

      - Stop the loop, since the full Amount has now been unwrapped

    - _(See more info in section 1.7 FractionIDs)_

    - _(See more info in section 2.4 FractionID priority)_

  - Update Balances

    - Decrement sender balance by Amount (which is also now equal to Transferred)
    - Increment target balance by amount
    - _This overall balance is duplicative to the sum of balance of the FractionIDs, and only provided for compatibility with the Fungible Token standard_

  - Validate Transfer

    - If Amount is not equal to Transferred then throw error

    - If these four balances are not all equal to each other then throw error

      - Balance owned by sender+target before the transaction
      - Balance owned by sender+target after the transaction
      - Sum balance across FractionIDs owned by sender+target before the transaction 
      - Sum balance across FractionIDs owned by sender+target after the transaction 

- Outcome

  - Tokens have been transferred from sender to target

- Return

  - Success

- Emit Event: Token Transfer


### Core Function: Wrap and Mint GRANT Token

- Input arguments

  - Amount to be minted
  - Token ID

- Logic

  - Check that user/self is not blacklisted

  - Check that status if TokenID is Active (that minting is allowed)

  - Identify the underlying token based on the TokenID

  - Check that balance of underlying tokens of user/self is sufficient

  - Lock away Amount of underlying tokens, ask the user to authorize a transfer of the underlying into balance this smart contract

    - decrease balance of user
    - increase balance of smart contract

  - Mint TokenID tokens (increase balance by Amount)

    - Increase TokenID balance of user by (100% - MintFee) \* Amount
    - Increase TokenID balance of admin account by MintFee \* Amount
    - Increase total TokenID balance by 100% of Amount

- Outcome

  - Underlying tokens locked (e.g. USDC)
  - GRANT tokens with TokenID minted, balance of self has increased

- Return

  - Success
  - New total balance

- Emit Event: Minted Grant Token

  - TokenID
  - Amount
  - Block height
  - User Account


### Core Function: Unwrap All

- Input arguments

  - TokenID (indicating Near- or USDC-backed token)

- Logic

  - Check that self is whitelisted
  - Call Unwrappable Account Balance
  - Call Unwrap Tokens, passing TokenID and Unwrappable Account Balance

- Outcome

  - All qualifying tokens have been unwrapped

- Emit Event: None (already emitted by Unwrap Tokens function)


### Core Function: Unwrap Tokens

- Input arguments

  - TokenID (indicating Near- or USDC-backed token)
  - Amount

- Logic

  - Check that user/self is whitelisted

  - Check that “Unwrappable Account Balance” is greater than or equal to Amount

  - Set Unwrapped variable to zero

  - Transfer Amount \* UnwrapFee to Admin Account

  - Set Amount to Amount \* (1-UnwrapFee)

  - Loop through the available FractionID of self starting from the FractionID with the smallest balance

    - If the FractionID is not unwrappable, then

      - Loop to next FractionID

    - Else

      - Set Unwrappable variable to equal amount of the Fraction ID

    - If \[Unwrapped + Unwrappable <= Amount] then unwrap the FractionID

      - Increase Unwrapped by the Unwrappable amount
      - Set balance of the FractionID to zero, which also means that the FractionID can be considered deactivated (or remove the FractionID from the listing entirely, as it is no longer needed, in order to save block space)
      - Loop to the next FractionID

    - Else

      - Unwrap the remaining amount required = Amount - Unwrapped
      - Increase Unwrapped by the same amount
      - Reduce the balance of the FractionID by the same amount 
      - Stop the loop, since the full Amount has now been unwrapped

  - Burn the tokens

    - Decrement the account balance of the user by Amount
    - Decrement the total outstanding GRANT tokens by Amount

  - Release the corresponding Amount of USDC or Near, transferring these to the user

- Outcome

  - Tokens have been unwrapped

- Emit Event: Unwrap Tokens

  - uint: Amount unwrapped
  - array of hex: FractionIDs deactivated


## 4.4 View Functions

### View Function: Account balance

- Input arguments

  - Account

- Logic

  - None

- Outcome

  - User is informed about their account balance

- Return

  - Return current token balance of Account

- Emit Event: None


### View Function: Unwrappable Account Balance

- Input arguments

  - Account

- Logic

  - Check that self is whitelisted
  - Iterate through the available token batches of Account, counting how many of them fill the Unwrap criteria: CirculationEventsCounter is equal to or greater than Circulation Threshold

- Outcome

  - User is informed about account balance available to unwrap

- Return

  - Return current token balance of Account that’s available to unwrap

- Emit Event: None


### View Function: Account status

- Input arguments

  - Account

- Logic

  - None

- Outcome

  - User is informed 

- Return

  - Return current status of Account, where the options are: whitelisted, blacklisted, standard account (neither whitelisted nor blacklisted)

- Emit event: None


### View Function: Total Overview

- Input arguments

  - None

- Logic

  - None

- Outcome

  - User is informed about overall totals

- Return

  - Total count / max value of TokenIDs
  - Total count of Active TokenIDs
  - Total count of Obsoleted TokenIDs
  - Array of tuples: TokenID, TokenNames, Active/Obsolete status


### View Function: Totals for TokenID

- Input arguments

  - TokenID

- Logic

  - None

- Outcome

  - User is informed about totals for a given GRANT TokenID

- Return

  - Total current balance of tokens
  - Total tokens ever minted
  - Current balance of tokens in Blacklisted accounts
  - Current balance of tokens in non-blacklisted accounts
  - Count of outstanding token holder Accounts with Balance > 0
  - Count of outstanding token holder FractionIDs with Balance > 0

- Emit event: none


## 4.5 Admin Functions

### Admin Function: Add Role

- Input arguments

  - Account
  - Boolean: Make Admin
  - Boolean: Make Blacklist arbitrator
  - Boolean: Make Whitelist processor

- Logic

  - Check that self has Admin role
  - Check that Account doesn’t already have any of the new roles
  - If Admin then add Account to list of Admins
  - If Arbitrator then add Account to list of Arbitrators
  - If Processor then add Account to list of Processors

- Outcome

  - Account has one or more new roles

- Return

  - Success

- Emit Event: Add Admin Role


### Admin Function: Revoke Role

- Input arguments

  - Account
  - Boolean: Revoke Admin
  - Boolean: Revoke Blacklist arbitrator
  - Boolean: Revoke Whitelist processor

- Logic

  - Check that self has Admin role
  - Check that Account has all of the roles to be revoked 
  - If Admin then remove Account from list of Admins
  - If Arbitrator then remove Account from list of Arbitrators
  - If Processor then remove Account from list of Processors

- Outcome

  - Account has been revoked from one or more new roles

- Return

  - Success

- Emit Event: Revoke Admin Role


### Admin Function: Change Circulation Threshold

- Input arguments

  - New Threshold

- Logic

  - Check that self has Admin role
  - Check that Threshold is different from current threshold value
  - Check that Threshold is within acceptable range
  - Replace old threshold value with Threshold

- Outcome

  - Threshold has been updated

- Return

  - Success

- Emit Event: Change Circulation Threshold


### Admin Function: Add Whitelisted Business

- Input arguments

  - Service Provider Name
  - Array: Account(s)

- Logic

  - Check that user has Whitelist Application Processor role
  - Check that Account(s) isn’t/aren’t already whitelisted
  - Check that Service Provider Name is unique and not already registered
  - Increment WhitelistID
  - Create Whitelisted business: Add Service Provider Name to the new and last WhitelistID
  - Set status of WhitelistID to active
  - Call Whitelist Accounts function, passing Account(s) and WhitelistID

- Outcome

  - The new Business has been added to whitelist, their accounts are also whitelisted

- Emit Event: Whitelisting Business


### Admin  Function: Whitelist Accounts (also un-blacklists them)

It’s possible for one Whitelisted service provider to have more than one account associated with it, for example for accounting reasons or to enable multiple officers within the business. It’s important that we allow a whitelisted business to transfer tokens between accounts without increasing the CirculationEventsCounter. 

- Input arguments

  - Array: Accounts
  - WhitelistID

- Logic

  - Check that user has whitelist Application Processor role
  - Check that Account(s) isn’t/aren’t already whitelisted
  - Check that WhitelistID is valid and active
  - Add account(s) to WhitelistID
  - Remove account from Blacklist if it was on there

- Outcome

  - Account(s) are assigned to the indicated Service Provider and whitelisted

- Emit Event: Whitelisting Accounts

_Note: Since we split whitelisting into two functions (business vs account) we also need to split how we un-whitelist, blacklist and un-blacklist for accounts vs businesses accordingly._ 


### Admin  Function: Unwhitelist account

- Input arguments

  - Account

- Logic

  - Check that user has whitelist Application Processor role
  - Check that Account is already whitelisted
  - Remove account from Whitelist

- Outcome

  - Whitelisting has been revoked

- Emit Event: Un-Whitelisting


### Blacklist account (also un-whitelists it)

- Input arguments

  - Account

- Logic

  - Check that user has Blacklist Arbitrator role
  - Check that Account isn’t already blacklisted
  - Add account to Blacklist
  - Remove account from Whitelist if it was on there

- Outcome

  - Account is Blacklisted

- Emit Event: Blacklisting


### Admin Function: Un-blacklist account

- Input arguments

  - Account

- Logic

  - Check that user has Blacklist Arbitrator role
  - Check that Account is already blacklisted
  - Remove account from Blacklist

- Outcome

  - Blacklisting has been revoked

- Emit Event: Un-Blacklisting


## 4.5 SuperAdmin Functions

### SuperAdmin Function: Add Token

- Input arguments

  - Name
  - Account address of underlying token

- Logic

  - Check that self has Admin role

  - Check that Account address  is not already used 

  - Check that Name is unique

  - Increment Token ID

  - Create new token

    - Set name 
    - Set account of underlying
    - Set Token ID

  - Set total token amount counter to zero

  - Set token status to Active

- Outcome

  - New token is available to wrap

- Return

  - Token ID

- Emit Event: Add GRANT Token

  - TokenID
  - Name
  - Account address of underlying token


### SuperAdmin Function: Obsolete Token

- Input arguments

  - Token ID

- Logic

  - Check that self has Admin role
  - Check that TokenID is not already Deactivated
  - Change status of TokenID to Deactivated

- Outcome

  - Token is deactivated, minting is no longer allowed

- Return

  - Success

- Emit Event: Obsolete GRANT Token

  - TokenID
  - Total outstanding amount
  - Count of outstanding token holder Accounts with Balance > 0
  - Count of outstanding token holder FractionIDs with Balance > 0


### SuperAdmin Function: Set Wrapping Fee

### SuperAdmin Function: Set Unwrapping Fee

# 5. Risks and Mitigation Plans

## 5.1 Circular Chain of Transactions

We have addressed the simple case where a whitelisted user attempts to send tokens to a non-whitelisted account and then back to the whitelisted account, by requiring that the whitelisted recipient be different from the previous whitelisted recipient before the counter is incremented. But there is still a possibility of users designing a circular chain of transactions between two whitelisted businesses that could artificially inflate the CirculationEventsCounter. We deem the risk of two whitelisted businesses colluding in this manner to be acceptably low, particularly with the introduction of a bond that would be slashed if they were found out.


## 5.2 Undesirable Complexity

The idea of GRANTS introduces a complexity not present with other fully liquid token grants. This complexity could decrease the token's desirability as a means of trade. If the token is undesirable then it's value decreases relative to the underlying liquid token, the risk of which could discourage users from participating in the system. We conclude that with enough support from the NDC leadership, and after signing up a few cornerstone service providers to publicly declare their acceptance of the token, this risk can be sufficiently mitigated.


## 5.3 Secondary Market Pricing

If the circulation threshold is set too high, then there is also the risk that a new secondary pricing model might emerge with market pricing of the token based on how many times a token has been circulated, where the price is initially low for uncirculated FractionIDs and closer to the price of the underlying token the closer a FractionID is to the circulation Threshold. The mitigation of this risk can include the following componentes

- Remove the circulation count from the user interface
- Don’t allow the user any choice on which FractionID to transfer, but rather base the choice on factors unrelated to the CirculationCounter. This way if the users have no control over which FractionID to transfer then it doesn’t make sense for a vendor to analyze the circulation count of a buyer’s wallet nor pricing the FractioIDs differently. 
- Don’t set the CirculationThreshold so high that circulation requirement is seen as an insurmountable obstacle and Unwrapping becomes unattainable.


## 5.4 Token Factory Complexity

We have designed a token “factory” concept that has the option of creating multiple different tokens (tokenID) within the same smart contract. While this creates significant synergies in terms of all tokens leveraging the same whitelist, the same blacklist, the same processors and arbitrators, etc, it also creates some risks of users confusing the various instances of GRANT tokens. It will be important to distinguish the different tokens from each other, particularly by creating unique and distinct names for each tokens. For example:

- Near-GRANT
- USDC-GRANT
- CAD-GRANT
- EUR-GRANT


## 5.5 Capturing All Swaps, Offramps & Bridges?

The blacklist prevents certain entities from using GRANT. This is a proactive measure to prevent unwanted off-ramping. However, the challenge will be in maintaining an up-to-date list, especially in a decentralized ecosystem where new exchanges and platforms can emerge rapidly. It will be quite challenging to ensure that all potential off-ramps are blacklisted. The mitigation for this risk includes

- Employing staff to scour the Near ecosystem on a continuous basis to find new discouraged use cases and add to the blacklist 
- Implementing a whistleblower bounty for the community to get rewarded for reporting possible new candidates for the blacklist


## 5.6 Whitelisting Objectivity

The whitelist is essentially a list of approved entities that can convert GRANT back to USDC or other underlying tokens. The potential challenge here is determining the criteria for being added to or removed from the whitelist. If the criteria are not transparent and consistent, this could introduce centralization or bias. The mitigation plan for this risk is to 

- Define transparent, clear and objective criteria for whitelisting
- Employ several whitelist processors who can review each application, and require that a majority of the processors agree before an application is approved


## 5.7 Expensive Cost of Entry

The stake requirement for whitelisted accounts introduces another layer of complexity. While it can deter misbehavior, it might also be a barrier for legitimate entities that don't have the required stake. Furthermore, the process of slashing based on whistleblower reports needs to be transparent and foolproof to prevent potential misuse.


# 6. Roadmap

1. Get OK from Coucil of Advisors that they will sponsor this idea if built
2. Participate in NearCon Hackathon
3. Update this whitepaper design with lessons learned from Hackathon
4. Estimate total cost of finishing v1 of this idea
5. Update the roadmap accordingly
6. Submit whitepaper to HoM for funding to finish v1

**End of Document**

****
