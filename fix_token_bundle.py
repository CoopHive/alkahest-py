#!/usr/bin/env python3
"""
Script to convert all TokenBundleClient methods to async patterns.
"""

import re

def fix_token_bundle_methods():
    # Read the file
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/token_bundle.rs', 'r') as f:
        content = f.read()
    
    # Define method conversion patterns
    method_conversions = [
        # collect_payment
        (
            r'pub fn collect_payment\(\s*&self,\s*buy_attestation: String,\s*fulfillment: String,\s*\) -> eyre::Result<String> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn collect_payment<'py>(
        &self,
        py: pyo3::Python<'py>,
        buy_attestation: String,
        fulfillment: String,
    ) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner
                .collect_payment(buy_attestation.parse().map_err(map_parse_to_pyerr)?, fulfillment.parse().map_err(map_parse_to_pyerr)?)
                .await.map_err(map_eyre_to_pyerr)?;
            Ok(receipt.transaction_hash.to_string())
        })
    }'''
        ),
        # collect_expired
        (
            r'pub fn collect_expired\(&self, buy_attestation: String\) -> eyre::Result<String> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn collect_expired<'py>(&self, py: pyo3::Python<'py>, buy_attestation: String) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner.collect_expired(buy_attestation.parse().map_err(map_parse_to_pyerr)?).await.map_err(map_eyre_to_pyerr)?;
            Ok(receipt.transaction_hash.to_string())
        })
    }'''
        ),
        # buy_with_bundle
        (
            r'pub fn buy_with_bundle\(\s*&self,\s*price: TokenBundleData,\s*item: ArbiterData,\s*expiration: u64,\s*\) -> eyre::Result<LogWithHash<AttestedLog>> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn buy_with_bundle<'py>(
        &self,
        py: pyo3::Python<'py>,
        price: TokenBundleData,
        item: ArbiterData,
        expiration: u64,
    ) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner
                .buy_with_bundle(&price.try_into().map_err(map_eyre_to_pyerr)?, &item.try_into().map_err(map_eyre_to_pyerr)?, expiration)
                .await.map_err(map_eyre_to_pyerr)?;
            Ok(LogWithHash::<AttestedLog> {
                log: get_attested_event(receipt.clone()).map_err(map_eyre_to_pyerr)?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }'''
        ),
        # pay_with_bundle
        (
            r'pub fn pay_with_bundle\(\s*&self,\s*price: TokenBundleData,\s*payee: String,\s*\) -> eyre::Result<LogWithHash<AttestedLog>> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn pay_with_bundle<'py>(
        &self,
        py: pyo3::Python<'py>,
        price: TokenBundleData,
        payee: String,
    ) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner
                .pay_with_bundle(&price.try_into().map_err(map_eyre_to_pyerr)?, payee.parse().map_err(map_parse_to_pyerr)?)
                .await.map_err(map_eyre_to_pyerr)?;
            Ok(LogWithHash::<AttestedLog> {
                log: get_attested_event(receipt.clone()).map_err(map_eyre_to_pyerr)?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }'''
        ),
        # buy_bundle_for_bundle
        (
            r'pub fn buy_bundle_for_bundle\(\s*&self,\s*bid: TokenBundleData,\s*ask: TokenBundleData,\s*expiration: u64,\s*\) -> eyre::Result<LogWithHash<AttestedLog>> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn buy_bundle_for_bundle<'py>(
        &self,
        py: pyo3::Python<'py>,
        bid: TokenBundleData,
        ask: TokenBundleData,
        expiration: u64,
    ) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner
                .buy_bundle_for_bundle(&bid.try_into().map_err(map_eyre_to_pyerr)?, &ask.try_into().map_err(map_eyre_to_pyerr)?, expiration)
                .await.map_err(map_eyre_to_pyerr)?;
            Ok(LogWithHash::<AttestedLog> {
                log: get_attested_event(receipt.clone()).map_err(map_eyre_to_pyerr)?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }'''
        ),
        # pay_bundle_for_bundle
        (
            r'pub fn pay_bundle_for_bundle\(\s*&self,\s*buy_attestation: String,\s*\) -> eyre::Result<LogWithHash<AttestedLog>> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}',
            '''pub fn pay_bundle_for_bundle<'py>(
        &self,
        py: pyo3::Python<'py>,
        buy_attestation: String,
    ) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let receipt = inner
                .pay_bundle_for_bundle(buy_attestation.parse().map_err(map_parse_to_pyerr)?)
                .await.map_err(map_eyre_to_pyerr)?;
            Ok(LogWithHash::<AttestedLog> {
                log: get_attested_event(receipt.clone()).map_err(map_eyre_to_pyerr)?.data.into(),
                transaction_hash: receipt.transaction_hash.to_string(),
            })
        })
    }'''
        )
    ]
    
    # Apply conversions
    for pattern, replacement in method_conversions:
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    
    # Write back
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/token_bundle.rs', 'w') as f:
        f.write(content)
    
    print("Fixed TokenBundleClient methods")

if __name__ == "__main__":
    fix_token_bundle_methods()
    print("All TokenBundleClient fixes applied!")
