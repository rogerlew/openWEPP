# Kernel-Boundary CQR Row 1 Soil Parser

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row1-soil-parser-001`

## Objective

Execute row #1 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: confirm the
soil runtime-input projection scope remains CRAP-clean and restore the
secondary typed soil parser/runtime assertions called out by the execplan.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #1.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No parser grammar changes.
- No public output schema changes.
- No runtime selection or runner orchestration changes.

## Plan

1. Record CRAP-before evidence for row #1.
2. Restore typed soil parser/runtime assertions for 9002 policy FC/WP,
   corrected theta projection, and harmonic vertical `ssc`.
3. Re-measure row #1 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #1 on `main` with the CRAP-before -> after disposition.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Row #1 owned production scope remains at 0 functions above CRAP 30. |
| Secondary coverage | 9002 policy FC/WP, corrected typed theta stores, and harmonic vertical `ssc` assertions are restored at stable parser/runtime-input surfaces. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW1-CQR`.

Row #1 remained CRAP-clean (`0 -> 0` production functions above CRAP 30) while
restoring the secondary typed 9002 soil parser/runtime assertions required by
the burndown execplan. No production parser, runtime projection, science math,
or output schema changed.
