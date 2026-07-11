# Required Reading Map

Status: `EXECUTED-INTAKE`

Evidence mode: `Static`

Byte counts were recorded with `wc -c` on 2026-07-11 at
`main@9fa0a294`, before contract, test, or production edits.

## Threshold Disposition

- Core: `390364` bytes — `OK` (`<=400000`).
- Core + conditional authority: `633359` bytes — `WARN`.
- Core + conditional authority + prior correction context: `665881` bytes —
  `WARN`.
- Full listed set including implementation-local source: `953491` bytes —
  `REQUIRES-JUSTIFICATION`.

Justification: `LANED-NOB-001` changes a conservation-sensitive production
router and its canonical contract. The large total is driven by the work-
package catalog, `SC-OFEROUTE-001`, and the active solver/runtime sources.
Those files cannot be omitted without losing contract authority, the rev-41
positivity posture, rev-47 local-numerics context, the real active consumer,
or the acceptance fixtures. Implementation-local files remain on-demand until
attribution identifies a touched mechanism; this preserves progressive
disclosure without weakening the declared write-set review.

## Core

| Bytes | Path | Intake disposition |
| ---: | --- | --- |
| 10624 | `AGENTS.md` | Read |
| 20708 | `docs/codex_exec_plans.md` | Read |
| 24803 | `docs/defect_closure_execplans.md` | Read |
| 19044 | `docs/work-packages/AGENTS.md` | Read |
| 279179 | `docs/work-packages/README.md` | Relevant catalog/current-state sections read; full catalog is reference-only |
| 12359 | `docs/work-packages/20260710-laned-router-h2637-34yr-negativeoutletbin-defect-closure-001/package.md` | Read |
| 8343 | `docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md` | Read |
| 5171 | `crates/AGENTS.md` | Read |
| 4534 | `tests/AGENTS.md` | Read |
| 5599 | `docs/specifications/science-contracts/AGENTS.md` | Read |

Core subtotal: `390364` bytes.

## Conditional Authority

These files apply because the package amends canonical routing authority and
changes production kernel behavior.

| Bytes | Path | Intake disposition |
| ---: | --- | --- |
| 3328 | `docs/standards/AGENTS.md` | Read |
| 13488 | `docs/standards/kernel-work-package-preparation.md` | Read |
| 3501 | `docs/standards/local-ci-gate-selection.md` | Read |
| 12423 | `docs/specifications/science-contract-authoring-procedure.md` | Read |
| 5044 | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Read |
| 9024 | `docs/specifications/science-contracts/index.md` | Routing lifecycle row read |
| 175917 | `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Relevant authority, algorithm, guard, invariant, tolerance, obligation, and history sections read; remaining historical narrative is on-demand |
| 2473 | `docs/decisions/0011-architecture-first-top-down-science-contracts.md` | Read |
| 8943 | `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` | Read |
| 8854 | `docs/decisions/0037-abandon-hybrid-implicit-stepping.md` | Read |

Conditional-authority subtotal: `242995` bytes.

## Prior Correction Context

| Bytes | Path | Relevance |
| ---: | --- | --- |
| 7146 | `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/package.md` | Rev-41 correction envelope |
| 2273 | `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/solver-localization.md` | Conservative face-limiter mechanism |
| 1403 | `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/implementation.md` | Rev-41 implementation shape |
| 1464 | `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/final-disposition.md` | Rev-41 closure evidence |
| 15511 | `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md` | Rev-47 authority and acceptance envelope |
| 1226 | `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/implementation.md` | Rev-47 implementation summary |
| 868 | `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/disposition.md` | Rev-47 final state |
| 1297 | `docs/work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001/artifacts/implementation.md` | Current hot-path delta |
| 1334 | `docs/work-packages/20260708-laned-router-post-tier1-hotpath-sweep-001/artifacts/final-disposition.md` | Current post-sweep baseline |

Prior-correction subtotal: `32522` bytes. All rows were read during intake.

## Implementation-Local On Demand

| Bytes | Path | Trigger |
| ---: | --- | --- |
| 101982 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | Solver/outlet-bin attribution and correction |
| 25628 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | Inter-OFE handoff if implicated |
| 15007 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/seam.rs` | Supply booking if implicated |
| 20755 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` | Plain Case-4 oracle validation |
| 19571 | `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs` | Contract-derived outlet/handoff regression |
| 62116 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | Active day consumer/closure proof |
| 15487 | `crates/openwepp-runner/src/hillslope/laned_active.rs` | Runtime forcing/diagnostic projection if implicated |
| 27064 | `tests/integration/laned_shadow_h2637.rs` | Committed H2637 regression and selector/off-path gates |

Implementation-local subtotal: `287610` bytes. Attribution-critical slices of
the solver, active runtime, and H2637 fixture were read during intake; each file
is read to the touched mechanism before its first edit.
