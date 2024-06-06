use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Client, Cluster, Program};
use std::rc::Rc;

#[tokio::main]
async fn main() {
    // Configure the client to use the local cluster.
    let payer = Rc::new(anchor_client::solana_sdk::signer::keypair::Keypair::new());
    let client = Client::new(Cluster::Localnet, payer.clone());

    // Load the program.
    let program_id = Pubkey::from_str("YOUR_PROGRAM_ID_HERE").unwrap();
    let program = client.program(program_id);

    // Create a new account to store the greeting
    let greeting_account = Keypair::new();
    let (greeting_account_pda, _bump_seed) = Pubkey::find_program_address(
        &[b"greeting_account".as_ref(), greeting_account.pubkey().as_ref()],
        &program_id,
    );

    // Send the transaction
    let tx = program
        .request()
        .instruction(system_program::create_account(
            &payer.pubkey(),
            &greeting_account.pubkey(),
            client
                .get_minimum_balance_for_rent_exemption(8 + 40)
                .await
                .unwrap(),
            8 + 40,
            &program_id,
        ))
        .args(gm_anchor::instruction::SayGm {
            name: "Brandon".to_string(),
        })
        .accounts(gm_anchor::accounts::SayGm {
            greeting_account: greeting_account_pda,
            user: payer.pubkey(),
            system_program: system_program::ID,
        })
        .signer(greeting_account.clone())
        .signer(payer.clone())
        .send()
        .await
        .unwrap();

    println!("Transaction sent: {}", tx);

    // Fetch the stored name
    let account: gm_anchor::GreetingAccount = program
        .account(greeting_account_pda)
        .await
        .unwrap();

    println!("Stored name: {}", account.name);
}
