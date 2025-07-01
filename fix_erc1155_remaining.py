#!/usr/bin/env python3
"""
Script to fix the remaining Erc1155Client methods to use async patterns.
"""

import re
import sys

def fix_erc1155_methods():
    # Read the file
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/erc1155.rs', 'r') as f:
        content = f.read()
    
    # Pattern to match methods that still use self.runtime.block_on
    pattern = r'(pub fn \w+)\((.*?)\) -> eyre::Result<([^>]+)> \{\s*self\.runtime\.block_on\(async \{(.*?)\}\)\s*\}'
    
    def replace_method(match):
        method_name = match.group(1)
        params = match.group(2)
        return_type = match.group(3)
        body = match.group(4)
        
        # Add Python lifetime and py parameter
        if '&self' in params:
            new_params = params.replace('&self', "&self, py: pyo3::Python<'py>")
        else:
            new_params = params + ", py: pyo3::Python<'py>"
        
        # Create new method signature
        new_method = f"{method_name}<'py>({new_params}) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {{\n"
        new_method += "        let inner = self.inner.clone();\n"
        new_method += "        pyo3_async_runtimes::tokio::future_into_py(py, async move {\n"
        
        # Fix the body to use proper error mapping
        new_body = body.strip()
        # Replace error handling
        new_body = re.sub(r'\.parse\(\)\?', '.parse().map_err(map_parse_to_pyerr)?', new_body)
        new_body = re.sub(r'\.await\?', '.await.map_err(map_eyre_to_pyerr)?', new_body)
        new_body = re.sub(r'\.try_into\(\)\?', '.try_into().map_err(map_eyre_to_pyerr)?', new_body)
        new_body = re.sub(r'get_attested_event\(([^)]+)\)\?', r'get_attested_event(\1).map_err(map_eyre_to_pyerr)?', new_body)
        new_body = re.sub(r'self\.inner', 'inner', new_body)
        
        new_method += "            " + new_body.replace('\n', '\n            ') + "\n"
        new_method += "        })\n    }"
        
        return new_method
    
    # Apply the pattern with DOTALL flag for multiline matching
    content = re.sub(pattern, replace_method, content, flags=re.DOTALL)
    
    # Write back
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/erc1155.rs', 'w') as f:
        f.write(content)
    
    print("Fixed Erc1155Client methods")

def fix_static_methods():
    # Read the file
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/erc1155.rs', 'r') as f:
        content = f.read()
    
    # Fix the malformed static methods
    fixes = [
        # Fix encode_self in PyERC1155EscrowObligationStatement
        (r'pub fn encode_self\(encode_self\) -> PyResult<Vec<u8>> \{\s*PyERC1155EscrowObligationStatement::encode\(self\)\s*\}',
         'pub fn encode_self(&self) -> PyResult<Vec<u8>> {\n        PyERC1155EscrowObligationStatement::encode(self)\n    }'),
        
        # Fix decode in PyERC1155PaymentObligationStatement
        (r'pub fn decode\(decode\) -> PyResult<PyERC1155PaymentObligationStatement> \{',
         'pub fn decode(statement_data: Vec<u8>) -> PyResult<PyERC1155PaymentObligationStatement> {'),
        
        # Fix encode in PyERC1155PaymentObligationStatement
        (r'pub fn encode\(encode\) -> PyResult<Vec<u8>> \{',
         'pub fn encode(obligation: &PyERC1155PaymentObligationStatement) -> PyResult<Vec<u8>> {'),
        
        # Fix encode_self in PyERC1155PaymentObligationStatement
        (r'pub fn encode_self\(encode_self\) -> PyResult<Vec<u8>> \{\s*PyERC1155PaymentObligationStatement::encode\(self\)\s*\}',
         'pub fn encode_self(&self) -> PyResult<Vec<u8>> {\n        PyERC1155PaymentObligationStatement::encode(self)\n    }'),
        
        # Fix missing parse error mappings
        (r'let token: Address = obligation\.token\.parse\(\)\?;',
         'let token: Address = obligation.token.parse().map_err(map_parse_to_pyerr)?;'),
        
        (r'let token_id: U256 = obligation\.token_id\.parse\(\)\?;',
         'let token_id: U256 = obligation.token_id.parse().map_err(map_parse_to_pyerr)?;'),
        
        (r'let amount: U256 = obligation\.amount\.parse\(\)\?;',
         'let amount: U256 = obligation.amount.parse().map_err(map_parse_to_pyerr)?;'),
        
        (r'let arbiter: Address = obligation\.arbiter\.parse\(\)\?;',
         'let arbiter: Address = obligation.arbiter.parse().map_err(map_parse_to_pyerr)?;'),
        
        (r'let payee: Address = obligation\.payee\.parse\(\)\?;',
         'let payee: Address = obligation.payee.parse().map_err(map_parse_to_pyerr)?;'),
        
        # Fix decode method call
        (r'let decoded =\s*alkahest_rs::clients::erc1155::Erc1155Client::decode_payment_statement\(&bytes\)\?;',
         'let decoded = alkahest_rs::clients::erc1155::Erc1155Client::decode_payment_statement(&bytes).map_err(map_eyre_to_pyerr)?;'),
    ]
    
    for pattern, replacement in fixes:
        content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    
    # Write back
    with open('/Users/thanhngocnguyenduc/Desktop/alkahest-py/src/clients/erc1155.rs', 'w') as f:
        f.write(content)
    
    print("Fixed static methods")

if __name__ == "__main__":
    fix_erc1155_methods()
    fix_static_methods()
    print("All fixes applied!")
