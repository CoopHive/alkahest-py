#!/usr/bin/env python3

import re
import sys

def convert_method_signature(line):
    """Convert method signature from sync to async PyO3 format."""
    # Pattern: pub fn methodname(&self, params...) -> eyre::Result<ReturnType>
    pattern = r'pub fn (\w+)\(&self,([^)]*)\) -> eyre::Result<([^>]+)>'
    match = re.search(pattern, line)
    
    if match:
        method_name = match.group(1)
        params = match.group(2).strip()
        return_type = match.group(3).strip()
        
        # Add py parameter
        if params:
            new_params = f" py: pyo3::Python<'py>,{params}"
        else:
            new_params = " py: pyo3::Python<'py>"
        
        return f"    pub fn {method_name}<'py>(&self,{new_params}) -> PyResult<pyo3::Bound<'py, pyo3::PyAny>> {{"
    
    return line

def convert_method_body(content):
    """Convert method body from runtime.block_on to pyo3_async_runtimes::tokio::future_into_py."""
    # Replace runtime.block_on with async wrapper
    content = re.sub(
        r'self\.runtime\.block_on\(async \{',
        '''let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {''',
        content
    )
    
    # Replace self.inner with inner
    content = re.sub(r'self\.inner\.', 'inner.', content)
    
    # Fix error handling
    content = re.sub(r'\.try_into\(\)\?', '.try_into().map_err(map_eyre_to_pyerr)?', content)
    content = re.sub(r'\.parse\(\)\?', '.parse().map_err(map_parse_to_pyerr)?', content)
    content = re.sub(r'\.await\?', '.await.map_err(map_eyre_to_pyerr)?', content)
    content = re.sub(r'return Err\(eyre::eyre!\("Invalid purpose"\)\);', 
                     'return Err(map_eyre_to_pyerr(eyre::eyre!("Invalid purpose")));', content)
    content = re.sub(r'get_attested_event\(([^)]+)\)\?', 
                     r'get_attested_event(\1).map_err(map_eyre_to_pyerr)?', content)
    content = re.sub(r'Ok\(LogWithHash \{', 'Ok(LogWithHash::<AttestedLog> {', content)
    
    return content

def main():
    if len(sys.argv) != 2:
        print("Usage: python convert_erc721_methods.py <rust_file>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    lines = content.split('\n')
    converted_lines = []
    
    for line in lines:
        # Convert method signatures
        converted_line = convert_method_signature(line)
        converted_lines.append(converted_line)
    
    converted_content = '\n'.join(converted_lines)
    converted_content = convert_method_body(converted_content)
    
    with open(file_path, 'w') as f:
        f.write(converted_content)
    
    print(f"Converted {file_path}")

if __name__ == "__main__":
    main()
