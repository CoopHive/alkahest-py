use alkahest_rs::contracts::IEAS::Attested;
use alloy::{
    primitives::{Address, FixedBytes, Log},
    rpc::types::TransactionReceipt,
    sol_types::SolEvent,
};
use clients::{
    attestation::AttestationClient, erc1155::Erc1155Client, erc20::Erc20Client,
    erc721::Erc721Client, token_bundle::TokenBundleClient,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};
use tokio::runtime::Runtime;
use types::{AddressConfig, EscowClaimedLog};

pub mod clients;
pub mod types;

#[pyclass]
#[derive(Clone)]
pub struct AlkahestClient {
    inner: alkahest_rs::AlkahestClient,
    erc20: Erc20Client,
    erc721: Erc721Client,
    erc1155: Erc1155Client,
    token_bundle: TokenBundleClient,
    attestation: AttestationClient,
}

#[pymethods]
impl AlkahestClient {
    #[new]
    #[pyo3(signature = (private_key, rpc_url, address_config=None))]
    pub fn __new__(
        private_key: String,
        rpc_url: String,
        address_config: Option<AddressConfig>,
    ) -> PyResult<Self> {
        let address_config = address_config.map(|x| x.try_into()).transpose()?;
        let client = alkahest_rs::AlkahestClient::new(private_key, rpc_url, address_config)?;

        let client = Self {
            inner: client.clone(),
            erc20: Erc20Client::new(client.erc20),
            erc721: Erc721Client::new(client.erc721),
            erc1155: Erc1155Client::new(client.erc1155),
            token_bundle: TokenBundleClient::new(client.token_bundle),
            attestation: AttestationClient::new(client.attestation),
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

    #[pyo3(signature = (contract_address, buy_attestation, from_block=None))]
    pub async fn wait_for_fulfillment(
        &self,
        contract_address: String,
        buy_attestation: String,
        from_block: Option<u64>,
    ) -> eyre::Result<EscowClaimedLog> {
        Runtime::new()?.block_on(async {
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
    m.add_class::<AlkahestClient>()?;

    Ok(())
}
