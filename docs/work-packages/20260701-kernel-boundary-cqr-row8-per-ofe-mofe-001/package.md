# Kernel-Boundary CQR Row 8 Per-OFE MOFE

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row8-per-ofe-mofe-001`

## Objective

Execute row #8 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in direct-runtime per-OFE/MOFE files from CRAP > 30
to CRAP <= 30 or an explicit ADR-0021 complete-with-warnings disposition, while
restoring row-relevant typed per-OFE/MOFE assertions at stable direct-runtime
surfaces.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #8.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- Focused tests or behavior-preserving decompositions needed to restore row #8
  typed per-OFE/MOFE assertions.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No public output schema changes.
- No runtime selection or runner orchestration changes.
- No watershed orchestration changes.

## Plan

1. Record CRAP-before evidence for row #8.
2. Add focused tests or behavior-preserving decompositions for the measured
   row #8 offenders.
3. Re-measure row #8 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #8 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #8 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #8 typed per-OFE/MOFE assertions are restored at stable typed surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Result: `EXECUTED-COMPLETE-ROW8-CQR`.

Row #8 reduced its scoped owned production functions above CRAP 30 from `2`
unique offenders (`4` duplicated report rows) to `0` without ADR-0021 warning
disposition. H2637 protected outputs remained byte-identical and
`compatibility_edge_invocations=0`.
