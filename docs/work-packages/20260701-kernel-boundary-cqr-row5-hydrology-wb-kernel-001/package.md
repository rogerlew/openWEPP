# Kernel-Boundary CQR Row 5 Hydrology WB Kernel

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row5-hydrology-wb-kernel-001`

## Objective

Execute row #5 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in hydrology WB kernel files from CRAP > 30 to CRAP
<= 30 or an explicit ADR-0021 complete-with-warnings disposition, while
restoring row-relevant typed hydrology assertions at stable contract surfaces.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #5.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- Focused tests or behavior-preserving decompositions needed to close row #5
  CRAP offenders and restore hydrology WB kernel assertions.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No public output schema changes.
- No direct-runtime publication or runner orchestration changes except if a
  row #5 assertion requires a stable public contract test.
- No watershed orchestration changes.

## Plan

1. Record CRAP-before evidence for row #5.
2. Add focused tests or behavior-preserving decompositions for the measured
   row #5 offenders.
3. Re-measure row #5 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #5 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #5 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #5 typed hydrology contract assertions are restored at stable typed surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW5-CQR`.

Row #5 reduced hydrology WB kernel CRAP offenders from `11` unique production
entries (`22` duplicated report rows) to `0` row-owned entries above CRAP 30.
Full gates passed, H2637 protected outputs are byte-identical, and
`compatibility_edge_invocations=0`.
