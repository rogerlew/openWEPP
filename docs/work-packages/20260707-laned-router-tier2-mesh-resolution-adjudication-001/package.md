# LANED Router Tier-2 Mesh-Resolution Adjudication

Status: SUPERSEDED-BY-20260708-DX-TARGET-RESCOPE

Supersession note (2026-07-07): do not execute this package as scaffolded.
ADR-0037 abandoned the hybrid implicit stepper and demoted H2637 to synthetic
stress evidence only, so this hybrid-era, 5-cells/OFE framing is replaced by
`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/`.
This file remains as historical context for the re-scope.

## Objective

Adjudicate whether the production Lane-D router may use a lower per-OFE mesh
resolution (candidate: 5 cells/OFE) under named fidelity tolerances, and land
the contract/config change only if the evidence supports it.

## Rationale

Router cost scales strongly with cells/OFE because finer spatial resolution
also tightens explicit CFL timesteps. The current 10-cell production mesh is a
working D-val setting, not yet proven to be the minimum production fidelity
requirement. This package separates mesh policy from Tier-1 local numeric
changes so timing gains and fidelity risk remain auditable.

## Required Reading

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- D10B package artifacts for Case-4 oracle acceptance.
- D15A and T3/T3-AGG timing artifacts.
- Tier-1 final disposition if Tier-1 has executed.

On-demand:
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`

## Scope

Included:
- Contract-first mesh-resolution tolerance amendment.
- Case-4 oracle ladder at candidate mesh policies.
- H2637 active endpoint/profile timing and hydrograph/fidelity deltas.
- Production config change only if tolerances are authority-backed and pass.

Excluded:
- Local numerical formula changes (Tier-1).
- Hybrid implicit selector promotion/default activation.
- Retuning physics or accepting fidelity deltas without named tolerances.

## Phase Plan

1. **T2-S0 Baseline.** Record current 10-cell active/hybrid timing, counters,
   and fidelity surfaces with exact release binary provenance.
2. **T2-S1 Tolerance proposal.** Draft mesh-resolution acceptance criteria in
   `SC-OFEROUTE-001` before code/config edits.
3. **T2-S2 Candidate runs.** Run Case-4 and H2637 at candidate mesh policies,
   including 5-cell and any intermediate needed for a non-divergence ladder.
4. **T2-S3 Adjudication.** Accept, reject, or hold the production mesh policy
   with evidence. Do not land a production config change unless criteria pass.
5. **T2-S4 Implementation if accepted.** Land only the ratified config/code
   surface and prove active/default/off behavior.
6. **T2-S5 Review/disposition.** Complete dual review, dual verification,
   gates, line-count governance, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `explorer` subagents for mesh comparator ladders,
timing/profile runs, review, verification, and bounded codebase questions.
Expected outputs are package-local review/verification/timing artifacts. Write
access is read-only unless a worker is explicitly assigned a disjoint
implementation write set.

Subagent requirement: REQUIRED for heavy comparator/timing/full closure gates
unless unavailable; record any tool-policy block before running locally.

## Required Artifacts

- `artifacts/baseline-profile.md`
- `artifacts/mesh-fidelity-adjudication.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
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
- Case-4 oracle ladder at current and candidate mesh policies
- H2637 active endpoint/profile timing with exact binary provenance
- H2637 fidelity deltas with named tolerances or hold disposition
- Protected-output byte identity if default/off surfaces are touched
- Focused Lane-D / `ofe_routing` tests
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `.rs` line-count governance
