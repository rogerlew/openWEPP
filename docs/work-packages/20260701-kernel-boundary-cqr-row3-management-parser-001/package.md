# Kernel-Boundary CQR Row 3 Management Parser

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row3-management-parser-001`

## Objective

Execute row #3 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in the typed PL management runtime-input projection
file from CRAP > 30 to CRAP <= 30 or an explicit ADR-0021
complete-with-warnings disposition, while restoring row-relevant typed
management assertions at stable runtime-input surfaces.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #3.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- Focused tests or behavior-preserving decompositions needed to restore row #3
  typed management parser/projection assertions.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No public output schema changes.
- No parser grammar changes.
- No runtime selection or runner orchestration changes.

## Plan

1. Record CRAP-before evidence for row #3.
2. Add focused tests or behavior-preserving decompositions for the measured row
   #3 offender.
3. Re-measure row #3 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #3 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #3 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #3 typed management assertions are restored at stable typed surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW3-CQR`.

Row #3 reduced its scoped owned production functions above CRAP 30 from `1`
unique offender (`2` duplicated report rows) to `0` without ADR-0021 warning
disposition. H2637 protected outputs remained byte-identical and
`compatibility_edge_invocations=0`.
