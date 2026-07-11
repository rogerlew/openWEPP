# Lane D Active Router 34-Year H2637 `NegativeOutletBin` Defect Closure

Status: `EXECUTED-COMPLETE`

Package ID: `20260710-laned-router-h2637-34yr-negativeoutletbin-defect-closure-001`

Queue row: none (defect discovered by the 2026-07-10 endpoint audit;
`ROADMAP.md` insertion is an operator sequencing decision).

Execution mode: `package-end-to-end`

Evidence mode: `Static + Ran`

## Objective

Close defect `LANED-NOB-001`: the Lane D active production-default router
fail-closes with `NegativeOutletBin` on the canonical 34-year H2637 endpoint
— valid canonical inputs, deterministic, first observed the first time the
production default was soaked on that endpoint. Until this closes, the
production default cannot complete the program's canonical perf/closure
fixture and no completed active-router timing exists for it.

This is a DC-ExecPlan (`docs/defect_closure_execplans.md`). It diagnoses
internally and lands a contract-first production correction when the
seven-gate bar is met. It may not relay the mechanism to a diagnostic-only
successor.

## Starting Evidence

`docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md`
(executional; staging recipe, binary provenance, and full matrix therein):

- All four active-default runs (3 reps `wepp_ui=false`, 1 rep
  `wepp_ui=true`) died identically:
  `CLIHILL-E-011 … HS-SIMPIPE-E-001 direct runtime day execution failed at
  lane 8 day 2621: direct runtime kernel guard failed in
  laned_active_cascade: lane 8 day 2621 routing failed: NegativeOutletBin`.
- Day 2621 ≈ 1994-03-05 (±1 index base): zero-precipitation hard-freeze day
  (7.2 °C → −5.0 °C) immediately after a three-day ~46 mm warm rain spell —
  a recession/melt-tail, source-quiet regime.
- The committed 2-year fixture (`tests/fixtures/laned_shadow_h2637`,
  1987–88) never reaches the failing span; the whole active arc predates
  any full-length soak. Its solver counters still match the Tier-1/post-
  sweep records exactly (no routing-workload drift on the covered span).
- Repro cost ≈ 50 s user to failure; daily/off configs on identical inputs
  complete clean (39–41 s, closure green).
- `RoutingError::NegativeOutletBin` has exactly two return sites, both in
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
  (~L1671, ~L2152 at `main@0c1ae324`); which fired is not yet known.

Historical context — **assessment-class, non-binding** (mechanism
attribution belongs to this package): the `NegativeOutletBin` class was
previously produced by front-arrival terminal-bin deficits in the abandoned
hybrid composition (fixed there by cross-span deficit carry, rev 30; the
subsystem was stripped per ADR-0037 — its plain-path byte-identity gate
means the strip did not cause this), and rev 41 landed the WA
positivity-preserving solver correction for a related plain-path positivity
family. Candidate neighborhoods worth ruling in/out early: plain-path
terminal-bin deficit under recession/source-quiet forcing; rev-47 Tier-1
numerics (analytic celerity / bounded Newton α-q) behavior in near-dry
recession states; hourly supply-seam booking on a freeze-following-melt day.

## Reproduction

Stage the 34-year inputs per audit §2: canonical wepp-forest WB05A
`with_wepp_ui/runs/` H2637 inputs (sha-pinned in the audit), management
patched exactly like the Tier-1 timing fixture (`ow-lanuse-1` datver, 19
`NativeCropland` landuses, 19 `routing_coefficients 500.0 0.0 0.0 0.0 0.0`
blocks), runfile with `wepp_ui = false`, five outputs on; run
`openwepp-cli-hill --run-dir <runs> --run-file <toml> --output-dir <out>
--policy compat --legacy-sidecar-discovery`. Fails in ~50 s. The audit
session's staging is scratch, not durable; restage from the canonical
sources.

## Correction Authority Envelope

### Defect

1. `LANED-NOB-001`: active production-default routing fail-closes
   (`NegativeOutletBin`, lane 8 day 2621) on the canonical 34-year H2637
   endpoint with valid inputs. Diagnostic-first: the symptom is established
   (deterministic ×4); the mechanism is not yet named — an attribution
   milestone is required before correction.

### In-scope authority and write set

- `SC-OFEROUTE-001` (invariant amendment/confirmation for terminal/outlet-
  bin nonnegativity under recession and source-quiet spans).
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/` (notably
  `kinematic_wave.rs`, `cascade.rs`, `seam.rs`, `dval.rs`) and module tests
  including `d10b_reconciliation_tests.rs`.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`.
- `crates/openwepp-runner/src/hillslope/laned_active.rs` (active supply /
  seam construction and its booking into the router).
- `tests/integration/laned_shadow_h2637.rs`; a new committed regression
  fixture/vector (strategy adjudicated in-package: minimal climate window
  reaching the failing regime vs. full-length staged vector behind an
  ignored gate — the committed regression must be deterministic and cheap
  enough for the workspace gate).
- This package; catalog/roadmap rows as the operator sequences.

### Authorized evidence surfaces

Trace/diagnostic envs (`OPENWEPP_LANED_ACTIVE_TRACE`,
`OPENWEPP_LANED_ACTIVE_STEP_TRACE`, `OPENWEPP_LANED_SHADOW_PROFILE`),
package-local instrumented runs on the staged 34-year inputs, and
day-2621-localized state capture. Diagnostics land in package artifacts,
not in production code.

### Allowed correction classes

- Amend or confirm canonical invariants/guards for the failing regime
  before any production edit (contract is the proximate authority,
  ADR-0011).
- Correct solver stepping, state carry, bin accounting, or celerity/α-q
  evaluation to canonical authority so positivity holds **by construction
  inside the flux update** (the rev-41 posture), not by clamping.
- Correct supply-seam booking if the mechanism is a seam accounting error
  (exact-ledger discipline; no attribution slack beyond the contract's
  recorded dust floors).
- Refine the typed failure only if the triggering input is proven invalid
  upstream — in which case terminal state is a validated non-defect with
  the upstream producer named, not a loosened guard.

### Protected boundaries

- No loosening, removal, or tolerance-widening of the fail-closed guard; no
  negative-storage clamping, empirical damping, mass injection/removal, or
  publication-only masking.
- No hybrid-stepper revival in any form (ADR-0037: revival starts from the
  archive branch through a new contract, never in-place).
- Winter/snow **physics** is out of scope: if the mechanism attributes to
  snow/winter kernels producing physically wrong (as opposed to wrongly
  booked) supply, HOLD at that boundary and route to the backlogged snow
  science review (ADR-0017 posture). The seam *booking* of that supply is
  in scope; the physics producing it is not.
- Daily/off routing path and the watershed tier are untouched; off-path
  byte identity is an acceptance obligation, not an edit surface.

## Conversion Rule

If a reproducible root cause lies inside this envelope and corrected
behavior is supported by canonical contract, pinned-baseline provenance, or
a contract-authorized physical invariant (nonnegative bin mass is one),
execute contract amendment, contract-derived failing tests,
pre-implementation gate, production correction, validation, dual review,
and disposition in this package. Do not stop at `HOLD` while those actions
remain possible in-envelope. Implementation size is not a boundary.

## Seven-Gate Bar

1. Reproduction: established (deterministic ×4, audit-recorded; restage and
   confirm once in-package with the current binary).
2. Mechanism: reduce to a named mechanism (which return site, which bin,
   which term goes negative, under what state), not a next variable.
3. Ownership: mechanism must lie in the declared write set; snow physics
   and upstream-invalid-input route out per the boundaries above.
4. Authority: corrected behavior must trace to `SC-OFEROUTE-001` (amended
   if needed), pinned provenance, or the nonnegativity invariant — never
   "make the guard stop firing."
5. Safety: fail-closed guard keeps its strength; positivity by
   construction; no surrogate numerics.
6. Testability: a committed regression fails before the fix and passes
   after; anti-tautology — it must assert run completion **and** closure
   reconstruction from independent operands on the failing span, not mere
   absence of the error.
7. Validation: the canonical 34-year endpoint completes under the
   production default with rev-27 day-closure hard-fails green and closure
   metrics at numerical scale, measured before (failing) and after.

## Acceptance Criteria

- The staged canonical 34-year H2637 endpoint completes (exit 0) under
  default activation with `wepp_ui` both false and true; closure metrics at
  the contract's numerical-closure scale across the full span.
- Committed regression per gate 6.
- Daily/off path outputs byte-identical pre/post on the same inputs.
- Plain Case-4 oracle ladder, 19-OFE conservation, selected-cohort active
  suite, and full workspace gates green.
- Timing evidence recorded per the QA-M3 build-provenance protocol for the
  first completed active 34-year endpoint run (closes the audit's
  apples-to-apples gap); no performance target is set — completion and
  closure are the acceptance, the number is evidence.

## Phase Plan

1. Restage, reproduce once, and capture localized day-2621 state
   (attribution milestone begins).
2. Attribute to a named mechanism; classify ownership (in-envelope /
   snow-physics boundary / invalid-input non-defect).
3. Amend or confirm `SC-OFEROUTE-001`; add failing contract-derived tests.
4. Record pre-implementation gate.
5. Implement the production correction.
6. Validate: full 34-year endpoint, byte-identity off-path, oracle/cohort/
   workspace gates, timing evidence.
7. Dual review, dual verification, disposition, defect-shaped handoff (if
   any residual defect is named).

## HOLD Legitimacy

A `HOLD` is legitimate only at: (a) missing or contradictory canonical
authority for the correct recession/terminal-bin treatment; (b) mechanism
proven outside the envelope (snow/winter physics, or a non-routing upstream
producer emitting invalid supply) with the owning route named; (c) input
proven invalid upstream such that the typed fail-closed report is the
correct behavior (validated non-defect). Each requires the hold-legitimacy
audit (boundary, evidence, in-envelope route considered, why it cannot
close). Diagnostic uncertainty, effort, or fix size are not boundaries.

## Required Reading

Core: root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`, this
package, and the discovery audit
(`docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md`).

Conditional/on-demand: `SC-OFEROUTE-001`;
`20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001`
(rev-41 positivity posture);
`20260708-laned-router-tier1-local-numerics-001` and
`20260708-laned-router-post-tier1-hotpath-sweep-001` (current numerics);
ADR-0037 (deficit-carry history and the revival prohibition); ADR-0011,
ADR-0017; `docs/numerics/` Z-rating and determinism notes. Record exact
tiers and bytes in `artifacts/required-reading-map.md` before edits.

## Review and Subagent Authorization

Dual independent review and dual verification are mandatory, including the
DC-ExecPlan reviewer obligations (HOLD legitimacy, envelope adequacy,
protected-boundary integrity, anti-tautology). This package authorizes
subagent delegation to bounded reviewer/verifier roles and a suite runner
for heavy gates; delegated write access is bounded to package artifacts.

## Security Impact

Expected `NONE`: local kernel/solver and test surfaces only; no parser
boundary widening, no subprocess or connectivity changes.

## Progress

- [x] Scaffolded from the 2026-07-10 endpoint audit.
- [x] Reproduced in-package; day-2621 state captured.
- [x] Mechanism named; ownership classified.
- [x] Contract adjudicated; contract-derived tests fail before correction.
- [x] Pre-implementation gate recorded.
- [x] Production correction landed.
- [x] 34-year endpoint completes; repository gates green; timing evidence
  recorded.
- [x] Dual review/verification and disposition complete.

## Surprises and Discoveries

- The material terminal deficit is not a bin-recorder defect. Eight
  consecutive raw predictor outlet faces became negative during a
  source-quiet dry-front recession. The existing recorder correctly retained
  and surfaced the terminal deficit.
- With `--legacy-sidecar-discovery`, TOML `wepp_ui = true` alone did not make
  the run effectively hourly. The first candidate manifest reported
  requested/effective `0/0`; it was rejected as evidence. Adding the canonical
  empty `wepp_ui.txt` sidecar yielded requested/effective `1/1` and the
  accepted true-mode run.
- The discovery audit's config-B scratch staging remained locally available,
  permitting exact five-output disabled-path pre/post byte comparison rather
  than reconstruction from hashes alone.

## Decision Log

- 2026-07-11: Classified the root cause as an unbounded negative downstream
  predictor-face extrapolation in `KinematicWaveSolver::step`; ownership is
  inside the declared routing write set. Snow physics and seam booking are not
  implicated by the finite positive upstream handoff and zero local source.
- 2026-07-11: Amended `SC-OFEROUTE-001` to rev 51 before production edits.
  The one-way outlet applies an exact-zero lower bound during face
  construction, before the existing available-water upper cap. This is a
  physical boundary-domain restriction, not a tolerance or state clamp.
- 2026-07-11: Chose a two-cell inline contract vector rather than committing a
  34-year fixture. It reproduces the exact negative raw-face mechanism,
  failed before the fix, independently reconstructs closure, and is cheap
  enough for every workspace run. The canonical full endpoint remains the
  executional acceptance fixture.
- 2026-07-11: Retained the `NegativeOutletBin` guard unchanged and pinned it
  with a separate defensive test.
- 2026-07-11: Accepted every independent review finding. Strengthened the
  regression to separate positive committed outlet discharge from the
  exact-zero stage face and observe all stage faces; completed exact consumer
  lineage and old-path evidence; corrected recorder comments and catalog
  status. The final workspace profile remained green.

## Outcomes and Retrospective

`LANED-NOB-001` is closed. The exact-zero one-way predictor outlet boundary is
contract-bound and implemented before the existing conservative upper cap;
the terminal guard remains live. The strengthened regression flips on that
line and independently reconstructs closure. Both effective 34-year endpoint
modes, protected five-output byte identity, the selected production cohort,
Case-4/19-OFE evidence, and the final `1694/1694` workspace profile are green.
All review findings were accepted and fixed; both independent verifiers pass.

The principal process lesson is that endpoint soaking must cover recession
tails beyond short committed fixtures, and effective mode selection must be
read from manifests rather than inferred from TOML intent under legacy sidecar
discovery.
