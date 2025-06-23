use std::str::FromStr;

use alkahest_rs::{contracts::IEAS::Attested, AlkahestClient};
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
    Bound, PyResult,
};
use tokio::runtime::Runtime;
use types::{AddressConfig, EscowClaimedLog};

use crate::{
    clients::{
        erc1155::{PyERC1155EscrowObligationStatement, PyERC1155PaymentObligationStatement},
        erc20::{PyERC20EscrowObligationStatement, PyERC20PaymentObligationStatement},
        erc721::{PyERC721EscrowObligationStatement, PyERC721PaymentObligationStatement},
        oracle::{
            PyArbitrateOptions, PyArbitrationResult, PyAttestationFilter, PyDecision,
            PyEscrowArbitrationResult, PyEscrowParams, PyFulfillmentParams, PyOracleAddresses,
            PyOracleAttestation, PySubscriptionResult, PyTrustedOracleArbiterDemandData,
        },
        string_obligation::PyStringObligationStatementData,
    },
    contract::{
        PyAttestation, PyAttestationRequest, PyAttestationRequestData, PyAttested,
        PyRevocationRequest, PyRevocationRequestData, PyRevoked, PyTimestamped,
    },
    fixtures::{PyMockERC1155, PyMockERC20, PyMockERC721},
    types::PyErc20Data,
    utils::{PyTestEnvManager, PyWalletProvider},
};

pub mod clients;
pub mod contract;
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
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
}

impl PyAlkahestClient {
    pub fn from_client(client: AlkahestClient) -> Self {
        let runtime = std::sync::Arc::new(Runtime::new().expect("Failed to create runtime"));
        Self {
            erc20: Erc20Client::new(client.erc20.clone(), runtime.clone()),
            erc721: Erc721Client::new(client.erc721.clone(), runtime.clone()),
            erc1155: Erc1155Client::new(client.erc1155.clone(), runtime.clone()),
            token_bundle: TokenBundleClient::new(client.token_bundle.clone(), runtime.clone()),
            attestation: AttestationClient::new(client.attestation.clone(), runtime.clone()),
            string_obligation: StringObligationClient::new(
                client.string_obligation.clone(),
                runtime.clone(),
            ),
            oracle: OracleClient::new(client.oracle.clone(), runtime.clone()),
            inner: client,
            runtime: runtime,
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
        address_config: Option<AddressConfig>,
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
            erc20: Erc20Client::new(client.erc20, runtime.clone()),
            erc721: Erc721Client::new(client.erc721, runtime.clone()),
            erc1155: Erc1155Client::new(client.erc1155, runtime.clone()),
            token_bundle: TokenBundleClient::new(client.token_bundle, runtime.clone()),
            attestation: AttestationClient::new(client.attestation, runtime.clone()),
            string_obligation: StringObligationClient::new(
                client.string_obligation,
                runtime.clone(),
            ),
            oracle: OracleClient::new(client.oracle, runtime.clone()),
            runtime: runtime,
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
    pub async fn wait_for_fulfillment(
        &self,
        contract_address: String,
        buy_attestation: String,
        from_block: Option<u64>,
    ) -> eyre::Result<EscowClaimedLog> {
        self.runtime.block_on(async {
            let contract_address: Address = contract_address.parse()?;
            let buy_attestation: FixedBytes<32> = buy_attestation.parse()?;
            let res = self
                .inner
                .wait_for_fulfillment(contract_address, buy_attestation, from_block)
                .await?;
            Ok(res.data.into())
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
    m.add_class::<PyTestEnvManager>()?;
    m.add_class::<PyWalletProvider>()?;
    m.add_class::<PyMockERC20>()?;
    m.add_class::<PyMockERC721>()?;
    m.add_class::<PyMockERC1155>()?;
    m.add_class::<PyERC20EscrowObligationStatement>()?;
    m.add_class::<PyERC20PaymentObligationStatement>()?;
    m.add_class::<PyERC721EscrowObligationStatement>()?;
    m.add_class::<PyERC721PaymentObligationStatement>()?;
    m.add_class::<PyERC1155EscrowObligationStatement>()?;
    m.add_class::<PyERC1155PaymentObligationStatement>()?;
    m.add_class::<PyStringObligationStatementData>()?;
    m.add_class::<PyErc20Data>()?;
    // Note: PyDecodedAttestation is now IntoPyObject, not a class, so it converts to dict automatically

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
