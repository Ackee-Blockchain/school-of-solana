# Common Issues
- [Insufficient Funds](#insufficient-funds)
- [Rustc version Mismatch](#rustc-versions-mismatch)
- [Program ID Mismatch](#program-id-mismatch)
- [WSL working directory](#wsl-working-directory)
- [How to Contribute](#how-to-contribute)


# Insufficient Funds
### Description
Running the Tests manually, i.e. starting `solana-test-validator` and using `anchor-deploy` and `anchor test --skip-local-validator` can result in:

>Error: Account qxyKnYiTRpu4i4UobGXptrwjcmbEhswyVkMXX8PFPXT has insufficient funds for spend (1.90000344 SOL) + fee (0.00137 SOL)

This means that the program deployer does not have enough funds to deploy the program to your Local Solana Validator.

### How to Fix
- use only `anchor test` - without manually starting the validator by yourself
- use `solana airdrop 500 <DEPLOYER_ADDRESS>` - this will require to have the config setup for the Localhost, and you need to know the deployer address. You can find the path to the deplyer wallet in the Anchor.toml file


# Rustc Versions mismatch
### Description
During the various commands, but mostly during the `anchor build` you may encounter `rustc` version mismatch as shown in the example below. Even thought the command `rustup default` show that you have sufficient version installed, the error still persists.

>error: package `solana-program v1.18.12` cannot be built because it requires rustc 1.75.0 or newer, while the currently active rustc version is 1.72.0-dev.

### How to Fix
- Solana CLI comes with its own `rustc` version, which will overwrite the installed rustc version. This means the only thing you need to do is to update the Solana CLI. If your program uses `solana-program v1.18.12` , upgrade the Solana CLI to `1.18.12` like:
```bash
solana-install init 1.18.12
```

# Program ID Mismatch
### Description
You may experience behavior when you run your tests and you are getting an error with the error code as follows:
>AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id. This is because the program id declared in the source code, the program ID within the Anchor.toml and the program id expected by Anchor do not match.

### How to Fix
In order to fix this, run:
```bash
anchor keys list
```
Which will output the correct program ID. Make sure that this program ID is included in the `Anchor.toml` and also in `declare_id!` macro. In the case of multiple programs, the command will output multiple program IDs, then the process is the same.

---

# WSL working directory
### Description

You installed WSL, and then started using it, but errors occurred. For example, you are not able to start `solana-test-validator`, or you are unable to build an anchor project using `anchor build`, with the error stating that you don't have the desired version. These problems often arise because WSL, by default, starts in the Windows system directory, such as the home or user directory.

### How to Fix

To fix these issues, just create a new directory inside the installed Linux subsystem, for example:

```bash
mkdir -p /home/solana/school-of-solana
cd /home/solana/school-of-solana
```

And perform everything inside this folder.

## How to Contribute
Write New Issue in the following format

## Issue Name
### Description
short description + optionally error message
### How to Fix
write steps how to fix it or what to do
