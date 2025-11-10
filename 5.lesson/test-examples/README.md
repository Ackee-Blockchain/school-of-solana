# 🧪 Test Examples (Anchor workspace)

This workspace demonstrates 4 testing approaches for the same Anchor program (`test-examples`).  
Each test interacts with the same `initialize(count: u8)` instruction, which writes the authority and computes a counter using `math_function`.

---

## 📚 What’s Included

| Type | Description | Location |
|------|--------------|-----------|
| **Unit (Rust’s Native Testing Framework)** | Rust-only test for pure logic (`math_function`) without Solana runtime. | `programs/test-examples/tests/unit.rs` |
| **Integration (Rust + Anchor Testing Client)** | Rust + Anchor Client test. Connects to a running local validator and executes `initialize`. | `programs/test-examples/tests/integration.rs` |
| **Integration (Anchor TS)** | TypeScript test using the Anchor TS client (`program.methods.initialize`) over RPC. | `tests/anchor-test.ts` |
| **Integration (Raw web3.js)** | TypeScript test that manually builds and sends the raw instruction (discriminator + args). | `tests/web3-raw-test.ts` |

---

## What Each Test Does
- Unit (`programs/test-examples/tests/unit.rs`)
  - Calls `math_function(count)` and checks results (e.g., if number <= 10 then return 10 - number; if number > 10 return None;).
  - Goal: verify core logic quickly without Solana runtime.
- Integration: Rust + Anchor Testing Client (`programs/test-examples/tests/integration.rs`)
  - Uses `Anchor Client` with a running local validator to execute `initialize` via real RPC. Performs an airdrop to the test wallet, sends a signed transaction, and fetches on-chain data using `program.account()`.
  - Goal: validate instruction handling, PDAs, and account transitions deterministically.
- Integration: Anchor TS (`tests/anchor-test.ts`)
  - Executes the `initialize(10)` instruction using the Anchor TypeScript client (`program.methods.initialize`) and verifies the on-chain `myData` account values through IDL deserialization.
  - Goal: to replicate a real-world usage scenario of the program via the Anchor SDK and ensure that the flow (client → transaction → program → state) works correctly.
- Integration: Raw web3.js (`tests/web3-raw-test.ts`)
  - Manually constructs and sends a raw `TransactionInstruction` for initialize, including discriminator and Borsh-encoded arguments, without relying on Anchor client helpers.
  - Goal: to validate low-level instruction encoding, account configuration, and transaction handling independently of the Anchor framework.

## ⚙️ Prerequisites

Before running tests, make sure you have the following:

- **Solana CLI** and **local validator** installed:  
  ```bash
  solana-test-validator -V
- **Anchor CLI** installed and configured for localnet:
  ```bash
  anchor --version
- Node.js 18+ and Yarn/NPM for TypeScript tests.
- Install JS deps in this folder: `npm i` or `yarn`

## ⚙️ Project Layout
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
## 🚀 Running Tests

- Rust unit + integration (integration test requires active validator, read next section for more info): `cd programs/test-examples && cargo test`
- Client based tests: `anchor test`

## ⚠️ Important Notes for Rust Integration Tests
When you use `anchor test` you do not need to start validator by yourself, because anchor do it automatically:
  - Automatically starts a local validator (solana-test-validator) before running the tests.
  - Deploys your program(s) to that validator.
  - Executes all TypeScript/JavaScript tests in the /tests directory using testing framework (e.g. Mocha).
  - Shuts down the validator automatically when the test suite completes.

  > [!WARNING]
  > If you manually start a validator beforehand and then run `anchor test`. You may encounter an error such as “port already in use”, since Anchor will attempt to launch its own validator instance on the same port.


But when you run Rust integration tests, then the local validator must be running before testing.
Unlike anchor test, cargo test does not start a validator automatically. 

Start it manually in a separate terminal (you must be in the `programs/test-examples` folder):
  ```bash
  anchor localnet
  ```
or
  ```bash
  solana-test-validator --reset
  ```
after local validator will be started, you can run Rust integration test.