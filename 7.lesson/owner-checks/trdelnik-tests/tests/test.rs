use anchor_lang::solana_program::{program_option::COption, program_pack::Pack};
use anchor_spl::token::TokenAccount as AnchorTokenAccount;
use fehler::throws;
use program_client;
use spl_token::state::{Account as TokenAccount, AccountState};
use trdelnik_client::{anyhow::Result, *};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();

    fixture.deploy().await?;
    fixture.create_mock_token_account().await?;
    fixture.create_valid_token_account().await?;

    fixture
}

#[trdelnik_test]
async fn test_insecure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    let acc = fixture
        .client
        .account_data::<AnchorTokenAccount>(fixture.mock_token_account.pubkey())
        .await?;
    println!(
        "Token account {:?} succesfully created\n\towner: {:?}",
        fixture.mock_token_account.pubkey(),
        acc.owner
    );

    // call an intstruction
    program_client::owner_checks_insecure_instruction::log_message(
        &fixture.client,
        fixture.mock_token_account.pubkey(),
        fixture.attacker.pubkey(),
        [fixture.attacker]
    ).await?.print();
}

#[trdelnik_test]
async fn test_secure(#[future] init_fixture: Result<Fixture>) {
    let fixture = init_fixture.await?;

    let acc = fixture
        .client
        .account_data::<AnchorTokenAccount>(fixture.valid_token_account.pubkey())
        .await?;
    println!(
        "Token account {:?} succesfully created\n\tamount: {:?}\n\towner: {:?}",
        fixture.valid_token_account.pubkey(),
        acc.amount,
        acc.owner
    );

    // call an intstruction
    program_client::owner_checks_secure_instruction::log_message(
        &fixture.client,
        fixture.valid_token_account.pubkey(),
        fixture.authority.pubkey(),
        [fixture.authority]
    ).await?.print();
}

struct Fixture {
    client: Client,
    mock_token_account: Keypair,
    valid_token_account: Keypair,
    authority: Keypair,
    attacker: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            valid_token_account: keypair(4),
            mock_token_account: keypair(3),
            authority: keypair(5),
            attacker: keypair(99),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&program_keypair(0), "owner_checks_insecure")
            .await?;
        self.client
            .deploy_by_name(&program_keypair(8), "owner_checks_secure")
            .await?;
    }

    #[throws]
    async fn create_mock_token_account(&self) {
        let token_account = 
            TokenAccount {
                mint: Pubkey::default(),
                owner: self.attacker.pubkey(),
                amount: u64::MAX,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            };

        let mut buf = [0; TokenAccount::LEN];
        token_account.pack_into_slice(&mut buf);
        
        self.client.create_account_with_data(
            &self.mock_token_account,
            buf.to_vec()
        ).await?;
    }

    #[throws]
    async fn create_valid_token_account(&self) {
        // token initialization
        let mint_keypair = keypair(99);
        let mint_authority = keypair(100);
        self.client.create_token_mint(
            &mint_keypair,
            mint_authority.pubkey(), 
            None, 
            9
        ).await?;
        // creation of a valid token account
        self.client.create_token_account(
            &self.valid_token_account, 
            &mint_keypair.pubkey(),
            &self.authority.pubkey(),
        ).await?;
        // mint tokens to newly created account
        self.client.mint_tokens(
            mint_keypair.pubkey(),
            &mint_authority, 
            self.valid_token_account.pubkey(), 
            100
        ).await?;
    }
}
