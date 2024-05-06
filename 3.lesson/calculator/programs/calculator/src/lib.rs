// Import necessary items from the Anchor framework.
use anchor_lang::prelude::*;

// Declare a unique identifier for the program.
declare_id!("Bo48JUrbWoLNhByiwAXofSsa4eJFZvvnENj8gDHbAMAu");

// Define a module named calculator marked as a Solana program, including necessary imports.
#[program]
pub mod calculator {
    use super::*;

    // Initialize the Calculator account with a greeting message provided by the user.
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    // Perform addition on two numbers provided as arguments and store the result in the Calculator account.
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
}

// Define the accounts required for the create function, including the user's account,
// the Calculator account to be initialized, and the system program account.
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+256
    )]
    pub calculator: Account<'info, Calculator>,
    pub system_program: Program<'info, System>,
}

// Define the accounts required for the add function, which only includes the Calculator account.
#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Define the data structure for the Calculator account, containing a
// greeting message and a field to store the result of operations.
#[account]
pub struct Calculator {
    greeting: String,
    result: i64,
}
