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

    // give alice some erc20 tokens
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
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

    // alice creates an escrow using permit signature (no pre-approval needed)
    let receipt = test
        .alice_client
        .erc20
        .permit_and_buy_erc20_for_erc20(&bid, &ask, 0)
        .await?;

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

    // all tokens in escrow
    assert_eq!(alice_balance, 0.try_into()?);
    assert_eq!(escrow_balance, 100.try_into()?);

    // escrow statement made
    let attested_event = AlkahestClient::get_attested_event(receipt)?;
    assert_ne!(attested_event.uid, FixedBytes::<32>::default());

    Ok(())
}
