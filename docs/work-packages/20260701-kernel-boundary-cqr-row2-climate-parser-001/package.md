# Kernel-Boundary CQR Row 2 Climate Parser

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row2-climate-parser-001`

## Objective

Execute row #2 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: confirm the
climate runtime-input projection scope remains CRAP-clean and restore the
secondary typed climate parser/runtime assertions called out by the execplan.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #2.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
- `tests/integration/infile_climate_parser_contract.rs`
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No parser grammar changes.
- No public output schema changes.
- No runtime selection or runner orchestration changes.

## Plan

1. Record CRAP-before evidence for row #2.
2. Restore typed climate parser/runtime assertions for non-breakpoint direct
   forcing, breakpoint direct forcing, datver-0 runtime policy, and direct-day
   out-of-range runtime errors.
3. Re-measure row #2 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #2 on `main` with the CRAP-before -> after disposition.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Row #2 owned production scope remains at 0 functions above CRAP 30. |
| Secondary coverage | Typed climate parser/runtime assertions are restored for direct non-breakpoint forcing, breakpoint forcing, datver-0 override behavior, itemp runtime rejection, and day-index errors. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW2-CQR`.

Row #2 remained CRAP-clean (`0 -> 0` production functions above CRAP 30) while
restoring secondary typed climate parser/runtime assertions. No production
parser, runtime projection, science math, or output schema changed.
