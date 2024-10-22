
# 1. Lecture - Introduction to Solana and Blockchain

In order not to waste time on our first lecture, please **prepare/install some stuff** in advance. 👇

## Table of Contents
- [Dev Setup](#dev-setup)
- [Solana Handbook](#solana-handbook)
- [Command cheatsheet](#command-cheatsheet)
    - [Solana commands](#solana-cli-commands)
    - [Anchor commands](#anchor-commands)


## Dev Setup

> [!TIP]
> For step by step guide check [Manual Setup](./Setup.md)

[WSL]: https://learn.microsoft.com/en-us/windows/wsl/install
[Rust]: https://www.rust-lang.org/tools/install
[Solana]: https://docs.solana.com/cli/install-solana-cli-tools
[Anchor]: https://www.anchor-lang.com/docs/installation
[VSCode]: https://code.visualstudio.com/
[RustAnalyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[Docker Image]: https://docs.docker.com/engine/install/


| Setup | Description | Version | How to Install |
|- | - | - | - |
| Windows subsystem for Linux(WSL) | optional but highly recommended | 2.0 | [Instructions][WSL] |
| Rust | - | 1.79.0 | [Instructions][Rust] |
| Solana tool suite | | 1.18.18 | [Instructions][Solana] |
| Anchor framework | - | 0.30.1 | [Instructions][Anchor] |
| VSCode | chose your own preferred IDE | - | [Instructions][VSCode] |
| RustAnalyzer | optional extension for VSCode | - | [Instructions][RustAnalyzer] |
| Docker | [Install Docker][Docker Image] | - | [Instructions](./Docker.md) |


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
    solana config set -u l # shorter option, l stands for localhost
    ```
    ```bash
    solana config set --url devnet # useful for devnet testing
    solana config set -u d # shorter option, d stands for devnet
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
    > You can also use the [Solana Faucet](https://faucet.solana.com/) to get some SOL.

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

-----

### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
