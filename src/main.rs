use alkahest_rs::contracts::ERC20EscrowObligation;
use alkahest_rs::fixtures::MockERC20Permit;
use alkahest_rs::types::{ApprovalPurpose, ArbiterData, Erc721Data};
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

    // Set up tokens - alice gets ERC20, bob gets ERC721
    let mock_erc20 = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    mock_erc20
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create a purchase offer
    let bid = Erc20Data {
        address: test.mock_addresses.erc20_a,
        value: 50.try_into()?,
    };

    let ask = Erc721Data {
        address: test.mock_addresses.erc721_a,
        id: 1.try_into()?,
    };

    // alice approves tokens for escrow
    test.alice_client
        .erc20
        .approve(&bid, ApprovalPurpose::Escrow)
        .await?;

    // alice creates purchase offer
    let receipt = test
        .alice_client
        .erc20
        .buy_erc721_for_erc20(&bid, &ask, 0)
        .await?;

    // Verify escrow happened
    let alice_balance = mock_erc20.balanceOf(test.alice.address()).call().await?;

    let escrow_balance = mock_erc20
        .balanceOf(
            test.addresses
                .erc20_addresses
                .ok_or(eyre::eyre!("no erc20-related addresses"))?
                .escrow_obligation,
        )
        .call()
        .await?;

    // tokens in escrow
    assert_eq!(alice_balance, 50.try_into()?);
    assert_eq!(escrow_balance, 50.try_into()?);

    // escrow statement made
    let attested_event = AlkahestClient::get_attested_event(receipt)?;
    assert_ne!(attested_event.uid, FixedBytes::<32>::default());

    Ok(())
}
