use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::token::{transfer, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("E49Vsg4wbsyYoUJPFrYRbnrMqDpixuqu2xpzD5JDMkF9");

#[program]
pub mod lesson_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let signer_key = ctx.accounts.signer.key();
        let bump = ctx.bumps.vault_data;

        let vault_data = &mut ctx.accounts.vault_data;

        vault_data.creator = signer_key;
        vault_data.bump = bump;

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_data", signer_key.as_ref(), &[bump]]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.new_mint.to_account_info(),
                to: ctx.accounts.new_vault.to_account_info(),
                authority: ctx.accounts.vault_data.to_account_info(),
            },
            signer_seeds,
        );

        mint_to(cpi_context, 1_000)?;

        Ok(())
    }
    pub fn grab(ctx: Context<Grab>) -> Result<()> {
        let vault_data = &ctx.accounts.vault_data;

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault_data",
            vault_data.creator.as_ref(),
            &[vault_data.bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.signer_vault.to_account_info(),
                authority: ctx.accounts.vault_data.to_account_info(),
            },
            signer_seeds,
        );

        transfer(cpi_context, 10)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8+VaultData::INIT_SPACE,
        seeds = [b"vault_data",signer.key().as_ref()],
        bump
    )]
    pub vault_data: Account<'info, VaultData>,
    #[account(
        init,
        payer = signer,
        seeds = [b"mint",signer.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = vault_data
    )]
    pub new_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = new_mint,
        associated_token::authority = vault_data,
    )]
    pub new_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Grab<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"vault_data",vault_data.creator.as_ref()],
        bump = vault_data.bump
    )]
    pub vault_data: Account<'info, VaultData>,
    #[account(
        seeds = [b"mint",vault_data.creator.as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = vault_data
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub signer_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_data,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultData {
    pub creator: Pubkey,
    pub bump: u8,
}
