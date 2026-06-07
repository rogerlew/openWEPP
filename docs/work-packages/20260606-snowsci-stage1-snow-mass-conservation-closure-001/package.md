# SNOWSCI Stage 1 — Snow Mass Conservation and Single-Sourcing Closure

Status: queued

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` end-to-end: openWEPP's snow
store is multi-represented and does not conserve by construction, which drives
`snow.runtime_swe` negative (`-0.006171 m` at J-95 for `p7`, `p11`, `p18`,
`p20`) and is the leading candidate cause of the `WBVAL06` water-balance
residual (`R > 0`, water vanishing) across the 18 valid-climate WAT emitters.

Make the snow store **single-sourced and conserving by construction** so that
`SWE >= 0` always holds, `in = out + ΔStorage` closes within a named tolerance,
and the four accreted `>= 0` snow-state guards become assertions that never fire
on valid runs. This is an **accounting/architecture** closure. It explicitly does
**not** adjudicate snow physics-magnitude equations — that is Stage 2, behind the
protected boundary.

This package owns correction inside the snow mass-conservation envelope. If the
root cause is in-envelope and authority-backed, it must land the contract-first
fix. If conservation cannot be achieved without changing a snow physics-magnitude
equation, it stops at the protected boundary and escalates that specific equation
to Stage 2 — that escalation is a successful boundary outcome, not a relay.

## Rationale

Static analysis of the negative-SWE site (recorded in WBVAL05's review and the
`20260605-snow-code-deferred-science-review.md` two-stage split) found:

- The carried snow state is a tuple of separately-tracked, separately-guarded
  quantities (`snow.runtime_swe`, `snow.runtime_depth_m`,
  `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`) reconciled across
  projection → WB14 redistribution → WB13 publication → next-day carry.
- The WB14 redistribution path (`runtime_swe_after`,
  `03_kernel_support_00_support_helpers.rs:~4127-4202`) is conserving (caps state
  loss to available pack water, fails closed beyond an overdraw tolerance, clamps
  to 0), so it does not emit the negative. The negative is a carry/reconciliation
  residual upstream.
- The runner already instruments `snow_runtime_swe_closure_error_m`
  (`crates/openwepp-runner/src/hillslope/mod.rs:~4753`) — the snow non-closure is
  *monitored*, not enforced.
- Negative SWE is defended at four consumers (WB14 kernel
  `validate_runtime_snow_state_domains`, WB13 builder, SIMIMPL28 reader, runner
  writeback) — accreted defenses around one non-conserved store.

This is a conservation/bounds hard gate (ADR-0011 + correctness re-anchoring) that
sits on rung-1's water-balance closure gate, so it cannot be deferred. Per the
ADR-0018 grouping rule it bundles the WBVAL05 negative-SWE follow-on and WBVAL06
because they are plausibly one snow mass-balance defect; the package's first
milestone confirms common cause before proceeding.

**Provenance — this is an openWEPP-introduced regression, not inherited legacy
behavior.** A 2026-06-06 upstream check compared the pinned baseline against the
newer wepp-forest tip (diff is pure `watbal_process_probe` instrumentation, zero
snow-physics change), `jimf-wepp-2023` (mainline 2023), `wepp-forest-wb61`, and
`wepp-forest-revegetation`. Every legacy variant carries the identical
negative-pack guard in `snowd.for` (`if (snodep .le. 0.0) then
wmelt = snodpt*densg*0.001; snodep = 0.0; densgt = 0.0`): legacy is
**depth-authoritative** (`snodpt` is the carried quantity, density secondary, SWE
implied) and conserving by construction — it never produces a negative pack.
openWEPP went **SWE-centric**, carrying SWE/depth/density as separate state and
reconstructing, and the SWE carry/publication path is where the negative arises
(consistent with WBVAL05: openWEPP's WB14 redistribution already clamps; the
negative is in the SWE carry legacy does not use). **Consequence for this
package:** separability risk is low — legacy proves a conservation-safe snow
accounting exists that resolves none of the Stage-2 physics questions, so Stage-1
conservation is almost certainly achievable without any physics-magnitude change.
Legacy's clamp is corroborating conservation evidence (it agrees with the
mass-conservation law that is the authority, per ADR-0017), not a magnitude oracle.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION`
  - Observable failures:
    - `snow.runtime_swe = -0.006171 m` at `sim_day_index=95`,
      `calendar_year=1990`, `julian_day=95` for `p7`/`p11`/`p18`/`p20`, now
      surfacing at `HKERNEL-WB14-RUNOFF-E-003` after the WBVAL05 WB18 fix.
    - `WBVAL06` complete-identity annual residual `> 1.0 mm/year` (max ~94.4 mm,
      `p4` year 5; `R > 0`) across the 18 WAT emitters — candidate same cause,
      to be confirmed in Milestone 1.
    - `snow_runtime_swe_closure_error_m` is materially non-zero on affected
      days.
  - Fixture: `/wc1/runs/in/indispensable-presenter/wepp/runs/` under the
    WBVAL04 publication-safe climate.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
    (single-source / conservation / `SWE >= 0`-by-construction invariants).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
    (snow-storage closure consumer gate; `Snow-Water` publication lineage).
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
    (snow-state projection / prior-day carry).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
    (WB14 snow redistribution, `validate_runtime_snow_state_domains`).
  - `crates/openwepp-runner/src/hillslope/mod.rs` (snow-state carry,
    `build_simulation_owned_wb13_row`, `snow.runtime_swe` carry, closure-error
    instrument).
  - `tests/integration/**snowsci_stage1**.rs`, `**snow**.rs`, `**watbal**.rs`
    for contract-derived regressions.
  - `docs/work-packages/20260606-snowsci-stage1-snow-mass-conservation-closure-001/**`
  - `docs/work-packages/README.md`

### Allowed Edit Classes

- Amend canonical `SC-SNOWFREEZE-001`/`SC-WATBAL-001` to require a single
  authoritative snow store, `SWE >= 0` by construction, and enforced per-step snow
  mass closure within a named tolerance.
- Single-source the snow store: make one quantity authoritative and derive or
  reconcile the others (depth, density, settle-day-count) from it, so they cannot
  drift to a negative or inconsistent SWE.
- Promote the existing `snow_runtime_swe_closure_error_m` from a monitored trace
  to an enforced conservation gate.
- Add contract-derived tests and bounded diagnostics needed to localize and prove
  the conservation closure.

### Protected Boundaries (do not cross)

- **No snow physics-magnitude change.** Do not alter the melt-model magnitude,
  the `snowd.for` settling/density equations (CRM Eq. 3.7.x), `driftf`/`driftg`
  semantics, the daily-temperature-in-hourly threshold, or rain/snow partition
  magnitude. These are Stage 2 (the deferred science review). If conservation
  requires changing any of them, STOP and escalate that specific equation to
  Stage 2 with evidence (this proves separability is false for that term).
- **No silent clamp.** Achieving `SWE >= 0` by clamping the negative to zero is
  forbidden — it converts the visible fail-closed into the invisible WBVAL06
  leak. `SWE >= 0` must hold *by construction* (conservation), not by clamping.
- **No baseline replication as authority for snow physics.** Stage 1's authority
  is conservation/physical invariants + `SC-*` contract text, not legacy-baseline
  matching (ADR-0017).
- Do not edit WEPPpy producers or `/wc1` inputs; do not reopen the snow/`RM`
  comparator route.

### Acceptance Criteria

- `p7`, `p11`, `p18`, `p20` reach WAT publication on the WBVAL04 valid climate
  without negative-SWE fail-closed (or fail closed for a genuinely different,
  newly-evidenced reason outside this envelope).
- The 18 WBVAL06 emitters' complete-identity annual residual collapses to within
  the WBVAL06 tolerance for years `2..6` **to the extent the residual is
  snow-sourced**; any non-snow remainder is re-routed to its owning defect with
  evidence.
- `snow_runtime_swe_closure_error_m` reads within the named tolerance on valid
  runs, and the snow-state `>= 0` guards do not fire.
- Every change is backed by canonical `SC-*` text, contract-derived red/green
  tests, pre-implementation failing evidence, and post-fix validation.

### Branch-out Boundaries

- If Milestone 1 disproves common cause (the negative-SWE hillslopes and the
  high-residual emitters do not share the snow mass lineage), keep the negative-SWE
  defect in this package and re-route the residual to a non-snow defect target.
- If conservation requires a physics-magnitude equation change, escalate that
  equation to Stage 2 and close this package at the protected boundary with the
  conservation work that *was* separable landed.

## Conversion Rule

If this package establishes a reproducible root cause inside the declared snow
mass-conservation envelope and the corrected behavior is supported by canonical
`SC-*` authority or a contract-authorized physical invariant (mass conservation,
`SWE >= 0`), it must proceed through contract amendment, contract-derived tests,
pre-implementation gate evidence, production correction, validation, review, and
disposition in this package. It may not close as `HOLD` merely because more
investigation is possible.

## Seven-Gate Bar

All seven true means `HOLD` is invalid and the package must land the fix:

1. Reproduction: negative SWE / non-zero closure error reproduced or statically
   tied to WBVAL04/WBVAL05/WBVAL06 evidence.
2. Mechanism: reduced to a named conservation/single-source mechanism in the
   snow carry/projection, not "trace the next snow variable."
3. Ownership: the mechanism is inside the declared snow mass-conservation
   write-set (and not a snow physics-magnitude equation — that is the protected
   boundary).
4. Authority: the corrected behavior traces to a conservation/physical invariant
   and `SC-SNOWFREEZE-001`/`SC-WATBAL-001`, not baseline replication.
5. Safety: no silent clamp, no guard loosening, no physics-magnitude change, no
   downstream compensation.
6. Testability: a contract-derived regression fails before and passes after.
7. Validation: the negative-SWE and closure-error acceptance is measurable before
   and after.

## Symptom-Existence and Common-Cause Gate (Milestone 1, diagnostic-first)

Before any fix:

1. Localize the exact site/path where SWE first goes negative — the
   depth↔density↔SWE reconciliation across the `04_snow_frost_irrigation`
   projection, the prior-day carry, and WB13 publication. (Static analysis could
   not pin this line; this milestone must.) Use legacy's depth-authoritative
   clamped representation (`snowd.for` `snodep <= 0 -> snodep = 0`,
   `wmelt = snodpt*densg*0.001`) as the reference for *where* openWEPP's SWE carry
   diverges from a conserving single source — every legacy variant has this guard,
   so the divergence is openWEPP-side.
2. Confirm the negative is a **conservation/single-source** defect, not a
   physics-magnitude error. If it is physics-magnitude, escalate to Stage 2.
3. Common-cause check: determine whether the J-95 negative-SWE hillslopes and the
   WBVAL06 high-residual emitters share the snow mass lineage. If yes, both are
   owned here; if no, branch per the boundary above.

## Legitimate HOLD Conditions

- Conservation requires a snow physics-magnitude equation change (→ Stage 2 with
  the specific equation named).
- Required evidence cannot be generated in the environment.
- Canonical authority is missing or contradictory for the conservation behavior.

Grind-HOLD (forbidden): "inspect the next snow helper," "trace SWE one level
deeper," "root cause is in the snow carry but implementation deferred."

## Milestones

1. Symptom-existence + common-cause gate (above).
2. Contract: amend `SC-SNOWFREEZE-001`/`SC-WATBAL-001` for single-source +
   conservation + `SWE >= 0` by construction.
3. Contract-derived red tests for negative-SWE-by-construction and per-step snow
   closure.
4. Pre-implementation gate evidence.
5. Production correction: single-source the snow store; enforce closure.
6. Validation: rerun `p7`/`p11`/`p18`/`p20` and the 18 emitters; check closure
   error and residual collapse.
7. Dual review, finding disposition, dual verification, defect-shaped handoff.

## Deliverables

- `artifacts/snow-store-single-source-design.md` (the conservation/single-source
  design + Milestone-1 localization and common-cause result).
- `artifacts/snow-conservation-validation-ledger.md` (before/after negative-SWE,
  closure-error, and WBVAL06-residual evidence).
- Standard contract, gate, dual-review, verification, disposition, and
  worker-handoff artifacts.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- WBVAL04/WBVAL05/WBVAL06 work packages and Claude review artifacts.
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Autonomy

Execute end-to-end for the declared scope. Do not ask for direction on
intermediate diagnostic steps. Ask only if hard-blocked by a proven protected
boundary (a required physics-magnitude change → Stage 2), missing authority, or
unavailable validation substrate.
