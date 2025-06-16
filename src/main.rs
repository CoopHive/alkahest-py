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
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    let token = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 100.try_into()?,
    };

    // Test approve for payment
    let _receipt = test
        .alice_client
        .erc20
        .approve(&token, ApprovalPurpose::Payment)
        .await?;

    // Verify approval for payment obligation
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

    println!("Payment allowance: {}", payment_allowance.to_string());

    // Test approve for escrow
    let _receipt = test
        .alice_client
        .erc20
        .approve(&token, ApprovalPurpose::Escrow)
        .await?;

    // Verify approval for escrow obligation
    let escrow_allowance = mock_erc20_a
        .allowance(
            test.alice.address(),
            test.addresses
                .erc20_addresses
                .ok_or(eyre::eyre!("no erc20-related addresses"))?
                .escrow_obligation,
        )
        .call()
        .await?;

    println!("Escrow allowance: {}", escrow_allowance.to_string());

    Ok(())
}
