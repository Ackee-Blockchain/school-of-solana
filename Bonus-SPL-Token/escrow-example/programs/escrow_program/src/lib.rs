use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("5wc4SpVzx9PbZWHzERcgL3YEuRg3kaGXS5cb8gCAmCAM");

#[program]
pub mod escrow_program {
    use super::*;

    pub fn initialize_exchange(
        ctx: Context<InitializeExchange>,
        a_to_b_amount: u64,
        b_to_a_amount: u64,
        side_b: Pubkey,
    ) -> Result<()> {
        _initialize_exchange(ctx, a_to_b_amount, b_to_a_amount, side_b)
    }
    pub fn finalize_exchange(ctx: Context<FinalizeExchange>) -> Result<()> {
        _finalize_exchange(ctx)
    }
    pub fn cancel_exchange(ctx: Context<CancelExchange>) -> Result<()> {
        _cancel_exchange(ctx)
    }
}
