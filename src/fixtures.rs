use alloy::sol;

// Test mock contracts
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    EAS,
    "src/fixtures/EAS.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    SchemaRegistry,
    "src/fixtures/SchemaRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    MockERC20Permit,
    "src/fixtures/MockERC20Permit.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    MockERC721,
    "src/fixtures/MockERC721.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    MockERC1155,
    "src/fixtures/MockERC1155.json"
);

use crate::{
    types::{AddressConfig, PyAddressConfig},
    utils::PyWalletProvider,
    PyAlkahestClient,
};
use alkahest_rs::{
    types::WalletProvider,
    utils::{setup_test_environment, MockAddresses, TestContext},
};
use alloy::{
    node_bindings::AnvilInstance,
    primitives::{Address, U256},
    providers::Provider,
};
use pyo3::{pyclass, pymethods, PyResult};

#[pyclass]
pub struct PyMockERC20 {
    inner: MockERC20Permit::MockERC20PermitInstance<WalletProvider>,
}
impl PyAlkahestClient {
    pub fn inner_provider(&self) -> &WalletProvider {
        &self.inner.wallet_provider
    }
}

#[pymethods]
impl PyMockERC20 {
    #[new]
    pub fn new(address: String, provider: &PyWalletProvider) -> PyResult<Self> {
        let addr = address
            .parse::<Address>()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let contract = MockERC20Permit::MockERC20PermitInstance::new(addr, provider.inner.clone());

        Ok(Self { inner: contract })
    }

    #[getter]
    pub fn address(&self) -> String {
        format!("{:?}", self.inner.address())
    }

    pub fn transfer(&self, to: String, value: u64) -> PyResult<()> {
        println!("Transferring {} tokens to {}", value, to);
        let to_addr = to
            .parse::<Address>()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        println!("Parsed address: {:?}", to_addr);
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            self.inner
                .transfer(to_addr, U256::from(value))
                .send()
                .await
                .map_err(|e| {
                    eprintln!("Transfer failed: {e}");
                    pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
                })?
                .get_receipt()
                .await
                .map_err(|e| {
                    eprintln!("Get receipt failed: {e}");
                    pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
                })?;

            Ok(())
        })
    }

    pub fn balance_of(&self, address: String) -> PyResult<u128> {
        let addr = address
            .parse::<Address>()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let rt = tokio::runtime::Runtime::new()?;
        let balance = rt
            .block_on(async { self.inner.balanceOf(addr).call().await })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let balance_u128 = balance.try_into().map_err(|_| {
            pyo3::exceptions::PyOverflowError::new_err("Balance too large for u128")
        })?;

        Ok(balance_u128)
    }
}
