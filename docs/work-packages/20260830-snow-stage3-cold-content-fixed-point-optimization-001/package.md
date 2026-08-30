# Stage-3 cold-content fixed-point optimization

Status: `ACTIVE`

Execution mode: `package-end-to-end`

This is a living ExecPlan maintained under `docs/codex_exec_plans.md`.

## Progress

- [x] 2026-08-30: scaffold package and freeze the retained baseline.
- [ ] Record result-blind iteration-history evidence.
- [ ] Select and contract a bounded solver correction.
- [ ] Implement and run focused qualification.
- [ ] Run canonical one-day qualification and reconcile disposition.

## Surprises and discoveries

- The predecessor reduced comparison-driven work but left 128 fixed-point caps;
  100 are cold-content-only after the other coupled maps converge.

## Decision log

- Decision: isolate the remaining fixed-point behavior in a fresh package.
  Rationale: its solver-policy authority, diagnostics, and acceptance baseline
  are distinct from the completed refreeze-lineage increment.
  Date/author: 2026-08-30, Codex.

## Outcomes and retrospective

Queued until result-bearing execution.

## Objective

Reduce the canonical one-day adaptive workload by correcting the covered
fixed-point behavior responsible for the remaining cold-content-only
96-iteration caps. The retained exact-head baseline at commit `792af753e` is
504 accepted supports, 227 rejected trials, 128 fixed-point caps, 45 scaled
physical-comparison rejections, 49 exact-floor supports, and `374.23 s` body
wall. One hundred caps are Picard/finalization attempts where LSE, soil, and
complete boundaries already converge and Stage 3 first differs at per-layer
`cold_content_j_m2`.

## Rationale

These bounded failures force support refinement despite convergence of the
other coupled maps. At watershed scale, repeated carrier evaluation dominates
wall time. The package must identify whether the cold-content map is slowly
contracting, oscillatory, or repeatedly reset by finalization, then correct the
limiting numerical transition without loosening physical tolerances.

## Included scope

- transient, opt-in, non-persisted iteration-history evidence;
- covered Stage-3/soil Picard and finalization convergence policy;
- exact 60-second fallback and stable ordinary supports;
- canonical SnowEnergy authority and contract-derived anti-evasion vectors if
  solver policy changes;
- five-parent/focused fixture and complete canonical one-day qualification;
- exact accepted/rejected counts, width histogram, runtime, rejection reasons,
  iteration distribution, and ledger/receipt closure.

## Excluded scope

- archive, memory, serialization-size, and generic per-step optimization;
- changes to constitutive snow, soil, vegetation, LSE, or WB14 physics;
- tolerance relaxation, state clamping, event/discrete weakening, or a larger
  temporal floor;
- persisted microstepping diagnostics, receipt fields, restart fields, or
  public output schema changes;
- production cutover or unrelated external-science HOLD surfaces.

## Correction authority envelope

Observed defect `STAGE3-FP-COLD-CAP-001` is the material population of valid
covered supports that exhaust 96 iterations after all coupled maps except
Stage-3 cold content have converged. In-scope authority is
`SC-SNOWENERGY-001`, its `TOL-SNOWENERGY-003` convergence norms, authentic-map
replay, fixed-point policy, and existing conservation/receipt invariants.
Allowed edits are diagnostic-only transient audit additions, contract-first
bounded numerical-iterate policy, contract-derived tests, the covered
fixed-point implementation, and package evidence. No proxy physics, tolerance
change, accepted-state canonicalization, or fallback acceptance is allowed.

## Deliverables

- result-blind convergence-history diagnosis distinguishing slow contraction,
  oscillation, and finalization reset;
- canonical contract amendment and failing pre-implementation vector for any
  selected solver change;
- smallest bounded implementation with discrete/posture and cumulative-ledger
  refusal tests;
- replacement five-parent and canonical one-day evidence;
- reviews, verification, line-count disposition, exact-diff reconciliation,
  disposition, and handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- predecessor package
  `docs/work-packages/20260826-snow-stage3-adaptive-compositional-microstepping-001/`

## Intended write set

- `docs/work-packages/README.md`
- this package tree
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs`
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/receipt_sets.rs`
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs`
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs`
- `crates/openwepp-runner/src/hillslope/tests03/cqr_laned_active_outputs.rs`
- compiler-discovered SnowEnergy contract consumers under `tests/integration/`

## Phase plan

Phase 0 freezes baseline/source identity, maps required reading, and records
iteration-history evidence without changing solver behavior. Phase 1 selects
one correction and executes contracts, contract-derived tests, and a failing
pre-implementation gate. Phase 2 implements the bounded solver change and runs
focused/five-parent evidence. Phase 3 runs canonical one-day qualification,
reconciles the terminal diff, reviews findings, verifies gates, and disposes.

## Exit criteria

- diagnostic evidence identifies the cold-only cap mechanism without entering
  production persistence or controller decisions;
- contract-first sequencing is evidenced before production edits;
- cold-content-only caps fall materially from 100 and total fixed-point caps
  fall from 128 without tolerance relaxation;
- accepted/rejected trials improve from 504/227, exact-floor supports do not
  increase from 49, and body wall does not regress from `374.23 s`;
- exact event/discrete comparison rejections remain zero;
- mass, energy, phase, custody, topology, receipt, rollback, and authentic
  installed-owner invariants pass with unchanged thresholds;
- affected source quality, contract, component, integration, consumer,
  conservation, restart/receipt, and anti-evasion requirements pass;
- exact terminal diff is reconciled and all findings are dispositioned.

## Review, verification, and security

Dual independent review and dual verification are required before `COMPLETE`.
Higher-precedence session policy currently prohibits delegated subagents, so
execution must not falsely label a local audit as independent; technical work
continues and final disposition remains review-bound if that policy persists.
Line-count governance applies at 2,000/3,000 lines. Security impact is
`NONE`: no network, credential, privilege, dependency, unsafe, or external
message surface is authorized.

Subagent authorization: none for this execution; local validation is required
and no delegated work may be claimed.

## Gate evidence non-deferral

Every current-scope exit criterion requires direct current evidence. A failed
or missing required gate yields truthful `HOLD`; it cannot be relabeled as a
later increment after implementation begins. Diagnostic uncertainty, effort,
or partial improvement is not a legitimate stop while an authority-backed
in-envelope correction remains available.
