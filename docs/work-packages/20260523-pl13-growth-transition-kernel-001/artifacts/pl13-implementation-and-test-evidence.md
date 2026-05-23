# PL13 Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implementation Summary

Static:

- `crates/openwepp-kernel-contract/src/lib.rs`
  - Added typed growth transition payload model:
    - growth state surface struct
    - annual/perennial growth control structs
    - transition control enum
    - transition payload struct
  - Extended `HillslopeGrowthKernelContext` with typed
    `transition_payload` builder support.

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Implemented production growth transition dispatch for annual/perennial
    branches.
  - Added typed growth-domain validators and deterministic daily action
    selectors.
  - Added typed error variants/codes (`HS-GROWTH-E-005..007`) for
    non-integral, out-of-range, and invalid growth state/payload domains.
  - Implemented explicit reset-state payload emission for planting/harvest/
    stop/senescence actions.

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - Seeds required growth transition state symbols in runtime surfaces:
    `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`.

- `tests/integration/parser_runtime_seam_integration.rs`
  - Added PL13 integration assertions for annual and perennial transition
    payload emission through scheduler execution.

- Contract authority updates:
  - `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  - `docs/specifications/science-contracts/index.md`

## Targeted PL13 Tests

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator pl13_contract_conformance -- --nocapture
cargo test -p openwepp --test parser_runtime_seam_integration pl13_contract_conformance -- --nocapture
cargo test -p openwepp-kernel-contract growth_context_can_carry_typed_transition_payload -- --nocapture
```

Result:

- Orchestrator PL13 conformance tests: `2 passed`, `0 failed`.
- Integration PL13 scheduler payload tests: `2 passed`, `0 failed`.
- Kernel-contract typed growth payload carriage test: `1 passed`, `0 failed`.

## Required Package Gates

Ran:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result:

- `cargo fmt --check`: `ok`
- `cargo clippy --workspace --all-targets -- -D warnings`: `ok`
- `cargo test --workspace`: `ok`
- `cargo deny check`: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)

Note:
- `cargo deny check` emitted `license-not-encountered` warnings for unmatched
  allowlist entries in `deny.toml`; exit status remained success.
