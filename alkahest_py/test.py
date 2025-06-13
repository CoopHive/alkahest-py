# from alkahest_py import AlkahestClient
# from dotenv import load_dotenv
# import os
# import asyncio

# load_dotenv()

# client = AlkahestClient(os.environ["PRIVATE_KEY"], os.environ["RPC_URL"])


# async def main():
#     # hash = await client.erc20.approve(
#     #     {"address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e", "value": 100},
#     #     "escrow",
#     # )
#     # print(hash)

#     escrow = await client.erc20.buy_with_erc20(
#         {"address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e", "value": 100},
#         {"arbiter": "0x0000000000000000000000000000000000000000", "demand": b""},
#         0,
#     )
#     print(escrow)


# if __name__ == "__main__":
#     asyncio.run(main())

from alkahest_py import PyTestEnvManager, PyMockERC20 
import asyncio
env = PyTestEnvManager()
print("RPC:", env.rpc_url)
print("God:", env.god)
wallet = env.god_wallet_provider
mocks = env.mock_addresses
print("ERC20 A:", mocks.erc20_a)
print("ERC721 B:", mocks.erc721_b)
mock = PyMockERC20(mocks.erc20_a, wallet)
print("alice :", env.alice)

# Transfer 100 tokens to Alice
mock.transfer(env.alice, 100)

# Check god balance
balance = mock.balance_of(env.god)
print("God wallet balance:", balance)

alice_balance = mock.balance_of(env.alice)
print("Alice wallet balance:", alice_balance)

# Erc20Data as dict
price = {
    "address": env.mock_addresses.erc20_a,
    "value": 100
}

# ArbiterData as dict
item = {
    "arbiter": env.addresses.erc20_addresses.payment_obligation,
    "demand": b"custom demand data"
}
async def main():
    result = await env.alice_client.erc20.permit_and_buy_with_erc20(price, item, 1749839992)
    print("Buy result:", result)

asyncio.run(main())


# Check god balance
balance2 = mock.balance_of(env.god)
print("God wallet balance:", balance2)

alice_balance2 = mock.balance_of(env.alice)
print("Alice wallet balance:", alice_balance2)
