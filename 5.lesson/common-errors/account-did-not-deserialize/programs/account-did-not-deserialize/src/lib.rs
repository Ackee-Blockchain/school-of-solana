use anchor_lang::prelude::*;

declare_id!("9GxG8C77oRp9Xej1dr8oKzFU6F7oYMeVbkFT91SrbxDa");

#[program]
pub mod account_did_not_deserialize {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = &mut ctx.accounts.data;

        data.authority = ctx.accounts.user.key();
        data.counter = 0;

        msg!("Data.counter = {}", data.counter);
        msg!("Data pubkey = {}", data.key().to_string());
        msg!("User pubkey = {}", data.authority.key().to_string());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(init,
        // Adjust space calculation to fix the test.
        // Add 8 bytes for the account discriminator.
        space = 32 + 1,
        payer = user,
    )]
    data: Account<'info, MyData>,

    system_program: Program<'info, System>,
}

#[account]
pub struct MyData {
    authority: Pubkey,
    counter: u8,
}
