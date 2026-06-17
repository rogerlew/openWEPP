# PERFIDX03 Gate Results

Status: HOLD 2026-06-17
Evidence mode: **Ran** + **Static**

## Required Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Diverse pre-flip registry coverage | PASS | Five-case audit has `unknown_symbol_count = 0` for every case. |
| Authority flip | FAIL/HOLD | Flip was attempted, but production activation was disabled after OFE5 timing regressed. |
| H2637 + ladder bit identity | NOT RUN | Expensive full anchor stopped because realized performance failed first. |
| Exercised-case logical output identity | PASS/PARTIAL | Synthetic multi-OFE and OFE5 cases showed no logical output divergence. |
| Determinism | PASS/PARTIAL | HBP/wat/plot deterministic; pass parquet byte churn exists even in baseline, logical rows equal. |
| Realized speedup | FAIL | OFE5 active flip mean `38.34s` vs baseline mean `27.01s`. |
| Current tree avoids accepted regression | PASS | No-flip OFE5 current sample `26.80s`. |

## Rust Closure Gates

```text
cargo fmt --check
PASS

cargo clippy --workspace --all-targets -- -D warnings
PASS

cargo test --workspace
PASS

cargo deny check
advisories ok, bans ok, licenses ok, sources ok

git diff --check
PASS
```

## Notes

The gate result is HOLD, not PASS. The current tree keeps registry coverage fixes
and inactive indexed-authority support, but the production runner does not enable
the regressing authority path.
