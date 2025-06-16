use alkahest_rs::contracts::ERC20EscrowObligation;
use alkahest_rs::fixtures::MockERC20Permit;
use alkahest_rs::types::{ApprovalPurpose, ArbiterData};
use alkahest_rs::utils::setup_test_environment;
use alkahest_rs::AlkahestClient;
use alkahest_rs::{clients::erc20::Erc20Client, types::Erc20Data};
use alloy::primitives::FixedBytes;
use alloy::{
    primitives::{Bytes, U256},
    sol_types::SolValue,
};
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // give alice some erc20 tokens for bidding
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // give bob some erc20 tokens for fulfillment
    let mock_erc20_b = MockERC20Permit::new(test.mock_addresses.erc20_b, &test.god_provider);
    mock_erc20_b
        .transfer(test.bob.address(), 200.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // begin test
    let bid = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 100.try_into()?,
    };
    let ask = Erc20Data {
        address: test.mock_addresses.erc20_b,
        value: 200.try_into()?,
    };

    // alice approves tokens for escrow and creates buy attestation
    test.alice_client
        .erc20
        .approve(&bid, ApprovalPurpose::Escrow)
        .await?;

    let buy_receipt = test
        .alice_client
        .erc20
        .buy_erc20_for_erc20(&bid, &ask, 0)
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // bob fulfills the buy attestation with permit
    let _sell_receipt = test
        .bob_client
        .erc20
        .permit_and_pay_erc20_for_erc20(buy_attestation)
        .await?;

    // verify token transfers
    let alice_token_b_balance = mock_erc20_b.balanceOf(test.alice.address()).call().await?;

    let bob_token_a_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    // both sides received the tokens
    assert_eq!(
        alice_token_b_balance,
        200.try_into()?,
        "Alice should have received token B"
    );
    assert_eq!(
        bob_token_a_balance,
        100.try_into()?,
        "Bob should have received token A"
    );

    Ok(())
}
