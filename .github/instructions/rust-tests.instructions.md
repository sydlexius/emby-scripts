---
applyTo: "smpr/src/**/*test*"
excludeAgent: "coding-agent"
---

# Test Code Review

- Tests run with `--test-threads=1` because config tests mutate process-global
  env vars. Verify test isolation where env vars are set and cleared.
- `#[ignore]` tests require a live media server (UAT). Verify they do not
  silently pass when the server is unreachable.
- Check test assertions for specificity: assert concrete values, not just
  `is_ok()` or `is_some()`.
- Verify that error paths are tested, not just success paths.
- Check for non-deterministic test behavior (unstable sort, timing-dependent).
