# 1. Lecture - Introduction to Solana and Blockchain

To make the most of our first lecture, please **set up your development environment** in advance.

## Table of Contents
<!-- no toc -->
- [Dev Setup](#dev-setup)
- [Solana Handbook](#solana-handbook)
- [Command cheatsheet](#command-cheatsheet)
  - [Solana CLI](#solana-cli-commands)
  - [Anchor](#anchor-commands)


## Dev Setup

>[!TIP]
>
> Checkout [Manual Setup](./Setup.md) section for step by step guide.

[WSL]: https://learn.microsoft.com/en-us/windows/wsl/install
[Rust]: https://www.rust-lang.org/tools/install
[Solana]: https://docs.solana.com/cli/install-solana-cli-tools
[Anchor]: https://www.anchor-lang.com/docs/installation
[VSCode]: https://code.visualstudio.com/
[RustAnalyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer


| Setup | Version | Installation | Note
|- | - | - | - |
| WSL | 2.0 | [Guide][WSL] | Highly recomended
| Rust |  1.86.0 | [Guide][Rust] |
| Solana | 2.2.12 | [Guide][Solana] |
| Anchor |  0.31.1 | [Guide][Anchor] |
| VS Code | | [Guide][VSCode] | Or any other favorite IDE
| Rust Analyzer |  | [Guide][RustAnalyzer] | Highly recommended
| Docker | |  [Guide](./Docker.md) | To use pre-configured docker image


## Solana Handbook

This is the main learning material for the first lecture. It is our introductory material to blockchain and Solana. Getting comfortable with it will help you pass the first task!

[**Solana Handbook**](https://ackee.xyz/solana/book/latest/)

## Command cheatsheet

### Solana CLI

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
    > [!NOTE]
    > Airdrop only works on **devnet**, **testnet** and **localhost**.
    ```bash
    solana airdrop 5
    ```
    > [!TIP]
    > You can also optionally specify the destination address of the airdrop.
    ```bash
    solana airdrop 5 <YOUR_PUBKEY>
    ```
    > [!TIP]
    > You can also use the [Solana Faucet](https://faucet.solana.com/) to get some SOL.

- #### Get PubKey from Keypair
    ```bash
    solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
    ```
- #### Run Solana test validator
    > [!NOTE]
    > In **most** cases you **DO NOT** need to start the local validator by yourself. Down below you can find the **Anchor commands** which will handle everything for you.
    ```bash
    solana-test-validator
    ```
- #### Get logs from the Solana validator
    ```bash
    solana logs
    ```

### Anchor

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
    In a separate terminal tab, call:
    ```bash
    solana-test-validator
    ```
    Within the anchor project directory:

    - Build the project
        ```bash
        anchor build
        ```
    - Run tests without starting the local validator (started it manually in the step above)
        ```bash
        anchor test --skip-local-validator
        ```

-----

### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
