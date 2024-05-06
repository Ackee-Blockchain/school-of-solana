# Common Issues
- [Insufficient Funds](#insufficient-funds)
- [Rustc version Mismatch](#rustc-versions-mismatch)
- [How to Contribute](#how-to-contribute)


## Insufficient Funds
### Description
Running the Tests manually, i.e. starting `solana-test-validator` and using `anchor-deploy` and `anchor test --skip-local-validator` can result in:
```text
Error: Account qxyKnYiTRpu4i4UobGXptrwjcmbEhswyVkMXX8PFPXT has insufficient funds for spend (1.90000344 SOL) + fee (0.00137 SOL)
```
This means that the program deployer does not have enough funds to deploy the program to your Local Solana Validator.

### How to Fix
- use only `anchor test` - without manually starting the validator by yourself
- use `solana airdrop 500 <DEPLOYER_ADDRESS>` - this will require to have the config setup for the Localhost, and you need to know the deployer address. You can find the path to the deplyer wallet in the Anchor.toml file

## Rustc Versions mismatch
### Description
During the various commands, but mostly during the `anchor build` you may encounter `rustc` version mismatch as shown in the example below. Even thought the command `rustup default` show that you have sufficient version installed, the error still persists.
```text
error: package `solana-program v1.18.12` cannot be built because it requires rustc 1.75.0 or newer, while the currently active rustc version is 1.72.0-dev.
```
### How to Fix
- Solana CLI comes with its own `rustc` version, which will overwrite the installed rustc version. This means the only thing you need to do is to update the Solana CLI. If your program uses `solana-program v1.18.12` , upgrade the Solana CLI to `1.18.12` like:
```bash
solana-install init 1.18.12
```



---

## How to Contribute
Write New Issue in the following format

## Issue Name
### Description
short description + optionally error message
### How to Fix
write steps how to fix it or what to do
