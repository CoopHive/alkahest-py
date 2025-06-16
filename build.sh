source .env/bin/activate
maturin develop

# Run comprehensive ERC20 test suite
echo "Running comprehensive ERC20 test suite..."
python alkahest_py/test_erc20.py

# Individual test files (for debugging if needed)
echo "Running individual tests..."
python alkahest_py/test_buy_with_erc20.py
python alkahest_py/test_erc20_approval.py
python alkahest_py/test_erc20_approve_if_less.py
python alkahest_py/test_permit_and_buy_with_erc20.py
python alkahest_py/test_pay_with_erc20.py
python alkahest_py/test_permit_and_pay_with_erc20.py
python alkahest_py/test_buy_erc20_for_erc20.py
python alkahest_py/test_permit_and_buy_erc20_for_erc20.py
python alkahest_py/test_pay_erc20_for_erc20.py
python alkahest_py/test_erc20_escrow_obligation_statement.py
