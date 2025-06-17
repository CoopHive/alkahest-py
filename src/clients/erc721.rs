use alkahest_rs::clients::erc721;
use alloy::primitives::Address;
use pyo3::{pyclass, pymethods};

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
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl Erc721Client {
    pub fn new(
        inner: erc721::Erc721Client,
        runtime: std::sync::Arc<tokio::runtime::Runtime>,
    ) -> Self {
        Self { inner, runtime }
    }
}

#[pymethods]
impl Erc721Client {
    pub async fn approve(&self, token: Erc721Data, purpose: String) -> eyre::Result<String> {
        self.runtime.block_on(async {
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self.inner.approve(&token.try_into()?, purpose).await?;

            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn approve_all(
        &self,
        token_contract: String,
        purpose: String,
    ) -> eyre::Result<String> {
        self.runtime.block_on(async {
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
        self.runtime.block_on(async {
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .collect_payment(buy_attestation.parse()?, fulfillment.parse()?)
                .await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn collect_expired(&self, buy_attestation: String) -> eyre::Result<String> {
        self.runtime.block_on(async {
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_with_erc721(&price.try_into()?, &item.try_into()?, expiration)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_with_erc721(&price.try_into()?, payee.parse()?)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc721_for_erc721(&bid.try_into()?, &ask.try_into()?, expiration)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc721_for_erc721(buy_attestation.parse()?)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc20_with_erc721(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc721_for_erc20(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc721_for_erc20(buy_attestation.parse()?)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc1155_with_erc721(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc721_for_erc1155(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc721_for_erc1155(buy_attestation.parse()?)
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
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_with_erc721(&bid.try_into()?, ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc721_for_bundle(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc721_for_bundle(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyERC721EscrowObligationStatement {
    #[pyo3(get)]
    pub token: String,
    #[pyo3(get)]
    pub token_id: String,
    #[pyo3(get)]
    pub arbiter: String,
    #[pyo3(get)]
    pub demand: Vec<u8>,
}

#[pymethods]
impl PyERC721EscrowObligationStatement {
    #[new]
    pub fn new(token: String, token_id: String, arbiter: String, demand: Vec<u8>) -> Self {
        Self {
            token,
            token_id,
            arbiter,
            demand,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyERC721EscrowObligationStatement(token='{}', token_id='{}', arbiter='{}', demand={:?})",
            self.token, self.token_id, self.arbiter, self.demand
        )
    }

    #[staticmethod]
    pub fn decode(statement_data: Vec<u8>) -> eyre::Result<PyERC721EscrowObligationStatement> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded = alkahest_rs::clients::erc721::Erc721Client::decode_escrow_statement(&bytes)?;
        Ok(decoded.into())
    }

    #[staticmethod]
    pub fn encode(obligation: &PyERC721EscrowObligationStatement) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::ERC721EscrowObligation;
        use alloy::{
            primitives::{Address, Bytes, U256},
            sol_types::SolValue,
        };

        let token: Address = obligation.token.parse()?;
        let token_id: U256 = obligation.token_id.parse()?;
        let arbiter: Address = obligation.arbiter.parse()?;
        let demand = Bytes::from(obligation.demand.clone());

        let statement_data = ERC721EscrowObligation::StatementData {
            token,
            tokenId: token_id,
            arbiter,
            demand,
        };

        Ok(statement_data.abi_encode())
    }

    pub fn encode_self(&self) -> eyre::Result<Vec<u8>> {
        PyERC721EscrowObligationStatement::encode(self)
    }
}

impl From<alkahest_rs::contracts::ERC721EscrowObligation::StatementData>
    for PyERC721EscrowObligationStatement
{
    fn from(data: alkahest_rs::contracts::ERC721EscrowObligation::StatementData) -> Self {
        Self {
            token: format!("{:?}", data.token),
            token_id: data.tokenId.to_string(),
            arbiter: format!("{:?}", data.arbiter),
            demand: data.demand.to_vec(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyERC721PaymentObligationStatement {
    #[pyo3(get)]
    pub token: String,
    #[pyo3(get)]
    pub token_id: String,
    #[pyo3(get)]
    pub payee: String,
}

#[pymethods]
impl PyERC721PaymentObligationStatement {
    #[new]
    pub fn new(token: String, token_id: String, payee: String) -> Self {
        Self {
            token,
            token_id,
            payee,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyERC721PaymentObligationStatement(token='{}', token_id='{}', payee='{}')",
            self.token, self.token_id, self.payee
        )
    }

    #[staticmethod]
    pub fn decode(statement_data: Vec<u8>) -> eyre::Result<PyERC721PaymentObligationStatement> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded = alkahest_rs::clients::erc721::Erc721Client::decode_payment_statement(&bytes)?;
        Ok(decoded.into())
    }

    #[staticmethod]
    pub fn encode(obligation: &PyERC721PaymentObligationStatement) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::ERC721PaymentObligation;
        use alloy::{
            primitives::{Address, U256},
            sol_types::SolValue,
        };

        let token: Address = obligation.token.parse()?;
        let token_id: U256 = obligation.token_id.parse()?;
        let payee: Address = obligation.payee.parse()?;

        let statement_data = ERC721PaymentObligation::StatementData {
            token,
            tokenId: token_id,
            payee,
        };

        Ok(statement_data.abi_encode())
    }

    pub fn encode_self(&self) -> eyre::Result<Vec<u8>> {
        PyERC721PaymentObligationStatement::encode(self)
    }
}

impl From<alkahest_rs::contracts::ERC721PaymentObligation::StatementData>
    for PyERC721PaymentObligationStatement
{
    fn from(data: alkahest_rs::contracts::ERC721PaymentObligation::StatementData) -> Self {
        Self {
            token: format!("{:?}", data.token),
            token_id: data.tokenId.to_string(),
            payee: format!("{:?}", data.payee),
        }
    }
}
