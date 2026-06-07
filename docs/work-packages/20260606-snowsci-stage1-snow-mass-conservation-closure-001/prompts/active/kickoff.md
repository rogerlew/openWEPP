# SNOWSCI Stage 1 Kickoff

Execution mode: package-end-to-end

Autonomy: execute end-to-end for the declared scope — localize, confirm common
cause, amend contracts, add red tests, record the pre-implementation gate,
single-source the snow store, validate, complete dual review/verification and
disposition — without asking for direction on intermediate diagnostic steps. Ask
only if hard-blocked by a proven protected boundary, missing authority, or an
unavailable validation substrate.

## Item 1 (close this defect, not a diagnostic step)

Close defect `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` end-to-end per `package.md`:
make openWEPP's snow store single-sourced and conserving by construction so
`SWE >= 0` holds always, `in = out + ΔStorage` closes within tolerance, and the
accreted `>= 0` snow-state guards become assertions that never fire. This is the
Stage-1 (conservation/architecture) rung-1 unblocker, subsuming the WBVAL05
negative-SWE follow-on and `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` pending
the Milestone-1 common-cause check.

## Hard constraints (read before editing)

- **Protected boundary — no snow physics-magnitude change.** Do not touch the
  melt/settling/density/partition *equations* (`snowd.for` CRM Eq. 3.7.x,
  `driftf`/`driftg`, the daily-temp-in-hourly threshold). Those are Stage 2. If
  conservation requires changing one, STOP and escalate that equation to Stage 2
  with evidence — that is a successful boundary outcome, not a relay.
- **No silent clamp.** `SWE >= 0` must hold by construction (conservation), never
  by clamping a negative to zero — a clamp converts the visible fail-closed into
  the invisible WBVAL06 leak (forbidden by ADR-0018).
- **Authority is conservation law + `SC-*` text, not baseline replication**
  (ADR-0017). Contract-first: amend `SC-*` before production code.

## Milestone 1 first (diagnostic-first symptom-existence + common-cause gate)

Localize the exact site where SWE first goes negative (the depth↔density↔SWE
reconciliation across `04_snow_frost_irrigation` projection, prior-day carry, and
WB13 publication), confirm it is a conservation/single-source defect rather than
physics-magnitude, and confirm whether the negative-SWE hillslopes and the
WBVAL06 high-residual emitters share the snow mass lineage. Branch per `package.md`
if common cause is disproven.

## Required reading

- `docs/work-packages/20260606-snowsci-stage1-snow-mass-conservation-closure-001/package.md`
- `docs/defect_closure_execplans.md`
- `docs/backlog/20260605-snow-code-deferred-science-review.md` (two-stage split)
- `docs/decisions/0011-...md`, `0017-...md`, `0018-...md`
- WBVAL05 review `review_claude_wb18_fix_and_negative_swe_boundary.md` and WBVAL06
  handoff/evidence; WBVAL04 redo comparison.
- `AGENTS.md`, `docs/codex_exec_plans.md`
- Snow state code: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`,
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`,
  `crates/openwepp-runner/src/hillslope/mod.rs`.
