use alkahest_rs::clients::string_obligation;
use alloy::primitives::FixedBytes;
use pyo3::{pyclass, pymethods};

use crate::contract::PyDecodedAttestation;

#[pyclass]
#[derive(Clone)]
pub struct StringObligationClient {
    inner: string_obligation::StringObligationClient,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl StringObligationClient {
    pub fn new(
        inner: string_obligation::StringObligationClient,
        runtime: std::sync::Arc<tokio::runtime::Runtime>,
    ) -> Self {
        Self { inner, runtime }
    }
}

#[pymethods]
impl StringObligationClient {
    pub async fn get_statement(
        &self,
        uid: String,
    ) -> eyre::Result<PyDecodedAttestation<PyStringObligationStatementData>> {
        self.runtime.block_on(async {
            let uid: FixedBytes<32> = uid.parse()?;
            let statement = self.inner.get_statement(uid).await?;
            Ok(statement.into())
        })
    }

    pub async fn make_statement(
        &self,
        statement_data: PyStringObligationStatementData,
        ref_uid: Option<String>,
    ) -> eyre::Result<String> {
        self.runtime.block_on(async {
            use alkahest_rs::contracts::StringObligation;

            let data = StringObligation::StatementData {
                item: statement_data.item.clone(),
            };

            let ref_uid = if let Some(ref_uid_str) = ref_uid {
                Some(ref_uid_str.parse()?)
            } else {
                None
            };

            let receipt = self.inner.make_statement(data, ref_uid).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }

    pub async fn make_statement_json(
        &self,
        json_data: String,
        ref_uid: Option<String>,
    ) -> eyre::Result<String> {
        self.runtime.block_on(async {
            let json_value: serde_json::Value = serde_json::from_str(&json_data)?;

            let ref_uid = if let Some(ref_uid_str) = ref_uid {
                Some(ref_uid_str.parse()?)
            } else {
                None
            };

            let receipt = self.inner.make_statement_json(json_value, ref_uid).await?;
            Ok(receipt.transaction_hash.to_string())
        })
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyStringObligationStatementData {
    #[pyo3(get)]
    pub item: String,
}

#[pymethods]
impl PyStringObligationStatementData {
    #[new]
    pub fn new(item: String) -> Self {
        Self { item }
    }

    fn __repr__(&self) -> String {
        format!("PyStringObligationStatementData(item='{}')", self.item)
    }

    #[staticmethod]
    pub fn encode(obligation: &PyStringObligationStatementData) -> eyre::Result<Vec<u8>> {
        use alkahest_rs::contracts::StringObligation;
        use alloy::sol_types::SolValue;

        let statement_data = StringObligation::StatementData {
            item: obligation.item.clone(),
        };

        Ok(statement_data.abi_encode())
    }

    #[staticmethod]
    pub fn decode(statement_data: Vec<u8>) -> eyre::Result<PyStringObligationStatementData> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded =
            alkahest_rs::clients::string_obligation::StringObligationClient::decode(&bytes)?;
        Ok(decoded.into())
    }

    #[staticmethod]
    pub fn decode_json(statement_data: Vec<u8>) -> eyre::Result<String> {
        use alloy::primitives::Bytes;
        let bytes = Bytes::from(statement_data);
        let decoded: serde_json::Value =
            string_obligation::StringObligationClient::decode_json(&bytes)?;
        Ok(serde_json::to_string(&decoded)?)
    }

    #[staticmethod]
    pub fn encode_json(json_data: String) -> eyre::Result<Vec<u8>> {
        let json_value: serde_json::Value = serde_json::from_str(&json_data)?;
        let encoded = string_obligation::StringObligationClient::encode_json(json_value)?;
        Ok(encoded.to_vec())
    }

    pub fn encode_self(&self) -> eyre::Result<Vec<u8>> {
        PyStringObligationStatementData::encode(self)
    }
}

impl From<alkahest_rs::contracts::StringObligation::StatementData>
    for PyStringObligationStatementData
{
    fn from(data: alkahest_rs::contracts::StringObligation::StatementData) -> Self {
        Self { item: data.item }
    }
}
