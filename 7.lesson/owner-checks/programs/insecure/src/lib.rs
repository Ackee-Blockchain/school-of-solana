use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Poo5jhFcGjMjYaz2cpmSNVq4ehvjKJhjU7aCZiS2LMP");

#[program]
pub mod owner_checks_insecure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key != & token.owner {
            return Err(ProgramError::InvalidAccountData.into());
        }
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    /// CHECK: don't do this a token account can contain arbitrary, invalid data.
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}
