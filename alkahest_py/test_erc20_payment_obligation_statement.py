import pytest
from alkahest_py import PyTestEnvManager, PyERC20PaymentObligationStatement

@pytest.mark.asyncio
async def test_basic_encode_decode():
    env = PyTestEnvManager()
    
    obligation = PyERC20PaymentObligationStatement(
    token=env.mock_addresses.erc20_a,
    amount=100,
    payee=env.addresses.erc20_addresses.payment_obligation
    )
    
    encoded_data = PyERC20PaymentObligationStatement.encode(obligation)
    decoded_obligation = PyERC20PaymentObligationStatement.decode(encoded_data)

    assert obligation.amount == decoded_obligation.amount, "Amount mismatch"
    assert obligation.token.lower() == decoded_obligation.token.lower(), "Token mismatch"
    assert obligation.payee.lower() == decoded_obligation.payee.lower(), "Payee mismatch"
