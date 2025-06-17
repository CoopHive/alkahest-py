use alkahest_rs::clients::erc1155;
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
pub struct Erc1155Client {
    inner: erc1155::Erc1155Client,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl Erc1155Client {
    pub fn new(
        inner: erc1155::Erc1155Client,
        runtime: std::sync::Arc<tokio::runtime::Runtime>,
    ) -> Self {
        Self { inner, runtime }
    }
}

#[pymethods]
impl Erc1155Client {
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

    pub async fn buy_with_erc1155(
        &self,
        price: Erc1155Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_with_erc1155(&price.try_into()?, &item.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_with_erc_1155(
        &self,
        price: Erc1155Data,
        payee: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let payee: Address = payee.parse()?;
            let receipt = self
                .inner
                .pay_with_erc1155(&price.try_into()?, payee)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc1155_for_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc1155_for_erc1155(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc1155_for_erc1155(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc1155_for_erc1155(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc20_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc20_with_erc1155(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc1155_for_erc20(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc1155_for_erc20(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc721_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_erc721_with_erc1155(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc1155_for_erc721(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc1155_for_erc721(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_bundle_with_erc1155(
        &self,
        bid: Erc1155Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_with_erc1155(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc1155_for_bundle(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        self.runtime.block_on(async {
            let receipt = self
                .inner
                .pay_erc1155_for_bundle(buy_attestation.parse()?)
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
pub struct PyERC1155EscrowObligationStatement {
    #[pyo3(get)]
    pub token: String,
    #[pyo3(get)]
    pub token_id: String,
    #[pyo3(get)]
    pub amount: String,
    #[pyo3(get)]
    pub arbiter: String,
    #[pyo3(get)]
    pub demand: Vec<u8>,
}

#[pymethods]
impl PyERC1155EscrowObligationStatement {
    #[new]
    pub fn new(
        token: String,
        token_id: String,
        amount: String,
        arbiter: String,
        demand: Vec<u8>,
    ) -> Self {
        Self {
            token,
            token_id,
            amount,
            arbiter,
            demand,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyERC1155EscrowObligationStatement(token='{}', token_id='{}', amount='{}', arbiter='{}', demand={:?})",
            self.token, self.token_id, self.amount, self.arbiter, self.demand
        )
    }

    #[staticmethod]
    pub fn decode(statement_data: Vec<u8>) -> eyre::Result<PyERC1155EscrowObligationStatement> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded =
            alkahest_rs::clients::erc1155::Erc1155Client::decode_escrow_statement(&bytes)?;
        Ok(decoded.into())
    }

    #[staticmethod]
    pub fn encode(obligation: &PyERC1155EscrowObligationStatement) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::ERC1155EscrowObligation;
        use alloy::{
            primitives::{Address, Bytes, U256},
            sol_types::SolValue,
        };

        let token: Address = obligation.token.parse()?;
        let token_id: U256 = obligation.token_id.parse()?;
        let amount: U256 = obligation.amount.parse()?;
        let arbiter: Address = obligation.arbiter.parse()?;
        let demand = Bytes::from(obligation.demand.clone());

        let statement_data = ERC1155EscrowObligation::StatementData {
            token,
            tokenId: token_id,
            amount,
            arbiter,
            demand,
        };

        Ok(statement_data.abi_encode())
    }

    pub fn encode_self(&self) -> eyre::Result<Vec<u8>> {
        PyERC1155EscrowObligationStatement::encode(self)
    }
}

impl From<alkahest_rs::contracts::ERC1155EscrowObligation::StatementData>
    for PyERC1155EscrowObligationStatement
{
    fn from(data: alkahest_rs::contracts::ERC1155EscrowObligation::StatementData) -> Self {
        Self {
            token: format!("{:?}", data.token),
            token_id: data.tokenId.to_string(),
            amount: data.amount.to_string(),
            arbiter: format!("{:?}", data.arbiter),
            demand: data.demand.to_vec(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyERC1155PaymentObligationStatement {
    #[pyo3(get)]
    pub token: String,
    #[pyo3(get)]
    pub token_id: String,
    #[pyo3(get)]
    pub amount: String,
    #[pyo3(get)]
    pub payee: String,
}

#[pymethods]
impl PyERC1155PaymentObligationStatement {
    #[new]
    pub fn new(token: String, token_id: String, amount: String, payee: String) -> Self {
        Self {
            token,
            token_id,
            amount,
            payee,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyERC1155PaymentObligationStatement(token='{}', token_id='{}', amount='{}', payee='{}')",
            self.token, self.token_id, self.amount, self.payee
        )
    }

    #[staticmethod]
    pub fn decode(statement_data: Vec<u8>) -> eyre::Result<PyERC1155PaymentObligationStatement> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded =
            alkahest_rs::clients::erc1155::Erc1155Client::decode_payment_statement(&bytes)?;
        Ok(decoded.into())
    }

    #[staticmethod]
    pub fn encode(obligation: &PyERC1155PaymentObligationStatement) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::ERC1155PaymentObligation;
        use alloy::{
            primitives::{Address, U256},
            sol_types::SolValue,
        };

        let token: Address = obligation.token.parse()?;
        let token_id: U256 = obligation.token_id.parse()?;
        let amount: U256 = obligation.amount.parse()?;
        let payee: Address = obligation.payee.parse()?;

        let statement_data = ERC1155PaymentObligation::StatementData {
            token,
            tokenId: token_id,
            amount,
            payee,
        };

        Ok(statement_data.abi_encode())
    }

    pub fn encode_self(&self) -> eyre::Result<Vec<u8>> {
        PyERC1155PaymentObligationStatement::encode(self)
    }
}

impl From<alkahest_rs::contracts::ERC1155PaymentObligation::StatementData>
    for PyERC1155PaymentObligationStatement
{
    fn from(data: alkahest_rs::contracts::ERC1155PaymentObligation::StatementData) -> Self {
        Self {
            token: format!("{:?}", data.token),
            token_id: data.tokenId.to_string(),
            amount: data.amount.to_string(),
            payee: format!("{:?}", data.payee),
        }
    }
}
