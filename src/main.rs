use std::time::{SystemTime, UNIX_EPOCH};

use alkahest_rs::contracts::{ERC20EscrowObligation, ERC20PaymentObligation};
use alkahest_rs::fixtures::{MockERC1155, MockERC20Permit, MockERC721};
use alkahest_rs::types::{ApprovalPurpose, ArbiterData, Erc1155Data, Erc721Data, TokenBundleData};
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

#[tokio::test]
async fn test_buy_erc1155_for_erc20() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20
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

    let ask = Erc1155Data {
        address: test.mock_addresses.erc1155_a,
        id: 1.try_into()?,
        value: 10.try_into()?,
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
        .buy_erc1155_for_erc20(&bid, &ask, 0)
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

#[tokio::test]
async fn test_buy_bundle_for_erc20() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20
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

    // Create bundle data
    let bundle = TokenBundleData {
        erc20s: vec![Erc20Data {
            address: test.mock_addresses.erc20_b,
            value: 20.try_into()?,
        }],
        erc721s: vec![Erc721Data {
            address: test.mock_addresses.erc721_a,
            id: 1.try_into()?,
        }],
        erc1155s: vec![Erc1155Data {
            address: test.mock_addresses.erc1155_a,
            id: 1.try_into()?,
            value: 5.try_into()?,
        }],
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
        .buy_bundle_for_erc20(&bid, &bundle, 0)
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

#[tokio::test]
async fn test_permit_and_buy_erc721_for_erc20() -> eyre::Result<()> {
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

    // alice creates purchase offer with permit (no pre-approval needed)
    let receipt = test
        .alice_client
        .erc20
        .permit_and_buy_erc721_for_erc20(&bid, &ask, 0)
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

#[tokio::test]
async fn test_permit_and_buy_erc1155_for_erc20() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20
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

    let ask = Erc1155Data {
        address: test.mock_addresses.erc1155_a,
        id: 1.try_into()?,
        value: 10.try_into()?,
    };

    // alice creates purchase offer with permit (no pre-approval needed)
    let receipt = test
        .alice_client
        .erc20
        .permit_and_buy_erc1155_for_erc20(&bid, &ask, 0)
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

#[tokio::test]
async fn test_permit_and_buy_bundle_for_erc20() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20
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

    // Create bundle data
    let bundle = TokenBundleData {
        erc20s: vec![Erc20Data {
            address: test.mock_addresses.erc20_b,
            value: 20.try_into()?,
        }],
        erc721s: vec![Erc721Data {
            address: test.mock_addresses.erc721_a,
            id: 1.try_into()?,
        }],
        erc1155s: vec![Erc1155Data {
            address: test.mock_addresses.erc1155_a,
            id: 1.try_into()?,
            value: 5.try_into()?,
        }],
    };

    // alice creates purchase offer with permit (no pre-approval needed)
    let receipt = test
        .alice_client
        .erc20
        .permit_and_buy_bundle_for_erc20(&bid, &bundle, 0)
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

#[tokio::test]
async fn test_pay_erc20_for_erc721() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets ERC721
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc721_a = MockERC721::new(test.mock_addresses.erc721_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Mint an ERC721 token to Bob
    mock_erc721_a
        .mint(test.bob.address())
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let erc721_token_id: U256 = 1.try_into()?;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // First create a buy attestation with Bob escrowing ERC721
    // Bob approves his ERC721 for escrow
    test.bob_client
        .erc721
        .approve(
            &Erc721Data {
                address: test.mock_addresses.erc721_a,
                id: erc721_token_id,
            },
            ApprovalPurpose::Escrow,
        )
        .await?;

    // Bob creates ERC721 escrow requesting ERC20
    let buy_receipt = test
        .bob_client
        .erc721
        .buy_erc20_with_erc721(
            &Erc721Data {
                address: test.mock_addresses.erc721_a,
                id: erc721_token_id,
            },
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check ownership before the exchange
    let initial_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        initial_erc721_owner,
        test.addresses
            .erc721_addresses
            .ok_or(eyre::eyre!("no erc721-related addresses"))?
            .escrow_obligation,
        "ERC721 should be in escrow"
    );

    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    // Alice approves her ERC20 tokens for payment
    test.alice_client
        .erc20
        .approve(
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            ApprovalPurpose::Payment,
        )
        .await?;

    // Alice fulfills Bob's escrow
    let pay_receipt = test
        .alice_client
        .erc20
        .pay_erc20_for_erc721(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    let final_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        final_erc721_owner,
        test.alice.address(),
        "Alice should now own the ERC721 token"
    );

    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    // Alice spent erc20_amount tokens
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // Bob received erc20_amount tokens
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received the correct amount of ERC20 tokens"
    );

    Ok(())
}

#[tokio::test]
async fn test_permit_and_pay_erc20_for_erc721() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets ERC721
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc721_a = MockERC721::new(test.mock_addresses.erc721_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Mint an ERC721 token to Bob
    mock_erc721_a
        .mint(test.bob.address())
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let erc721_token_id: U256 = 1.try_into()?;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // First create a buy attestation with Bob escrowing ERC721
    // Bob approves his ERC721 for escrow

    test.bob_client
        .erc721
        .approve(
            &Erc721Data {
                address: test.mock_addresses.erc721_a,
                id: erc721_token_id,
            },
            ApprovalPurpose::Escrow,
        )
        .await?;

    // Bob creates ERC721 escrow requesting ERC20
    let buy_receipt = test
        .bob_client
        .erc721
        .buy_erc20_with_erc721(
            &Erc721Data {
                address: test.mock_addresses.erc721_a,
                id: erc721_token_id,
            },
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check ownership before the exchange
    let initial_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        initial_erc721_owner,
        test.addresses
            .erc721_addresses
            .ok_or(eyre::eyre!("no erc721-related addresses"))?
            .escrow_obligation,
        "ERC721 should be in escrow"
    );

    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    // Alice fulfills Bob's escrow using permit
    let pay_receipt = test
        .alice_client
        .erc20
        .permit_and_pay_erc20_for_erc721(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    let final_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        final_erc721_owner,
        test.alice.address(),
        "Alice should now own the ERC721 token"
    );

    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    // Alice spent erc20_amount tokens
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // Bob received erc20_amount tokens
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received the correct amount of ERC20 tokens"
    );

    Ok(())
}

#[tokio::test]
async fn test_pay_erc20_for_erc1155() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets ERC1155
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc1155_a = MockERC1155::new(test.mock_addresses.erc1155_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Mint ERC1155 tokens to Bob
    let token_id = 1.try_into()?;
    let token_amount = 50.try_into()?;
    mock_erc1155_a
        .mint(test.bob.address(), token_id, token_amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // First create a buy attestation with Bob escrowing ERC1155
    // Bob approves his ERC1155 for escrow
    test.bob_client
        .erc1155
        .approve_all(test.mock_addresses.erc1155_a, ApprovalPurpose::Escrow)
        .await?;

    // Bob creates ERC1155 escrow requesting ERC20
    let buy_receipt = test
        .bob_client
        .erc1155
        .buy_erc20_with_erc1155(
            &Erc1155Data {
                address: test.mock_addresses.erc1155_a,
                id: token_id,
                value: token_amount,
            },
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check balances before the exchange
    let initial_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), token_id)
        .call()
        .await?;
    assert_eq!(
        initial_alice_erc1155_balance,
        0.try_into()?,
        "Alice should start with 0 ERC1155 tokens"
    );

    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    // Alice approves her ERC20 tokens for payment
    test.alice_client
        .erc20
        .approve(
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            ApprovalPurpose::Payment,
        )
        .await?;

    // Alice fulfills Bob's escrow
    let pay_receipt = test
        .alice_client
        .erc20
        .pay_erc20_for_erc1155(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    let final_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), token_id)
        .call()
        .await?;
    assert_eq!(
        final_alice_erc1155_balance, token_amount,
        "Alice should have received the ERC1155 tokens"
    );

    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    // Alice spent erc20_amount tokens
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // Bob received erc20_amount tokens
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received the correct amount of ERC20 tokens"
    );

    Ok(())
}

#[tokio::test]
async fn test_permit_and_pay_erc20_for_erc1155() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets ERC1155
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc1155_a = MockERC1155::new(test.mock_addresses.erc1155_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Mint ERC1155 tokens to Bob
    let token_id = 1.try_into()?;
    let token_amount = 50.try_into()?;
    mock_erc1155_a
        .mint(test.bob.address(), token_id, token_amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // First create a buy attestation with Bob escrowing ERC1155
    // Bob approves his ERC1155 for escrow
    test.bob_client
        .erc1155
        .approve_all(test.mock_addresses.erc1155_a, ApprovalPurpose::Escrow)
        .await?;

    // Bob creates ERC1155 escrow requesting ERC20
    let buy_receipt = test
        .bob_client
        .erc1155
        .buy_erc20_with_erc1155(
            &Erc1155Data {
                address: test.mock_addresses.erc1155_a,
                id: token_id,
                value: token_amount,
            },
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check balances before the exchange
    let initial_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), token_id)
        .call()
        .await?;
    assert_eq!(
        initial_alice_erc1155_balance,
        0.try_into()?,
        "Alice should start with 0 ERC1155 tokens"
    );

    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;

    // Alice fulfills Bob's escrow using permit
    let pay_receipt = test
        .alice_client
        .erc20
        .permit_and_pay_erc20_for_erc1155(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    let final_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), token_id)
        .call()
        .await?;
    assert_eq!(
        final_alice_erc1155_balance, token_amount,
        "Alice should have received the ERC1155 tokens"
    );

    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;

    // Alice spent erc20_amount tokens
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // Bob received erc20_amount tokens
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received the correct amount of ERC20 tokens"
    );

    Ok(())
}

#[tokio::test]
async fn test_pay_erc20_for_bundle() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets bundle tokens
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc20_b = MockERC20Permit::new(test.mock_addresses.erc20_b, &test.god_provider);
    let mock_erc721_a = MockERC721::new(test.mock_addresses.erc721_a, &test.god_provider);
    let mock_erc1155_a = MockERC1155::new(test.mock_addresses.erc1155_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Give Bob bundle tokens
    mock_erc20_b
        .transfer(test.bob.address(), 50.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    mock_erc721_a
        .mint(test.bob.address())
        .send()
        .await?
        .get_receipt()
        .await?;

    let erc1155_token_id = 1.try_into()?;
    let erc1155_amount = 20.try_into()?;
    mock_erc1155_a
        .mint(test.bob.address(), erc1155_token_id, erc1155_amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let bob_erc20_amount: U256 = 25.try_into()?; // Half of Bob's tokens
    let erc721_token_id: U256 = 1.try_into()?;
    let erc1155_bundle_amount = 10.try_into()?; // Half of Bob's tokens
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // Create token bundle
    let bundle = TokenBundleData {
        erc20s: vec![Erc20Data {
            address: test.mock_addresses.erc20_b,
            value: bob_erc20_amount,
        }],
        erc721s: vec![Erc721Data {
            address: test.mock_addresses.erc721_a,
            id: erc721_token_id,
        }],
        erc1155s: vec![Erc1155Data {
            address: test.mock_addresses.erc1155_a,
            id: erc1155_token_id,
            value: erc1155_bundle_amount,
        }],
    };

    // Bob approves his tokens for the bundle escrow
    test.bob_client
        .token_bundle
        .approve(&bundle, ApprovalPurpose::Escrow)
        .await?;

    // Bob creates bundle escrow demanding ERC20 from Alice
    // First encode the payment statement data as the demand
    let payment_statement_data = ERC20PaymentObligation::StatementData {
        token: test.mock_addresses.erc20_a,
        amount: erc20_amount,
        payee: test.bob.address(),
    };

    // Create the bundle escrow with demand for ERC20 payment
    let buy_receipt = test
        .bob_client
        .token_bundle
        .buy_with_bundle(
            &bundle,
            &ArbiterData {
                arbiter: test
                    .addresses
                    .erc20_addresses
                    .ok_or(eyre::eyre!("no erc20-related addresses"))?
                    .payment_obligation,
                demand: payment_statement_data.abi_encode().into(),
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check balances before the exchange
    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let initial_alice_bob_erc20_balance =
        mock_erc20_b.balanceOf(test.alice.address()).call().await?;
    let initial_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), erc1155_token_id)
        .call()
        .await?;

    // Alice approves her ERC20 tokens for payment
    test.alice_client
        .erc20
        .approve(
            &Erc20Data {
                address: test.mock_addresses.erc20_a,
                value: erc20_amount,
            },
            ApprovalPurpose::Payment,
        )
        .await?;

    // Alice fulfills Bob's bundle escrow
    let pay_receipt = test
        .alice_client
        .erc20
        .pay_erc20_for_bundle(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    // 1. Alice should now own ERC721
    let final_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        final_erc721_owner,
        test.alice.address(),
        "Alice should now own the ERC721 token"
    );

    // 2. Alice should have received Bob's ERC20
    let final_alice_bob_erc20_balance = mock_erc20_b.balanceOf(test.alice.address()).call().await?;
    assert_eq!(
        final_alice_bob_erc20_balance - initial_alice_bob_erc20_balance,
        bob_erc20_amount,
        "Alice should have received Bob's ERC20 tokens"
    );

    // 3. Alice should have received Bob's ERC1155
    let final_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), erc1155_token_id)
        .call()
        .await?;
    assert_eq!(
        final_alice_erc1155_balance - initial_alice_erc1155_balance,
        erc1155_bundle_amount,
        "Alice should have received Bob's ERC1155 tokens"
    );

    // 4. Alice should have spent her ERC20
    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // 5. Bob should have received Alice's ERC20
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received Alice's ERC20 tokens"
    );

    Ok(())
}

#[tokio::test]
async fn test_permit_and_pay_erc20_for_bundle() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Set up tokens - alice gets ERC20, bob gets bundle tokens
    let mock_erc20_a = MockERC20Permit::new(test.mock_addresses.erc20_a, &test.god_provider);
    let mock_erc20_b = MockERC20Permit::new(test.mock_addresses.erc20_b, &test.god_provider);
    let mock_erc721_a = MockERC721::new(test.mock_addresses.erc721_a, &test.god_provider);
    let mock_erc1155_a = MockERC1155::new(test.mock_addresses.erc1155_a, &test.god_provider);

    // Give Alice ERC20 tokens
    mock_erc20_a
        .transfer(test.alice.address(), 100.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Give Bob bundle tokens
    mock_erc20_b
        .transfer(test.bob.address(), 50.try_into()?)
        .send()
        .await?
        .get_receipt()
        .await?;

    mock_erc721_a
        .mint(test.bob.address())
        .send()
        .await?
        .get_receipt()
        .await?;

    let erc1155_token_id = 1.try_into()?;
    let erc1155_amount = 20.try_into()?;
    mock_erc1155_a
        .mint(test.bob.address(), erc1155_token_id, erc1155_amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    // Create test data
    let erc20_amount: U256 = 50.try_into()?;
    let bob_erc20_amount: U256 = 25.try_into()?; // Half of Bob's tokens
    let erc721_token_id: U256 = 1.try_into()?;
    let erc1155_bundle_amount = 10.try_into()?; // Half of Bob's tokens
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600; // 1 hour

    // Create token bundle
    let bundle = TokenBundleData {
        erc20s: vec![Erc20Data {
            address: test.mock_addresses.erc20_b,
            value: bob_erc20_amount,
        }],
        erc721s: vec![Erc721Data {
            address: test.mock_addresses.erc721_a,
            id: erc721_token_id,
        }],
        erc1155s: vec![Erc1155Data {
            address: test.mock_addresses.erc1155_a,
            id: erc1155_token_id,
            value: erc1155_bundle_amount,
        }],
    };

    // Bob approves his tokens for the bundle escrow
    test.bob_client
        .token_bundle
        .approve(&bundle, ApprovalPurpose::Escrow)
        .await?;

    // Bob creates bundle escrow demanding ERC20 from Alice
    // First encode the payment statement data as the demand
    let payment_statement_data = ERC20PaymentObligation::StatementData {
        token: test.mock_addresses.erc20_a,
        amount: erc20_amount,
        payee: test.bob.address(),
    };

    // Create the bundle escrow with demand for ERC20 payment
    let buy_receipt = test
        .bob_client
        .token_bundle
        .buy_with_bundle(
            &bundle,
            &ArbiterData {
                arbiter: test
                    .addresses
                    .erc20_addresses
                    .ok_or(eyre::eyre!("no erc20-related addresses"))?
                    .payment_obligation,
                demand: payment_statement_data.abi_encode().into(),
            },
            expiration as u64,
        )
        .await?;

    let buy_attestation = AlkahestClient::get_attested_event(buy_receipt)?.uid;

    // Check balances before the exchange
    let initial_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    let initial_alice_bob_erc20_balance =
        mock_erc20_b.balanceOf(test.alice.address()).call().await?;
    let initial_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), erc1155_token_id)
        .call()
        .await?;

    // Alice fulfills Bob's bundle escrow using permit
    let pay_receipt = test
        .alice_client
        .erc20
        .permit_and_pay_erc20_for_bundle(buy_attestation)
        .await?;

    // Verify the payment attestation was created
    let pay_attestation = AlkahestClient::get_attested_event(pay_receipt)?;
    assert_ne!(pay_attestation.uid, FixedBytes::<32>::default());

    // Verify token transfers
    // 1. Alice should now own ERC721
    let final_erc721_owner = mock_erc721_a.ownerOf(erc721_token_id).call().await?;
    assert_eq!(
        final_erc721_owner,
        test.alice.address(),
        "Alice should now own the ERC721 token"
    );

    // 2. Alice should have received Bob's ERC20
    let final_alice_bob_erc20_balance = mock_erc20_b.balanceOf(test.alice.address()).call().await?;
    assert_eq!(
        final_alice_bob_erc20_balance - initial_alice_bob_erc20_balance,
        bob_erc20_amount,
        "Alice should have received Bob's ERC20 tokens"
    );

    // 3. Alice should have received Bob's ERC1155
    let final_alice_erc1155_balance = mock_erc1155_a
        .balanceOf(test.alice.address(), erc1155_token_id)
        .call()
        .await?;
    assert_eq!(
        final_alice_erc1155_balance - initial_alice_erc1155_balance,
        erc1155_bundle_amount,
        "Alice should have received Bob's ERC1155 tokens"
    );

    // 4. Alice should have spent her ERC20
    let final_alice_erc20_balance = mock_erc20_a.balanceOf(test.alice.address()).call().await?;
    assert_eq!(
        initial_alice_erc20_balance - final_alice_erc20_balance,
        erc20_amount,
        "Alice should have spent the correct amount of ERC20 tokens"
    );

    // 5. Bob should have received Alice's ERC20
    let bob_erc20_balance = mock_erc20_a.balanceOf(test.bob.address()).call().await?;
    assert_eq!(
        bob_erc20_balance, erc20_amount,
        "Bob should have received Alice's ERC20 tokens"
    );

    Ok(())
}
