use anchor_lang::prelude::*;

// Import necessary modules and items
use crate::{BankAccount, BANK_ACCOUNT_SEED};

// Define the create function for initializing a bank account
pub fn _create(ctx: Context<Create>, name: String) -> Result<()> {
    // Get mutable reference to the bank account
    let bank = &mut ctx.accounts.bank;
    // Set bank name
    bank.name = name;
    // Initialize balance to zero
    bank.balance = 0;
    // Set owner of the bank account
    bank.owner = *ctx.accounts.user.key;
    // Return success
    Ok(())
}

// Define the Create accounts structure
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)] // Mutable reference to the user's account
    pub user: Signer<'info>,
    #[account( // Bank account to be created
        init, // Initialize account
        payer=user, // Account that pays for initialization
        space=100, // Space required for the account
        seeds=[BANK_ACCOUNT_SEED.as_bytes(), user.key().as_ref()], // Seeds for account address derivation
        bump // Bump seed
    )]
    pub bank: Account<'info, BankAccount>, // Bank account type
    pub system_program: Program<'info, System>, // System program
}
