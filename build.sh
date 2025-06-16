source .env/bin/activate
maturin develop
python alkahest_py/test_erc20_approve_if_less.py