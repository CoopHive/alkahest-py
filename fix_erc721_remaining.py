#!/usr/bin/env python3

import re

# Read the file
with open('src/clients/erc721.rs', 'r') as f:
    content = f.read()

# Fix method signatures that weren't converted properly
# Pattern for methods with eyre::Result return type that need conversion
patterns_to_fix = [
    # Pattern 1: Methods with parameters but no py parameter
    (r'pub fn (\w+)\(\s*&self,\s*([^)]+)\) -> eyre::Result<([^>]+)>',
     r'pub fn \1<\'py>(&self, py: pyo3::Python<\'py>, \2) -> PyResult<pyo3::Bound<\'py, pyo3::PyAny>>'),
    
    # Pattern 2: Methods with only &self parameter
    (r'pub fn (\w+)\(\s*&self\s*\) -> eyre::Result<([^>]+)>',
     r'pub fn \1<\'py>(&self, py: pyo3::Python<\'py>) -> PyResult<pyo3::Bound<\'py, pyo3::PyAny>>'),
]

for pattern, replacement in patterns_to_fix:
    content = re.sub(pattern, replacement, content)

# Fix specific issues with inner references that weren't caught
content = re.sub(r'let receipt = self\s*\.\s*inner\.', 'let receipt = inner.', content)

# Fix get_attested_event calls that weren't mapped to error handlers
content = re.sub(r'get_attested_event\(([^)]+)\)\?', r'get_attested_event(\1).map_err(map_eyre_to_pyerr)?', content)

# Write back the fixed content
with open('src/clients/erc721.rs', 'w') as f:
    f.write(content)

print("Fixed remaining ERC721 method signatures and error handling")
