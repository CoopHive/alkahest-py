### Run pytest directly

```bash
# Run all tests
pytest -v

# Run specific test file
pytest alkahest_py/test_erc20_approval.py -v

# Run tests with specific pattern
pytest -k "erc20" -v

# Run tests excluding slow ones (if marked)
pytest -m "not slow" -v
```

### Run tests from any directory

```bash
cd /path/to/alkahest-py
python -m pytest alkahest_py/ -v
```
