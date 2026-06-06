# WBVAL04 Rocky Mountain Daymet WBVAL01 Redo

Status: executed-hold - valid-climate validation complete, defects routed

Package type: validation/characterization work package

## Objective

Redo WBVAL01 on `/wc1/runs/in/indispensable-presenter` after the WEPPpy
observed-Daymet radiation producer fix is rebuilt into the run artifacts.
Produce a fresh single-OFE water-balance closure ledger for all eligible
hillslopes, replacing the stale WBVAL01 evidence that was blocked by invalid
daily radiation and later WBVAL03 reruns.

This package is intentionally not a Defect-Closure ExecPlan. It does not own
kernel correction. It validates the post-climate-fix population and, if new
invariant violations remain, emits defect-shaped follow-ons rather than
diagnostic breadcrumbs.

## Execution Result

WBVAL04 execution on `2026-06-06` found that the rebuilt
`indispensable-presenter` climate artifacts are now publication-safe for
openWEPP: `wepp.cli` and `wepp_cli.parquet` both had `2191` rows, zero
`rad > baseline sunmap.r3` exceedances, and a minimum positive margin of
`0.000293 Ly/day` below the exact bound.

The validation batch then ran all `22` single-OFE hillslopes with release
`openwepp-cli-hill` at source commit
`5b23ef27d398e69bf754be730d28fce63a38c131`. Current outcomes are:

- `18/22` emitted WAT and all `18` are conservation-break for years `2..6`
  under the complete declared identity.
- `4/22` (`p7`, `p11`, `p18`, `p20`) still fail closed at J-95 with
  `HKERNEL-WB11-PERC-E-003`.
- The prior radiation blockers (`p2`, `p4`, `p6`, `p9`, `p14`, `p17`) now
  emit WAT and join the conservation-residual defect population.

Final disposition is `executed-hold`: WBVAL04 achieved the WBVAL01 redo
validation objective but cannot close as complete while valid-climate invariant
violations remain. Follow-ons are named in `artifacts/worker-handoff.md`.

## Rationale

Original WBVAL01 established the Rocky Mountain single-OFE validation surface
but could not produce a full population ledger: `6` hillslopes failed closed on
`CLIM-RUNTIME-E-017`, `4` failed on `HKERNEL-WB11-PERC-E-003`, and only `12`
emitted complete WAT ledgers. WBVAL02 and WBVAL03 showed that the radiation
blocker preempted later snowmelt/percolation and water-balance surfaces.

After the WEPPpy climate producer publishes physically bounded daily radiation,
the correct next step is to rerun the whole WBVAL01 validation population in
one package. This is right-sized because all targets share the same run,
wrapper set, validation harness, closure identity, and downstream roadmap
decision.

## Included Scope

- Verify current WEPPpy climate artifacts for
  `/wc1/runs/in/indispensable-presenter`:
  - `climate/wepp.cli`
  - `climate/wepp_cli.parquet`
  - `climate/daymet_1990-1995.parquet`
  - `climate/daymet_radiation_toa_normalization_wepp.csv`
- Enumerate all hillslope inputs under
  `/wc1/runs/in/indispensable-presenter/wepp/runs/`.
- Recreate or reuse WBVAL01 TOML wrappers only after path and climate
  preconditions are verified.
- Run `openwepp-cli-hill` for every single-OFE hillslope with the current
  release binary/source hash recorded.
- Produce a fresh per-hillslope, per-year water-balance closure ledger using a
  complete declared identity.
- Compare the new results against prior WBVAL01/WBVAL02/WBVAL03 evidence:
  radiation blockers, J-95 percolation blockers, WAT emitters, and residual
  classes.
- Produce defect-shaped follow-ons for any remaining fail-closed or
  conservation residual classes.

## Excluded Scope

- No production Rust kernel edits.
- No contract amendments unless execution proves package metadata itself is
  wrong; any kernel defect correction must branch to a new DC-ExecPlan.
- No WEPPpy climate edits from this openWEPP package.
- No relaxation of `CLIM-RUNTIME-E-017` or any radiation guard.
- No snow-magnitude or fixed-comparator target reopening.
- No MOFE routing closure beyond observe-only watershed preview.

## Deliverables

- `artifacts/climate-precondition-audit.md`
- `artifacts/run-manifest.md`
- `artifacts/single-ofe-closure-ledger.md`
- `artifacts/wbval01-redo-comparison.md`
- Standard package artifacts listed in `artifacts/README.md`

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `/workdir/openWEPP/docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `/workdir/openWEPP/docs/backlog/20260605-snow-code-deferred-science-review.md`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval01-rocky-mountain-daily-climate-single-ofe-wb-closure-validation-001/`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001/`
- `/workdir/openWEPP/docs/work-packages/20260606-wbval03-snowmelt-wb-closure-defect-closure-001/`
- `/workdir/wepppy/docs/adrs/ADR-0006-observed-daymet-radiation-toa-normalization.md`
- `/workdir/wepppy/docs/work-packages/20260606_indispensable_presenter_daymet_radiation_bounds/artifacts/execution_evidence.md`
- Run inputs: `/wc1/runs/in/indispensable-presenter/wepp/runs/`

## Intended Write Set

- `docs/work-packages/20260606-wbval04-rocky-mountain-daymet-wbval01-redo-001/**`
- `docs/work-packages/README.md`

Production Rust, canonical contracts, tests, and WEPPpy files are out of scope
for this validation package.

## Phase Plan

1. Climate precondition audit.
   - Verify `wepp.cli` and `wepp_cli.parquet` contain no daily `rad` value
     above baseline `sunmap.r3` for the same date and CLI latitude.
   - Verify Daymet provenance columns/artifact exist and record affected rows.
   - If this fails, close `HOLD` at the WEPPpy climate boundary.
2. Input inventory and wrapper readiness.
   - Enumerate single-OFE vs multi-OFE hillslopes.
   - Recreate/reuse TOML wrappers with exact paths and source hash recorded.
3. Release-binary validation run.
   - Build or select the release `openwepp-cli-hill`.
   - Run every single-OFE hillslope and record command, exit status, and first
     fail-closed code where applicable.
4. Closure ledger.
   - Compute year `2..6` closure against the complete WBVAL identity.
   - Treat year `1` as classified only if an initial storage surface is
     available; otherwise record the explicit boundary.
5. Comparison and follow-on shaping.
   - Compare against WBVAL01, WBVAL02, and WBVAL03 outcomes.
   - Name any remaining defect targets with defect ID, observed failure,
     suspected mechanism, owning write set, failing fixture, authority, and
     acceptance target.
6. Review, verification, and disposition.
   - Complete dual independent reviews, finding disposition, dual
     verification, gate results, worker handoff, and final disposition.

## Exit Criteria

- Climate precondition audit either passes with zero CLI radiation bound
  exceedances or the package closes `HOLD` with exact WEPPpy-boundary evidence.
- If climate passes, every single-OFE hillslope has a fresh run result and
  classification.
- `single-ofe-closure-ledger.md` declares the identity, units, tolerance, and
  per-year status truthfully.
- `wbval01-redo-comparison.md` states what changed relative to WBVAL01 and what
  remains blocked or broken.
- Any follow-on is defect-shaped; no handoff names only a next diagnostic step.
- Dual reviews and verifications are complete with no undispositioned findings.

## Security Impact Gate

- Security impact: none.
- Rationale: the package reads local run artifacts, executes local openWEPP
  binaries, and writes package-local documentation/evidence. It does not add
  network, auth, secret, upload, download, queue, or public API behavior.
- Dedicated security review required: no.
