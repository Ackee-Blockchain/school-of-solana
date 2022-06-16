<div align="center">

# SSoS - 1. Lecture

In order not to waste time on our first lecture, please **prepare/install some stuff** in advance. 👇

</div>

# 👨‍💻 Dev Setup

- [Rust](https://www.rust-lang.org/tools/install) 1.61.0
- [NVM](https://github.com/nvm-sh/nvm)
- [Typescript](https://www.typescriptlang.org/download)
- [Solana tool suite](https://docs.solana.com/cli/install-solana-cli-tools) v1.10.25
- [Anchor](https://book.anchor-lang.com/getting_started/installation.html) 0.24.2
- [VSCode](https://code.visualstudio.com/) optional
- [RustAnalyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# 👨‍💻 Solana Handbook

- 30 pages of introduction to Solana by Ackee Blockchain
- Created for Summer School of Solana 💜

Get it here 👉 👉 👉
https://ackeeblockchain.com/solana-handbook.pdf

# Command cheatsheet

### Set CLI config url to localhost cluster

```bash
solana config set --url localhost
```

### Get current config

```bash
solana config get
```

### Create CLI Keypair

```bash
solana-keygen new
vs
solana-keygen new -o test.json
```

If the `--no-outfile` flag is omitted
the default behavior is to write the keypair to `~/.config/solana/id.json`.

### Get PubKey from Keypair

```bash
solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
```

### Run Solana test validator

```bash
solana-test-validator
vs
solana-test-validator -r
```

### Airdrop (test-validator)

```bash
solana airdrop 10 <PUBKEY>
```

### Get logs from Solana validator

```bash
solana logs
```
