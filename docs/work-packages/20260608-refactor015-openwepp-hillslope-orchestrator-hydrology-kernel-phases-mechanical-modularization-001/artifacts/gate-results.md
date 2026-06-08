# REFACTOR015 gate results

Status: complete
Evidence mode: ran
Date: 2026-06-08

Disposition outcome: `complete-with-external-blocker`

## Static
- Required gate set was executed for this package.
- `hphys0225` workspace integration failure is pre-existing and unrelated to
  kernel-phase layout movement.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test -p openwepp-hillslope-orchestrator --tests`
   - result: pass
   - output sample: `107 passed; 0 failed`
4. `cargo test --workspace`
   - result: fail (exit 101)
   - failing test: `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs::hphys0225_runtime_source_forbids_legacy_max_reconciliation`
   - failure message: `HPHYS0225 must cap available pool from layer-derived state only`
5. `cargo deny check`
   - result: pass with warnings only
   - warnings:
     - duplicate lock entries: `getrandom`, `hashbrown`, `twox-hash`
     - unmatched license allowlist entries in `deny.toml`: `ISC`, `Unicode-DFS-2016`
