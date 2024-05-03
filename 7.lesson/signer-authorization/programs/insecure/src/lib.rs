use anchor_lang::prelude::*;

declare_id!("Poo5jhFcGjMjYaz2cpmSNVq4ehvjKJhjU7aCZiS2LMP");

#[program]
pub mod signer_authorization_insecure {
    use super::*;

    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        ctx.accounts.state.authority = *ctx.accounts.authority.key;
        Ok(())
    }
    
    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("GM to authority {}", ctx.accounts.authority.key().to_string());
        msg!("HOWEVER... {} i'am the real authority", ctx.accounts.state.authority);
        Ok(())
    }   
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32, 
    )]
    state: Account<'info, State>,
    /// CHECK: 
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // CHECK:
    state: Account<'info, State>,
    /// CHECK:
    authority: AccountInfo<'info>,
}

#[account]
pub struct State {
    authority: Pubkey,
}
