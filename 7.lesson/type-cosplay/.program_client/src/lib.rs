// DO NOT EDIT - automatically generated file
pub mod type_cosplay_insecure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn add_user(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_insecure::instruction::AddUser {},
                type_cosplay_insecure::accounts::AddUser {
                    user: a_user,
                    vault: a_vault,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn add_user_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_insecure::instruction::AddUser {}.data(),
            accounts: type_cosplay_insecure::accounts::AddUser {
                user: a_user,
                vault: a_vault,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn withdraw(
        client: &Client,
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_insecure::instruction::Withdraw { amount: i_amount },
                type_cosplay_insecure::accounts::Withdraw {
                    meta: a_meta,
                    user: a_user,
                    authority: a_authority,
                    source_vault: a_source_vault,
                    destination_vault: a_destination_vault,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn withdraw_ix(
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_insecure::instruction::Withdraw { amount: i_amount }.data(),
            accounts: type_cosplay_insecure::accounts::Withdraw {
                meta: a_meta,
                user: a_user,
                authority: a_authority,
                source_vault: a_source_vault,
                destination_vault: a_destination_vault,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
pub mod type_cosplay_secure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 239u8, 58u8, 114u8, 151u8, 218u8, 23u8, 115u8, 211u8, 109u8, 171u8, 244u8,
        210u8, 156u8, 47u8, 250u8, 168u8, 157u8, 98u8, 83u8, 7u8, 236u8, 150u8, 238u8, 226u8,
        191u8, 69u8, 100u8, 128u8, 159u8, 2u8,
    ]);
    pub async fn add_user(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_secure::instruction::AddUser {},
                type_cosplay_secure::accounts::AddUser {
                    user: a_user,
                    vault: a_vault,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn add_user_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_secure::instruction::AddUser {}.data(),
            accounts: type_cosplay_secure::accounts::AddUser {
                user: a_user,
                vault: a_vault,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn withdraw(
        client: &Client,
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_secure::instruction::Withdraw { amount: i_amount },
                type_cosplay_secure::accounts::Withdraw {
                    meta: a_meta,
                    user: a_user,
                    authority: a_authority,
                    source_vault: a_source_vault,
                    destination_vault: a_destination_vault,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn withdraw_ix(
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_secure::instruction::Withdraw { amount: i_amount }.data(),
            accounts: type_cosplay_secure::accounts::Withdraw {
                meta: a_meta,
                user: a_user,
                authority: a_authority,
                source_vault: a_source_vault,
                destination_vault: a_destination_vault,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
