use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar, native_token::sol_to_lamports,
};

use crate::{instruction::TurnstileInstruction, state::State};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TurnstileInstruction::try_from_slice(instruction_data)?;

    match instruction {
        TurnstileInstruction::Initialze { init_state } => {
            initialize(program_id, accounts, init_state)
        }
        TurnstileInstruction::Coin => coin(program_id, accounts),
        TurnstileInstruction::Push => push(program_id, accounts),
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    init_state: bool,
) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;
    let initialezer_account_info = next_account_info(account_into_iter)?;
    let treasury_account_info = next_account_info(account_into_iter)?;

    let (treasury, bump) =
        Pubkey::find_program_address(&[initialezer_account_info.key.as_ref()], program_id);

    assert_eq!(treasury, *treasury_account_info.key);

    let rent = Rent::get()?;
    invoke(
        &system_instruction::create_account(
            &initialezer_account_info.key,
            &state_account_info.key,
            rent.minimum_balance(State::SERIALZED_SIZE),
            State::SERIALZED_SIZE as u64,
            &program_id,
        ),
        &[initialezer_account_info.clone(), state_account_info.clone()],
    )?;

    let state = State { locked: init_state };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    invoke_signed(
        &system_instruction::create_account(
            &initialezer_account_info.key,
            &treasury,
            rent.minimum_balance(0),
            0,
            &system_program::id(),
        ),
        &[
            initialezer_account_info.clone(),
            treasury_account_info.clone(),
        ],
        &[&[initialezer_account_info.key.as_ref(), &[bump]]],
    )?;

    Ok(())
}

pub fn coin(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;
    let treasury_account_info = next_account_info(account_into_iter)?;
    let user_account_info = next_account_info(account_into_iter)?;

    assert_eq!(user_account_info.is_signer, true);
    
    invoke(
        &system_instruction::transfer(
            &user_account_info.key,
            &treasury_account_info.key,
            sol_to_lamports(1.0)
        ),
        &[user_account_info.clone(), treasury_account_info.clone()],
    )?;

    let state = State { locked: false };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn push(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    let state = State { locked: true };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}
