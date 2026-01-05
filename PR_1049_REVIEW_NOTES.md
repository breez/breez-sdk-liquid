# PR #1049 Review Notes: Retry chain service fetching during recovery

**PR**: https://github.com/breez/breez-sdk-liquid/pull/1049
**Author**: @dangeross
**Branch**: `savage-recovery-chain-retry` → `main`

## Summary

This PR adds retry logic to chain service methods (`get_script_history_with_retry` and `script_get_balance_with_retry`) to handle issues during recovery of large real-time synced datasets that hit chain service rate limits.

---

## Code Analysis

### Files Changed

| File | Changes |
|------|---------|
| `lib/core/src/chain/bitcoin/electrum.rs` | Added `get_script_history_with_retry`, `script_get_balance_with_retry` |
| `lib/core/src/chain/bitcoin/esplora.rs` | Added `get_script_history_with_retry`, `script_get_balance_with_retry` |
| `lib/core/src/chain/bitcoin/mod.rs` | Updated trait with new methods |
| `lib/core/src/chain/liquid/electrum.rs` | Added `get_script_history_with_retry` |
| `lib/core/src/chain/liquid/esplora.rs` | Added `get_script_history_with_retry` |
| `lib/core/src/chain/liquid/mod.rs` | Updated trait with new method |
| `lib/core/src/test_utils/chain.rs` | Updated mock implementations |

---

## Issues Found

### 1. **Inconsistent Backoff Strategy Between Bitcoin and Liquid Implementations** (Medium)

**Bitcoin implementations** use exponential backoff:
```rust
// bitcoin/electrum.rs:143, bitcoin/esplora.rs:150
tokio::time::sleep(Duration::from_secs(retry)).await;  // 1s, 2s, 3s...
```

**Liquid implementations** use constant 1-second delays:
```rust
// liquid/electrum.rs:108, liquid/esplora.rs:137
tokio::time::sleep(Duration::from_secs(1)).await;  // Always 1s
```

This inconsistency could lead to confusion and different behavior. Consider standardizing the backoff strategy.

### 2. **Retry Logic Only Triggers on Empty Results, Not Errors** (High)

The current implementation:
```rust
script_history = self.get_script_history(script).await?;  // Error propagates immediately
match script_history.is_empty() {
    true => { /* retry */ }
    false => break,
}
```

Rate limits typically return HTTP errors (429), not empty results. The `?` operator immediately propagates errors rather than retrying. This means:
- **Actual rate limit errors** (HTTP 429) → immediate failure, no retry
- **Empty results** → retry

This may not be the intended behavior for handling rate limits.

### 3. **Recoverer Doesn't Use the New Retry Methods** (Medium)

Looking at `recoverer.rs`:
```rust
// recoverer.rs:283-285 - Uses batch method without retry
let lbtc_script_histories = self
    .liquid_chain_service
    .get_scripts_history(&swap_lbtc_scripts)  // No retry version
    .await?;

// recoverer.rs:322-325 - Same for Bitcoin
let btc_script_histories = self
    .bitcoin_chain_service
    .get_scripts_history(&swap_btc_scripts)  // No retry version
    .await?;
```

The retry methods are for single scripts (`get_script_history`), but recovery uses batch methods (`get_scripts_history`). The new retry logic won't help recovery directly.

The retry methods **are** used in:
- `verify_tx` methods (with 10 retries for Bitcoin, 30 for Liquid)
- `get_script_utxos` in Liquid (with 10 retries)

### 4. **Code Duplication** (Low)

As noted in the PR discussion, the retry pattern is duplicated across 4 implementations. A shared helper like `with_retries()` would reduce duplication:

```rust
async fn with_retries<T, F, Fut>(
    retries: u64,
    should_retry: impl Fn(&T) -> bool,
    operation: F,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
```

### 5. **Semantic Concern with Retry Condition** (Medium)

The retry triggers when `script_history.is_empty()`, but an empty history can be a legitimate result for scripts with no transactions. The assumption that empty = rate limited may cause unnecessary delays for scripts that genuinely have no history.

### 6. **Off-by-One Consideration** (Low/Info)

With `while retry <= retries`, the total attempts are `retries + 1`:
- `retries = 3` → up to 4 total attempts (initial + 3 retries)

This is fine but should be documented clearly in the API.

---

## Questions for the Author

1. **Rate limit error handling**: If the issue is rate limiting, shouldn't we retry on errors (potentially specific error types) rather than empty results?

2. **Batch vs single script retry**: Would it make sense to add retry versions of `get_scripts_history` (batch) for direct use in the recoverer?

3. **Backoff consistency**: Should the Liquid implementations also use exponential backoff like Bitcoin?

4. **Empty result validity**: Is it valid to have a script with no history during recovery, or should empty always trigger a retry?

---

## Suggestions for Improvement

### 1. Add Error-Based Retry Logic

Consider catching rate limit errors specifically:
```rust
async fn get_script_history_with_retry(&self, script: &Script, retries: u64) -> Result<Vec<History>> {
    let mut retry = 0;
    loop {
        match self.get_script_history(script).await {
            Ok(history) if !history.is_empty() => return Ok(history),
            Ok(history) if retry >= retries => return Ok(history),
            Err(e) if retry >= retries => return Err(e),
            result => {
                retry += 1;
                let delay = Duration::from_secs(retry);
                info!("Retry {retry}/{retries} after {delay:?}: {:?}", result.as_ref().err());
                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

### 2. Extract Common Retry Logic

Create a utility function to reduce duplication:
```rust
// In a utils module
pub async fn retry_with_backoff<T, E, F, Fut>(
    max_retries: u64,
    should_retry: impl Fn(&Result<T, E>) -> bool,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
```

### 3. Add Batch Retry Method

For use in the recoverer:
```rust
async fn get_scripts_history_with_retry(
    &self,
    scripts: &[&Script],
    retries: u64,
) -> Result<Vec<Vec<History>>>;
```

### 4. Standardize Backoff Strategy

Use the same exponential backoff pattern across all implementations.

---

## Testing Considerations

- Add unit tests for retry behavior with mocked chain services
- Test both error retry and empty-result retry scenarios
- Test that the exponential backoff timing is correct
- Test boundary conditions (0 retries, max retries)

---

## Overall Assessment

**The PR addresses a real issue** encountered during recovery with large datasets. However, there are some concerns about whether the retry logic will actually help with rate limits (since it doesn't retry on errors) and the fact that the recoverer uses batch methods that don't have retry versions.

**Recommendation**: Consider revising to:
1. Add error-based retry logic for actual rate limit errors
2. Either add batch retry methods or refactor the recoverer to use single-script methods
3. Standardize the backoff strategy across implementations
4. Extract common retry logic to reduce duplication

The implementation is functional for retrying on empty results, but may not fully address the original rate limit issue as described in the PR.
