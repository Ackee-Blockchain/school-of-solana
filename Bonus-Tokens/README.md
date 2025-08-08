# Tokens on Solana

## Table of Contents
<!-- no toc -->
- [Tokens](#tokens)
  - [Token Program](#token-program)
  - [Mint Account](#mint-account)
  - [Token Account](#token-account)
  - [Associated Token Account](#associated-token-account)
- [Token 2022](#token-2022)
  - [Mint Extensions](#mint-extensions)
  - [Token Extensions](#token-extensions)
- [Examples](#examples)
- [Useful Links](#useful-links)
---
## Tokens

SPL tokens can represent various assets, including cryptocurrencies, stablecoins, NFTs, and even tokenized real-world assets like commodities or real estate. They enable transfers, exchanges, staking, lending, and a variety of financial operations.

### Fungible and Non-Fungible Tokens

**Fungible Tokens**: These tokens are interchangeable with one another. They are indistinguishable and hold the same value.

**Non-Fungible Tokens (NFTs)**: Each NFT is a special digital asset that holds a unique information or value. NFTs can represent ownership of a specific digital or physical item, such as digital art or real estate.

### Token Program

Token program defines a common implementation for fungible and non-fungible tokens. All tokens on Solana are created using the SPL Token program. It supports functions like minting, transferring, freezing, burning, and more.

### Mint Account

Tokens on Solana are uniquely identified by the address of a mint account owned by the Token Program. Mint accounts store global metadata of tokens.

Fields of a mint account:
- **Supply**: Total supply of the token.
- **Decimals**: The number of decimal places the token can be divided into.
- **Mint authority**: The account authorized to mint new tokens. (Optional)
- **Freeze authority**: The account authorized to freeze token transfers. (Optional)

This is what the [mint account of USDC](https://explorer.solana.com/address/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v) looks like.

<img src=./usdc_mint.png style="border-radius: 10px">

### Token Account

A Token Account on Solana is a type of account that holds a balance of a specific token for a user. Each token account is tied to one type of token, meaning you need a separate token account for each different type of token you want to hold.

#### Token Account Structure

Main fields:

- **Mint**: The type of token the account holds.
- **Owner**: The account with authority to transfer the tokens.
- **Amount**: The number of tokens the account holds.

Additional fields:

- **Delegate**: Delegate authority having possession over delegate amount. (Optional)
- **IsNative**: Specifies whether the token account holds wrapped SOL. (Optional)
- **Delegate** amount: Amount authorized by the delegate authority.
- **Close authority**: Authority able to close the token account. (Optional)

### Associated Token Account

An Associated Token Account (ATA) is a token account that has its address derived (as a PDA) from its owner's wallet address and the address of the mint. Each user has a unique ATA for each combination of wallet and token mint, which is convenient because programs can derive the address and interact with the token account without needing the actual address to be provided.

>[!NOTE]
>A user can receive tokens even if they do not yet have a token account for that mint. The sender is able to fund the creation of the receiver's ATA, enabling things like airdrop campaigns.

## Token 2022

Token extensions are the next generation of the Solana Program Library Token standard. Extensions provide advanced modular and configurable functionality.

The existing Token Program serves most needs for Tokens on Solana through a simple set of interfaces and structures.

As more developers have come to Solana with new ideas, however, they have forked the Token Program to add functionality. It's simple to change and deploy the program, but it's difficult to achieve adoption across the ecosystem.

Solana's programming model requires programs to be included in transactions along with accounts, making it complicated to craft transactions involving multiple token programs.

On top of the technical difficulty, wallets and on-chain programs must trust any token program that they choose to support.

Token 2022, was developed to achieve both of these goals and deployed to a different address than the Token program.

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

### Confidential Transfers (Mint)

Enables private transfers where the transfer amounts are hidden using encryption. Users can see who sent and received tokens, but not how much was transferred.

### Confidential Mint-Burn

Allows minting and burning tokens with hidden amounts. The supply changes are encrypted, so observers can't see how many tokens were created or destroyed.

### Group Pointer

Points to a group account that defines which collection this token belongs to. Similar to NFT collections, it helps organize related tokens together.

### Group

Stores collection information directly on the mint account. Contains the rules and metadata for how tokens in this group should behave.

### Group Member Pointer

Points from a token to its parent group, establishing the membership relationship. Allows clients to easily find which collection a token belongs to.

### Group Member

Stores specific information about how this token participates in its group. Contains member-specific rules and metadata separate from the main group configuration.

### Scaled UI Amount

Applies an updatable multiplier to how token amounts are displayed in wallets and apps. The actual token balance never changes, only what users see.

### Pausable

Allows an authority to temporarily freeze all token operations like transfers, mints, and burns. Useful for emergency situations or maintenance periods.

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

## Examples

>[!TIP]
>You can try these examples out on localnet by running:
>
>```bash
>solana-test-validator
>```

### Creating a New SPL Token

Use the spl-token CLI to create a new SPL token and establish its mint account.
```bash
spl-token create-token
```

Check its supply.
```bash
spl-token supply <TOKEN_ADDRESS>
```

### Creating an Associated Token Account

By default the create-account command creates an associated token account with your wallet address as the token account owner. Use optional `--owner` flag to create a token account with a different owner.
```bash
spl-token create-account --owner <OWNER_ADDRESS> <TOKEN_ADDRESS>
```

### Minting Tokens

Mint tokens to a specified token account.
```bash
spl-token mint <TOKEN_ADDRESS> <AMOUNT> <RECIPIENT_ADDRESS>
```

### Transferring Tokens Between Accounts

Transfer tokens from one token account to another.
```bash
spl-token transfer <TOKEN_ADDRESS> <AMOUNT> <RECIPIENT_TOKEN_ACCOUNT>
```

## Useful Links
>[!TIP]
>Be sure to check out SPL tokens in:
>
>- [Solana Documentation](https://solana.com/docs/core/tokens)
>- [Solana Handbook](https://ackee.xyz/solana/book/latest/chapter4/)
>- [Token 2022](https://www.solana-program.com/docs/token-2022)

-----

### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
