use std::str::FromStr;

use alkahest_rs::{
    contracts::IEAS::Attested,
    extensions::{
        HasAttestation, HasErc1155, HasErc20, HasErc721, HasOracle, HasStringObligation,
        HasTokenBundle,
    },
    AlkahestClient,
};
use alloy::{
    primitives::{Address, FixedBytes, Log},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
};
use clients::{
    attestation::AttestationClient, erc1155::Erc1155Client, erc20::Erc20Client,
    erc721::Erc721Client, oracle::OracleClient, string_obligation::StringObligationClient,
    token_bundle::TokenBundleClient,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyAny, PyResult, Python,
};
use tokio::runtime::Runtime;
use types::{EscowClaimedLog, ExtensionAddresses};

use crate::{
    clients::{
        erc1155::{PyERC1155EscrowObligationData, PyERC1155PaymentObligationData},
        erc20::{PyERC20EscrowObligationData, PyERC20PaymentObligationData},
        erc721::{PyERC721EscrowObligationData, PyERC721PaymentObligationData},
        oracle::{
            PyArbitrateOptions, PyArbitrationResult, PyAttestationFilter, PyDecision,
            PyEscrowArbitrationResult, PyEscrowParams, PyFulfillmentParams, PyOracleAddresses,
            PyOracleAttestation, PySubscriptionResult, PyTrustedOracleArbiterDemandData,
        },
        string_obligation::PyStringObligationData,
    },
    contract::{
        PyAttestation, PyAttestationRequest, PyAttestationRequestData, PyAttested,
        PyRevocationRequest, PyRevocationRequestData, PyRevoked, PyTimestamped,
    },
    fixtures::{PyMockERC1155, PyMockERC20, PyMockERC721},
    types::PyErc20Data,
    utils::{EnvTestManager, PyWalletProvider},
};

pub mod clients;
pub mod contract;
pub mod error_handling;
pub mod fixtures;
pub mod types;
pub mod utils;

#[pyclass]
#[derive(Clone)]
pub struct PyAlkahestClient {
    inner: AlkahestClient,
    erc20: Erc20Client,
    erc721: Erc721Client,
    erc1155: Erc1155Client,
    token_bundle: TokenBundleClient,
    attestation: AttestationClient,
    string_obligation: StringObligationClient,
    oracle: OracleClient,
}

impl PyAlkahestClient {
    pub fn from_client(client: AlkahestClient) -> Self {
        Self {
            erc20: Erc20Client::new(client.extensions.erc20().clone()),
            erc721: Erc721Client::new(client.extensions.erc721().clone()),
            erc1155: Erc1155Client::new(client.extensions.erc1155().clone()),
            token_bundle: TokenBundleClient::new(client.extensions.token_bundle().clone()),
            attestation: AttestationClient::new(client.extensions.attestation().clone()),
            string_obligation: StringObligationClient::new(
                client.extensions.string_obligation().clone(),
            ),
            oracle: OracleClient::new(client.extensions.oracle().clone()),
            inner: client,
        }
    }
}

#[pymethods]
impl PyAlkahestClient {
    #[new]
    #[pyo3(signature = (private_key, rpc_url, address_config=None))]
    pub fn __new__(
        private_key: String,
        rpc_url: String,
        address_config: Option<ExtensionAddresses>,
    ) -> PyResult<Self> {
        let address_config = address_config.map(|x| x.try_into()).transpose()?;

        // Convert private_key String to LocalSigner
        let signer = PrivateKeySigner::from_str(&private_key)
            .map_err(|e| eyre::eyre!("Failed to parse private key: {}", e))?;

        // Create a shared runtime
        let runtime = std::sync::Arc::new(Runtime::new()?);

        // Since new is async, we must block_on it
        let client = runtime.clone().block_on(async {
            alkahest_rs::AlkahestClient::new(signer.clone(), rpc_url.clone(), address_config).await
        })?;

        let client = Self {
            inner: client.clone(),
            erc20: Erc20Client::new(client.extensions.erc20().clone()),
            erc721: Erc721Client::new(client.extensions.erc721().clone()),
            erc1155: Erc1155Client::new(client.extensions.erc1155().clone()),
            token_bundle: TokenBundleClient::new(client.extensions.token_bundle().clone()),
            attestation: AttestationClient::new(client.extensions.attestation().clone()),
            string_obligation: StringObligationClient::new(
                client.extensions.string_obligation().clone(),
            ),
            oracle: OracleClient::new(client.extensions.oracle().clone()),
        };

        Ok(client)
    }

    #[getter]
    pub fn erc20(&self) -> Erc20Client {
        self.erc20.clone()
    }

    #[getter]
    pub fn erc721(&self) -> Erc721Client {
        self.erc721.clone()
    }

    #[getter]
    pub fn erc1155(&self) -> Erc1155Client {
        self.erc1155.clone()
    }

    #[getter]
    pub fn token_bundle(&self) -> TokenBundleClient {
        self.token_bundle.clone()
    }

    #[getter]
    pub fn attestation(&self) -> AttestationClient {
        self.attestation.clone()
    }

    #[getter]
    pub fn string_obligation(&self) -> StringObligationClient {
        self.string_obligation.clone()
    }

    #[getter]
    pub fn oracle(&self) -> OracleClient {
        self.oracle.clone()
    }

    #[pyo3(signature = (contract_address, buy_attestation, from_block=None))]
    pub fn wait_for_fulfillment<'py>(
        &self,
        py: Python<'py>,
        contract_address: String,
        buy_attestation: String,
        from_block: Option<u64>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let contract_address: Address = contract_address.parse().map_err(|e| {
                pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Parse error: {}", e))
            })?;
            let buy_attestation: FixedBytes<32> = buy_attestation.parse().map_err(|e| {
                pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Parse error: {}", e))
            })?;
            let res = inner
                .wait_for_fulfillment(contract_address, buy_attestation, from_block)
                .await
                .map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e))
                })?;
            let result: EscowClaimedLog = res.data.into();
            Ok(result)
        })
    }
}

pub fn get_attested_event(receipt: TransactionReceipt) -> eyre::Result<Log<Attested>> {
    let attested_event = receipt
        .inner
        .logs()
        .iter()
        .filter(|log| log.topic0() == Some(&Attested::SIGNATURE_HASH))
        .collect::<Vec<_>>()
        .first()
        .map(|log| log.log_decode::<Attested>())
        .ok_or_else(|| eyre::eyre!("No Attested event found"))??;

    Ok(attested_event.inner)
}

#[pymodule]
fn alkahest_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAlkahestClient>()?;
    m.add_class::<StringObligationClient>()?;
    m.add_class::<OracleClient>()?;
    m.add_class::<PyOracleAddresses>()?;
    m.add_class::<PyAttestationFilter>()?;
    m.add_class::<PyOracleAttestation>()?;
    m.add_class::<PyDecision>()?;
    m.add_class::<PyFulfillmentParams>()?;
    m.add_class::<PyArbitrateOptions>()?;
    m.add_class::<PyArbitrationResult>()?;
    m.add_class::<PySubscriptionResult>()?;
    m.add_class::<PyTrustedOracleArbiterDemandData>()?;
    m.add_class::<PyEscrowParams>()?;
    m.add_class::<PyEscrowArbitrationResult>()?;
    m.add_class::<EnvTestManager>()?;
    m.add_class::<PyWalletProvider>()?;
    m.add_class::<PyMockERC20>()?;
    m.add_class::<PyMockERC721>()?;
    m.add_class::<PyMockERC1155>()?;
    m.add_class::<PyERC20EscrowObligationData>()?;
    m.add_class::<PyERC20PaymentObligationData>()?;
    m.add_class::<PyERC721EscrowObligationData>()?;
    m.add_class::<PyERC721PaymentObligationData>()?;
    m.add_class::<PyERC1155EscrowObligationData>()?;
    m.add_class::<PyERC1155PaymentObligationData>()?;
    m.add_class::<PyStringObligationData>()?;
    m.add_class::<PyErc20Data>()?;

    // IEAS (Ethereum Attestation Service) Types from contract.rs
    m.add_class::<PyAttestation>()?;
    m.add_class::<PyAttestationRequest>()?;
    m.add_class::<PyAttestationRequestData>()?;
    m.add_class::<PyAttested>()?;
    m.add_class::<PyRevocationRequest>()?;
    m.add_class::<PyRevocationRequestData>()?;
    m.add_class::<PyRevoked>()?;
    m.add_class::<PyTimestamped>()?;
    Ok(())
}
