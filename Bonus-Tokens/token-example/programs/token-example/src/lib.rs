use anchor_lang::prelude::*;

declare_id!("ApjAG3oR2KFBdF5ZargvmDaS5UV3wZJMw8UGFtb6ij4m");

mod instructions;
use instructions::*;

#[program]
pub mod token_example {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, fee_bps: u16, max_fee: u64) -> Result<()> {
        _initialize(ctx, fee_bps, max_fee)
    }

    pub fn mint(ctx: Context<MintContext>, amount: u64) -> Result<()> {
        _mint(ctx, amount)
    }

    pub fn transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()> {
        _transfer(ctx, amount)
    }

    pub fn withdraw(ctx: Context<WithdrawContext>) -> Result<()> {
        _withdraw(ctx)
    }
}
