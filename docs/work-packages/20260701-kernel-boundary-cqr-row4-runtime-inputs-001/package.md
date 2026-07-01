# Kernel-Boundary CQR Row 4 Runtime Inputs

Status: EXECUTED-COMPLETE-ROW4-CQR

Package id: `20260701-kernel-boundary-cqr-row4-runtime-inputs-001`

## Objective

Execute row #4 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in the runtime input core/snow/frost/irrigation,
projection-helper, SIMIMPL28 hourly-forcing, and series-helper files from
CRAP > 30 to CRAP <= 30 or an explicit ADR-0021 complete-with-warnings
disposition, while restoring the deleted row-relevant typed contract
assertions.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #4.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/07_series_helpers.rs`
- Focused crate or integration tests needed to restore row #4 contract
  assertions at typed runtime input surfaces.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No symbol-map runtime restoration.
- No public output schema changes.
- No watershed orchestration changes.

## Plan

1. Record CRAP-before evidence for row #4.
2. Add focused tests or behavior-preserving decompositions for the measured
   row #4 offenders.
3. Re-measure row #4 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity where applicable, full Rust gates,
   authority guards, line-count governance, dual review, and verification.
5. Commit row #4 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #4 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #4 typed contract assertions are restored at stable typed runtime input surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Complete.

Row #4 CRAP burndown reduced 24 unique production offender entries
(`48` duplicated report rows) to 0 entries above CRAP 30. Focused typed
assertions, full Rust gates, authority guards, H2637 byte-identity, review, and
verification are recorded in `artifacts/`.
