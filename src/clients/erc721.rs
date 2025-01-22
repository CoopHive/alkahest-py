use alkahest_rs::clients::erc721;
use alloy::primitives::Address;
use pyo3::{pyclass, pymethods};
use tokio::runtime::Runtime;

use crate::{
    get_attested_event,
    types::{
        ArbiterData, AttestedLog, Erc1155Data, Erc20Data, Erc721Data, LogWithHash, TokenBundleData,
    },
};

#[pyclass]
#[derive(Clone)]
pub struct Erc721Client {
    inner: erc721::Erc721Client,
}

impl Erc721Client {
    pub fn new(inner: erc721::Erc721Client) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Erc721Client {
    pub async fn approve(&self, token: Erc721Data, purpose: String) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self.inner.approve(token.try_into()?, purpose).await?;

            Ok(receipt.transaction_hash.to_string())
        })
    }

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

    pub async fn buy_with_erc721(
        &self,
        price: Erc721Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_with_erc721(price.try_into()?, item.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_with_erc721(
        &self,
        price: Erc721Data,
        payee: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_with_erc721(price.try_into()?, payee.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc_721_for_erc_721(
        &self,
        bid: Erc721Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc_721_for_erc_721(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc_721_for_erc_721(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_erc_721_for_erc_721(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc20_with_erc721(
        &self,
        bid: Erc721Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc20_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc1155_with_erc721(
        &self,
        bid: Erc721Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc1155_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_bundle_with_erc721(
        &self,
        bid: Erc721Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_with_erc721(bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }
}
