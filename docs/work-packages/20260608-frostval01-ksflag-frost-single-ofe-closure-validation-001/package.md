# FROSTVAL01 ksflag Frost Single-OFE Closure Validation

Status: executed-hold

Package type: validation/characterization work package

## Objective

Validate openWEPP's standard-WEPP frost path (`ksflag = 1`) on a real
frost-enabled run, as roadmap **rung-2** built on the closed rung-1 single-OFE
water-balance foundation. Confirm two things on the single-OFE hillslopes of
`/wc1/runs/al/algebraic-radium`:

1. **Frost actually activates** — openWEPP honors `ksflag = 1`, computes frost
   depth / frozen soil water on cold days, and the frost gate bites
   infiltration/percolation; and
2. **single-OFE water-balance closure holds with the frost gate engaged** — the
   rung-1 conservation closure (snow conservation + interception publication +
   totalwatsed3 audit) remains closed within tolerance when frost is on.

This is a validation/characterization package: it measures and classifies, emits
defect-shaped follow-ons for any frost activation or conservation defect, and
makes **no production edits**.

## Run Under Validation

`/wc1/runs/al/algebraic-radium` (legacy comparator `wepp_260606`):

- `44` hillslopes total: **43 single-OFE** (the frost-rung targets) + **1
  17-OFE** hillslope (MOFE — observe-only, deferred to rung-3).
- All hillslope managements declare `1 # Landuse - <Cropland>` (lanuse=1), so the
  baseline `infile.for:2205` `if (lanuse≠1) ksflag = 0` override does not fire and
  **`ksflag = 1` (frost Ks-adjustments enabled) on every hillslope**. Plant cover
  varies (Corn, Tah_4899, bromegrass, Fallow); the WEPP landuse field is Cropland
  throughout.
- Climate: observed **gridmet** daily (srad→Langley `×(86400/41840)`, the correct
  full-day-mean conversion — no over-TOA radiation-bound issue like daymet).
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606` +
  `wepp_260606_hill` (260430 baseline + negmelt fix). Per ADR-0017 a flag, not a
  target.

## Rationale

Rung-1 (single-OFE WB closure) is complete and auditable (SNOWSCI-S1 + WBVAL06 +
the totalwatsed3 interception companion). Rung-2 adds exactly one mechanism on
that closed foundation: the frost column gate. Per the agreed sequence, frost is a
per-column vertical mechanism, so it is settled on single-OFE geometry before
routing (MOFE) is layered on.

By forcing Cropland/`ksflag = 1`, this run exercises the **standard WEPP frost
Ks-adjustment path**, not the forest `ksatadj` sat-fraction conductivity model
(which the baseline runs with frost off for forest, `lanuse≠1`). So this validates
openWEPP's standard frost gate; the forest-`ksatadj` path is a separate concern
not exercised here.

The load-bearing risk this package must guard: if openWEPP's frost path is stubbed
or silently inactive, the water balance would "close" trivially (no frost effect)
and falsely pass rung-2. Milestone 1 therefore proves frost is genuinely engaged
before any closure-under-frost result is trusted.

## Included Scope

- Run `openwepp-cli-hill` on the 43 single-OFE hillslopes with the current
  release binary (record build hash).
- **Milestone 1 — frost-activation audit (do first):** confirm openWEPP honors
  `ksflag = 1` and that frost state activates — frost depth, frozen soil water
  (`frost.runtime_ws_frz` / `frzw` / `frozwt`), and `ksflag`-gated conductivity
  differ from a no-frost baseline on cold days. Compare against `wepp_260606`
  frost behavior as a flag. If frost does not activate, that is the primary
  finding and the closure audit is deferred.
- **Conservation-closure-under-frost audit:** recompute the rung-1 complete
  identity per hillslope/year **with frost engaged**, including `Interception`,
  `frozwt`/frozen storage, and snow terms; confirm closure within the rung-1
  tolerance; run the totalwatsed3 closure audit on the output.
- Classify each single-OFE hillslope: `frost-active` vs `frost-inactive/partial`;
  `frost-conservation-clean` vs `frost-conservation-break` (first year + dominant
  non-closing term).
- Characterize whether the `ksflag` gate measurably affects infiltration/
  percolation vs no-frost (does the gate bite).
- Emit defect-shaped follow-ons (frost DC-ExecPlans) for any activation or
  conservation defect.

## Excluded Scope / Protected Boundaries

- No production Rust kernel edits — this characterizes; corrections are
  defect-shaped follow-on DC-ExecPlans.
- No comparator-match acceptance; `wepp_260606` is a flag (ADR-0017). Acceptance
  is conservation closure with frost engaged.
- No frost physics-magnitude tuning (route to a frost DC-ExecPlan).
- **Snow magnitude protected boundary (Stage-2):** if a frost residual attributes
  to snow magnitude, route to `docs/backlog/20260605-snow-code-deferred-science-review.md`,
  not an in-package snow fix.
- **MOFE deferred:** the 17-OFE hillslope is observe-only; no inter-OFE routing
  closure here (rung-3).
- **Forest `ksatadj` path out of scope:** this validates standard `ksflag` frost
  on Cropland; the forest sat-fraction conductivity model is a separate concern.
- Year-1 initial-storage exclusion carries over from rung-1.

## Deliverables

- `artifacts/frost-activation-ledger.md` — per single-OFE hillslope: `ksflag`
  honored, frost depth / frozen-water activation on cold days, ksflag-gated
  conductivity effect, and a `wepp_260606` flag comparison.
- `artifacts/frost-closure-ledger.md` — per hillslope/year complete-identity
  residual with frost engaged, totalwatsed3 audit result, and
  `frost-clean`/`frost-break` classification.
- `artifacts/run-manifest.md` — 43 single-OFE inventory, OFE counts, exact
  invocations, binary hash, input paths (evidence mode **Ran**).
- `artifacts/rung3-frost-defect-handoff.md` — prioritized defect-shaped follow-ons
  for any activation/conservation defects, plus the roadmap continuation.
- Standard gate, dual review, verification, disposition, worker-handoff artifacts.

## Acceptance / Exit Criteria

- Milestone 1 proves frost is genuinely active on cold days (not stubbed/silent)
  before any closure-under-frost result is reported.
- A complete frost-activation + closure ledger over all 43 single-OFE hillslopes,
  residuals computed from explicitly declared, fully-populated terms (no imputed
  zeros).
- Each hillslope classified `frost-active`/`frost-inactive` and
  `frost-conservation-clean`/`frost-conservation-break`.
- Defect-shaped follow-ons named for any breaks (frost activation or conservation).
- Truthful evidence mode per artifact (**Ran** for actual `openwepp-cli-hill`
  invocations; **Static** for reasoned classification).
- A `frost-conservation-break` population is a **valid, expected** outcome — it is
  the rung-2 DC-ExecPlan input, not a package failure. Package failure is an
  incomplete/untruthful ledger, or reporting closure-under-frost without first
  proving frost is active.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/workdir/openWEPP/docs/work-packages/README.md` (Current roadmap, rung-2)
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- Frost reference: legacy `infile.for` (`ksflag` read + `lanuse≠1` override),
  `infpar.for` (`ksflag`-gated conductivity), `winter.for` (frost depth) in
  `/workdir/wepp-forest_260430_baseline/src/`.
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606`,
  `wepp_260606_hill`.
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`
