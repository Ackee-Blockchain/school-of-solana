use anchor_lang::prelude::*;

declare_id!("49wYaLtFcnpybqZwXUvZ4RwJMVAWFcZXaYvorNmrc1wZ");

#[program]
pub mod signature_verification_failed {
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
        space = 8 + 32 + 1,
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
