use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TurnstileInstruction {
    /// Initialize a Turnstile state
    ///
    /// Passed accounts:
    ///
    /// (1) [signer, writable] State Account
    /// (2) [signer, writable] Initializer
    /// (3) [writable] Treasury wallet (PDA)
    /// (4) [] System Program
    Initialze { init_state: bool },
    /// Push
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Push,
    /// Coin
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    /// (2) [writable] Treasury wallet (PDA)
    /// (3) [signer, writable] Users wallet
    /// (4) [] System Program
    Coin,
}

pub fn initialize(
    turnstile_program: Pubkey,
    state: Pubkey,
    initializer: Pubkey,
    init_state: bool,
) -> Instruction {
    let (treasury, _) = Pubkey::find_program_address(&[initializer.as_ref()], &turnstile_program);
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(state, true),
            AccountMeta::new(initializer, true),
            AccountMeta::new(treasury, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TurnstileInstruction::Initialze { init_state }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn coin(turnstile_program: Pubkey, state: Pubkey, initializer: Pubkey) -> Instruction {
    let (treasury, _) = Pubkey::find_program_address(&[initializer.as_ref()], &turnstile_program);
    Instruction {
        program_id: turnstile_program,
        accounts: vec![
            AccountMeta::new(state, false),
            AccountMeta::new(treasury, false),
        ],
        data: TurnstileInstruction::Coin.try_to_vec().unwrap(), //[2]
    }
}

pub fn push(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Push.try_to_vec().unwrap(), //[1]
    }
}
