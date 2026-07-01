# Kernel-Boundary CQR Row 7 WB Publication

Status: COMPLETE

Package id: `20260701-kernel-boundary-cqr-row7-wb-publication-001`

## Objective

Execute row #7 of
`docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`: reduce every
owned production function in WB publication, direct publication, and direct seed
projection files from CRAP > 30 to CRAP <= 30 or an explicit ADR-0021
complete-with-warnings disposition, while restoring row-relevant typed contract
assertions at stable publication/seed-projection surfaces.

## Authority

- `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`, row #7.
- ADR-0021 CRAP threshold: production functions above 30 are non-conforming.
- `docs/specifications/science-contracts/AGENTS.md`: no physics change without
  contract-first authority.

## Scope

In scope:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-runner/src/hillslope/direct_seed_projections/**`
- Focused tests or behavior-preserving decompositions needed to restore row #7
  typed publication and seed-projection assertions.
- Package artifacts and work-package catalog updates.

Out of scope:

- No process-physics changes.
- No public output schema changes.
- No symbol-map runtime restoration.
- No watershed orchestration changes outside direct publication consumers.

## Plan

1. Record CRAP-before evidence for row #7.
2. Add focused tests or behavior-preserving decompositions for the measured
   row #7 offenders.
3. Re-measure row #7 CRAP-after with the same LCOV + `cargo crap` method.
4. Run focused tests, H2637 identity, full Rust gates, authority guards,
   line-count governance, dual review, and verification.
5. Commit row #7 on `main` with the CRAP-before -> after delta.

## Gates

| Gate | Requirement |
| --- | --- |
| Primary CRAP | Every row #7 owned production function with CRAP > 30 is <= 30 or explicitly dispositioned. |
| Secondary coverage | Deleted row #7 typed publication/seed-projection contract assertions are restored at stable typed surfaces; no touched-file line coverage regression. |
| Behavior identity | H2637 protected outputs remain byte/value identical for this behavior-preserving CQR row. |
| Rust gates | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`, `cargo deny check`. |
| Authority guards | `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`. |
| Governance | `.rs` line-count governance recorded; dual review and dual verification artifacts complete. |

## Disposition

Executed complete. Row #7 CRAP offenders were reduced from 17 unique production
entries above 30 to 0 in the refreshed full-workspace CRAP report. H2637
protected outputs remain byte-identical to the retained baseline, and full gates
passed.
