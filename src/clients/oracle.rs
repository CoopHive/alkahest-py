use alkahest_rs::{
    clients::oracle::OracleClient as InnerOracleClient, contracts::StringObligation,
};
use alloy::primitives::FixedBytes;
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};

use crate::clients::string_obligation::PyStringObligationStatementData;

#[pyclass]
#[derive(Clone)]
pub struct OracleClient {
    inner: InnerOracleClient,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl OracleClient {
    pub fn new(inner: InnerOracleClient, runtime: std::sync::Arc<tokio::runtime::Runtime>) -> Self {
        Self { inner, runtime }
    }
}

#[pymethods]
impl OracleClient {
    pub fn unsubscribe(&self, local_id: String) -> eyre::Result<()> {
        self.runtime.block_on(async {
            let local_id: FixedBytes<32> = local_id.parse()?;
            self.inner.unsubscribe(local_id).await
        })
    }

    /// Get the EAS contract address
    pub fn get_eas_address(&self) -> String {
        format!("{:?}", self.inner.addresses.eas)
    }

    /// Get the trusted oracle arbiter address
    pub fn get_trusted_oracle_arbiter_address(&self) -> String {
        format!("{:?}", self.inner.addresses.trusted_oracle_arbiter)
    }

    /// Arbitrate past attestations based on a decision function
    pub fn arbitrate_past(
        &self,
        fulfillment_params: PyFulfillmentParams,
        _py: Python,
        decision_func: PyObject,
        options: Option<PyArbitrateOptions>,
    ) -> PyResult<PyArbitrationResult> {
        let opts = options.unwrap_or_default();

        let result: eyre::Result<Vec<PyDecision>> = self.runtime.block_on(async {
            // Convert PyAttestationFilter to Rust AttestationFilter
            let rust_filter = fulfillment_params
                .filter
                .try_into()
                .map_err(|e| eyre::eyre!("Failed to convert filter: {}", e))?;

            // Convert PyStringObligationStatementData to Rust StatementData
            let statement_abi = StringObligation::StatementData {
                item: fulfillment_params.statement_abi.item.clone(),
            };

            // Create fulfillment parameters using the statement_abi from the params
            let fulfillment = alkahest_rs::clients::oracle::FulfillmentParams {
                statement_abi,
                filter: rust_filter,
            };

            let arbitrate_options = alkahest_rs::clients::oracle::ArbitrateOptions {
                require_oracle: opts.require_oracle,
                skip_arbitrated: opts.skip_arbitrated,
            }; // Create arbitration closure that calls back to Python
            let arbitrate_func =
                |statement_data: &StringObligation::StatementData| -> Option<bool> {
                    Python::with_gil(|py| {
                        // Convert StringObligation::StatementData to Python string
                        let py_statement = pyo3::types::PyString::new(py, &statement_data.item);

                        // Call the Python decision function with the decoded string
                        match decision_func.call1(py, (py_statement,)) {
                            Ok(result) => {
                                // Try to extract boolean result
                                match result.extract::<bool>(py) {
                                    Ok(decision) => Some(decision),
                                    Err(_) => {
                                        // If not a boolean, try to interpret as truthy/falsy
                                        match result.is_truthy(py) {
                                            Ok(truthy) => Some(truthy),
                                            Err(_) => None,
                                        }
                                    }
                                }
                            }
                            Err(_) => None,
                        }
                    })
                };

            // Call the actual Rust arbitrate_past method
            let decisions = self
                .inner
                .arbitrate_past(&fulfillment, arbitrate_func, &arbitrate_options)
                .await?;

            // Convert Rust decisions to Python decisions
            let py_decisions = decisions
                .into_iter()
                .map(|decision| {
                    let attestation = PyOracleAttestation::__new__(
                        format!(
                            "0x{}",
                            alloy::hex::encode(decision.attestation.uid.as_slice())
                        ),
                        format!(
                            "0x{}",
                            alloy::hex::encode(decision.attestation.schema.as_slice())
                        ),
                        format!(
                            "0x{}",
                            alloy::hex::encode(decision.attestation.refUID.as_slice())
                        ),
                        decision.attestation.time,
                        decision.attestation.expirationTime,
                        decision.attestation.revocationTime,
                        format!("0x{:x}", decision.attestation.recipient),
                        format!("0x{:x}", decision.attestation.attester),
                        decision.attestation.revocable,
                        format!("0x{}", alloy::hex::encode(&decision.attestation.data)),
                    );
                    PyDecision::__new__(
                        attestation,
                        decision.decision,
                        format!(
                            "0x{}",
                            alloy::hex::encode(decision.receipt.transaction_hash.as_slice())
                        ),
                        Some(decision.statement.item), // Use the string directly instead of hex encoding
                        None,                          // demand is None for this simple case
                    )
                })
                .collect();

            Ok(py_decisions)
        });

        match result {
            Ok(py_decisions) => {
                let total_count = py_decisions.len();
                let successful_count = py_decisions.iter().filter(|d| d.decision).count();

                Ok(PyArbitrationResult::__new__(
                    py_decisions,
                    successful_count,
                    total_count,
                ))
            }
            Err(e) => Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Arbitration failed: {}", e),
            )),
        }
    }

    /// Create demand data for trusted oracle arbiter
    pub fn create_trusted_oracle_demand(&self, oracle_address: String) -> PyResult<Vec<u8>> {
        use alkahest_rs::clients::arbiters::{ArbitersClient, TrustedOracleArbiter};
        use alloy::primitives::{Address, Bytes};
        
        let oracle_addr: Address = oracle_address.parse()
            .map_err(|e| pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid oracle address: {}", e)
            ))?;
            
        let demand_data = TrustedOracleArbiter::DemandData {
            oracle: oracle_addr,
            data: Bytes::new(),
        };
        
        let encoded = ArbitersClient::encode_trusted_oracle_arbiter_demand(&demand_data);
        Ok(encoded.to_vec())
    }
}

// ===== HELPER TYPES =====

#[pyclass]
#[derive(Clone)]
pub struct PyOracleAddresses {
    #[pyo3(get)]
    pub eas: String,
    #[pyo3(get)]
    pub trusted_oracle_arbiter: String,
}

#[pymethods]
impl PyOracleAddresses {
    #[new]
    pub fn __new__(eas: String, trusted_oracle_arbiter: String) -> Self {
        Self {
            eas,
            trusted_oracle_arbiter,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyOracleAddresses(eas={}, trusted_oracle_arbiter={})",
            self.eas, self.trusted_oracle_arbiter
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl TryFrom<PyOracleAddresses> for alkahest_rs::clients::oracle::OracleAddresses {
    type Error = eyre::Error;

    fn try_from(value: PyOracleAddresses) -> eyre::Result<Self> {
        Ok(Self {
            eas: value.eas.parse()?,
            trusted_oracle_arbiter: value.trusted_oracle_arbiter.parse()?,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyAttestationFilter {
    #[pyo3(get, set)]
    pub attester: Option<String>,
    #[pyo3(get, set)]
    pub recipient: Option<String>,
    #[pyo3(get, set)]
    pub schema_uid: Option<String>,
    #[pyo3(get, set)]
    pub uid: Option<String>,
    #[pyo3(get, set)]
    pub ref_uid: Option<String>,
    #[pyo3(get, set)]
    pub from_block: Option<u64>,
    #[pyo3(get, set)]
    pub to_block: Option<u64>,
}

#[pymethods]
impl PyAttestationFilter {
    #[new]
    #[pyo3(signature = (attester=None, recipient=None, schema_uid=None, uid=None, ref_uid=None, from_block=None, to_block=None))]
    pub fn __new__(
        attester: Option<String>,
        recipient: Option<String>,
        schema_uid: Option<String>,
        uid: Option<String>,
        ref_uid: Option<String>,
        from_block: Option<u64>,
        to_block: Option<u64>,
    ) -> Self {
        Self {
            attester,
            recipient,
            schema_uid,
            uid,
            ref_uid,
            from_block,
            to_block,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyAttestationFilter(attester={:?}, recipient={:?}, schema_uid={:?}, uid={:?}, ref_uid={:?}, from_block={:?}, to_block={:?})",
            self.attester, self.recipient, self.schema_uid, self.uid, self.ref_uid, self.from_block, self.to_block
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyArbitrateOptions {
    #[pyo3(get, set)]
    pub require_oracle: bool,
    #[pyo3(get, set)]
    pub skip_arbitrated: bool,
}

#[pymethods]
impl PyArbitrateOptions {
    #[new]
    #[pyo3(signature = (require_oracle=false, skip_arbitrated=false))]
    pub fn __new__(require_oracle: bool, skip_arbitrated: bool) -> Self {
        Self {
            require_oracle,
            skip_arbitrated,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyArbitrateOptions(require_oracle={}, skip_arbitrated={})",
            self.require_oracle, self.skip_arbitrated
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl Default for PyArbitrateOptions {
    fn default() -> Self {
        Self {
            require_oracle: false,
            skip_arbitrated: false,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyFulfillmentParams {
    #[pyo3(get, set)]
    pub statement_abi: PyStringObligationStatementData,
    #[pyo3(get, set)]
    pub filter: PyAttestationFilter,
}

#[pymethods]
impl PyFulfillmentParams {
    #[new]
    pub fn __new__(
        statement_abi: PyStringObligationStatementData,
        filter: PyAttestationFilter,
    ) -> Self {
        Self {
            statement_abi,
            filter,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyFulfillmentParams(statement_abi={:?}, filter={})",
            self.statement_abi,
            self.filter.__str__()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

// Remove the old PyFulfillmentParams definition

#[pyclass]
#[derive(Clone)]
pub struct PyFulfillmentParamsWithoutRefUid {
    #[pyo3(get, set)]
    pub schema_uid: String,
    #[pyo3(get, set)]
    pub filter: PyAttestationFilter, // Note: ref_uid will be ignored
}

#[pymethods]
impl PyFulfillmentParamsWithoutRefUid {
    #[new]
    pub fn __new__(schema_uid: String, filter: PyAttestationFilter) -> Self {
        Self { schema_uid, filter }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyFulfillmentParamsWithoutRefUid(schema_uid={}, filter={})",
            self.schema_uid,
            self.filter.__str__()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyEscrowParams {
    #[pyo3(get, set)]
    pub demand_schema_uid: String,
    #[pyo3(get, set)]
    pub filter: PyAttestationFilter,
}

#[pymethods]
impl PyEscrowParams {
    #[new]
    pub fn __new__(demand_schema_uid: String, filter: PyAttestationFilter) -> Self {
        Self {
            demand_schema_uid,
            filter,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyEscrowParams(demand_schema_uid={}, filter={})",
            self.demand_schema_uid,
            self.filter.__str__()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyOracleAttestation {
    #[pyo3(get)]
    pub uid: String,
    #[pyo3(get)]
    pub schema: String,
    #[pyo3(get)]
    pub ref_uid: String,
    #[pyo3(get)]
    pub time: u64,
    #[pyo3(get)]
    pub expiration_time: u64,
    #[pyo3(get)]
    pub revocation_time: u64,
    #[pyo3(get)]
    pub recipient: String,
    #[pyo3(get)]
    pub attester: String,
    #[pyo3(get)]
    pub revocable: bool,
    #[pyo3(get)]
    pub data: String,
}

#[pymethods]
impl PyOracleAttestation {
    #[new]
    pub fn __new__(
        uid: String,
        schema: String,
        ref_uid: String,
        time: u64,
        expiration_time: u64,
        revocation_time: u64,
        recipient: String,
        attester: String,
        revocable: bool,
        data: String,
    ) -> Self {
        Self {
            uid,
            schema,
            ref_uid,
            time,
            expiration_time,
            revocation_time,
            recipient,
            attester,
            revocable,
            data,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyOracleAttestation(uid={}, schema={}, attester={}, recipient={})",
            self.uid, self.schema, self.attester, self.recipient
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyDecision {
    #[pyo3(get)]
    pub attestation: PyOracleAttestation,
    #[pyo3(get)]
    pub decision: bool,
    #[pyo3(get)]
    pub transaction_hash: String,
    #[pyo3(get)]
    pub statement_data: Option<String>, // JSON or hex representation
    #[pyo3(get)]
    pub demand_data: Option<String>, // JSON or hex representation
}

#[pymethods]
impl PyDecision {
    #[new]
    #[pyo3(signature = (attestation, decision, transaction_hash, statement_data=None, demand_data=None))]
    pub fn __new__(
        attestation: PyOracleAttestation,
        decision: bool,
        transaction_hash: String,
        statement_data: Option<String>,
        demand_data: Option<String>,
    ) -> Self {
        Self {
            attestation,
            decision,
            transaction_hash,
            statement_data,
            demand_data,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyDecision(decision={}, tx_hash={})",
            self.decision, self.transaction_hash
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

// ===== RESULT TYPES =====

#[pyclass]
#[derive(Clone)]
pub struct PyArbitrationResult {
    #[pyo3(get)]
    pub decisions: Vec<PyDecision>,
    #[pyo3(get)]
    pub successful_count: usize,
    #[pyo3(get)]
    pub total_count: usize,
}

#[pymethods]
impl PyArbitrationResult {
    #[new]
    pub fn __new__(
        decisions: Vec<PyDecision>,
        successful_count: usize,
        total_count: usize,
    ) -> Self {
        Self {
            decisions,
            successful_count,
            total_count,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyArbitrationResult(successful={}/{}, decisions={})",
            self.successful_count,
            self.total_count,
            self.decisions.len()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PySubscriptionResult {
    #[pyo3(get)]
    pub subscription_id: String,
    #[pyo3(get)]
    pub initial_decisions: Vec<PyDecision>,
}

#[pymethods]
impl PySubscriptionResult {
    #[new]
    pub fn __new__(subscription_id: String, initial_decisions: Vec<PyDecision>) -> Self {
        Self {
            subscription_id,
            initial_decisions,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PySubscriptionResult(id={}, initial_decisions={})",
            self.subscription_id,
            self.initial_decisions.len()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyEscrowResult {
    #[pyo3(get)]
    pub escrow_attestations: Vec<PyOracleAttestation>,
    #[pyo3(get)]
    pub escrow_demands: Vec<String>, // JSON or hex representation
}

#[pymethods]
impl PyEscrowResult {
    #[new]
    pub fn __new__(
        escrow_attestations: Vec<PyOracleAttestation>,
        escrow_demands: Vec<String>,
    ) -> Self {
        Self {
            escrow_attestations,
            escrow_demands,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyEscrowResult(attestations={}, demands={})",
            self.escrow_attestations.len(),
            self.escrow_demands.len()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyEscrowArbitrationResult {
    #[pyo3(get)]
    pub decisions: Vec<PyDecision>,
    #[pyo3(get)]
    pub escrow_attestations: Vec<PyOracleAttestation>,
    #[pyo3(get)]
    pub escrow_demands: Vec<String>, // JSON or hex representation
}

#[pymethods]
impl PyEscrowArbitrationResult {
    #[new]
    pub fn __new__(
        decisions: Vec<PyDecision>,
        escrow_attestations: Vec<PyOracleAttestation>,
        escrow_demands: Vec<String>,
    ) -> Self {
        Self {
            decisions,
            escrow_attestations,
            escrow_demands,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyEscrowArbitrationResult(decisions={}, escrows={}, demands={})",
            self.decisions.len(),
            self.escrow_attestations.len(),
            self.escrow_demands.len()
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyEscrowSubscriptionResult {
    #[pyo3(get)]
    pub initial_decisions: Vec<PyDecision>,
    #[pyo3(get)]
    pub escrow_attestations: Vec<PyOracleAttestation>,
    #[pyo3(get)]
    pub escrow_subscription_id: String,
    #[pyo3(get)]
    pub fulfillment_subscription_id: String,
}

#[pymethods]
impl PyEscrowSubscriptionResult {
    #[new]
    pub fn __new__(
        initial_decisions: Vec<PyDecision>,
        escrow_attestations: Vec<PyOracleAttestation>,
        escrow_subscription_id: String,
        fulfillment_subscription_id: String,
    ) -> Self {
        Self {
            initial_decisions,
            escrow_attestations,
            escrow_subscription_id,
            fulfillment_subscription_id,
        }
    }

    pub fn __str__(&self) -> String {
        format!(
            "PyEscrowSubscriptionResult(decisions={}, escrows={}, escrow_id={}, fulfillment_id={})",
            self.initial_decisions.len(),
            self.escrow_attestations.len(),
            self.escrow_subscription_id,
            self.fulfillment_subscription_id
        )
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}

// ===== TYPE CONVERSIONS =====

impl TryFrom<PyAttestationFilter> for alkahest_rs::clients::oracle::AttestationFilter {
    type Error = eyre::Error;

    fn try_from(py_filter: PyAttestationFilter) -> eyre::Result<Self> {
        use alloy::{
            eips::BlockNumberOrTag,
            primitives::{Address, FixedBytes},
            rpc::types::{FilterBlockOption, ValueOrArray},
        };

        let block_option = if py_filter.from_block.is_some() || py_filter.to_block.is_some() {
            Some(FilterBlockOption::Range {
                from_block: py_filter.from_block.map(|b| BlockNumberOrTag::Number(b)),
                to_block: py_filter.to_block.map(|b| BlockNumberOrTag::Number(b)),
            })
        } else {
            None
        };

        let attester = if let Some(addr_str) = py_filter.attester {
            Some(ValueOrArray::Value(addr_str.parse::<Address>()?))
        } else {
            None
        };

        let recipient = if let Some(addr_str) = py_filter.recipient {
            Some(ValueOrArray::Value(addr_str.parse::<Address>()?))
        } else {
            None
        };

        let schema_uid = if let Some(uid_str) = py_filter.schema_uid {
            let bytes = alloy::hex::decode(uid_str.strip_prefix("0x").unwrap_or(&uid_str))?;
            if bytes.len() != 32 {
                return Err(eyre::eyre!("Schema UID must be 32 bytes"));
            }
            let mut fixed_bytes = [0u8; 32];
            fixed_bytes.copy_from_slice(&bytes);
            Some(ValueOrArray::Value(FixedBytes::from(fixed_bytes)))
        } else {
            None
        };

        let uid = if let Some(uid_str) = py_filter.uid {
            let bytes = alloy::hex::decode(uid_str.strip_prefix("0x").unwrap_or(&uid_str))?;
            if bytes.len() != 32 {
                return Err(eyre::eyre!("UID must be 32 bytes"));
            }
            let mut fixed_bytes = [0u8; 32];
            fixed_bytes.copy_from_slice(&bytes);
            Some(ValueOrArray::Value(FixedBytes::from(fixed_bytes)))
        } else {
            None
        };

        let ref_uid = if let Some(uid_str) = py_filter.ref_uid {
            let bytes = alloy::hex::decode(uid_str.strip_prefix("0x").unwrap_or(&uid_str))?;
            if bytes.len() != 32 {
                return Err(eyre::eyre!("Ref UID must be 32 bytes"));
            }
            let mut fixed_bytes = [0u8; 32];
            fixed_bytes.copy_from_slice(&bytes);
            Some(ValueOrArray::Value(FixedBytes::from(fixed_bytes)))
        } else {
            None
        };

        Ok(alkahest_rs::clients::oracle::AttestationFilter {
            block_option,
            attester,
            recipient,
            schema_uid,
            uid,
            ref_uid,
        })
    }
}

impl TryFrom<PyAttestationFilter> for alkahest_rs::clients::oracle::AttestationFilterWithoutRefUid {
    type Error = eyre::Error;

    fn try_from(py_filter: PyAttestationFilter) -> eyre::Result<Self> {
        let full_filter: alkahest_rs::clients::oracle::AttestationFilter = py_filter.try_into()?;

        Ok(
            alkahest_rs::clients::oracle::AttestationFilterWithoutRefUid {
                block_option: full_filter.block_option,
                attester: full_filter.attester,
                recipient: full_filter.recipient,
                schema_uid: full_filter.schema_uid,
                uid: full_filter.uid,
            },
        )
    }
}
