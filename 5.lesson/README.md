# 5. Lecture - Best development practices and debugging

This week, we are taking a look at a few common errors you’re likely to run into while developing Solana programs. Understanding these errors will help you debug more quickly and effectively.

>[!TIP]
>Check out [examples](./common-errors/)! Try running the code and resolving the errors yourself.

## Table of Contents
<!-- no toc -->
- [Types of Tests](#types-of-tests)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
  - [Popular Tools for Integration Testing in Rust](#popular-tools-for-integration-testing-in-rust)
  - [Popular Tools for Integration Testing in Solana Client](#popular-tools-for-integration-testing-in-solana-client)
  - [How to Choose Between These Tools](#how-to-choose-between-these-tools)
- [Common Errors](#common-errors)
  - [Signer Verification Failed](#signer-verification-failed)
  - [No Prior Credit](#no-prior-credit)
  - [Account Already In Use](#account-already-in-use)
  - [Account Did Not Deserialize](#account-did-not-deserialize)
  - [Constraint Seeds](#constraint-seeds)
  - [Program Failed To Complete](#program-failed-to-complete)
- [On-chain Data Fetching](#on-chain-data-fetching)
---

## Types of Tests
You should always aim for high test coverage. Write tests for both successful and failing scenarios.

> [!TIP]
> You can find code examples and instructions for running each test in the [test-examples](./test-examples/) folder.

### Unit Tests

Unit testing is the foundational layer of any robust software testing strategy, ensuring that individual components of a program function correctly in isolation. In the context of Solana smart contracts (on-chain programs), unit tests are designed to validate small pieces of logic, such as a single instruction handler or helper function, without requiring deployment to a cluster or any interaction with other accounts or programs. The goal is to confirm that the internal logic behaves as expected under controlled conditions before it is combined into larger workflows or integrated with the blockchain runtime.

The primary advantage of unit tests lies in their speed. Unlike integration tests that rely on runtime simulation or RPC calls, unit tests run entirely in memory, without the overhead of serialization, network communication, or account state initialization. They execute as regular Rust functions, often completing in milliseconds. Because they operate off-chain and bypass the Solana runtime, unit tests are deterministic and require no external setup, which makes them ideal for continuous integration environments and early development phases.

#### Rust’s Native Testing Framework

The simplest and fastest way to perform unit tests on Solana programs is through Rust’s standard testing framework, built directly into the language. Using this approach, developers write tests in the same crate as their program logic, typically under the `tests` module, and execute them using the `cargo test` command.
This method is particularly useful for:
- testing pure functions, 
- hash computations, 
- access control checks, 
- arithmetic operations that do not depend on Solana accounts or external contexts

Because it bypasses all blockchain-related abstractions, it achieves near-instant execution. However, the trade-off is realism: such tests cannot simulate account data, rent-exempt balances, or instruction execution. They are best suited for early logic validation before the program interacts with the Solana runtime.

Below is an example of a simple unit test.

Imagine a helper function, `math_function`, which safely subtracts `count` from `10` and returns the result as an `Option<u8>`.
```rust
    pub fn math_function(count: u8) -> Option<u8> {
        10u8.checked_sub(count)
    }
```

Then we call `math_function` inside the instruction:
```rust
    ...
    pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
        let data = &mut ctx.accounts.data;

        data.authority = ctx.accounts.user.key();
        require!(count <= 10, MyError::InvalidInstructionData);

        // Never panics due to require macro above.
        data.counter = math_function(count).unwrap();

        msg!("Data.counter = {}", data.counter);
        msg!("Data pubkey = {}", data.key().to_string());
        msg!("User pubkey = {}", data.authority.key().to_string());

        Ok(())
    }
    ...
```

We want to confirm that `math_function(...)` works correctly without on-chain integration, so we expect:
- If `count` is less than or equal to `10`, it returns `Some(10 - count)`.
- If `count` is greater than `10`, it returns `None`.

Here is an example of a simple unit test:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_function() {
        // Check that the function returns `Some(8)` when input is within bounds
        assert_eq!(math_function(2), Some(8));
        // Check that the function returns `None` when input exceeds bounds
        assert_eq!(math_function(11), None);
        // Check that the function returns `Some(0)` when input == bounds
        assert_eq!(math_function(10), Some(0));
    }
}
```

### Integration Tests

While unit tests validate isolated logic, integration tests focus on verifying that multiple parts of a Solana program work together correctly inside a simulated blockchain environment. They ensure that interactions between instructions, accounts, and programs behave as expected under realistic runtime conditions. Integration tests execute the actual Solana runtime in a controlled, local environment. Each instruction is processed as it would be on-chain, complete with account validation, rent checks, signer verification, and system program calls. This allows developers to confirm that PDAs are derived correctly, that instructions modify state as intended, and that errors are raised in the right scenarios. Such realism inevitably comes at a cost: integration tests run more slowly than unit tests, but they remain significantly faster and more deterministic than full tests on devnet or testnet, since they operate entirely in-memory without network communication.

The Solana ecosystem provides several frameworks designed to facilitate integration testing, each offering a different balance between control, abstraction, and developer convenience. The most notable options are `solana-program-test`, `Anchor’s testing framework`, and client-based testing libraries such as `@solana/web3.js` or `Solana Kit`.

### Popular Tools for Integration Testing in Rust

#### Solana Program Test Framework

The `solana-program-test` crate is the backbone of integration testing in Solana’s Rust ecosystem. It launches a lightweight, in-process version of the Solana runtime that behaves nearly identically to a local validator but runs fully in memory. Developers can register one or more programs, create test accounts, send instructions, and inspect the resulting on-chain state — all without deploying to an external cluster.

This framework is good for testing interactions between multiple accounts and verifying instruction-level behavior. For example, a developer might simulate the full lifecycle of a voting process, from poll initialization to casting votes and closing results, while asserting that all state transitions occur correctly. The framework also supports testing multiple programs simultaneously, which is useful for projects involving CPIs or modular on-chain architectures. 

#### Anchor Testing Environment

For projects built with Anchor, integration testing becomes even more streamlined. Anchor internally uses `solana-program-test`, but enhances it with an expressive and ergonomic API. Instead of manually constructing transactions and serializing account data, developers can interact with their programs directly using method calls, with account validation and PDA derivation handled automatically based on the program’s IDL.

This abstraction reduces boilerplate and makes integration tests easier to maintain, especially in large codebases. It also allows for deeper type safety, since both the client and program share the same type definitions. Anchor’s test runner is particularly powerful for verifying complex interactions, such as reward distribution or account initialization flows involving multiple seeds. However, while it greatly improves developer productivity, it offers less flexibility for low-level debugging or for testing non-Anchor programs that do not follow its IDL conventions.

Here is an example of a Rust integration test via the Anchor client:
```rust
/// Integration test using Anchor Client:
///  - Connects to a local Solana validator
///  - Performs an airdrop for a fresh payer account
///  - Calls the `initialize` instruction of the program
///  - Fetches on-chain account data to verify correct state
#[test]
fn test_initialize_with_airdrop() {

    // Create a new random keypair and connect to the local Solana validator
    let payer = Rc::new(Keypair::new());
    let client = anchor_client::Client::new(Cluster::Localnet, payer.clone());
    
    // Access the deployed Anchor program by its ID
    let program = client.program(test_examples::id()).unwrap();
    
    // Get the RPC client from the program
    let rpc = program.rpc();

    // Airdrop 1 SOL to payer
    let sig = rpc
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .expect("Airdrop request failed");

    // Wait until the airdrop transaction is fully confirmed on-chain
    rpc.poll_for_signature(&sig).expect("Airdrop not finalized");

    // Derive PDA
    let (data_pda, _bump) =
        Pubkey::find_program_address(&[b"data1", b"data2", payer.pubkey().as_ref()], &program.id());

    // Build and send the `initialize` instruction
    program
        .request()
        .accounts(test_examples::accounts::Initialize {
            user: payer.pubkey(),
            data: data_pda,
            system_program: system_program::id(),
        })
        .args(test_examples::instruction::Initialize { count: 10 })
        .send()
        .unwrap();

    // Fetch on-chain account
    let acc: test_examples::MyData = program.account(data_pda).unwrap();

    // Verify that the logic ran correctly
    assert_eq!(acc.counter, 0);
}
```

### Popular Tools for Integration Testing in Solana Client

#### Anchor’s TypeScript/JS testing tooling
When you build a program using Anchor, the easiest way to test it is with the TypeScript package `@coral-xyz/anchor`. This library is used for Anchor-based programs because it understands IDLs (Interface Description Languages) automatically generated by Anchor, account types, PDAs (Program Derived Addresses), and it simplifies the interaction model tremendously.

With `@coral-xyz/anchor` you typically start tests (often with Mocha + Chai or another test framework) by importing your program’s IDL, instantiating a `Program object`, and setting up a `Provider` (which combines a connection to a cluster, commonly localnet, and a wallet).

Here is an example of a provider setup:
```ts
  anchor.setProvider(anchor.AnchorProvider.env());
  let connection = anchor.getProvider().connection;
  const program = anchor.workspace.TestExamples as Program<TestExamples>;
```

The line `anchor.setProvider(anchor.AnchorProvider.env());` sets up the provider based on the cluster defined in `Anchor.toml`
```bash
    ...
    [provider]
    cluster = "Localnet"
    ...
```

To invoke an instruction, use the template `program.methods.<name-of-instruction>().accounts({...}).signers([...]).rpc()`. All of that ties back into the Anchor backend, which handles serialization, account struct decoding, and so forth. Because the library is designed to map very directly onto your on-chain program, writing integration tests becomes efficient and less error-prone.

Here is an example of an integration test:
```ts
it("Cannot initialize with incorrect data account!", async () => {
const bad_data = Keypair.generate();

try {
    // Attempt to initialize the program with the incorrect account.
    await program.methods
    .initialize(10)
    .accountsStrict({
        user: user.publicKey,
        data: bad_data.publicKey,
        systemProgram: SystemProgram.programId,
    })
    .signers([user])
    .rpc();

} catch (_err) {
    // Make sure that the error matches the expected "ConstraintSeeds" error.
    const err = anchor.AnchorError.parse(_err.logs);
    assert.strictEqual(err.error.errorCode.code, "ConstraintSeeds");
}
});
```

The benefits of this tool include increased productivity (less boilerplate), strong typing support (IDL + TS types), and automatic account deserialization, which allows simple assertions:

```ts
// Fetch and verify the on-chain data.
let dataAccount = await program.account.myData.fetch(data);
assert.deepEqual(dataAccount.authority, user.publicKey);
assert.strictEqual(dataAccount.counter, 0);
```

Because it is aligned with the Anchor ecosystem, this is often the easiest path for teams who use Anchor for both their on-chain Rust programs and their TypeScript clients.



#### @solana/web3.js (Solana’s core JavaScript SDK)

Another major tool for integration testing is the foundational library `@solana/web3.js`. This is the standard JavaScript/TypeScript SDK offered by Solana that exposes primitives such as Connection, TransactionInstruction, Transaction, Keypair, and so on. It is lower-level than Anchor’s abstractions and does not presume any IDL or account schema generation.

When using `web3.js` for integration tests, you often manually build transactions, manually decode accounts (for example using `Borsh` if your accounts are serialized that way), specify all the `AccountMeta` fields, and send transactions through a local validator (e.g., `solana-test-validator`). Because you are working at the primitive level, you have maximum control: you might simulate unusual edge cases, custom account metadata, or CPIs across several programs, and you’re not constrained by Anchor’s conventions.

Here is an example of an integration test via `web3.js`:
```ts
  it("initializes via raw instruction", async () => {
    // discriminator + borsh-encoded args (count: u8)
    const dataBytes = Buffer.concat([ixDiscriminator("initialize"), Buffer.from([10])]);
    const ix = new TransactionInstruction({
      programId: program.programId,
      keys: [
        { pubkey: user.publicKey, isSigner: true, isWritable: true },
        { pubkey: data, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: dataBytes,
    });

    const tx = new Transaction().add(ix);
    await provider.sendAndConfirm(tx, [user]);

    const acc = await program.account.myData.fetch(data);
    assert.deepEqual(acc.authority, user.publicKey);
    assert.strictEqual(acc.counter, 0);
  });
```

The benefit of `web3.js` in integration testing is flexibility and universality: whether your program is built with Anchor or not, as long as you know the instruction layout and account schema you can write tests. But the cost is extra boilerplate, slower iteration (you’ll write more code to set up accounts and transactions manually), and a greater possibility of mistakes in serialization or account setup.

#### Other libraries: gill, @solana/kit, etc.

Beyond `Anchor` and `@solana/web3.js`, a new generation of tools is reshaping Solana development — led by `Gill` and `@solana/kit`.
`Gill` is a modern TypeScript/JavaScript SDK built on top of `@solana/kit` (the successor to `web3.js`). It focuses on developer experience, offering cleaner APIs, strong typing, and reduced boilerplate. Instead of manually managing instructions and accounts, developers can define programs and interactions declaratively, with automatic handling of serialization, PDA derivation, and transactions. Because `Gill` is built on the new Kit runtime, it benefits from a modular and promise-based architecture, making it ideal for modern dApp development and integration testing that simulates real client behavior.

When it comes to Anchor programs, `Gill` can also be used successfully. It communicates with any Solana program at the RPC level, so Anchor’s on-chain programs are fully compatible. However, unlike the `@coral-xyz/anchor` client, `Gill` does not automatically process Anchor IDLs, derive PDAs, or provide account helpers. Developers need to manually define program interfaces and account structures. This trade-off offers greater flexibility and independence from the `Anchor framework`, but comes with slightly more setup effort.


### How to Choose Between These Tools

| Situation | Recommended Tool | Why |
| --- | --- | --- |
| Rust integration tests for an Anchor program | Anchor Testing Environment | Gives you Anchor’s macros, IDL helpers, and account setup out of the box. |
| Rust integration tests for a non-Anchor program | Solana Program Test Framework | Lightweight simulator for raw Solana programs without Anchor dependencies. |
| TypeScript tests for Anchor programs | `@coral-xyz/anchor` | Fastest path to tests aligned with your IDL, accounts, and strong typing. |
| Custom instruction layout or CPI-heavy flows without Anchor | `@solana/web3.js` | Maximum flexibility at the cost of more boilerplate and manual serialization. |
| Client-facing flows (wallet UX, UI logic, batching) | gill / Kit | Developer-friendly SDK focused on app-level interactions, though examples are still maturing. |


## Common Errors

### Signer Verification Failed

#### Cause

This error occurs when a required signer has not signed the transaction, which prevents the program from verifying authority for that action.

#### Example

In this example, `user` is set as payer for initializing the `data` account. However, this transaction will fail because each account has to also sign its initialization.

```ts
it("Is initialized!", async () => {
const tx = await program.methods
    .initialize()
    .accountsStrict({
    user: user.publicKey,
    data: data.publicKey,
    systemProgram: SystemProgram.programId,
    })
    .signers([user]) // Missing signer.
    .rpc();
});
```
#### Solution

If an account is a PDA, we do not include it as a signer because the program will sign on its behalf. In this case, both `data` and `user` are regular accounts, so they both need to be included as signers.

```ts
.signers([user, data])
```

### No Prior Credit

#### Cause

This error typically occurs when a transaction requires the payer account to have enough SOL to cover the cost of the transaction, but it has insufficient balance.

#### Example

In this example, we are again calling the `initialize` instruction again. However, this time there is nothing wrong with the code itself. The problem is that we did not ensure that accounts used for testing had sufficient balance to cover the transaction costs.

```ts
it("Is initialized!", async () => {
const tx = await program.methods
    .initialize()
    .accountsStrict({
    user: user.publicKey,
    data: data.publicKey,
    systemProgram: SystemProgram.programId,
    })
    .signers([user, data])
    .rpc();
});
```

#### Solution

Always make sure to airdrop SOL to the accounts used for testing before running the tests.

```ts
before("prepare", async () => {
    await airdrop(connection, user.publicKey);
});
```

Here is an example `airdrop` function you can use:

```ts
async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}
```

You can retrieve account balance using the `getBalance` function:

```ts
await connection.getBalance(user.publicKey)
```

### Account Already In Use

#### Cause

This error occurs when trying to initialize an account that already exists.

#### Example

In this example, we are trying to call `initialize` instruction twice with the same `data` account.

```ts
it("Is initialized!", async () => {
const tx = await program.methods
    .initialize()
    .accountsStrict({
    user: user.publicKey,
    data: data.publicKey,
    systemProgram: SystemProgram.programId,
    })
    .signers([user, data])
    .rpc();

const repeat_tx = await program.methods
    .initialize()
    .accountsStrict({
    user: user.publicKey,
    data: data.publicKey,
    systemProgram: SystemProgram.programId,
    })
    .signers([user, data])
    .rpc();
});
```

#### Solution

Whenever this error arises, make sure to check that the correct accounts are being passed in the transaction. In our example, simply use a different account for the second `initialize` call.

```ts
const repeat_tx = await program.methods
    .initialize()
    .accountsStrict({
    user: user.publicKey,
    data: data2.publicKey,
    systemProgram: SystemProgram.programId,
    })
    .signers([user, data2])
    .rpc();
```

### Account Did Not Deserialize

#### Cause

This error occurs when a program cannot interpret the data in an account according to its expected struct format.

#### Example

In this example, we are working with the context of `initialize` instruction, which initializes `data` account of type `MyData`. However, the allocated space does not include the 8 bytes required for the account discriminator, which is needed for deserialization.

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(init,
        space = 32 + 1,
        payer = user,
    )]
    data: Account<'info, MyData>,
    system_program: Program<'info, System>,
}
```

```rust
#[account]
pub struct MyData {
    authority: Pubkey,
    counter: u8,
}
```

#### Solution

Deserialization issues often result from mismatched data formats or incorrect space allocation. To fix this, allocate 8 more bytes for the account discriminator.

```rust
#[account(init,
    space = 8 + 32 + 1,
    payer = user,
)]
data: Account<'info, MyData>,
```

### Constraint Seeds

#### Cause

This error occurs when the seeds provided to derive a PDA do not match those specified in the program. It’s crucial to keep the seed **order** and **values** consistent.

#### Example

In this example, we have defined `data` as a PDA in the program:

```rust
#[account(init,
    space = 8 + 32 + 1,
    payer = user,
    seeds = [b"data1", b"data2"],
    bump
)]
data: Account<'info, MyData>,
```

Now, let’s derive the PDA for `data` in our test to pass it to a transaction. However, neither `option1` nor `option2` will work as expected:

`option1` uses incorrect values:

```ts
const option1 = PublicKey.findProgramAddressSync(
[Buffer.from("data"), Buffer.from("data2")],
program.programId
)[0];
```

`option2` uses correct seed values but in the wrong order:
```ts
const option2 = PublicKey.findProgramAddressSync(
[Buffer.from("data2"), Buffer.from("data1")],
program.programId
)[0];
```

#### Solution

To fix this error, make sure that both seed **order** and **values** match exactly what is specified in the program.

```ts
const correctPda = PublicKey.findProgramAddressSync(
[Buffer.from("data1"), Buffer.from("data2")],
program.programId
)[0];
```

### Program Failed To Complete

#### Cause

This error can happen when the program encounters a panic or an unhandled condition, causing the transaction to fail.

#### Example

In this example, we subtract two numbers inside `initialize` instruction:

```rs
pub fn initialize(ctx: Context<Initialize>, count: u8) -> Result<()> {
    let data = &mut ctx.accounts.data;

    data.authority = ctx.accounts.user.key();
    data.counter = 10 - count;

    Ok(())
}
```

We pass a value for count that is intentionally too high so that the subtraction underflows and transaction fails.

```ts
it("Is initialized!", async () => {
const tx = await program.methods
    .initialize(11) // The value 11 is intentionally too high.
    .accountsStrict({
    user: user.publicKey,
    data: data,
    systemProgram: SystemProgram.programId,
    })
    .signers([user])
    .rpc();
});
```

#### Solution

This time we are not really fixing the error, but we can make the program more resilient with custom error handling and clear error messages.

```rust
// Prevent underflow by checking that `count` does not exceed 10.
require!(count <= 10, MyError::InvalidInstructionData);
```

```rust
#[error_code]
pub enum MyError {
    // Add custom error for clearer error messages.
    #[msg("Invalid instruction data")]
    InvalidInstructionData,
}
```

Now the program will return a meaningful error if count is too high, preventing unexpected panics and making debugging easier.

-----

### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
