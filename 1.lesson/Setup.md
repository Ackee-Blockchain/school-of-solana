# Manual Setup Guide for School of Solana


This guide will walk you through setting up your environment for the School of Solana if you prefer not to use the provided Docker image. Follow the steps below to install all the necessary tools and dependencies.

### System Requirements
Use WSL 2.0 with Ubuntu 22.04 distro or standalone Ubuntu 22.04.


> [!TIP]
> **Install WSL 2.0**
> 
> Follow this [installation guide](https://learn.microsoft.com/en-us/windows/wsl/install).

## Setup

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

[Rust installation guide](https://www.rust-lang.org/tools/install)

Set the default version:

```bash
rustup install 1.86.0
rustup default 1.86.0
```

Verify the installation:

```bash
rustc --version
cargo --version
```


### 3. Install Solana CLI

[Solana installation guide](https://docs.anza.xyz/cli/install/)

Set the default version:

```bash
agave-install init 2.2.12
```

Verify the installation:

```bash
solana --version
```

### 4. Install Node.js and Yarn

Install Node.js and Yarn for managing JavaScript dependencies:

```bash
npm install --global yarn
```

### 5. Install Anchor CLI

[Anchor installation guide](https://www.anchor-lang.com/docs/installation)

Set the default version:
```bash
avm install 0.31.1
avm use 0.31.1
```

Verify the installation:

```bash
anchor --version
```
