use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("5fA4GU4TwqDxJv58Uq1jBDVWCevJXekRgMbwT5cofSi5");

// Defines the main functionalities of the puppet_master program.
#[program]
mod puppet_master {
    use super::*;

    // Function to manipulate data on a puppet account via a cross-program invocation (CPI).
    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        // Convert the puppet_program account into account info for the CPI call.
        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        // Set up the CPI accounts expected by the puppet::set_data function.
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet.to_account_info(),
        };
        // Create a CPI context with the program and accounts to call.
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // Perform the CPI call to set data on the puppet account.
        let result = puppet::cpi::set_data(cpi_ctx, data)?;
        // The below statement calls sol_get_return and deserializes the result.
        // `return_data` contains the return from `set_data`,
        // which in this example is just `data`.
        let return_data = result.get();
        if return_data != 42 {
            panic!();
        }
        Ok(())
    }
}

// Specifies the accounts required for the `pull_strings` function.
#[derive(Accounts)]
pub struct PullStrings<'info> {
    // References a mutable puppet account to manipulate its data.
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    // Reference to the puppet program to enable CPI calls to it.
    pub puppet_program: Program<'info, Puppet>,
}
