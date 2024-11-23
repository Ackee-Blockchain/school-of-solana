# 7. Lecture - Security

The focus of this week's lecture is security on Solana. We are taking a closer look at the most common attack vectors every developer should be familiar with, as well as the Solana runtime policy.

## Table of Contents
<!-- no toc -->
- [Common Solana Attack Vectors](#common-solana-attack-vectors)
- [Solana Runtime Policy](#solana-runtime-policy)
  - [Immutability](#immutability)
  - [Data](#data)
  - [Ownership](#ownership)
  - [Transaction](#transaction)
  - [Data Allocation](#data-allocation)
  - [Balance](#balance)
  - [Rent](#rent)
---

## Common Attack Vectors on Solana

We have prepared a repository containing a list of common attack vector on Solana.

The repository provides:
- Description of each attack.
- Strategy for mitigating the attacks.
- Example programs with proof-of-concept tests to demonstrate the attack vectors.

>[!IMPORTANT]
> [**Common Attack Vectors Repository**](https://github.com/Ackee-Blockchain/solana-common-attack-vectors)

## Solana Runtime Policy

### Immutability

Executable accounts become immutable when upgrade authority is set to null.

### Data

Only the owner of an account may modify its data.

### Ownership

Only the owner of an account may assign a new owner.

### Transaction

Total balances on all the accounts are equal before and after the execution of a transaction.

After the transaction is executed, balances of read-only accounts must be equal to the balances before the transaction.

All instructions in the transaction are executed atomically. If one fails, all account modifications are discarded.

### Data Allocation

Only the owner may change account size and data. And if the account is writable. And if the account is not executable.

Newly allocated account data is always zeroed out.

### Balance

Only the owner of an account may subtract its lamports.

Any program account may add lamports to an account.

### Rent

Rent fees are charged every epoch and are determined by account size.

Accounts with sufficient balance to cover 2 years of rent are exempt from fees.

-----



### Need help?
If you have any questions feel free to reach out to us on [Discord](https://discord.gg/z3JVuZyFnp).
