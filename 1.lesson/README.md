
# 1. Lecture - Introduction to Solana and Blockchain

In order not to waste time on our first lecture, please **prepare/install some stuff** in advance. 👇

## Table of Contents
- [Dev Setup](#dev-setup)
- [Solana Handbook](#solana-handbook)
- [Command cheatsheet](#command-cheatsheet)
    - [Solana commands](#solana-cli-commands)
    - [Anchor commands](#anchor-commands)


## Dev Setup
[WSL]: https://learn.microsoft.com/en-us/windows/wsl/install
[Rust]: https://www.rust-lang.org/tools/install
[Solana]: https://docs.solana.com/cli/install-solana-cli-tools
[Anchor]: https://www.anchor-lang.com/docs/installation
[nvm]: https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating
[node]: https://nodejs.org/en/download/package-manager
[VSCode]: https://code.visualstudio.com/
[RustAnalyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[Docker Image]: https://docs.docker.com/engine/install/


| Setup                                      | Description                     | Version               | How to Install                |
| -------------------------------------------| --------------------------------| ----------------------| ------------------------------|
| Windows subsystem for Linux(WSL)           | optional but highly recommended | 2.0                   | [Instructions][WSL]           |
| Rust                                       | -                               | stable                | [Instructions][Rust]          |
| Solana tool suite                          | -                               | 1.18.18               | [Instructions][Solana]        |
| Anchor framework                           | -                               | 0.31.1                | [Instructions][Anchor]        |
| nvm                                        | node.js version manager         | -                     | [Instructions][nvm]           |
| node                                       | js runtime                      | -                     | [Instructions][node]          |
| yarn                                       | preffered js package manager    | -                     | `npm install --global yarn`   |
| VSCode                                     | chose your own preferred IDE    | -                     | [Instructions][VSCode]        |
| RustAnalyzer                               | recommended extension for VSCode| -                     | [Instructions][RustAnalyzer]  |
| [Docker](#docker-image-setup)              | tool to run prebuilt images     | -                     | [Instructions][Docker Image]  |

## Docker Image setup

To make setup even easier, we prepared for you docker images for:
- x86 based systems (Windows, Linux, older Macs) - [todo](link)
- arm based systems (arm Mac) - [todo](link)

To use the pre-built Docker image for this course, you can pull the image from Docker Hub (using the correct url from the options above):
```bash
docker pull ackeexyz/solana-auditors-bootcamp:v1
```
Then run the following command. This will create new container.
```bash
docker run -it --name solana-auditors-bootcamp -p 8899:8899 -p 9900:9900 -p 8000:8000 -p 8080:8080 ackeexyz/solana-auditors-bootcamp:v1
```
Then visit the following url
  http://localhost:8080/
When you want to continue work inside the Docker Image, use
```bash
docker start solana-auditors-bootcamp
```
and again visit the URL.
To stop the Image, use
```bash
docker stop solana-auditors-bootcamp
```
It is also possible to start/stop the Image from the Docker Desktop GUI.

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

-----

### Need help?
If you have any questions feel free to reach out to us at [Discord](https://discord.gg/z3JVuZyFnp).
