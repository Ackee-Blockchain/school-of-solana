# Manual Setup Guide for School of Solana


This guide will walk you through setting up your environment for the School of Solana if you prefer not to use the provided Docker image. Follow the steps below to install all the necessary tools and dependencies.

### System Requirements
- WSL 2.0 with Ubuntu 20.04 distro or
- standalone Ubuntu 20.04

> [!TIP]
> **Install WSL 2.0**
> You can check this guide [Install WSL 2.0](https://learn.microsoft.com/en-us/windows/wsl/install)



### 1.  Install System Dependencies

Update your package list and install the required packages:

```bash
sudo apt-get update
sudo apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    npm \
    vim \
    nano \
    wget \
```

### 2. Install Rust

Install Rust using the [Install Rust](https://www.rust-lang.org/tools/install)

Set the default version:

```bash
rustup install 1.79.0
rustup default 1.79.0
```

Verify the installation:

```bash
rustc --version
cargo --version
```


### 3. Install Solana CLI

Install Solana CLI using the [Install the Solana CLI](https://docs.solanalabs.com/cli/install)

Set the default version:

```bash
solana-install init 1.18.18
```

Verify the installation:

```bash
solana --versions
```

### 4. Install Node.js and Yarn

Install Node.js and Yarn for managing JavaScript dependencies:

```bash
npm install --global yarn
```


### 5. Install Anchor CLI

Install Anchor Framework using the [Anchor Installation](https://www.anchor-lang.com/docs/installation)

Set the default version:
```bash
avm install 0.30.1
avm use 0.30.1
```

Verify the installation:

```bash
anchor --version
```
