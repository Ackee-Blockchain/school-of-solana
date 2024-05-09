# 7. Lecture - Security

## Table of Contents
- [Best Security Practices](#best-security-practices)
    - [Signer Authorization](#signer-authorization)
    - [Owner Checks](#owner-checks)
    - [Account Data Matching](#account-data-matching)
    - [Reinitialization Attacks](#reinitialization-attacks)
    - [Duplicate Mutable Accounts](#duplicate-mutable-accounts)
    - [Type Cosplay](#type-cosplay)
    - [Arbitrary CPI](#arbitrary-cpi)
    - [Bump Seed Canonicalization](#bump-seed-canonicalization)
    - [Closing Accounts and Revival Attacks](#closing-accounts-and-revival-attacks)

---

# Best Security Practices

## Signer Authorization

### Plain Rust

Use `Signer Checks` to verify that specific accounts have signed a transaction. Without appropriate signer checks, accounts may be able to execute instructions they shouldn’t be authorized to perform.

To implement a signer check in Rust, simply check that an account’s `is_signer` property is `true`

```rust
if !ctx.accounts.authority.is_signer {
	return Err(ProgramError::MissingRequiredSignature.into());
}
```

### Anchor
In Anchor, you can use the `Signer` account type in your account validation struct to have Anchor automatically perform a signer check on a given account

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    // performs signer check
    pub user: Signer<'info>,
}
```

## Owner Checks

### Plain Rust

Use `Owner Checks` to verify that accounts are owned by the expected program. Without appropriate owner checks, accounts owned by unexpected programs could be used in an instruction.

To implement an owner check in Rust, simply check that an account’s owner matches an expected program ID

```rust
if ctx.accounts.account.owner != ctx.program_id {
    return Err(ProgramError::IncorrectProgramId.into());
}
```

### Anchor

Anchor program account types implement the `Owner` trait which allows the `Account<'info, T>` wrapper to automatically verify program ownership

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    pub user: Signer<'info>,
    // performs ownership check
    pub user_data: Account<'info, UserData>,
}
```

Anchor gives you the option to explicitly define the owner of an account if it should be anything other than the currently executing program

## Account Data Matching

### Plain Rust

Use `Data Validation checks` to verify that account data matches an expected value. Without appropriate data validation checks, unexpected accounts may be used in an instruction.

To implement data validation checks in Rust, simply compare the data stored on an account to an expected value.

```rust
if ctx.accounts.user.key() != ctx.accounts.user_data.user {
    return Err(ProgramError::InvalidAccountData.into());
}
```

### Anchor

In Anchor, you can use `constraint` to checks whether the given expression evaluates to true. Alternatively, you can use `has_one` to check that a target account field stored on the account matches the key of an account in the `Accounts` struct.

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    pub user: Signer<'info>,
    #[account(constraint = user_data.authority = user.key())]
    pub user_data: Account<'info, UserData>,
}
```

## Reinitialization Attacks

### Plain Rust

Use an `account discriminator or initialization flag` to check whether an account has already been initialized to prevent an account from being reinitialized and overriding existing account data.

To prevent account reinitialization in plain Rust, initialize accounts with an `is_initialized` flag and check if it has already been set to true when initializing an account

```rust
if account.is_initialized {
    return Err(ProgramError::AccountAlreadyInitialized.into());
}
```

### Anchor

To simplify this, use Anchor’s `init` constraint to create an account via a CPI to the system program and sets its discriminator

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    pub user: Signer<'info>,
    // if account is already initialized init will return error
    #[account(init)]
    pub user_data: Account<'info, UserData>,
}
```

## Duplicate Mutable Accounts

### Plain Rust

When an instruction requires two mutable accounts of the same type, an attacker can pass in the same account twice, causing the account to be mutated in unintended ways.

To check for duplicate mutable accounts in Rust, simply compare the public keys of the two accounts and throw an error if they are the same.

```rust
if ctx.accounts.account_one.key() == ctx.accounts.account_two.key() {
    return Err(ProgramError::InvalidArgument)
}
```

### Anchor

In Anchor, you can use constraint to add an explicit constraint to an account checking that it is not the same as another account.

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = user_data.key() != different_user_data.key()
    )]
    pub user_data: Account<'info, UserData>,
    #[account(mut)]
    pub different_user_data: Account<'info, UserData>,
}
```

## Type Cosplay

### Plain Rust

Use discriminators to distinguish between different account types

To implement a discriminator in Rust, include a field in the account struct to represent the account type

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    discriminant: AccountDiscriminant,
    user: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountDiscriminant {
    User,
    Admin,
}
```

To implement a discriminator check in Rust, verify that the discriminator of the deserialized account data matches the expected value

```rust
if user.discriminant != AccountDiscriminant::User {
    return Err(ProgramError::InvalidAccountData.into());
}
```

### Anchor

In Anchor, program account types automatically implement the `Discriminator` trait which creates an 8 byte unique identifier for a type

Use Anchor’s `Account<'info, T>` type to automatically check the discriminator of the account when deserializing the account data

```rust
#[derive(Accounts)]
pub struct UpdateUserData<'info> {
    pub user: Signer<'info>,
    // Account type automatically checks the Discriminator
    // of the UserData Account
    pub user_data: Account<'info, UserData>,
}
```

## Arbitrary CPI

### Plain Rust

To generate a CPI, the target program must be passed into the invoking instruction as an account. This means that any target program could be passed into the instruction. Your program should check for incorrect or unexpected programs.

Perform program checks in native programs by simply comparing the public key of the passed-in program to the program you expected.

```rust
pub fn cpi_secure(ctx: Context<Cpi>, amount: u64) -> Result<()> {
    if &spl_token::ID != ctx.accounts.token_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }
    ...
    // CPI logic goes here
}
```

### Anchor

If a program is written in Anchor, then it may have a publicly available CPI module. This makes invoking the program from another Anchor program simple and secure. The Anchor CPI module automatically checks that the address of the program passed in matches the address of the program stored in the module.

The best practice while using Anchor is to always use `Program<'info, T>`, which will check that the account is executable and it is the given Program. For example:

```rust
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct InitializeExchange<'info> {
    // ...
    pub token_program: Program<'info, Token>,
    // ...
}
```

## Bump Seed Canonicalization

### Plain Rust

The [`create_program_address`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html#method.create_program_address) function derives a PDA without searching for the `canonical bump`. This means there may be multiple valid bumps, all of which will produce different addresses.

Using [`find_program_address`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html#method.find_program_address) ensures that the highest valid bump, or canonical bump, is used for the derivation, thus creating a deterministic way to find an address given specific seeds.

Recommended workflow:

1. During Account Initialization derive the PDA using the [`find_program_address`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html#method.find_program_address). This will produce PDA along with the canonical bump. The next step is to store the produced bump along with program data in an Account.
2. When using the PDA with different instructions, use [`create_program_address`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html#method.create_program_address) with the bump stored in the Account in step 1.

### Anchor

Upon initialization, you can use Anchor's `seeds` and `bump` constraint to ensure that PDA derivations in the account validation struct always use the `canonical bump`.

Anchor allows you to specify a bump with the `bump = <some_bump>` constraint when verifying the address of a PDA.

```rust
pub fn _initialize_exchange(
    ctx: Context<InitializeExchange>,
) -> Result<()> {
    // ...
    // Store the produced canonical bump
    let escrow = &mut ctx.accounts.escrow;
    escrow.bump = ctx.bumps.escrow;
    // ...
    Ok(())
}
pub fn _update_exchange(
    ctx: Context<UpdateExchange>,
) -> Result<()> {
    // Address correctness is ensured in the Context
}

pub struct InitializeExchange<'info> {
    #[account(mut)]
    pub side_a: Signer<'info>,
    #[account(
        init,
        payer = side_a,
        space = 8 + Escrow::LEN,
        seeds = [b"escrow"], // specify desired seeds here
        bump // produce canonical bump
    )]
    pub escrow: Account<'info, Escrow>,
    // ...
}
pub struct UpdateExchange<'info> {
    #[account(
        mut,
        seeds = [b"escrow"], // specify desired seeds here
        escrow.bump // check the bump against the one already stored
    )]
    pub escrow: Account<'info, Escrow>,
}
```

## Closing Accounts and Revival Attacks

`Closing an account` improperly creates an opportunity for reinitialization/revival attacks.

The Solana runtime `garbage collects accounts` when they are no longer rent exempt. Closing accounts involves transferring the lamports stored in the account for rent exemption to another account of your choosing.

### Plain Rust
The recommended workflow for manually closing an account is to use the Anchor's logic (i.e. check the close [Reference](https://github.com/coral-xyz/anchor/blob/460a16171a715671f77ead5629391c0466366c08/lang/src/common.rs#L6))

### Anchor

You can use the Anchor `#[account(close = <address_to_send_lamports>)]` constraint to securely close accounts. The close parameter will ([Close Reference](https://github.com/coral-xyz/anchor/blob/460a16171a715671f77ead5629391c0466366c08/lang/src/common.rs#L6)):

1. `Move` lamports from the data Account to the `<address_to_send_lamports>`
2. `Assign` ownership of the data Account back to the `System Program`
3. `Realloc` the data size of the Account to 0.



For example:

```rust
#[derive(Accounts)]
pub struct CancelExchange<'info> {
    #[account(mut)]
    pub side_a: Signer<'info>,
    #[account(
        mut,
        close = side_a, // this is <address_to_send_lamports> (i.e. Rent receiver)
        seeds = [b"escrow"],
        escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
}
```


-----



### Need help?
If you have any questions feel free to reach out to us at [Discord](https://discord.gg/z3JVuZyFnp).
