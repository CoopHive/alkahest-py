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

    // approve tokens for payment
    test.alice_client
        .erc20
        .approve(&price, ApprovalPurpose::Payment)
        .await?;

    // alice makes direct payment to bob
    let receipt = test
        .alice_client
        .erc20
        .pay_with_erc20(&price, test.bob.address())
        .await?;

    // Verify payment happened
    let alice_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    let bob_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    println!(
        "Alice's balance after payment: {}",
        alice_balance.to_string()
    );
    println!("Bob's balance after payment: {}", bob_balance.to_string());

    // payment statement made
    let attested_event = AlkahestClient::get_attested_event(receipt)?;
    println!("Payment statement made: {:?}", attested_event);

    Ok(())
}
