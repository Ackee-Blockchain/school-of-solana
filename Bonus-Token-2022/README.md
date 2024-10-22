# BONUS Lecture - Token 2022

## Table of Contents
- [Token-2022](#token-2022)
    - [Motivation](#motivation)
    - [Mint Extensions](#mint-extensions)
        - [Mint Close Authority](#mint-close-authority)
        - [Transfer Fees](#transfer-fees)
        - [Non-Transferable Tokens](#non-transferable-tokens)
        - [Permanent Delegate](#permanent-delegate)
        - [Transfer Hook](#transfer-hook)
        - [Metadata Pointer](#metadata-pointer)
        - [Metadata](#metadata)
        - [Confidential Transfers](#confidential-transfers)
    - [Token Extensions](#token-extensions)
        - [Default Account State](#default-account-state)
        - [Immutable Owner](#immutable-owner)
        - [Required Memo on Transfer](#required-memo-on-transfer)
        - [CPI Guard](#cpi-guard)

---

# Token-2022

Token extensions are the next generation of the Solana Program Library Token standard. Over a dozen extensions provide advanced configurable functionality, specifically designed to meet the needs of businesses with compliance obligations.

## Motivation

The existing Token Program serves most needs for Tokens on Solana through a simple set of interfaces and structures.

As more developers have come to Solana with new ideas, however, they have forked the Token Program to add functionality. It's simple to change and deploy the program, but it's difficult to achieve adoption across the ecosystem.

Solana's programming model requires programs to be included in transactions along with accounts, making it complicated to craft transactions involving multiple token programs.

On top of the technical difficulty, wallets and on-chain programs must trust any token program that they choose to support.

We need to add new token functionality, with minimal disruption to users, wallets, and dApps. Most importantly, we must preserve the safety of existing tokens.

A new token program, Token-2022, was developed to achieve both of these goals and deployed to a different address than the Token program.

## Mint Extensions

Mint extensions are added on top of the original Solana Token Program and extend the abilities of tokens.

### Mint Close Authority

The Token program allows owners to close token accounts, but it is impossible to close mint accounts. In Token-2022, it is possible to close mints by initializing the MintCloseAuthority extension before initializing the mint.

### Transfer Fees

In the Token program, it is impossible to assess a fee on every transfer. The existing systems typically involve freezing user accounts, and forcing them to go through a third party to unfreeze, transfer, and refreeze the accounts.

With Token-2022, it's possible to configure a transfer fee on a mint so that fees are assessed at the protocol level. On every transfer, some amount is withheld on the recipient account, untouchable by the recipient. These tokens can be withheld by a separate authority on the mint.

### Non-Transferable Tokens

To accompany immutably owned token accounts, the `NonTransferable` mint extension allows for "soul-bound" tokens that cannot be moved to any other entity. For example, this extension is perfect for achievements that can only belong to one person or account.

This extension is very similar to issuing a token and then freezing the account, but allows the owner to burn and close the account if they want.

### Interest-Bearing Tokens

Tokens that constantly grow or decrease in value have many uses in the real world. The most well known example is a bond.

With Token, this has only been possible through proxy contracts that require regular rebase or update operations.

With the Token-2022 extension model, however, we have the possibility to change how the UI amount of tokens are represented. Using the `InterestBearingMint` extension and the `amount_to_ui_amount` instruction, you can set an interest rate on your token and fetch its amount with interest at any time.

Interest is continuously compounded based on the timestamp in the network. Due to drift that may occur in the network timestamp, the accumulated interest could be lower than the expected value. Thankfully, this is rare.



### Permanent Delegate

With Token-2022, it's possible to specify a permanent account delegate for a mint. This authority has unlimited delegate privileges over any account for that mint, meaning that it can burn or transfer any amount of tokens.

While this feature certainly has room for abuse, it has many important real-world use cases.

In some jurisdictions, a stablecoin issuer must be able to seize assets from sanctioned entities. Through the permanent delegate, the stablecoin issuer can transfer or burn tokens from accounts owned by sanctioned entities.

### Transfer Hook

Token creators may need more control over how their token is transferred. The most prominent use case revolves around NFT royalties. Whenever a token is moved, the creator should be entitled to royalties, but due to the design of the current token program, it's impossible to stop a transfer at the protocol level.

Current solutions typically resort to perpetually freezing tokens, which requires a whole proxy layer to interact with the token. Wallets and marketplaces need to be aware of the proxy layer in order to properly use the token.

Worse still, different royalty systems have different proxy layers for using their token. All in all, these systems harm composability and make development harder.

To improve the situation, Token-2022 introduces the concept of the transfer-hook extension. A token creator must develop and deploy a program that implements the interface and then configure their token mint to use their program.

### Metadata Pointer

With the potential proliferation of multiple metadata programs, a mint can have multiple different accounts all claiming to describe the mint.

To make it easy for clients to distinguish, the metadata pointer extension allows a token creator to designate an address that describes the canonical metadata.

To avoid phony mints claiming to be stablecoins, however, a client must check that the mint and the metadata both point to each other.

### Metadata

To facilitate token-metadata usage, Token-2022 allows a mint creator to include their token's metadata directly in the mint account.

The metadata extension should work directly with the metadata-pointer extension. During mint creation, you should also add the metadata-pointer extension, pointed at the mint itself.

### Confidential Transfers

TBD

[zk-token-protocol-paper](https://github.com/solana-labs/solana-program-library/tree/master/token/zk-token-protocol-paper)

## Token Extensions

Token Account extensions are added on top of Solana accounts and add Token account - related features.

### Default Account State

A mint creator may want to restrict who can use their token. There are many heavy-handed approaches to this problem, most of which include going through a centralized service at the beginning. Even through a centralized service, however, it's possible for anyone to create a new token account and transfer the tokens around.

To simplify the restriction, a mint creator may use the `DefaultAccountState` extension, which can force all new token accounts to be frozen. This way, users must eventually interact with some service to unfreeze their account and use tokens.

### Immutable Owner

Token account owners may reassign ownership to any other address. This is useful in many situations, but it can also create security vulnerabilities.

For example, the addresses for Associated Token Accounts are derived based on the owner and the mint, making it easy to find the "right" token account for an owner. If the account owner has reassigned ownership of their associated token account, then applications may derive the address for that account and use it, not knowing that it does not belong to the owner anymore.

To avoid this issue, Token-2022 includes the `ImmutableOwner` extension, which makes it impossible to reassign ownership of an account. The Associated Token Account program always uses this extension when creating accounts.

### Required Memo on Transfer

Traditional banking systems typically require a memo to accompany all transfers. The Token-2022 program contains an extension to satisfy this requirement.

By enabling required memo transfers on your token account, the program enforces that all incoming transfers must have an accompanying memo instruction right before the transfer instruction.

### CPI Guard

CPI Guard is an extension that prohibits certain actions inside cross-program invocations, to protect users from implicitly signing for actions they can't see, hidden in programs that aren't the System or Token programs.

Users may choose to enable or disable the CPI Guard extension on their token account at will. When enabled, it has the following effects during CPI:

- **Transfer**: the signing authority must be the account delegate
- **Burn**: the signing authority must be the account delegate
- **Approve**: prohibited
- **Close Account**: the lamport destination must be the account owner
- **Set Close Authority**: prohibited unless unsetting
- **Set Owner**: always prohibited, including outside CPI

#### Background

When interacting with a dapp, users sign transactions that are constructed by frontend code. Given a user's signature, there are three fundamental ways for a dapp to transfer funds from the user to the dapp (or, equivalently, burn them):

- Insert a transfer instruction (which can transfer assets) in the transaction.
- Insert an approve instruction (which can assign new delegate and delegate amount) in the transaction, and perform a CPI transfer under program authority.
- Insert an opaque program instruction, and perform a CPI transfer with the user's authorization.

The first two are safe, in that the user can see exactly what is being done, with zero ambiguity. The third is quite dangerous. A wallet signature allows the program to perform any action as the user, without any visibility into its actions. There have been some attempts for workarounds, for instance, simulating the transaction and warning about balance changes. But, fundamentally, this is intractable.

There are two ways to make this much safer:

- Wallets warn whenever a wallet signature is made available to an opaque (non-system, non-token) instruction. Users should be educated to treat the request for a signature on such an instruction as highly suspect
- The token program prohibits CPI calls with the user authority, forcing opaque programs to directly ask for the user's authority

**The CPI Guard covers the second instance**.


-----



### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
