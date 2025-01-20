# alkahest-py

## build

0. install [uv](https://docs.astral.sh/uv/) & [the rust toolchain](https://rustup.rs). install maturin with `uv tool install maturin`

1. clone this repo and https://github.com/coophive/alkahest-rs into parallel directories

```bash
mkdit alkahest-sdks && cd alkahest-sdks
git clone https://github.com/coophive/alkahest-rs
git clone https://github.com/coophive/alkahest-py
```
2. build alkahest-py

```bash
cd alkahest-py
uv run maturin build
```

3. generate docs via `cargo doc --open`

## usage

1. install into any python project via the wheel, e.g. `pip install path-to-alkahest-py/target/wheels/alkahest_py-0.1.0-cp313-cp313-macosx_11_0_arm64.whl` (or `uv pip install...`). your exact file might be different by system, but should be in the same directory.

2. import the module and create an AlkahestClient instance

```python
from alkahest_py import AlkahestClient

client = AlkahestClient(
        "0xprivatekey",
        "https://rpc_url.com"
)

async def main():
    hash = await client.erc20.approve(
        {"address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e", "value": 100},
        "escrow",
    )

    print(hash)

if __name__ == "__main__":
    asyncio.run(main())
```

3. see the `cargo doc` generated docs for API details. most functions are in the submodules Erc20Client, Erc721Client etc. the alkahest-rs docs will probably be more useful than the alkahest-py docs, since many rust types get wrangled into python strings. all "bytes" types (FixedBytes<32>, Address, Bytes) are strings starting with "0x" in python. structs (ArbiterData, Erc20Data) are dictionaries with item names matching the struct's fields. ApprovalPurpose can be "escrow" or "payment".

see alkahest-py/test.py for a usage example.
