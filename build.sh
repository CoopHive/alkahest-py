source .env/bin/activate
maturin develop

echo "Running test suite..."
pytest -v





