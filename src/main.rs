use alkahest_rs::contracts::ERC20EscrowObligation;
use alkahest_rs::fixtures::MockERC20Permit;
use alkahest_rs::types::{ApprovalPurpose, ArbiterData};
use alkahest_rs::utils::setup_test_environment;
use alkahest_rs::AlkahestClient;
use alkahest_rs::{clients::erc20::Erc20Client, types::Erc20Data};
use alloy::{
    primitives::{Bytes, U256},
    sol_types::SolValue,
};
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // give alice some erc20 tokens
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    let price = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 100.try_into()?,
    };

    // Create custom arbiter data
    let arbiter = test
        .addresses
        .erc20_addresses
        .clone()
        .ok_or(eyre::eyre!("no erc20-related addresses"))?
        .payment_obligation;
    let demand = Bytes::from(b"custom demand data");
    let item = ArbiterData { arbiter, demand };

    // approve tokens for escrow
    test.alice_client
        .erc20
        .approve(&price, ApprovalPurpose::Escrow)
        .await?;

    // alice creates escrow with custom demand
    let receipt = test
        .alice_client
        .erc20
        .buy_with_erc20(&price, &item, 0)
        .await?;

    // Verify escrow happened
    let alice_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    let escrow_balance = mock_erc20_a
        .balanceOf(
            test.addresses
                .erc20_addresses
                .ok_or(eyre::eyre!("no erc20-related addresses"))?
                .escrow_obligation,
        )
        .call()
        .await?;

    println!("Alice's balance: {}", alice_balance);
    println!("Escrow balance: {}", escrow_balance);

    // escrow statement made
    let attested_event = AlkahestClient::get_attested_event(receipt)?;
    println!("Attested event: {:?}", attested_event);
    Ok(())
}
