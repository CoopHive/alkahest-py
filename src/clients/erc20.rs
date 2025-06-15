use alkahest_rs::clients::erc20;
use pyo3::{pyclass, pymethods, PyResult};
use tokio::runtime::Runtime;

use crate::{
    get_attested_event,
    types::{
        ArbiterData, AttestedLog, Erc1155Data, Erc20Data, Erc721Data, LogWithHash, TokenBundleData,
    },
};

#[pyclass]
#[derive(Clone)]
pub struct Erc20Client {
    inner: erc20::Erc20Client,
}

impl Erc20Client {
    pub fn new(inner: erc20::Erc20Client) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Erc20Client {
    pub async fn approve(&self, token: Erc20Data, purpose: String) -> eyre::Result<String> {
        Runtime::new()?.block_on(async {
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self.inner.approve(&token.try_into()?, purpose).await?;

            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn approve_if_less(
        &self,
        token: Erc20Data,
        purpose: String,
    ) -> eyre::Result<Option<String>> {
        Runtime::new()?.block_on(async {
            let purpose = match purpose.as_str() {
                "payment" => alkahest_rs::types::ApprovalPurpose::Payment,
                "escrow" => alkahest_rs::types::ApprovalPurpose::Escrow,
                _ => return Err(eyre::eyre!("Invalid purpose")),
            };
            let receipt = self
                .inner
                .approve_if_less(&token.try_into()?, purpose)
                .await?;

            Ok(receipt.map(|x| x.transaction_hash.to_string()))
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

    pub fn buy_with_erc20(
        &self,
        price: Erc20Data,
        item: ArbiterData,
        expiration: u64,
    ) -> PyResult<LogWithHash<AttestedLog>> {
        println!("buy_with_erc20 called",);
        let rt = Runtime::new().map_err(|e| eyre::eyre!(e.to_string()))?;
        let result = rt.block_on(async {
            let receipt = self
                .inner
                .buy_with_erc20(&price.try_into()?, &item.try_into()?, expiration)
                .await?;
            Ok::<_, eyre::Report>(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })?;

        Ok(result.into()) // You may need to convert `LogWithHash` to a PyO3-compatible struct
    }

    pub async fn permit_and_buy_with_erc20(
        &self,
        price: Erc20Data,
        item: ArbiterData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        let price: alkahest_rs::types::Erc20Data = price
            .try_into()
            .map_err(|e: eyre::Error| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let item: alkahest_rs::types::ArbiterData = item
            .try_into()
            .map_err(|e: eyre::Error| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Runtime::new()?.block_on(async {
            match self
                .inner
                .permit_and_buy_with_erc20(&price, &item, expiration)
                .await
            {
                Ok(receipt) => Ok(LogWithHash {
                    log: get_attested_event(receipt.clone())?.data.into(),
                    transaction_hash: receipt.transaction_hash.to_string(),
                }),
                Err(e) => {
                    eprintln!("inner.permit_and_buy_with_erc20 failed: {:?}", e);
                    Err(e)
                }
            }
        })
    }

    pub async fn pay_with_erc20(
        &self,
        price: Erc20Data,
        payee: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_with_erc20(&price.try_into()?, payee.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_pay_with_erc20(
        &self,
        price: Erc20Data,
        payee: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_pay_with_erc20(&price.try_into()?, payee.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc20_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc20_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_buy_erc20_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc20Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_buy_erc20_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc20_for_erc20(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_erc20_for_erc20(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_pay_erc20_for_erc20(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_pay_erc20_for_erc20(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc721_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc721_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_buy_erc721_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc721Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_buy_erc721_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc20_for_erc721(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_erc20_for_erc721(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_pay_erc20_for_erc721(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_pay_erc20_for_erc721(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_erc1155_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_erc1155_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_buy_erc1155_for_erc20(
        &self,
        bid: Erc20Data,
        ask: Erc1155Data,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_buy_erc1155_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;

            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn pay_erc20_for_erc1155(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .pay_erc20_for_erc1155(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_pay_erc20_for_erc1155(
        &self,
        buy_attestation: String,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_pay_erc20_for_erc1155(buy_attestation.parse()?)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn buy_bundle_for_erc20(
        &self,
        bid: Erc20Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .buy_bundle_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;

            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub async fn permit_and_buy_bundle_for_erc20(
        &self,
        bid: Erc20Data,
        ask: TokenBundleData,
        expiration: u64,
    ) -> eyre::Result<LogWithHash<AttestedLog>> {
        Runtime::new()?.block_on(async {
            let receipt = self
                .inner
                .permit_and_buy_bundle_for_erc20(&bid.try_into()?, &ask.try_into()?, expiration)
                .await?;
            Ok(LogWithHash {
                log: get_attested_event(receipt.clone())?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }

    pub fn decode_escrow_statement(
        &self,
        statement_data: Vec<u8>,
    ) -> eyre::Result<crate::types::PyERC20EscrowObligation> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded = alkahest_rs::clients::erc20::Erc20Client::decode_escrow_statement(&bytes)?;
        Ok(decoded.into())
    }

    pub fn encode_escrow_statement(
        &self,
        obligation: &crate::types::PyERC20EscrowObligation,
    ) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::ERC20EscrowObligation;
        use alloy::{
            primitives::{Address, Bytes, U256},
            sol_types::SolValue,
        };

        let token: Address = obligation.token.parse()?;
        let amount: U256 = U256::from(obligation.amount);
        let arbiter: Address = obligation.arbiter.parse()?;
        let demand = Bytes::from(obligation.demand.clone());

        let statement_data = ERC20EscrowObligation::StatementData {
            token,
            amount,
            arbiter,
            demand,
        };

        Ok(statement_data.abi_encode())
    }
}
