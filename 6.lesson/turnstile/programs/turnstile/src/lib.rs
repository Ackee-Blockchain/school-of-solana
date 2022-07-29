use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = true;
        state.payer = Pubkey::default();

        let treasury = &mut ctx.accounts.treasury;
        treasury.bump = *ctx.bumps.get("treasury").unwrap();
        Ok(())
    }

    pub fn coin(ctx: Context<Coin>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = false;
        state.payer = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn push(ctx: Context<Push>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.locked = true;
        state.payer = Pubkey::default();
        Ok(())
    }
}

// validate incoming accounts here
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 32 + 8 + 1
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 8,
        seeds = [b"treasury"], bump
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

// validate incoming accounts here
#[derive(Accounts)]
pub struct Coin<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Push<'info> {
    #[account(mut, has_one = payer)]
    pub state: Account<'info, State>,
    #[account(mut, seeds = [b"treasury"], bump = treasury.bump)]
    pub treasury: Account<'info, Treasury>,
    pub payer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub locked: bool,
    pub payer: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Treasury {
    pub bump: u8,
}
