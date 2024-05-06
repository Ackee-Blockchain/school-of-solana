use crate::BankAccount; // Import the BankAccount struct
use anchor_lang::prelude::*; // Import necessary modules and items

// Define the deposit function for depositing funds into a bank account
pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Create a transfer instruction
    let txn = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(), // Source account
        &ctx.accounts.bank.key(), // Destination account
        amount,                   // Amount to transfer
    );

    // Invoke the transfer instruction
    anchor_lang::solana_program::program::invoke(
        &txn, // Transaction to invoke
        &[
            // Accounts involved in the transaction
            ctx.accounts.user.to_account_info(),
            ctx.accounts.bank.to_account_info(),
        ],
    )?;

    // Increase the balance of the bank account
    ctx.accounts.bank.balance += amount;

    // Return success
    Ok(())
}

// Define the Deposit accounts structure
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)] // Mutable reference to the user's account
    pub user: Signer<'info>,
    #[account(mut)] // Mutable reference to the bank account
    pub bank: Account<'info, BankAccount>,
    pub system_program: Program<'info, System>, // System program
}
