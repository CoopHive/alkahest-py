use alkahest_rs::clients::attestation;
use alloy::primitives::{Address, FixedBytes};
use pyo3::{pyclass, pymethods};
use tokio::runtime::Runtime;

use crate::types::{ArbiterData, AttestationRequest};

#[pyclass]
#[derive(Clone)]
pub struct AttestationClient {
    inner: attestation::AttestationClient,
}

impl AttestationClient {
    pub fn new(inner: attestation::AttestationClient) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl AttestationClient {
    pub async fn register_schema(
        &self,
        schema: String,
        resolver: String,
        revocable: bool,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let schema: FixedBytes<32> = schema.parse()?;
            let resolver: Address = resolver.parse()?;
            let receipt = self
                .inner
                .register_schema(schema.to_string(), resolver, revocable)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn attest(&self, attestation: AttestationRequest) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self.inner.attest(attestation.try_into()?).await?;
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

    pub async fn collect_payment_2(
        &self,
        buy_attestation: String,
        fulfillment: String,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .collect_payment_2(buy_attestation.parse()?, fulfillment.parse()?)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn create_escrow(
        &self,
        attestation: AttestationRequest,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .create_escrow(attestation.try_into()?, demand.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn create_escrow_2(
        &self,
        attestation: String,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .create_escrow_2(attestation.parse()?, demand.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn attest_and_create_escrow(
        &self,
        attestation: AttestationRequest,
        demand: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .attest_and_create_escrow(attestation.try_into()?, demand.try_into()?, expiration)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }
}
