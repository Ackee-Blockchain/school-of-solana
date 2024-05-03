
# 1. Lecture - Introduction to Solana and Blockchain

In order not to waste time on our first lecture, please **prepare/install some stuff** in advance. 👇

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

# Command cheatsheet

## Solana CLI commands

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

## Anchor commands
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
    In separate window call:
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
        > This may give you something like:
        ```text
        Error: Account qxyKnYiTRpu4i4UobGXptrwjcmbEhswyVkMXX8PFPXT has insufficient funds for spend (1.90000344 SOL) + fee (0.00137 SOL)
        ```
        > This means that the program deployer has not enough funds to deploy the program to your local validator. You can resolve it by running
        ```bash
        solana config set --url locahost
        solana aidrop 500 qxyKnYiTRpu4i4UobGXptrwjcmbEhswyVkMXX8PFPXT # change the address correspondingly
        ```
    - Run again
        ```bash
        anchor test --skip-local-validator
        ```


-----

### Need help?
If you have any questions feel free to reach out to us at [Discord](https://discord.gg/z3JVuZyFnp).
