use alkahest_rs::contracts::ERC20EscrowObligation;
use alkahest_rs::fixtures::MockERC20Permit;
use alkahest_rs::types::ApprovalPurpose;
use alkahest_rs::utils::setup_test_environment;
use alkahest_rs::{clients::erc20::Erc20Client, types::Erc20Data};
use alloy::{
    primitives::{Bytes, U256},
    sol_types::SolValue,
};
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let test = setup_test_environment().await?;

    // give alice some erc20 tokens
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    mock_erc20_a
        .transfer(test.alice.address(), 200.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    let token = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 100.try_into()?,
    };

    // First time should approve (no existing allowance)
    let receipt_opt = test
        .alice_client
        .erc20
        .approve_if_less(&token, ApprovalPurpose::Payment)
        .await?;

    assert!(
        receipt_opt.is_some(),
        "First approval should return receipt"
    );

    // Verify approval happened
    let payment_allowance = mock_erc20_a
        .allowance(
            test.alice.address(),
            test.addresses
                .erc20_addresses
                .clone()
                .ok_or(eyre::eyre!("no erc20-related addresses"))?
                .payment_obligation,
        )
        .call()
        .await?;

    println!("Payment allowance: {:?}", payment_allowance);

    // Second time should not approve (existing allowance is sufficient)
    let receipt_opt = test
        .alice_client
        .erc20
        .approve_if_less(&token, ApprovalPurpose::Payment)
        .await?;
    println!("Second approval receipt: {:?}", receipt_opt);

    // Now test with a larger amount
    let larger_token = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 150.try_into()?,
    };

    // This should approve again because we need a higher allowance
    let receipt_opt = test
        .alice_client
        .erc20
        .approve_if_less(&larger_token, ApprovalPurpose::Payment)
        .await?;

    // Verify new approval amount
    let new_payment_allowance = mock_erc20_a
        .allowance(
            test.alice.address(),
            test.addresses
                .erc20_addresses
                .ok_or(eyre::eyre!("no erc20-related addresses"))?
                .payment_obligation,
        )
        .call()
        .await?;

    println!("New payment allowance: {:?}", new_payment_allowance);

    Ok(())
}
