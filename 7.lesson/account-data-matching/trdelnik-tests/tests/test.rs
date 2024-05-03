use anchor_spl::token::TokenAccount as AnchorTokenAccount;
use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;

    // token initialization
    let mint_keypair = keypair(6);
    let mint_authority = keypair(100);
    fixture.client
        .create_token_mint(&mint_keypair, mint_authority.pubkey(), None, 9)
        .await?;
    // create a token account belonging to an `authority`
    fixture.create_user_token_account().await?;
    // create a token account belonging to the `attacker`
    fixture.create_attacker_token_account().await?;

    fixture
}

#[trdelnik_test]
async fn test_insecure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    let acc = fixture
        .client
        .account_data::<AnchorTokenAccount>(fixture.attacker_token_account.pubkey())
        .await?;
    println!(
        "Token account {:?} succesfully created\n\tamount: {:?}\n\towner: {:?}",
        fixture.attacker_token_account.pubkey(),
        acc.amount,
        acc.owner
    );

    program_client::account_data_matching_insecure_instruction::log_message(
        &fixture.client,
        fixture.user_token_account.pubkey(),
        fixture.attacker.pubkey(),
        [fixture.attacker]
    ).await?.print();
}

#[trdelnik_test]
async fn test_secure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    let acc = fixture
        .client
        .account_data::<AnchorTokenAccount>(fixture.user_token_account.pubkey())
        .await?;
    println!(
        "Token account {:?} succesfully created\n\tamount: {:?}\n\towner: {:?}",
        fixture.user_token_account.pubkey(),
        acc.amount,
        acc.owner
    );

    program_client::account_data_matching_secure_instruction::log_message(
        &fixture.client,
        fixture.user_token_account.pubkey(),
        fixture.authority.pubkey(),
        [fixture.authority]
    ).await?.print();
}

struct Fixture {
    client: Client,
    authority: Keypair,
    user_token_account: Keypair,
    attacker_token_account: Keypair,
    attacker: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            authority: keypair(42),
            user_token_account: keypair(4),
            attacker_token_account: keypair(5),
            attacker: keypair(99),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&program_keypair(0), "account_data_matching_insecure")
            .await?;
        self.client
            .deploy_by_name(&program_keypair(8), "account_data_matching_secure")
            .await?;
    }

    #[throws]
    async fn create_user_token_account(&self) {
        // token initialization
        let mint_keypair = keypair(6);
        let mint_authority = keypair(100);
        // creation of a valid token account
        self.client
            .create_token_account(
                &self.user_token_account,
                &mint_keypair.pubkey(),
                &self.authority.pubkey(),
            )
            .await?;
        // mint tokens to newly created account
        self.client
            .mint_tokens(
                mint_keypair.pubkey(),
                &mint_authority,
                self.user_token_account.pubkey(),
                300,
            )
            .await?;
    }

    #[throws]
    async fn create_attacker_token_account(&self) {
        let mint_keypair = keypair(6);
        let mint_authority = keypair(100);
        // creation of a valid token account
        self.client
            .create_token_account(
                &self.attacker_token_account,
                &mint_keypair.pubkey(),
                &self.attacker.pubkey(),
            )
            .await?;
        // mint tokens to newly created account
        self.client
            .mint_tokens(
                mint_keypair.pubkey(),
                &mint_authority,
                self.attacker_token_account.pubkey(),
                10,
            )
            .await?;
    }
}
