
# 1. Lecture - Introduction to Solana and Blockchain

In order not to waste time on our first lecture, please **prepare/install some stuff** in advance. 👇

## Table of Contents
- [Dev Setup](#dev-setup)
- [Solana Handbook](#solana-handbook)
- [Command cheatsheet](#command-cheatsheet)
    - [Solana commands](#solana-cli-commands)
    - [Anchor commands](#anchor-commands)
- [Common Issues](#common-issues)
    - [Insufficient Funds](#insufficient-funds)
    - [Rustc version Mismatch](#rustc-versions-mismatch)


## Dev Setup
- [How to install Linux on Windows with WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
    > Strongly recommended to use WSL on Windows devices!
- [Rust](https://www.rust-lang.org/tools/install)
    > stable version
- [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools)
    > v1.18.11
- [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
    > 0.29.0
- [VSCode](https://code.visualstudio.com/)
    > optional. Chose your own preferred IDE
- [RustAnalyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
    > optional extension for VSCode

## Solana Handbook

- Introduction to Solana by Ackee Blockchain
[Solana Handbook](https://ackeeblockchain.com/solana-handbook.pdf)

## Command cheatsheet

### Solana CLI commands

- #### Get current config

    ```bash
    solana config get
    ```

- #### Set CLI config url to localhost cluster

    ```bash
    solana config set --url localhost # useful for local development
    ```
    ```bash
    solana config set --url devnet # useful for devnet testing
    ```
    More at [Clusters and Public RPC Endpoints](https://solana.com/docs/core/clusters)


- #### Create CLI Keypair
    ```bash
    solana-keygen new -o test.json
    ```
- #### Airdrop
    > As you may guess, Airdrop will only work on Devnet, Testnet or Localhost. No you cannot airdrop SOL on Mainnet!!
    ```bash
    solana airdrop 5
    ```
    > You can also optionally specify the destination address of the airdrop
    ```bash
    solana airdrop 5 <YOUR_PUBKEY>
    ```

- #### Get PubKey from Keypair
    ```bash
    solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
    ```
- #### Run Solana test validator
    > In **most cases (99%)** you **DO NOT NEED TO** start the local validator by yourself. **Down below** you can find the **Anchor commands** which will handle everything for you.
    ```bash
    solana-test-validator
    ```
- #### Get logs from the Solana validator
    ```bash
    solana logs
    ```

### Anchor commands
- #### Initialize new project
    ```bash
    anchor init <your_project_name>
    ```
- #### Build the project
    ```bash
    anchor build
    ```
- #### Test the project (preferred)
    ```bash
    anchor test
    ```
- #### Test the project (less preferred)
    In separate window, call:
    ```bash
    solana-test-validator
    ```
    Within the anchor project directory
    - Build the project
        ```bash
        anchor build
        ```
    - Run Tests without starting the local validator (as you started it manually in the step above)
        ```bash
        anchor test --skip-local-validator
        ```

## Common Issues
- ### Insufficient Funds
    Running the Tests manually, i.e. starting `solana-test-validator` and using `anchor-deploy` and `anchor test --skip-local-validator` can result in:
    ```text
    Error: Account qxyKnYiTRpu4i4UobGXptrwjcmbEhswyVkMXX8PFPXT has insufficient funds for spend (1.90000344 SOL) + fee (0.00137 SOL)
    ```
    This means that the program deployer does not have enough funds to deploy the program to your Local Solana Validator.
    - **To fix this**
        - use only `anchor test` - without manually starting the validator by yourself
        - use `solana airdrop 500 <DEPLOYER_ADDRESS>` - this will require to have the config setup for the Localhost, and you need to know the deployer address. You can find the path to the deplyer wallet in the Anchor.toml file
- ### Rustc Versions mismatch
    During the various commands, but mostly during the `anchor build` you may encounter `rustc` version mismatch as shown in the example below. Even thought the command `rustup default` show that you have sufficient version installed, the error still persists.
    ```text
    error: package `solana-program v1.18.12` cannot be built because it requires rustc 1.75.0 or newer, while the currently active rustc version is 1.72.0-dev.
    ```
    - **To fix this**
        - Solana CLI comes with its own `rustc` version, which will overwrite the installed rustc version. This means the only thing you need to do is to update the Solana CLI. If your program uses `solana-program v1.18.12` , upgrade the Solana CLI to `1.18.12` like:
            ```bash
            solana-install init 1.18.12
            ```

- ### How to Contribute
    short description + optionally error message
    - **To fix this**
        - write description what to do


-----



### Need help?
If you have any questions feel free to reach out to us at [Discord](https://discord.gg/z3JVuZyFnp).
