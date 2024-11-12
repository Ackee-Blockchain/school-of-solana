# BONUS Lecture - SPL Tokens

## Table of Contents
<!-- no toc -->
- [Tokens](#tokens)
  - [Fungible and Non-Fungible Tokens](#fungible-and-non-fungible-tokens)
- [Token Program](#token-program)
  - [Mint Account](#mint-account)
  - [Token Account](#token-account)
    - [Token Account Structure](#token-account-structure)
  - [Associated Token Account](#associated-token-account)
- [Examples](#examples)
  - [Creating a New SPL Token](#creating-a-new-spl-token)
  - [Creating an Associated Token Account](#creating-an-associated-token-account)
  - [Minting Tokens](#minting-tokens)
  - [Transferring Tokens Between Accounts](#transferring-tokens-between-accounts)
- [Useful Links](#useful-links)

---
## Tokens

SPL tokens can represent various assets, including cryptocurrencies, stablecoins, NFTs, and even tokenized real-world assets like commodities or real estate. They enable transfers, exchanges, staking, lending, and a variety of financial operations.

### Fungible and Non-Fungible Tokens

**Fungible Tokens**: These tokens are interchangeable with one another. They are indistinguishable and hold the same value.

**Non-Fungible Tokens (NFTs)**: Each NFT is a special digital asset that holds a unique information or value. NFTs can represent ownership of a specific digital or physical item, such as digital art or real estate.

## Token Program

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

-----

### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
