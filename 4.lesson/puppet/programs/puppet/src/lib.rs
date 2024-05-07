use anchor_lang::prelude::*;

declare_id!("7kZGeneY2LYUhjPoveEkriYFWinHLbMfDbBQpcTvQbip");

// Defines the main functionalities of the puppet program.
#[program]
pub mod puppet {
    use super::*;

    // Function to initialize account data.
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // Function to set `data` on a puppet account.
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<u64> {
        // Access the puppet account and update its `data`.
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(data)
    }
}

// Defines the account contexts for the `initialize` function.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // Initializes a new `Data` account with space allocation for storage.
    #[account(
      init,
      payer = user,
      space = 8 + 8
    )] // 8 bytes for the discriminator, 8 for the data.
    pub puppet: Account<'info, Data>,
    // Marks the transaction's signer as the payer for the account creation.
    #[account(mut)]
    pub user: Signer<'info>,
    // Includes the System Program in the transaction, needed for account creation.
    pub system_program: Program<'info, System>,
}

// Defines the account contexts for the `set_data` function.
#[derive(Accounts)]
pub struct SetData<'info> {
    // References a mutable puppet account to update its data.
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

// Defines the data structure stored within a `puppet` account.
#[account]
pub struct Data {
    pub data: u64, // Storage for a 64-bit unsigned integer.
}
