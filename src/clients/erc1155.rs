use alkahest_rs::clients::erc1155;
use alloy::primitives::Address;
use pyo3::{pyclass, pymethods};
use tokio::runtime::Runtime;

use crate::types::{ArbiterData, Erc1155Data, Erc20Data, Erc721Data, TokenBundleData};

#[pyclass]
#[derive(Clone)]
pub struct Erc1155Client {
    inner: erc1155::Erc1155Client,
}

impl Erc1155Client {
    pub fn new(inner: erc1155::Erc1155Client) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Erc1155Client {
    pub async fn approve_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let token_contract: Address = token_contract.parse()?;
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self.inner.approve_all(token_contract, purpose).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn revoke_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let token_contract: Address = token_contract.parse()?;
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self.inner.revoke_all(token_contract, purpose).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn buy_with_erc1155(
        &self,
        price: Erc1155Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_with_erc1155(price.try_into()?, item.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn pay_with_erc_1155(
        &self,
        price: Erc1155Data,
        payee: String,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let payee: Address = payee.parse()?;
            let receipt = self
                .inner
                .pay_with_erc1155(price.try_into()?, payee)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn buy_erc1155_for_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc1155_for_erc1155(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn pay_erc1155_for_erc1155(&self, buy_attestation: String) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_erc1155_for_erc1155(buy_attestation.parse()?)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn buy_erc20_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc20_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn buy_erc721_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc721_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn buy_bundle_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_with_erc1155(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }
}
