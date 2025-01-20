from alkahest_py import AlkahestClient
import asyncio

client = AlkahestClient(
    "0x...",
    "https://base-sepolia.infura.io/v3/...",
)


async def main():
    hash = await client.erc20.approve(
        {"address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e", "value": 100},
        "escrow",
    )

    print(hash)


if __name__ == "__main__":
    asyncio.run(main())
