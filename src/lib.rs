use std::str::FromStr;

use alkahest_rs::{
    clients::{
        attestation::AttestationAddresses, erc1155::Erc1155Addresses, erc20::Erc20Addresses,
        erc721::Erc721Addresses, oracle::OracleAddresses,
        string_obligation::StringObligationAddresses, token_bundle::TokenBundleAddresses,
    },
    contracts::IEAS::Attested,
    extensions::{
        AlkahestExtension, AttestationModule, Erc1155Module, Erc20Module, Erc721Module,
        HasAttestation, HasErc1155, HasErc20, HasErc721, HasOracle, HasStringObligation,
        HasTokenBundle, NoExtension, OracleModule, StringObligationModule, TokenBundleModule,
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
    types::{PyAnyMethods, PyModule, PyModuleMethods},
    Bound, FromPyObject, PyAny, PyResult, Python,
};
use tokio::runtime::Runtime;
use types::{DefaultExtensionConfig, EscowClaimedLog};

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
    inner: std::sync::Arc<dyn std::any::Any + Send + Sync>,
    // Store connection info to create new extension clients
    private_key: Option<String>,
    rpc_url: Option<String>,
    erc20: Option<Erc20Client>,
    erc721: Option<Erc721Client>,
    erc1155: Option<Erc1155Client>,
    token_bundle: Option<TokenBundleClient>,
    attestation: Option<AttestationClient>,
    string_obligation: Option<StringObligationClient>,
    oracle: Option<OracleClient>,
}

impl PyAlkahestClient {
    pub fn from_client(client: AlkahestClient) -> Self {
        Self {
            inner: std::sync::Arc::new(client.clone()),
            private_key: None, // Not available when creating from existing client
            rpc_url: None,     // Not available when creating from existing client
            erc20: Some(Erc20Client::new(client.extensions.erc20().clone())),
            erc721: Some(Erc721Client::new(client.extensions.erc721().clone())),
            erc1155: Some(Erc1155Client::new(client.extensions.erc1155().clone())),
            token_bundle: Some(TokenBundleClient::new(
                client.extensions.token_bundle().clone(),
            )),
            attestation: Some(AttestationClient::new(
                client.extensions.attestation().clone(),
            )),
            string_obligation: Some(StringObligationClient::new(
                client.extensions.string_obligation().clone(),
            )),
            oracle: Some(OracleClient::new(client.extensions.oracle().clone())),
        }
    }

    /// Create a PyAlkahestClient from a client with a single extension
    pub fn from_client_with_single_extension<T>(
        client: alkahest_rs::AlkahestClient<T>,
        extension_type: &str,
    ) -> Self
    where
        T: AlkahestExtension + Clone + Send + Sync + 'static,
    {
        // For now, we'll leave all extensions as None since extracting the specific
        // extension from the generic client type is complex. In a full implementation,
        // we would need to match on the extension type and extract the appropriate
        // extension to create the wrapper client.

        // The client still has the extension functionality in the inner client,
        // but the Python wrapper doesn't expose it through the .erc20, .erc721, etc. properties
        Self {
            inner: std::sync::Arc::new(client),
            private_key: None, // Connection info not available when creating from existing client
            rpc_url: None,     // Connection info not available when creating from existing client
            erc20: None,       // TODO: Extract if extension_type == "erc20"
            erc721: None,      // TODO: Extract if extension_type == "erc721"
            erc1155: None,     // TODO: Extract if extension_type == "erc1155"
            token_bundle: None, // TODO: Extract if extension_type == "token_bundle"
            attestation: None, // TODO: Extract if extension_type == "attestation"
            string_obligation: None, // TODO: Extract if extension_type == "string_obligation"
            oracle: None,      // TODO: Extract if extension_type == "oracle"
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
        address_config: Option<DefaultExtensionConfig>,
    ) -> PyResult<Self> {
        let address_config = address_config.map(|x| x.try_into()).transpose()?;

        // Convert private_key String to LocalSigner
        let signer = PrivateKeySigner::from_str(&private_key)
            .map_err(|e| eyre::eyre!("Failed to parse private key: {}", e))?;

        // Create a shared runtime
        let runtime = std::sync::Arc::new(Runtime::new()?);

        // Since new is async, we must block_on it
        let client: alkahest_rs::AlkahestClient = runtime.clone().block_on(async {
            alkahest_rs::AlkahestClient::new(signer.clone(), rpc_url.clone(), address_config).await
        })?;

        let client = Self {
            inner: std::sync::Arc::new(client.clone()),
            private_key: Some(private_key.clone()),
            rpc_url: Some(rpc_url.clone()),
            erc20: Some(Erc20Client::new(client.extensions.erc20().clone())),
            erc721: Some(Erc721Client::new(client.extensions.erc721().clone())),
            erc1155: Some(Erc1155Client::new(client.extensions.erc1155().clone())),
            token_bundle: Some(TokenBundleClient::new(
                client.extensions.token_bundle().clone(),
            )),
            attestation: Some(AttestationClient::new(
                client.extensions.attestation().clone(),
            )),
            string_obligation: Some(StringObligationClient::new(
                client.extensions.string_obligation().clone(),
            )),
            oracle: Some(OracleClient::new(client.extensions.oracle().clone())),
        };

        Ok(client)
    }

    /// Create a PyAlkahestClient with no extensions
    #[staticmethod]
    #[pyo3(signature = (private_key, rpc_url))]
    pub fn with_no_extensions(private_key: String, rpc_url: String) -> PyResult<Self> {
        // Convert private_key String to LocalSigner
        let signer = PrivateKeySigner::from_str(&private_key)
            .map_err(|e| eyre::eyre!("Failed to parse private key: {}", e))?;

        // Create a shared runtime
        let runtime = std::sync::Arc::new(Runtime::new()?);

        // Create client with NoExtension
        let client = runtime.clone().block_on(async {
            alkahest_rs::AlkahestClient::<NoExtension>::new(signer.clone(), rpc_url.clone(), None)
                .await
        })?;

        let py_client = Self {
            inner: std::sync::Arc::new(client),
            private_key: Some(private_key.clone()),
            rpc_url: Some(rpc_url.clone()),
            erc20: None,
            erc721: None,
            erc1155: None,
            token_bundle: None,
            attestation: None,
            string_obligation: None,
            oracle: None,
        };

        Ok(py_client)
    }

    /// Add ERC20 extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_erc20<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::Erc20Addresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc721 = self.erc721.clone();
        let erc1155 = self.erc1155.clone();
        let token_bundle = self.token_bundle.clone();
        let attestation = self.attestation.clone();
        let string_obligation = self.string_obligation.clone();
        let oracle = self.oracle.clone();
        let existing_erc20 = self.erc20.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // If we already have ERC20 extension, just return a copy with the same extension
            if let Some(erc20_client) = existing_erc20 {
                let new_client = Self {
                    inner,
                    private_key,
                    rpc_url,
                    erc20: Some(erc20_client),
                    erc721,
                    erc1155,
                    token_bundle,
                    attestation,
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // Create ERC20 extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent ERC20 client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<Erc20Addresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let erc20_extension = Erc20Module::init_with_config(signer, url.clone(), addresses)
                    .await
                    .map_err(|e| {
                        pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to create ERC20 extension: {}",
                            e
                        ))
                    })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20: Some(Erc20Client::new(erc20_extension.client)),
                    erc721,
                    erc1155,
                    token_bundle,
                    attestation,
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add ERC20 extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add ERC721 extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_erc721<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::Erc721Addresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc1155 = self.erc1155.clone();
        let token_bundle = self.token_bundle.clone();
        let attestation = self.attestation.clone();
        let string_obligation = self.string_obligation.clone();
        let oracle = self.oracle.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create ERC721 extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent ERC721 client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<Erc721Addresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let erc721_extension =
                    Erc721Module::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create ERC721 extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721: Some(Erc721Client::new(erc721_extension.client)),
                    erc1155,
                    token_bundle,
                    attestation,
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add ERC721 extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add ERC1155 extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_erc1155<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::Erc1155Addresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc721 = self.erc721.clone();
        let token_bundle = self.token_bundle.clone();
        let attestation = self.attestation.clone();
        let string_obligation = self.string_obligation.clone();
        let oracle = self.oracle.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create ERC1155 extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent ERC1155 client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<Erc1155Addresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let erc1155_extension =
                    Erc1155Module::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create ERC1155 extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721,
                    erc1155: Some(Erc1155Client::new(erc1155_extension.client)),
                    token_bundle,
                    attestation,
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add ERC1155 extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add TokenBundle extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_token_bundle<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::TokenBundleAddresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc721 = self.erc721.clone();
        let erc1155 = self.erc1155.clone();
        let attestation = self.attestation.clone();
        let string_obligation = self.string_obligation.clone();
        let oracle = self.oracle.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create TokenBundle extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent TokenBundle client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<TokenBundleAddresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let token_bundle_extension =
                    TokenBundleModule::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create TokenBundle extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721,
                    erc1155,
                    token_bundle: Some(TokenBundleClient::new(token_bundle_extension.client)),
                    attestation,
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add TokenBundle extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add Attestation extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_attestation<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::AttestationAddresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc721 = self.erc721.clone();
        let erc1155 = self.erc1155.clone();
        let token_bundle = self.token_bundle.clone();
        let string_obligation = self.string_obligation.clone();
        let oracle = self.oracle.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create Attestation extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent Attestation client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<AttestationAddresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let attestation_extension =
                    AttestationModule::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create Attestation extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721,
                    erc1155,
                    token_bundle,
                    attestation: Some(AttestationClient::new(attestation_extension.client)),
                    string_obligation,
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add Attestation extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add StringObligation extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_string_obligation<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::StringObligationAddresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc721 = self.erc721.clone();
        let erc1155 = self.erc1155.clone();
        let token_bundle = self.token_bundle.clone();
        let attestation = self.attestation.clone();
        let oracle = self.oracle.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create StringObligation extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent StringObligation client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<StringObligationAddresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let string_obligation_extension =
                    StringObligationModule::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create StringObligation extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721,
                    erc1155,
                    token_bundle,
                    attestation,
                    string_obligation: Some(StringObligationClient::new(
                        string_obligation_extension.client,
                    )),
                    oracle,
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add StringObligation extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// Add Oracle extension to the client and return a new client instance with that extension
    #[pyo3(signature = (config=None))]
    pub fn with_oracle<'py>(
        &self,
        py: Python<'py>,
        config: Option<crate::types::OracleAddresses>,
    ) -> PyResult<pyo3::Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        let private_key = self.private_key.clone();
        let rpc_url = self.rpc_url.clone();
        let erc20 = self.erc20.clone();
        let erc721 = self.erc721.clone();
        let erc1155 = self.erc1155.clone();
        let token_bundle = self.token_bundle.clone();
        let attestation = self.attestation.clone();
        let string_obligation = self.string_obligation.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Create Oracle extension using stored connection info
            if let (Some(pk), Some(url)) = (private_key, rpc_url) {
                // Create independent Oracle client using init_with_config
                let signer = PrivateKeySigner::from_str(&pk).map_err(|e| {
                    pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to parse private key: {}",
                        e
                    ))
                })?;

                let addresses: Option<OracleAddresses> =
                    config.and_then(|addr| addr.try_into().ok());
                let oracle_extension =
                    OracleModule::init_with_config(signer, url.clone(), addresses)
                        .await
                        .map_err(|e| {
                            pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to create Oracle extension: {}",
                                e
                            ))
                        })?;

                let new_client = Self {
                    inner,
                    private_key: Some(pk),
                    rpc_url: Some(url),
                    erc20,
                    erc721,
                    erc1155,
                    token_bundle,
                    attestation,
                    string_obligation,
                    oracle: Some(OracleClient::new(oracle_extension.client)),
                };
                return Ok(new_client);
            }

            // If no connection info available, return error
            Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Cannot add Oracle extension: no connection information available. Use AlkahestClient.with_no_extensions() to create a client with stored connection info.",
            ))
        })
    }

    /// List available extensions
    pub fn list_extensions(&self) -> Vec<String> {
        vec![
            "erc20".to_string(),
            "erc721".to_string(),
            "erc1155".to_string(),
            "token_bundle".to_string(),
            "attestation".to_string(),
            "string_obligation".to_string(),
            "oracle".to_string(),
        ]
    }

    /// Check if a specific extension is available
    pub fn has_extension(&self, extension_type: String) -> bool {
        match extension_type.as_str() {
            "erc20" => self.erc20.is_some(),
            "erc721" => self.erc721.is_some(),
            "erc1155" => self.erc1155.is_some(),
            "token_bundle" => self.token_bundle.is_some(),
            "attestation" => self.attestation.is_some(),
            "string_obligation" => self.string_obligation.is_some(),
            "oracle" => self.oracle.is_some(),
            _ => false,
        }
    }

    #[getter]
    pub fn erc20(&self) -> PyResult<Erc20Client> {
        self.erc20.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "ERC20 extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn erc721(&self) -> PyResult<Erc721Client> {
        self.erc721.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "ERC721 extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn erc1155(&self) -> PyResult<Erc1155Client> {
        self.erc1155.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "ERC1155 extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn token_bundle(&self) -> PyResult<TokenBundleClient> {
        self.token_bundle.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "TokenBundle extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn attestation(&self) -> PyResult<AttestationClient> {
        self.attestation.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "Attestation extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn string_obligation(&self) -> PyResult<StringObligationClient> {
        self.string_obligation.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "StringObligation extension is not available in this client",
            )
        })
    }

    #[getter]
    pub fn oracle(&self) -> PyResult<OracleClient> {
        self.oracle.clone().ok_or_else(|| {
            pyo3::PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                "Oracle extension is not available in this client",
            )
        })
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

            // Try to downcast to the appropriate client type
            let res = if let Some(client) = inner.downcast_ref::<AlkahestClient>() {
                client
                    .wait_for_fulfillment(contract_address, buy_attestation, from_block)
                    .await
                    .map_err(|e| {
                        pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e))
                    })?
            } else if let Some(client) =
                inner.downcast_ref::<alkahest_rs::AlkahestClient<NoExtension>>()
            {
                client
                    .wait_for_fulfillment(contract_address, buy_attestation, from_block)
                    .await
                    .map_err(|e| {
                        pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e))
                    })?
            } else {
                return Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Unknown client type",
                ));
            };

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

    // Address Configuration Classes
    m.add_class::<crate::types::PyErc20Addresses>()?;
    m.add_class::<crate::types::PyErc721Addresses>()?;
    m.add_class::<crate::types::PyErc1155Addresses>()?;
    m.add_class::<crate::types::PyTokenBundleAddresses>()?;
    m.add_class::<crate::types::PyAttestationAddresses>()?;
    m.add_class::<crate::types::PyStringObligationAddresses>()?;
    m.add_class::<crate::types::PyArbitersAddresses>()?;

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
