# SNOW-SURFACE-EB-04W2B Warm-Day New-Snow Activation And Closure

Status: `executed / HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`

Date: `2026-08-02`

Plan class: `Defect-Closure ExecPlan / critical kernel control and conservation`

This living DC-ExecPlan follows `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`.

## Purpose / Objective

Close the EB-04W2A snowbench mass loss end-to-end. A valid positive typed
hourly snowfall event must enter the shared snow partition even when prior SWE
is zero and daily mean temperature is nonnegative, and the public consumer
must fail closed unless independently reconstructed daily SWE storage closes.
After closure, rerun the exact frozen W2A legacy/albedo comparison without
changing cells, forcing, coefficients, or hypothesis rules.

## Implementation Intent

Intent: `production correction + conservation validation + diagnostic rerun`.
This is contract-derived control and bookkeeping work, not new process physics
or empirical calibration. W2A's contrast remains inadmissible until every
replayed cell closes.

## Correction Authority Envelope

Defect `SNOW-SURFACE-EB-04W2B-D001`: the public typed partition can return an
inactive outcome with zero accumulation when hourly forcing contains positive
snowfall, prior SWE is zero, and daily mean temperature is nonnegative.

Defect `SNOW-SURFACE-EB-04W2B-D002`: the public typed partition and snowbench
consumer do not fail closed on independently reconstructed daily SWE storage.

Authorized corrections are limited to canonical snow/runoff contract text,
contract-derived tests, the shared typed snow-partition activation and closure
boundary, and snowbench's real-consumer closure check. Typed hourly snowfall is
authoritative at this consumer boundary regardless of whether it was produced
by direct-runtime Harder-Pomeroy partitioning or the snowbench forcing bridge.
No upstream phase-provider behavior is generalized or changed.

Conversion rule: if the reproducible mechanism is in this envelope and the
expected behavior is supported by canonical contracts, pinned provenance, or
a contract-authorized physical invariant, proceed through contract amendment,
tests, pre-implementation gate, production correction, validation, and dual
review. Do not relay an intermediate diagnostic step.

The seven-gate bar is reproduction, named mechanism, ownership, authority,
safety, testability, and measurable validation. `HOLD` is exceptional and is
invalid while an in-envelope contract amendment, test, correction, or
validation remains possible. A hold-legitimacy audit must name the boundary,
cite proof, list the attempted correction route, and explain why it cannot
close here.

## Included Scope And Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`;
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`;
- contract/test bindings required by those amendments;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs` for the production typed-forcing availability trigger only;
- `crates/openwepp-runner/src/hillslope/snowbench.rs` and
  `snowbench_coe_melt.rs` for typed kernel/consumer closure errors;
- direct-publication snow consumer control and its focused real-path test;
- focused module/integration tests for the shared API and snowbench consumer;
- `docs/ROADMAP.md`, `docs/planning/snow-surface-energy-balance-roadmap.md`,
  and `docs/work-packages/README.md`;
- the snow assurance report's source identity/review locks and generated
  adoption transaction required when the canonical snow contract changes;
- W2A package-local runner only if required to rerun the frozen comparison;
- ignored rerun outputs under `target/`.

## Excluded Scope

- Harder-Pomeroy, legacy RST, or external snowbench phase equations; material
  direct precipitation may activate the existing provider, but its phase math
  and decisions remain protected;
- precipitation factors, forcing files, observations, cells, hypothesis rules,
  melt/albedo coefficients, selectors, defaults, or promotion;
- new surrogate/proxy/heuristic physics, silent clamping, or fallback defaults;
- unrelated snow density, Stage-3 energy, canopy, runoff, or publication work.

## Phase Plan

1. Scaffold, bind authority, and reproduce the failure.
2. Amend contracts; add failing contract-derived warm-day snow, mixed-event,
   rain-only, and independent closure tests; record pre-implementation gate.
3. Correct the shared activation and fail-closed closure boundary, then enforce
   the independent check through the real snowbench consumer.
4. Run focused, frost, quick, critical full-workspace, lint, doc, and
   conservation validation selected from the exact diff.
5. Complete two independent Rust reviews, finding disposition, and two
   prerequisite verifications of criteria 1-6 before any result-bearing rerun.
6. If and only if those prerequisites pass, rerun W2A's exact frozen eight-cell
   contrast and adjudicate it under its unchanged rules.
7. Have the same two verifiers terminally recheck the rerun, exact diff,
   prompt lifecycle, roadmap/catalog update, and final disposition.

## Acceptance Criteria

1. Zero-prior-SWE, warm-mean typed snowfall activates the shared partition and
   conserves its exact `snowfall_depth * 0.1` SWE input.
2. A mixed typed rain/snow event exercises the same real path and closes daily
   storage; warm all-rain/no-pack behavior remains inactive and unchanged.
3. Daily closure is independently reconstructed as
   `SWE_before + typed_snowfall_SWE + rain_retained - snowpack_loss - sublimation - SWE_after`
   and material residuals hard-fail at the contract tolerance.
4. Direct runtime and snowbench both consume the corrected shared API; wrappers
   or shadow paths do not carry the closure claim.
5. No phase, forcing, coefficient, selector, default, or melt equation changes.
6. Every required critical validation, dual review/disposition, dual
   verification, prompt lifecycle, line-count, security, and exact-diff gate
   passes.
7. The frozen W2A contrast reruns only after criteria 1-6 prerequisites pass;
   its scientific result is reported without result-aware rule changes, then
   terminal verification is refreshed against the retained rerun.

## Validation And Delegation

Risk: `Critical` because the diff changes a production activation predicate and
mass-conservation boundary. Run affected contract tests, focused owning-crate
and snowbench tests, frost and quick profiles, warnings-denied formatting/lint,
full-workspace correctness, documentation checks, and frozen-consumer evidence.

Subagent authorization: this package explicitly authorizes two independent
Rust review agents and two independent terminal verification agents. Reviewers
may write only their named package artifact and return compact findings;
verifiers may write only their named verification artifact. No agent may edit
production, contracts, tests, roadmap, or another agent's artifact.

No surrogate physics: production edits must implement canonical contract-backed
control and conservation behavior only.

Real consumer proof: demonstrate both direct-runtime construction and the
snowbench executable use the corrected public typed partition; negative proof
must exclude wrapper, skeleton, shadow, and compatibility-only claims.

## Progress

- [x] (2026-08-02) User authorized scaffold and autonomous execution.
- [x] (2026-08-02) Reconciled the authority boundary: upstream phase providers
  may differ, but valid typed snowfall is authoritative to the shared consumer.
- [x] (2026-08-02) Amended both governing contracts after a reproducing red
  gate and added warm-snow, mixed-event, rain-only, and closure vectors.
- [x] (2026-08-02) Corrected shared activation and added independently
  reconstructed fail-closed storage checks at both partition and snowbench
  consumer boundaries.
- [x] (2026-08-02) Reran the frozen W2A contrast: all eight cells closed; the
  albedo response remained immaterial and did not qualify for promotion. This
  first run was later adjudicated prerequisite-ineligible by review.
- [x] (2026-08-02) First dual review found the direct-production pre-provider
  bypass, typed-error loss, missing guard tests, and evidence defects.
- [x] (2026-08-02) Corrected every in-envelope finding and passed focused,
  owning-crate, frost, clippy, formatting, and assurance validation.
- [x] (2026-08-02) Renewed quick validation found a deterministic downstream
  EROD16 continuity-instrument regression (`61/231` refusals versus the hard
  `<=20%` bound). A temporary old-trigger reversal was directionally
  consistent with causal ownership but did not retain closure-grade evidence.
- [x] (2026-08-02) Corrected re-review's two remaining in-envelope findings:
  the presence threshold is sufficient rather than exclusive, and a real
  production builder/frame test now proves the published snow storage handoff
  and hydrology-projection SWE.
- [x] (2026-08-02) Dual re-review and dual terminal verification pass the
  corrected snow diff for `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`; both verifiers
  independently reproduce EROD16 at `61/231` and reject terminal rerun/full
  closure or EB-04X advancement.

## Surprises & Discoveries

- W2A proves the shared snowbench failure but does not prove a direct-runtime
  phase-provider defect; direct retained traces contain no typed snow on the
  implicated dates. This package therefore corrects only consumer semantics.
- The assurance catalog binds canonical source hashes. Amending
  `SC-SNOWFREEZE-001` therefore created an immediate, non-deferrable source
  adoption obligation; the snow report was adopted with a checked transaction
  rather than left stale.
- Review exposed a second pre-consumer gate: production SIMIMPL28 suppressed
  typed hourly rows on warm-mean precipitation days before the corrected
  partition could inspect snowfall. Resolving existing phase rows for any
  material precipitation is therefore a non-deferrable control prerequisite;
  no phase equation or coefficient changes.
- Correct warm-mean snowfall changes the McKenzie burn fixture's runoff storm
  population from 227 to 231 and raises the separate concave EROD16
  instrument's flux-closure refusals from 37 to 61. The production fixture run
  itself completes, but the retained cross-domain correctness gate fails
  deterministically. Erosion solver mechanics and that gate's authority are
  outside this package's snow correction envelope.

## Decision Log

- Decision: treat typed hourly forcing as authoritative after the producer
  boundary and independently reconstruct snowfall SWE from that forcing.
  Rationale: this conserves valid input without changing or conflating phase
  providers. Date/Author: 2026-08-02 / Codex.

## Outcomes & Retrospective

The warm-day snowfall loss and shared/consumer closure defects are corrected in
the working diff, including the previously hidden direct-production provider
bypass. Nominal and boundary tests pass, and W2A's earlier diagnostic result is
scientifically suggestive but prerequisite-ineligible.

Promotion/closure is held because corrected winter hydrology exposes a hard
downstream EROD16 continuity-instrument failure. The terminal W2A rerun was not
performed, the critical full profile was not repeated after this deterministic
quick-profile failure, and EB-04X must not start. A separately authorized
cross-domain package should partition the 24 added refusals, determine whether
the concave solver needs a numerical/process correction or the fixture gate
requires authority-backed rebaselining, and return a clean full-workspace gate
before EB-04W2B can resume.
