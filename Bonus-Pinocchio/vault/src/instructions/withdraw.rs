use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_system::instructions::Transfer;

pub struct WithdrawalAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub bump: [u8; 1]
}

impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawalAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vault.lamports().eq(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        let (vault_key, bump) = find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID);
        if &vault_key != vault.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { owner, vault, bump: [bump] })
    }
}


pub struct Withdraw<'a> {
    pub accounts: WithdrawalAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Withdraw<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = WithdrawalAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl <'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        let seeds = [
            Seed::from(b"vault"),
            Seed::from(self.accounts.owner.key().as_ref()),
            Seed::from(&self.accounts.bump)
        ];

        let signers = [Signer::from(&seeds)];

        Transfer {
            from: self.accounts.vault,
            to: self.accounts.owner,
            lamports: self.accounts.vault.lamports()
        }
        .invoke_signed(&signers)
    }
}