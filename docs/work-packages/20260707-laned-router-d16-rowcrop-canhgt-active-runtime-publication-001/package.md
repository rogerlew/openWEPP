# D16 Row-Crop Canopy-Height Active Runtime Publication

Status: EXECUTED-COMPLETE

## Objective

Close the selected-cohort active-suite blocker where `mn_corn_h4` fails closed
because the active Lane D Rev-21 operand guard observes positive post-growth
LAI with missing/non-positive canopy height. Implement the authority-backed
runtime publication fix: daily PL growth must compute and publish `Hc`/`canhgt`
from the legacy WEPP canopy-height equation, and Lane D active/shadow friction
operand builders must consume that same post-growth daily surface.

After the fix, rerun the D16 selected-cohort active plain-vs-hybrid evidence
path far enough to determine whether the prior `EXECUTED-HOLD-ACTIVE-RUN`
condition is closed or replaced by a new, explicitly evidenced hold.

## Rationale

The previous selected-cohort package proved that the row-crop fixture is not
missing crop growth: LAI becomes positive by day 136. The defect is a runtime
publication/source-shape mismatch. openWEPP carries daily growth state for
`sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, and `hia`, but not
`canhgt`, while Lane D pairs post-growth LAI with a static typed-management
`canhgt` seed. The legacy WEPP growth routine computes canopy height daily:
`canhgt = (1 - exp(-bbb * vdmt)) * hmax`.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `tests/AGENTS.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/active-suite-runs.md`
- `docs/work-packages/20260707-laned-router-d16-selected-cohort-active-suite-001/artifacts/hold-legitimacy-audit.md`
- `/workdir/wepp-forest_260430_baseline/src/grow.for`
- `/workdir/wepp-forest_260430_baseline/src/initgr.for`
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`

## Scope

Included:

- Contract-first amendment to make daily `Hc`/`canhgt` a required PL16 growth
  state surface and the source of Lane D `h_c` when paired with post-growth LAI.
- Runtime growth-state extension carrying daily canopy height through
  `DirectGrowthStateSurface`.
- Runner projection of crop parameters `bbb` and `hmax` into growth inputs.
- Daily ET/routing publication update so Lane D active/shadow operand builders
  consume post-growth `canhgt`, not only static initial typed-management
  canopy height.
- Contract-derived focused tests and source guards.
- Re-execution of the selected-cohort active suite evidence path, or a precise
  new hold if a later fixture blocks after this defect is closed.

Excluded:

- Changing route-coefficient values or Disturbed/wepppy management production.
- Relaxing the Rev-21 guard that requires positive `h_c` when `LAI > 0`.
- Hybrid default promotion, tolerance ratification, mesh policy, or broad
  `OPENWEPP_LANED_ACTIVE` default activation.
- Surrogate/proxy canopy-height formulas not present in legacy WEPP authority.

## Correction Authority Envelope

Observed failure:

- `mn_corn_h4` active plain fails on lane 1 day 136:
  `LAI 0.01182723510043506 > 0 with missing/non-positive typed-management
  canhgt`.

Authority-backed correction:

- `SC-PLANT-001` already declares `Hc` as a plant state/coupling surface and
  cites baseline `grow.for` / `initgr.for`.
- `/workdir/wepp-forest_260430_baseline/src/grow.for` computes daily
  `canhgt = (1 - exp(-bbb * vdmt)) * hmax`.
- `/workdir/wepp-forest_260430_baseline/src/initgr.for` initializes live
  canopy geometry from `cancov`, `bb`, `bbb`, and `hmax`.
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for` consumes daily
  `canhgt` for live friction cover context.

Protected boundaries:

- Do not weaken positive-canopy fail-closed guards.
- Do not synthesize `canhgt` from LAI, canopy cover, PMET diagnostics, frost
  diagnostics, or compatibility wrappers.
- Default/off output identity remains protected by existing selector isolation;
  this package does not change default Lane D activation posture.

## Phase Plan

1. **S0 Scaffold and authority map.** Create package structure, catalog entry,
   and required-reading/source-authority artifacts.
2. **S1 Contract amendment.** Amend `SC-PLANT-001` and `SC-OFEROUTE-001` so
   daily `Hc`/`canhgt` publication and Lane D `h_c` consumption are canonical.
3. **S2 Implementation.** Add daily canopy height to growth state, parameter
   projection, ET publication, and active/shadow Lane D operand consumption.
4. **S3 Focused validation.** Run contract/source-guard tests and focused
   Lane D/growth tests.
5. **S4 Selected-cohort rerun.** Rebuild the release runner and rerun the
   active plain/hybrid selected-cohort path or record the first new blocker.
6. **S5 Closure.** Complete review, verification, gate table, disposition,
   line-count governance, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegation
to science-authority review, implementation review, comparator/timing, package
QA, and verification subagents for contract/source review, selected-cohort
rerun verification, gate review, and disposition review. Expected outputs are
package-local `artifacts/review-*.md`, `artifacts/verification-*.md`, and
compact timing/comparator evidence. Write access is read-only unless a worker
is explicitly assigned a bounded package-artifact correction.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/source-authority.md`
- `artifacts/implementation.md`
- `artifacts/active-suite-rerun.md`
- `artifacts/timing-and-deltas.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks for `SC-PLANT-001` and `SC-OFEROUTE-001`.
- Focused growth/canopy-height tests.
- Focused Lane D / `ofe_routing` active operand-source tests.
- Selected-cohort active plain rerun through at least the former
  `mn_corn_h4` day-136 blocker.
- Selected-cohort active plain and explicit hybrid rerun for every selected
  member unless a new, later blocker is proven.
- Protected-output/default-off isolation evidence if runtime selectors are
  touched.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Closure Outcomes

- `EXECUTED-COMPLETE`: daily `canhgt` is contract-backed, implemented, consumed
  by active Lane D, and the selected-cohort active plain/hybrid suite no longer
  holds on canopy-height publication.
- `EXECUTED-HOLD-ACTIVE-RUN`: this defect is closed, but a later active
  selected-cohort run failure appears; record exact failure/evidence/follow-on.
- `EXECUTED-HOLD-AUTHORITY`: legacy/contract authority proves insufficient or
  contradictory for daily canopy-height publication.
- `EXECUTED-HOLD-VALIDATION`: implementation lands but required gates cannot
  complete in-envelope; record why and first actionable follow-on.

## Final Outcome

`EXECUTED-COMPLETE`: the former `mn_corn_h4` active plain day-136 failure is
closed. Daily PL growth now computes and publishes post-growth canopy height,
Lane D active/shadow operand builders consume that post-growth surface, and the
selected-cohort active plain/hybrid suite completed for all four members.

This package did not promote hybrid to the default selector. It removes the
selected-cohort active-run evidence hold so default-promotion/tolerance
adjudication can proceed on the existing known hybrid fidelity deltas.
