source .env/bin/activate
maturin develop

# Run comprehensive ERC20 test suite
echo "Running comprehensive ERC20 test suite..."
python alkahest_py/test_erc20.py

# Individual test files (for debugging if needed)
echo "Running individual tests..."
python alkahest_py/test_buy_with_erc20.py
