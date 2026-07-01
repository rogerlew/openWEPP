# Kernel-Boundary CQR Row 6 Growth Decomposition

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row6-growth-decomposition-001`

## Objective

Execute row #6 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in direct-runtime growth/decomposition files from
CRAP > 30 to CRAP <= 30 or an explicit ADR-0021 complete-with-warnings
disposition, while restoring row-relevant typed growth/decomposition assertions
at stable direct-runtime surfaces.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #6.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs`
- Focused tests or behavior-preserving decompositions needed to restore row #6
  typed growth/decomposition assertions.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No public output schema changes.
- No runtime selection or runner orchestration changes.
- No watershed orchestration changes.

## Plan

1. Record CRAP-before evidence for row #6.
2. Add focused tests or behavior-preserving decompositions for the measured
   row #6 offenders.
3. Re-measure row #6 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #6 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #6 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #6 typed growth/decomposition assertions are restored at stable typed surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW6-CQR`.

Row #6 reduced growth/decomposition CRAP offenders from `2` unique production
entries (`4` duplicated report rows) to `0` row-owned entries above CRAP 30.
Full gates passed, H2637 protected outputs are byte-identical, and
`compatibility_edge_invocations=0`.
