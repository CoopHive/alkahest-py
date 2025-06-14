
use alkahest_rs::clients::erc20::Erc20Client;
use alkahest_rs::contracts::ERC20EscrowObligation;
use alkahest_rs::utils::setup_test_environment;
use alloy::{
    primitives::{Bytes, U256},
    sol_types::SolValue,
};
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // test setup
    let test = setup_test_environment().await?;

    // Create sample statement data
    let token_address = test.mock_addresses.erc20_a;
    let amount: U256 = 100.try_into()?;
    let arbiter = test
        .addresses
        .erc20_addresses
        .ok_or(eyre::eyre!("no erc20-related addresses"))?
        .payment_obligation;
    let demand = Bytes::from(vec![1, 2, 3, 4]); // sample demand data

    let escrow_data = ERC20EscrowObligation::StatementData {
        token: token_address,
        amount,
        arbiter,
        demand: demand.clone(),
    };

    // Encode the data
    let encoded = escrow_data.abi_encode();

    // Decode the data
    let decoded = Erc20Client::decode_escrow_statement(&encoded.into())?;
    println!("token address: {:?}", token_address);
    println!("amount: {:?}", amount);
    println!("arbiter: {:?}", arbiter);
    println!("demand: {:?}", demand);

    println!("decoded token: {:?}", decoded.token);
    println!("decoded amount: {:?}", decoded.amount);
    println!("decoded arbiter: {:?}", decoded.arbiter);
    println!("decoded demand: {:?}", decoded.demand);

    Ok(())
}
