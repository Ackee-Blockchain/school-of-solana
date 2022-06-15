use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub counter: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    /// Increment a counter
    ///
    /// Passed accounts:
    ///
    /// [writable] state account
    /// [signer, writable] fee payer
    Increment,
    /// Decrement a counter
    ///
    /// Passed accounts:
    ///
    /// [writable] state account
    /// [signer, writable] fee payer
    Decrement,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data)?;

    match instruction {
        CounterInstruction::Increment => process_increment(program_id, accounts),
        CounterInstruction::Decrement => process_decrement(program_id, accounts),
    }
}

pub fn process_increment(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    let mut state = Counter::try_from_slice(*state_account_info.data.borrow())?;
    state.counter += 1;
    state.serialize(&mut *state_account_info.data.borrow_mut())?;
    
    Ok(())
}

pub fn process_decrement(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    let mut state = Counter::try_from_slice(*state_account_info.data.borrow())?;
    state.counter -= 1;
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}
