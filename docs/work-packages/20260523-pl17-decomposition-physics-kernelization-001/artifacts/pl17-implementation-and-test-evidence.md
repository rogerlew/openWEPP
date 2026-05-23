# PL17 Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implementation Summary

Static:
- `openwepp-hillslope-orchestrator` decomposition dispatch now computes equation-driven updates for tracked decomposition pools before payload emission.
- Added dedicated PL17 decomposition equation input validators and state-domain validation helpers.
- Added annual/perennial event modifiers on equation-updated pools (burn/remove/cut transfer and grazing digest-based removal).
- Runtime management projection now emits slot/crop decomposition parameter symbols (`oratea`, `orater`) and primary aliases.
- Integration test seed surfaces updated for PL17 required-symbol posture.

## Commands Executed

```bash
cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture
cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture
cargo test -p openwepp-hillslope-orchestrator
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

## Results

- PL17 contract tests: `4 passed`
- INT10 coupling tests: `3 passed`
- Orchestrator crate tests: `51 passed`
- `fmt`: `ok`
- `clippy -D warnings`: `ok`
- `workspace tests`: `ok`
- `deny`: `ok` (with non-fatal unmatched allowlist warnings)
