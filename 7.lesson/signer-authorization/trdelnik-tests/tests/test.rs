use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *, solana_sdk::native_token::sol_to_lamports, anchor_lang::{system_program, Key}};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;
    fixture.create_state_insecure().await?;
    fixture.create_state_secure().await?;

    fixture
}

#[trdelnik_test]
async fn test_insecure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    
    program_client::signer_authorization_insecure_instruction::log_message(
        &fixture.client,
        fixture.insecure_state.pubkey(), 
        fixture.attacker.pubkey(),
        None,
    )
    .await?
    .print();
}

#[trdelnik_test]
async fn test_secure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;
    
    program_client::signer_authorization_secure_instruction::log_message(
        &fixture.client,
        fixture.secure_state.pubkey(),
        fixture.authority.pubkey(), 
        Some(fixture.authority),
    )
    .await?;
}

struct Fixture {
    client: Client,
    authority: Keypair,
    secure_state: Keypair,
    insecure_state: Keypair,
    attacker: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            authority: keypair(42),
            secure_state: keypair(41),
            insecure_state: keypair(40),
            attacker: keypair(99),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;

        self.client
            .deploy_by_name(&program_keypair(0), "signer_authorization_insecure")
            .await?;

        self.client
            .deploy_by_name(&program_keypair(8), "signer_authorization_secure")
            .await?;
    }

    #[throws]
    async fn create_state_insecure(&mut self) {
        // create state
        self.client
                .airdrop(self.authority.pubkey(), sol_to_lamports(1.0))
                .await?;
        program_client::signer_authorization_insecure_instruction::create_state(
            &self.client, 
            self.insecure_state.pubkey(), 
            self.authority.pubkey(), 
            system_program::ID, 
            [self.authority.clone(), self.insecure_state.clone()]
        )
        .await?;
    }

    #[throws]
    async fn create_state_secure(&mut self) {
        // create state
        self.client
                .airdrop(self.authority.pubkey(), sol_to_lamports(1.0))
                .await?;
        program_client::signer_authorization_secure_instruction::create_state(
            &self.client, 
            self.secure_state.pubkey(), 
            self.authority.pubkey(), 
            system_program::ID, 
            [self.authority.clone(), self.secure_state.clone()]
        )
        .await?;
    }
}
