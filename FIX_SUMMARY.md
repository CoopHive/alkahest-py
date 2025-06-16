# Fix for "failed to register pending transaction to watch" Error

## Problem Description

After the first call to `approve()`, any subsequent function calls would fail with the error:

```
failed to register pending transaction to watch
```

## Root Cause

The issue was caused by each Python wrapper method creating a new Tokio runtime using `Runtime::new()?.block_on()`. This approach caused several problems:

1. **Multiple runtimes**: Creating multiple Tokio runtimes can cause conflicts, especially when dealing with async code that might have pending transactions or watchers.
2. **Runtime per call**: Each function call created a completely new runtime, which doesn't share state with previous calls.
3. **Transaction watching**: The underlying blockchain client tries to maintain state about pending transactions, but since each call gets a new runtime, this state is lost.

## Solution

We implemented a **shared runtime architecture** where:

1. **Single Runtime Creation**: Each `PyAlkahestClient` instance now has a shared `Arc<Runtime>` that is created once during client initialization.

2. **Runtime Sharing**: All client wrappers (`Erc20Client`, `Erc721Client`, `Erc1155Client`, `TokenBundleClient`, `AttestationClient`) now accept and store a reference to the shared runtime.

3. **Consistent Runtime Usage**: All methods now use `self.runtime.block_on()` instead of creating new runtimes.

## Changes Made

### 1. Updated PyAlkahestClient (src/lib.rs)

- Added `runtime: std::sync::Arc<tokio::runtime::Runtime>` field
- Modified `__new__()` to create and share the runtime
- Updated `from_client()` to create and pass runtime to all clients
- Updated `wait_for_fulfillment()` to use shared runtime

### 2. Updated All Client Wrappers

**Modified files:**

- `src/clients/erc20.rs`
- `src/clients/erc721.rs`
- `src/clients/erc1155.rs`
- `src/clients/token_bundle.rs`
- `src/clients/attestation.rs`

**Changes in each file:**

- Added `runtime: std::sync::Arc<tokio::runtime::Runtime>` field to the struct
- Updated `new()` constructor to accept runtime parameter
- Replaced all `Runtime::new()?.block_on()` with `self.runtime.block_on()`
- Removed unused `tokio::runtime::Runtime` imports

### 3. Code Cleanup

- Removed unused `Runtime` imports from all client files
- Cleaned up other unused imports (`PyResult`, etc.)

## Verification

The fix was verified by:

1. **Build Success**: Code compiles without errors
2. **Existing Tests Pass**: All existing test suites continue to work
3. **Sequential Calls Test**: Created and ran a specific test that reproduces the original issue:
   - Multiple `approve()` calls
   - Multiple `approve_if_less()` calls
   - Mixed sequences of different function calls
   - All tests pass without the original error

## Impact

- ✅ **Fixed**: "failed to register pending transaction to watch" error
- ✅ **Maintained**: All existing functionality works as before
- ✅ **Improved**: Better resource management with shared runtime
- ✅ **Performance**: Reduced overhead from creating multiple runtimes

## Test Results

```
🚀 Testing Runtime Fix for 'failed to register pending transaction to watch'
🧪 Testing sequential function calls...
✅ First approve() successful
✅ Second approve_if_less() successful
✅ Third approve() call with different purpose successful
✅ Fourth approve_if_less() call successful
🎉 SUCCESS! All sequential calls completed without errors!
✅ The 'failed to register pending transaction to watch' issue has been fixed.
```

The fix successfully resolves the runtime conflict issue while maintaining full backwards compatibility.
