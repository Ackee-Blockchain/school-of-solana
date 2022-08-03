use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Po8QNbexAedzyagU3ZupR7pLeaqe4BmJmrNCaJ8xu6h");


/// TODO fix this program
#[program]
pub mod type_cosplay_insecure {
    use super::*;

    pub fn add_user(ctx: Context<AddUser>) -> Result<()> {
        // create user
        let user = User {
            authority: *ctx.accounts.authority.to_account_info().key,
            vault: *ctx.accounts.vault.key,
        };
        user.serialize(&mut &mut (*ctx.accounts.user.data).borrow_mut()[..])?;
        // create its vault (everybody gets a 10 tokens at the beginning)
        let vault = Vault {
            balance: 10,
        };
        vault.serialize(&mut &mut (*ctx.accounts.vault.data).borrow_mut()[..])?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user = User::deserialize(&mut &(*ctx.accounts.user.data).borrow_mut()[..]).unwrap();
        msg!("user.authority: {} user.vault {}", user.authority, user.vault);
        let mut dst_vault = Vault::deserialize(&mut &(*ctx.accounts.destination_vault.data).borrow_mut()[..]).unwrap();
        let mut src_vault = Vault::deserialize(&mut &(*ctx.accounts.source_vault.data).borrow_mut()[..]).unwrap();
        
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner.into());
        }
        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }
        if user.vault != *ctx.accounts.source_vault.key {
            return Err(ProgramError::InvalidAccountData.into());
        }
        if amount > src_vault.balance {
            return Err(ProgramError::InsufficientFunds.into());
        }

        msg!("Sending {} token from {} to {}", amount, user.vault, *ctx.accounts.destination_vault.key);
        // transfer happening...
        src_vault.balance -= amount;
        dst_vault.balance += amount;
        src_vault.serialize(&mut &mut (*ctx.accounts.source_vault.data).borrow_mut()[..])?;
        dst_vault.serialize(&mut &mut (*ctx.accounts.destination_vault.data).borrow_mut()[..])?;
        
        // save metadata about transfer
        let meta = TransferMetadata {
            sender: *ctx.accounts.authority.to_account_info().key,
            dst_vault: *ctx.accounts.destination_vault.key,
            src_vault: user.vault,
        };
        meta.serialize(&mut &mut (*ctx.accounts.meta.data).borrow_mut()[..])?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 32 + 32,
    )]
    user: AccountInfo<'info>,
    #[account(
        init, 
        payer = authority, 
        space = 8,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 32 + 32 + 32,
    )]
    meta: AccountInfo<'info>,
    user: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut)]
    source_vault: AccountInfo<'info>,
    #[account(mut)]
    destination_vault: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    pub authority: Pubkey,
    pub vault: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferMetadata {
    pub sender: Pubkey,
    pub dst_vault: Pubkey,
    pub src_vault: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub balance: u64,
}
