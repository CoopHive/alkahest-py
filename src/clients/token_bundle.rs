use alkahest_rs::clients::token_bundle;
use pyo3::{pyclass, pymethods};

use crate::{
    get_attested_event,
    types::{ArbiterData, AttestedLog, LogWithHash, TokenBundleData},
};

#[pyclass]
#[derive(Clone)]
pub struct TokenBundleClient {
    inner: token_bundle::TokenBundleClient,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl TokenBundleClient {
    pub fn new(
        inner: token_bundle::TokenBundleClient,
        runtime: std::sync::Arc<tokio::runtime::Runtime>,
    ) -> Self {
        Self { inner, runtime }
    }
}

#[pymethods]
impl TokenBundleClient {
    pub fn approve(&self, token: TokenBundleData, purpose: String) -> eyre::Result<String> {
        self.runtime.block_on(async {
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipts = self.inner.approve(&token.try_into()?, purpose).await?;

            // Return the transaction hash of the last receipt, or empty string if no receipts
            match receipts.last() {
                Some(receipt) => Ok(receipt.transaction_hash.to_string()),
                None => Ok("".to_string()),
            }
        })
    }

    pub fn collect_payment(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        self.runtime.block_on(async {
            let receipt = self.inner.collect_expired(buy_attestation.parse()?).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub fn buy_with_bundle(
        &self,
        price: TokenBundleData,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_with_bundle(&price.try_into()?, &item.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub fn pay_with_bundle(
        &self,
        price: TokenBundleData,
        payee: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_with_bundle(&price.try_into()?, payee.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub fn buy_bundle_for_bundle(
        &self,
        bid: TokenBundleData,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_for_bundle(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub fn pay_bundle_for_bundle(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_bundle_for_bundle(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }
}
