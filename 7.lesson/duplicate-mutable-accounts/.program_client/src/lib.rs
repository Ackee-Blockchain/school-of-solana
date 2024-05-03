// DO NOT EDIT - automatically generated file
pub mod duplicate_mutable_accounts_insecure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn create_user_account(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                duplicate_mutable_accounts_insecure::instruction::CreateUserAccount {},
                duplicate_mutable_accounts_insecure::accounts::CreateUser {
                    user: a_user,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_user_account_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: duplicate_mutable_accounts_insecure::instruction::CreateUserAccount {}.data(),
            accounts: duplicate_mutable_accounts_insecure::accounts::CreateUser {
                user: a_user,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn close_user_a(
        client: &Client,
        a_user_a: anchor_lang::solana_program::pubkey::Pubkey,
        a_user_b: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                duplicate_mutable_accounts_insecure::instruction::CloseUserA {},
                duplicate_mutable_accounts_insecure::accounts::Close {
                    user_a: a_user_a,
                    user_b: a_user_b,
                },
                signers,
            )
            .await?)
    }
    pub fn close_user_a_ix(
        a_user_a: anchor_lang::solana_program::pubkey::Pubkey,
        a_user_b: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: duplicate_mutable_accounts_insecure::instruction::CloseUserA {}.data(),
            accounts: duplicate_mutable_accounts_insecure::accounts::Close {
                user_a: a_user_a,
                user_b: a_user_b,
            }
            .to_account_metas(None),
        }
    }
}
pub mod duplicate_mutable_accounts_secure_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 239u8, 58u8, 114u8, 151u8, 218u8, 23u8, 115u8, 211u8, 109u8, 171u8, 244u8,
        210u8, 156u8, 47u8, 250u8, 168u8, 157u8, 98u8, 83u8, 7u8, 236u8, 150u8, 238u8, 226u8,
        191u8, 69u8, 100u8, 128u8, 159u8, 2u8,
    ]);
    pub async fn create_user_account(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                duplicate_mutable_accounts_secure::instruction::CreateUserAccount {},
                duplicate_mutable_accounts_secure::accounts::CreateUser {
                    user: a_user,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn create_user_account_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: duplicate_mutable_accounts_secure::instruction::CreateUserAccount {}.data(),
            accounts: duplicate_mutable_accounts_secure::accounts::CreateUser {
                user: a_user,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn close_user_a(
        client: &Client,
        a_user_a: anchor_lang::solana_program::pubkey::Pubkey,
        a_user_b: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                duplicate_mutable_accounts_secure::instruction::CloseUserA {},
                duplicate_mutable_accounts_secure::accounts::Close {
                    user_a: a_user_a,
                    user_b: a_user_b,
                },
                signers,
            )
            .await?)
    }
    pub fn close_user_a_ix(
        a_user_a: anchor_lang::solana_program::pubkey::Pubkey,
        a_user_b: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: duplicate_mutable_accounts_secure::instruction::CloseUserA {}.data(),
            accounts: duplicate_mutable_accounts_secure::accounts::Close {
                user_a: a_user_a,
                user_b: a_user_b,
            }
            .to_account_metas(None),
        }
    }
}
