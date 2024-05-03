use anchor_lang::prelude::*;

declare_id!("Po8QNbexAedzyagU3ZupR7pLeaqe4BmJmrNCaJ8xu6h");

#[program]
pub mod signer_authorization_secure {
    use super::*;

    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        ctx.accounts.state.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
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
    #[account(has_one = authority)]
    state: Account<'info, State>,
    /// CHECK:
    authority: Signer<'info>,
}

#[account]
pub struct State {
    authority: Pubkey,
}
