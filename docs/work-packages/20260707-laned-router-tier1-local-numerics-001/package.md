# LANED Router Tier-1 Local Numerics

Status: EXECUTED-SUPERSEDED-BY-GAP-OFEHYB-002

Superseded by:
`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/`

This package is retained as the earlier broad Tier-1 scaffold. Execute the
GAP-OFEHYB-002 package instead; it carries the current source-memory hybrid
baseline, the explicit `SC-OFEROUTE-002#GAP-OFEHYB-002` closure target, and
`INV-OFEHYB-008` fidelity/timing ratification obligations.

Execution note (2026-07-07): this package was executed as a supersession
closure after `GAP-OFEHYB-002` completed on `main`. No additional code,
contract, comparator, or timing work is authorized from this obsolete broad
Tier-1 scaffold.

## Objective

Execute the Tier-1 local-numerics optimization package for the Lane-D router:
analytic/cheaper celerity, Newton or equivalent faster friction/equilibrium
solves, and bounded local math reductions such as `h * sqrt(h)` in place of
`powf` where contract-ratified.

## Rationale

D15A exhausted bit-identical optimization headroom. T3-AGG then proved the
hybrid coverage arithmetic but not endpoint value because implicit cell solves
are dominated by repeated friction/equilibrium evaluations. Tier-1 attacks the
shared local numeric cost that prices both explicit CFL/step work and implicit
branch solves.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md`
- `docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/optimization-plan.md`
- `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/final-disposition.md`

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs`
- D10B Case-4 oracle tests and active H2637 fixtures.

## Scope

Included:
- Contract-first amendment for any non-bit-identical local numeric method.
- Baseline/profile refresh using the D14 two-instrument structure: endpoint
  timing plus persistent slot/profile counters, and perf if needed.
- Implementation of one coherent Tier-1 optimization set with independent
  correctness tests.
- Case-4 oracle ladder, active H2637 timing/profile, and fidelity delta
  evidence.

Excluded:
- Mesh-resolution policy changes (Tier-2).
- Hybrid selector promotion/default activation.
- Surrogate physics, empirical retuning, or compatibility wrappers.

## Phase Plan

1. **T1-S0 Baseline/profile.** Rebuild the exact release runner, record binary
   provenance, run H2637 active/plain and active-hybrid profile where relevant,
   and identify the local numeric cost share.
2. **T1-S1 Contract amendment.** Amend `SC-OFEROUTE-001` for the selected local
   numeric changes and acceptance tolerances before code.
3. **T1-S2 Implementation.** Land the selected analytic celerity/Newton/math
   reductions with typed guards and no silent fallback.
4. **T1-S3 Focused validation.** Run friction/equilibrium unit vectors,
   `ofe_routing` quick tests, and any new contract-derived vectors.
5. **T1-S4 Comparator/timing.** Run Case-4 oracle ladder, H2637 active endpoint
   timing/profile, and fidelity deltas vs pre-change active/hybrid outputs.
6. **T1-S5 Review/disposition.** Complete dual review, dual verification,
   line-count governance, gate results, and final disposition.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` subagents for timing/comparator gates, dual
review, QA verification, and bounded codebase questions. Expected outputs are
package-local review/verification/timing artifacts. Write access is read-only
unless a worker is explicitly assigned a disjoint implementation write set.

Subagent requirement: REQUIRED for heavy release timing, comparator ladders, and
full closure gates unless unavailable; record any tool-policy block before
running locally.

## Required Artifacts

- `artifacts/baseline-profile.md`
- `artifacts/optimization-plan.md`
- `artifacts/implementation.md`
- `artifacts/timing-and-fidelity.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks for touched `SC-*` contracts
- Focused `ofe_routing` and Lane-D active tests
- Case-4 oracle ladder within current ratified tolerances
- H2637 active endpoint/profile timing with exact binary provenance
- Fidelity deltas with named tolerances or hold disposition
- Protected-output byte identity if default/off surfaces are touched
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `.rs` line-count governance
