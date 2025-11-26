# Test Examples (Anchor Workspace)

This workspace demonstrates four testing approaches for the same Anchor program (`test-examples`). Each scenario targets the shared `initialize(count: u8)` instruction, which records the authority and computes a counter via `math_function`.

---

## What’s Included

| Type | Description | Location |
|------|-------------|----------|
| **Unit (Rust’s Native Testing Framework)** | Pure Rust test of `math_function`; no Solana runtime involved. | `programs/test-examples/tests/unit.rs` |
| **Integration (Rust + Anchor Client)** | Rust test that uses Anchor Client against a running local validator. | `programs/test-examples/tests/integration.rs` |
| **Integration (Anchor TS)** | TypeScript test built with the Anchor TS client (`program.methods.initialize`). | `tests/anchor-test.ts` |
| **Integration (Raw web3.js)** | TypeScript test that assembles and sends the instruction manually (discriminator + args). | `tests/web3-raw-test.ts` |

---

## What Each Test Does
- **Unit (`programs/test-examples/tests/unit.rs`)**
  - Calls `math_function(count)` and checks the outcomes (≤ 10 → `Some(10 - count)`, > 10 → `None`).
  - Goal: validate core math logic quickly without touching the Solana runtime.
- **Integration – Rust + Anchor Client (`programs/test-examples/tests/integration.rs`)**
  - Uses Anchor Client plus a running local validator to execute `initialize`, including wallet airdrop, signed transaction, and fetching on-chain data via `program.account()`.
  - Goal: confirm instruction handling, PDA derivations, and account transitions deterministically.
- **Integration – Anchor TS (`tests/anchor-test.ts`)**
  - Executes `initialize(10)` using the Anchor TypeScript client and verifies the `myData` account through IDL-powered deserialization.
  - Goal: mirror a real-world Anchor SDK workflow (client → transaction → program → state).
- **Integration – Raw web3.js (`tests/web3-raw-test.ts`)**
  - Manually constructs and sends a `TransactionInstruction` for `initialize`, encoding the discriminator and Borsh arguments without Anchor helpers.
  - Goal: validate low-level instruction encoding, account metadata, and transaction handling independently of Anchor.

## Prerequisites

Before running the tests, make sure you have:

- **Solana CLI** and a local validator installed:
  ```bash
  solana-test-validator -V
  ```
- **Anchor CLI** configured for localnet:
  ```bash
  anchor --version
  ```
- Node.js 18+ plus Yarn or npm for the TypeScript suites.
- JavaScript dependencies installed in this folder (`npm i` or `yarn`).

## Project Layout
```text
programs/test-examples/
├── src/lib.rs                 # Anchor program source
├── tests/
│   ├── unit.rs                # Rust unit tests (pure logic)
│   └── integration.rs         # Rust integration tests (with Anchor client)
tests/
├── anchor-test.ts             # Anchor client test (Mocha/Chai)
└── web3-raw-test.ts           # Raw web3.js instruction test
```

## Running Tests

- Rust unit + integration (integration test requires an active validator — see the next section): `cd programs/test-examples && cargo test`
- Client-based tests: `anchor test`

## Important Notes for Rust Integration Tests
When you run `anchor test`, you do **not** need to start a validator manually because Anchor handles it for you:
  - Starts a local validator (`solana-test-validator`) before executing tests.
  - Deploys your program(s) to that validator.
  - Runs every TypeScript/JavaScript test in the `tests/` directory (e.g., via Mocha).
  - Shuts the validator down after the suite finishes.

  > [!WARNING]
  > If you start your own validator and then run `anchor test`, you may see “port already in use,” since Anchor tries to launch its own validator on the same port.

Rust integration tests behave differently: `cargo test` does **not** start a validator. You must have one running beforehand.

Start it manually in a separate terminal (from `programs/test-examples`):
  ```bash
  anchor localnet
  ```
or
  ```bash
  solana-test-validator --reset
  ```
Once the validator is up, run the Rust integration tests. 
